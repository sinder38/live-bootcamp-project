/// Sets up a test application with a random email and password, and signs up the user.
#[macro_export]
macro_rules! signup {
    () => {
 {

     let app = TestApp::new().await;
     let random_email = get_random_email();

     let signup_body = serde_json::json!({
         "email": random_email,
         "password": "password123",
         "requires2FA": false
     });

     let response = app.post_signup(&signup_body).await;

     assert_eq!(response.status().as_u16(), 201);
     app
 }
    };
}

#[macro_export]
macro_rules! signup_and_login {
    () => {
        {

        let app = TestApp::new().await;
        let random_email = get_random_email();

        let signup_body = serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": false
        });

        let response = app.post_signup(&signup_body).await;

        assert_eq!(response.status().as_u16(), 201);

        let login_body = serde_json::json!({
            "email": random_email,
            "password": "password123",
        });

        let response = app.post_login(&login_body).await;

        assert_eq!(response.status().as_u16(), 200);
        (app, response)

        }
    };
}
