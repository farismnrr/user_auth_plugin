
use super::request_logger_middleware::*;
use actix_web::{test, web, App, HttpResponse};

#[actix_web::test]
async fn test_request_logger_middleware() {
    let app = test::init_service(
        App::new()
            .wrap(RequestLoggerMiddleware)
            .route("/", web::get().to(|| HttpResponse::Ok()))
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
}
