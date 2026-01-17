use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Tenants::Table)
                    .add_column(
                        ColumnDef::new(Tenants::ApiKey)
                            .string()
                            .string_len(255)
                            .unique_key()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Generate and set keys for existing tenants if any
        // ... handled in repository/app logic or manual sql if needed
        // For now just allow null and we will populate on start or manually.
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Tenants::Table)
                    .drop_column(Tenants::ApiKey)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Tenants {
    Table,
    ApiKey,
}
