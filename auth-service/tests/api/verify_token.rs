use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::StatusCode;

use crate::{helpers::TestApp, signup_and_login};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = vec![
        serde_json::json!({
            "token": true,
        }),
        serde_json::json!({}),
        serde_json::json!({"tok": ""}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_token(&test_case).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let (app, response) = signup_and_login!();

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie");

    assert!(!auth_cookie.value().is_empty());

    let token = auth_cookie.value();

    let verify_token_body = serde_json::json!({
        "token": &token,
    });

    let response = app.post_verify_token(&verify_token_body).await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let test_cases = vec!["invalid_token", "tok", ""];

    for test_case in test_cases {
        let verify_token_body = serde_json::json!({
            "token": test_case,
        });

        let response = app.post_verify_token(&verify_token_body).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Deserializetion error")
                .error,
            "Invalid auth token".to_owned()
        );
    }
}

//...
