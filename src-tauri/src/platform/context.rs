use std::sync::Arc;
use super::notifications::NotificationPlatform;
use super::permissions::PermissionPlatform;
use super::storage::SecureStoragePlatform;
use super::capabilities::PlatformCapabilities;

pub struct PlatformContext {
    pub notifications: Arc<dyn NotificationPlatform>,
    pub storage: Arc<dyn SecureStoragePlatform>,
    pub permissions: Arc<dyn PermissionPlatform>,
    pub capabilities: PlatformCapabilities,
}

impl PlatformContext {
    pub fn new(
        notifications: Arc<dyn NotificationPlatform>,
        storage: Arc<dyn SecureStoragePlatform>,
        permissions: Arc<dyn PermissionPlatform>,
        capabilities: PlatformCapabilities,
    ) -> Self {
        Self {
            notifications,
            storage,
            permissions,
            capabilities,
        }
    }
}
