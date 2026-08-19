use serde::{Deserialize, Serialize};

use crate::feature::user::model::User;

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    id: i64,
    username: String,
}

impl UserClaims {
    pub const fn username(&self) -> &str {
        &self.username.as_str()
    }

    pub const fn id(&self) -> i64 {
        self.id
    }
}

impl From<User> for UserClaims {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            username: user.username().into(),
        }
    }
}
