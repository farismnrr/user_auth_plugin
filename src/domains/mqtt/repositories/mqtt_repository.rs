use crate::domains::common::errors::AppError;
use crate::domains::common::infrastructures::rocksdb_connection::RocksDbCache;
use crate::domains::mqtt::dtos::mqtt_dto::CreateMqttUserRequest;
use crate::domains::mqtt::entities::mqtt_user::{Entity as MqttUserEntity, Model as MqttUser};
use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[async_trait]
pub trait MqttRepositoryTrait: Send + Sync {
    async fn create(
        &self,
        req: CreateMqttUserRequest,
        hashed_password: String,
    ) -> Result<MqttUser, AppError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<MqttUser>, AppError>;
    async fn find_all(&self) -> Result<Vec<MqttUser>, AppError>;
    async fn delete(&self, username: &str) -> Result<(), AppError>;
}

pub struct MqttRepository {
    db: Arc<DatabaseConnection>,
    cache: Arc<RocksDbCache>,
}

impl MqttRepository {
    pub fn new(db: Arc<DatabaseConnection>, cache: Arc<RocksDbCache>) -> Self {
        Self { db, cache }
    }
}

#[async_trait]
impl MqttRepositoryTrait for MqttRepository {
    async fn create(
        &self,
        req: CreateMqttUserRequest,
        hashed_password: String,
    ) -> Result<MqttUser, AppError> {
        let user = crate::domains::mqtt::entities::mqtt_user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(req.username.unwrap()),
            password: Set(hashed_password),
            is_superuser: Set(req.is_superuser.unwrap()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            deleted_at: Set(None),
        };

        let result = MqttUserEntity::insert(user.clone()).exec(&*self.db).await;

        match result {
            Ok(_) => Ok(user.try_into_model().unwrap()),
            Err(e) => match e {
                DbErr::Exec(RuntimeErr::SqlxError(sqlx::Error::Database(db_err)))
                | DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    let msg = db_err.message().to_lowercase();
                    if msg.contains("duplicate") || msg.contains("unique") {
                        Err(AppError::Conflict("Username already exists".to_string()))
                    } else {
                        Err(AppError::DatabaseError(db_err.to_string()))
                    }
                }
                _ => Err(AppError::DatabaseError(e.to_string())),
            },
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<MqttUser>, AppError> {
        let cache_key = format!("mqtt_user:{}", username);
        if let Some(cached_user) = self.cache.get::<MqttUser>(&cache_key) {
            return Ok(Some(cached_user));
        }

        let user = MqttUserEntity::find()
            .filter(crate::domains::mqtt::entities::mqtt_user::Column::Username.eq(username))
            .filter(crate::domains::mqtt::entities::mqtt_user::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(ref u) = user {
            // Cache for 1 hour
            self.cache.set(&cache_key, u, Duration::from_secs(3600));
        }

        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<MqttUser>, AppError> {
        let users = MqttUserEntity::find()
            .filter(crate::domains::mqtt::entities::mqtt_user::Column::DeletedAt.is_null())
            .order_by_desc(crate::domains::mqtt::entities::mqtt_user::Column::CreatedAt)
            .all(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(users)
    }

    async fn delete(&self, username: &str) -> Result<(), AppError> {
        let existing = self.find_by_username(username).await?;

        if let Some(user) = existing {
            let mut active: crate::domains::mqtt::entities::mqtt_user::ActiveModel =
                user.clone().into();
            active.deleted_at = Set(Some(chrono::Utc::now().into()));

            active
                .update(&*self.db)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            // Invalidate cache
            self.cache.del(&format!("mqtt_user:{}", username));
            Ok(())
        } else {
            Err(AppError::NotFound("MQTT User not found".to_string()))
        }
    }
}
