use std::convert::Infallible;

use axum::extract::FromRequestParts;
use axum_extra::extract::CookieJar;
use jwt_simple::claims::Claims;
use jwt_simple::prelude::*;
use jwt_simple::reexports::coarsetime::Duration;
use serde::Serialize;

use crate::app::error::AppError;
use crate::app::state::AppState;
use crate::feature::user::model::{UserClaims, UserRow};

const SECRET_KEY: &[u8] = b"I-am-so-s3cr3t";

#[derive(Serialize, Debug, Clone)]
pub struct User {
    id: i64,
    username: String,
}

impl User {
    pub fn new(user_row: UserRow) -> Self {
        Self {
            id: user_row.id,
            username: user_row.username,
        }
    }

    pub const fn username(&self) -> &str {
        &self.username.as_str()
    }

    pub const fn id(&self) -> i64 {
        self.id
    }

    pub fn auth_token(self) -> Result<String, AppError> {
        let key = HS256Key::from_bytes(SECRET_KEY);
        let claims = Claims::with_custom_claims(UserClaims::from(self), Duration::from_mins(60));
        let token = key.authenticate(claims)?;
        Ok(token)
    }

    pub fn from_auth_token(token: &str) -> Result<Self, AppError> {
        let key = HS256Key::from_bytes(SECRET_KEY);
        let claims: UserClaims = key.verify_token(token, None)?.custom;
        Ok(Self {
            id: claims.id(),
            username: claims.username().into(),
        })
    }
}

impl FromRequestParts<AppState> for User {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        println!("===DEBUG token: {:?}", &parts.headers);

        let token = match jar.get("token") {
            Some(token) => token.value(),
            None => return Err(AppError::Unauthenticated),
        };

        User::from_auth_token(token)
    }
}

impl FromRequestParts<AppState> for Option<User> {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(User::from_request_parts(parts, state).await.ok())
    }
}
