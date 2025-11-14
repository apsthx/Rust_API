mod configs;
mod controllers;
mod middlewares;
mod models;
mod routes;
mod structs;
mod libs;

use anyhow::Result;
use dotenvy::dotenv;
use std::env;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Main entry point for Rust Clinic API
/// Equivalent to Go's main.go
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "clinic_api=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Clinic API server...");

    // Initialize database connections
    let app_state = configs::init_databases().await?;
    tracing::info!("Database connections established");

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin([
            "https://www.app-apsx.com".parse()?,
            "https://app-apsx.com".parse()?,
            "https://clinic.app-apsx.com".parse()?,
            "https://admin.app-apsx.com".parse()?,
            "http://localhost:3000".parse()?,
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router
    let app = routes::create_router(app_state)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Get port from environment or use default
    let port = env::var("API_PORT")
        .unwrap_or_else(|_| ":8002".to_string())
        .trim_start_matches(':')
        .to_string();

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on {}", addr);
    tracing::info!("Environment: {}", env::var("ENV").unwrap_or_else(|_| "DEV".to_string()));

    // Start server
    axum::serve(listener, app).await?;

    Ok(())
}
