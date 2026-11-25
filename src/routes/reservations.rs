use crate::{
    handlers::reservations::{create_reservation, list_reservations, update_reservation_status},
    state::AppState,
};

use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", patch(update_reservation_status))
        .route("/", post(create_reservation))
        .route("/", get(list_reservations))
}
