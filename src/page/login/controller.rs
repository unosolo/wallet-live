use askama::Template;
use axum::response::Html;

use crate::app::error::AppError;

#[derive(Template)]
#[template(path = "login/login.html")]
struct LoginPage;

pub async fn index() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}
