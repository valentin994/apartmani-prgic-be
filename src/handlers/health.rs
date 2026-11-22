use axum::{extract::State, http::StatusCode};

use crate::{error::ApiError, state::AppState};

pub async fn ping(State(state): State<AppState>) -> Result<StatusCode, ApiError> {
    tracing::info!("Healthcheck");
    sqlx::query("select 1").execute(&state.db).await?;
    Ok(StatusCode::OK)
}
