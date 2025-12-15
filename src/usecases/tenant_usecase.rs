use crate::dtos::tenant_dto::{CreateTenantRequest, TenantResponse, UpdateTenantRequest};
use crate::errors::{AppError, ValidationDetail};
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
    /// * `Result<(TenantResponse, bool), AppError>` - (Tenant data, created_flag)
    pub async fn create_tenant(&self, req: CreateTenantRequest) -> Result<(TenantResponse, bool), AppError> {
        // Validate name length
        if req.name.is_empty() {
            return Err(AppError::ValidationError(
                "Validation Error".to_string(),
                Some(vec![ValidationDetail {
                    field: "name".to_string(),
                    message: "Name cannot be empty".to_string(),
                }]),
            ));
        }
        
        if req.name.len() > 255 {
            return Err(AppError::ValidationError(
                "Validation Error".to_string(),
                Some(vec![ValidationDetail {
                    field: "name".to_string(),
                    message: "Name too long".to_string(),
                }]),
            ));
        }

        // Check if tenant name already exists (including deleted)
        if let Some(existing_tenant) = self.tenant_repo.find_by_name_with_deleted(&req.name).await? {
            // If tenant is deleted, restore it
            if existing_tenant.deleted_at.is_some() {
                let updated_req = UpdateTenantRequest {
                    name: Some(req.name),
                    description: req.description.clone(),
                };
                
                // We need to support "restoring" in repository or handle it via update.
                // However, standard update checks for id existence first.
                // Since we found it, it exists.
                // We need to set deleted_at to None.
                // But update DTO doesn't support setting deleted_at currently.
                // We might need a restore method in repository.
                // OR we can manually update via repository if we expose a method, OR assume standard update handles it?
                // Standard update logic in repo:
                // tenant.updated_at = now
                // tenant.name = ...
                // It does NOT touch deleted_at.
                
                // Let's implement restore in repository is cleanest.
                // For now, let's assume we add restore method.
                self.tenant_repo.restore(existing_tenant.id).await?;
                
                // After restore, we might want to update description if provided.
                if req.description.is_some() {
                    self.tenant_repo.update(existing_tenant.id, updated_req).await?;
                }
                
                let restored = self.tenant_repo.find_by_id(existing_tenant.id).await?.unwrap();
                return Ok((TenantResponse::from(restored), true)); // Treated as created (restored)
            }

            // Return existing active tenant with false flag (not created)
            return Ok((TenantResponse::from(existing_tenant), false));
        }

        // Create tenant
        let tenant = self.tenant_repo.create(req).await?;

        Ok((TenantResponse::from(tenant), true))
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
            .ok_or_else(|| AppError::NotFound("Tenant not found".to_string()))?;

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
            if name.is_empty() {
                return Err(AppError::ValidationError(
                    "Validation Error".to_string(),
                    Some(vec![ValidationDetail {
                        field: "name".to_string(),
                        message: "Name cannot be empty".to_string(),
                    }]),
                ));
            }
            if name.len() > 255 {
                return Err(AppError::ValidationError(
                    "Validation Error".to_string(),
                    Some(vec![ValidationDetail {
                        field: "name".to_string(),
                        message: "Name too long".to_string(),
                    }]),
                ));
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
