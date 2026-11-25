pub mod admin;
pub mod apartments;
pub mod reservations;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
}
