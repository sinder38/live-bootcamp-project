use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

use crate::domain::{AppResult, AuthAPIError};
use crate::utils::auth::validate_token;
use crate::utils::constants::JWT_COOKIE_NAME;

pub async fn logout(jar: CookieJar) -> AppResult<(CookieJar, impl IntoResponse)> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;

    validate_token(cookie.value())
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    Ok((jar, StatusCode::OK))
}
