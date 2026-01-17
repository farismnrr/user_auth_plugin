use crate::domains::common::{errors::AppError, infrastructures::rocksdb_connection::RocksDbCache};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

#[async_trait]
pub trait InvitationCodeRepositoryTrait: Send + Sync {
    async fn save_code(&self, code: String, ttl: Duration) -> Result<(), AppError>;
    async fn validate_and_delete_code(&self, code: &str) -> Result<bool, AppError>;
}

pub struct InvitationCodeRepository {
    cache: Arc<RocksDbCache>,
}

impl InvitationCodeRepository {
    pub fn new(cache: Arc<RocksDbCache>) -> Self {
        Self { cache }
    }
}

#[async_trait]
impl InvitationCodeRepositoryTrait for InvitationCodeRepository {
    async fn save_code(&self, code: String, ttl: Duration) -> Result<(), AppError> {
        let key = format!("invite:{}", code);
        // We store the code itself as value, or just "valid"
        self.cache.set(&key, "valid".to_string(), ttl);
        Ok(())
    }

    async fn validate_and_delete_code(&self, code: &str) -> Result<bool, AppError> {
        let key = format!("invite:{}", code);

        // Check if exists
        let exists: Option<String> = self.cache.get(&key);

        if exists.is_some() {
            // If exists, delete it (one-time use)
            self.cache.del(&key);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
