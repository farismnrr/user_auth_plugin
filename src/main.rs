//! IoTNet - User Authentication Plugin
//!
//! This is the main entry point for the User Auth Plugin authentication service built with Actix-web.
//! The application provides user management and authentication capabilities with JWT-based
//! authentication, rate limiting, and comprehensive API endpoints.

mod server;
mod infrastructures;
mod middlewares;
mod dtos;
mod entities;
mod repositories;
mod usecases;
mod controllers;
mod routes;
mod errors;
mod utils;
mod validators;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run_server().await
}