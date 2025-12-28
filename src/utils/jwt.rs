use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT claims structure.
///
/// # Fields
///
/// * `sub` - Subject (user ID)
/// * `tenant_id` - Tenant ID for tenant-scoped authentication
/// * `role` - User's role within the tenant
/// * `exp` - Expiration time (Unix timestamp)
/// * `iat` - Issued at (Unix timestamp)
/// * `token_type` - Token type ("access" or "refresh")
/// * `jti` - JWT ID (optional, for refresh tokens to ensure uniqueness)
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub tenant_id: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
}

/// JWT token service for generating and validating tokens.
///
/// This service handles JWT token generation for both access and refresh tokens,
/// with configurable expiry times from environment variables.
pub struct JwtService {
    secret: String,
    access_token_expiry: i64,
    refresh_token_expiry: i64,
}

impl Default for JwtService {
    fn default() -> Self {
        Self::new()
    }
}

impl JwtService {
    /// Creates a new JwtService instance.
    ///
    /// Reads configuration from environment variables:
    /// - `JWT_SECRET`: Secret key for signing tokens (default: "default-secret-key")
    /// - `JWT_ACCESS_TOKEN_EXPIRY`: Access token lifetime in seconds (default: 900)
    /// - `JWT_REFRESH_TOKEN_EXPIRY`: Refresh token lifetime in seconds (default: 604800)
    pub fn new() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-secret-key".to_string());
        
        let access_token_expiry = std::env::var("JWT_ACCESS_TOKEN_EXPIRY")
            .unwrap_or_else(|_| "900".to_string())
            .parse::<i64>()
            .unwrap_or(900);
        
        let refresh_token_expiry = std::env::var("JWT_REFRESH_TOKEN_EXPIRY")
            .unwrap_or_else(|_| "604800".to_string())
            .parse::<i64>()
            .unwrap_or(604800);

        Self {
            secret,
            access_token_expiry,
            refresh_token_expiry,
        }
    }

    /// Generates an access token for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user
    /// * `tenant_id` - UUID of the tenant
    /// * `role` - User's role within the tenant
    ///
    /// # Returns
    ///
    /// Returns a JWT access token string valid for the configured duration.
    ///
    /// # Errors
    ///
    /// Returns `jsonwebtoken::errors::Error` if token encoding fails.
    pub fn generate_access_token(&self, user_id: Uuid, tenant_id: Uuid, role: String) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.access_token_expiry);

        let claims = Claims {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            role,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
            jti: None, // Access tokens don't need JTI
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    /// Generates a refresh token for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user
    /// * `tenant_id` - UUID of the tenant
    /// * `role` - User's role within the tenant
    ///
    /// # Returns
    ///
    /// Returns a JWT refresh token string valid for the configured duration.
    /// This token should be stored as an HTTP-only cookie.
    ///
    /// # Errors
    ///
    /// Returns `jsonwebtoken::errors::Error` if token encoding fails.
    pub fn generate_refresh_token(&self, user_id: Uuid, tenant_id: Uuid, role: String) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.refresh_token_expiry);

        let claims = Claims {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            role,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
            jti: Some(Uuid::new_v4().to_string()), // Unique ID for each refresh token
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    /// Validates and decodes a JWT token.
    ///
    /// # Arguments
    ///
    /// * `token` - JWT token string to validate
    ///
    /// # Returns
    ///
    /// Returns decoded `Claims` if token is valid.
    ///
    /// # Errors
    ///
    /// Returns `jsonwebtoken::errors::Error` if token is invalid or expired.
    #[allow(dead_code)]
    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_nbf = true;
        validation.leeway = 0; // Strict checking
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )?;

        Ok(token_data.claims)
    }

    /// Gets the refresh token expiry duration in seconds.
    ///
    /// This value is used for setting the max-age of the refresh token cookie.
    ///
    /// # Returns
    ///
    /// Refresh token expiry in seconds (configured via `JWT_REFRESH_TOKEN_EXPIRY`).
    pub fn get_refresh_token_expiry(&self) -> i64 {
        self.refresh_token_expiry
    }
}
