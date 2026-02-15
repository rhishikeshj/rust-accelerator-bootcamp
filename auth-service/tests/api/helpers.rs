use auth_service::{
    app_state::AppState,
    services::{BannedTokenStore, HashMapUserStore, HashsetBannedTokenStore},
    utils::constants::test,
    Application,
};
use reqwest::cookie::Jar;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub cookies: Arc<Jar>,
    pub banned_token_store: Arc<RwLock<dyn BannedTokenStore>>,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = HashMapUserStore::default();
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let app_state = AppState::new(
            Arc::new(RwLock::new(user_store)),
            banned_token_store.clone(),
        );

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookies = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookies.clone())
            .build()
            .unwrap();

        // Create new `TestApp` instance and return it
        Self {
            cookies,
            address,
            http_client,
            banned_token_store,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

pub fn get_badly_formatted_email() -> String {
    format!("{}-example.com", Uuid::new_v4())
}
