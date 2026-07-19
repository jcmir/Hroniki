use super::super::notifications::NotificationPlatform;
use super::super::permissions::{PermissionKind, PermissionPlatform, PermissionStatus};
use super::super::storage::{SecretIdentifier, SecureStoragePlatform};
use async_trait::async_trait;

pub struct AndroidNotificationPlatform;

#[async_trait]
impl NotificationPlatform for AndroidNotificationPlatform {
    async fn show(&self, _title: &str, _body: Option<&str>) -> Result<(), String> {
        Err("Android notification platform not initialized".to_string())
    }
}

pub struct AndroidSecureStoragePlatform;

#[async_trait]
impl SecureStoragePlatform for AndroidSecureStoragePlatform {
    async fn store(&self, _id: SecretIdentifier, _value: &[u8]) -> Result<(), String> {
        Err("Android secure storage not initialized".to_string())
    }

    async fn load(&self, _id: SecretIdentifier) -> Result<Option<Vec<u8>>, String> {
        Err("Android secure storage not initialized".to_string())
    }

    async fn delete(&self, _id: SecretIdentifier) -> Result<(), String> {
        Err("Android secure storage not initialized".to_string())
    }
}

pub struct AndroidPermissionPlatform;

#[async_trait]
impl PermissionPlatform for AndroidPermissionPlatform {
    async fn check_permission(&self, _kind: PermissionKind) -> Result<PermissionStatus, String> {
        Err("Android permission platform not initialized".to_string())
    }

    async fn request_permission(&self, _kind: PermissionKind) -> Result<PermissionStatus, String> {
        Err("Android permission platform not initialized".to_string())
    }
}
