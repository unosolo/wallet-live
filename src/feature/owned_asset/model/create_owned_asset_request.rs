use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateOwnedAssetRequest {
    pub asset_id: i64,
    pub bought_for: f64,
    pub quantity_owned: f64,
}
