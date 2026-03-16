use axum::http::StatusCode;
use axum::{extract::State, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::domain::{LoginAttemptId, TwoFACode};
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

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth(String),
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

#[axum::debug_handler]
pub async fn login_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(info): Json<LoginRequest>,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let user_store = state.user_store.read().await;
    if let Err(e) = user_store.validate_user(&info.email, &info.password).await {
        eprintln!("Error: {e:?}");
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    };

    let user = match user_store.get_user(&info.email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };
    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    // First, we must generate a new random login attempt ID and 2FA code
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    {
        let mut guard = state.two_fa_code_store.write().await;
        if let Err(e) = guard
            .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
            .await
        {
            eprintln!("Error in storing 2fa code {e:?}");
            return (jar, Err(AuthAPIError::UnexpectedError));
        }
    }

    if let Err(e) = state
        .email_client
        .send_email(
            &email,
            "Your 2FA code for auth-service login",
            &format!("Your 2FA code is {two_fa_code:?}"),
        )
        .await
    {
        eprintln!("Error in sending 2fa code {e:?}");
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    // Finally, we need to return the login attempt ID to the client
    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_string(),
    }));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    match auth::generate_auth_cookie(email) {
        Ok(auth_cookie) => {
            let updated_jar = jar.add(auth_cookie);
            (
                updated_jar,
                Ok((
                    StatusCode::OK,
                    Json(LoginResponse::RegularAuth("User logged in".to_string())),
                )),
            )
        }
        Err(e) => {
            eprintln!("Error: {e:?}");
            return (jar, Err(AuthAPIError::UnexpectedError));
        }
    }
}
