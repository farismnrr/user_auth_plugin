/// Converts a relative URL path to a full URL using the BASE_URL environment variable.
///
/// # Arguments
///
/// * `path` - Optional relative path (e.g., "/assets/profiles/image.png")
///
/// # Returns
///
/// Returns the full URL if path is provided, or None if path is None.
/// If the path already starts with "http://" or "https://", it returns the path unchanged.
///
/// # Examples
///
/// ```
/// use user_auth_plugin::utils::url_helper::to_full_url;
///
/// let full_url = to_full_url(Some("/assets/profiles/image.png".to_string()));
/// assert_eq!(full_url, Some("http://localhost:5500/assets/profiles/image.png".to_string()));
///
/// let already_full = to_full_url(Some("http://example.com/image.png".to_string()));
/// assert_eq!(already_full, Some("http://example.com/image.png".to_string()));
///
/// let none_path = to_full_url(None);
/// assert_eq!(none_path, None);
/// ```
pub fn to_full_url(path: Option<String>) -> Option<String> {
    path.map(|p| {
        if p.starts_with("http://") || p.starts_with("https://") {
            p // Already a full URL
        } else {
            let base_url = std::env::var("BASE_URL")
                .unwrap_or_else(|_| "http://localhost:5500".to_string());
            format!("{}{}", base_url, p)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
