use crate::domain::AuthAPIError;
use crate::utils::auth;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token_handler(
    Json(info): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthAPIError> {
    auth::validate_token(&info.token).await.map_err(|e| {
        eprintln!("Error: {e:?}");
        AuthAPIError::InvalidToken
    })?;
    Ok(StatusCode::OK)
}
