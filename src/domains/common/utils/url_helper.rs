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
/// use user_auth_plugin::domains::common::utils::url_helper::to_full_url;
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
            return p; // Already a full URL
        }

        let base_url = std::env::var("BASE_URL")
            .unwrap_or_else(|_| "http://localhost:5500".to_string());
        format!("{}{}", base_url, p)
    })
}


