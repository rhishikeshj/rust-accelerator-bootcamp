use super::UserStore;
use crate::{domain::User, services::UserStoreError};
use std::collections::{hash_map::Entry, HashMap};

#[derive(Default)]
pub struct HashMapUserStore {
    // email -> User
    users: HashMap<String, User>,
}

impl UserStore for HashMapUserStore {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email.clone()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(user);
                Ok(())
            }
        }
    }

    fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
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
    use super::*;

    #[tokio::test]
    async fn test_add_user() -> Result<(), UserStoreError> {
        let mut store = HashMapUserStore::default();
        store.add_user(User::new("rhi@artis.works", "pass", false))?;

        assert!(store
            .add_user(User::new("rhi@artis.works", "pass", true))
            .is_err_and(|e| matches!(e, UserStoreError::UserAlreadyExists)));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user() -> Result<(), UserStoreError> {
        let mut store = HashMapUserStore::default();
        assert!(store
            .get_user("rhi@artis.works")
            .is_err_and(|e| matches!(e, UserStoreError::UserNotFound)));

        let _ = store.add_user(User::new("rhi@artis.works", "pass", false))?;
        assert!(store
            .get_user("rhi@artis.works")
            .is_ok_and(|u| u.email == "rhi@artis.works"));

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_user() -> Result<(), UserStoreError> {
        let mut store = HashMapUserStore::default();
        store.add_user(User::new("rhi@artis.works", "pass", false))?;

        assert!(store.validate_user("rhi@artis.works", "pass").is_ok());
        assert!(store
            .validate_user("nouser@artis.works", "pass")
            .is_err_and(|e| matches!(e, UserStoreError::UserNotFound)));

        assert!(store
            .validate_user("rhi@artis.works", "bad-pass")
            .is_err_and(|e| matches!(e, UserStoreError::InvalidCredentials)));

        Ok(())
    }
}
