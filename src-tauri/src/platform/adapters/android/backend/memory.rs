use super::{KeyStoreBackend, KeyStoreError};
use crate::platform::adapters::android::storage::WrappedSecret;
use async_trait::async_trait;

pub struct MemoryKeyStoreBackend;

impl MemoryKeyStoreBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MemoryKeyStoreBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl KeyStoreBackend for MemoryKeyStoreBackend {
    async fn wrap_key(&self, plaintext: &[u8]) -> Result<WrappedSecret, KeyStoreError> {
        let mut nonce = vec![0u8; 12];
        getrandom::getrandom(&mut nonce).map_err(|e| {
            KeyStoreError::EncryptionFailed(format!("Failed to generate CSPRNG nonce: {}", e))
        })?;

        // Симулируем XOR-шифрование с использованием nonce для уникальности ciphertext
        let ciphertext: Vec<u8> = plaintext
            .iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ nonce[i % nonce.len()])
            .collect();

        Ok(WrappedSecret {
            version: 1,
            algorithm: "AES-GCM-NoPadding".to_string(),
            nonce,
            ciphertext,
        })
    }

    async fn unwrap_key(&self, secret: &WrappedSecret) -> Result<Vec<u8>, KeyStoreError> {
        if secret.version != 1 {
            return Err(KeyStoreError::InvalidVersion(secret.version));
        }

        // Обратная операция XOR-дешифрования с nonce
        let plaintext: Vec<u8> = secret
            .ciphertext
            .iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ secret.nonce[i % secret.nonce.len()])
            .collect();

        Ok(plaintext)
    }
}
