use serde::{Deserialize, Serialize};
use crate::dtos::user_dto::UserResponse;

/// Request DTO for user login.
///
/// Accepts either email or username for authentication.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email_or_username: String,
    pub password: String,
}

/// Request DTO for user registration.
///
/// All fields are required for creating a new user account.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

/// Authentication response containing user information and access token.
///
/// The refresh token is not included in the response body as it is set
/// as an HTTP-only cookie for security purposes.
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
}
