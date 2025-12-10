//! Database Migrations
//!
//! This module manages all database schema migrations using SeaORM migration framework.

pub use sea_orm_migration::prelude::*;

mod m20250108_000001_create_users_table;
mod m20250109_000001_add_role_to_users;
mod m20250109_000002_create_user_details_table;
mod m20250110_000001_create_user_sessions_table;
mod m20250110_000002_create_user_activity_logs_table;

/// Migration coordinator that registers all migrations in order.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250108_000001_create_users_table::Migration),
            Box::new(m20250109_000001_add_role_to_users::Migration),
            Box::new(m20250109_000002_create_user_details_table::Migration),
            Box::new(m20250110_000001_create_user_sessions_table::Migration),
            Box::new(m20250110_000002_create_user_activity_logs_table::Migration),
        ]
    }
}
