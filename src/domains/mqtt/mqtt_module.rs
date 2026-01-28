use actix_web::web;

use super::controllers::mqtt_controller::MqttController;

pub struct MqttModule;

impl MqttModule {
    pub fn configure_module(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/mqtt")
                .configure(super::routes::mqtt_routes::configure_routes::<MqttController>),
        );
    }
}
