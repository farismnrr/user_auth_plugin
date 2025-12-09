//! HTTP//! Middleware Layer
//!
//! This module contains middleware components for request/response processing.
//! Middlewares handle cross-cutting concerns like authentication, logging, and API key validation.

pub mod api_key;
pub mod logger_request;
pub mod powered_by;
pub mod auth;
