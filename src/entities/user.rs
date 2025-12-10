use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// User entity representing the users table in the database.
///
/// This SeaORM model maps to the `users` table and includes all user-related fields
/// including authentication credentials, role information, and timestamps.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::user_details::Entity")]
    UserDetails,
    #[sea_orm(has_many = "super::user_session::Entity")]
    UserSessions,
    #[sea_orm(has_many = "super::user_activity_log::Entity")]
    UserActivityLogs,
}

impl Related<super::user_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserDetails.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
