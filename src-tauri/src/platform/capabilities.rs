use serde::{Deserialize, Serialize};

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
