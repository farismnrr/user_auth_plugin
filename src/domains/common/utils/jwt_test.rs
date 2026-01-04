
use super::jwt::*;
use std::{thread, time};
use uuid::Uuid;

fn setup_env() {
    std::env::set_var("JWT_SECRET", "test_secret");
    std::env::set_var("JWT_ACCESS_TOKEN_EXPIRY", "1");
    std::env::set_var("JWT_REFRESH_TOKEN_EXPIRY", "2");
}

#[test]
fn test_generate_and_validate_access_token() {
    setup_env();
    let jwt_service = JwtService::new();
    let user_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();
    let role = "user".to_string();

    let token = jwt_service.generate_access_token(user_id, tenant_id, role.clone()).unwrap();
    let claims = jwt_service.validate_token(&token).unwrap();

    assert_eq!(claims.sub, user_id.to_string());
    assert_eq!(claims.tenant_id, tenant_id.to_string());
    assert_eq!(claims.role, role);
    assert_eq!(claims.token_type, "access");
}

#[test]
fn test_generate_and_validate_refresh_token() {
    setup_env();
    let jwt_service = JwtService::new();
    let user_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();
    let role = "admin".to_string();

    let token = jwt_service.generate_refresh_token(user_id, tenant_id, role.clone()).unwrap();
    let claims = jwt_service.validate_token(&token).unwrap();

    assert_eq!(claims.sub, user_id.to_string());
    assert_eq!(claims.tenant_id, tenant_id.to_string());
    assert_eq!(claims.role, role);
    assert_eq!(claims.token_type, "refresh");
    assert!(claims.jti.is_some());
}

#[test]
fn test_token_expiry() {
    setup_env();
    let jwt_service = JwtService::new();
    let user_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();
    let role = "user".to_string();

    // Expire in 1 second
    let token = jwt_service.generate_access_token(user_id, tenant_id, role).unwrap();
    
    // Wait 2 seconds
    thread::sleep(time::Duration::from_secs(2));

    let result = jwt_service.validate_token(&token);
    assert!(result.is_err());
    match result.unwrap_err().kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {},
        _ => panic!("Expected ExpiredSignature error"),
    }
}
