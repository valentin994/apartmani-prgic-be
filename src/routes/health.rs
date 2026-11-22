use axum::{Router, routing::get};

use crate::{handlers::health::ping, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(ping))
}
