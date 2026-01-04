use actix_web::{error::JsonPayloadError, HttpResponse};
use crate::domains::common::dtos::response_dto::ErrorResponseDTO;

/// Custom error handler for JSON payload errors.
/// Returns 415 for Content-Type errors, 400 for other JSON errors.
pub fn json_error_handler(err: JsonPayloadError, _req: &actix_web::HttpRequest) -> actix_web::Error {
    let (status_code, message) = match &err {
        JsonPayloadError::ContentType => (
            actix_web::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Unsupported Media Type"
        ),
        _ => (
            actix_web::http::StatusCode::BAD_REQUEST,
            "Bad Request"
        ),
    };
    
    let resp = ErrorResponseDTO {
        status: false,
        message,
        details: Some(format!("{}", err)),
        result: None,
    };
    
    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::build(status_code).json(resp)
    ).into()
}
