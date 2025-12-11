use crate::dtos::response_dto::SuccessResponseDTO;
use crate::dtos::tenant_dto::{CreateTenantRequest, UpdateTenantRequest};
use crate::usecases::tenant_usecase::TenantUseCase;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web::error::ResponseError;
use std::sync::Arc;
use uuid::Uuid;

/// Creates a new tenant.
///
/// # Arguments
///
/// * `tenant_usecase` - Tenant use case instance
/// * `req` - Create tenant request
///
/// # Returns
///
/// * `HttpResponse` - 201 Created with tenant data or error
#[post("/tenants")]
pub async fn create_tenant(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
    req: web::Json<CreateTenantRequest>,
) -> HttpResponse {
    match tenant_usecase.create_tenant(req.into_inner()).await {
        Ok(tenant) => HttpResponse::Created().json(SuccessResponseDTO::new("Tenant created successfully", tenant)),
        Err(e) => e.error_response(),
    }
}

/// Gets a tenant by ID.
///
/// # Arguments
///
/// * `tenant_usecase` - Tenant use case instance
/// * `id` - Tenant UUID path parameter
///
/// # Returns
///
/// * `HttpResponse` - 200 OK with tenant data or error
#[get("/tenants/{id}")]
pub async fn get_tenant(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    match tenant_usecase.get_tenant(id.into_inner()).await {
        Ok(tenant) => HttpResponse::Ok().json(SuccessResponseDTO::new("Tenant retrieved successfully", tenant)),
        Err(e) => e.error_response(),
    }
}

/// Gets all active tenants.
///
/// # Arguments
///
/// * `tenant_usecase` - Tenant use case instance
///
/// # Returns
///
/// * `HttpResponse` - 200 OK with list of tenants or error
#[get("/tenants")]
pub async fn get_all_tenants(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
) -> HttpResponse {
    match tenant_usecase.get_all_tenants().await {
        Ok(tenants) => HttpResponse::Ok().json(SuccessResponseDTO::new("Tenants retrieved successfully", tenants)),
        Err(e) => e.error_response(),
    }
}

/// Updates a tenant.
///
/// # Arguments
///
/// * `tenant_usecase` - Tenant use case instance
/// * `id` - Tenant UUID path parameter
/// * `req` - Update tenant request
///
/// # Returns
///
/// * `HttpResponse` - 200 OK with updated tenant data or error
#[put("/tenants/{id}")]
pub async fn update_tenant(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
    id: web::Path<Uuid>,
    req: web::Json<UpdateTenantRequest>,
) -> HttpResponse {
    match tenant_usecase.update_tenant(id.into_inner(), req.into_inner()).await {
        Ok(tenant) => HttpResponse::Ok().json(SuccessResponseDTO::new("Tenant updated successfully", tenant)),
        Err(e) => e.error_response(),
    }
}

/// Deletes a tenant.
///
/// # Arguments
///
/// * `tenant_usecase` - Tenant use case instance
/// * `id` - Tenant UUID path parameter
///
/// # Returns
///
/// * `HttpResponse` - 204 No Content or error
#[delete("/tenants/{id}")]
pub async fn delete_tenant(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    match tenant_usecase.delete_tenant(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}
