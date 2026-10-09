use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::invoice::Invoice, repositories::invoice};

#[derive(Deserialize)]
pub struct CreateInvoice {
    pub customer_id: Uuid,
    pub amount: i64,
    pub currency: String,
}

pub async fn create_invoice(
    State(pool): State<PgPool>,
    Json(input): Json<CreateInvoice>,
) -> Result<(StatusCode, Json<Invoice>), StatusCode> {
    if input.amount <= 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let currency = input.currency.trim().to_ascii_uppercase();

    if currency.len() != 3 || !currency.bytes().all(|b| b.is_ascii_alphabetic()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let result = invoice::create(&pool, input.customer_id, input.amount, &currency).await;

    match result {
        Ok(invoice) => Ok((StatusCode::CREATED, Json(invoice))),
        Err(sqlx::Error::Database(err)) if err.code().as_deref() == Some("23503") => {
            Err(StatusCode::NOT_FOUND)
        }
        Err(err) => {
            tracing::error!("Failed to create invoice: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_invoice(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Invoice>, StatusCode> {
    let result = invoice::get_by_id(&pool, id).await.map_err(|err| {
        tracing::error!("Failed to retrieve invoice: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    result.map(Json).ok_or(StatusCode::NOT_FOUND)
}
