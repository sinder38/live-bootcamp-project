use crate::domain::{AppResult, AuthAPIError, Email, Password};
use crate::utils::auth::{self, generate_auth_cookie};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// The login route can return 2 possible success responses
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

/// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
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

    let user = user_store
        .get_user(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    // Handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_2fa(jar: CookieJar) -> AppResult<(CookieJar, Json<LoginResponse>)> {
    let payload = TwoFactorAuthResponse {
        message: "2FA required".to_string(),
        login_attempt_id: "123456".to_string(),
    };
    Ok((jar, Json(LoginResponse::TwoFactorAuth(payload))))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> AppResult<(CookieJar, Json<LoginResponse>)> {
    let auth_cookie =
        auth::generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;

    let updated_jar = jar.add(auth_cookie);

    Ok((updated_jar, Json(LoginResponse::RegularAuth)))
}
