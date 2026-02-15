use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

use crate::app_state::AppState;
use crate::domain::AuthAPIError;
use crate::utils::auth;
use crate::utils::constants::JWT_COOKIE_NAME;

pub async fn logout_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar), AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;
    let token = cookie.value().to_owned();

    auth::validate_token(&state.banned_token_store, &token)
        .await
        .map_err(|e| {
            eprintln!("Error: {e:?}");
            AuthAPIError::InvalidToken
        })?;

    let updated_jar = jar.remove(Cookie::from("jwt"));
    {
        let mut banned_token_store = state.banned_token_store.write().await;
        banned_token_store
            .ban(&token)
            .await
            .expect("cannot ban token");
    }

    Ok((StatusCode::OK, updated_jar))
}
