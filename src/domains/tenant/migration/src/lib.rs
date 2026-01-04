use sea_orm_migration::prelude::*;

mod m20250111_000001_create_tenants_table;
mod m20250111_000005_create_user_tenants_junction;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250111_000001_create_tenants_table::Migration),
            Box::new(m20250111_000005_create_user_tenants_junction::Migration),
        ]
    }
}
