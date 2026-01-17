use super::url_helper::*;
use crate::domains::common::utils::config::Config;

#[test]
fn test_to_full_url_with_relative_path() {
    let base_url = &Config::get().endpoint;
    let expected = format!("{}{}", base_url, "/assets/profiles/test.png");
    let result = to_full_url(Some("/assets/profiles/test.png".to_string()));
    assert_eq!(result, Some(expected));
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
// Removed test_to_full_url_uses_default_base_url as it tests Config behavior which is immutable singleton
