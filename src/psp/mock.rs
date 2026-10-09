pub struct ChargeResult {
    pub success: bool,
    pub provider_reference: String,
}

pub async fn charge(amount: i64) -> ChargeResult {
    let success = amount != 9999;

    ChargeResult {
        success,
        provider_reference: format!("mock_{}", uuid::Uuid::new_v4()),
    }
}
