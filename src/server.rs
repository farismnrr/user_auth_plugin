//! Server Configuration and Initialization
//!
//! This module handles server setup, middleware configuration, database connections,
//! health monitoring, and graceful shutdown.

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use log::info;
use std::io::Write;
use std::sync::OnceLock;

use crate::domains::auth::auth_module::AuthModule;
use crate::domains::tenant::tenant_module::TenantModule;
use crate::domains::user::user_module::UserModule;

// Repositories
use crate::domains::tenant::repositories::tenant_repository::TenantRepository;
use crate::domains::tenant::repositories::user_tenant_repository::UserTenantRepository;
use crate::domains::user::repositories::user_activity_log_repository::UserActivityLogRepository;
use crate::domains::user::repositories::user_details_repository::UserDetailsRepository;
use crate::domains::user::repositories::user_repository::UserRepository;
use crate::domains::user::repositories::user_session_repository::UserSessionRepository;

// UseCases
use crate::domains::auth::usecases::auth_usecase::AuthUseCase;
use crate::domains::tenant::usecases::tenant_usecase::TenantUseCase;
use crate::domains::user::usecases::user_details_usecase::UserDetailsUseCase;
use crate::domains::user::usecases::user_usecase::UserUseCase;

use crate::domains::common::infrastructures::postgres_connection;
use crate::domains::common::middlewares::powered_by_middleware::PoweredByMiddleware;
use crate::domains::common::middlewares::request_logger_middleware::RequestLoggerMiddleware;
use std::sync::Arc;
use tokio::sync::watch;

/// Health check endpoint handler.
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("OK")
}

/// Serves dynamic runtime configuration for the frontend.
async fn serve_runtime_config(allowed_origins: web::Data<Vec<String>>) -> impl Responder {
    use crate::domains::common::utils::config::Config;
    let config = Config::get();
    info!(
        "Serving runtime config with allowed origins: {:?}",
        allowed_origins.get_ref()
    );

    // Convert Vec<String> to a comma-separated string for the JS config
    let origins_str = allowed_origins.join(",");

    let config_content = format!(
        "window.config = {{ API_KEY: \"{}\", ENDPOINT: \"{}\", ALLOWED_ORIGINS: \"{}\" }};",
        config.api_key, config.endpoint, origins_str
    );

    HttpResponse::Ok()
        .content_type("application/javascript; charset=utf-8")
        .body(config_content)
}

/// Initializes and runs the Actix-web server.
///
/// This function performs the following initialization steps:
/// 1. Loads environment variables
/// 2. Configures logging with colored output
/// 3. Establishes database connections
/// 4. Sets up dependency injection
/// 5. Configures HTTP server with middlewares
/// 6. Starts health monitoring
/// 7. Handles graceful shutdown

pub async fn run_server() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // Initialize centralized configuration
    use crate::domains::common::utils::config::Config;
    let config = Config::init();

    // ================================================================================================
    // 📝 LOG SECTION
    // ================================================================================================
    //
    // This section initializes the logging infrastructure.
    // It performs the following:
    // 1. Creates a "logs" directory if it doesn't exist.
    // 2. Manages log rotation: checks for a previous run's log file or creates a new one
    //    based on the current timestamp.
    // 3. Sets up a `DualWriter` to output logs to both the console (with colors) and the file.
    // 4. Initializes the global logger (env_logger) with specific formatting.

    std::fs::create_dir_all("logs")
        .map_err(|e| std::io::Error::other(format!("Failed to create logs directory: {}", e)))?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let log_file_path = format!("logs/{}.log", timestamp);
    let current_log_marker = "logs/.current_log";

    let _ = std::fs::write(current_log_marker, &log_file_path);
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .map_err(|e| std::io::Error::other(format!("Failed to open log file: {}", e)))?;

    struct DualWriter {
        file: std::sync::Mutex<std::fs::File>,
    }

    impl Write for DualWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            std::io::stdout().write_all(buf)?;
            self.file.lock().unwrap().write_all(buf)?;
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            std::io::stdout().flush()?;
            self.file.lock().unwrap().flush()?;
            Ok(())
        }
    }

    let dual_writer = DualWriter {
        file: std::sync::Mutex::new(log_file),
    };

    static LOGGER_INIT: OnceLock<()> = OnceLock::new();
    LOGGER_INIT.get_or_init(|| {
        let env = env_logger::Env::new().filter_or("LOG_LEVEL", "info");
        use ansi_term::Colour;

        env_logger::Builder::from_env(env)
            .format(move |buf, record| {
                let ts = Local::now().format("%Y%m%d_%H%M%S");
                let level_str = match record.level() {
                    log::Level::Error => Colour::Red.paint("ERROR"),
                    log::Level::Warn => Colour::Yellow.paint("WARN"),
                    log::Level::Info => Colour::Green.paint("INFO"),
                    log::Level::Debug => Colour::Blue.paint("DEBUG"),
                    log::Level::Trace => Colour::Purple.paint("TRACE"),
                };

                writeln!(buf, "[{} {}] {}", ts, level_str, record.args())
            })
            .format_target(false)
            .target(env_logger::Target::Pipe(Box::new(dual_writer)))
            .init();
    });

    info!("Server Version: VERIFIED_FINAL");
    info!("🟢 Starting server initialization");
    info!("📝 Logging to file: {}", log_file_path);

    // ================================================================================================
    // 🗄️ DATABASE SECTION
    // ================================================================================================
    //
    // Initializes the asynchronous Database connection pool.
    // This pool allows multiple concurrent database operations throughout the application.
    // It selects between Postgres and SQLite based on DB_TYPE environment variable.
    // Default is SQLite if unspecified or empty.

    let db_type = &config.db_type;
    info!("Database Type: {}", db_type);

    let db = match db_type.as_str() {
        "postgres" => postgres_connection::initialize()
            .await
            .map_err(|e| std::io::Error::other(format!("Postgres initialization failed: {}", e)))?,
        "sqlite" => crate::domains::common::infrastructures::sqlite_connection::initialize()
            .await
            .map_err(|e| std::io::Error::other(format!("SQLite initialization failed: {}", e)))?,
        _ => {
            return Err(std::io::Error::other(format!(
                "Unsupported CORE_DB_TYPE: {}",
                db_type
            )))
        }
    };

    // ================================================================================================
    // 🚀 SERVER SECTION
    // ================================================================================================
    //
    // Configures and starts the Actix Web server.
    // 1. Injects shared state (Config, DB, UseCases) into `web::Data`.
    // 2. Registers global middlewares (Logger, Compression, CORS, Headers).
    // 3. Defines routing:
    //    - Health checks ("/")
    //    - Static files ("/assets")
    //    - API Routes ("/api") protected by API Keys and JWT.

    let secret_key = Arc::new(config.secret_key.clone());

    // Create assets directory if it doesn't exist
    std::fs::create_dir_all("assets").ok();

    // CORS Configuration - from centralized config
    let allowed_origins: Arc<Vec<String>> = Arc::new(config.allowed_origins.clone());

    let server_host = &config.server_host;
    let server_port = config.server_port;

    info!(
        "🚀 Actix server running on http://{}:{}",
        server_host, server_port
    );

    // RocksDB Cache Initialization
    use crate::domains::common::infrastructures::rocksdb_connection::RocksDbCache;
    let cache = Arc::new(
        RocksDbCache::new_with_recovery("./rocksdb_cache").map_err(|e| {
            std::io::Error::other(format!("Failed to initialize RocksDB cache: {}", e))
        })?,
    );

    // ================================================================================================
    // 🧠 REPOSITORY SECTION
    // ================================================================================================
    let db_arc = db.clone();
    let user_repo = Arc::new(UserRepository::new(db_arc.clone(), cache.clone()));
    let user_details_repo = Arc::new(UserDetailsRepository::new(db_arc.clone(), cache.clone()));
    let user_session_repo = Arc::new(UserSessionRepository::new(db_arc.clone()));
    let user_activity_log_repo = Arc::new(UserActivityLogRepository::new(db_arc.clone()));
    let user_tenant_repo = Arc::new(UserTenantRepository::new(db_arc.clone(), cache.clone()));
    let tenant_repo = Arc::new(TenantRepository::new(db_arc.clone(), cache.clone()));

    use crate::domains::auth::repositories::invitation_code_repository::InvitationCodeRepository;
    let invitation_code_repo = Arc::new(InvitationCodeRepository::new(cache.clone()));

    // ================================================================================================
    // 🧠 USECASE SECTION
    // ================================================================================================
    let user_usecase = Arc::new(UserUseCase::new(
        user_repo.clone(),
        user_details_repo.clone(),
        user_tenant_repo.clone(),
    ));
    let auth_usecase = Arc::new(AuthUseCase::new(
        user_repo.clone(),
        user_details_repo.clone(),
        user_tenant_repo.clone(),
        user_session_repo.clone(),
        user_activity_log_repo.clone(),
        invitation_code_repo.clone(),
    ));
    let user_details_usecase = Arc::new(UserDetailsUseCase::new(user_details_repo.clone()));
    let tenant_usecase = Arc::new(TenantUseCase::new(tenant_repo.clone()));

    // Prepare variables for the factory closure
    let db_for_factory = db.clone();
    let secret_for_factory = secret_key.clone();
    let allowed_origins_for_factory = allowed_origins.clone();

    let user_usecase_for_factory = user_usecase.clone();
    let auth_usecase_for_factory = auth_usecase.clone();
    let user_details_usecase_for_factory = user_details_usecase.clone();
    let tenant_usecase_for_factory = tenant_usecase.clone();

    let server = HttpServer::new(move || {
        let mut cors = actix_cors::Cors::default()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        for origin in allowed_origins_for_factory.iter() {
            cors = cors.allowed_origin(origin);
        }

        let mut app = App::new()
            .app_data(web::Data::from(secret_for_factory.clone()))
            .app_data(web::Data::from(db_for_factory.clone()))
            .app_data(web::JsonConfig::default().error_handler(
                crate::domains::common::errors::json_error_handler::json_error_handler,
            ))
            .app_data(web::PathConfig::default().error_handler(
                crate::domains::common::errors::path_error_handler::path_error_handler,
            ))
            // Register App Data
            .app_data(web::Data::new(user_usecase_for_factory.clone()))
            .app_data(web::Data::new(auth_usecase_for_factory.clone()))
            .app_data(web::Data::new(user_details_usecase_for_factory.clone()))
            .app_data(web::Data::new(tenant_usecase_for_factory.clone()))
            .app_data(web::Data::from(allowed_origins_for_factory.clone()))
            // Register Modules
            .configure(AuthModule::configure_module)
            .service(
                web::scope("/api")
                    .configure(UserModule::configure_module)
                    .configure(TenantModule::configure_module),
            )
            .wrap(PoweredByMiddleware)
            .wrap(RequestLoggerMiddleware)
            .wrap(middleware::Compress::default())
            .route("/health", web::get().to(healthcheck))
            .route("/runtime-env.js", web::get().to(serve_runtime_config))
            .wrap(cors);

        // Only serve static files if web/dist exists (production mode)
        if std::path::Path::new("./web/dist").exists() {
            app = app.service(
                actix_files::Files::new("/", "./web/dist")
                    .index_file("index.html")
                    .default_handler(web::to(|| {
                        actix_files::NamedFile::open_async("./web/dist/index.html")
                    })),
            );
        }

        app
    })
    .bind((server_host.as_str(), server_port))?;

    // ================================================================================================
    // 🛑 SHUTDOWN SECTION
    // ================================================================================================
    //
    // Handles Graceful Shutdown.
    // 1. Listens for OS signals (Ctrl+C) and internal health check failures.
    // 2. Stops the server from accepting new requests upon signal reception.
    // 3. Waits for active requests to complete (or times out).
    // 4. Signals the database connection pool to close gracefully.

    let srv = server.run();

    let srv_handle = srv.handle();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // Launch health monitor based on DB type
    match db_type.as_str() {
        "postgres" => postgres_connection::monitor_health(db.clone(), shutdown_tx.clone()),
        "sqlite" => crate::domains::common::infrastructures::sqlite_connection::monitor_health(
            db.clone(),
            shutdown_tx.clone(),
        ),
        _ => {} // Should not happen given earlier check
    }

    let db_for_shutdown = db.clone();

    let shutdown_tx_clone = shutdown_tx.clone();
    let mut shutdown_rx_clone = shutdown_rx.clone();
    let shutdown_task = tokio::spawn(async move {
        tokio::select! {
            result = tokio::signal::ctrl_c() => {
                if let Err(e) = result {
                    log::error!("Failed to listen for shutdown signal: {}", e);
                    return;
                }
                info!("Shutdown signal received, stopping server...");
            }
            _ = shutdown_rx_clone.changed() => {
                if *shutdown_rx_clone.borrow() {
                    info!("Health check triggered shutdown, stopping server...");
                }
            }
        }

        srv_handle.stop(false).await;

        let _ = shutdown_tx_clone.send(true);

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    });

    let db_rx = shutdown_rx.clone();
    let db_type_for_shutdown = db_type.clone();
    tokio::spawn(async move {
        match db_type_for_shutdown.as_str() {
            "postgres" => postgres_connection::shutdown(db_for_shutdown, db_rx).await,
            "sqlite" => {
                crate::domains::common::infrastructures::sqlite_connection::shutdown(
                    db_for_shutdown,
                    db_rx,
                )
                .await
            }
            _ => {}
        }
    });

    let cache_rx = shutdown_rx.clone();
    let cache_for_shutdown = cache.clone();
    tokio::spawn(async move {
        cache_for_shutdown.shutdown(cache_rx).await;
    });

    let cache_tx = shutdown_tx.clone();
    let cache_for_health = cache.clone();
    RocksDbCache::monitor_health(cache_for_health, cache_tx).await;

    let server_result = srv.await;
    let _ = shutdown_task.await;

    info!("Shutting down server...");

    server_result
}
