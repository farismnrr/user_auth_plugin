use crate::domains::common::dtos::response_dto::ErrorResponseDTO;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    web, Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use log::{debug, error};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use std::rc::Rc;

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
        use crate::domains::common::utils::config::Config;
        let api_key = Config::get().api_key.clone();
        ok(ApiKeyMiddlewareService {
            service: Rc::new(service),
            api_key,
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

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let expected_global_key = self.api_key.clone();
        let path = req.path().to_string();
        let api_key_value = req
            .headers()
            .get("X-API-Key")
            .and_then(|v| v.to_str().ok())
            .map(str::trim)
            .unwrap_or("")
            .to_string();

        let db = req.app_data::<web::Data<DatabaseConnection>>().cloned();
        let service = self.service.clone();

        Box::pin(async move {
            let mut resolved_tenant_id = None;

            if db.is_none() {
                error!("[Middleware | ApiKey] Database connection not found in app_data");
            }

            if let Some(db) = db {
                // 1. First check if it matches the global super key
                if !expected_global_key.is_empty() && api_key_value == expected_global_key {
                    debug!(
                        "[Middleware | ApiKey] Global API Key detected. Resolving default tenant."
                    );
                    // Fetch the first tenant created (acting as default tenant for simple API Key auth)
                    match crate::domains::tenant::entities::tenant::Entity::find()
                        .order_by_asc(crate::domains::tenant::entities::tenant::Column::CreatedAt)
                        .one(db.as_ref())
                        .await
                    {
                        Ok(Some(tenant)) => {
                            debug!(
                                "[Middleware | ApiKey] Resolved Default Tenant ID: {}",
                                tenant.id
                            );
                            resolved_tenant_id = Some(tenant.id);
                        }
                        Ok(None) => {
                            debug!(
                                "[Middleware | ApiKey] No tenant found in database for global key."
                            );
                        }
                        Err(e) => {
                            error!(
                                "[Middleware | ApiKey] Database error fetching default tenant: {}",
                                e
                            );
                        }
                    }
                } else if !api_key_value.is_empty() {
                    // 2. Check if it's a tenant-specific API Key
                    use sea_orm::ColumnTrait;
                    use sea_orm::QueryFilter;

                    debug!("[Middleware | ApiKey] Checking tenant-specific API Key.");
                    match crate::domains::tenant::entities::tenant::Entity::find()
                        .filter(
                            crate::domains::tenant::entities::tenant::Column::ApiKey
                                .eq(api_key_value),
                        )
                        .filter(
                            crate::domains::tenant::entities::tenant::Column::DeletedAt.is_null(),
                        )
                        .one(db.as_ref())
                        .await
                    {
                        Ok(Some(tenant)) => {
                            debug!(
                                "[Middleware | ApiKey] Resolved Tenant ID from specific key: {}",
                                tenant.id
                            );
                            resolved_tenant_id = Some(tenant.id);
                        }
                        Ok(None) => {
                            debug!("[Middleware | ApiKey] No tenant found with matching API Key.");
                        }
                        Err(e) => {
                            error!(
                                "[Middleware | ApiKey] Database error fetching tenant by key: {}",
                                e
                            );
                        }
                    }
                }
            }

            if let Some(tid) = resolved_tenant_id {
                debug!(
                    "[Middleware | ApiKey] Authorized request to '{}' with Tenant ID: {}",
                    path, tid
                );
                req.extensions_mut().insert(TenantId(tid));
                let res = service.call(req).await?;
                Ok(res.map_into_left_body())
            } else {
                debug!("[Middleware | ApiKey] Unauthorized request to '{}'", path);
                let res = HttpResponse::Unauthorized()
                    .insert_header((header::CONTENT_TYPE, "application/json"))
                    .json(ErrorResponseDTO {
                        status: false,
                        message: "Unauthorized",
                        details: None::<()>,
                        result: None,
                    });
                Ok(req.into_response(res.map_into_right_body()))
            }
        })
    }
}
