use super::jwt::*;
use uuid::Uuid;

fn setup_env() {
    use crate::domains::common::utils::config::Config;
    Config::init_for_test();
}

#[test]
fn test_generate_and_validate_access_token() {
    setup_env();
    let jwt_service = JwtService::new();
    let user_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();
    let role = "user".to_string();

    let token = jwt_service
        .generate_access_token(user_id, tenant_id, role.clone())
        .unwrap();
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

    let token = jwt_service
        .generate_refresh_token(user_id, tenant_id, role.clone(), None)
        .unwrap();
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

    // Token expires based on ACCESS_TOKEN_EXPIRY from config (default 900s)
    // Test that token is valid immediately after generation
    let token = jwt_service
        .generate_access_token(user_id, tenant_id, role)
        .unwrap();

    let result = jwt_service.validate_token(&token);
    assert!(result.is_ok());
}
