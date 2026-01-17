use super::sso_validator::*;
use crate::domains::common::errors::AppError;

#[test]
fn test_valid_params() {
    let state = Some("validState123".to_string());
    let nonce = Some("validNonce456".to_string());
    let redirect_uri = Some("https://example.com/callback".to_string());

    assert!(validate_sso_params(&state, &nonce, &redirect_uri).is_ok());
}

#[test]
fn test_empty_params() {
    assert!(validate_sso_params(&None, &None, &None).is_ok());
}

#[test]
fn test_state_too_long() {
    let state = Some("a".repeat(129));
    let result = validate_sso_params(&state, &None, &None);
    assert!(matches!(result, Err(AppError::ValidationError(..))));
}

#[test]
fn test_state_invalid_chars() {
    let state = Some("invalid-state!".to_string());
    let result = validate_sso_params(&state, &None, &None);
    assert!(matches!(result, Err(AppError::ValidationError(..))));
}

#[test]
fn test_nonce_too_long() {
    let nonce = Some("a".repeat(129));
    let result = validate_sso_params(&None, &nonce, &None);
    assert!(matches!(result, Err(AppError::ValidationError(..))));
}

#[test]
fn test_nonce_invalid_chars() {
    let nonce = Some("invalid_nonce".to_string());
    let result = validate_sso_params(&None, &nonce, &None);
    assert!(matches!(result, Err(AppError::ValidationError(..))));
}

#[test]
fn test_redirect_uri_too_long() {
    let uri = Some("https://example.com/".to_string() + &"a".repeat(250));
    let result = validate_sso_params(&None, &None, &uri);
    assert!(matches!(result, Err(AppError::ValidationError(..))));
}

#[test]
fn test_redirect_uri_injection() {
    let uri = Some("https://example.com/<script>".to_string());
    let result = validate_sso_params(&None, &None, &uri);
    assert!(matches!(result, Err(AppError::ValidationError(..))));
}

// Tests for validate_redirect_uri_whitelist
#[test]
fn test_whitelist_valid_origin() {
    let allowed = vec![
        "https://app.example.com".to_string(),
        "https://localhost:3000".to_string(),
    ];
    let uri = Some("https://app.example.com/callback".to_string());
    assert!(validate_redirect_uri_whitelist(&uri, &allowed).is_ok());
}

#[test]
fn test_whitelist_none_uri() {
    let allowed = vec!["https://app.example.com".to_string()];
    assert!(validate_redirect_uri_whitelist(&None, &allowed).is_ok());
}

#[test]
fn test_whitelist_invalid_origin() {
    let allowed = vec!["https://app.example.com".to_string()];
    let uri = Some("https://evil.com/callback".to_string());
    let result = validate_redirect_uri_whitelist(&uri, &allowed);
    assert!(matches!(result, Err(AppError::Forbidden(..))));
}

#[test]
fn test_whitelist_invalid_url_format() {
    let allowed = vec!["https://app.example.com".to_string()];
    let uri = Some("not-a-valid-url".to_string());
    let result = validate_redirect_uri_whitelist(&uri, &allowed);
    assert!(matches!(result, Err(AppError::Forbidden(..))));
}

#[test]
fn test_whitelist_origin_with_port() {
    let allowed = vec!["http://localhost:3000".to_string()];
    let uri = Some("http://localhost:3000/auth/callback".to_string());
    assert!(validate_redirect_uri_whitelist(&uri, &allowed).is_ok());
}

#[test]
fn test_whitelist_different_port_rejected() {
    let allowed = vec!["http://localhost:3000".to_string()];
    let uri = Some("http://localhost:4000/auth/callback".to_string());
    let result = validate_redirect_uri_whitelist(&uri, &allowed);
    assert!(matches!(result, Err(AppError::Forbidden(..))));
}
