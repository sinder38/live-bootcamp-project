use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

use crate::domain::{AppResult, AuthAPIError};
use crate::services::banned_user_store::Token;
use crate::utils::auth::validate_token;
use crate::utils::constants::JWT_COOKIE_NAME;
use crate::AppState;

pub async fn logout(
    jar: CookieJar,
    State(state): State<AppState>,
) -> AppResult<(CookieJar, impl IntoResponse)> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;
    let token: Token = cookie.value().to_string();

    validate_token(&token, state.banned_token_store.clone())
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    state
        .banned_token_store
        .write()
        .await
        .add_token(token)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    Ok((jar, StatusCode::OK))
}
