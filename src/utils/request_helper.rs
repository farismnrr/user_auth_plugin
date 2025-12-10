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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_token() {
        let token = "test_refresh_token_12345";
        let hash1 = hash_token(token);
        let hash2 = hash_token(token);
        
        // Same token should produce same hash
        assert_eq!(hash1, hash2);
        
        // Hash should be 64 characters (SHA-256 hex)
        assert_eq!(hash1.len(), 64);
        
        // Different token should produce different hash
        let different_hash = hash_token("different_token");
        assert_ne!(hash1, different_hash);
    }
}
