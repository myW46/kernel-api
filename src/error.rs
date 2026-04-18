use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DbError(#[from] sqlx::Error),

    #[error("Internal server error")]
    InternalError,

    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
