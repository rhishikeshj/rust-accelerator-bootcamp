use axum::http::StatusCode;
use axum::{extract::State, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::utils::auth;
use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
};

// Using Email and Password in the LoginRequest
// struct allows us to avoid parsing the email and password
// from strings to types and it even more `parse-dont-validate`
// but we lose out on the ability to return 400 on invalidation
// axum directly returns 422
#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email: Email,
    pub password: Password,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LoginResponse {
    pub message: String,
}

#[axum::debug_handler]
pub async fn login_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(info): Json<LoginRequest>,
) -> Result<(StatusCode, CookieJar, Json<LoginResponse>), AuthAPIError> {
    let user_store = state.user_store.read().await;
    user_store
        .validate_user(&info.email, &info.password)
        .await
        .map_err(|e| {
            eprintln!("Error: {e:?}");
            AuthAPIError::IncorrectCredentials
        })?;

    let auth_cookie = auth::generate_auth_cookie(&info.email).map_err(|e| {
        eprintln!("Error: {e:?}");
        AuthAPIError::UnexpectedError
    })?;
    let updated_jar = jar.add(auth_cookie);
    Ok((
        StatusCode::OK,
        updated_jar,
        Json(LoginResponse {
            message: "User logged in".to_string(),
        }),
    ))
}
