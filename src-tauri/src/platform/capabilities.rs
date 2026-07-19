use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Platform Capabilities Registry (MVP Boolean representation).
/// Future versions may introduce capability levels (e.g. HardwareBacked, SoftwareBacked, Unavailable).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlatformCapabilities {
    pub notifications: bool,
    pub exact_alarms: bool,
    pub saf_backup: bool,
    pub biometric: bool,
    pub strongbox: bool,
    pub secure_hardware: bool,
}

impl PlatformCapabilities {
    pub fn new(
        notifications: bool,
        exact_alarms: bool,
        saf_backup: bool,
        biometric: bool,
        strongbox: bool,
        secure_hardware: bool,
    ) -> Self {
        Self {
            notifications,
            exact_alarms,
            saf_backup,
            biometric,
            strongbox,
            secure_hardware,
        }
    }
}

#[async_trait]
pub trait PlatformCapabilitiesProvider: Send + Sync {
    async fn refresh(&self) -> Result<PlatformCapabilities, String>;
    fn current(&self) -> PlatformCapabilities;
}

pub struct StaticCapabilitiesProvider {
    capabilities: Arc<RwLock<PlatformCapabilities>>,
}

impl StaticCapabilitiesProvider {
    pub fn new(capabilities: PlatformCapabilities) -> Self {
        Self {
            capabilities: Arc::new(RwLock::new(capabilities)),
        }
    }

    pub async fn update(&self, new_caps: PlatformCapabilities) {
        let mut caps = self.capabilities.write().await;
        *caps = new_caps;
    }
}

#[async_trait]
impl PlatformCapabilitiesProvider for StaticCapabilitiesProvider {
    async fn refresh(&self) -> Result<PlatformCapabilities, String> {
        Ok(self.capabilities.read().await.clone())
    }

    fn current(&self) -> PlatformCapabilities {
        // Safe synchronous fallback for quick capability checks
        if let Ok(guard) = self.capabilities.try_read() {
            guard.clone()
        } else {
            PlatformCapabilities::new(false, false, false, false, false, false)
        }
    }
}
