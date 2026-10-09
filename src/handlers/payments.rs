use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::payments::Payment,
    psp::mock,
    repositories::payments::{self, StartPaymentError},
};

pub async fn start_payment(
    State(pool): State<PgPool>,
    Path(invoice_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Payment>), StatusCode> {
    let payment = payments::start_payment(&pool, invoice_id)
        .await
        .map_err(|err| match err {
            StartPaymentError::InvoiceNotFound => StatusCode::NOT_FOUND,
            StartPaymentError::InvoiceUnavailable => StatusCode::CONFLICT,
            StartPaymentError::Database(err) => {
                tracing::error!("Failed to start payment: {err}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    // The database transaction has committed before calling the PSP.
    let result = mock::charge(payment.amount).await;

    let completed = payments::finish_payment(
        &pool,
        payment.id,
        invoice_id,
        result.success,
        &result.provider_reference,
    )
    .await
    .map_err(|err| {
        tracing::error!("Failed to save payment result: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, Json(completed)))
}
