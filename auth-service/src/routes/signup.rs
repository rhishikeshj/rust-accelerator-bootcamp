use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, UserInfoError},
};
use axum::http::StatusCode;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl TryFrom<SignupRequest> for User {
    type Error = UserInfoError;

    fn try_from(value: SignupRequest) -> Result<User, Self::Error> {
        Self::new(
            Email::new(&value.email)?,
            Password::new(&value.password)?,
            value.requires_2fa,
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(info): Json<SignupRequest>,
) -> Result<(StatusCode, Json<SignupResponse>), AuthAPIError> {
    let mut user_store = state.user_store.write().await;
    let user: User = info.try_into().map_err(|e| {
        eprintln!("Error: {e:?}");
        AuthAPIError::InvalidCredentials
    })?;

    user_store.add_user(user).await.map_err(|e| match e {
        crate::services::UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
        crate::services::UserStoreError::UserNotFound
        | crate::services::UserStoreError::InvalidCredentials
        | crate::services::UserStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
    })?;

    Ok((
        StatusCode::CREATED,
        Json(SignupResponse {
            message: "User created successfully!".to_string(),
        }),
    ))
}
