mod config;
mod db;
mod error;

mod handlers;
mod models;
mod psp;
mod repositories;

use axum::{Json, Router, routing::get, routing::post};
use serde_json::json;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = config::Config::from_env()?;
    let pool = db::create_pool(&config.database_url).await?;

    sqlx::query("SELECT 1").execute(&pool).await?;

    //    let app = Router::new().route("/health", get(health)).with_state(pool);
    let app = Router::new()
        .route("/health", get(health))
        .route("/customers", post(handlers::customer::create_customer))
        .route("/customers/{id}", get(handlers::customer::get_customer))
        .route("/invoices", post(handlers::invoice::create_invoice))
        .route("/invoices/{id}", get(handlers::invoice::get_invoice))
        .route(
            "/invoices/{id}/payments",
            post(handlers::payments::start_payment),
        )
        .with_state(pool);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address).await?;

    tracing::info!("Server listening on {address}");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
