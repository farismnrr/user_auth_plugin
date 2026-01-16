//! Unit tests for centralized configuration module
//!
//! These tests validate that required environment variables are set correctly.
//! They do NOT set dummy values - they check actual env vars from .env file.

#[cfg(test)]
mod tests {
    use super::super::config::Config;

    #[test]
    fn test_config_has_required_security_vars() {
        let config = Config::init_for_test();

        assert!(!config.api_key.is_empty(), "API_KEY must not be empty");
        assert!(
            !config.tenant_secret_key.is_empty(),
            "TENANT_SECRET_KEY must not be empty"
        );
        assert!(
            !config.secret_key.is_empty(),
            "SECRET_KEY must not be empty"
        );
    }

    #[test]
    fn test_config_has_required_server_vars() {
        let config = Config::init_for_test();

        assert!(
            !config.server_host.is_empty(),
            "SERVER_HOST must not be empty"
        );
        assert!(config.server_port > 0, "SERVER_PORT must be valid");
        assert!(!config.endpoint.is_empty(), "ENDPOINT must not be empty");
    }

    #[test]
    fn test_config_has_required_database_vars() {
        let config = Config::init_for_test();

        assert!(
            config.db_type == "sqlite" || config.db_type == "postgres",
            "CORE_DB_TYPE must be 'sqlite' or 'postgres'"
        );
        assert!(!config.db_host.is_empty(), "CORE_DB_HOST must not be empty");
        assert!(!config.db_port.is_empty(), "CORE_DB_PORT must not be empty");
        assert!(!config.db_user.is_empty(), "CORE_DB_USER must not be empty");
        assert!(!config.db_pass.is_empty(), "CORE_DB_PASS must not be empty");
        assert!(!config.db_name.is_empty(), "CORE_DB_NAME must not be empty");
    }

    #[test]
    fn test_config_has_cors_origins() {
        let config = Config::init_for_test();

        assert!(
            !config.allowed_origins.is_empty(),
            "VITE_ALLOWED_ORIGINS must contain at least one origin"
        );

        for origin in &config.allowed_origins {
            assert!(!origin.is_empty(), "CORS origin must not be empty");
        }
    }

    #[test]
    fn test_config_has_valid_token_expiry() {
        let config = Config::init_for_test();

        assert!(
            config.access_token_expiry > 0,
            "ACCESS_TOKEN_EXPIRY must be positive"
        );
        assert!(
            config.refresh_token_expiry > 0,
            "REFRESH_TOKEN_EXPIRY must be positive"
        );
        assert!(config.cache_ttl > 0, "CACHE_TTL must be positive");
    }

    #[test]
    fn test_config_server_port_is_valid() {
        let config = Config::init_for_test();

        assert!(config.server_port > 0, "SERVER_PORT must be valid");
    }

    #[test]
    fn test_config_singleton_pattern() {
        let config1 = Config::init_for_test();
        let config2 = Config::get();

        assert_eq!(config1.api_key, config2.api_key);
        assert_eq!(config1.server_port, config2.server_port);
    }
}
