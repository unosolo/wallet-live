use crate::app::error::AppError;
use crate::feature::asset::model::Asset;
use crate::feature::asset::{AssetRepository, model};

pub async fn create(
    repository: AssetRepository,
    asset: model::CreateAssetRequest,
) -> Result<Asset, AppError> {
    let new_asset = repository.add(asset).await?;
    Ok(new_asset)
}

pub async fn update(
    repository: AssetRepository,
    asset: model::UpdateAssetRequest,
) -> Result<Option<Asset>, AppError> {
    let updated_asset = repository.update(asset).await?;
    Ok(updated_asset)
}
