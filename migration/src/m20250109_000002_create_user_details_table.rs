//! User Details Table Migration
//!
//! Creates the user_details table with a one-to-one relationship to the users table.
//! This table stores additional user information like full name, phone, address, etc.

use sea_orm_migration::prelude::*;

/// Migration to create the user_details table.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserDetails::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(UserDetails::UserId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserDetails::FullName)
                            .string()
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserDetails::PhoneNumber)
                            .string()
                            .string_len(20)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserDetails::Address)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserDetails::DateOfBirth)
                            .date()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserDetails::ProfilePictureUrl)
                            .string()
                            .string_len(500)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserDetails::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserDetails::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(UserDetails::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_details_user_id")
                            .from(UserDetails::Table, UserDetails::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserDetails::Table).to_owned())
            .await
    }
}

/// Column identifiers for the user_details table.
#[derive(DeriveIden)]
enum UserDetails {
    Table,
    Id,
    UserId,
    FullName,
    PhoneNumber,
    Address,
    DateOfBirth,
    ProfilePictureUrl,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}

/// Column identifiers for the users table (for foreign key reference).
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
