use crate::domain::{Email, Password, User};
use rand;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

//  ──────────────────────────── Banned token store ────────────────────────────

#[derive(Debug, PartialEq)]
pub enum TokenStoreError {
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn ban(&mut self, token: &str) -> Result<(), TokenStoreError>;
    async fn is_banned(&self, token: &str) -> Result<bool, TokenStoreError>;
}

//  ────────────────────────────── 2FA code store ──────────────────────────────

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(Uuid);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        match Uuid::parse_str(&id) {
            Ok(id) => Ok(LoginAttemptId(id)),
            Err(e) => {
                eprintln!("Error parsing login-attempt-id {e:?}");
                Err(format!("{e:?}"))
            }
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl AsRef<Uuid> for LoginAttemptId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        let code_num = code.parse::<i32>();
        match code_num {
            Ok(n) if n >= 100000 && n <= 999999 => Ok(Self(code)),
            Ok(n) => Err(format!("Bad 2FA code {n}")),
            Err(e) => Err(format!("Cannot parse 2FA code {e:?}")),
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let code = rand::random_range(100000..=999999);
        Self(code.to_string())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}
