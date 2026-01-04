
use super::tenant_secret_middleware::*;
use actix_web::{test, App, http, web};
use std::env;

#[actix_web::test]
async fn test_tenant_secret_middleware_missing_config() {
    env::remove_var("TENANT_SECRET_KEY");
    
    let middleware = TenantSecretMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" }))
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&srv, req).await;
    
    // Should be 500 because config is missing
    assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_web::test]
async fn test_tenant_secret_middleware_success() {
    env::set_var("TENANT_SECRET_KEY", "test_master_key");
    
    let middleware = TenantSecretMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" }))
    ).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("X-Tenant-Secret-Key", "test_master_key"))
        .to_request();
        
    let resp = test::call_service(&srv, req).await;
    
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn test_tenant_secret_middleware_invalid_key() {
    env::set_var("TENANT_SECRET_KEY", "test_master_key");
    
    let middleware = TenantSecretMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" }))
    ).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("X-Tenant-Secret-Key", "wrong"))
        .to_request();
        
    let resp = test::call_service(&srv, req).await;
    
    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
}
