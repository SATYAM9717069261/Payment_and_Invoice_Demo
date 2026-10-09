use sqlx::PgPool;
use uuid::Uuid;

use crate::models::invoice::Invoice;

pub async fn create(
    pool: &PgPool,
    customer_id: Uuid,
    amount: i64,
    currency: &str,
) -> Result<Invoice, sqlx::Error> {
    sqlx::query_as::<_, Invoice>(
        r#"
        INSERT INTO invoices (id, customer_id, amount, currency)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id, customer_id, amount, currency,
            status, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(customer_id)
    .bind(amount)
    .bind(currency)
    .fetch_one(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Invoice>, sqlx::Error> {
    sqlx::query_as::<_, Invoice>(
        r#"
        SELECT
            id, customer_id, amount, currency,
            status, created_at, updated_at
        FROM invoices
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
