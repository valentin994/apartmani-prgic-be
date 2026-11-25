use crate::{
    auth::RequireAdmin,
    dto::apartments::{Apartment, UpdateApartment},
    error::ApiError,
    state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn list_apartments(
    State(state): State<AppState>,
) -> Result<Json<Vec<Apartment>>, ApiError> {
    tracing::info!("trying to fetch all apartments");
    let apartments: Vec<Apartment> = sqlx::query_as("select * from apartments;")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(apartments))
}

pub async fn update_apartment_price(
    _require_admin: RequireAdmin,
    State(state): State<AppState>,
    Path(id): Path<u16>,
    Json(apartment): Json<UpdateApartment>,
) -> Result<Json<Apartment>, ApiError> {
    tracing::info!("Updating apt: {}, with price {}", id, apartment.price);
    let apartment: Apartment =
        sqlx::query_as("update apartments SET price = COALESCE(?, price) where id = ? returning *")
            .bind(apartment.price)
            .bind(id)
            .fetch_one(&state.db)
            .await?;
    Ok(Json(apartment))
}
