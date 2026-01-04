
use super::url_helper::*;

#[test]
fn test_to_full_url_with_relative_path() {
    std::env::set_var("BASE_URL", "http://localhost:5500");
    let result = to_full_url(Some("/assets/profiles/test.png".to_string()));
    assert_eq!(result, Some("http://localhost:5500/assets/profiles/test.png".to_string()));
}

#[test]
fn test_to_full_url_with_http_url() {
    let result = to_full_url(Some("http://example.com/image.png".to_string()));
    assert_eq!(result, Some("http://example.com/image.png".to_string()));
}

#[test]
fn test_to_full_url_with_https_url() {
    let result = to_full_url(Some("https://example.com/image.png".to_string()));
    assert_eq!(result, Some("https://example.com/image.png".to_string()));
}

#[test]
fn test_to_full_url_with_none() {
    let result = to_full_url(None);
    assert_eq!(result, None);
}

#[test]
fn test_to_full_url_uses_default_base_url() {
    std::env::remove_var("BASE_URL");
    let result = to_full_url(Some("/test.png".to_string()));
    assert_eq!(result, Some("http://localhost:5500/test.png".to_string()));
}
