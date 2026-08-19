use password_auth::VerifyError;
use serde::Deserialize;

use crate::app::error::AppError;
use crate::feature::user::UserRepository;
use crate::feature::user::model::{AddUserRequest, User, UserRow};

#[derive(Deserialize)]
pub struct UnauthenticatedUser {
    username: String,
    password: String,
}

impl UnauthenticatedUser {
    pub async fn authenticate(&self, repository: &UserRepository) -> Result<User, AppError> {
        println!("===DEBUG Authenticando {}...", self.username);
        let user_row: UserRow = match repository.get_by_username(&self.username).await? {
            Some(user_row) => user_row,
            None => return Err(AppError::ResourceNotFound),
        };

        match password_auth::verify_password(&self.password, &user_row.password_hash) {
            Ok(()) => Ok(User::new(user_row)),
            Err(VerifyError::PasswordInvalid) => Err(AppError::Unauthenticated),
            Err(VerifyError::Parse(err)) => panic!("Hashing algorithm failed: {err}"),
        }
    }

    pub async fn register(self, repository: &UserRepository) -> Result<User, AppError> {
        println!("===DEBUG REgistering {}", self.username);
        let password_hash = password_auth::generate_hash(self.password);
        let user_request = AddUserRequest {
            username: self.username,
            password_hash,
        };
        let user_row: UserRow = match repository.add(user_request).await {
            Ok(user_row) => user_row,
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                return Err(AppError::Unauthenticated);
            }
            Err(err) => return Err(AppError::Database(err)),
        };

        Ok(User::new(user_row))
    }
}
