use crate::platform::storage::{SecretIdentifier, SecureStoragePlatform};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WrappedSecret {
    pub version: u32,
    pub algorithm: String,
    pub ciphertext: Vec<u8>,
}

pub struct AndroidSecureStoragePlatform {
    // В будущем здесь будет находиться Mutex/JNI-окружение
    simulated_store:
        std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, WrappedSecret>>>,
}

impl AndroidSecureStoragePlatform {
    pub fn new() -> Self {
        Self {
            simulated_store: std::sync::Arc::new(tokio::sync::Mutex::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    fn make_key(&self, id: &SecretIdentifier) -> String {
        format!("{}:{}", id.namespace, id.kind.as_str())
    }
}

impl Default for AndroidSecureStoragePlatform {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SecureStoragePlatform for AndroidSecureStoragePlatform {
    async fn store(&self, id: SecretIdentifier, value: &[u8]) -> Result<(), String> {
        let key = self.make_key(&id);

        // Симулируем "Key Wrapping" с версионированием
        let wrapped = WrappedSecret {
            version: 1,
            algorithm: "AES-GCM-NoPadding".to_string(),
            ciphertext: value.to_vec(), // В реальном Android здесь будет зашифрованный байт-массив
        };

        let mut map = self.simulated_store.lock().await;
        map.insert(key, wrapped);
        Ok(())
    }

    async fn load(&self, id: SecretIdentifier) -> Result<Option<Vec<u8>>, String> {
        let key = self.make_key(&id);
        let map = self.simulated_store.lock().await;

        if let Some(wrapped) = map.get(&key) {
            // Проверка версии перед дешифрованием
            if wrapped.version != 1 {
                return Err("Unsupported wrapped secret version".to_string());
            }
            Ok(Some(wrapped.ciphertext.clone()))
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
