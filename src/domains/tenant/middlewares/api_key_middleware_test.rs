
use super::api_key_middleware::*;
use actix_web::{test, App, http, web};
use std::env;

#[actix_web::test]
async fn test_api_key_middleware_missing_header() {
    env::set_var("API_KEY", "secret_api_key");
    
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
    env::set_var("API_KEY", "secret_api_key");
    
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
    // Tests that it passes auth even if DB is missing (logic says it logs error/debug but proceeds Authorized)
    // Wait, let's check source: 
    // "debug!("[Middleware | ApiKey] Authorized request to '{}'", path);"
    // "let res = service.call(req).await?;"
    // So yes, it should pass even without DB injecting TenantId.
    
    env::set_var("API_KEY", "secret_api_key");
    
    let middleware = ApiKeyMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" }))
    ).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("X-API-Key", "secret_api_key"))
        .to_request();
        
    let resp = test::call_service(&srv, req).await;
    
    assert_eq!(resp.status(), http::StatusCode::OK);
}
