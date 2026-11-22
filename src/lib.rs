pub mod dto;
pub mod error;
pub mod handlers;
pub mod routes;
pub mod state;

use axum::{
    Router,
    http::{HeaderValue, Method},
};
use tower_http::cors::CorsLayer;

use crate::state::AppState;

pub fn app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://apartmani-prgic.hr".parse::<HeaderValue>().unwrap(),
            "https://www.apartmani-prgic.hr"
                .parse::<HeaderValue>()
                .unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_credentials(true);
    Router::new()
        .merge(routes::router())
        .layer(cors)
        .with_state(state)
}
