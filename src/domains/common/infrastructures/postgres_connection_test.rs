use super::postgres_connection::*;

#[tokio::test]
async fn test_postgres_initialize_env_vars() {
    use crate::domains::common::utils::config::Config;
    Config::init_for_test();

    // Connection will use config from .env.
    // It might succeed if a DB is running, or fail if not.
    // We just want to ensure it doesn't panic due to missing environment variables.
    let _ = initialize().await;
}
