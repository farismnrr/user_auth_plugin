use crate::dtos::tenant_dto::{CreateTenantRequest, TenantResponse, UpdateTenantRequest};
use crate::errors::AppError;
use crate::repositories::tenant_repository::TenantRepositoryTrait;
use std::sync::Arc;
use uuid::Uuid;

/// Tenant use case for business logic.
///
/// This use case handles tenant-related operations including validation
/// and orchestration of repository calls.
pub struct TenantUseCase {
    tenant_repo: Arc<dyn TenantRepositoryTrait>,
}

impl TenantUseCase {
    /// Creates a new TenantUseCase instance.
    ///
    /// # Arguments
    ///
    /// * `tenant_repo` - Arc-wrapped tenant repository
    pub fn new(tenant_repo: Arc<dyn TenantRepositoryTrait>) -> Self {
        Self { tenant_repo }
    }

    /// Creates a new tenant.
    ///
    /// If a tenant with the same name already exists, returns the existing tenant
    /// with 200 OK instead of throwing a conflict error.
    ///
    /// # Arguments
    ///
    /// * `req` - Create tenant request with name and description
    ///
    /// # Returns
    ///
    /// * `Result<TenantResponse, AppError>` - Created or existing tenant
    pub async fn create_tenant(&self, req: CreateTenantRequest) -> Result<TenantResponse, AppError> {
        // Validate name length
        if req.name.is_empty() || req.name.len() > 255 {
            return Err(AppError::ValidationError("Name must be between 1 and 255 characters".to_string()));
        }

        // Check if tenant name already exists
        if let Some(existing_tenant) = self.tenant_repo.find_by_name(&req.name).await? {
            // Return existing tenant instead of error
            return Ok(TenantResponse::from(existing_tenant));
        }

        // Create tenant
        let tenant = self.tenant_repo.create(req).await?;

        Ok(TenantResponse::from(tenant))
    }

    /// Gets a tenant by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Tenant UUID
    ///
    /// # Returns
    ///
    /// * `Result<TenantResponse, AppError>` - Tenant data or error
    pub async fn get_tenant(&self, id: Uuid) -> Result<TenantResponse, AppError> {
        let tenant = self
            .tenant_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Tenant with id {} not found", id)))?;

        Ok(TenantResponse::from(tenant))
    }

    /// Gets all active tenants.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<TenantResponse>, AppError>` - List of tenants or error
    pub async fn get_all_tenants(&self) -> Result<Vec<TenantResponse>, AppError> {
        let tenants = self.tenant_repo.find_all().await?;
        Ok(tenants.into_iter().map(TenantResponse::from).collect())
    }

    /// Updates a tenant.
    ///
    /// # Arguments
    ///
    /// * `id` - Tenant UUID
    /// * `req` - Update tenant request
    ///
    /// # Returns
    ///
    /// * `Result<TenantResponse, AppError>` - Updated tenant or error
    pub async fn update_tenant(
        &self,
        id: Uuid,
        req: UpdateTenantRequest,
    ) -> Result<TenantResponse, AppError> {
        // Validate name length if provided
        if let Some(ref name) = req.name {
            if name.is_empty() || name.len() > 255 {
                return Err(AppError::ValidationError("Name must be between 1 and 255 characters".to_string()));
            }
        }

        // Check if new name conflicts with existing tenant
        if let Some(ref name) = req.name {
            if let Some(existing) = self.tenant_repo.find_by_name(name).await? {
                if existing.id != id {
                    return Err(AppError::Conflict("Tenant name already exists".to_string()));
                }
            }
        }

        // Update tenant
        let tenant = self.tenant_repo.update(id, req).await?;

        Ok(TenantResponse::from(tenant))
    }

    /// Deletes a tenant.
    ///
    /// # Arguments
    ///
    /// * `id` - Tenant UUID
    ///
    /// # Returns
    ///
    /// * `Result<(), AppError>` - Success or error
    pub async fn delete_tenant(&self, id: Uuid) -> Result<(), AppError> {
        self.tenant_repo.delete(id).await
    }
}
