//! Tenant Secret Key Authentication Middleware
//!
//! This middleware validates tenant secret keys from the X-Tenant-Secret-Key header
//! for tenant creation and bootstrapping operations.

use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse,
};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use log::debug;
use crate::domains::common::dtos::response_dto::ErrorResponseDTO;

/// Tenant secret key authentication middleware.
///
/// This middleware checks for a valid tenant secret key in the X-Tenant-Secret-Key header.
/// The expected key is read from the `TENANT_SECRET_KEY` environment variable.
#[derive(Clone)]
pub struct TenantSecretMiddleware;

impl<S, B> Transform<S, ServiceRequest> for TenantSecretMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = TenantSecretMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        use crate::domains::common::utils::config::Config;
        let tenant_secret_key = Config::get().tenant_secret_key.clone();
        ok(TenantSecretMiddlewareService { service, tenant_secret_key })  
    }
}

/// Service wrapper for tenant secret key validation.
#[derive(Clone)]
pub struct TenantSecretMiddlewareService<S> {
    service: S,
    tenant_secret_key: String,
}

impl<S, B> Service<ServiceRequest> for TenantSecretMiddlewareService<S>
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
        let expected_key = self.tenant_secret_key.clone();
        let path = req.path().to_string();
        let has_secret_key_header = req.headers().get("X-Tenant-Secret-Key").is_some();
        let secret_key_value = req
            .headers()
            .get("X-Tenant-Secret-Key")
            .and_then(|v| v.to_str().ok())
            .map(str::trim)
            .unwrap_or("");
        
        debug!(
            "[Middleware | TenantSecret] call - path='{}' x_tenant_secret_key_present={} key_len={} expected_key_set={}",
            path, has_secret_key_header, secret_key_value.len(), !expected_key.is_empty()
        );

        // Check if TENANT_SECRET_KEY is configured
        if expected_key.is_empty() {
            debug!("[Middleware | TenantSecret] TENANT_SECRET_KEY not configured for '{}'", path);
            let res = HttpResponse::InternalServerError()
                .insert_header((header::CONTENT_TYPE, "application/json"))
                .json(ErrorResponseDTO {
                    status: false,
                    message: "TENANT_SECRET_KEY not configured",
                    details: None::<()>,
                    result: None,
                });
            return Box::pin(async move { Ok(req.into_response(res.map_into_right_body())) });
        }

        // Validate the secret key
        let is_valid = secret_key_value == expected_key;
        if !is_valid {
            debug!("[Middleware | TenantSecret] Invalid tenant secret key for '{}'", path);
            let response = ErrorResponseDTO::<()> {
                status: false,
                message: "Unauthorized",
                details: None,
                result: None,
            };
            let res = HttpResponse::Unauthorized()
                .insert_header((header::CONTENT_TYPE, "application/json"))
                .json(response);
            return Box::pin(async move { Ok(req.into_response(res.map_into_right_body())) });
        }

        debug!("[Middleware | TenantSecret] Authorized request to '{}'", path);
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}
