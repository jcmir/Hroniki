use super::super::notifications::NotificationPlatform;
use super::super::permissions::{PermissionKind, PermissionPlatform, PermissionStatus};
use super::super::storage::{SecretIdentifier, SecureStoragePlatform};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DesktopNotificationPlatform;

#[async_trait]
impl NotificationPlatform for DesktopNotificationPlatform {
    async fn show(&self, title: &str, body: Option<&str>) -> Result<(), String> {
        println!(
            "[DesktopNotification] Show: title='{}', body='{:?}'",
            title, body
        );
        Ok(())
    }
}

pub struct MemorySecureStoragePlatform {
    store: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl Default for MemorySecureStoragePlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl MemorySecureStoragePlatform {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn make_key(&self, id: &SecretIdentifier) -> String {
        format!("{}:{}", id.namespace, id.kind.as_str())
    }
}

#[async_trait]
impl SecureStoragePlatform for MemorySecureStoragePlatform {
    async fn store(&self, id: SecretIdentifier, value: &[u8]) -> Result<(), String> {
        let key = self.make_key(&id);
        let mut map = self.store.lock().await;
        map.insert(key, value.to_vec());
        Ok(())
    }

    async fn load(&self, id: SecretIdentifier) -> Result<Option<Vec<u8>>, String> {
        let key = self.make_key(&id);
        let map = self.store.lock().await;
        Ok(map.get(&key).cloned())
    }

    async fn delete(&self, id: SecretIdentifier) -> Result<(), String> {
        let key = self.make_key(&id);
        let mut map = self.store.lock().await;
        map.remove(&key);
        Ok(())
    }
}

pub struct DesktopPermissionPlatform;

#[async_trait]
impl PermissionPlatform for DesktopPermissionPlatform {
    async fn check_permission(&self, _kind: PermissionKind) -> Result<PermissionStatus, String> {
        Ok(PermissionStatus::Granted)
    }

    async fn request_permission(&self, _kind: PermissionKind) -> Result<PermissionStatus, String> {
        Ok(PermissionStatus::Granted)
    }
}
