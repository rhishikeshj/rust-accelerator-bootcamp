pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Debug)]
pub enum UserInfoError {
    InvalidEmail,
    InvalidPassword,
}
