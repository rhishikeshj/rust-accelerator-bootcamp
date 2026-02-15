use crate::helpers::TestApp;
use auth_service::{
    domain::Email,
    utils::{auth, constants::JWT_COOKIE_NAME},
};
use reqwest::Url;

#[tokio::test]
async fn logout_works() {
    let app = TestApp::new().await;

    // add cookie
    let email = Email::new("rhi@artis.works").unwrap();
    let cookie = auth::generate_auth_cookie(&email).unwrap();
    app.cookies.add_cookie_str(
        &format!("{cookie}; HttpOnly; SameSite=Lax; Secure; Path=/"),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_should_ban_token() {
    let app = TestApp::new().await;

    // add cookie
    let email = Email::new("rhi@artis.works").unwrap();
    let cookie = auth::generate_auth_cookie(&email).unwrap();
    let token = cookie.value();
    app.cookies.add_cookie_str(
        &format!("{cookie}; HttpOnly; SameSite=Lax; Secure; Path=/"),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    {
        let banned_token_store = app.banned_token_store.read().await;
        assert!(banned_token_store.is_banned(token).await.unwrap())
    }
}

#[tokio::test]
async fn logout_called_twice_does_not_work() {
    let app = TestApp::new().await;

    // add cookie
    let email = Email::new("rhi@artis.works").unwrap();
    let cookie = auth::generate_auth_cookie(&email).unwrap();
    app.cookies.add_cookie_str(
        &format!("{cookie}; HttpOnly; SameSite=Lax; Secure; Path=/"),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookies.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401);
}
