use sqlx::PgPool;
use uuid::Uuid;

use crate::models::customer::Customer;

pub async fn create(pool: &PgPool, email: &str, name: &str) -> Result<Customer, sqlx::Error> {
    sqlx::query_as::<_, Customer>(
        r#"
        INSERT INTO customers (id, email, name)
        VALUES ($1, $2, $3)
        RETURNING id, email, name, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(email)
    .bind(name)
    .fetch_one(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Customer>, sqlx::Error> {
    sqlx::query_as::<_, Customer>(
        r#"
        SELECT id, email, name, created_at
        FROM customers
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
