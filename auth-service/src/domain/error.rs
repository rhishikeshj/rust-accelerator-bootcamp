pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
}

pub enum UserInfoError {
    InvalidEmail,
    InvalidPassword,
}
