use crate::app::error::AppError;
use crate::app::response::RedirectResponse;
use crate::app::state::AppState;
use crate::feature::user::{
    UserRepository,
    model::{UnauthenticatedUser, User},
};
use axum::extract::State;
use axum::{Json, response::IntoResponse};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite}; // Import the Cookie;

#[axum::debug_handler]
pub async fn login(
    State(_state): State<AppState>,
    repository: UserRepository,
    jar: CookieJar,
    Json(unauthenticated_user): Json<UnauthenticatedUser>,
) -> Result<impl IntoResponse, AppError> {
    let user: User = match unauthenticated_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::ResourceNotFound) => unauthenticated_user.register(&repository).await?,
        Err(other_error) => return Err(other_error),
    };

    let token = user.auth_token()?;

    let cookie = Cookie::build(("token", token))
        .http_only(true)
        .path("/") // Valid for the whole site
        .same_site(SameSite::Lax)
        .secure(false)
        .build();

    let response = RedirectResponse { redirect_url: "/" };

    Ok((jar.add(cookie), Json(response)))
}

#[axum::debug_handler]
pub async fn logout(jar: CookieJar) -> Result<impl IntoResponse, AppError> {
    let cookie = Cookie::build(("token", String::from("")))
        .http_only(true)
        .path("/") // Valid for the whole site
        .same_site(SameSite::Lax)
        .secure(false)
        .build();

    let response = RedirectResponse { redirect_url: "/" };

    Ok((jar.add(cookie), Json(response)))
}
