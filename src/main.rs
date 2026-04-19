mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod services;

use crate::handlers::auth::register;
use crate::handlers::auth::login;
use axum::{
    extract::Extension,
    http::{header, HeaderValue, Method},
    routing::get,
    Router,
};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    // 1. Сначала инициализируем трейсинг (чтобы видеть логи)
    tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "kernel_api=info".into()))
    .init();
    // 2. Загружаем .env
    dotenv().ok();

    // 3. Подключаемся к БД
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be in .env");
    let pool = db::pool::create_pool(&database_url)
    .await
    .expect("Не удалось подключиться к базе данных");

    tracing::info!("✅ База данных успешно подключена!");

    // 4. Настраиваем CORS
    let cors = CorsLayer::new()
    .allow_origin("http://127.0.0.1:5500".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
    .allow_credentials(false);

    // 5. Создаём роутер ОДИН РАЗ со всеми слоями
    let app = Router::new()
    .route("/health", get(|| async { "OK" }))
    .route("/auth/register", axum::routing::post(register))
    .route("/auth/login", axum::routing::post(login))
        .layer(cors)  // ✅ CORS применяется здесь
        .with_state(pool);

    // 6. Запускаем сервер
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Сервер запущен на http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}