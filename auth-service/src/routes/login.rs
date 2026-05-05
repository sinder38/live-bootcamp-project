use crate::domain::{AppResult, AuthAPIError, Email, Password};
use crate::utils::auth;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> AppResult<(CookieJar, impl IntoResponse)> {
    // NOTE: Disclosing credential requirements is acceptable
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.read().await;

    user_store
        .validate_user(&email, &password)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    // NOTE: requesting user is redundant.
    //
    // // TODO: call `user_store.get_user`. Return AuthAPIError::IncorrectCredentials if the operation fails.
    // let _user = user_store
    //     .get_user(&email)
    //     .await
    //     .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    // Call the generate_auth_cookie function defined in the auth module.
    // If the function call fails return AuthAPIError::UnexpectedError.
    let auth_cookie =
        auth::generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;

    let updated_jar = jar.add(auth_cookie);

    Ok((updated_jar, StatusCode::OK.into_response()))
}
