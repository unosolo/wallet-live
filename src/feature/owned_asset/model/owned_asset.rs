use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct OwnedAsset {
    pub id: i64,
    pub user_id: i64,
    pub asset_id: i64,
    pub bought_for: f64,
    pub quantity_owned: f64,
    // This points to the 'time' crate's helper module
    #[serde(with = "time::serde::iso8601::option")]
    pub timestamp: Option<OffsetDateTime>,
}
