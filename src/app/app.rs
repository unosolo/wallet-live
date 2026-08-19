use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::app::state::AppState;
use crate::routes::api::router as api_router;
use crate::routes::auth::router as auth_router;
use crate::routes::web::router as web_router;

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        // Define layer for a logger info into the consolue when running the server
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();
        tracing_subscriber::registry().with(layer).init();

        dotenvy::dotenv()?;

        // Define a router
        let state = AppState::new().await?;
        let api_routes = api_router().with_state(state.clone());
        let auth_routes = auth_router().with_state(state.clone());
        let web_routes = web_router().with_state(state.clone());
        let root_router = Router::new()
            .nest("/api", api_routes)
            .nest("/auth", auth_routes)
            .merge(web_routes)
            .with_state(state);

        info!("Starting server...at http://localhost:3000/");
        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        axum::serve(listener, root_router).await?;
        Ok(())
    }
}
