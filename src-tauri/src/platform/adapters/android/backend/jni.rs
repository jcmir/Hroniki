use std::sync::Arc;
use async_trait::async_trait;
use super::{KeyStoreBackend, KeyStoreError, KeyStoreState, KeyStoreJniBridge};
use crate::platform::adapters::android::storage::WrappedSecret;

pub struct JniKeyStoreBackend {
    state: tokio::sync::RwLock<KeyStoreState>,
    bridge: Arc<dyn KeyStoreJniBridge>,
}

impl JniKeyStoreBackend {
    pub fn new(bridge: Arc<dyn KeyStoreJniBridge>) -> Self {
        Self {
            state: tokio::sync::RwLock::new(KeyStoreState::Uninitialized),
            bridge,
        }
    }

    pub async fn set_state(&self, new_state: KeyStoreState) {
        let mut w = self.state.write().await;
        *w = new_state;
    }

    pub async fn get_state(&self) -> KeyStoreState {
        let r = self.state.read().await;
        r.clone()
    }
}

#[async_trait]
impl KeyStoreBackend for JniKeyStoreBackend {
    async fn wrap_key(&self, plaintext: &[u8]) -> Result<WrappedSecret, KeyStoreError> {
        let state = self.get_state().await;
        if state != KeyStoreState::Ready {
            return Err(KeyStoreError::BackendUnavailable);
        }

        self.bridge.encrypt(plaintext).await
    }

    async fn unwrap_key(&self, secret: &WrappedSecret) -> Result<Vec<u8>, KeyStoreError> {
        secret.validate()?;

        let state = self.get_state().await;
        if state != KeyStoreState::Ready {
            return Err(KeyStoreError::BackendUnavailable);
        }

        self.bridge.decrypt(secret).await
    }
}
