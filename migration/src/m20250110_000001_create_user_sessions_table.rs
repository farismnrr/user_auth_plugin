//! User Sessions Table Migration
//!
//! Creates the user_sessions table to track multiple active sessions per user.
//! Each session is identified by a hashed refresh token and includes metadata
//! like user agent and IP address for security auditing.

use sea_orm_migration::prelude::*;

/// Migration to create the user_sessions table.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserSessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserSessions::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(UserSessions::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSessions::RefreshTokenHash)
                            .string()
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserSessions::UserAgent)
                            .string()
                            .string_len(512)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserSessions::IpAddress)
                            .string()
                            .string_len(45)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserSessions::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSessions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_sessions_user_id")
                            .from(UserSessions::Table, UserSessions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on user_id for faster lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_user_sessions_user_id")
                    .table(UserSessions::Table)
                    .col(UserSessions::UserId)
                    .to_owned(),
            )
            .await?;

        // Create index on expires_at for cleanup queries
        manager
            .create_index(
                Index::create()
                    .name("idx_user_sessions_expires_at")
                    .table(UserSessions::Table)
                    .col(UserSessions::ExpiresAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSessions::Table).to_owned())
            .await
    }
}

/// Column identifiers for the user_sessions table.
#[derive(DeriveIden)]
enum UserSessions {
    Table,
    Id,
    UserId,
    RefreshTokenHash,
    UserAgent,
    IpAddress,
    ExpiresAt,
    CreatedAt,
}

/// Reference to users table for foreign key.
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
