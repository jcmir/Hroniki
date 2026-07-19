use crate::platform::permissions::{PermissionKind, PermissionPlatform, PermissionStatus};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AndroidPermissionPlatform {
    permission_states: Arc<Mutex<HashMap<PermissionKind, PermissionStatus>>>,
    sdk_version: u32,
}

impl Default for AndroidPermissionPlatform {
    fn default() -> Self {
        Self::new(33) // Default to Android 13+ (API 33)
    }
}

impl AndroidPermissionPlatform {
    pub fn new(sdk_version: u32) -> Self {
        Self {
            permission_states: Arc::new(Mutex::new(HashMap::new())),
            sdk_version,
        }
    }

    pub fn sdk_version(&self) -> u32 {
        self.sdk_version
    }

    pub async fn set_permission_status(&self, kind: PermissionKind, status: PermissionStatus) {
        let mut map = self.permission_states.lock().await;
        map.insert(kind, status);
    }
}

#[async_trait]
impl PermissionPlatform for AndroidPermissionPlatform {
    async fn check_permission(&self, kind: PermissionKind) -> Result<PermissionStatus, String> {
        let map = self.permission_states.lock().await;
        if let Some(status) = map.get(&kind) {
            return Ok(*status);
        }

        // Default platform behavior by API level rules:
        match kind {
            PermissionKind::Notifications => {
                if self.sdk_version >= 33 {
                    // Android 13+ requires explicit POST_NOTIFICATIONS grant
                    Ok(PermissionStatus::Denied)
                } else {
                    // Pre-Android 13 notifications are granted by default
                    Ok(PermissionStatus::Granted)
                }
            }
            PermissionKind::Storage => Ok(PermissionStatus::Denied),
            PermissionKind::ExactAlarms => {
                if self.sdk_version >= 31 {
                    Ok(PermissionStatus::Denied)
                } else {
                    Ok(PermissionStatus::Granted)
                }
            }
            PermissionKind::Biometrics
            | PermissionKind::Camera
            | PermissionKind::MediaImages
            | PermissionKind::MediaLegacy => Ok(PermissionStatus::Denied),
        }
    }

    async fn request_permission(&self, kind: PermissionKind) -> Result<PermissionStatus, String> {
        let mut map = self.permission_states.lock().await;

        // If permission was permanently denied, return PermanentlyDenied without re-requesting OS dialog
        if let Some(PermissionStatus::PermanentlyDenied) = map.get(&kind) {
            return Ok(PermissionStatus::PermanentlyDenied);
        }

        let new_status = match map.get(&kind) {
            Some(status) => *status,
            None => PermissionStatus::Granted,
        };

        map.insert(kind, new_status);
        tracing::info!(
            "[AndroidPermissionPlatform] Permission {:?} ({}) requested -> {:?}",
            kind,
            kind.android_permission_name(),
            new_status
        );
        Ok(new_status)
    }
}
