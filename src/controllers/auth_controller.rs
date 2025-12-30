use crate::dtos::auth_dto::{LoginRequest, LoginRequestJson, RegisterRequest, RegisterRequestJson};
use crate::dtos::change_password_dto::ChangePasswordRequest;
use crate::dtos::response_dto::SuccessResponseDTO;
use crate::errors::AppError;
use crate::usecases::auth_usecase::AuthUseCase;
use crate::validators::sso_validator::{validate_sso_params, validate_redirect_uri_whitelist};
use actix_web::{
    cookie::{Cookie, SameSite},
    web, HttpMessage, HttpResponse, Responder,
};
use std::sync::Arc;

/// Register a new user
use crate::middlewares::api_key_middleware::TenantId;

// ...

pub async fn register(
    usecase: web::Data<Arc<AuthUseCase>>,
    allowed_origins: web::Data<Vec<String>>,
    body: web::Json<RegisterRequestJson>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, AppError> {
    // Extract tenant_id from request extensions (set by ApiKeyMiddleware)
    let tenant_id = req.extensions()
        .get::<TenantId>()
        .map(|id| id.0)
        .ok_or_else(|| AppError::NotFound("Tenant ID not found in request context".to_string()))?;

    // Validate SSO params if present
    validate_sso_params(&body.state, &body.nonce, &body.redirect_uri)?;
    
    // Validate redirect_uri against allowed origins whitelist
    validate_redirect_uri_whitelist(&body.redirect_uri, &allowed_origins)?;

    let register_req = RegisterRequest {
        username: body.username.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
        role: body.role.clone(),
        tenant_id,
        state: body.state.clone(),
        nonce: body.nonce.clone(),
        redirect_uri: body.redirect_uri.clone(),
    };

    let auth_response = usecase.register(register_req, &req).await?;

    Ok(HttpResponse::Created().json(SuccessResponseDTO::new(
        "User registered successfully",
        serde_json::json!({
            "user_id": auth_response.user_id
        }),
    )))
}

/// Authenticates a user and returns access token with refresh token cookie.
///
/// The refresh token is set as an HTTP-only cookie for security.
/// In production, ensure `secure` is set to `true` when using HTTPS.
pub async fn login(
    usecase: web::Data<Arc<AuthUseCase>>,
    allowed_origins: web::Data<Vec<String>>,
    body: web::Json<LoginRequestJson>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, AppError> {
    // Extract tenant_id from request extensions (set by ApiKeyMiddleware)
    let tenant_id = req.extensions()
        .get::<TenantId>()
        .map(|id| id.0)
        .ok_or_else(|| AppError::NotFound("Tenant ID not found in request context".to_string()))?;

    // Validate SSO params if present
    validate_sso_params(&body.state, &body.nonce, &body.redirect_uri)?;
    
    // Validate redirect_uri against allowed origins whitelist
    validate_redirect_uri_whitelist(&body.redirect_uri, &allowed_origins)?;

    let login_req = LoginRequest {
        email_or_username: body.email_or_username.to_string(),
        password: body.password.clone(),
        tenant_id,
        state: body.state.clone(),
        nonce: body.nonce.clone(),
        redirect_uri: body.redirect_uri.clone(),
    };

    let (auth_response, refresh_token) = usecase.login(login_req, &req).await?;

    let refresh_token_expiry = usecase.get_refresh_token_expiry();
    let cookie = Cookie::build("refresh_token", refresh_token)
        .path("/")
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::seconds(
            refresh_token_expiry,
        ))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(SuccessResponseDTO::new(
            "Login successful",
            serde_json::json!({
                "access_token": auth_response.access_token
            })
        )))
}

/// Logs out a user by clearing the refresh token cookie and deleting the session.
///
/// This endpoint requires JWT authentication to ensure only logged-in users can logout.
/// It verifies the user exists and deletes the current session identified by the refresh token.
pub async fn logout(
    usecase: web::Data<Arc<AuthUseCase>>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, AppError> {
    // Extract user_id from JWT token
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;

    // Logout (deletes session)
    usecase.logout(user_id, &req).await?;

    // Clear refresh token cookie
    let cookie = Cookie::build("refresh_token", "")
        .path("/")
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::seconds(0))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(SuccessResponseDTO::<()>::no_data(
            "Logged out successfully",
        )))
}

/// Logs out a user via redirect (SSO Logout).
///
/// This endpoint clears the refresh token cookie and redirects the user
/// to the specified `redirect_uri` (or defaults to home).
/// It is designed for browser-based flows where the `Authorization` header
/// might not be available, relying solely on the HTTP-Only cookie.
pub async fn sso_logout(
    usecase: web::Data<Arc<AuthUseCase>>,
    allowed_origins: web::Data<Vec<String>>,
    query: web::Query<crate::dtos::auth_dto::SsoLogoutQuery>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, AppError> {
    // Validate redirect_uri against allowed origins whitelist
    validate_redirect_uri_whitelist(&query.redirect_uri, &allowed_origins)?;
    
    // Attempt logout logic (delete session) - ignore errors (e.g. if already logged out)
    let _ = usecase.sso_logout(&req).await;

    // Clear refresh token cookie
    let cookie = Cookie::build("refresh_token", "")
        .path("/")
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::seconds(0))
        .finish();

    let redirect_url = query.redirect_uri.clone().unwrap_or_else(|| "/".to_string());

    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .cookie(cookie)
        .finish())
}

/// Refreshes access token using refresh token from cookie.
///
/// Delegates to use case for cookie extraction and token refresh logic.
pub async fn refresh(
    usecase: web::Data<Arc<AuthUseCase>>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, AppError> {
    let (new_access_token, new_refresh_token) = usecase.refresh_token_from_request(&req).await?;

    let refresh_token_expiry = usecase.get_refresh_token_expiry();
    let cookie = Cookie::build("refresh_token", new_refresh_token)
        .path("/")
        .http_only(true)
        .secure(false) // TODO: Set to true in production
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::seconds(
            refresh_token_expiry,
        ))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(SuccessResponseDTO::new(
        "Token refreshed successfully",
        serde_json::json!({
            "access_token": new_access_token
        }),
    )))
}

/// Verifies JWT token and returns user data if valid.
///
/// This endpoint uses JWT middleware to validate the token.
/// It then checks if the user still exists in the database.
pub async fn verify(
    usecase: web::Data<Arc<AuthUseCase>>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, AppError> {
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;
    // Verify user exists but don't return the user data to match contract
    let _ = usecase.verify_user_exists(user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": true,
        "message": "Token is valid"
    })))
}

/// Changes the authenticated user's password.
///
/// This endpoint requires JWT authentication. It validates the old password,
/// ensures the new password meets requirements and matches confirmation,
/// then updates the password and logs the activity.
pub async fn change_password(
    usecase: web::Data<Arc<AuthUseCase>>,
    body: web::Json<ChangePasswordRequest>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, AppError> {
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;
    usecase
        .change_password(user_id, body.into_inner(), &req)
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponseDTO::<()>::no_data(
        "Password changed successfully",
    )))
}
