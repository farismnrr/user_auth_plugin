use crate::domains::auth::controllers::auth_controller::{
    change_password, generate_invitation_code, login, logout, refresh, register, verify,
};
use crate::domains::auth::middlewares::auth_middleware;
use crate::domains::tenant::middlewares::api_key_middleware::ApiKeyMiddleware;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

/// Configures authentication routes (ApiKey and JWT protected).
///
/// # Routes (under /auth scope)
///
/// **ApiKey Protected:**
/// - `POST /register`
/// - `POST /login`
/// - `POST /refresh`
///
/// **JWT Protected:**
/// - `POST /logout`
/// - `POST /verify`
/// - `PUT /change-password`
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let jwt_auth = HttpAuthentication::bearer(auth_middleware::validator);

    // AuthUseCase is now registered globally in server.rs

    cfg.service(
        web::scope("/auth")
            // SSO Logout (Cookie based, no Bearer auth required)
            // Accessed via browser redirect
            // Moved to top to avoid middleware wrapping issues
            .route("/sso/logout", web::get().to(crate::domains::auth::controllers::auth_controller::sso_logout))

            // ApiKey protected routes
            .service(
                web::resource("/register")
                    .wrap(ApiKeyMiddleware)
                    .route(web::post().to(register))
            )
            .service(
                web::resource("/login")
                    .wrap(ApiKeyMiddleware)
                    .route(web::post().to(login))
            )
            .service(
                web::resource("/refresh")
                    .wrap(ApiKeyMiddleware)
                    .route(web::get().to(refresh))
            )
            // Internal routes (TenantSecret Protected)
            .service(
                web::resource("/internal/invitations")
                    .wrap(crate::domains::tenant::middlewares::tenant_secret_middleware::TenantSecretMiddleware)
                    .route(web::post().to(generate_invitation_code))
            )
            // JWT protected routes
            .service(
                web::scope("")
                    .wrap(jwt_auth)
                    // Routes requiring only JWT
                    .route("/verify", web::get().to(verify))
                    .route("/logout", web::delete().to(logout))

                    // Nested scope for JWT + ApiKey protected routes
                    .service(
                        web::scope("")
                            .wrap(ApiKeyMiddleware)
                            .route("/reset", web::put().to(change_password))
                    )
            )
    );
}
