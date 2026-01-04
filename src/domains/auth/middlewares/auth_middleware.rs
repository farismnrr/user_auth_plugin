use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use uuid::Uuid;
use crate::domains::common::utils::jwt::JwtService;

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
            // Check if tenant_id matches the one in request extensions (from ApiKeyMiddleware)
            let tenant_id_str = req.extensions()
                .get::<crate::domains::tenant::middlewares::api_key_middleware::TenantId>()
                .map(|t| t.0.to_string());
            
            if let Some(tid_str) = tenant_id_str {
               if claims.tenant_id != tid_str {
                   let err = actix_web::error::InternalError::from_response(
                        "Unauthorized", 
                        actix_web::HttpResponse::Forbidden()
                            .content_type("application/json")
                            .body(r#"{"status":false,"message":"Unauthorized"}"#)
                    ).into();
                    return Err((err, req));
               }
            }

            // Parse user_id from claims
            match Uuid::parse_str(&claims.sub) {
                Ok(user_id) => {
                    // Inject user_id into request extensions
                    req.extensions_mut().insert(user_id);
                    Ok(req)
                }
                Err(_) => {
                    let err = actix_web::error::InternalError::from_response(
                        "Invalid user ID", 
                        actix_web::HttpResponse::Unauthorized()
                            .content_type("application/json")
                            .body(r#"{"status":false,"message":"Unauthorized","details":"Invalid user ID","result":null}"#)
                    ).into();
                    Err((err, req))
                }
            }
        }
        Err(e) => {
            let mut message = "Unauthorized";
            if e.to_string().contains("ExpiredSignature") {
                message = "Token expired";
            }

            let err = actix_web::error::InternalError::from_response(
                message, 
                actix_web::HttpResponse::Unauthorized()
                    .content_type("application/json")
                    .body(format!(r#"{{"status":false,"message":"{}"}}"#, message))
            ).into();
            Err((err, req))
        }
    }
}

