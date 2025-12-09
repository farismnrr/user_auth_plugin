use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use uuid::Uuid;
use crate::utils::jwt::JwtService;

/// Extracts and validates JWT token, then injects user_id into request extensions.
///
/// This function is used by the JWT auth middleware to authenticate requests.
/// It extracts the Bearer token from the Authorization header, validates it,
/// and injects the user_id into the request extensions for controllers to access.
///
/// # Arguments
///
/// * `req` - The service request
/// * `credentials` - Bearer token credentials
///
/// # Returns
///
/// Returns `Ok(())` if authentication succeeds, or an error if it fails.
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let jwt_service = JwtService::new();
    
    match jwt_service.validate_token(token) {
        Ok(claims) => {
            // Parse user_id from claims
            match Uuid::parse_str(&claims.sub) {
                Ok(user_id) => {
                    // Inject user_id into request extensions
                    req.extensions_mut().insert(user_id);
                    Ok(req)
                }
                Err(_) => Err((ErrorUnauthorized("Invalid user ID in token"), req)),
            }
        }
        Err(e) => Err((ErrorUnauthorized(format!("Invalid token: {}", e)), req)),
    }
}
