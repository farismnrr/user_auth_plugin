//! Migration CLI Entry Point
//!
//! This binary provides a command-line interface for running database migrations.

use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
