use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionKind {
    Notifications,
    Storage,
    ExactAlarms,
    Biometrics,
    Camera,
    /// Android 13+ (API 33+): replaces READ_EXTERNAL_STORAGE for images
    MediaImages,
    /// Android ≤ 12 (API 32): legacy storage read
    MediaLegacy,
}

impl PermissionKind {
    pub fn android_permission_name(&self) -> &'static str {
        match self {
            PermissionKind::Notifications => "android.permission.POST_NOTIFICATIONS",
            PermissionKind::Storage => "android.permission.READ_EXTERNAL_STORAGE",
            PermissionKind::ExactAlarms => "android.permission.SCHEDULE_EXACT_ALARM",
            PermissionKind::Biometrics => "android.permission.USE_BIOMETRIC",
            PermissionKind::Camera => "android.permission.CAMERA",
            PermissionKind::MediaImages => "android.permission.READ_MEDIA_IMAGES",
            PermissionKind::MediaLegacy => "android.permission.READ_EXTERNAL_STORAGE",
        }
    }

    /// Returns true if this permission only applies to Android 13+ (API 33+)
    pub fn is_api33_only(&self) -> bool {
        matches!(self, PermissionKind::MediaImages)
    }

    /// Returns true if this permission only applies to Android ≤ 12 (API 32-)
    pub fn is_legacy_only(&self) -> bool {
        matches!(self, PermissionKind::MediaLegacy)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_kind_variants() {
        assert_eq!(
            PermissionKind::MediaImages.android_permission_name(),
            "android.permission.READ_MEDIA_IMAGES"
        );
        assert_eq!(
            PermissionKind::MediaLegacy.android_permission_name(),
            "android.permission.READ_EXTERNAL_STORAGE"
        );
        assert_eq!(
            PermissionKind::Camera.android_permission_name(),
            "android.permission.CAMERA"
        );

        assert!(PermissionKind::MediaImages.is_api33_only());
        assert!(!PermissionKind::MediaImages.is_legacy_only());

        assert!(PermissionKind::MediaLegacy.is_legacy_only());
        assert!(!PermissionKind::MediaLegacy.is_api33_only());
    }
}
