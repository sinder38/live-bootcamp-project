use crate::helpers::{get_random_email, TestApp};

/// Sets up a test application with a random email and password, and signs up the user.
macro_rules! setup  {
    () => {
        {
            let app = TestApp::new().await;

            let random_email = get_random_email();

            let signup_body = serde_json::json!({
                "email": random_email,
                "password": "password123",
                "requires2FA": false
            });

            app.post_signup(&signup_body).await;

            app
        }
    };
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = setup!();

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({
            "email": random_email,
            "pwd": "password123"
        }),
    ];
    for test_case in test_cases {
        let response = app.post_login(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = setup!();

    let test_cases = [
        serde_json::json!({
            "email": get_random_email(),
            "password": "",
        }),
        serde_json::json!({
            "email": "eeee".to_string(),
            "password": "password123"
        }),
        serde_json::json!({
            "email": "eeee".to_string(),
            "password": "eeee"
        }),
    ];
    for test_case in test_cases {
        let response = app.post_login(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = setup!();

    let test_cases = [serde_json::json!({
        "email": get_random_email(),
        "password": "EEEEEEEEEEE",
    })];
    for test_case in test_cases {
        let response = app.post_login(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
    }
}
