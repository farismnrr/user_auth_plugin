use super::powered_by_middleware::*;
use actix_web::{test, web, App, HttpResponse};

#[actix_web::test]
async fn test_powered_by_header() {
    let app = test::init_service(
        App::new()
            .wrap(PoweredByMiddleware)
            .route("/", web::get().to(|| HttpResponse::Ok())),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.headers().contains_key("x-powered-by"));
    assert_eq!(
        resp.headers()
            .get("x-powered-by")
            .unwrap()
            .to_str()
            .unwrap(),
        "IoTNet"
    );
}
