//! User Activity Logs Table Migration
//!
//! Creates the user_activity_logs table to track all user activities including
//! authentication events, profile updates, and errors. Supports nullable user_id
//! to log failed registration attempts.

use sea_orm_migration::prelude::*;

/// Migration to create the user_activity_logs table.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserActivityLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserActivityLogs::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(UserActivityLogs::UserId)
                            .uuid()
                            .null(), // Nullable to log failed registrations
                    )
                    .col(
                        ColumnDef::new(UserActivityLogs::ActivityType)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserActivityLogs::Status)
                            .string()
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserActivityLogs::ErrorMessage)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserActivityLogs::IpAddress)
                            .string()
                            .string_len(45)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserActivityLogs::UserAgent)
                            .string()
                            .string_len(512)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserActivityLogs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_activity_logs_user_id")
                            .from(UserActivityLogs::Table, UserActivityLogs::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on user_id for faster user activity lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_user_activity_logs_user_id")
                    .table(UserActivityLogs::Table)
                    .col(UserActivityLogs::UserId)
                    .to_owned(),
            )
            .await?;

        // Create index on created_at for time-based queries
        manager
            .create_index(
                Index::create()
                    .name("idx_user_activity_logs_created_at")
                    .table(UserActivityLogs::Table)
                    .col(UserActivityLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Create index on activity_type for filtering by activity
        manager
            .create_index(
                Index::create()
                    .name("idx_user_activity_logs_activity_type")
                    .table(UserActivityLogs::Table)
                    .col(UserActivityLogs::ActivityType)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserActivityLogs::Table).to_owned())
            .await
    }
}

/// Column identifiers for the user_activity_logs table.
#[derive(DeriveIden)]
enum UserActivityLogs {
    Table,
    Id,
    UserId,
    ActivityType,
    Status,
    ErrorMessage,
    IpAddress,
    UserAgent,
    CreatedAt,
}

/// Reference to users table for foreign key.
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
