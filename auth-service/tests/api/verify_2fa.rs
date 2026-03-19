use crate::helpers::{self, TestApp};
use auth_service::{
    domain::{Email, LoginAttemptId},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = helpers::get_random_email();

    let test_case = serde_json::json!({
        "email": random_email
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 422);

    let app = TestApp::new().await;
    let random_email = helpers::get_random_email();

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": "bad-attempt-id",
        "2FACode": "123456"
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 422);

    let app = TestApp::new().await;
    let random_email = helpers::get_random_email();

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": LoginAttemptId::default(),
        "2FACode": "1234"
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
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

    assert_eq!(response.status().as_u16(), 206);
    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": LoginAttemptId::default(),
        "2FACode": "123456"
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 401);

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": json_body.login_attempt_id,
        "2FACode": "123456"
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
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
    // capture the incorrect 2fa info
    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    let (_stored_login_attempt_id, stored_code) = {
        let guard = app.two_fa_code_store.read().await;
        guard
            .get_code(&email)
            .await
            .expect("login attempt id not found in store")
    };

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&test_case).await;
    assert_eq!(response.status().as_u16(), 206);

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": json_body.login_attempt_id,
        "2FACode": stored_code
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
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

    let (_stored_login_attempt_id, stored_code) = {
        let guard = app.two_fa_code_store.read().await;
        guard
            .get_code(&email)
            .await
            .expect("login attempt id not found in store")
    };

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": json_body.login_attempt_id,
        "2FACode": stored_code
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 200);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
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
    let (_stored_login_attempt_id, stored_code) = {
        let guard = app.two_fa_code_store.read().await;
        guard
            .get_code(&email)
            .await
            .expect("login attempt id not found in store")
    };

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": json_body.login_attempt_id,
        "2FACode": stored_code
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 200);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 401);
}
