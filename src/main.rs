mod config;
mod error;
mod db;
mod models;
mod auth;
mod handlers;
mod services;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use crate::handlers::auth::register;
#[tokio::main]
async fn main() {
    // Инициализация логов (полезно для отладки)
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "kernel_api=info".into()))
        .init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be in .env");

    let pool = db::pool::create_pool(&database_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    tracing::info!("База данных успешно подключена!");
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/auth/register", axum::routing::post(register))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Сервер запущен на http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}