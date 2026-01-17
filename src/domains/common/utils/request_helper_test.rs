use super::request_helper::*;

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
