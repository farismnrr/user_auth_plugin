use crate::controllers::user_controller::{
    delete_user, get_all_users, get_user, update_user,
};
use crate::controllers::user_details_controller::{
    update_user_details, upload_profile_picture,
};
use crate::middlewares::auth;
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
pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    let jwt_auth = HttpAuthentication::bearer(auth::validator);
    
    cfg.service(
        web::scope("/users")
            .wrap(jwt_auth)  // Apply JWT auth to ALL routes
            .route("", web::get().to(get_user))           // GET /users (current user)
            .route("/all", web::get().to(get_all_users))  // GET /users/all
            .route("", web::put().to(update_user))        // PUT /users (current user)
            .route("", web::delete().to(delete_user))     // DELETE /users (current user)
            .route("/details", web::put().to(update_user_details))      // PUT /users/details
            .route("/uploads", web::patch().to(upload_profile_picture)) // PATCH /users/uploads
    );
}
