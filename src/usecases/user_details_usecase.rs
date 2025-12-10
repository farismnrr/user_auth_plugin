use crate::dtos::user_details_dto::{UpdateUserDetailsRequest, UserDetailsResponse};
use crate::entities::user_details::Model as UserDetails;
use crate::errors::AppError;
use crate::repositories::user_details_repository::UserDetailsRepositoryTrait;
use std::sync::Arc;
use uuid::Uuid;
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::io::Write;

/// User details use case handling user_details operations.
///
/// This use case manages user_details updates including text fields and profile picture uploads.
pub struct UserDetailsUseCase {
    repository: Arc<dyn UserDetailsRepositoryTrait>,
}

impl UserDetailsUseCase {
    /// Creates a new UserDetailsUseCase instance.
    ///
    /// # Arguments
    ///
    /// * `repository` - Arc-wrapped user_details repository implementation
    pub fn new(repository: Arc<dyn UserDetailsRepositoryTrait>) -> Self {
        Self { repository }
    }

    /// Updates user_details text fields.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user whose details to update
    /// * `req` - Update request containing optional fields
    ///
    /// # Returns
    ///
    /// Returns updated `UserDetailsResponse` or `AppError` if user_details not found.
    pub async fn update_user_details(
        &self,
        user_id: Uuid,
        req: UpdateUserDetailsRequest,
    ) -> Result<UserDetailsResponse, AppError> {
        let user_details = self.repository.update(
            user_id,
            req.full_name,
            req.phone_number,
            req.address,
            req.date_of_birth,
        ).await?;

        Ok(Self::to_response(user_details))
    }

    /// Get user details by user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user whose details to retrieve
    ///
    /// # Returns
    ///
    /// Returns `UserDetailsResponse` or `AppError` if user_details not found.
    pub async fn get_user_details(
        &self,
        user_id: Uuid,
    ) -> Result<UserDetailsResponse, AppError> {
        let user_details = self.repository.find_by_user_id(user_id).await?
            .ok_or_else(|| AppError::NotFound(format!("User details not found for user {}", user_id)))?;
        
        Ok(Self::to_response(user_details))
    }

    /// Updates profile picture.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user whose profile picture to update
    /// * `mut payload` - Multipart form data containing the image file
    ///
    /// # Returns
    ///
    /// Returns updated `UserDetailsResponse` or `AppError` if upload fails.
    pub async fn update_profile_picture(
        &self,
        user_id: Uuid,
        mut payload: Multipart,
    ) -> Result<UserDetailsResponse, AppError> {
        const MAX_FILE_SIZE: usize = 1_048_576; // 1MB

        if let Some(item) = payload.next().await {
            let mut field = item.map_err(|e| AppError::BadRequest(format!("Failed to read multipart field: {}", e)))?;

            let content_disposition = field.content_disposition();
            let filename = content_disposition
                .get_filename()
                .ok_or_else(|| AppError::BadRequest("No filename provided".to_string()))?;

            // Validate file extension
            if !filename.ends_with(".png") && !filename.ends_with(".jpg") && !filename.ends_with(".jpeg") {
                return Err(AppError::BadRequest("Only PNG and JPEG images are allowed".to_string()));
            }

            // Generate unique filename
            let file_ext = filename.rsplit('.').next().unwrap_or("png");
            let new_filename = format!("{}_{}.{}", user_id, chrono::Utc::now().timestamp(), file_ext);
            let filepath = format!("assets/profiles/{}", new_filename);

            // Read file data
            let mut file_data = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| AppError::BadRequest(format!("Failed to read file chunk: {}", e)))?;
                file_data.extend_from_slice(&data);

                // Check file size
                if file_data.len() > MAX_FILE_SIZE {
                    return Err(AppError::BadRequest("File size exceeds 1MB limit".to_string()));
                }
            }

            // Save file
            let mut file = std::fs::File::create(&filepath)
                .map_err(|e| AppError::InternalError(format!("Failed to create file: {}", e)))?;
            file.write_all(&file_data)
                .map_err(|e| AppError::InternalError(format!("Failed to write file: {}", e)))?;

            // Set file permissions to 644 (readable by all, writable by owner)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let permissions = std::fs::Permissions::from_mode(0o644);
                std::fs::set_permissions(&filepath, permissions)
                    .map_err(|e| AppError::InternalError(format!("Failed to set file permissions: {}", e)))?;
            }

            // Update profile picture URL in database
            let profile_picture_url = format!("/{}", filepath);
            log::info!("Profile picture uploaded successfully: {} for user {}", profile_picture_url, user_id);
            let user_details = self.repository.update_profile_picture(user_id, profile_picture_url).await?;

            return Ok(Self::to_response(user_details));
        }

        Err(AppError::BadRequest("No file provided".to_string()))
    }

    /// Converts UserDetails entity to UserDetailsResponse DTO.
    /// Converts relative profile picture paths to full URLs.
    fn to_response(user_details: UserDetails) -> UserDetailsResponse {
        use crate::utils::url_helper::to_full_url;

        UserDetailsResponse {
            id: user_details.id,
            user_id: user_details.user_id,
            full_name: user_details.full_name,
            phone_number: user_details.phone_number,
            address: user_details.address,
            date_of_birth: user_details.date_of_birth,
            profile_picture_url: to_full_url(user_details.profile_picture_url),
            created_at: user_details.created_at,
            updated_at: user_details.updated_at,
        }
    }
}
