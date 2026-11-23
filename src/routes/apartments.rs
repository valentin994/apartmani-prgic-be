use crate::{
    handlers::apartments::{list_apartments, update_apartment_price},
    state::AppState,
};

use axum::{
    Router,
    routing::{get, patch},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", patch(update_apartment_price))
        .route("/", get(list_apartments))
}
