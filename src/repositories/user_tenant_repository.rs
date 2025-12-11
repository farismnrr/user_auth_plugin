use crate::entities::{user_tenant, tenant, user};
use crate::errors::AppError;
use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait UserTenantRepositoryTrait: Send + Sync {
    async fn add_user_to_tenant(&self, user_id: Uuid, tenant_id: Uuid, role: String) -> Result<(), AppError>;
    async fn get_user_role_in_tenant(&self, user_id: Uuid, tenant_id: Uuid) -> Result<Option<String>, AppError>;
}

pub struct UserTenantRepository {
    db: Arc<DatabaseConnection>,
}

impl UserTenantRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserTenantRepositoryTrait for UserTenantRepository {
    async fn add_user_to_tenant(&self, user_id: Uuid, tenant_id: Uuid, role: String) -> Result<(), AppError> {
        let user_tenant = user_tenant::ActiveModel {
            user_id: Set(user_id),
            tenant_id: Set(tenant_id),
            role: Set(role),
            ..Default::default()
        };

        user_tenant::Entity::insert(user_tenant)
            .exec(&*self.db)
            .await
            .map_err(|e| match e {
                DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    if db_err.message().contains("duplicate") || db_err.message().contains("unique") {
                        AppError::Conflict("User already assigned to this tenant".to_string())
                    } else {
                        AppError::DatabaseError(db_err.to_string())
                    }
                }
                _ => AppError::DatabaseError(e.to_string()),
            })?;

        Ok(())
    }

    async fn get_user_role_in_tenant(&self, user_id: Uuid, tenant_id: Uuid) -> Result<Option<String>, AppError> {
        let result = user_tenant::Entity::find()
            .filter(user_tenant::Column::UserId.eq(user_id))
            .filter(user_tenant::Column::TenantId.eq(tenant_id))
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(result.map(|ut| ut.role))
    }
}
