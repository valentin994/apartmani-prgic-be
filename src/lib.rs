pub mod auth;
pub mod dto;
pub mod error;
pub mod handlers;
pub mod routes;
pub mod state;

use crate::state::AppState;
use axum::{
    Router,
    http::{HeaderValue, Method},
};
use tower_http::cors::CorsLayer;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::SqliteStore;

pub fn app(state: AppState, session_store: SqliteStore) -> Router {
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://apartmani-prgic.hr".parse::<HeaderValue>().unwrap(),
            "https://www.apartmani-prgic.hr"
                .parse::<HeaderValue>()
                .unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH])
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_credentials(true);
    let session_layer = SessionManagerLayer::new(session_store);
    Router::new()
        .merge(routes::router())
        .layer(session_layer)
        .layer(cors)
        .with_state(state)
}
