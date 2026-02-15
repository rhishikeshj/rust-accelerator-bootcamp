#[derive(Debug, PartialEq)]
pub enum TokenStoreError {
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn ban(&mut self, token: &str) -> Result<(), TokenStoreError>;
    async fn is_banned(&self, token: &str) -> Result<bool, TokenStoreError>;
}
