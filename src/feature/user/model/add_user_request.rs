use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddUserRequest {
    pub username: String,
    pub password_hash: String,
}
