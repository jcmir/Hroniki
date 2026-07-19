use super::capabilities::{PlatformCapabilities, PlatformCapabilitiesProvider};
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
    pub capabilities_provider: Arc<dyn PlatformCapabilitiesProvider>,
    pub capabilities: PlatformCapabilities,
}

impl PlatformContext {
    pub fn new(
        notifications: Arc<dyn NotificationPlatform>,
        storage: Arc<dyn SecureStoragePlatform>,
        permissions: Arc<dyn PermissionPlatform>,
        schedule: Arc<dyn SchedulePlatform>,
        capabilities_provider: Arc<dyn PlatformCapabilitiesProvider>,
    ) -> Self {
        let capabilities = capabilities_provider.current();
        Self {
            notifications,
            storage,
            permissions,
            schedule,
            capabilities_provider,
            capabilities,
        }
    }

    pub async fn refresh_capabilities(&mut self) -> Result<PlatformCapabilities, String> {
        let updated = self.capabilities_provider.refresh().await?;
        self.capabilities = updated.clone();
        Ok(updated)
    }
}
