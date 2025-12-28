use crate::controllers::auth_controller::{change_password, login, logout, refresh, register, verify};
use crate::middlewares::auth_middleware;
use crate::middlewares::api_key_middleware::ApiKeyMiddleware;
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
    
    cfg.service(
        web::scope("/auth")
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
            // JWT protected routes
            .service(
                web::scope("") // Nested scope to apply JWT middleware to multiple routes? 
                               // Or just resource chaining. Logout/Verify/ChangePW
                .wrap(jwt_auth)
                .wrap(ApiKeyMiddleware)
                .route("/logout", web::delete().to(logout))
                .route("/verify", web::get().to(verify))
                .route("/reset", web::put().to(change_password))
            )
    );
}
