use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error {0}")]
    Database(#[from] sqlx::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Session error: {0}")]
    Session(#[from] tower_sessions::session::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let response = match self {
            ApiError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized for this action".to_string(),
            ),
            ApiError::Session(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        response.into_response()
    }
}
