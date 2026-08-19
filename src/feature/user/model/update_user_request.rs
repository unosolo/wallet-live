use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub id: i64,
    pub username: Option<String>,
    pub password_hash: Option<String>,
}
