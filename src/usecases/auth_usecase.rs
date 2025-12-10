use crate::dtos::auth_dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::dtos::change_password_dto::ChangePasswordRequest;
use crate::dtos::user_dto::{CreateUserRequest, UserResponse};
use crate::dtos::user_details_dto::UserDetailsResponse;
use crate::entities::user::Model as User;
use crate::entities::user_details::Model as UserDetails;
use crate::errors::AppError;
use crate::repositories::user_activity_log_repository::UserActivityLogRepositoryTrait;
use crate::repositories::user_repository::UserRepositoryTrait;
use crate::repositories::user_session_repository::UserSessionRepositoryTrait;
use crate::utils::{jwt::JwtService, password, request_helper};
use crate::validators::user_validator;
use actix_web::HttpMessage;
use chrono::Utc;
use std::sync::Arc;

/// Authentication use case handling user registration, login, and session management.
///
/// This use case manages authentication operations including user registration,
/// login with password verification, JWT token generation, session tracking,
/// and comprehensive activity logging for audit trails.
pub struct AuthUseCase {
    repository: Arc<dyn UserRepositoryTrait>,
    user_details_repository:
        Arc<dyn crate::repositories::user_details_repository::UserDetailsRepositoryTrait>,
    session_repository: Arc<dyn UserSessionRepositoryTrait>,
    activity_log_repository: Arc<dyn UserActivityLogRepositoryTrait>,
    jwt_service: JwtService,
}

impl AuthUseCase {
    /// Creates a new AuthUseCase instance.
    ///
    /// # Arguments
    ///
    /// * `repository` - Arc-wrapped user repository implementation
    /// * `user_details_repository` - Arc-wrapped user_details repository implementation
    /// * `session_repository` - Arc-wrapped session repository implementation
    /// * `activity_log_repository` - Arc-wrapped activity log repository implementation
    pub fn new(
        repository: Arc<dyn UserRepositoryTrait>,
        user_details_repository: Arc<
            dyn crate::repositories::user_details_repository::UserDetailsRepositoryTrait,
        >,
        session_repository: Arc<dyn UserSessionRepositoryTrait>,
        activity_log_repository: Arc<dyn UserActivityLogRepositoryTrait>,
    ) -> Self {
        Self {
            repository,
            user_details_repository,
            session_repository,
            activity_log_repository,
            jwt_service: JwtService::new(),
        }
    }

    /// Registers a new user and generates authentication tokens.
    ///
    /// # Arguments
    ///
    /// * `req` - Registration request containing username, email, password, and role
    /// * `http_req` - HTTP request for extracting client info
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
    pub async fn register(
        &self,
        req: RegisterRequest,
        http_req: &actix_web::HttpRequest,
    ) -> Result<AuthResponse, AppError> {
        let (ip_address, user_agent) = request_helper::extract_client_info(http_req);

        // Validate input
        if let Err(e) = user_validator::validate_username(&req.username) {
            self.log_activity_failure(None, "register", &e, ip_address.clone(), user_agent.clone())
                .await;
            return Err(e);
        }
        if let Err(e) = user_validator::validate_email(&req.email) {
            self.log_activity_failure(None, "register", &e, ip_address.clone(), user_agent.clone())
                .await;
            return Err(e);
        }
        if let Err(e) = user_validator::validate_password(&req.password) {
            self.log_activity_failure(None, "register", &e, ip_address.clone(), user_agent.clone())
                .await;
            return Err(e);
        }

        // Check if email already exists
        if self.repository.find_by_email(&req.email).await?.is_some() {
            let err = AppError::Conflict("Email already registered".to_string());
            self.log_activity_failure(None, "register", &err, ip_address, user_agent)
                .await;
            return Err(err);
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
        let access_token = self
            .jwt_service
            .generate_access_token(user.id)
            .map_err(|e| {
                AppError::InternalError(format!("Failed to generate access token: {}", e))
            })?;

        let _refresh_token = self
            .jwt_service
            .generate_refresh_token(user.id)
            .map_err(|e| {
                AppError::InternalError(format!("Failed to generate refresh token: {}", e))
            })?;

        // Log successful registration
        self.log_activity_success(Some(user.id), "register", ip_address, user_agent)
            .await;

        Ok(AuthResponse {
            user: Self::user_to_response(user),
            access_token,
        })
    }

    /// Authenticates a user and generates tokens with session tracking.
    ///
    /// Accepts either email or username for login.
    ///
    /// # Arguments
    ///
    /// * `req` - Login request containing email_or_username and password
    /// * `http_req` - HTTP request for extracting client info
    ///
    /// # Returns
    ///
    /// Returns a tuple of `(AuthResponse, refresh_token)`.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Unauthorized` if credentials are invalid.
    pub async fn login(
        &self,
        req: LoginRequest,
        http_req: &actix_web::HttpRequest,
    ) -> Result<(AuthResponse, String), AppError> {
        let (ip_address, user_agent) = request_helper::extract_client_info(http_req);

        // Try to find user by email first
        let user = match self.repository.find_by_email(&req.email_or_username).await? {
            Some(u) => u,
            None => {
                // If not found by email, try username
                self.repository
                    .find_by_username(&req.email_or_username)
                    .await?
                    .ok_or_else(|| {
                        let err = AppError::Unauthorized("Invalid credentials".to_string());
                        err
                    })?
            }
        };

        // Verify password
        if !password::verify_password(&req.password, &user.password_hash)? {
            let err = AppError::Unauthorized("Invalid credentials".to_string());
            self.log_activity_failure(
                Some(user.id),
                "login",
                &err,
                ip_address,
                user_agent,
            )
            .await;
            return Err(err);
        }

        // Generate tokens
        let access_token = self
            .jwt_service
            .generate_access_token(user.id)
            .map_err(|e| {
                AppError::InternalError(format!("Failed to generate access token: {}", e))
            })?;

        let refresh_token = self
            .jwt_service
            .generate_refresh_token(user.id)
            .map_err(|e| {
                AppError::InternalError(format!("Failed to generate refresh token: {}", e))
            })?;

        // Create session record
        let refresh_token_hash = request_helper::hash_token(&refresh_token);
        let expires_at = Utc::now()
            + chrono::Duration::seconds(self.jwt_service.get_refresh_token_expiry());

        self.session_repository
            .create_session(
                user.id,
                refresh_token_hash,
                user_agent.clone(),
                ip_address.clone(),
                expires_at,
            )
            .await?;

        // Fetch user_details
        let user_details = self.user_details_repository.find_by_user_id(user.id).await?;

        // Log successful login
        self.log_activity_success(Some(user.id), "login", ip_address, user_agent)
            .await;

        Ok((
            AuthResponse {
                user: Self::user_to_response_with_details(user, user_details),
                access_token,
            },
            refresh_token,
        ))
    }

    /// Logs out a user by deleting their session.
    ///
    /// This endpoint requires JWT authentication and a valid refresh token cookie.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID from JWT token
    /// * `http_req` - HTTP request containing refresh token cookie
    ///
    /// # Errors
    ///
    /// Returns `AppError::Unauthorized` if refresh token is missing or invalid.
    pub async fn logout(
        &self,
        user_id: uuid::Uuid,
        http_req: &actix_web::HttpRequest,
    ) -> Result<(), AppError> {
        let (ip_address, user_agent) = request_helper::extract_client_info(http_req);

        // Extract refresh token from cookie
        let refresh_token = http_req
            .cookie("refresh_token")
            .ok_or_else(|| AppError::Unauthorized("Refresh token not found".to_string()))?
            .value()
            .to_string();

        // Hash the refresh token to find the session
        let refresh_token_hash = request_helper::hash_token(&refresh_token);

        // Find and delete the session
        if let Some(session) = self
            .session_repository
            .find_by_refresh_token_hash(&refresh_token_hash)
            .await?
        {
            self.session_repository.delete_session(session.id).await?;
        }

        // Log successful logout
        self.log_activity_success(Some(user_id), "logout", ip_address, user_agent)
            .await;

        Ok(())
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
    /// a refresh token (not an access token), checks the session exists, and
    /// generates a new access token.
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
        let claims = self
            .jwt_service
            .validate_token(refresh_token)
            .map_err(|e| AppError::Unauthorized(format!("Invalid refresh token: {}", e)))?;

        // Verify it's a refresh token (not an access token)
        if claims.token_type != "refresh" {
            return Err(AppError::Unauthorized(
                "Token is not a refresh token".to_string(),
            ));
        }

        // Extract user_id from claims
        let user_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::Unauthorized(format!("Invalid user ID in token: {}", e)))?;

        // Verify session exists
        let refresh_token_hash = request_helper::hash_token(refresh_token);
        self.session_repository
            .find_by_refresh_token_hash(&refresh_token_hash)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Session not found or expired".to_string()))?;

        // Verify user still exists in database
        let _user = self
            .repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        // Generate new access token
        let new_access_token = self
            .jwt_service
            .generate_access_token(user_id)
            .map_err(|e| {
                AppError::InternalError(format!("Failed to generate access token: {}", e))
            })?;

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
    pub async fn refresh_token_from_request(
        &self,
        req: &actix_web::HttpRequest,
    ) -> Result<String, AppError> {
        // Extract refresh token from cookie
        let refresh_token = req
            .cookie("refresh_token")
            .ok_or_else(|| AppError::Unauthorized("Refresh token not found".to_string()))?
            .value()
            .to_string();

        // Delegate to refresh_token method
        self.refresh_token(&refresh_token).await
    }

    /// Changes a user's password.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID from JWT token
    /// * `req` - Change password request with old and new passwords
    /// * `http_req` - HTTP request for extracting client info
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if password was changed successfully.
    ///
    /// # Errors
    ///
    /// - `AppError::Unauthorized` if old password is incorrect
    /// - `AppError::ValidationError` if passwords don't match or are invalid
    pub async fn change_password(
        &self,
        user_id: uuid::Uuid,
        req: ChangePasswordRequest,
        http_req: &actix_web::HttpRequest,
    ) -> Result<(), AppError> {
        let (ip_address, user_agent) = request_helper::extract_client_info(http_req);

        // Validate new password matches confirmation
        if req.new_password != req.confirm_new_password {
            let err = AppError::ValidationError("New passwords do not match".to_string());
            self.log_activity_failure(
                Some(user_id),
                "change_password",
                &err,
                ip_address,
                user_agent,
            )
            .await;
            return Err(err);
        }

        // Validate new password strength
        if let Err(e) = user_validator::validate_password(&req.new_password) {
            self.log_activity_failure(
                Some(user_id),
                "change_password",
                &e,
                ip_address.clone(),
                user_agent.clone(),
            )
            .await;
            return Err(e);
        }

        // Get user from database
        let user = self
            .repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Verify old password
        if !password::verify_password(&req.old_password, &user.password_hash)? {
            let err = AppError::Unauthorized("Old password is incorrect".to_string());
            self.log_activity_failure(
                Some(user_id),
                "change_password",
                &err,
                ip_address,
                user_agent,
            )
            .await;
            return Err(err);
        }

        // Update password
        let update_req = crate::dtos::user_dto::UpdateUserRequest {
            username: None,
            email: None,
            password: Some(req.new_password),
            role: None,
        };
        self.repository.update(user_id, update_req).await?;

        // Log successful password change
        self.log_activity_success(Some(user_id), "change_password", ip_address, user_agent)
            .await;

        Ok(())
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
    pub async fn verify_user_exists(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<UserResponse, AppError> {
        // Check if user exists in database
        let user = self
            .repository
            .find_by_id(user_id)
            .await?
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
    pub fn extract_user_id_from_request(
        req: &actix_web::HttpRequest,
    ) -> Result<uuid::Uuid, AppError> {
        req.extensions()
            .get::<uuid::Uuid>()
            .copied()
            .ok_or_else(|| {
                AppError::Unauthorized(
                    "User ID not found in token. Please ensure you are authenticated.".to_string(),
                )
            })
    }

    /// Logs a successful activity.
    async fn log_activity_success(
        &self,
        user_id: Option<uuid::Uuid>,
        activity_type: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) {
        let _ = self
            .activity_log_repository
            .log_activity(
                user_id,
                activity_type.to_string(),
                "success".to_string(),
                None,
                ip_address,
                user_agent,
            )
            .await;
    }

    /// Logs a failed activity.
    async fn log_activity_failure(
        &self,
        user_id: Option<uuid::Uuid>,
        activity_type: &str,
        error: &AppError,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) {
        let _ = self
            .activity_log_repository
            .log_activity(
                user_id,
                activity_type.to_string(),
                "failure".to_string(),
                Some(error.to_string()),
                ip_address,
                user_agent,
            )
            .await;
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
    fn user_to_response_with_details(
        user: User,
        user_details: Option<UserDetails>,
    ) -> UserResponse {
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
