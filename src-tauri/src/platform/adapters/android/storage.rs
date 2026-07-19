use super::backend::{KeyStoreBackend, MemoryKeyStoreBackend};
use crate::platform::storage::{SecretIdentifier, SecureStoragePlatform};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WrappedSecret {
    pub version: u32,
    pub algorithm: String,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub tag: Vec<u8>,
}

pub struct AndroidSecureStoragePlatform {
    backend: Arc<dyn KeyStoreBackend>,
    simulated_store: Arc<tokio::sync::Mutex<std::collections::HashMap<String, WrappedSecret>>>,
}

impl AndroidSecureStoragePlatform {
    pub fn new(backend: Arc<dyn KeyStoreBackend>) -> Self {
        Self {
            backend,
            simulated_store: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }

    fn make_key(&self, id: &SecretIdentifier) -> String {
        format!("{}:{}", id.namespace, id.kind.as_str())
    }
}

impl Default for AndroidSecureStoragePlatform {
    fn default() -> Self {
        // По умолчанию на Desktop/Tests используем Memory-симуляцию
        Self::new(Arc::new(MemoryKeyStoreBackend::new()))
    }
}

#[async_trait]
impl SecureStoragePlatform for AndroidSecureStoragePlatform {
    async fn store(&self, id: SecretIdentifier, value: &[u8]) -> Result<(), String> {
        let key = self.make_key(&id);

        let wrapped = self
            .backend
            .wrap_key(value)
            .await
            .map_err(|e| e.to_string())?;

        let mut map = self.simulated_store.lock().await;
        map.insert(key, wrapped);
        Ok(())
    }

    async fn load(&self, id: SecretIdentifier) -> Result<Option<Vec<u8>>, String> {
        let key = self.make_key(&id);
        let map = self.simulated_store.lock().await;

        if let Some(wrapped) = map.get(&key) {
            let unwrapped = self
                .backend
                .unwrap_key(wrapped)
                .await
                .map_err(|e| e.to_string())?;
            Ok(Some(unwrapped))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: SecretIdentifier) -> Result<(), String> {
        let key = self.make_key(&id);
        let mut map = self.simulated_store.lock().await;
        map.remove(&key);
        Ok(())
    }
}
