//! IoTNet - User Authentication Plugin
//!
//! This is the main entry point for the User Auth Plugin authentication service built with Actix-web.
//! The application provides user management and authentication capabilities with JWT-based
//! authentication, rate limiting, and comprehensive API endpoints.

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    user_auth_plugin::server::run_server().await
}
