use crate::dtos::response_dto::SuccessResponseDTO;
use crate::dtos::tenant_dto::{CreateTenantRequest, UpdateTenantRequest};
use crate::errors::AppError;
use crate::usecases::tenant_usecase::TenantUseCase;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use uuid::Uuid;

/// Creates a new tenant.
///
/// Supports dual authentication:
/// 1. JWT Bearer token (for authenticated users)
/// 2. X-Tenant-Secret-Key header (for bootstrapping)
///
/// Authentication is handled by middleware.
///
/// # Arguments
///
/// * `tenant_usecase` - Tenant use case instance
/// * `req` - Create tenant request
///
/// # Returns
///
/// * `Result<impl Responder, AppError>` - 201 Created with tenant data or error
use serde_json::json;

pub async fn create_tenant(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
    req: web::Json<CreateTenantRequest>,
) -> Result<impl Responder, AppError> {
    let (tenant, created) = tenant_usecase.create_tenant(req.into_inner()).await?;
    
    if created {
        Ok(HttpResponse::Created().json(SuccessResponseDTO::new(
            "Tenant created successfully",
            json!({ 
                "tenant_id": tenant.id,
                "id": tenant.id 
            }),
        )))
    } else {
        Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
            "Tenant already exists",
            json!({ 
                "tenant_id": tenant.id,
                "id": tenant.id 
            }),
        )))
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
/// * `Result<impl Responder, AppError>` - 200 OK with tenant data or error
pub async fn get_tenant(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let tenant = tenant_usecase.get_tenant(id.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
        "Tenant retrieved successfully",
        json!({ "tenant": tenant }),
    )))
}

/// Gets all active tenants.
///
/// # Arguments
///
/// * `tenant_usecase` - Tenant use case instance
///
/// # Returns
///
/// * `Result<impl Responder, AppError>` - 200 OK with list of tenants or error
pub async fn get_all_tenants(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
) -> Result<impl Responder, AppError> {
    let tenants = tenant_usecase.get_all_tenants().await?;
    let total = tenants.len();
    
    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
        "Tenants retrieved successfully",
        json!({
            "tenants": tenants,
            "pagination": {
                "page": 1,
                "limit": 10,
                "total": total,
                "total_pages": if total == 0 { 0 } else { (total as f64 / 10.0).ceil() as u64 }
            }
        }),
    )))
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
/// * `Result<impl Responder, AppError>` - 200 OK with updated tenant data or error
pub async fn update_tenant(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
    id: web::Path<Uuid>,
    req: web::Json<UpdateTenantRequest>,
) -> Result<impl Responder, AppError> {
    let tenant = tenant_usecase.update_tenant(id.into_inner(), req.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
        "Tenant updated successfully",
        tenant,
    )))
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
/// * `Result<impl Responder, AppError>` - 204 No Content or error
pub async fn delete_tenant(
    tenant_usecase: web::Data<Arc<TenantUseCase>>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    log::info!("Attempting to delete tenant with ID: {}", id);
    tenant_usecase.delete_tenant(id.into_inner()).await?;
    
    Ok(HttpResponse::NoContent().finish())
}
