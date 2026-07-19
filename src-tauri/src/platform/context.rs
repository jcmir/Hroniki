use super::capabilities::PlatformCapabilities;
use super::notifications::NotificationPlatform;
use super::permissions::PermissionPlatform;
use super::schedule::SchedulePlatform;
use super::storage::SecureStoragePlatform;
use std::sync::Arc;

pub struct PlatformContext {
    pub notifications: Arc<dyn NotificationPlatform>,
    pub storage: Arc<dyn SecureStoragePlatform>,
    pub permissions: Arc<dyn PermissionPlatform>,
    pub schedule: Arc<dyn SchedulePlatform>,
    pub capabilities: PlatformCapabilities,
}

impl PlatformContext {
    pub fn new(
        notifications: Arc<dyn NotificationPlatform>,
        storage: Arc<dyn SecureStoragePlatform>,
        permissions: Arc<dyn PermissionPlatform>,
        schedule: Arc<dyn SchedulePlatform>,
        capabilities: PlatformCapabilities,
    ) -> Self {
        Self {
            notifications,
            storage,
            permissions,
            schedule,
            capabilities,
        }
    }
}
