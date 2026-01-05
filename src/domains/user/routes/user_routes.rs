use crate::domains::user::controllers::user_controller::{
    delete_user, get_all_users, get_user, update_user,
};
use crate::domains::user::controllers::user_details_controller::{
    get_user_details, update_user_details, upload_profile_picture,
};
use crate::domains::auth::middlewares::auth_middleware;
use crate::domains::tenant::middlewares::api_key_middleware::ApiKeyMiddleware;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

/// Configures user management routes.
///
/// # Routes
///
/// **JWT-authenticated routes:**
/// - `GET /users` - Get current user (from JWT)
/// - `GET /users/all` - Get all users (from JWT)
/// - `PUT /users` - Update current user (from JWT) - Returns ID only
/// - `DELETE /users` - Delete current user (from JWT) - Returns ID only
/// - `PUT /users/details` - Update current user's details (from JWT) - Returns ID only
/// - `PATCH /users/uploads` - Upload profile picture (from JWT) - Returns ID only
///
/// All routes require JWT Bearer token authentication.
pub fn configure_user_routes(
    cfg: &mut web::ServiceConfig,
) {
    let jwt_auth = HttpAuthentication::bearer(auth_middleware::validator);
    
    // UseCases are now registered globally in server.rs

    cfg.service(
        web::scope("/users")
            .wrap(jwt_auth)  // Apply JWT auth to ALL routes
            .wrap(ApiKeyMiddleware) // Apply ApiKey middleware to resolve Tenant ID
            .route("", web::get().to(get_user))           // GET /users (current user)
            .route("/all", web::get().to(get_all_users))  // GET /users/all
            .route("", web::put().to(update_user))        // PUT /users (current user)
            .route("", web::delete().to(delete_user))     // DELETE /users (current user)
            .route("/details", web::get().to(get_user_details))         // GET /users/details
            .route("/details", web::put().to(update_user_details))      // PUT /users/details
            .route("/uploads", web::patch().to(upload_profile_picture)) // PATCH /users/uploads
    );
}
