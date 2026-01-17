use super::password::*;

#[test]
fn test_hash_and_verify_password() {
    let password = "test_password_123";
    let hash = hash_password(password).unwrap();

    assert!(verify_password(password, &hash).unwrap());
    assert!(!verify_password("wrong_password", &hash).unwrap());
}
