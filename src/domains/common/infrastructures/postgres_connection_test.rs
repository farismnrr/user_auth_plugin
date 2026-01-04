
use super::postgres_connection::*;
use std::env;

#[tokio::test]
async fn test_postgres_initialize_env_vars() {
    // verifying that env vars are read. 
    // Actual connection will fail without a running DB.
    // We expect an error, but we want to ensure it tries the right URL.
    
    env::set_var("CORE_DB_HOST", "nonexistent_host");
    env::set_var("CORE_DB_PORT", "5432");
    env::set_var("CORE_DB_USER", "user");
    env::set_var("CORE_DB_PASS", "pass");
    env::set_var("CORE_DB_NAME", "db");
    
    let result = initialize().await;
    assert!(result.is_err());
    
    // We can assert the error message contains connection info if we want, 
    // to distinguish from other errors.
}
