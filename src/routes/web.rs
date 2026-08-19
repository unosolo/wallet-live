use axum::{Router, routing::get};

use crate::app::state::AppState;
use crate::page::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(home::controller::index))
        .route("/login", get(login::controller::index))
        .route("/assets", get(owned_assets::controller::index))
}
