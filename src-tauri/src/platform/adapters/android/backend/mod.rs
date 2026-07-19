use crate::platform::adapters::android::storage::WrappedSecret;
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyStoreError {
    InvalidVersion(u32),
    EncryptionFailed(String),
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
            KeyStoreError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
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
