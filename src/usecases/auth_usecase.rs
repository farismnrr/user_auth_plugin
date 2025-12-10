use crate::dtos::auth_dto::{LoginRequest, RegisterRequest, AuthResponse};
use crate::dtos::user_dto::{CreateUserRequest, UserResponse};
use crate::dtos::user_details_dto::UserDetailsResponse;
use crate::entities::user::Model as User;
use crate::entities::user_details::Model as UserDetails;
use crate::errors::AppError;
use crate::repositories::user_repository::UserRepositoryTrait;
use crate::utils::jwt::JwtService;
use crate::utils::password;
use crate::validators::user_validator;
use actix_web::HttpMessage;
use std::sync::Arc;

/// Authentication use case handling user registration and login.
///
/// This use case manages authentication operations including user registration,
/// login with password verification, and JWT token generation. It coordinates
/// between the repository, validators, password utilities, and JWT service.
pub struct AuthUseCase {
    repository: Arc<dyn UserRepositoryTrait>,
    user_details_repository: Arc<dyn crate::repositories::user_details_repository::UserDetailsRepositoryTrait>,
    jwt_service: JwtService,
}

impl AuthUseCase {
    /// Creates a new AuthUseCase instance.
    ///
    /// # Arguments
    ///
    /// * `repository` - Arc-wrapped user repository implementation
    /// * `user_details_repository` - Arc-wrapped user_details repository implementation
    pub fn new(
        repository: Arc<dyn UserRepositoryTrait>,
        user_details_repository: Arc<dyn crate::repositories::user_details_repository::UserDetailsRepositoryTrait>,
    ) -> Self {
        Self {
            repository,
            user_details_repository,
            jwt_service: JwtService::new(),
        }
    }

    /// Registers a new user and generates authentication tokens.
    ///
    /// # Arguments
    ///
    /// * `req` - Registration request containing username, email, password, and role
    ///
    /// # Returns
    ///
    /// Returns `AuthResponse` with user data and access token.
    ///
    /// # Errors
    ///
    /// - `AppError::BadRequest` if email is already registered
    /// - `AppError::ValidationError` if input validation fails
    /// - `AppError::InternalError` if token generation fails
    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        // Validate input
        user_validator::validate_username(&req.username)?;
        user_validator::validate_email(&req.email)?;
        user_validator::validate_password(&req.password)?;

        // Check if email already exists
        if self.repository.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        // Create user via repository
        let create_req = CreateUserRequest {
            username: req.username,
            email: req.email,
            password: req.password,
            role: req.role,
        };
        let user = self.repository.create(create_req).await?;

        // Create user_details with default profile picture
        let _user_details = self.user_details_repository.create(user.id).await?;

        // Generate tokens
        let access_token = self.jwt_service.generate_access_token(user.id)
            .map_err(|e| AppError::InternalError(format!("Failed to generate access token: {}", e)))?;
        
        let _refresh_token = self.jwt_service.generate_refresh_token(user.id)
            .map_err(|e| AppError::InternalError(format!("Failed to generate refresh token: {}", e)))?;

        Ok(AuthResponse {
            user: Self::user_to_response(user),
            access_token,
        })
    }

    /// Authenticates a user and generates tokens.
    ///
    /// Accepts either email or username for login.
    ///
    /// # Arguments
    ///
    /// * `req` - Login request containing email_or_username and password
    ///
    /// # Returns
    ///
    /// Returns a tuple of `(AuthResponse, refresh_token)`.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Unauthorized` if credentials are invalid.
    pub async fn login(&self, req: LoginRequest) -> Result<(AuthResponse, String), AppError> {
        // Try to find user by email first
        let user = match self.repository.find_by_email(&req.email_or_username).await? {
            Some(u) => u,
            None => {
                // If not found by email, try username
                self.repository.find_by_username(&req.email_or_username).await?
                    .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?
            }
        };

        // Verify password
        if !password::verify_password(&req.password, &user.password_hash)? {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Generate tokens
        let access_token = self.jwt_service.generate_access_token(user.id)
            .map_err(|e| AppError::InternalError(format!("Failed to generate access token: {}", e)))?;
        
        let refresh_token = self.jwt_service.generate_refresh_token(user.id)
            .map_err(|e| AppError::InternalError(format!("Failed to generate refresh token: {}", e)))?;

        // Fetch user_details
        let user_details = self.user_details_repository.find_by_user_id(user.id).await?;

        Ok((
            AuthResponse {
                user: Self::user_to_response_with_details(user, user_details),
                access_token,
            },
            refresh_token,
        ))
    }

    /// Gets the refresh token expiry duration in seconds.
    ///
    /// This value is used for setting the max-age of the refresh token cookie.
    ///
    /// # Returns
    ///
    /// Refresh token expiry in seconds (default: 604800 = 7 days).
    pub fn get_refresh_token_expiry(&self) -> i64 {
        self.jwt_service.get_refresh_token_expiry()
    }

    /// Validates refresh token and generates a new access token.
    ///
    /// This method validates the refresh token from the cookie, verifies it's
    /// a refresh token (not an access token), and generates a new access token.
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - Refresh token string from the HTTP-only cookie
    ///
    /// # Returns
    ///
    /// Returns a new access token string.
    ///
    /// # Errors
    ///
    /// - `AppError::Unauthorized` if token is invalid, expired, or not a refresh token
    /// - `AppError::InternalError` if new access token generation fails
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<String, AppError> {
        // Validate the refresh token
        let claims = self.jwt_service.validate_token(refresh_token)
            .map_err(|e| AppError::Unauthorized(format!("Invalid refresh token: {}", e)))?;

        // Verify it's a refresh token (not an access token)
        if claims.token_type != "refresh" {
            return Err(AppError::Unauthorized("Token is not a refresh token".to_string()));
        }

        // Extract user_id from claims
        let user_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::Unauthorized(format!("Invalid user ID in token: {}", e)))?;

        // Verify user still exists in database
        let _user = self.repository.find_by_id(user_id).await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        // Generate new access token
        let new_access_token = self.jwt_service.generate_access_token(user_id)
            .map_err(|e| AppError::InternalError(format!("Failed to generate access token: {}", e)))?;

        Ok(new_access_token)
    }

    /// Extracts refresh token from request cookie and generates a new access token.
    ///
    /// This is a convenience method that handles cookie extraction and delegates
    /// to `refresh_token` for the actual token validation and generation.
    ///
    /// # Arguments
    ///
    /// * `req` - HTTP request containing the refresh token cookie
    ///
    /// # Returns
    ///
    /// Returns a new access token string.
    ///
    /// # Errors
    ///
    /// - `AppError::Unauthorized` if cookie is missing or token is invalid
    /// - `AppError::InternalError` if new access token generation fails
    pub async fn refresh_token_from_request(&self, req: &actix_web::HttpRequest) -> Result<String, AppError> {
        // Extract refresh token from cookie
        let refresh_token = req.cookie("refresh_token")
            .ok_or_else(|| AppError::Unauthorized("Refresh token not found".to_string()))?
            .value()
            .to_string();

        // Delegate to refresh_token method
        self.refresh_token(&refresh_token).await
    }

    /// Verifies if user exists in database by user_id.
    ///
    /// This method checks if the user (from JWT middleware) still exists in the database.
    /// Useful for the /auth/verify endpoint to ensure user hasn't been deleted.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User UUID (extracted from JWT by middleware)
    ///
    /// # Returns
    ///
    /// Returns `UserResponse` if user exists.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Forbidden` if user doesn't exist in database.
    pub async fn verify_user_exists(&self, user_id: uuid::Uuid) -> Result<UserResponse, AppError> {
        // Check if user exists in database
        let user = self.repository.find_by_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found in database".to_string()))?;

        // Fetch user_details
        let user_details = self.user_details_repository.find_by_user_id(user.id).await?;

        Ok(Self::user_to_response_with_details(user, user_details))
    }

    /// Extracts user_id from JWT token in HTTP request.
    ///
    /// This helper method retrieves the user_id that was injected into request
    /// extensions by the JWT authentication middleware.
    ///
    /// # Arguments
    ///
    /// * `req` - HTTP request containing user_id in extensions
    ///
    /// # Returns
    ///
    /// Returns the authenticated user's UUID.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Unauthorized` if user_id is not found in request.
    pub fn extract_user_id_from_request(req: &actix_web::HttpRequest) -> Result<uuid::Uuid, AppError> {
        req.extensions()
            .get::<uuid::Uuid>()
            .copied()
            .ok_or_else(|| AppError::Unauthorized(
                "User ID not found in token. Please ensure you are authenticated.".to_string()
            ))
    }

    /// Converts a User entity to UserResponse DTO.
    fn user_to_response(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
            details: None, // Will be populated when fetching with user_details
        }
    }

    /// Converts a User entity and UserDetails to UserResponse DTO with details.
    fn user_to_response_with_details(user: User, user_details: Option<UserDetails>) -> UserResponse {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
            details: user_details.map(|details| UserDetailsResponse {
                id: details.id,
                user_id: details.user_id,
                full_name: details.full_name,
                phone_number: details.phone_number,
                address: details.address,
                date_of_birth: details.date_of_birth,
                profile_picture_url: details.profile_picture_url,
                created_at: details.created_at,
                updated_at: details.updated_at,
            }),
        }
    }
}
