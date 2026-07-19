use crate::platform::lifecycle::{LifecycleEvent, LifecycleTranslator};
use serde::{Deserialize, Serialize};

/// Platform OS-specific lifecycle abstraction.
/// Note: PlatformLifecycleEvent::Locked != PlatformLifecycleEvent::Background.
/// Background triggers when app UI is no longer active (e.g. Home Button).
/// Locked triggers when OS session boundaries change (e.g. Screen Off / Device lock).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformLifecycleEvent {
    Background,
    Foreground,
    Terminating,
    MemoryPressure,
    Locked,
}

pub struct AndroidLifecyclePlatform {
    translator: std::sync::Arc<LifecycleTranslator>,
}

impl AndroidLifecyclePlatform {
    pub fn new(translator: std::sync::Arc<LifecycleTranslator>) -> Self {
        Self { translator }
    }

    pub fn handle_os_event(&self, os_event: PlatformLifecycleEvent) {
        let lifecycle_event = match os_event {
            PlatformLifecycleEvent::Background => LifecycleEvent::AppSuspended,
            PlatformLifecycleEvent::Foreground => LifecycleEvent::AppResumed,
            PlatformLifecycleEvent::Terminating => LifecycleEvent::AppClosed,
            PlatformLifecycleEvent::MemoryPressure => {
                LifecycleEvent::Unknown("MemoryPressure".to_string())
            }
            PlatformLifecycleEvent::Locked => LifecycleEvent::AppLocked,
        };

        self.translator.translate(lifecycle_event);
    }
}
