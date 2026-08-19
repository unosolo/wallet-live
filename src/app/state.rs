use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new() -> color_eyre::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        tracing::info!("Connecting to DB at {}", &database_url);
        let db = PgPoolOptions::new()
            .max_connections(5)
            // Max time a connection can exist before being recycled
            .max_lifetime(Duration::from_secs(30 * 60))
            // Max time a connection can sit idle in the pool
            .idle_timeout(Duration::from_secs(10 * 60))
            .connect(&database_url)
            .await?;
        tracing::info!("Connected to DB at {}", &database_url);
        Ok(Self { db })
    }
}
