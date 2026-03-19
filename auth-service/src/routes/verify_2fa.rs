use crate::app_state::AppState;
use crate::domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode};
use crate::utils::auth;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Verify2FARequest {
    pub email: Email,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: LoginAttemptId,
    #[serde(rename = "2FACode")]
    pub two_fa_code: TwoFACode,
}

// use axum::extract::rejection::JsonRejection;
// pub async fn verify_2fa_handler_with_json_rejection(
//     request: Result<Json<Verify2FARequest>, JsonRejection>,
// ) -> impl IntoResponse {
//     match request {
//         Ok(_) => StatusCode::OK.into_response(),
//         Err(JsonRejection::JsonDataError(e)) => {
//             eprintln!("Json error : {e:?}");
//             StatusCode::UNPROCESSABLE_ENTITY.into_response()
//         }
//         Err(_) => StatusCode::UNPROCESSABLE_ENTITY.into_response(),
//     }
// }

pub async fn verify_2fa_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let code_from_store = {
        let guard = state.two_fa_code_store.read().await;
        guard.get_code(&request.email).await.map_err(|e| {
            eprintln!("Error in getting code from store {e:?}");
            AuthAPIError::IncorrectCredentials
        })
    };

    match code_from_store {
        Ok((stored_login_attempt_id, stored_code)) => {
            if stored_login_attempt_id == request.login_attempt_id
                && stored_code == request.two_fa_code
            {
                {
                    let mut guard = state.two_fa_code_store.write().await;
                    if let Err(e) = guard.remove_code(&request.email).await {
                        eprintln!("Error in removing code from store {e:?}");
                        return (jar, Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()));
                    }
                }
                match auth::generate_auth_cookie(&request.email) {
                    Ok(auth_cookie) => {
                        let updated_jar = jar.add(auth_cookie);
                        (updated_jar, Ok(StatusCode::OK.into_response()))
                    }
                    Err(e) => {
                        eprintln!("Error: {e:?}");
                        (jar, Err(AuthAPIError::UnexpectedError))
                    }
                }
            } else {
                (jar, Err(AuthAPIError::IncorrectCredentials))
            }
        }
        Err(e) => (jar, Err(e)),
    }
}
