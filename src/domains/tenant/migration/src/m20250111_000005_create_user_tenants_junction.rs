use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create user_tenants junction table for many-to-many relationship
        manager
            .create_table(
                Table::create()
                    .table(UserTenants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserTenants::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserTenants::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserTenants::TenantId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserTenants::Role)
                            .string()
                            .not_null()
                            .default("user"),
                    )
                    .col(
                        ColumnDef::new(UserTenants::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserTenants::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_tenants_user_id")
                            .from(UserTenants::Table, UserTenants::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_tenants_tenant_id")
                            .from(UserTenants::Table, UserTenants::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique constraint to prevent duplicate assignments
        manager
            .create_index(
                Index::create()
                    .name("idx_user_tenants_unique")
                    .table(UserTenants::Table)
                    .col(UserTenants::UserId)
                    .col(UserTenants::TenantId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop user_tenants table
        manager
            .drop_table(Table::drop().table(UserTenants::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserTenants {
    Table,
    Id,
    UserId,
    TenantId,
    Role,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Tenants {
    Table,
    Id,
}
