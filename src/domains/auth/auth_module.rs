use actix_web::web;

pub struct AuthModule;

impl AuthModule {
    pub fn configure_module(cfg: &mut web::ServiceConfig) {
        // App mapping is now handled at server.rs level for all UseCases.
        // We only need to configure routes here.
        crate::domains::auth::routes::auth_routes::configure_routes(cfg);
    }
}
