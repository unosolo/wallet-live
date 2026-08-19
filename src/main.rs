mod app;
mod feature;
mod global;
mod page;
mod routes;

use crate::app::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
