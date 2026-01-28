use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct M20240523000001CreateMqttUsersTable;

#[async_trait::async_trait]
impl MigrationTrait for M20240523000001CreateMqttUsersTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MqttUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MqttUsers::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MqttUsers::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(MqttUsers::Password).string().not_null())
                    .col(
                        ColumnDef::new(MqttUsers::IsSuperuser)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(MqttUsers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MqttUsers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MqttUsers::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MqttUsers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MqttUsers {
    Table,
    Id,
    Username,
    Password,
    IsSuperuser,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
