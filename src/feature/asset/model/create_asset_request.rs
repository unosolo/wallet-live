use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAssetRequest {
    pub name: String,
    pub unit_value: f64,
}
