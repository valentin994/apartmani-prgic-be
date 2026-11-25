use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_email::Email;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Reservation {
    pub id: i64,
    pub apartment_id: i64,
    pub guest_name: String,
    pub guest_email: String,
    pub guest_address: String,
    pub guest_phone_number: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub price: u32,
    pub status: ReservationStatus,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReservation {
    pub apartment_id: i64,
    pub guest_name: String,
    pub guest_email: Email,
    pub guest_phone_number: String,
    pub guest_address: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ReservationStatus {
    Pending,
    Accepted,
    Confirmed,
    Canceled,
}

#[derive(Debug, Deserialize)]
pub struct ReservationQuery {
    pub status: Option<ReservationStatus>,
    pub apartment_id: Option<i64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}
fn default_limit() -> u32 {
    20
}

#[derive(Debug, Deserialize)]
pub struct UpdateReservationStatus {
    pub status: ReservationStatus,
}
