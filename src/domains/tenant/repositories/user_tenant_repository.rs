use crate::domains::common::errors::AppError;
use crate::domains::common::infrastructures::rocksdb_connection::RocksDbCache;
use crate::domains::tenant::entities::user_tenant;
use async_trait::async_trait;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// Information about user's tenant association
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserTenantInfo {
    pub tenant_id: Uuid,
    pub role: String,
}

#[async_trait]
pub trait UserTenantRepositoryTrait: Send + Sync {
    async fn add_user_to_tenant(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        role: String,
    ) -> Result<(), AppError>;
    async fn get_user_roles_in_tenant(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<Vec<String>, AppError>;
    async fn get_all_tenants_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserTenantInfo>, AppError>;
}

pub struct UserTenantRepository {
    db: Arc<DatabaseConnection>,
    cache: Arc<RocksDbCache>,
}

impl UserTenantRepository {
    pub fn new(db: Arc<DatabaseConnection>, cache: Arc<RocksDbCache>) -> Self {
        Self { db, cache }
    }
}

#[async_trait]
impl UserTenantRepositoryTrait for UserTenantRepository {
    async fn add_user_to_tenant(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        role: String,
    ) -> Result<(), AppError> {
        let user_tenant = user_tenant::ActiveModel {
            id: Set(Uuid::new_v4()), // Generate UUID in repository
            user_id: Set(user_id),
            tenant_id: Set(tenant_id),
            role: Set(role),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };

        let result = user_tenant::Entity::insert(user_tenant)
            .exec(&*self.db)
            .await;

        if let Err(e) = result {
            match e {
                DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    if db_err.message().contains("duplicate") || db_err.message().contains("unique")
                    {
                        return Err(AppError::Conflict(
                            "User already assigned to this tenant with this role".to_string(),
                        ));
                    }
                    return Err(AppError::DatabaseError(db_err.to_string()));
                }
                _ => return Err(AppError::DatabaseError(e.to_string())),
            }
        }

        // Invalidate cache
        let roles_cache_key = format!("user_roles:{}:{}", user_id, tenant_id);
        let tenants_cache_key = format!("user_all_tenants:{}", user_id);
        let _ = self.cache.del(&roles_cache_key);
        let _ = self.cache.del(&tenants_cache_key);

        Ok(())
    }

    async fn get_user_roles_in_tenant(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<Vec<String>, AppError> {
        let cache_key = format!("user_roles:{}:{}", user_id, tenant_id);
        if let Some(cached_roles) = self.cache.get::<Vec<String>>(&cache_key) {
            return Ok(cached_roles);
        }

        let results = user_tenant::Entity::find()
            .filter(user_tenant::Column::UserId.eq(user_id))
            .filter(user_tenant::Column::TenantId.eq(tenant_id))
            .all(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let roles: Vec<String> = results.into_iter().map(|ut| ut.role).collect();

        if !roles.is_empty() {
            let ttl_secs = std::env::var("CACHE_TTL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse::<u64>()
                .unwrap_or(3600);
            self.cache
                .set(&cache_key, &roles, Duration::from_secs(ttl_secs));
        }

        Ok(roles)
    }

    async fn get_all_tenants_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserTenantInfo>, AppError> {
        let cache_key = format!("user_all_tenants:{}", user_id);

        // Try cache first
        if let Some(cached) = self.cache.get::<Vec<UserTenantInfo>>(&cache_key) {
            log::debug!("Cache hit for user_all_tenants:{}", user_id);
            return Ok(cached);
        }

        log::debug!(
            "Cache miss for user_all_tenants:{}, querying database",
            user_id
        );

        // Query database
        let results = user_tenant::Entity::find()
            .filter(user_tenant::Column::UserId.eq(user_id))
            .all(&*self.db)
            .await
            .map_err(|e| {
                log::error!("Database error in get_all_tenants_for_user: {}", e);
                AppError::InternalError(format!("Database error: {}", e))
            })?;

        // Map to UserTenantInfo
        let tenant_infos: Vec<UserTenantInfo> = results
            .into_iter()
            .map(|ut| UserTenantInfo {
                tenant_id: ut.tenant_id,
                role: ut.role,
            })
            .collect();

        log::debug!(
            "Found {} tenant associations for user {}",
            tenant_infos.len(),
            user_id
        );

        // Cache for 5 minutes
        let _ = self
            .cache
            .set(&cache_key, &tenant_infos, Duration::from_secs(300));

        Ok(tenant_infos)
    }
}
