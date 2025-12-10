//! Server Configuration and Initialization
//!
//! This module handles server setup, middleware configuration, database connections,
//! health monitoring, and graceful shutdown.

use actix_web::{App, HttpServer, HttpResponse, middleware, web, Responder};
use chrono::Local;
use std::io::Write;
use log::info;

use crate::middlewares::api_key::ApiKeyMiddleware;
use crate::middlewares::powered_by::PoweredByMiddleware;
use crate::middlewares::logger_request::RequestLoggerMiddleware;
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

    info!("🟢 Starting server initialization");

    let env = env_logger::Env::new().filter_or("LOG_LEVEL", "info");
    use ansi_term::Colour;
    env_logger::Builder::from_env(env)
        .format(move |buf, record| {
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
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
        .init();
    info!("🟢 Logging initialized successfully");

    let db = postgres_connection::initialize().await
        .map_err(|e| std::io::Error::other(format!("Postgres initialization failed: {}", e)))?;

    use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};
    use crate::repositories::user_details_repository::{UserDetailsRepository, UserDetailsRepositoryTrait};
    use crate::usecases::user_usecase::UserUseCase;
    use crate::usecases::auth_usecase::AuthUseCase;
    use crate::usecases::user_details_usecase::UserDetailsUseCase;
    use crate::routes::user_routes;
    use crate::routes::auth_routes;

    let user_repository = UserRepository::new(db.clone());
    let user_repository: Arc<dyn UserRepositoryTrait> = Arc::new(user_repository);
    
    let user_details_repository = UserDetailsRepository::new(db.clone());
    let user_details_repository: Arc<dyn UserDetailsRepositoryTrait> = Arc::new(user_details_repository);
    
    let user_usecase = Arc::new(UserUseCase::new(user_repository.clone(), user_details_repository.clone()));
    let auth_usecase = Arc::new(AuthUseCase::new(user_repository.clone(), user_details_repository.clone()));
    let user_details_usecase = Arc::new(UserDetailsUseCase::new(user_details_repository.clone()));

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

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(secret_for_factory.clone()))
            .app_data(web::Data::from(db_for_factory.clone()))

            .app_data(web::Data::new(user_usecase_for_factory.clone()))
            .app_data(web::Data::new(auth_usecase_for_factory.clone()))
            .app_data(web::Data::new(user_details_usecase_for_factory.clone()))
            .wrap(PoweredByMiddleware)
            .wrap(RequestLoggerMiddleware)
            .wrap(middleware::Compress::default())

            .route("/", web::get().to(healthcheck))

            // Static files for profile pictures
            .service(
                actix_files::Files::new("/assets", "assets")
                    .show_files_listing()
                    .use_etag(true)
                    .use_last_modified(true)
            )
            
            // CORS middleware for cross-origin access
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )

            // API key protected routes (login, register, refresh)
            .service(
                web::scope("/api")
                    .wrap(ApiKeyMiddleware)
                    .route("/ping", web::get().to(|| async { HttpResponse::Ok().body("pong") }))
                    .configure(auth_routes::configure_api_key_routes)
            )
            
            // JWT protected routes (logout, verify, users)
            .configure(user_routes::configure_user_routes)
            .configure(auth_routes::configure_jwt_routes)
    })
    .bind(("0.0.0.0", 5500))?;

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