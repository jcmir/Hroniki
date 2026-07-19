use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionKind {
    Notifications,
    Storage,
    ExactAlarms,
    Biometrics,
    Camera,
}

impl PermissionKind {
    pub fn android_permission_name(&self) -> &'static str {
        match self {
            PermissionKind::Notifications => "android.permission.POST_NOTIFICATIONS",
            PermissionKind::Storage => "android.permission.READ_EXTERNAL_STORAGE",
            PermissionKind::ExactAlarms => "android.permission.SCHEDULE_EXACT_ALARM",
            PermissionKind::Biometrics => "android.permission.USE_BIOMETRIC",
            PermissionKind::Camera => "android.permission.CAMERA",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionStatus {
    Granted,
    Denied,
    PermanentlyDenied,
    Unsupported,
}

#[async_trait]
pub trait PermissionPlatform: Send + Sync {
    async fn check_permission(&self, kind: PermissionKind) -> Result<PermissionStatus, String>;
    async fn request_permission(&self, kind: PermissionKind) -> Result<PermissionStatus, String>;
}
