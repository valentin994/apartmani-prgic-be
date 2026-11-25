use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    auth::RequireAdmin,
    dto::{
        Paginated,
        reservations::{
            CreateReservation, Reservation, ReservationQuery, ReservationStatus,
            UpdateReservationStatus,
        },
    },
    error::ApiError,
    state::AppState,
};

pub async fn create_reservation(
    State(state): State<AppState>,
    Json(reservation): Json<CreateReservation>,
) -> Result<Json<Reservation>, ApiError> {
    tracing::info!("Adding reservation {:?}", reservation);
    if reservation.end_date <= reservation.start_date {
        return Err(ApiError::BadRequest(
            "End date must come after start date".to_string(),
        ));
    }
    let price: u32 = sqlx::query_scalar("select price from apartments where id = ?;")
        .bind(reservation.apartment_id)
        .fetch_one(&state.db)
        .await?;

    let total_price = price * (reservation.end_date - reservation.start_date).num_days() as u32;
    let created_reservation: Reservation = sqlx::query_as(
        r#"
    INSERT INTO reservations
        (apartment_id, guest_name, guest_email, guest_address,guest_phone_number, start_date, end_date, price, status, note)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    RETURNING *
    "#,
    )
    .bind(reservation.apartment_id)
    .bind(reservation.guest_name)
    .bind(reservation.guest_email.to_string())
    .bind(reservation.guest_address)
    .bind(reservation.guest_phone_number)
    .bind(reservation.start_date)
    .bind(reservation.end_date)
    .bind(total_price)
    .bind(ReservationStatus::Pending)
    .bind(reservation.note.unwrap_or_default())
    .fetch_one(&state.db)
    .await?;
    Ok(Json(created_reservation))
}

pub async fn list_reservations(
    _require_admin: RequireAdmin,
    Query(params): Query<ReservationQuery>,
    State(state): State<AppState>,
) -> Result<Json<Paginated<Reservation>>, ApiError> {
    tracing::info!("Listing all reservations");
    if params.page == 0 {
        return Err(ApiError::BadRequest(
            "page must be 1 or greater".to_string(),
        ));
    }
    let limit = params.limit.clamp(1, 50);
    let offset = (params.page - 1) as i64 * limit as i64;

    let apply_filters = |qb: &mut sqlx::QueryBuilder<sqlx::Sqlite>| {
        if let Some(status) = &params.status {
            qb.push(" AND status = ").push_bind(status.clone());
        }
        if let Some(apt_id) = params.apartment_id {
            qb.push(" AND apartment_id = ").push_bind(apt_id);
        }
        if let Some(start) = params.start_date {
            qb.push(" AND start_date >= ").push_bind(start);
        }
        if let Some(end) = params.end_date {
            qb.push(" AND end_date <= ").push_bind(end);
        }
    };

    let mut qb = sqlx::QueryBuilder::new("select * from reservations where 1=1");
    apply_filters(&mut qb);
    let mut count_qb = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM reservations WHERE 1=1");
    apply_filters(&mut count_qb);
    let total: i64 = count_qb.build_query_scalar().fetch_one(&state.db).await?;
    qb.push(" ORDER BY start_date DESC LIMIT ")
        .push_bind(limit as i64);
    qb.push(" OFFSET ").push_bind(offset);
    let reservations: Vec<Reservation> = qb.build_query_as().fetch_all(&state.db).await?;
    Ok(Json(Paginated {
        data: reservations,
        page: params.page,
        total,
        limit,
    }))
}

pub async fn update_reservation_status(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateReservationStatus>,
) -> Result<Json<Reservation>, ApiError> {
    tracing::info!("updating apartment: {}", id);
    let reservation: Reservation = sqlx::query_as(
        r#"
        UPDATE reservations
        SET status = ?
        WHERE id = ?
        RETURNING *
        "#,
    )
    .bind(payload.status)
    .bind(id)
    .fetch_one(&state.db)
    .await?;
    Ok(Json(reservation))
}
