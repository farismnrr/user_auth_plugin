use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::dtos::user_details_dto::UserDetailsResponse;

/// Request DTO for creating a new user.
///
/// All fields are required for user creation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

/// Request DTO for updating an existing user.
///
/// All fields are optional. Only provided fields will be updated.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
}

/// Response DTO for user data.
///
/// This DTO excludes sensitive information like password hash.
/// Used for all user-related API responses.
/// Includes nested `details` object containing user_details data.
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub details: Option<UserDetailsResponse>,
}
