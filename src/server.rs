//! Server Configuration and Initialization
//!
//! This module handles server setup, middleware configuration, database connections,
//! health monitoring, and graceful shutdown.

use actix_web::{App, HttpServer, HttpResponse, middleware, web, Responder};
use chrono::Local;
use std::io::Write;
use log::info;
use std::sync::OnceLock;

use crate::repositories::user_repository::UserRepository;
use crate::repositories::user_details_repository::UserDetailsRepository;
use crate::repositories::user_session_repository::UserSessionRepository;
use crate::repositories::user_activity_log_repository::UserActivityLogRepository;
use crate::repositories::tenant_repository::TenantRepository;
use crate::repositories::user_tenant_repository::UserTenantRepository;
use crate::usecases::user_usecase::UserUseCase;
use crate::usecases::auth_usecase::AuthUseCase;
use crate::usecases::user_details_usecase::UserDetailsUseCase;
use crate::usecases::tenant_usecase::TenantUseCase;
use crate::routes::user_routes;
use crate::routes::auth_routes;
use crate::routes::tenant_routes;


use crate::middlewares::powered_by_middleware::PoweredByMiddleware;
use crate::middlewares::request_logger_middleware::RequestLoggerMiddleware;
use crate::infrastructures::postgres_connection;
use std::sync::Arc;
use tokio::sync::watch;

/// Health check endpoint handler.
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("OK")
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

    let current_log_marker = "logs/.current_log";
    let log_file_path = if let Ok(existing_path) = std::fs::read_to_string(current_log_marker) {
        if std::path::Path::new(&existing_path).exists() {
            existing_path.trim().to_string()
        } else {
            let timestamp = Local::now().format("%Y%m%d_%H%M%S");
            let new_path = format!("logs/{}.log", timestamp);
            std::fs::write(current_log_marker, &new_path)?;
            new_path
        }
    } else {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let new_path = format!("logs/{}.log", timestamp);
        std::fs::write(current_log_marker, &new_path)?;
        new_path
    };

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
        file: std::sync::Mutex::new(log_file)
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
                    log::Level::Warn  => Colour::Yellow.paint("WARN"),
                    log::Level::Info  => Colour::Green.paint("INFO"),
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
    // Initializes the asynchronous Postgres connection pool.
    // This pool allows multiple concurrent database operations throughout the application.

    let db = postgres_connection::initialize().await
        .map_err(|e| std::io::Error::other(format!("Postgres initialization failed: {}", e)))?;

    // ================================================================================================
    // 📂 REPOSITORY SECTION
    // ================================================================================================
    //
    // Instantiates the Data Access Layer (Repositories).
    // Each repository provides an abstraction over database operations for specific entities.
    // They are wrapped in `Arc` (Atomic Reference Counting) to allow thread-safe sharing
    // across multiple potential threads in the server.

    use crate::infrastructures::cache::RocksDbCache;
    let cache = Arc::new(RocksDbCache::new("./rocksdb_cache")
        .map_err(|e| std::io::Error::other(format!("Failed to initialize RocksDB cache: {}", e)))?);

    let user_repository = Arc::new(UserRepository::new(db.clone(), cache.clone()));
    let user_details_repository = Arc::new(UserDetailsRepository::new(db.clone(), cache.clone()));
    let user_session_repository = Arc::new(UserSessionRepository::new(db.clone()));
    let user_activity_log_repository = Arc::new(UserActivityLogRepository::new(db.clone()));
    let tenant_repository = Arc::new(TenantRepository::new(db.clone(), cache.clone()));
    let user_tenant_repository = Arc::new(UserTenantRepository::new(db.clone(), cache.clone()));

    // ================================================================================================
    // 🧠 USECASE SECTION
    // ================================================================================================
    //
    // Instantiates the Business Logic Layer (UseCases).
    // UseCases contain the core application rules and orchestrate data flow using Repositories.
    // Like repositories, they are wrapped in `Arc` for thread-safe sharing.

    let user_usecase = Arc::new(UserUseCase::new(
        user_repository.clone(),
        user_details_repository.clone(),
        user_tenant_repository.clone()
    ));
    let auth_usecase = Arc::new(AuthUseCase::new(
        user_repository.clone(),
        user_details_repository.clone(),
        user_tenant_repository.clone(),
        user_session_repository.clone(),
        user_activity_log_repository.clone(),
    ));
    let user_details_usecase = Arc::new(UserDetailsUseCase::new(user_details_repository.clone()));
    let tenant_usecase = Arc::new(TenantUseCase::new(tenant_repository.clone()));

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

    let secret_key = Arc::new(std::env::var("SECRET_KEY").unwrap_or_else(|_| {
        info!("SECRET_KEY not set; using empty string for development");
        String::new()
    }));

    info!("🚀 Actix server running on http://0.0.0.0:5500");

    let db_for_factory = db.clone();

    let secret_for_factory = secret_key.clone();
    let user_usecase_for_factory = user_usecase.clone();
    let auth_usecase_for_factory = auth_usecase.clone();
    let user_details_usecase_for_factory = user_details_usecase.clone();
    let tenant_usecase_for_factory = tenant_usecase.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(secret_for_factory.clone()))
            .app_data(web::Data::from(db_for_factory.clone()))
            .app_data(web::JsonConfig::default()
                .error_handler(crate::errors::json_error_handler::json_error_handler)
            )
            .app_data(web::PathConfig::default()
                .error_handler(crate::errors::path_error_handler::path_error_handler)
            )

            .app_data(web::Data::new(user_usecase_for_factory.clone()))
            .app_data(web::Data::new(auth_usecase_for_factory.clone()))
            .app_data(web::Data::new(user_details_usecase_for_factory.clone()))
            .app_data(web::Data::new(tenant_usecase_for_factory.clone()))
            .wrap(PoweredByMiddleware)
            .wrap(RequestLoggerMiddleware)
            .wrap(middleware::Compress::default())

            .route("/", web::get().to(healthcheck))

            .service(
                actix_files::Files::new("/assets", "assets")
                    .show_files_listing()
                    .use_etag(true)
                    .use_last_modified(true)
            )
            
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )

            .service(
                web::scope("/api")
                    .configure(tenant_routes::configure_tenant_routes)
                    .configure(user_routes::configure_user_routes)
            )
            
            // .configure(user_routes::configure_user_routes) // MOVED OUT
            // .configure(auth_routes::configure_jwt_routes) // REPLACED
            .configure(auth_routes::configure_routes)
    })
    .bind(("0.0.0.0", 5500))?;

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
    
    postgres_connection::monitor_health(db.clone(), shutdown_tx.clone());
    
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
    tokio::spawn(async move {
        postgres_connection::shutdown(db_for_shutdown, db_rx).await;
    });

    let server_result = srv.await;
    let _ = shutdown_task.await;

    info!("Shutting down server...");

    server_result
}