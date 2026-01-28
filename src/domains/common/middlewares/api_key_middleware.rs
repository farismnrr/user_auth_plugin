use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;

use crate::domains::common::errors::AppError;
use crate::domains::common::utils::config::Config;

pub struct ApiKeyMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddlewareMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ApiKeyMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers().clone();

        let api_key_header = headers.get("X-API-Key");
        let valid_key = &Config::get().api_key;

        if let Some(key) = api_key_header {
            if let Ok(key_str) = key.to_str() {
                if key_str == valid_key {
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    });
                }
            }
        }

        // Return 401 if missing or invalid
        // Need to convert AppError::Unauthorized to ServiceResponse error or modify request?
        // Usually, middleware returns strict response.
        // We can use AppError here but matching return types of Service acts weird sometimes.
        // Let's return strict HttpResponse and turn into Error.

        // This stops the chain
        Box::pin(async move {
            Err(Error::from(AppError::Unauthorized(
                "Unauthorized".to_string(),
            )))
        })
    }
}
