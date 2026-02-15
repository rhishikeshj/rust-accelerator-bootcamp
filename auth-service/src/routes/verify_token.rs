use crate::app_state::AppState;
use crate::domain::AuthAPIError;
use crate::utils::auth;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token_handler(
    State(state): State<AppState>,
    Json(info): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthAPIError> {
    auth::validate_token(&state.banned_token_store, &info.token)
        .await
        .map_err(|e| {
            eprintln!("Error: {e:?}");
            AuthAPIError::InvalidToken
        })?;
    Ok(StatusCode::OK)
}
