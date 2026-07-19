use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionKind {
    Notifications,
    Storage,
    Biometrics,
    Camera,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionStatus {
    Granted,
    Denied,
    NotDetermined,
}

#[async_trait]
pub trait PermissionPlatform: Send + Sync {
    async fn check_permission(&self, kind: PermissionKind) -> Result<PermissionStatus, String>;
    async fn request_permission(&self, kind: PermissionKind) -> Result<PermissionStatus, String>;
}
