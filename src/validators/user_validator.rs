use crate::errors::{AppError, ValidationDetail};

/// Validates a username.
pub fn validate_username(username: &str) -> Result<(), AppError> {
    log::debug!("Checking username (len: {})", username.trim().len());
    let trimmed = username.trim();
    
    if trimmed.is_empty() {
        return Err(AppError::ValidationError(
            "Validation Error".to_string(),
            Some(vec![ValidationDetail {
                field: "username".to_string(),
                message: "Username cannot be empty".to_string(),
            }]),
        ));
    }
    
    if trimmed.len() < 3 {
        return Err(AppError::ValidationError(
             "Validation Error".to_string(),
            Some(vec![ValidationDetail {
                field: "username".to_string(),
                message: "Username too short".to_string(),
            }]),
        ));
    }
    
    if trimmed.len() > 50 {
        return Err(AppError::ValidationError(
             "Validation Error".to_string(),
            Some(vec![ValidationDetail {
                field: "username".to_string(),
                message: "Username too long".to_string(),
            }]),
        ));
    }

    // Check for allowed characters (alphanumeric, underscore, dash)
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(AppError::ValidationError(
            "Validation Error".to_string(),
            Some(vec![ValidationDetail {
                field: "username".to_string(),
                message: "Invalid characters".to_string(),
            }]),
        ));
    }

    // Check for reserved words
    let reserved_words = [
        "admin", "root", "system", "superuser", "administrator", "god", 
        "null", "undefined", "test", "demo"
    ];
    
    if reserved_words.contains(&trimmed.to_lowercase().as_str()) {
        return Err(AppError::Conflict(
            "Reserved Username".to_string(),
        )); 
    }
    
    validate_no_xss(trimmed, "username")?;
    
    Ok(())
}

/// Validates an email address.
pub fn validate_email(email: &str) -> Result<(), AppError> {
    let trimmed = email.trim();
    
    if !trimmed.contains('@') || !trimmed.contains('.') {
        return Err(AppError::ValidationError(
            "Validation Error".to_string(),
            Some(vec![ValidationDetail {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
            }]),
        ));
    }
    
    // Check @ comes before .
    if let Some(at_pos) = trimmed.find('@') {
        // Check there's content before @
        if at_pos == 0 {
             return Err(AppError::ValidationError(
                "Validation Error".to_string(),
                Some(vec![ValidationDetail {
                    field: "email".to_string(),
                    message: "Invalid email format".to_string(),
                }]),
            ));
        }
        
        if let Some(dot_pos) = trimmed.rfind('.') {
            if at_pos >= dot_pos {
                 return Err(AppError::ValidationError(
                    "Validation Error".to_string(),
                    Some(vec![ValidationDetail {
                        field: "email".to_string(),
                        message: "Invalid email format".to_string(),
                    }]),
                ));
            }
        }
    }
    
    Ok(())
}

/// Validates a password.
pub fn validate_password(password: &str, field_name: &str) -> Result<(), AppError> {
    if password.len() < 6 {
         return Err(AppError::ValidationError(
            "Validation Error".to_string(),
            Some(vec![ValidationDetail {
                field: field_name.to_string(),
                message: "Password too weak".to_string(),
            }]),
        ));
    }
    
    if password.len() > 128 {
         return Err(AppError::ValidationError(
            "Validation Error".to_string(),
             Some(vec![ValidationDetail {
                field: field_name.to_string(),
                message: "Password too long".to_string(),
            }]),
        ));
    }
    
    Ok(())
}


pub fn validate_no_xss(input: &str, field: &str) -> Result<(), AppError> {
    if input.contains('<') || input.contains('>') || input.contains("javascript:") {
        return Err(AppError::ValidationError(
            "Validation Error".to_string(),
            Some(vec![ValidationDetail {
                field: field.to_string(),
                message: "Invalid characters".to_string(),
            }]),
        ));
    }
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        assert!(validate_username("validuser").is_ok());
        assert!(validate_username("ab").is_err());
        assert!(validate_username("").is_err());
        assert!(validate_username("   ").is_err());
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("test@").is_err());
        assert!(validate_email("@example.com").is_err());
    }

    #[test]
    fn test_validate_password() {
        assert!(validate_password("password123", "password").is_ok());
        assert!(validate_password("12345", "password").is_err());
        assert!(validate_password(&"a".repeat(129), "password").is_err());
    }
}
