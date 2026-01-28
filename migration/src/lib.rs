pub use sea_orm_migration::prelude::*;

mod mqtt;
mod tenants;
mod users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // User domain migrations
            Box::new(users::M20250108CreateUsersTable),
            Box::new(users::M20250109CreateUserDetailsTable),
            Box::new(users::M20250110CreateUserSessionsTable),
            Box::new(users::M20250110CreateUserActivityLogsTable),
            // Tenant domain migrations
            Box::new(tenants::M20250111CreateTenantsTable),
            Box::new(tenants::M20250111CreateUserTenantsJunction),
            Box::new(tenants::M20250116AddApiKeyToTenants),
            Box::new(tenants::M20250118RelaxUserTenantsUniqueConstraint),
            // MQTT domain migrations
            Box::new(mqtt::M20240523000001CreateMqttUsersTable),
        ]
    }
}
