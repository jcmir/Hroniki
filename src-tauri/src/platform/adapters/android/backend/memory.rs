use super::{KeyStoreBackend, KeyStoreError};
use crate::platform::adapters::android::storage::WrappedSecret;
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use async_trait::async_trait;

pub struct MemoryKeyStoreBackend {
    master_key: [u8; 32],
}

impl MemoryKeyStoreBackend {
    pub fn new() -> Self {
        let mut master_key = [0u8; 32];
        getrandom::getrandom(&mut master_key).expect("Failed to initialize Master Key");
        Self { master_key }
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
        let mut nonce_bytes = [0u8; 12];
        getrandom::getrandom(&mut nonce_bytes).map_err(|_| KeyStoreError::EncryptionFailed)?;

        let cipher = Aes256Gcm::new_from_slice(&self.master_key)
            .map_err(|_| KeyStoreError::EncryptionFailed)?;

        let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);

        let encrypted = cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| KeyStoreError::EncryptionFailed)?;

        // В aes-gcm результат шифрования содержит ciphertext + 16-байтный tag в конце
        if encrypted.len() < 16 {
            return Err(KeyStoreError::EncryptionFailed);
        }

        let mut ciphertext = encrypted;
        let tag = ciphertext.split_off(ciphertext.len() - 16);

        Ok(WrappedSecret {
            version: 1,
            algorithm: "AES-GCM-NoPadding".to_string(),
            nonce: nonce_bytes.to_vec(),
            ciphertext,
            tag,
        })
    }

    async fn unwrap_key(&self, secret: &WrappedSecret) -> Result<Vec<u8>, KeyStoreError> {
        secret.validate()?;

        let cipher = Aes256Gcm::new_from_slice(&self.master_key)
            .map_err(|_| KeyStoreError::DecryptionFailed)?;

        let nonce = aes_gcm::Nonce::from_slice(&secret.nonce);

        // Собираем обратно ciphertext + tag
        let mut payload = secret.ciphertext.clone();
        payload.extend_from_slice(&secret.tag);

        let decrypted = cipher
            .decrypt(nonce, payload.as_slice())
            .map_err(|_| KeyStoreError::DecryptionFailed)?;

        Ok(decrypted)
    }
}
