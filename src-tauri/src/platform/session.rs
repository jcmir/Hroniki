use crate::events::{DomainEvent, EventBus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Active,
    Locked,
    AwaitingUnlock,
}

pub struct SessionManager {
    tokens: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    decrypted_cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    state: Arc<Mutex<SessionState>>,
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
            state: Arc::new(Mutex::new(SessionState::Active)),
        }
    }

    pub fn start_event_listener(self: &Arc<Self>, event_bus: &Arc<EventBus>) {
        let mut rx = event_bus.subscribe();
        let manager = self.clone();
        let event_bus_clone = event_bus.clone();

        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(DomainEvent::ApplicationLocked) => {
                        manager.clear_ram_session().await;
                    }
                    Ok(DomainEvent::ApplicationResumed) => {
                        manager.handle_application_resumed(&event_bus_clone).await;
                    }
                    Ok(_) => {}
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                }
            }
        });
    }

    pub async fn state(&self) -> SessionState {
        *self.state.lock().await
    }

    pub async fn is_locked(&self) -> bool {
        let state = self.state.lock().await;
        *state == SessionState::Locked || *state == SessionState::AwaitingUnlock
    }

    pub async fn is_awaiting_unlock(&self) -> bool {
        *self.state.lock().await == SessionState::AwaitingUnlock
    }

    pub async fn is_active(&self) -> bool {
        *self.state.lock().await == SessionState::Active
    }

    pub async fn set_token(&self, key: impl Into<String>, value: Vec<u8>) {
        if self.is_active().await {
            let mut tokens = self.tokens.lock().await;
            tokens.insert(key.into(), value);
        }
    }

    pub async fn get_token(&self, key: &str) -> Option<Vec<u8>> {
        if self.is_active().await {
            let tokens = self.tokens.lock().await;
            tokens.get(key).cloned()
        } else {
            None
        }
    }

    pub async fn set_cache(&self, key: impl Into<String>, value: Vec<u8>) {
        if self.is_active().await {
            let mut cache = self.decrypted_cache.lock().await;
            cache.insert(key.into(), value);
        }
    }

    pub async fn get_cache(&self, key: &str) -> Option<Vec<u8>> {
        if self.is_active().await {
            let cache = self.decrypted_cache.lock().await;
            cache.get(key).cloned()
        } else {
            None
        }
    }

    /// Clears RAM tokens & decrypted caches upon ApplicationLocked, transitioning state to Locked.
    pub async fn clear_ram_session(&self) {
        let mut tokens = self.tokens.lock().await;
        let mut cache = self.decrypted_cache.lock().await;
        let mut state = self.state.lock().await;

        tokens.clear();
        cache.clear();
        *state = SessionState::Locked;

        tracing::info!("[SessionManager] State -> Locked. Cleared RAM tokens and decrypted cache.");
    }

    /// Handles ApplicationResumed lifecycle event, moving Locked session to AwaitingUnlock.
    pub async fn handle_application_resumed(&self, event_bus: &Arc<EventBus>) {
        let mut state = self.state.lock().await;
        if *state == SessionState::Locked {
            *state = SessionState::AwaitingUnlock;
            tracing::info!(
                "[SessionManager] State -> AwaitingUnlock. Publishing AuthenticationRequired."
            );
            event_bus.publish(DomainEvent::AuthenticationRequired);
        }
    }

    /// Restores active session after successful user authentication.
    pub async fn unlock_session(&self, event_bus: &Arc<EventBus>) {
        let mut state = self.state.lock().await;
        *state = SessionState::Active;
        tracing::info!(
            "[SessionManager] State -> Active. Authentication successful, publishing SessionRestored."
        );
        event_bus.publish(DomainEvent::SessionRestored);
    }
}
