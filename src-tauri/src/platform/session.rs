use crate::events::{DomainEvent, EventBus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

pub struct SessionManager {
    tokens: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    decrypted_cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    is_locked: Arc<Mutex<bool>>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
            decrypted_cache: Arc::new(Mutex::new(HashMap::new())),
            is_locked: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start_event_listener(self: &Arc<Self>, event_bus: &Arc<EventBus>) {
        let mut rx = event_bus.subscribe();
        let manager = self.clone();

        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(DomainEvent::ApplicationLocked) => {
                        manager.clear_ram_session().await;
                    }
                    Ok(_) => {}
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                }
            }
        });
    }

    pub async fn set_token(&self, key: impl Into<String>, value: Vec<u8>) {
        let mut tokens = self.tokens.lock().await;
        tokens.insert(key.into(), value);
    }

    pub async fn get_token(&self, key: &str) -> Option<Vec<u8>> {
        let tokens = self.tokens.lock().await;
        tokens.get(key).cloned()
    }

    pub async fn set_cache(&self, key: impl Into<String>, value: Vec<u8>) {
        let mut cache = self.decrypted_cache.lock().await;
        cache.insert(key.into(), value);
    }

    pub async fn get_cache(&self, key: &str) -> Option<Vec<u8>> {
        let cache = self.decrypted_cache.lock().await;
        cache.get(key).cloned()
    }

    pub async fn is_locked(&self) -> bool {
        *self.is_locked.lock().await
    }

    /// Clears only RAM tokens & decrypted caches upon ApplicationLocked.
    /// Disk database and KeyStore secrets remain untouched.
    pub async fn clear_ram_session(&self) {
        let mut tokens = self.tokens.lock().await;
        let mut cache = self.decrypted_cache.lock().await;
        let mut locked = self.is_locked.lock().await;

        tokens.clear();
        cache.clear();
        *locked = true;

        tracing::info!(
            "[SessionManager] Cleared RAM tokens and decrypted cache due to ApplicationLocked event."
        );
    }

    pub async fn unlock_session(&self) {
        let mut locked = self.is_locked.lock().await;
        *locked = false;
    }
}
