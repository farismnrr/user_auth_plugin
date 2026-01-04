use actix_web::HttpRequest;
use sha2::{Digest, Sha256};

/// Extracts client information from HTTP request.
///
/// Returns a tuple of (ip_address, user_agent) as Option<String>.
pub fn extract_client_info(req: &HttpRequest) -> (Option<String>, Option<String>) {
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    (ip_address, user_agent)
}

/// Hashes a refresh token using SHA-256 for secure storage.
///
/// # Arguments
///
/// * `token` - The refresh token to hash
///
/// # Returns
///
/// Returns the hexadecimal string representation of the hash.
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}


