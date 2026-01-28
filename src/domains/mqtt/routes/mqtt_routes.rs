use actix_web::web;

use crate::domains::mqtt::controllers::mqtt_controller::MqttController;
use crate::domains::common::middlewares::api_key_middleware::ApiKeyMiddleware;

pub fn configure_routes<T>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/create")
            .wrap(ApiKeyMiddleware) // assuming this middleware exists and used by others
            .route(web::post().to(MqttController::create)),
    )
    .service(
        web::resource("/check") // check likely needs api key too if it's external, or maybe internal secret?
        // EMQX usually uses HTTP request, so we should protect it.
        // Contract scenario "Missing API Key" implies authentication is required.
            .wrap(ApiKeyMiddleware)
            .route(web::post().to(MqttController::check)),
    )
    .service(
        web::resource("/acl")
            .wrap(ApiKeyMiddleware)
            .route(web::post().to(MqttController::acl)),
    )
    .service(
        web::resource("")
            .wrap(ApiKeyMiddleware)
            .route(web::get().to(MqttController::list)),
    )
    .service(
        web::resource("/{username}")
            .wrap(ApiKeyMiddleware)
            .route(web::delete().to(MqttController::delete)),
    );
}
