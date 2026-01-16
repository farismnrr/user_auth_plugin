use super::api_key_middleware::*;
use actix_web::{test, App, http, web};

#[actix_web::test]
async fn test_api_key_middleware_missing_header() {
    use crate::domains::common::utils::config::Config;
    Config::init_for_test();
    
    let middleware = ApiKeyMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" }))
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&srv, req).await;
    
    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_api_key_middleware_invalid_key() {
    use crate::domains::common::utils::config::Config;
    Config::init_for_test();
    
    let middleware = ApiKeyMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" }))
    ).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("X-API-Key", "wrong_key"))
        .to_request();
    let resp = test::call_service(&srv, req).await;
    
    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_api_key_middleware_valid_key_no_db() {
    use crate::domains::common::utils::config::Config;
    let config = Config::init_for_test();
    
    let middleware = ApiKeyMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" }))
    ).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("X-API-Key", config.api_key.as_str()))
        .to_request();
        
    let resp = test::call_service(&srv, req).await;
    
    assert_eq!(resp.status(), http::StatusCode::OK);
}
