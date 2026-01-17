//! PostgreSQL Database Connection Management
//!
//! This module handles PostgreSQL database connection initialization, health monitoring,
//! and graceful shutdown. It uses SeaORM for database operations and includes automatic
//! database creation if it doesn't exist.

use anyhow::Context;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;

/// Initializes a PostgreSQL database connection from environment variables.
///
/// This function reads database configuration from environment variables and establishes
/// a connection pool. If the target database doesn't exist, it will attempt to create it.
///
/// # Environment Variables
///
/// - `CORE_DB_HOST`: Database host (default: "127.0.0.1")
/// - `CORE_DB_PORT`: Database port (default: "5432")
/// - `CORE_DB_USER`: Database user (default: "postgres")
/// - `CORE_DB_PASS`: Database password (default: "postgres")
/// - `CORE_DB_NAME`: Database name (default: "user_auth_plugin")
///
/// # Errors
///
/// Returns an error if the connection fails or database creation fails.
pub async fn initialize() -> anyhow::Result<Arc<DatabaseConnection>> {
    dotenvy::dotenv().ok();

    use crate::domains::common::utils::config::Config;
    let config = Config::get();

    let host = &config.db_host;
    let port = &config.db_port;
    let user = &config.db_user;
    let pass = &config.db_pass;
    let name = &config.db_name;

    log::info!("Connecting to Postgres at {}:{}/{}", host, port, name);

    let db = try_connect(host, port, user, pass, name).await?;

    log::info!("✅ Postgres connected successfully");
    Ok(Arc::new(db))
}

/// Attempts to connect to PostgreSQL, creating the database if it doesn't exist.
///
/// This function first tries to connect directly to the target database. If that fails,
/// it connects to the default 'postgres' database to check if the target database exists,
/// and creates it if necessary.
async fn try_connect(
    host: &str,
    port: &str,
    user: &str,
    pass: &str,
    name: &str,
) -> anyhow::Result<DatabaseConnection> {
    let url = format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, name);

    let mut opt = ConnectOptions::new(url.clone());
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging_level(log::LevelFilter::Debug);

    match Database::connect(opt.clone()).await {
        Ok(db) => return Ok(db),
        Err(e) => {
            log::warn!(
                "Direct connect to target DB failed: {}. Trying to create DB if missing",
                e
            );
        }
    }

    let admin_url = format!("postgres://{}:{}@{}:{}/postgres", user, pass, host, port);
    let mut admin_opt = ConnectOptions::new(admin_url);
    admin_opt
        .max_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .sqlx_logging_level(log::LevelFilter::Debug);

    let admin_db = Database::connect(admin_opt)
        .await
        .context("Failed to connect to Postgres admin DB. Is Postgres running?")?;

    let exists: Option<i64> = admin_db
        .query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            format!("SELECT 1 FROM pg_database WHERE datname = '{}'", name),
        ))
        .await
        .context("Failed to check existing databases")?
        .and_then(|row| row.try_get("", "?column?").ok());

    if exists.is_none() {
        log::info!("Creating Postgres database: {}", name);
        admin_db
            .execute(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                format!("CREATE DATABASE \"{}\"", name),
            ))
            .await
            .context("Failed to create database")?;
    }

    drop(admin_db);

    let db = Database::connect(opt)
        .await
        .context("Failed connecting to Postgres (after create)")?;

    Ok(db)
}

/// Monitors PostgreSQL connection health and triggers shutdown on failure.
///
/// This function spawns a background task that periodically pings the database.
/// If the health check fails, it signals the application to shut down gracefully.
///
/// # Arguments
///
/// * `db` - Arc-wrapped database connection to monitor
/// * `shutdown_tx` - Channel sender to trigger shutdown signal
pub fn monitor_health(db: Arc<DatabaseConnection>, shutdown_tx: watch::Sender<bool>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        interval.tick().await;

        loop {
            interval.tick().await;

            let ping_result = db
                .query_one(Statement::from_string(
                    sea_orm::DatabaseBackend::Postgres,
                    "SELECT 1".to_string(),
                ))
                .await;

            if let Err(e) = ping_result {
                log::error!("❌ Postgres health check failed: {}", e);
                log::error!("🛑 Triggering server shutdown due to database disconnection");
                let _ = shutdown_tx.send(true);
                break;
            }
        }
    });
}

/// Gracefully shuts down the PostgreSQL connection.
///
/// This function waits for the shutdown signal and then closes the database connection.
///
/// # Arguments
///
/// * `db` - Arc-wrapped database connection to close
/// * `shutdown_rx` - Channel receiver to wait for shutdown signal
pub async fn shutdown(db: Arc<DatabaseConnection>, shutdown_rx: watch::Receiver<bool>) {
    let mut rx = shutdown_rx;
    let _ = rx.changed().await;

    let db_owned = Arc::try_unwrap(db).unwrap_or_else(|arc| (*arc).clone());

    if let Err(e) = db_owned.close().await {
        log::error!("Error closing database connection: {}", e);
    }
    log::info!("Postgres connection closed");
}
