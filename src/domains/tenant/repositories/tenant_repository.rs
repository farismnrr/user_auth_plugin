use crate::domains::common::errors::AppError;
use crate::domains::common::infrastructures::rocksdb_connection::RocksDbCache;
use crate::domains::tenant::dtos::tenant_dto::{CreateTenantRequest, UpdateTenantRequest};
use crate::domains::tenant::entities::tenant::{Entity as TenantEntity, Model as Tenant};
use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use std::time::Duration;
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

    /// Finds a tenant by their name including deleted ones.
    async fn find_by_name_with_deleted(&self, name: &str) -> Result<Option<Tenant>, AppError>;

    /// Retrieves all non-deleted tenants from the database.
    async fn find_all(&self) -> Result<Vec<Tenant>, AppError>;

    /// Updates an existing tenant.
    async fn update(&self, id: Uuid, tenant: UpdateTenantRequest) -> Result<Tenant, AppError>;

    /// Soft deletes a tenant by setting deleted_at timestamp.
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;

    /// Restores a soft-deleted tenant.
    async fn restore(&self, id: Uuid) -> Result<(), AppError>;
}

/// Tenant repository implementation using SeaORM.
///
/// This implementation provides PostgreSQL-backed tenant data access operations.
pub struct TenantRepository {
    db: Arc<DatabaseConnection>,
    cache: Arc<RocksDbCache>,
}

impl TenantRepository {
    /// Creates a new TenantRepository instance.
    ///
    /// # Arguments
    ///
    /// * `db` - Arc-wrapped database connection
    pub fn new(db: Arc<DatabaseConnection>, cache: Arc<RocksDbCache>) -> Self {
        Self { db, cache }
    }
}

#[async_trait]
impl TenantRepositoryTrait for TenantRepository {
    async fn create(&self, req: CreateTenantRequest) -> Result<Tenant, AppError> {
        let tenant = crate::domains::tenant::entities::tenant::ActiveModel {
            id: Set(Uuid::new_v4()), // Generate UUID in repository
            name: Set(req.name.clone()),
            description: Set(req.description.clone()),
            api_key: Set(Some(uuid::Uuid::new_v4().to_string().replace("-", "") + &uuid::Uuid::new_v4().to_string().replace("-", ""))), // 64 chars
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            deleted_at: Set(None),
        };

        let result = TenantEntity::insert(tenant.clone()).exec(&*self.db).await;

        if let Err(e) = result {
            match e {
                DbErr::Exec(RuntimeErr::SqlxError(sqlx::Error::Database(db_err)))
                | DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    let msg = db_err.message().to_lowercase();
                    if msg.contains("duplicate") || msg.contains("unique") {
                        return Err(AppError::Conflict("Tenant name already exists".to_string()));
                    }
                    return Err(AppError::DatabaseError(db_err.to_string()));
                }
                _ => return Err(AppError::DatabaseError(e.to_string())),
            }
        }

        Ok(tenant.try_into_model().unwrap())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>, AppError> {
        let cache_key = format!("tenant:{}", id);
        if let Some(cached_tenant) = self.cache.get::<Tenant>(&cache_key) {
            return Ok(Some(cached_tenant));
        }

        let tenant = TenantEntity::find_by_id(id)
            .filter(crate::domains::tenant::entities::tenant::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(ref t) = tenant {
            let ttl_secs = std::env::var("CACHE_TTL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse::<u64>()
                .unwrap_or(3600);
            self.cache.set(&cache_key, t, Duration::from_secs(ttl_secs));
        }

        Ok(tenant)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Tenant>, AppError> {
        let cache_key = format!("tenant:name:{}", name);
        if let Some(cached_tenant) = self.cache.get::<Tenant>(&cache_key) {
            return Ok(Some(cached_tenant));
        }

        let tenant = TenantEntity::find()
            .filter(crate::domains::tenant::entities::tenant::Column::Name.eq(name))
            .filter(crate::domains::tenant::entities::tenant::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(ref t) = tenant {
            let ttl_secs = std::env::var("CACHE_TTL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse::<u64>()
                .unwrap_or(3600);
            self.cache.set(&cache_key, t, Duration::from_secs(ttl_secs));
        }

        Ok(tenant)
    }

    async fn find_by_name_with_deleted(&self, name: &str) -> Result<Option<Tenant>, AppError> {
        let tenant = TenantEntity::find()
            .filter(crate::domains::tenant::entities::tenant::Column::Name.eq(name))
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tenant)
    }

    async fn find_all(&self) -> Result<Vec<Tenant>, AppError> {
        let tenants = TenantEntity::find()
            .filter(crate::domains::tenant::entities::tenant::Column::DeletedAt.is_null())
            .order_by_desc(crate::domains::tenant::entities::tenant::Column::CreatedAt)
            .all(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tenants)
    }

    async fn update(&self, id: Uuid, req: UpdateTenantRequest) -> Result<Tenant, AppError> {
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound("Tenant not found".to_string()));
        }

        let mut tenant: crate::domains::tenant::entities::tenant::ActiveModel =
            existing.clone().unwrap().into();

        if let Some(ref name) = req.name {
            tenant.name = Set(name.clone());
        }
        if let Some(ref description) = req.description {
            tenant.description = Set(Some(description.clone()));
        }

        tenant.updated_at = Set(chrono::Utc::now());

        let result = tenant.update(&*self.db).await;

        if let Err(e) = result {
            match e {
                DbErr::Exec(RuntimeErr::SqlxError(sqlx::Error::Database(db_err)))
                | DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    let msg = db_err.message().to_lowercase();
                    if msg.contains("duplicate") || msg.contains("unique") {
                        return Err(AppError::Conflict("Tenant name already exists".to_string()));
                    }
                    return Err(AppError::DatabaseError(db_err.to_string()));
                }
                _ => return Err(AppError::DatabaseError(e.to_string())),
            }
        }

        let result = result.unwrap();

        // Invalidate cache
        self.cache.del(&format!("tenant:{}", id));

        // Invalidate name cache. Use the fetched existing tenant to get the old name.
        if let Some(old_tenant) = existing {
            self.cache.del(&format!("tenant:name:{}", old_tenant.name));
        }
        self.cache.del(&format!("tenant:name:{}", result.name)); // Invalidate new name too

        Ok(result)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound("Tenant not found".to_string()));
        }

        let mut tenant: crate::domains::tenant::entities::tenant::ActiveModel =
            existing.clone().unwrap().into();
        tenant.deleted_at = Set(Some(chrono::Utc::now()));

        tenant
            .update(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Invalidate cache
        self.cache.del(&format!("tenant:{}", id));
        // We need to fetch it to invalidate name cache, but delete returns () and we only have id here.
        // Ideally we should have fetched it before deleting or just ignore the name cache if it's acceptable for it to expire naturally.
        // But wait, line 161 fetches `existing`.
        // `existing` is Option<Tenant>.
        if let Some(t) = existing {
            self.cache.del(&format!("tenant:name:{}", t.name));
        }

        Ok(())
    }

    async fn restore(&self, id: Uuid) -> Result<(), AppError> {
        let existing = TenantEntity::find_by_id(id)
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if existing.is_none() {
            return Err(AppError::NotFound(format!(
                "Tenant with id {} not found",
                id
            )));
        }

        // We use ActiveModel to update
        let mut tenant: crate::domains::tenant::entities::tenant::ActiveModel =
            existing.unwrap().into();
        tenant.deleted_at = Set(None);

        tenant
            .update(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
