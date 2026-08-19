use askama::Template;
use axum::response::Redirect;

use crate::app::error::AppError;
use crate::feature::user::model::User;

#[derive(Template)]
#[template(path = "home/home.html")]
struct HomePage;

pub async fn index(maybe_user: Option<User>) -> Result<Redirect, AppError> {
    match maybe_user {
        Some(_) => Ok(Redirect::to("/assets")),
        None => Ok(Redirect::to("/login")),
    }
}
