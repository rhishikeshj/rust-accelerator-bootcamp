use crate::helpers::{self, TestApp};

#[tokio::test]
async fn login_works() {
    let app = TestApp::new().await;

    let random_email = helpers::get_random_email();

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&test_case).await;
    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed for input: {:?}",
        test_case
    );

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&test_case).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_fails_on_malformed_input() {
    let app = TestApp::new().await;

    let test_case = serde_json::json!({
        "email": "bad"
    });

    let response = app.post_login(&test_case).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn login_fails_on_incorrect_credentials() {
    let app = TestApp::new().await;

    let random_email = helpers::get_random_email();

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&test_case).await;
    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed for input: {:?}",
        test_case
    );

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "bad_password",
    });

    let response = app.post_login(&test_case).await;

    assert_eq!(response.status().as_u16(), 401);
}
