use std::fmt;

pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    UnexpectedError,
}

#[derive(Debug)]
pub enum UserInfoError {
    InvalidEmail,
    InvalidPassword,
}

impl fmt::Display for UserInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
