use crate::domains::user::entities::user_session::{self, Entity as UserSessionEntity, Model as UserSession};
use crate::domains::common::errors::AppError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

/// Trait defining user session repository operations.
///
/// This trait abstracts database operations for session management, enabling
/// multi-device login tracking and session lifecycle management.
#[async_trait]
pub trait UserSessionRepositoryTrait: Send + Sync {
    /// Creates a new session in the database.
    async fn create_session(
        &self,
        user_id: Uuid,
        refresh_token_hash: String,
        user_agent: Option<String>,
        ip_address: Option<String>,
        expires_at: DateTime<Utc>,
    ) -> Result<UserSession, AppError>;

    /// Finds a session by refresh token hash.
    async fn find_by_refresh_token_hash(
        &self,
        hash: &str,
    ) -> Result<Option<UserSession>, AppError>;

    /// Deletes a specific session by ID.
    async fn delete_session(&self, id: Uuid) -> Result<(), AppError>;

    /// Deletes all sessions for a specific user.
    async fn delete_all_sessions_for_user(&self, user_id: Uuid) -> Result<(), AppError>;
}

/// User session repository implementation using SeaORM.
///
/// This implementation provides PostgreSQL-backed session data access operations.
pub struct UserSessionRepository {
    db: Arc<DatabaseConnection>,
}

impl UserSessionRepository {
    /// Creates a new UserSessionRepository instance.
    ///
    /// # Arguments
    ///
    /// * `db` - Arc-wrapped database connection
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserSessionRepositoryTrait for UserSessionRepository {
    async fn create_session(
        &self,
        user_id: Uuid,
        refresh_token_hash: String,
        user_agent: Option<String>,
        ip_address: Option<String>,
        expires_at: DateTime<Utc>,
    ) -> Result<UserSession, AppError> {
        let session = user_session::ActiveModel {
            id: Set(Uuid::new_v4()),  // Generate UUID in repository
            user_id: Set(user_id),
            refresh_token_hash: Set(refresh_token_hash),
            user_agent: Set(user_agent),
            ip_address: Set(ip_address),
            expires_at: Set(expires_at),
            created_at: Set(Utc::now().into()),
            ..Default::default()
        };

        UserSessionEntity::insert(session.clone())
            .exec(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(session.try_into_model().unwrap())
    }

    async fn find_by_refresh_token_hash(
        &self,
        hash: &str,
    ) -> Result<Option<UserSession>, AppError> {
        let session = UserSessionEntity::find()
            .filter(user_session::Column::RefreshTokenHash.eq(hash))
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(session)
    }

    async fn delete_session(&self, id: Uuid) -> Result<(), AppError> {
        let result = UserSessionEntity::delete_by_id(id)
            .exec(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Session with id {} not found",
                id
            )));
        }

        Ok(())
    }

    async fn delete_all_sessions_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        UserSessionEntity::delete_many()
            .filter(user_session::Column::UserId.eq(user_id))
            .exec(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
