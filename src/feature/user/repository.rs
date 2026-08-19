use axum::extract::FromRequestParts;
use sqlx::PgPool;
use std::convert::Infallible;

use crate::app::state::AppState;
use crate::feature::user::model::{AddUserRequest, UpdateUserRequest, UserRow};

#[derive(Debug)]
pub struct UserRepository {
    db: PgPool,
}

impl UserRepository {
    pub async fn get_by_username(&self, username: &str) -> sqlx::Result<Option<UserRow>> {
        sqlx::query_as!(
            UserRow,
            r#"SELECT * FROM "user" WHERE username = $1;"#,
            username
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn list(&self) -> sqlx::Result<Vec<UserRow>> {
        sqlx::query_as!(UserRow, r#"SELECT * FROM "user";"#)
            .fetch_all(&self.db)
            .await
    }

    pub async fn add(&self, user: AddUserRequest) -> sqlx::Result<UserRow> {
        sqlx::query_as!(
            UserRow,
            r#"INSERT INTO "user" (username, password_hash)
            VALUES ($1, $2) RETURNING *;"#,
            user.username,
            user.password_hash.into(),
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn update(&self, user: UpdateUserRequest) -> sqlx::Result<Option<UserRow>> {
        sqlx::query_as!(
            UserRow,
            r#"UPDATE "user"
            SET username = COALESCE($1, username), password_hash = COALESCE($2, password_hash)
            WHERE id = $3
            RETURNING *;"#,
            user.username,
            user.password_hash,
            user.id
        )
        .fetch_optional(&self.db)
        .await
    }
}

impl FromRequestParts<AppState> for UserRepository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            db: state.db.clone(),
        })
    }
}

#[cfg(test)]
impl From<PgPool> for UserRepository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
