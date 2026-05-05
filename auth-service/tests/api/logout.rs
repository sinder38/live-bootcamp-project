use crate::{
    helpers::{get_random_email, TestApp},
    signup_and_login,
};
use auth_service::{domain::BannedTokenStore, utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::{StatusCode, Url};

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Deserializetion error")
            .error,
        "Missing token".to_owned()
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse(&app.address).expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Deserializetion error")
            .error,
        "Invalid auth token".to_owned()
    );
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let (app, response) = signup_and_login!();

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_logout().await;

    // Save login token
    let token = auth_cookie.value();

    assert_eq!(response.status(), StatusCode::OK);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie");

    assert!(auth_cookie.value().is_empty());
    let banned_token_store = app.banned_token_store.read().await;
    let contains_token = banned_token_store
        .contains_token(token)
        .await
        .expect("Failed to check if token is banned");

    assert!(contains_token);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let (app, response) = signup_and_login!();

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_logout().await;

    assert_eq!(response.status(), StatusCode::OK);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie");

    assert!(auth_cookie.value().is_empty());

    // Second logout should return 400
    let response = app.post_logout().await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Deserializetion error")
            .error,
        "Missing token".to_string()
    );
}
