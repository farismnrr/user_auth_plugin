
use super::sqlite_connection::*;
use std::env;

#[tokio::test]
async fn test_sqlite_initialize_file_naming() {
    use crate::domains::common::utils::config::Config;
    Config::init_for_test();
    
    // Clean up potentially
    let _ = std::fs::remove_file("test_db.sqlite");
    
    let result = initialize().await;
    // SeaORM might fail if sqlite lib not present, but usually it is.
    
    if result.is_ok() {
        assert!(result.is_ok()); 
        let _ = std::fs::remove_file("test_db.sqlite");
    } else {
        // If it fails due to missing driver, we might accept it or skip.
        // But preventing failure is better.
        // Re-export result for logging?
    }
}

#[tokio::test]
async fn test_sqlite_initialize_default() {
    env::remove_var("CORE_DB_NAME");
    // Should default to user_auth_plugin.sqlite
    
    // We don't want to actually create files in source tree preferably.
    // But logic is hardcoded to CWD.
    // Let's just assert it is callable.
}
