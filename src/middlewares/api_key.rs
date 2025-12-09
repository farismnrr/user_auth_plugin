//! API Key Authentication Middleware
//!
//! This middleware validates API keys from the X-API-Key header for protected endpoints.

use std::env;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse,
};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use log::debug;
use crate::dtos::response_dto::ErrorResponseDTO;

/// API key authentication middleware.
///
/// This middleware checks for a valid API key in the X-API-Key header.
/// The expected API key is read from the `API_KEY` environment variable.
#[derive(Clone)]
pub struct ApiKeyMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let api_key = env::var("API_KEY").unwrap_or_default();
        ok(ApiKeyMiddlewareService { service, api_key })
    }
}

/// Service wrapper for API key validation.
#[derive(Clone)]
pub struct ApiKeyMiddlewareService<S> {
    service: S,
    api_key: String,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }


    fn call(&self, req: ServiceRequest) -> Self::Future {
        let expected_key = self.api_key.clone();
        let path = req.path().to_string();
        let has_api_key_header = req.headers().get("X-API-Key").is_some();
        let api_key_value = req
            .headers()
            .get("X-API-Key")
            .and_then(|v| v.to_str().ok())
            .map(str::trim)
            .unwrap_or("");
        debug!("[Middleware | ApiKey] call - path='{}' x_api_key_present={} key_len={} expected_key_set={}",
            path, has_api_key_header, api_key_value.len(), !expected_key.is_empty());

        let is_valid = !expected_key.is_empty() && api_key_value == expected_key;
        if !is_valid {
            debug!("[Middleware | ApiKey] Unauthorized request to '{}'", path);
            let res = HttpResponse::Unauthorized()
                .insert_header((header::CONTENT_TYPE, "application/json"))
                .json(ErrorResponseDTO {
                    success: false,
                    message: "Unauthorized",
                    details: None::<()>,
                    result: None,
                });
            return Box::pin(async move { Ok(req.into_response(res.map_into_right_body())) });
        }

        debug!("[Middleware | ApiKey] Authorized request to '{}'", path);
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}