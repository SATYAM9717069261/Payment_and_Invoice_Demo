use sqlx::PgPool;
use uuid::Uuid;

use crate::models::payments::Payment;

#[derive(Debug)]
pub enum StartPaymentError {
    InvoiceNotFound,
    InvoiceUnavailable,
    Database(sqlx::Error),
}

pub async fn start_payment(pool: &PgPool, invoice_id: Uuid) -> Result<Payment, StartPaymentError> {
    let mut tx = pool.begin().await.map_err(StartPaymentError::Database)?;

    let invoice = sqlx::query_as::<_, (i64,)>(
        r#"
        UPDATE invoices
        SET status = 'processing',
            updated_at = NOW()
        WHERE id = $1
          AND status IN ('pending', 'failed')
        RETURNING amount
        "#,
    )
    .bind(invoice_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(StartPaymentError::Database)?;

    let Some((amount,)) = invoice else {
        let exists =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM invoices WHERE id = $1)")
                .bind(invoice_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(StartPaymentError::Database)?;

        tx.rollback().await.map_err(StartPaymentError::Database)?;

        return if exists {
            Err(StartPaymentError::InvoiceUnavailable)
        } else {
            Err(StartPaymentError::InvoiceNotFound)
        };
    };

    let payment = sqlx::query_as::<_, Payment>(
        r#"
        INSERT INTO payments (id, invoice_id, amount, status)
        VALUES ($1, $2, $3, 'processing')
        RETURNING
            id, invoice_id, amount, status,
            provider_reference, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(invoice_id)
    .bind(amount)
    .fetch_one(&mut *tx)
    .await
    .map_err(StartPaymentError::Database)?;

    tx.commit().await.map_err(StartPaymentError::Database)?;

    Ok(payment)
}

pub async fn finish_payment(
    pool: &PgPool,
    payment_id: Uuid,
    invoice_id: Uuid,
    success: bool,
    provider_reference: &str,
) -> Result<Payment, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let payment_status = if success { "succeeded" } else { "failed" };
    let invoice_status = if success { "paid" } else { "failed" };

    let payment = sqlx::query_as::<_, Payment>(
        r#"
        UPDATE payments
        SET status = $2,
            provider_reference = $3
        WHERE id = $1
          AND status = 'processing'
        RETURNING
            id, invoice_id, amount, status,
            provider_reference, created_at
        "#,
    )
    .bind(payment_id)
    .bind(payment_status)
    .bind(provider_reference)
    .fetch_one(&mut *tx)
    .await?;

    let updated = sqlx::query(
        r#"
        UPDATE invoices
        SET status = $2,
            updated_at = NOW()
        WHERE id = $1
          AND status = 'processing'
        "#,
    )
    .bind(invoice_id)
    .bind(invoice_status)
    .execute(&mut *tx)
    .await?;

    if updated.rows_affected() != 1 {
        return Err(sqlx::Error::Protocol(
            "Invoice was not in processing state".to_string(),
        ));
    }

    tx.commit().await?;

    Ok(payment)
}
