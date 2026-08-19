use axum::Json; // Add State to imports
use axum::extract::State;

use crate::app::error::AppError;
use crate::app::state::AppState;
use crate::feature::asset::model::Asset;
use crate::feature::asset::{AssetRepository, model, service};
use crate::feature::auth::Admin;

#[tracing::instrument]
pub async fn list_assets(repository: AssetRepository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list().await?;
    Ok(Json(assets))
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn create_asset(
    _: Admin,
    State(_state): State<AppState>,
    repository: AssetRepository,
    Json(request): Json<model::CreateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let new_asset = service::create(repository, request).await?;
    Ok(Json(new_asset))
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn update_asset(
    _: Admin,
    State(_state): State<AppState>,
    repository: AssetRepository,
    Json(request): Json<model::UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    match service::update(repository, request).await? {
        Some(updated_asset) => Ok(Json(updated_asset)),
        None => Err(AppError::ResourceNotFound),
    }
}
