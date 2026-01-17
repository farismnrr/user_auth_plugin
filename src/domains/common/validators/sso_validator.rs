use crate::domains::common::errors::AppError;

/// Validates SSO parameters (state, nonce, redirect_uri)
///
/// Rules:
/// - Max length: 128 characters
/// - State/Nonce: Alphanumeric only
/// - Redirect URI: Must be a valid URI format (basic check)
pub fn validate_sso_params(
    state: &Option<String>,
    nonce: &Option<String>,
    redirect_uri: &Option<String>,
) -> Result<(), AppError> {
    const MAX_LENGTH: usize = 128;

    if let Some(s) = state {
        if s.len() > MAX_LENGTH {
            return Err(AppError::ValidationError(
                format!("State parameter too long (max {} chars)", MAX_LENGTH),
                None,
            ));
        }
        if !s.chars().all(char::is_alphanumeric) {
            return Err(AppError::ValidationError(
                "State parameter must be alphanumeric".to_string(),
                None,
            ));
        }
    }

    if let Some(n) = nonce {
        if n.len() > MAX_LENGTH {
            return Err(AppError::ValidationError(
                format!("Nonce parameter too long (max {} chars)", MAX_LENGTH),
                None,
            ));
        }
        if !n.chars().all(char::is_alphanumeric) {
            return Err(AppError::ValidationError(
                "Nonce parameter must be alphanumeric".to_string(),
                None,
            ));
        }
    }

    if let Some(uri) = redirect_uri {
        if uri.len() > 256 {
            // slightly larger limit for URIs
            return Err(AppError::ValidationError(
                "Redirect URI too long (max 256 chars)".to_string(),
                None,
            ));
        }
        // Basic URI validation - preventing obvious script injection
        if uri.contains('<') || uri.contains('>') || uri.contains('"') || uri.contains('\'') {
            return Err(AppError::ValidationError(
                "Redirect URI contains invalid characters".to_string(),
                None,
            ));
        }
    }

    Ok(())
}

/// Validates redirect_uri against the allowed origins whitelist.
///
/// This prevents Open Redirect attacks by ensuring the redirect destination
/// is explicitly whitelisted. Extracts origin (protocol + host) and performs exact match.
///
/// # Arguments
/// * `redirect_uri` - The redirect URI to validate (optional)
/// * `allowed_origins` - Slice of allowed origin strings
///
/// # Returns
/// * `Ok(())` if redirect_uri is None or matches an allowed origin
/// * `Err(AppError::Forbidden)` if redirect_uri is not whitelisted
pub fn validate_redirect_uri_whitelist(
    redirect_uri: &Option<String>,
    allowed_origins: &[String],
) -> Result<(), AppError> {
    if let Some(uri) = redirect_uri {
        // Parse the URI to extract origin
        let parsed = url::Url::parse(uri)
            .map_err(|_| AppError::Forbidden("Invalid redirect URI format".to_string()))?;

        // Get the origin (scheme + host + port) using the proper method
        let origin = parsed.origin().ascii_serialization();

        // Exact match against allowed origins
        if !allowed_origins.iter().any(|o| o == &origin) {
            log::warn!(
                "[SSO Security] Blocked redirect_uri not in whitelist: {} (origin: {})",
                uri,
                origin
            );
            return Err(AppError::Forbidden(
                "Redirect URI not in allowed origins".to_string(),
            ));
        }
    }
    Ok(())
}
