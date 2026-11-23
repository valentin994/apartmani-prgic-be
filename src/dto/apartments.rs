use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Apartment {
    pub id: u16,
    pub price: u16,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApartment {
    pub price: u16,
}
