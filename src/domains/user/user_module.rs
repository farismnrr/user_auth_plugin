use actix_web::web;

pub struct UserModule;

impl UserModule {
    pub fn configure_module(cfg: &mut web::ServiceConfig) {
        // App mapping is now handled at server.rs level for all UseCases.
        crate::domains::user::routes::user_routes::configure_user_routes(cfg);
    }
}
