use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::customer::Customer, repositories::customer};

#[derive(Deserialize)]
pub struct CreateCustomer {
    pub email: String,
    pub name: String,
}

pub async fn create_customer(
    State(pool): State<PgPool>,
    Json(input): Json<CreateCustomer>,
) -> Result<(StatusCode, Json<Customer>), StatusCode> {
    if input.email.trim().is_empty() || input.name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let customer = customer::create(&pool, &input.email, &input.name)
        .await
        .map_err(|err| {
            if let sqlx::Error::Database(db_err) = &err {
                if db_err.code().as_deref() == Some("23505") {
                    return StatusCode::CONFLICT;
                }
            }

            tracing::error!("Failed to create customer: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::CREATED, Json(customer)))
}

pub async fn get_customer(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Customer>, StatusCode> {
    let customer = customer::get_by_id(&pool, id)
        .await
        .map_err(|err| {
            tracing::error!("Failed to retrieve customer: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(customer))
}
