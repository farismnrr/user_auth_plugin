
use crate::domains::common::errors::json_error_handler::*;
use actix_web::{error::JsonPayloadError, http::StatusCode, test};
use actix_web::Error as ActixError;

#[actix_web::test]
async fn test_json_error_handler_content_type() {
    let req = test::TestRequest::default().to_http_request();
    let err = JsonPayloadError::ContentType;
    
    let error_response: ActixError = json_error_handler(err, &req);
    let resp = error_response.error_response();
    
    assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[actix_web::test]
async fn test_json_error_handler_payload() {
    let req = test::TestRequest::default().to_http_request();
    // Simulate a payload error
    let err = JsonPayloadError::Overflow { limit: 1024 };
    
    let error_response: ActixError = json_error_handler(err, &req);
    let resp = error_response.error_response();
    
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
