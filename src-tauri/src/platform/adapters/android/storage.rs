use super::backend::{KeyStoreBackend, MemoryKeyStoreBackend};
use crate::platform::storage::{SecretIdentifier, SecureStoragePlatform};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WrappedSecret {
    pub version: u32,
    pub algorithm: String,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub tag: Vec<u8>,
}

impl WrappedSecret {
    pub fn validate(&self) -> Result<(), super::backend::KeyStoreError> {
        if self.version != 1 {
            return Err(super::backend::KeyStoreError::InvalidVersion(self.version));
        }
        if self.algorithm != "AES-GCM-NoPadding" {
            return Err(super::backend::KeyStoreError::InvalidSecretFormat);
        }
        if self.nonce.len() != 12 {
            return Err(super::backend::KeyStoreError::InvalidSecretFormat);
        }
        if self.tag.len() != 16 {
            return Err(super::backend::KeyStoreError::InvalidSecretFormat);
        }
        Ok(())
    }
}

pub struct AndroidSecureStoragePlatform {
    backend: Arc<dyn KeyStoreBackend>,
    simulated_store: Arc<tokio::sync::Mutex<HashMap<String, WrappedSecret>>>,
}

impl AndroidSecureStoragePlatform {
    pub fn new(backend: Arc<dyn KeyStoreBackend>) -> Self {
        Self {
            backend,
            simulated_store: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
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

/// Storage Access Framework (SAF) Adapter.
/// Isolates content:// URIs from domain core, operating only on encrypted raw byte streams.
pub struct AndroidStorageAdapter {
    saf_documents: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl Default for AndroidStorageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidStorageAdapter {
    pub fn new() -> Self {
        Self {
            saf_documents: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Exports encrypted backup bytes to a target SAF document URI.
    /// Returns clean Ok(()) while keeping SAF content:// resolution strictly inside adapter.
    pub async fn export_backup_archive(
        &self,
        target_saf_uri: &str,
        encrypted_bytes: &[u8],
    ) -> Result<(), String> {
        if !target_saf_uri.starts_with("content://") && !target_saf_uri.starts_with("file://") {
            return Err("Invalid SAF URI format".to_string());
        }

        let mut map = self.saf_documents.lock().await;
        map.insert(target_saf_uri.to_string(), encrypted_bytes.to_vec());
        tracing::info!(
            "[AndroidStorageAdapter] Exported {} bytes to SAF URI: {}",
            encrypted_bytes.len(),
            target_saf_uri
        );
        Ok(())
    }

    /// Imports encrypted backup archive bytes from a source SAF document URI.
    /// Translates content:// URI into plain byte stream for domain core.
    pub async fn import_backup_archive(&self, source_saf_uri: &str) -> Result<Vec<u8>, String> {
        if !source_saf_uri.starts_with("content://") && !source_saf_uri.starts_with("file://") {
            return Err("Invalid SAF URI format".to_string());
        }

        let map = self.saf_documents.lock().await;
        if let Some(bytes) = map.get(source_saf_uri) {
            tracing::info!(
                "[AndroidStorageAdapter] Imported {} bytes from SAF URI: {}",
                bytes.len(),
                source_saf_uri
            );
            Ok(bytes.clone())
        } else {
            Err(format!("SAF Document '{}' not found", source_saf_uri))
        }
    }

    pub async fn contains_saf_uri(&self, saf_uri: &str) -> bool {
        let map = self.saf_documents.lock().await;
        map.contains_key(saf_uri)
    }
}
