use serde::{Deserialize, Serialize};

/// Platform Capabilities Registry (MVP Boolean representation).
/// Future versions may introduce capability levels (e.g. HardwareBacked, SoftwareBacked, Unavailable).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlatformCapabilities {
    pub notifications: bool,
    pub biometric: bool,
    pub secure_hardware: bool,
    pub background_tasks: bool,
}

impl PlatformCapabilities {
    pub fn new(
        notifications: bool,
        biometric: bool,
        secure_hardware: bool,
        background_tasks: bool,
    ) -> Self {
        Self {
            notifications,
            biometric,
            secure_hardware,
            background_tasks,
        }
    }
}
