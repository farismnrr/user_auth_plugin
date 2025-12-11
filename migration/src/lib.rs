//! Database Migrations
//!
//! This module manages all database schema migrations using SeaORM migration framework.

pub use sea_orm_migration::prelude::*;

mod m20250108_000001_create_users_table;
mod m20250109_000002_create_user_details_table;
mod m20250110_000001_create_user_sessions_table;
mod m20250110_000002_create_user_activity_logs_table;
mod m20250111_000001_create_tenants_table;
mod m20250111_000005_create_user_tenants_junction;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250108_000001_create_users_table::Migration),
            Box::new(m20250109_000002_create_user_details_table::Migration),
            Box::new(m20250110_000001_create_user_sessions_table::Migration),
            Box::new(m20250110_000002_create_user_activity_logs_table::Migration),
            Box::new(m20250111_000001_create_tenants_table::Migration),
            Box::new(m20250111_000005_create_user_tenants_junction::Migration),
        ]
    }
}
