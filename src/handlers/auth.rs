use axum::{extract::State, http::StatusCode, Json};

use crate::auth::password;
use crate::error::AppError;
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, error::DatabaseError};
use tracing::info;

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

#[derive(Deserialize)]
pub struct LoginRequest{
    pub  email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse{
    pub id: uuid::Uuid,
    pub token: String,
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
    info!("New user registered: {} (ID: {})", user.username, user.id); 
    // 3. Формируем ответ без пароля
    let response = AuthResponse {
        id: user.id,
        username: user.username,
        email: user.email,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> { 

    let user: User= User::find_by_email(&payload.email, &pool)
        .await
        .map_err(|e| AppError::DbError(e))?
        .ok_or(AppError::Unauthorized)?;

    let is_valide = password::verify(&payload.password, &user.password_hash);
    if !is_valide{
        return Err(AppError::Unauthorized);
    }
    info!("User logged in: {} (ID: {})", user.username, user.id);
    let token = crate::auth::jwt::sign(user.id)?;
    let response = LoginResponse{
        id:user.id,
        token: token,
    };
    
    Ok((StatusCode::OK, Json(response)))

}
