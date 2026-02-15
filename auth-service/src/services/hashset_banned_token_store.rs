use std::collections::HashSet;

use super::banned_token_store::{BannedTokenStore, TokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn ban(&mut self, token: &str) -> Result<(), TokenStoreError> {
        self.tokens.insert(token.to_owned());
        Ok(())
    }

    async fn is_banned(&self, token: &str) -> Result<bool, TokenStoreError> {
        Ok(self.tokens.contains(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ban_user() -> Result<(), TokenStoreError> {
        let mut store = HashsetBannedTokenStore::default();
        store.ban("banned").await?;

        assert!(store.is_banned("banned").await.unwrap());
        Ok(())
    }
}
