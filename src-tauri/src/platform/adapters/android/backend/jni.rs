use super::{KeyStoreBackend, KeyStoreError};
use crate::platform::adapters::android::storage::WrappedSecret;
use async_trait::async_trait;

pub struct JniKeyStoreBackend {
    pub initialized: bool,
}

impl JniKeyStoreBackend {
    pub fn new() -> Self {
        Self { initialized: false }
    }
}

impl Default for JniKeyStoreBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl KeyStoreBackend for JniKeyStoreBackend {
    async fn wrap_key(&self, _plaintext: &[u8]) -> Result<WrappedSecret, KeyStoreError> {
        Err(KeyStoreError::BackendUnavailable)
    }

    async fn unwrap_key(&self, _secret: &WrappedSecret) -> Result<Vec<u8>, KeyStoreError> {
        Err(KeyStoreError::BackendUnavailable)
    }
}
