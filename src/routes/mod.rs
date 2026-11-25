use axum::Router;
pub mod admin;
pub mod apartments;
pub mod health;
pub mod reservations;

use admin::router as admin_router;
use apartments::router as apartments_router;
use health::router as health_router;
use reservations::router as reservations_router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/health", health_router())
        .nest("/admin", admin_router())
        .nest("/apartments", apartments_router())
        .nest("/reservations", reservations_router())
}
