use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::platform::adapters::android::storage::WrappedSecret;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
}

impl EncryptionAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            EncryptionAlgorithm::Aes256Gcm => "AES-GCM-NoPadding",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyStoreError {
    InvalidVersion(u32),
    EncryptionFailed,
    DecryptionFailed,
    BackendUnavailable,
    InvalidSecretFormat,
}

impl std::fmt::Display for KeyStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyStoreError::InvalidVersion(v) => {
                write!(f, "Unsupported wrapped secret version: {}", v)
            }
            KeyStoreError::EncryptionFailed => write!(f, "Encryption failed"),
            KeyStoreError::DecryptionFailed => write!(f, "Decryption failed (auth tag mismatch)"),
            KeyStoreError::BackendUnavailable => write!(f, "KeyStore backend unavailable"),
            KeyStoreError::InvalidSecretFormat => write!(f, "Invalid wrapped secret format"),
        }
    }
}

impl std::error::Error for KeyStoreError {}

#[async_trait]
pub trait KeyStoreBackend: Send + Sync {
    async fn wrap_key(&self, plaintext: &[u8]) -> Result<WrappedSecret, KeyStoreError>;
    async fn unwrap_key(&self, secret: &WrappedSecret) -> Result<Vec<u8>, KeyStoreError>;
}

pub mod jni;
pub mod memory;

pub use jni::JniKeyStoreBackend;
pub use memory::MemoryKeyStoreBackend;
