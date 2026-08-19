use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAssetRequest {
    pub id: i64,
    pub name: Option<String>,
    pub unit_value: Option<f64>,
}
