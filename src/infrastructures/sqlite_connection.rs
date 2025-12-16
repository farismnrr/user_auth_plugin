//! SQLite Database Connection Management
//!
//! This module handles SQLite database connection initialization, health monitoring,
//! and graceful shutdown. It uses SeaORM for database operations.

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;

/// Initializes a SQLite database connection from environment variables.
///
/// Use `DB_TYPE=sqlite` to select this backend.
/// Default database file is `user_auth_plugin.db` if `CORE_DB_NAME` is not set.
///
/// # Environment Variables
///
/// - `CORE_DB_NAME`: Database filename (default: "user_auth_plugin").
///   The system will append `.db` if not present? Or just use as is. 
///   To align with the request "default sqlite", we'll default to a local file.
///
/// # Errors
///
/// Returns an error if the connection fails.
pub async fn initialize() -> anyhow::Result<Arc<DatabaseConnection>> {
    let name = std::env::var("CORE_DB_NAME")
        .ok()
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "user_auth_plugin.sqlite".to_string());
    
    // Ensure .db or .sqlite extension if not present
    let db_filename = if name.ends_with(".db") || name.ends_with(".sqlite") {
        name
    } else {
        format!("{}.sqlite", name)
    };

    let url = format!("sqlite://{}?mode=rwc", db_filename);

    log::info!("Connecting to SQLite at {}", url);
    
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;
    
    log::info!("✅ SQLite connected successfully");
    Ok(Arc::new(db))
}

/// Monitors SQLite connection health and triggers shutdown on failure.
pub fn monitor_health(db: Arc<DatabaseConnection>, shutdown_tx: watch::Sender<bool>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60)); // Check less frequently for SQLite file
        interval.tick().await;
        
        loop {
            interval.tick().await;
            
            // Simple query to check if we can talk to the DB
            let ping_result = db
                .query_one(Statement::from_string(
                    sea_orm::DatabaseBackend::Sqlite,
                    "SELECT 1".to_string(),
                ))
                .await;
            
            if let Err(e) = ping_result {
                log::error!("❌ SQLite health check failed: {}", e);
                log::error!("🛑 Triggering server shutdown due to database error");
                let _ = shutdown_tx.send(true);
                break;
            }
        }
    });
}

/// Gracefully shuts down the SQLite connection.
pub async fn shutdown(db: Arc<DatabaseConnection>, shutdown_rx: watch::Receiver<bool>) {
    let mut rx = shutdown_rx;
    let _ = rx.changed().await;
    
    let db_owned = Arc::try_unwrap(db).unwrap_or_else(|arc| (*arc).clone());
    
    if let Err(e) = db_owned.close().await {
        log::error!("Error closing SQLite database connection: {}", e);
    }
    log::info!("SQLite connection closed");
}
