use crate::{app_state::AppState, domain::User};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let mut user_store = state.user_store.write().await;
    let new_user = User {
        email: request.email,
        password: request.password,
        requires_2fa: request.requires_2fa,
    };

    user_store.add_user(new_user).expect("Failed to add user");

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}
