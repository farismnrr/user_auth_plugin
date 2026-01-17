use crate::domains::common::errors::AppError;
use actix_web::{error::ResponseError, http::StatusCode};

#[test]
fn test_app_error_status_codes() {
    assert_eq!(
        AppError::NotFound("".into()).status_code(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        AppError::BadRequest("".into()).status_code(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        AppError::Unauthorized("".into()).status_code(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        AppError::InternalError("".into()).status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        AppError::ValidationError("".into(), None).status_code(),
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        AppError::DatabaseError("".into()).status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        AppError::Conflict("".into()).status_code(),
        StatusCode::CONFLICT
    );
    assert_eq!(
        AppError::Forbidden("".into()).status_code(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        AppError::PayloadTooLarge("".into()).status_code(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
}

#[test]
fn test_app_error_display() {
    let err = AppError::BadRequest("Bad input".to_string());
    assert_eq!(format!("{}", err), "Bad input");
}
