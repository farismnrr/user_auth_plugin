use crate::dtos::user_dto::{CreateUserRequest, UpdateUserRequest};
use crate::entities::user::{self, Entity as UserEntity, Model as User};
use crate::errors::AppError;
use crate::utils::password;
use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;
use crate::infrastructures::cache::RocksDbCache;
use std::time::Duration;

/// Trait defining user repository operations.
///
/// This trait abstracts database operations for user management, allowing for
/// different implementations (e.g., PostgreSQL, MySQL, in-memory for testing).
#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    /// Creates a new user in the database.
    async fn create(&self, user: CreateUserRequest) -> Result<User, AppError>;
    
    /// Finds a user by their ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    
    /// Finds a user by their username.
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    
    /// Retrieves all users from the database.
    async fn find_all(&self) -> Result<Vec<User>, AppError>;
    
    /// Updates an existing user.
    async fn update(&self, id: Uuid, user: UpdateUserRequest) -> Result<User, AppError>;
    
    /// Deletes a user by their ID.
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;

    /// Finds a user by email, including soft-deleted ones.
    async fn find_by_email_with_deleted(&self, email: &str) -> Result<Option<User>, AppError>;

    /// Finds a user by username, including soft-deleted ones.
    async fn find_by_username_with_deleted(&self, username: &str) -> Result<Option<User>, AppError>;

    /// Restores a soft-deleted user.
    async fn restore(&self, id: Uuid, req: CreateUserRequest) -> Result<User, AppError>;
}

/// User repository implementation using SeaORM.
///
/// This implementation provides PostgreSQL-backed user data access operations.
pub struct UserRepository {
    db: Arc<DatabaseConnection>,
    cache: Arc<RocksDbCache>,
}

impl UserRepository {
    /// Creates a new UserRepository instance.
    ///
    /// # Arguments
    ///
    /// * `db` - Arc-wrapped database connection
    pub fn new(db: Arc<DatabaseConnection>, cache: Arc<RocksDbCache>) -> Self {
        Self { db, cache }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, req: CreateUserRequest) -> Result<User, AppError> {
        let password_hash = password::hash_password(&req.password)?;

        let user = user::ActiveModel {
            id: Set(Uuid::new_v4()),  // Generate UUID in repository
            username: Set(req.username.clone()),
            email: Set(req.email.clone()),
            password_hash: Set(password_hash),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            deleted_at: Set(None),
            ..Default::default()
        };

        let result = UserEntity::insert(user.clone())
            .exec(&*self.db)
            .await
            .map_err(|e| match e {
                DbErr::Exec(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) | DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    let msg = db_err.message().to_lowercase();
                    if msg.contains("duplicate") || msg.contains("unique") {
                        AppError::Conflict("Username or email already exists".to_string())
                    } else {
                        AppError::DatabaseError(db_err.to_string())
                    }
                }
                _ => AppError::DatabaseError(e.to_string()),
            })?;

        Ok(user.try_into_model().unwrap())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let cache_key = format!("user:{}", id);
        if let Some(cached_user) = self.cache.get::<User>(&cache_key) {
            log::info!("CACHE HIT: {}", cache_key);
            return Ok(Some(cached_user));
        }

        let user = UserEntity::find_by_id(id)
            .filter(user::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(ref u) = user {
            self.cache.set(&cache_key, u, Duration::from_secs(3600));
        }

        Ok(user)
    }



    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let cache_key = format!("user:username:{}", username);
        if let Some(cached_user) = self.cache.get::<User>(&cache_key) {
             log::info!("CACHE HIT: {}", cache_key);
             return Ok(Some(cached_user));
        }

        let user = UserEntity::find()
            .filter(user::Column::Username.eq(username))
            .filter(user::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(ref u) = user {
             self.cache.set(&cache_key, u, Duration::from_secs(3600));
        }

        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let users = UserEntity::find()
            .filter(user::Column::DeletedAt.is_null())
            .order_by_desc(user::Column::CreatedAt)
            .all(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(users)
    }

    async fn update(&self, id: Uuid, req: UpdateUserRequest) -> Result<User, AppError> {
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        let mut user: user::ActiveModel = existing.clone().unwrap().into();

        if let Some(ref username) = req.username {
            user.username = Set(username.clone());
        }
        if let Some(ref email) = req.email {
            user.email = Set(email.clone());
        }
        if let Some(ref password) = req.password {
            let password_hash = password::hash_password(password)?;
            user.password_hash = Set(password_hash);
        }


        user.updated_at = Set(chrono::Utc::now());

        let result = user
            .update(&*self.db)
            .await
            .map_err(|e| match e {
                DbErr::Exec(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) | DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    let msg = db_err.message().to_lowercase();
                    if msg.contains("duplicate") || msg.contains("unique") {
                        AppError::Conflict("Username or email already exists".to_string())
                    } else {
                        AppError::DatabaseError(db_err.to_string())
                    }
                }
                _ => AppError::DatabaseError(e.to_string()),
            })?;

        // Invalidate cache
        self.cache.del(&format!("user:{}", id));
        // We might not know the old username easily without another DB call if it wasn't in cache or existing, 
        // but typically user update changes are critical enough. 
        // For stricter consistency, we could invalidate the username key if we knew it.
        // For now, at least ID cache is cleared.
        // Also if username was changed, invalidate old username key is tricky without previous value.
        // Since we loaded 'existing' earlier, we can use it.
        // Let's invalidate 'existing' username too.
        if let Some(old_user) = existing {
             self.cache.del(&format!("user:username:{}", old_user.username));
        }
        // Also invalidate new username just in case.
        self.cache.del(&format!("user:username:{}", result.username));

        Ok(result)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            // Idempotent delete: if not found, consider it deleted.
            log::warn!("User {} not found for deletion (already deleted?)", id);
            return Ok(());
        }

        let mut user: user::ActiveModel = existing.unwrap().into();
        user.deleted_at = Set(Some(chrono::Utc::now()));

        log::info!("Soft deleting user...");

        let res = user.update(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        log::info!("Soft delete successful. DeletedAt: {:?}", res.deleted_at);
        
        // Invalidate cache
        self.cache.del(&format!("user:{}", id));
        self.cache.del(&format!("user:username:{}", res.username)); // Invalidate username cache too

        Ok(())
    }

    async fn find_by_email_with_deleted(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = UserEntity::find()
            .filter(user::Column::Email.eq(email))
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        if user.is_some() {
            log::info!("FOUND user by email (inc deleted): {} -> {:?}", email, user.as_ref().unwrap().id);
        } else {
            log::info!("NOT FOUND user by email (inc deleted): {}", email);
        }
        Ok(user)
    }

    async fn find_by_username_with_deleted(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = UserEntity::find()
            .filter(user::Column::Username.eq(username))
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    async fn restore(&self, id: Uuid, req: CreateUserRequest) -> Result<User, AppError> {
        let existing = UserEntity::find_by_id(id)
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if existing.is_none() {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        let mut user: user::ActiveModel = existing.unwrap().into();
        let password_hash = password::hash_password(&req.password)?;

        user.username = Set(req.username);
        user.email = Set(req.email);
        user.password_hash = Set(password_hash);
        user.deleted_at = Set(None);
        user.updated_at = Set(chrono::Utc::now());

        let result = user.update(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }
}
