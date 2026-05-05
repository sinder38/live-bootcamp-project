use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use crate::domain::{AppResult, AuthAPIError};
use crate::services::banned_user_store::Token;
use crate::utils::auth::validate_token;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: Token,
}

pub async fn verify_token(
    State(state): State<AppState>,
    Json(data): Json<VerifyTokenRequest>,
) -> AppResult<impl IntoResponse> {
    validate_token(&data.token, state.banned_token_store)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    Ok(StatusCode::OK)
}
