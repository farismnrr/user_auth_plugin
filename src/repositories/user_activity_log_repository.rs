use crate::entities::user_activity_log::{
    self, Entity as UserActivityLogEntity, Model as UserActivityLog,
};
use crate::errors::AppError;
use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

/// Trait defining user activity log repository operations.
///
/// This trait abstracts database operations for activity logging, providing
/// an audit trail for all user actions and authentication events.
#[async_trait]
pub trait UserActivityLogRepositoryTrait: Send + Sync {
    /// Logs a user activity.
    async fn log_activity(
        &self,
        user_id: Option<Uuid>,
        activity_type: String,
        status: String,
        error_message: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<UserActivityLog, AppError>;
}

/// User activity log repository implementation using SeaORM.
///
/// This implementation provides PostgreSQL-backed activity logging operations.
pub struct UserActivityLogRepository {
    db: Arc<DatabaseConnection>,
}

impl UserActivityLogRepository {
    /// Creates a new UserActivityLogRepository instance.
    ///
    /// # Arguments
    ///
    /// * `db` - Arc-wrapped database connection
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserActivityLogRepositoryTrait for UserActivityLogRepository {
    async fn log_activity(
        &self,
        user_id: Option<Uuid>,
        activity_type: String,
        status: String,
        error_message: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<UserActivityLog, AppError> {
        let log = user_activity_log::ActiveModel {
            id: Set(Uuid::new_v4()),  // Generate UUID in repository
            user_id: Set(user_id),
            activity_type: Set(activity_type),
            status: Set(status),
            error_message: Set(error_message),
            ip_address: Set(ip_address),
            user_agent: Set(user_agent),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        let result = UserActivityLogEntity::insert(log.clone())
            .exec(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(log.try_into_model().unwrap())
    }
}
