use axum::Json; // Add State to imports
use axum::extract::{Path, State};

use crate::app::error::AppError;
use crate::app::state::AppState;
//use crate::feature::auth::Admin;
use crate::feature::owned_asset::model::{
    CreateOwnedAssetRequest, Holding, OwnedAsset, UpdateOwnedAssetRequest,
};
use crate::feature::owned_asset::{OwnedAssetRepository, service};
use crate::feature::user::model::User;

#[tracing::instrument]
pub async fn list(
    State(_state): State<AppState>,
    user: User,
    repository: OwnedAssetRepository,
    Path(_user_id): Path<i64>,
) -> Result<Json<Vec<Holding>>, AppError> {
    let assets = repository.list(user.id()).await?;
    Ok(Json(assets))
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn create(
    State(_state): State<AppState>,
    user: User,
    repository: OwnedAssetRepository,
    Path(_user_id): Path<i64>,
    Json(owned_asset): Json<CreateOwnedAssetRequest>,
) -> Result<Json<OwnedAsset>, AppError> {
    let new_asset = service::create(repository, user.id(), owned_asset).await?;
    Ok(Json(new_asset))
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn update(
    State(_state): State<AppState>,
    user: User,
    repository: OwnedAssetRepository,
    Path((_, owned_asset_id)): Path<(i64, i64)>,
    Json(owned_asset): Json<UpdateOwnedAssetRequest>,
) -> Result<Json<OwnedAsset>, AppError> {
    match service::update(repository, user.id(), owned_asset_id, owned_asset).await? {
        Some(updated_asset) => Ok(Json(updated_asset)),
        None => Err(AppError::ResourceNotFound),
    }
}
