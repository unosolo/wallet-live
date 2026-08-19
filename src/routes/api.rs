use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::app::state::AppState;
use crate::feature::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assets", get(asset::controller::list_assets))
        .route("/assets", post(asset::controller::create_asset))
        .route("/assets", patch(asset::controller::update_asset))
        .route(
            "/users/{user_id}/owned_assets",
            get(owned_asset::controller::list),
        )
        .route(
            "/users/{user_id}/owned_assets",
            post(owned_asset::controller::create),
        )
        .route(
            "/users/{user_id}/owned_assets/{owned_asset_id}",
            patch(owned_asset::controller::update),
        )
}
