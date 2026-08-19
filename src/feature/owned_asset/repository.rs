use axum::extract::FromRequestParts;
use sqlx::PgPool;
use std::convert::Infallible;

use crate::app::state::AppState;
use crate::feature::owned_asset::model::{
    CreateOwnedAssetRequest, Holding, OwnedAsset, UpdateOwnedAssetRequest,
};

#[derive(Debug)]
pub struct OwnedAssetRepository {
    db: PgPool,
}

impl OwnedAssetRepository {
    pub async fn list(&self, user_id: i64) -> sqlx::Result<Vec<Holding>> {
        sqlx::query_as!(
            Holding,
            r#"SELECT
                a.id,
                a.name,
                a.unit_value,
                SUM((a.unit_value - o.bought_for) * o.quantity_owned) AS "value_delta!",
                SUM(o.quantity_owned) AS "quantity_owned!",
                JSON_AGG(
                    JSON_BUILD_OBJECT(
                        'bought_at', o.timestamp,
                        'bought_for', o.bought_for,
                        'quantity_bought', o.quantity_owned,
                        'value_delta', (a.unit_value - o.bought_for) * o.quantity_owned
                    )
                ) AS "purchase_history!: _"
            FROM asset AS a
                INNER JOIN owned_asset AS o
                    ON o.asset_id = a.id
            WHERE o.user_id = $1
            GROUP BY a.id;"#,
            user_id
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn add(
        &self,
        user_id: i64,
        owned_asset: CreateOwnedAssetRequest,
    ) -> sqlx::Result<OwnedAsset> {
        sqlx::query_as!(
            OwnedAsset,
            "INSERT INTO owned_asset (user_id, asset_id, bought_for, quantity_owned)
            VALUES ($1, $2, $3, $4)
            RETURNING *;",
            user_id,
            owned_asset.asset_id,
            owned_asset.bought_for,
            owned_asset.quantity_owned
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn update(
        &self,
        user_id: i64,
        owned_asset_id: i64,
        owned_asset: UpdateOwnedAssetRequest,
    ) -> sqlx::Result<Option<OwnedAsset>> {
        sqlx::query_as!(
            OwnedAsset,
            "UPDATE owned_asset
            SET bought_for = COALESCE($1, bought_for), quantity_owned = COALESCE($2, quantity_owned)
            WHERE id = $3 and user_id = $4
            RETURNING *;",
            owned_asset.bought_for,
            owned_asset.quantity_owned,
            user_id,
            owned_asset_id,
        )
        .fetch_optional(&self.db)
        .await
    }
}

impl FromRequestParts<AppState> for OwnedAssetRepository {
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
impl From<PgPool> for OwnedAssetRepository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
