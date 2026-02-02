use super::UserInfoError;
use validator::Validate;

#[derive(Clone, PartialEq, Validate, Hash, Eq)]
pub struct Email {
    #[validate(email)]
    e: String,
}

impl Email {
    pub fn new(email: &str) -> Result<Self, UserInfoError> {
        let maybe_email = Self {
            e: email.to_owned(),
        };

        if let Err(_e) = maybe_email.validate() {
            Err(UserInfoError::InvalidEmail)
        } else {
            Ok(maybe_email)
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.e
    }
}

#[derive(Clone, PartialEq, Validate)]
pub struct Password {
    #[validate(length(min = 8, max = 30))]
    p: String,
}

impl Password {
    pub fn new(password: &str) -> Result<Self, UserInfoError> {
        let maybe_password = Self {
            p: password.to_owned(),
        };

        if let Err(_e) = maybe_password.validate() {
            Err(UserInfoError::InvalidPassword)
        } else {
            Ok(maybe_password)
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.p
    }
}

#[derive(Clone)]
pub struct User {
    pub email: Email,
    password: Password,
    requires_2fa: bool,
}

impl User {
    pub fn new(
        email: Email,
        password: Password,
        requires_2fa: bool,
    ) -> Result<Self, UserInfoError> {
        Ok(Self {
            email,
            password,
            requires_2fa,
        })
    }

    pub fn validate(&self, email: &Email, password: &Password) -> bool {
        self.email == *email && self.password == *password
    }
}
