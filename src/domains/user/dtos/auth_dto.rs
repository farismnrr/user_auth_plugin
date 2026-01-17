use serde::{Deserialize, Serialize};

/// Request DTO for user login (JSON Body).
///
/// tenant_id is extracted from headers/middleware.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequestJson {
    pub email_or_username: String,
    pub password: String,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub redirect_uri: Option<String>,
    pub role: Option<String>,
}

/// Request DTO for user login (Internal).
///
/// Accepts either email or username for authentication.
/// tenant_id is required to validate user's access to specific tenant.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email_or_username: String,
    pub password: String,
    pub tenant_id: uuid::Uuid,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub redirect_uri: Option<String>,
    pub role: Option<String>,
}

/// Request DTO for user registration (JSON Body).
///
/// tenant_id is extracted from headers/middleware, so it's not in the body.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterRequestJson {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub redirect_uri: Option<String>,
    pub invitation_code: Option<String>,
}

/// Request DTO for user registration (Internal).
///
/// All fields are required for creating a new user account.
/// tenant_id and role are used to assign user to a tenant with specific role.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub tenant_id: uuid::Uuid,
    pub role: String,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub redirect_uri: Option<String>,
    pub invitation_code: Option<String>,
}

/// Authentication response containing only access token.
///
/// The refresh token is not included in the response body as it is set
/// as an HTTP-only cookie for security purposes.
/// User information can be retrieved via /auth/verify endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub user_id: uuid::Uuid,
    pub access_token: String,
}

/// Query parameters for SSO logout endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SsoLogoutQuery {
    pub redirect_uri: Option<String>,
}
