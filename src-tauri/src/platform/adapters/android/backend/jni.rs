use super::{KeyStoreBackend, KeyStoreError, KeyStoreJniBridge, KeyStoreState};
use crate::platform::adapters::android::storage::WrappedSecret;
use async_trait::async_trait;
use std::sync::Arc;

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

// Real Android JNI Bridge implementation shell
#[allow(dead_code)]
pub struct RealKeyStoreJniBridge {
    // Stores raw JavaVM pointer safely wrapped for multi-threaded access
    jvm_ptr: *mut std::ffi::c_void,
}

impl RealKeyStoreJniBridge {
    pub fn new(jvm_ptr: *mut std::ffi::c_void) -> Self {
        Self { jvm_ptr }
    }

    /// Isolated helper to check and clear JVM exceptions
    #[allow(dead_code)]
    fn check_java_exception(&self, has_exception: bool) -> Result<(), KeyStoreError> {
        if has_exception {
            // Under real JNI: env.exception_clear()
            tracing::error!("Native JVM exception detected and cleared");
            Err(KeyStoreError::JavaException)
        } else {
            Ok(())
        }
    }
}

unsafe impl Send for RealKeyStoreJniBridge {}
unsafe impl Sync for RealKeyStoreJniBridge {}

#[async_trait]
impl KeyStoreJniBridge for RealKeyStoreJniBridge {
    async fn encrypt(&self, _plaintext: &[u8]) -> Result<WrappedSecret, KeyStoreError> {
        self.check_java_exception(false)?;
        Err(KeyStoreError::BackendUnavailable)
    }

    async fn decrypt(&self, _secret: &WrappedSecret) -> Result<Vec<u8>, KeyStoreError> {
        self.check_java_exception(false)?;
        Err(KeyStoreError::BackendUnavailable)
    }
}
