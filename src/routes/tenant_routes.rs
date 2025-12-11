use crate::controllers::tenant_controller;
use crate::middlewares::auth::validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

/// Configures tenant routes.
///
/// All tenant routes require JWT authentication.
///
/// # Arguments
///
/// * `cfg` - Service configuration
pub fn configure_tenant_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);
    
    cfg.service(
        web::scope("")
            .wrap(auth)
            .service(tenant_controller::create_tenant)
            .service(tenant_controller::get_tenant)
            .service(tenant_controller::get_all_tenants)
            .service(tenant_controller::update_tenant)
            .service(tenant_controller::delete_tenant),
    );
}
