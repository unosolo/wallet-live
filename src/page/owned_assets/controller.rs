use askama::Template;
use axum::response::Html;
use tokio::try_join;

use crate::app::error::AppError;
use crate::feature::asset::AssetRepository;
use crate::feature::asset::model::Asset;
use crate::feature::owned_asset::OwnedAssetRepository;
use crate::feature::owned_asset::model::Holding;
use crate::feature::user::model::User;
use crate::global::askama_filters as filters;

#[derive(Template)]
#[template(path = "owned_assets/owned_assets.html")]
struct AssetPage {
    holdings: Vec<Holding>,
    available_assets: Vec<Asset>,
    user: User,
}

pub async fn index(
    asset_repository: AssetRepository,
    owned_asset_repository: OwnedAssetRepository,
    user: User,
) -> Result<Html<String>, AppError> {
    let (holdings, available_assets) = try_join!(
        owned_asset_repository.list(user.id()),
        asset_repository.list(),
    )?;

    let html = AssetPage {
        holdings,
        available_assets,
        user,
    }
    .render()?;

    Ok(Html(html))
}
