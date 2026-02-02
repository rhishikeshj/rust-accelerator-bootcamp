use super::UserInfoError;

#[derive(Clone)]
pub struct User {
    pub email: String,
    password: String,
    requires_2fa: bool,
}

impl User {
    pub fn new(email: &str, password: &str, requires_2fa: bool) -> Result<Self, UserInfoError> {
        if email.is_empty() || !email.contains("@") {
            Err(UserInfoError::InvalidEmail)
        } else if password.is_empty() {
            Err(UserInfoError::InvalidPassword)
        } else {
            Ok(Self {
                email: email.to_owned(),
                password: password.to_owned(),
                requires_2fa,
            })
        }
    }

    pub fn validate(&self, email: &str, password: &str) -> bool {
        self.email == email && self.password == password
    }
}
