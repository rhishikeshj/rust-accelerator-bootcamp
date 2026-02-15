use super::UserStore;
use crate::{
    domain::{Email, Password, User},
    services::UserStoreError,
};
use std::collections::{hash_map::Entry, HashMap};

#[derive(Default)]
pub struct HashMapUserStore {
    // email -> User
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email.clone()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(user);
                Ok(())
            }
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        self.users
            .get(email) // Option<&User>
            .map(|user| {
                if !user.validate(email, password) {
                    Err(UserStoreError::InvalidCredentials)
                } else {
                    Ok(())
                }
            })
            .ok_or(UserStoreError::UserNotFound)
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{Email, Password};

    use super::*;

    #[tokio::test]
    async fn test_add_user() -> Result<(), UserStoreError> {
        let mut store = HashMapUserStore::default();
        store
            .add_user(
                User::new(
                    Email::new("rhi@artis.works").unwrap(),
                    Password::new("password123").unwrap(),
                    false,
                )
                .unwrap(),
            )
            .await?;

        assert!(store
            .add_user(
                User::new(
                    Email::new("rhi@artis.works").unwrap(),
                    Password::new("password123").unwrap(),
                    true
                )
                .unwrap()
            )
            .await
            .is_err_and(|e| matches!(e, UserStoreError::UserAlreadyExists)));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user() -> Result<(), UserStoreError> {
        let mut store = HashMapUserStore::default();
        assert!(store
            .get_user(&Email::new("rhi@artis.works").unwrap())
            .await
            .is_err_and(|e| matches!(e, UserStoreError::UserNotFound)));

        store
            .add_user(
                User::new(
                    Email::new("rhi@artis.works").unwrap(),
                    Password::new("password123").unwrap(),
                    false,
                )
                .unwrap(),
            )
            .await?;
        assert!(store
            .get_user(&Email::new("rhi@artis.works").unwrap())
            .await
            .is_ok_and(|u| u.email == Email::new("rhi@artis.works").unwrap()));

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_user() -> Result<(), UserStoreError> {
        let mut store = HashMapUserStore::default();
        store
            .add_user(
                User::new(
                    Email::new("rhi@artis.works").unwrap(),
                    Password::new("password123").unwrap(),
                    false,
                )
                .unwrap(),
            )
            .await?;

        assert!(store
            .validate_user(
                &Email::new("rhi@artis.works").unwrap(),
                &Password::new("password123").unwrap()
            )
            .await
            .is_ok());
        assert!(store
            .validate_user(
                &Email::new("nouser@artis.works").unwrap(),
                &Password::new("password123").unwrap()
            )
            .await
            .is_err_and(|e| matches!(e, UserStoreError::UserNotFound)));

        assert!(store
            .validate_user(
                &Email::new("rhi@artis.works").unwrap(),
                &Password::new("bad-pass-123").unwrap()
            )
            .await
            .is_err_and(|e| matches!(e, UserStoreError::InvalidCredentials)));

        Ok(())
    }
}
