use crate::domains::common::errors::AppError;
use crate::domains::common::infrastructures::rocksdb_connection::RocksDbCache;
use crate::domains::user::entities::user_details::{
    self, Entity as UserDetailsEntity, Model as UserDetails,
};
use async_trait::async_trait;
use chrono::NaiveDate;
use sea_orm::*;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// Trait defining user_details repository operations.
///
/// This trait abstracts database operations for user_details management.
#[async_trait]
pub trait UserDetailsRepositoryTrait: Send + Sync {
    /// Creates a new user_details record with default profile picture.
    async fn create(&self, user_id: Uuid) -> Result<UserDetails, AppError>;

    /// Finds user_details by user_id.
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserDetails>, AppError>;

    /// Updates user_details text fields (excludes profile_picture_url).
    async fn update(
        &self,
        user_id: Uuid,
        full_name: Option<String>,
        phone_number: Option<String>,
        address: Option<String>,
        date_of_birth: Option<NaiveDate>,
    ) -> Result<UserDetails, AppError>;

    /// Updates only the profile_picture_url field.
    async fn update_profile_picture(
        &self,
        user_id: Uuid,
        profile_picture_url: String,
    ) -> Result<UserDetails, AppError>;
}

/// User details repository implementation using SeaORM.
///
/// This implementation provides PostgreSQL-backed user_details data access operations.
pub struct UserDetailsRepository {
    db: Arc<DatabaseConnection>,
    cache: Arc<RocksDbCache>,
}

impl UserDetailsRepository {
    /// Creates a new UserDetailsRepository instance.
    ///
    /// # Arguments
    ///
    /// * `db` - Arc-wrapped database connection
    pub fn new(db: Arc<DatabaseConnection>, cache: Arc<RocksDbCache>) -> Self {
        Self { db, cache }
    }
}

#[async_trait]
impl UserDetailsRepositoryTrait for UserDetailsRepository {
    async fn create(&self, user_id: Uuid) -> Result<UserDetails, AppError> {
        let user_details = user_details::ActiveModel {
            id: Set(Uuid::new_v4()), // Generate UUID in repository
            user_id: Set(user_id),
            profile_picture_url: Set(Some(
                "https://storage.googleapis.com/farismnrr-gclouds.appspot.com/default.png"
                    .to_string(),
            )),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            full_name: Set(None),
            phone_number: Set(None),
            address: Set(None),
            date_of_birth: Set(None),
            deleted_at: Set(None),
        };

        let result = UserDetailsEntity::insert(user_details.clone())
            .exec(&*self.db)
            .await;

        if let Err(e) = result {
            match e {
                DbErr::Exec(RuntimeErr::SqlxError(sqlx::Error::Database(db_err)))
                | DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    let msg = db_err.message().to_lowercase();
                    if msg.contains("duplicate") || msg.contains("unique") {
                        return Err(AppError::BadRequest(
                            "User details already exist for this user".to_string(),
                        ));
                    }
                    return Err(AppError::DatabaseError(db_err.to_string()));
                }
                _ => return Err(AppError::DatabaseError(e.to_string())),
            }
        }

        Ok(user_details.try_into_model().unwrap())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserDetails>, AppError> {
        let cache_key = format!("user_details:{}", user_id);
        if let Some(cached_details) = self.cache.get::<UserDetails>(&cache_key) {
            return Ok(Some(cached_details));
        }

        let user_details = UserDetailsEntity::find()
            .filter(user_details::Column::UserId.eq(user_id))
            .filter(user_details::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(ref d) = user_details {
            let ttl_secs = std::env::var("CACHE_TTL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse::<u64>()
                .unwrap_or(3600);
            self.cache.set(&cache_key, d, Duration::from_secs(ttl_secs));
        }

        Ok(user_details)
    }

    async fn update(
        &self,
        user_id: Uuid,
        full_name: Option<String>,
        phone_number: Option<String>,
        address: Option<String>,
        date_of_birth: Option<NaiveDate>,
    ) -> Result<UserDetails, AppError> {
        let existing = self.find_by_user_id(user_id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound(format!(
                "User details not found for user {}",
                user_id
            )));
        }

        let mut user_details: user_details::ActiveModel = existing.unwrap().into();

        if let Some(name) = full_name {
            user_details.full_name = Set(Some(name));
        }
        if let Some(phone) = phone_number {
            user_details.phone_number = Set(Some(phone));
        }
        if let Some(addr) = address {
            user_details.address = Set(Some(addr));
        }
        if let Some(dob) = date_of_birth {
            user_details.date_of_birth = Set(Some(dob));
        }

        user_details.updated_at = Set(chrono::Utc::now());

        let result = user_details
            .update(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.cache.del(&format!("user_details:{}", user_id));
        Ok(result)
    }

    async fn update_profile_picture(
        &self,
        user_id: Uuid,
        profile_picture_url: String,
    ) -> Result<UserDetails, AppError> {
        let existing = self.find_by_user_id(user_id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound(format!(
                "User details not found for user {}",
                user_id
            )));
        }

        let mut user_details: user_details::ActiveModel = existing.unwrap().into();
        user_details.profile_picture_url = Set(Some(profile_picture_url));
        user_details.updated_at = Set(chrono::Utc::now());

        let result = user_details
            .update(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.cache.del(&format!("user_details:{}", user_id));
        Ok(result)
    }
}
