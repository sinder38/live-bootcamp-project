use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::domain::{AuthAPIError, Email, Password};
use crate::AppState;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
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

    Ok(StatusCode::OK.into_response())
}
