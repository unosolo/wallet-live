use axum::Json;
use axum::extract::State;
use sqlx::PgPool; // Add State to imports

use crate::app::state::AppState;
use crate::feature::asset::controller::{create_asset, list_assets, update_asset};
use crate::feature::asset::model::{CreateAssetRequest, UpdateAssetRequest};
use crate::feature::auth::Admin;

// using sqlx for async tests
#[sqlx::test]
async fn test_create_asset(db: PgPool) {
    let state = AppState { db: db.clone() };

    let asset = CreateAssetRequest {
        name: "Test".to_string(),
        unit_value: 0.0,
    };

    let Json(new_asset) = create_asset(
        Admin,
        State(state.clone()),
        state.db.into(),
        Json(asset.clone()),
    )
    .await
    .expect("success");

    assert_eq!(asset.name, new_asset.name);
    assert_eq!(asset.unit_value, new_asset.unit_value);

    insta::assert_json_snapshot!(new_asset);
}

#[sqlx::test(fixtures("bitcoin_asset"))]
async fn test_list_assets(db: PgPool) {
    let Json(assets) = list_assets(db.into()).await.expect("success");

    assert_eq!(assets.len(), 1);
    assert_eq!(assets[0].name, "Bitcoin");

    insta::assert_json_snapshot!(assets);
}

#[sqlx::test(fixtures("bitcoin_asset"))]
async fn test_update_asset(db: PgPool) {
    let state = AppState { db: db.clone() };

    let asset = UpdateAssetRequest {
        id: 1,
        name: Some("Test".to_string()),
        unit_value: Some(20.0),
    };

    let Json(updated_asset) = update_asset(
        Admin,
        State(state.clone()),
        state.db.into(),
        Json(asset.clone()),
    )
    .await
    .expect("success");

    assert_eq!(asset.name.unwrap(), updated_asset.name);
    assert_eq!(asset.unit_value.unwrap(), updated_asset.unit_value);

    insta::assert_json_snapshot!(updated_asset);
}
