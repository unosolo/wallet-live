use axum::{extract::FromRequestParts, http::header::AUTHORIZATION};

use crate::app::error::AppError;
use crate::app::state::AppState;

const ADMIN_SECRET_KEY: &'static str = "admin-secret-key";

pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Some(auth) = parts.headers.get(AUTHORIZATION) else {
            return Err(AppError::Unauthenticated);
        };

        if auth != ADMIN_SECRET_KEY {
            return Err(AppError::Unauthenticated);
        }

        Ok(Self)
    }
}
