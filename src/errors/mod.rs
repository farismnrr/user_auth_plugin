//! Error Handling
//!
//! This module defines the application's error types and their conversion to HTTP responses.
//! All errors implement the `ResponseError` trait for automatic HTTP error response generation.

pub mod json_error_handler;
pub mod path_error_handler;

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

/// Application-wide error type.
///
/// This enum represents all possible error conditions in the application.
/// Each variant maps to a specific HTTP status code and error response.
#[derive(Debug, Serialize, Clone)]
pub struct ValidationDetail {
    pub field: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
    ValidationError(String, Option<Vec<ValidationDetail>>),
    DatabaseError(String),
    Conflict(String),
    Forbidden(String),
    PayloadTooLarge(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::Unauthorized(msg) => write!(f, "{}", msg),
            AppError::InternalError(msg) => write!(f, "{}", msg),
            AppError::ValidationError(msg, _) => write!(f, "{}", msg),
            AppError::DatabaseError(msg) => write!(f, "{}", msg),
            AppError::Conflict(msg) => write!(f, "{}", msg),
            AppError::Forbidden(msg) => write!(f, "{}", msg),
            AppError::PayloadTooLarge(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AppError {}

#[derive(Serialize)]
struct ErrorResponse {
    status: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Vec<ValidationDetail>>,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_, _) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let details = match self {
            AppError::ValidationError(_, d) => d.clone(),
            _ => None,
        };
        
        let error_response = ErrorResponse {
            status: false,
            message: self.to_string(),
            details,
        };

        HttpResponse::build(status_code).json(error_response)
    }
}
