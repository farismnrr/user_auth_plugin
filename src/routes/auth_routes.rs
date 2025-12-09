use crate::controllers::auth_controller::{login, logout, refresh, register, verify};
use crate::middlewares::auth;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

/// Configures API key protected authentication routes.
///
/// # Routes (under /api scope)
///
/// - `POST /api/auth/register` - Register a new user (X-API-Key auth)
/// - `POST /api/auth/login` - Authenticate and receive tokens (X-API-Key auth)
/// - `POST /api/auth/refresh` - Refresh access token using refresh token cookie (X-API-Key auth)
///
/// All routes require X-API-Key header authentication.
pub fn configure_api_key_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/refresh", web::post().to(refresh))
    );
}

/// Configures JWT protected authentication routes.
///
/// # Routes (at root level)
///
/// - `POST /auth/logout` - Clear refresh token cookie (JWT auth)
/// - `POST /auth/verify` - Verify JWT token and get user data (JWT auth)
///
/// All routes require JWT Bearer token authentication.
pub fn configure_jwt_routes(cfg: &mut web::ServiceConfig) {
    let jwt_auth = HttpAuthentication::bearer(auth::validator);
    
    cfg.service(
        web::scope("/auth")
            .wrap(jwt_auth)
            .route("/logout", web::post().to(logout))
            .route("/verify", web::post().to(verify))
    );
}
