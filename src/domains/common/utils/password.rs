use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::domains::common::errors::AppError;

/// Hashes a password using Argon2.
///
/// # Arguments
///
/// * `password` - Plain text password to hash
///
/// # Returns
///
/// Returns the hashed password as a string.
///
/// # Errors
///
/// Returns `AppError::InternalError` if hashing fails.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalError(format!("Failed to hash password: {}", e)))?
        .to_string();
    
    Ok(password_hash)
}

/// Verifies a password against a hash.
///
/// # Arguments
///
/// * `password` - Plain text password to verify
/// * `hash` - Argon2 hash to verify against
///
/// # Returns
///
/// Returns `true` if the password matches the hash, `false` otherwise.
///
/// # Errors
///
/// Returns `AppError::InternalError` if verification fails.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::InternalError(format!("Failed to parse password hash: {}", e)))?;
    
    let argon2 = Argon2::default();
    
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}


