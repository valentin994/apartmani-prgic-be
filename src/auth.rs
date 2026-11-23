use axum::{extract::FromRequestParts, http::request::Parts};
use tower_sessions::Session;

use crate::{error::ApiError, state::AppState};

pub struct RequireAdmin;

impl FromRequestParts<AppState> for RequireAdmin {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::Unauthorized)?;

        session
            .get::<i64>("admin_id")
            .await?
            .ok_or(ApiError::Unauthorized)?;
        Ok(RequireAdmin)
    }
}
