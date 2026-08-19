use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateOwnedAssetRequest {
    pub bought_for:  Option<f64>,
    pub quantity_owned:  Option<f64>,
}
