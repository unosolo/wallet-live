use crate::app::error::AppError;
use crate::feature::owned_asset::OwnedAssetRepository;
use crate::feature::owned_asset::model::{
    CreateOwnedAssetRequest, OwnedAsset, UpdateOwnedAssetRequest,
};

pub async fn create(
    repository: OwnedAssetRepository,
    user_id: i64,
    owned_asset: CreateOwnedAssetRequest,
) -> Result<OwnedAsset, AppError> {
    let new_asset = repository.add(user_id, owned_asset).await?;
    Ok(new_asset)
}

pub async fn update(
    repository: OwnedAssetRepository,
    user_id: i64,
    owned_asset_id: i64,
    owned_asset: UpdateOwnedAssetRequest,
) -> Result<Option<OwnedAsset>, AppError> {
    let updated_asset = repository
        .update(user_id, owned_asset_id, owned_asset)
        .await?;
    Ok(updated_asset)
}
