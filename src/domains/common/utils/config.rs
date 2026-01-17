//! Centralized Configuration Management
//!
//! This module provides a single source of truth for all environment variables
//! with strict validation. All env vars must be explicitly set - no fallbacks.

use std::env;
use std::sync::OnceLock;

/// Global configuration instance
static CONFIG: OnceLock<Config> = OnceLock::new();

/// Application configuration
#[derive(Clone, Debug)]
pub struct Config {
    // Security
    pub api_key: String,
    pub tenant_secret_key: String,
    pub secret_key: String,
    pub jwt_secret: String,

    // Server
    pub server_host: String,
    pub server_port: u16,
    pub endpoint: String,

    // Database
    pub db_type: String,
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,

    // CORS
    pub allowed_origins: Vec<String>,

    // Optional (with reasonable defaults)
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
    pub cache_ttl: u64,
}

impl Config {
    /// Initialize configuration from environment variables
    ///
    /// # Panics
    /// Panics if any required environment variable is missing or invalid
    pub fn init() -> &'static Self {
        CONFIG.get_or_init(|| {
            // Load .env file if it exists
            dotenvy::dotenv().ok();

            // Security - REQUIRED
            let api_key = env::var("API_KEY").expect("API_KEY must be set in environment");
            let tenant_secret_key = env::var("TENANT_SECRET_KEY")
                .expect("TENANT_SECRET_KEY must be set in environment");
            let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set in environment");
            let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment");

            // Server - REQUIRED
            let server_host =
                env::var("SERVER_HOST").expect("SERVER_HOST must be set in environment");
            let server_port = env::var("SERVER_PORT")
                .expect("SERVER_PORT must be set in environment")
                .parse::<u16>()
                .expect("SERVER_PORT must be a valid port number");
            let endpoint = env::var("ENDPOINT").expect("ENDPOINT must be set in environment");

            // Database - REQUIRED
            let db_type = env::var("CORE_DB_TYPE")
                .expect("CORE_DB_TYPE must be set in environment (sqlite or postgres)");

            if db_type.trim().is_empty() {
                panic!("CORE_DB_TYPE cannot be empty");
            }
            if db_type != "sqlite" && db_type != "postgres" {
                panic!(
                    "CORE_DB_TYPE must be 'sqlite' or 'postgres', got: {}",
                    db_type
                );
            }

            let db_host =
                env::var("CORE_DB_HOST").expect("CORE_DB_HOST must be set in environment");
            let db_port =
                env::var("CORE_DB_PORT").expect("CORE_DB_PORT must be set in environment");
            let db_user =
                env::var("CORE_DB_USER").expect("CORE_DB_USER must be set in environment");
            let db_pass =
                env::var("CORE_DB_PASS").expect("CORE_DB_PASS must be set in environment");
            let db_name =
                env::var("CORE_DB_NAME").expect("CORE_DB_NAME must be set in environment");

            // CORS - REQUIRED
            let allowed_origins_raw = env::var("VITE_ALLOWED_ORIGINS")
                .expect("VITE_ALLOWED_ORIGINS must be set in environment");
            let allowed_origins: Vec<String> = allowed_origins_raw
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if allowed_origins.is_empty() {
                panic!("VITE_ALLOWED_ORIGINS must contain at least one origin");
            }

            // Optional with defaults
            let access_token_expiry = env::var("ACCESS_TOKEN_EXPIRY")
                .unwrap_or_else(|_| "900".to_string())
                .parse::<i64>()
                .unwrap_or(900);

            let refresh_token_expiry = env::var("REFRESH_TOKEN_EXPIRY")
                .unwrap_or_else(|_| "604800".to_string())
                .parse::<i64>()
                .unwrap_or(604800);

            let cache_ttl = env::var("CACHE_TTL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse::<u64>()
                .unwrap_or(3600);

            Config {
                api_key,
                tenant_secret_key,
                secret_key,
                jwt_secret,
                server_host,
                server_port,
                endpoint,
                db_type,
                db_host,
                db_port,
                db_user,
                db_pass,
                db_name,
                allowed_origins,
                access_token_expiry,
                refresh_token_expiry,
                cache_ttl,
            }
        })
    }

    /// Get the global configuration instance
    pub fn get() -> &'static Self {
        CONFIG
            .get()
            .expect("Config not initialized. Call Config::init() first")
    }

    /// Initialize config for tests - safe to call multiple times
    ///
    /// This is a helper that will initialize config if not already done,
    /// or return the existing instance if already initialized.
    pub fn init_for_test() -> &'static Self {
        if CONFIG.get().is_some() {
            Self::get()
        } else {
            Self::init()
        }
    }
}
