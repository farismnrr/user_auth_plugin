use super::tenant_secret_middleware::*;
use actix_web::{http, test, web, App};

#[actix_web::test]
async fn test_tenant_secret_middleware_missing_config() {
    use crate::domains::common::utils::config::Config;
    Config::init_for_test();

    let middleware = TenantSecretMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" })),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&srv, req).await;

    // Should be 401 because header is missing (config is set from .env)
    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_tenant_secret_middleware_success() {
    use crate::domains::common::utils::config::Config;
    let config = Config::init_for_test();

    let middleware = TenantSecretMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" })),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("X-Tenant-Secret-Key", config.tenant_secret_key.as_str()))
        .to_request();

    let resp = test::call_service(&srv, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn test_tenant_secret_middleware_invalid_key() {
    use crate::domains::common::utils::config::Config;
    Config::init_for_test();

    let middleware = TenantSecretMiddleware;
    let srv = test::init_service(
        App::new()
            .wrap(middleware)
            .route("/", web::get().to(|| async { "ok" })),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(("X-Tenant-Secret-Key", "wrong"))
        .to_request();

    let resp = test::call_service(&srv, req).await;

    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
}
