#![allow(unused)]
use anyhow::{Context, Result};
use axum::{http::uri::Port, middleware, Router};
use dotenvy::dotenv;
use std::{env, error::Error};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

mod config;
mod features;
mod middlewares;
mod routes;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::dotenv()?;

    // Set up tracing with filtering from environment variables (RUST_LOG)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()) // Allow dynamic log levels
        .with_target(true) // Show module paths
        .with_thread_names(true) // Show thread names
        .init();

    // debug!("🔍 This is a debug message");
    // warn!("⚠️ A warning occurred");
    // error!("❌ Something went wrong!");

    // init DB
    config::database::connect_db().await?;

    // Axum init route and middlewares
    let app = Router::new()
        .merge(routes::app_routes())
        .nest("/api", routes::api_routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::map_response(
            middlewares::response::main_response_mapper,
        ));

    // Axum start server
    let url = env::var("APP_URL").unwrap_or("0.0.0.0".to_string());
    let port = env::var("APP_PORT").expect("❌ APP_PORT not found on .env");
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", url, port))
        .await
        .unwrap();

    info!("🚀 Server starting on {} port {}", url, port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
