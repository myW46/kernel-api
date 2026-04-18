use axum::{extract::State, http::StatusCode, Json};

use crate::auth::password;
use crate::error::AppError;
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct AuthResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    // 1. Хешируем пароль через наш модуль auth/password
    let hashed_password = password::hash(&payload.password)?;

    // 2. Создаем пользователя в БД через метод модели
    let user = User::create(&payload.username, &payload.email, &hashed_password, &pool)
        .await
        .map_err(|e| {
            // Если ошибка БД (например, email уже занят), возвращаем ошибку
            // Здесь можно добавить логику проверки уникальности
            AppError::DbError(e)
        })?;

    // 3. Формируем ответ без пароля
    let response = AuthResponse {
        id: user.id,
        username: user.username,
        email: user.email,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
