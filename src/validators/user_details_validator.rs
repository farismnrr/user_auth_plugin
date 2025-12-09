use crate::errors::AppError;

/// Validates phone number format and length.
///
/// # Rules
/// - Optional field (can be None)
/// - If provided, must be between 10-20 characters
/// - Must start with + for international format
///
/// # Errors
/// Returns `AppError::ValidationError` if validation fails.
pub fn validate_phone_number(phone: &Option<String>) -> Result<(), AppError> {
    if let Some(p) = phone {
        if p.len() < 10 || p.len() > 20 {
            return Err(AppError::ValidationError(
                "Phone number must be between 10-20 characters".to_string(),
            ));
        }
        if !p.starts_with('+') {
            return Err(AppError::ValidationError(
                "Phone number must start with + for international format".to_string(),
            ));
        }
    }
    Ok(())
}

/// Validates full name format and length.
///
/// # Rules
/// - Optional field (can be None)
/// - If provided, must be between 2-100 characters
/// - Must contain only letters, spaces, hyphens, and apostrophes
///
/// # Errors
/// Returns `AppError::ValidationError` if validation fails.
pub fn validate_full_name(name: &Option<String>) -> Result<(), AppError> {
    if let Some(n) = name {
        if n.trim().len() < 2 || n.len() > 100 {
            return Err(AppError::ValidationError(
                "Full name must be between 2-100 characters".to_string(),
            ));
        }
        if !n.chars().all(|c| c.is_alphabetic() || c.is_whitespace() || c == '-' || c == '\'') {
            return Err(AppError::ValidationError(
                "Full name must contain only letters, spaces, hyphens, and apostrophes".to_string(),
            ));
        }
    }
    Ok(())
}

/// Validates address format and length.
///
/// # Rules
/// - Optional field (can be None)
/// - If provided, must be between 5-500 characters
///
/// # Errors
/// Returns `AppError::ValidationError` if validation fails.
pub fn validate_address(address: &Option<String>) -> Result<(), AppError> {
    if let Some(a) = address {
        if a.trim().len() < 5 || a.len() > 500 {
            return Err(AppError::ValidationError(
                "Address must be between 5-500 characters".to_string(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_phone_number() {
        assert!(validate_phone_number(&None).is_ok());
        // Valid
        assert!(validate_phone_number(&Some("+1234567890".to_string())).is_ok());
        assert!(validate_phone_number(&Some("+12345678901234567".to_string())).is_ok()); // 19 chars (within limit)
        
        // Invalid - too short
        assert!(validate_phone_number(&Some("+123".to_string())).is_err());
        
        // Invalid - too long (21 chars)
        assert!(validate_phone_number(&Some("+123456789012345678901".to_string())).is_err());
        
        // Invalid - doesn't start with +
        assert!(validate_phone_number(&Some("1234567890".to_string())).is_err());
    }

    #[test]
    fn test_validate_full_name() {
        assert!(validate_full_name(&None).is_ok());
        assert!(validate_full_name(&Some("John Doe".to_string())).is_ok());
        assert!(validate_full_name(&Some("Mary-Jane O'Connor".to_string())).is_ok());
        
        assert!(validate_full_name(&Some("A".to_string())).is_err());
        assert!(validate_full_name(&Some("John123".to_string())).is_err());
    }

    #[test]
    fn test_validate_address() {
        assert!(validate_address(&None).is_ok());
        assert!(validate_address(&Some("123 Main St".to_string())).is_ok());
        
        assert!(validate_address(&Some("123".to_string())).is_err());
    }
}
