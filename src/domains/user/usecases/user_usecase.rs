use crate::domains::common::errors::AppError;
use crate::domains::tenant::repositories::user_tenant_repository::UserTenantRepositoryTrait;
use crate::domains::user::dtos::user_details_dto::UserDetailsResponse;
use crate::domains::user::dtos::user_dto::{UpdateUserRequest, UserResponse};
use crate::domains::user::entities::user::Model as User;
use crate::domains::user::entities::user_details::Model as UserDetails;
use crate::domains::user::repositories::user_details_repository::UserDetailsRepositoryTrait;
use crate::domains::user::repositories::user_repository::UserRepositoryTrait;
use crate::domains::user::validators::user_validator;
use std::sync::Arc;
use uuid::Uuid;

/// User use case orchestrating user management business logic.
///
/// This use case handles all user-related operations including creation, retrieval,
/// updates, and deletion. It coordinates between the repository layer and validators
/// to ensure data integrity and business rule compliance.
pub struct UserUseCase {
    repository: Arc<dyn UserRepositoryTrait>,
    user_details_repository: Arc<dyn UserDetailsRepositoryTrait>,
    user_tenant_repository: Arc<dyn UserTenantRepositoryTrait>,
}

impl UserUseCase {
    /// Creates a new UserUseCase instance.
    ///
    /// # Arguments
    ///
    /// * `repository` - Arc-wrapped user repository implementation
    /// * `user_details_repository` - Arc-wrapped user_details repository implementation
    /// * `user_tenant_repository` - Arc-wrapped user_tenant repository implementation
    pub fn new(
        repository: Arc<dyn UserRepositoryTrait>,
        user_details_repository: Arc<dyn UserDetailsRepositoryTrait>,
        user_tenant_repository: Arc<dyn UserTenantRepositoryTrait>,
    ) -> Self {
        Self {
            repository,
            user_details_repository,
            user_tenant_repository,
        }
    }

    /// Retrieves a user by their ID.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the user to retrieve
    ///
    /// # Returns
    ///
    /// Returns `UserResponse` with nested user_details if found, or `AppError::NotFound` if the user doesn't exist.
    pub async fn get_user(&self, id: Uuid, tenant_id: Uuid) -> Result<UserResponse, AppError> {
        let user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))?;

        // Fetch user_details
        let user_details = self
            .user_details_repository
            .find_by_user_id(user.id)
            .await?;

        // Fetch role
        let role_result = self
            .user_tenant_repository
            .get_user_role_in_tenant(user.id, tenant_id)
            .await?;
        log::info!(
            "get_user: user_id={}, tenant_id={}, fetched_role={:?}",
            user.id,
            tenant_id,
            role_result
        );

        let role = role_result.unwrap_or_else(|| "user".to_string());

        Ok(Self::user_to_response(user, user_details, role))
    }

    /// Retrieves all users from the database.
    ///
    /// # Returns
    ///
    /// Returns a vector of `UserResponse` containing all users with their user_details.
    pub async fn get_all_users(
        &self,
        tenant_id: Uuid,
        requesting_user_role: &str,
    ) -> Result<Vec<UserResponse>, AppError> {
        log::info!("get_all_users: requesting_role='{}'", requesting_user_role);
        if requesting_user_role != "admin" {
            return Err(AppError::Forbidden("Forbidden".to_string()));
        }

        let users = self.repository.find_all().await?;

        let mut responses = Vec::new();
        for user in users {
            let user_details = self
                .user_details_repository
                .find_by_user_id(user.id)
                .await?;
            let role = self
                .user_tenant_repository
                .get_user_role_in_tenant(user.id, tenant_id)
                .await?
                .unwrap_or_else(|| "user".to_string());
            responses.push(Self::user_to_response(user, user_details, role));
        }

        Ok(responses)
    }

    /// Updates an existing user.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the user to update
    /// * `req` - Update request containing optional fields to update
    ///
    /// # Returns
    ///
    /// Returns updated `UserResponse` or `AppError` if validation fails or user not found.
    pub async fn update_user(
        &self,
        id: Uuid,
        req: UpdateUserRequest,
        tenant_id: Uuid,
    ) -> Result<UserResponse, AppError> {
        log::info!("update_user: req={:?}", req);
        // Validate input if provided
        if let Some(ref username) = req.username {
            log::info!("Validating username: {}", username);
            user_validator::validate_username(username)?;
        }
        if let Some(ref email) = req.email {
            user_validator::validate_email(email)?;
        }
        if let Some(ref password) = req.password {
            user_validator::validate_password(password, "password")?;
        }

        // Update user via repository
        let user = self.repository.update(id, req).await?;

        // Fetch user_details
        let user_details = self
            .user_details_repository
            .find_by_user_id(user.id)
            .await?;

        // Fetch role
        let role = self
            .user_tenant_repository
            .get_user_role_in_tenant(user.id, tenant_id)
            .await?
            .unwrap_or_else(|| "user".to_string());

        Ok(Self::user_to_response(user, user_details, role))
    }

    /// Deletes a user by their ID.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the user to delete
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if successful, or `AppError::NotFound` if user doesn't exist.
    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await
    }

    /// Converts a User entity to UserResponse DTO with optional user_details.
    /// Converts relative profile picture paths to full URLs.
    fn user_to_response(
        user: User,
        user_details: Option<UserDetails>,
        role: String,
    ) -> UserResponse {
        use crate::domains::common::utils::url_helper::to_full_url;

        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role,
            details: user_details.map(|details| {
                let (first, last) = match details.full_name {
                    Some(s) => {
                        let parse_name = |name: String| -> (Option<String>, Option<String>) {
                            if let Some((f, l)) = name.split_once(' ') {
                                return (Some(f.to_string()), Some(l.to_string()));
                            }
                            (Some(name), None)
                        };
                        parse_name(s)
                    }
                    None => (None, None),
                };

                UserDetailsResponse {
                    id: details.id,
                    user_id: details.user_id,
                    first_name: first,
                    last_name: last,
                    phone_number: details.phone_number,
                    address: details.address,
                    date_of_birth: details.date_of_birth,
                    profile_picture_url: to_full_url(details.profile_picture_url),
                    created_at: details.created_at,
                    updated_at: details.updated_at,
                }
            }),
        }
    }
}
