use actix_web::web;

pub struct TenantModule;

impl TenantModule {
    pub fn configure_module(cfg: &mut web::ServiceConfig) {
        // App mapping is now handled at server.rs level for all UseCases.
        crate::domains::tenant::routes::tenant_routes::configure_tenant_routes(cfg);
    }
}
