use crate::{
    handlers::admin::{login, logout},
    state::AppState,
};

use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(login))
        .route("/logout", post(logout))
}
