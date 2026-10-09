use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub amount: i64,
    pub status: String,
    pub provider_reference: Option<String>,
    pub created_at: DateTime<Utc>,
}
