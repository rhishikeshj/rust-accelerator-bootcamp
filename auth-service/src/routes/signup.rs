use axum::{extract::State, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl From<SignupRequest> for User {
    fn from(value: SignupRequest) -> Self {
        Self::new(&value.email, &value.password, value.requires_2fa)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(info): Json<SignupRequest>,
) -> Result<(StatusCode, Json<SignupResponse>), (StatusCode, String)> {
    let mut user_store = state.user_store.write().await;
    user_store.add_user(info.into()).map_err(|_e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong!"),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(SignupResponse {
            message: "User created successfully!".to_string(),
        }),
    ))
}
