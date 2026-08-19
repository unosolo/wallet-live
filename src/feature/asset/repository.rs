use axum::extract::FromRequestParts;
use sqlx::PgPool;
use std::convert::Infallible;

use crate::app::state::AppState;
use crate::feature::asset::model::{Asset, CreateAssetRequest, UpdateAssetRequest};

#[derive(Debug)]
pub struct AssetRepository {
    db: PgPool,
}

impl AssetRepository {
    pub async fn list(&self) -> sqlx::Result<Vec<Asset>> {
        sqlx::query_as!(Asset, "SELECT * FROM asset;")
            .fetch_all(&self.db)
            .await
    }

    pub async fn add(&self, asset: CreateAssetRequest) -> sqlx::Result<Asset> {
        sqlx::query_as!(
            Asset,
            "INSERT INTO asset (name, unit_value)
            VALUES ($1, $2) RETURNING *;",
            asset.name,
            asset.unit_value
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn update(&self, asset: UpdateAssetRequest) -> sqlx::Result<Option<Asset>> {
        sqlx::query_as!(
            Asset,
            "UPDATE asset
            SET name = COALESCE($1, name), unit_value = COALESCE($2, unit_value)
            WHERE id = $3
            RETURNING *;",
            asset.name,
            asset.unit_value,
            asset.id
        )
        .fetch_optional(&self.db)
        .await
    }
}

impl FromRequestParts<AppState> for AssetRepository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            db: state.db.clone(),
        })
    }
}

#[cfg(test)]
impl From<PgPool> for AssetRepository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
