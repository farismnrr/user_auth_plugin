use actix_web::{error::PathError, HttpResponse};
use crate::domains::common::dtos::response_dto::ErrorResponseDTO;

/// Custom error handler for Path extraction errors.
/// Returns 400 Bad Request with JSON body.
pub fn path_error_handler(err: PathError, _req: &actix_web::HttpRequest) -> actix_web::Error {
    let resp = ErrorResponseDTO {
        status: false,
        message: "Bad Request",
        details: Some(format!("{}", err)),
        result: None,
    };
    
    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(resp)
    ).into()
}
