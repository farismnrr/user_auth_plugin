use crate::errors::{AppError, ValidationDetail};

/// Validates phone number format and length.
pub fn validate_phone_number(phone: &Option<String>) -> Result<(), AppError> {
    if let Some(p) = phone {
        // Delegate to user_validator::validate_phone for core logic, but map error if needed
        // Assuming user_validator::validate_phone(p) returns AppError::ValidationError
        // We might want to just call that.
        // But for now, let's keep logic here or use the one from user_validator if consistent.
        
        // Actually, user_validator::validate_phone expects &str.
        // Let's use logic here to ensure specific field name "phone".
        
        if !p.starts_with('+') {
            return Err(AppError::ValidationError(
                "Validation Error".to_string(),
                Some(vec![ValidationDetail {
                    field: "phone".to_string(),
                    message: "Invalid phone format".to_string(),
                }]),
            ));
        }
         if !p[1..].chars().all(|c| c.is_digit(10)) {
            return Err(AppError::ValidationError(
                "Validation Error".to_string(),
                Some(vec![ValidationDetail {
                    field: "phone".to_string(),
                    message: "Invalid phone format".to_string(),
                }]),
            ));
         }
         if p.len() < 10 || p.len() > 20 {
             return Err(AppError::ValidationError(
                "Validation Error".to_string(),
                Some(vec![ValidationDetail {
                    field: "phone".to_string(),
                    message: "Invalid phone format".to_string(),
                }]),
            ));
         }
    }
    Ok(())
}

/// Validates a name part (first name or last name).
pub fn validate_name_part(name: &Option<String>, field_name: &str) -> Result<(), AppError> {
    if let Some(n) = name {
        if n.trim().len() < 2 || n.len() > 50 {
             return Err(AppError::ValidationError(
                "Validation Error".to_string(),
                Some(vec![ValidationDetail {
                    field: field_name.to_string(),
                    message: "Invalid length".to_string(),
                }]),
            ));
        }
        if !n.chars().all(|c| c.is_alphabetic() || c.is_whitespace() || c == '-' || c == '\'') {
             return Err(AppError::ValidationError(
                "Validation Error".to_string(),
                Some(vec![ValidationDetail {
                    field: field_name.to_string(),
                    message: "Invalid characters".to_string(),
                }]),
            ));
        }
    }
    Ok(())
}

/// Validates address format and length.
pub fn validate_address(address: &Option<String>) -> Result<(), AppError> {
    if let Some(a) = address {
        if a.trim().len() < 5 || a.len() > 500 {
            return Err(AppError::ValidationError(
                "Validation Error".to_string(),
                Some(vec![ValidationDetail {
                    field: "address".to_string(),
                    message: "Too long".to_string(), // Or too short, but usually max length checks return "Too long"
                    // Contract 4e says: "details": [{"field":"address","message":"Too long"}]
                }]),
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
        assert!(validate_phone_number(&Some("+1234567890".to_string())).is_ok());
        assert!(validate_phone_number(&Some("123".to_string())).is_err());
    }

    #[test]
    fn test_validate_name_part() {
        assert!(validate_name_part(&None, "first_name").is_ok());
        assert!(validate_name_part(&Some("John".to_string()), "first_name").is_ok());
        assert!(validate_name_part(&Some("A".to_string()), "first_name").is_err());
    }
}
