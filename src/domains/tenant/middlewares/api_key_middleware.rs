use std::env;
use std::rc::Rc;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse, web, HttpMessage,
};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use log::{debug, error};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use crate::domains::common::dtos::response_dto::ErrorResponseDTO;


/// API key authentication middleware.
///
/// This middleware checks for a valid API key in the X-API-Key header.
/// The expected API key is read from the `API_KEY` environment variable.
/// If valid, it also resolves the default tenant and injects `tenant_id` into request extensions.

/// Wrapper for Tenant ID in request extensions to avoid TypeMap conflicts with User ID
#[derive(Clone, Copy, Debug)]
pub struct TenantId(pub uuid::Uuid);

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
        ok(ApiKeyMiddlewareService { 
            service: Rc::new(service), 
            api_key 
        })
    }
}

/// Service wrapper for API key validation.
pub struct ApiKeyMiddlewareService<S> {
    service: Rc<S>,
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
                    status: false,
                    message: "Unauthorized",
                    details: None::<()>,
                    result: None,
                });
            return Box::pin(async move { Ok(req.into_response(res.map_into_right_body())) });
        }

        // Get DB connection to fetch default tenant
        let db = req.app_data::<web::Data<DatabaseConnection>>().cloned();
        let service = self.service.clone();

        Box::pin(async move {
            if db.is_none() {
                error!("[Middleware | ApiKey] Database connection not found in app_data");
            }

            if let Some(db) = db {
                // Fetch the first tenant created (acting as default tenant for simple API Key auth)
                match crate::domains::tenant::entities::tenant::Entity::find()
                    .order_by_asc(crate::domains::tenant::entities::tenant::Column::CreatedAt)
                    .one(db.as_ref())
                    .await
                {
                    Ok(Some(tenant)) => {
                        debug!("[Middleware | ApiKey] Resolved Tenant ID: {}", tenant.id);
                        req.extensions_mut().insert(TenantId(tenant.id));
                    }
                    Ok(None) => {
                        debug!("[Middleware | ApiKey] No tenant found in database. Proceeding without Tenant ID.");
                    }
                    Err(e) => {
                        error!("[Middleware | ApiKey] Database error fetching tenant: {}", e);
                    }
                }
            }

            debug!("[Middleware | ApiKey] Authorized request to '{}'", path);
            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}