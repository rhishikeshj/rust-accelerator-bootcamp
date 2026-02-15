use super::UserInfoError;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use validator::Validate;

#[derive(Clone, PartialEq, Validate, Hash, Eq, Debug)]
pub struct Email {
    #[validate(email)]
    e: String,
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let email_str = String::deserialize(deserializer)?;
        // Email::new(&email_str).map_err(|e| {
        //     Error::invalid_value(Unexpected::Str(&format!("{e}")), &"Valid email expected")
        // })

        Email::new(&email_str).map_err(Error::custom)
    }
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

#[derive(Clone, PartialEq, Validate, Debug)]
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

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let e = String::deserialize(deserializer)?;
        Password::new(&e).map_err(Error::custom)
    }
}

#[allow(dead_code)]
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
