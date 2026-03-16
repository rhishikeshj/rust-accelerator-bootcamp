use crate::helpers::{self, TestApp};
use auth_service::{
    domain::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME,
};

#[tokio::test]
async fn login_works() {
    let app = TestApp::new().await;

    let random_email = helpers::get_random_email();

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
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

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
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
        "requires2FA": false
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

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email = helpers::get_random_email();
    let email = Email::new(&random_email).unwrap();

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

    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let guard = app.two_fa_code_store.read().await;
    let (stored_code, _) = guard
        .get_code(&email)
        .await
        .expect("login attempt id not found in store");

    assert_eq!(
        json_body.login_attempt_id,
        stored_code.as_ref().to_string(),
        "Login attempt id not stored"
    );
}
