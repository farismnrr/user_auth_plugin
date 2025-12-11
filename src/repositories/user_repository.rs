use crate::dtos::user_dto::{CreateUserRequest, UpdateUserRequest};
use crate::entities::user::{self, Entity as UserEntity, Model as User};
use crate::errors::AppError;
use crate::utils::password;
use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

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
    
    /// Finds a user by their email address.
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    
    /// Finds a user by their username.
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    
    /// Retrieves all users from the database.
    async fn find_all(&self) -> Result<Vec<User>, AppError>;
    
    /// Updates an existing user.
    async fn update(&self, id: Uuid, user: UpdateUserRequest) -> Result<User, AppError>;
    
    /// Deletes a user by their ID.
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}

/// User repository implementation using SeaORM.
///
/// This implementation provides PostgreSQL-backed user data access operations.
pub struct UserRepository {
    db: Arc<DatabaseConnection>,
}

impl UserRepository {
    /// Creates a new UserRepository instance.
    ///
    /// # Arguments
    ///
    /// * `db` - Arc-wrapped database connection
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, req: CreateUserRequest) -> Result<User, AppError> {
        let password_hash = password::hash_password(&req.password)?;

        let user = user::ActiveModel {
            username: Set(req.username.clone()),
            email: Set(req.email.clone()),
            password_hash: Set(password_hash),
            ..Default::default()
        };

        let result = UserEntity::insert(user)
            .exec_with_returning(&*self.db)
            .await
            .map_err(|e| match e {
                DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    if db_err.message().contains("duplicate") || db_err.message().contains("unique") {
                        AppError::Conflict("Username or email already exists".to_string())
                    } else {
                        AppError::DatabaseError(db_err.to_string())
                    }
                }
                _ => AppError::DatabaseError(e.to_string()),
            })?;

        Ok(result)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = UserEntity::find_by_id(id)
            .filter(user::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = UserEntity::find()
            .filter(user::Column::Email.eq(email))
            .filter(user::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = UserEntity::find()
            .filter(user::Column::Username.eq(username))
            .filter(user::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

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
            return Err(AppError::NotFound(format!("User with id {} not found", id)));
        }

        let mut user: user::ActiveModel = existing.unwrap().into();

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
                DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(db_err))) => {
                    if db_err.message().contains("duplicate") || db_err.message().contains("unique") {
                        AppError::Conflict("Username or email already exists".to_string())
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
            return Err(AppError::NotFound(format!("User with id {} not found", id)));
        }

        let mut user: user::ActiveModel = existing.unwrap().into();
        user.deleted_at = Set(Some(chrono::Utc::now()));

        user.update(&*self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
