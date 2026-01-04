
use super::auth_middleware::validator;
use actix_web::{test, http, dev::ServiceRequest, FromRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::HttpMessage;

// Since validator uses internal JwtService::new() which reads env vars,
// we must setup env vars for the test.

#[actix_web::test]
async fn test_auth_middleware_validator_success() {
    // 1. Setup Env for JwtService
    std::env::set_var("JWT_SECRET", "test_secret_key_12345");
    std::env::set_var("JWT_ACCESS_TOKEN_EXPIRY", "60");
    std::env::set_var("JWT_REFRESH_TOKEN_EXPIRY", "120");
    
    // 2. Generate a valid token
    use crate::domains::common::utils::jwt::JwtService;
    let jwt_service = JwtService::new();
    let user_id = uuid::Uuid::new_v4();
    let tenant_id = uuid::Uuid::new_v4();
    let token = jwt_service.generate_access_token(user_id, tenant_id, "user".to_string()).unwrap();

    // 3. Create Request
    let req = test::TestRequest::default()
        .insert_header((http::header::AUTHORIZATION, format!("Bearer {}", token)))
        .to_srv_request();
    
    // 4. Extract BearerAuth (manually or via extractor)
    // The validator func takes `BearerAuth` struct. We can construct it via FromRequest 
    // or just mock if we could, but BearerAuth is opaque.
    // Easier way: Use it as an extractor in a test handler? 
    // BUT the file exports `validator` function directly.
    // `BearerAuth` typically implements `FromRequest`.
    
    let (req, mut payload) = req.into_parts();
    let bearer_auth = BearerAuth::from_request(&req, &mut payload).await.unwrap();
    let req = ServiceRequest::from_parts(req, payload);

    // 5. Call validator
    let result = validator(req, bearer_auth).await;
    
    // 6. Assert success
    assert!(result.is_ok());
    let req = result.unwrap();
    
    // 7. Check if user_id is injected
    let injected_user_id = req.extensions().get::<uuid::Uuid>().cloned();
    assert_eq!(injected_user_id, Some(user_id));
}

#[actix_web::test]
async fn test_auth_middleware_validator_invalid_token() {
    // 1. Setup Env
    std::env::set_var("JWT_SECRET", "test_secret_key_12345");
    
    // 2. Create Request with invalid token
    let req = test::TestRequest::default()
        .insert_header((http::header::AUTHORIZATION, "Bearer invalid_token"))
        .to_srv_request();
        
    let (req, mut payload) = req.into_parts();
    let bearer_auth = BearerAuth::from_request(&req, &mut payload).await.unwrap();
    let req = ServiceRequest::from_parts(req, payload);

    // 3. Call validator
    let result = validator(req, bearer_auth).await;
    
    // 4. Assert failure
    assert!(result.is_err());
    let (err, _) = result.err().unwrap();
    
    // 5. Verify error response
    let resp = err.error_response();
    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
}
