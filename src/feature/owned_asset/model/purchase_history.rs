use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// purchased history struct with bought_at, bought_for, quantity_bought and value_delta fields
#[derive(Serialize, Deserialize)]
pub struct PurchaseHistory {
    #[serde(with = "time::serde::iso8601")]
    pub bought_at: OffsetDateTime,
    pub bought_for: f64,
    pub quantity_bought: f64,
    pub value_delta: f64,
}
