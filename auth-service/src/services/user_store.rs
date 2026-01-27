use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

pub trait UserStore: Send + Sync {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    fn get_user(&self, email: &str) -> Result<User, UserStoreError>;
    fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError>;
}
