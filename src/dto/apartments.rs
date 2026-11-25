use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Apartment {
    pub id: u64,
    pub price: u16,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApartment {
    pub price: u16,
}
