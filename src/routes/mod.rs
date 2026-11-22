use axum::Router;
pub mod health;

use health::router as health_router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest("/health", health_router())
}
