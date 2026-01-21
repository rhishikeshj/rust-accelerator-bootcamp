use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

pub async fn signup_handler(Json(info): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}
