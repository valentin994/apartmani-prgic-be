use crate::{dto::admin::Admin, error::ApiError, state::AppState};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State};
use tower_sessions::Session;

pub async fn login(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<Admin>,
) -> Result<(), ApiError> {
    tracing::info!("Logging in...");
    let (admin_id, hash): (i64, String) =
        sqlx::query_as("select id, password_hash from admins where username = ?")
            .bind(&payload.username)
            .fetch_optional(&state.db)
            .await?
            .ok_or(ApiError::Unauthorized)?;
    let parsed_hash = PasswordHash::new(&hash).map_err(|_| ApiError::Unauthorized)?;
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized)?;
    session.insert("admin_id", admin_id).await?;
    tracing::info!("Logged in");
    Ok(())
}

pub async fn logout(session: Session) -> Result<(), ApiError> {
    session.delete().await?;
    Ok(())
}
