use axum::{Router, routing::post};

use crate::app::state::AppState;
use crate::feature::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::controller::login))
        .route("/logout", post(auth::controller::logout))
}
