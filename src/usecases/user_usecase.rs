use crate::dtos::user_dto::{UpdateUserRequest, UserResponse};
use crate::dtos::user_details_dto::UserDetailsResponse;
use crate::entities::user::Model as User;
use crate::entities::user_details::Model as UserDetails;
use crate::errors::AppError;
use crate::repositories::user_repository::UserRepositoryTrait;
use crate::repositories::user_details_repository::UserDetailsRepositoryTrait;
use crate::validators::user_validator;
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
}

impl UserUseCase {
    /// Creates a new UserUseCase instance.
    ///
    /// # Arguments
    ///
    /// * `repository` - Arc-wrapped user repository implementation
    /// * `user_details_repository` - Arc-wrapped user_details repository implementation
    pub fn new(
        repository: Arc<dyn UserRepositoryTrait>,
        user_details_repository: Arc<dyn UserDetailsRepositoryTrait>,
    ) -> Self {
        Self { 
            repository,
            user_details_repository,
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
    pub async fn get_user(&self, id: Uuid) -> Result<UserResponse, AppError> {
        let user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))?;

        // Fetch user_details
        let user_details = self.user_details_repository.find_by_user_id(user.id).await?;

        Ok(Self::user_to_response(user, user_details))
    }

    /// Retrieves all users from the database.
    ///
    /// # Returns
    ///
    /// Returns a vector of `UserResponse` containing all users with their user_details.
    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, AppError> {
        let users = self.repository.find_all().await?;

        let mut responses = Vec::new();
        for user in users {
            let user_details = self.user_details_repository.find_by_user_id(user.id).await?;
            responses.push(Self::user_to_response(user, user_details));
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
    ) -> Result<UserResponse, AppError> {
        // Validate input if provided
        if let Some(ref username) = req.username {
            user_validator::validate_username(username)?;
        }
        if let Some(ref email) = req.email {
            user_validator::validate_email(email)?;
        }
        if let Some(ref password) = req.password {
            user_validator::validate_password(password)?;
        }

        // Update user via repository
        let user = self.repository.update(id, req).await?;

        // Fetch user_details
        let user_details = self.user_details_repository.find_by_user_id(user.id).await?;

        Ok(Self::user_to_response(user, user_details))
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
    fn user_to_response(user: User, user_details: Option<UserDetails>) -> UserResponse {
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
