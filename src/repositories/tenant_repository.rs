use crate::dtos::tenant_dto::{CreateTenantRequest, UpdateTenantRequest};
use crate::entities::tenant::{self, Entity as TenantEntity, Model as Tenant};
use crate::errors::AppError;
use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

/// Trait defining tenant repository operations.
///
/// This trait abstracts database operations for tenant management.
#[async_trait]
pub trait TenantRepositoryTrait: Send + Sync {
    /// Creates a new tenant in the database.
    async fn create(&self, tenant: CreateTenantRequest) -> Result<Tenant, AppError>;
    
    /// Finds a tenant by their ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>, AppError>;
    
    /// Finds a tenant by their name.
    async fn find_by_name(&self, name: &str) -> Result<Option<Tenant>, AppError>;
    
    /// Retrieves all non-deleted tenants from the database.
    async fn find_all(&self) -> Result<Vec<Tenant>, AppError>;
    
    /// Updates an existing tenant.
    async fn update(&self, id: Uuid, tenant: UpdateTenantRequest) -> Result<Tenant, AppError>;
    
    /// Soft deletes a tenant by setting deleted_at timestamp.
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}

/// Tenant repository implementation using SeaORM.
///
/// This implementation provides PostgreSQL-backed tenant data access operations.
pub struct TenantRepository {
    db: Arc<DatabaseConnection>,
}

impl TenantRepository {
    /// Creates a new TenantRepository instance.
    ///
    /// # Arguments
    ///
    /// * `db` - Arc-wrapped database connection
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TenantRepositoryTrait for TenantRepository {
    async fn create(&self, req: CreateTenantRequest) -> Result<Tenant, AppError> {
        let tenant = tenant::ActiveModel {
            name: Set(req.name.clone()),
            description: Set(req.description.clone()),
            ..Default::default()
        };

        let result = TenantEntity::insert(tenant)
            .exec_with_returning(&*self.db)
            .await
            .map_err(|e| match e {
                DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    if db_err.message().contains("duplicate") || db_err.message().contains("unique") {
                        AppError::Conflict("Tenant name already exists".to_string())
                    } else {
                        AppError::DatabaseError(db_err.to_string())
                    }
                }
                _ => AppError::DatabaseError(e.to_string()),
            })?;

        Ok(result)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>, AppError> {
        let tenant = TenantEntity::find_by_id(id)
            .filter(tenant::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tenant)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Tenant>, AppError> {
        let tenant = TenantEntity::find()
            .filter(tenant::Column::Name.eq(name))
            .filter(tenant::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tenant)
    }

    async fn find_all(&self) -> Result<Vec<Tenant>, AppError> {
        let tenants = TenantEntity::find()
            .filter(tenant::Column::DeletedAt.is_null())
            .order_by_desc(tenant::Column::CreatedAt)
            .all(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tenants)
    }

    async fn update(&self, id: Uuid, req: UpdateTenantRequest) -> Result<Tenant, AppError> {
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound(format!("Tenant with id {} not found", id)));
        }

        let mut tenant: tenant::ActiveModel = existing.unwrap().into();

        if let Some(ref name) = req.name {
            tenant.name = Set(name.clone());
        }
        if let Some(ref description) = req.description {
            tenant.description = Set(Some(description.clone()));
        }

        tenant.updated_at = Set(chrono::Utc::now());

        let result = tenant
            .update(&*self.db)
            .await
            .map_err(|e| match e {
                DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    if db_err.message().contains("duplicate") || db_err.message().contains("unique") {
                        AppError::Conflict("Tenant name already exists".to_string())
                    } else {
                        AppError::DatabaseError(db_err.to_string())
                    }
                }
                _ => AppError::DatabaseError(e.to_string()),
            })?;

        Ok(result)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound(format!("Tenant with id {} not found", id)));
        }

        let mut tenant: tenant::ActiveModel = existing.unwrap().into();
        tenant.deleted_at = Set(Some(chrono::Utc::now()));

        tenant
            .update(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
