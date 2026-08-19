use serde::Serialize;
use sqlx::types::Json;

use crate::feature::owned_asset::model::PurchaseHistory;

#[derive(Serialize)]
pub struct Holding {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub value_delta: f64,
    pub quantity_owned: f64,
    pub purchase_history: Json<Vec<PurchaseHistory>>,
}
