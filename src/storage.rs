//! Storage implementation for chain data

use crate::error::Result;
use async_trait::async_trait;
use std::collections::HashMap;

/// Trait for key-value storage backends
#[async_trait]
pub trait Storage: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn put(&self, key: &str, value: Vec<u8>) -> Result<()>;
    async fn delete(&self, key: &str) -> Result<()>;
    async fn exists(&self, key: &str) -> Result<bool>;
    async fn clear(&self) -> Result<()>;
    async fn initialize(&self) -> Result<()>;
}

/// In-memory storage implementation
#[derive(Debug, Default)]
pub struct MemoryStorage {
    data: tokio::sync::RwLock<HashMap<String, Vec<u8>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let data = self.data.read().await;
        Ok(data.get(key).cloned())
    }

    async fn put(&self, key: &str, value: Vec<u8>) -> Result<()> {
        let mut data = self.data.write().await;
        data.insert(key.to_string(), value);
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let mut data = self.data.write().await;
        data.remove(key);
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        let data = self.data.read().await;
        Ok(data.contains_key(key))
    }

    async fn clear(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.clear();
        Ok(())
    }

    async fn initialize(&self) -> Result<()> {
        // In-memory storage doesn't need initialization
        Ok(())
    }
}
