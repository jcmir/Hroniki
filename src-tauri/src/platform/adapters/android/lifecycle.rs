use crate::platform::lifecycle::{LifecycleEvent, LifecycleTranslator};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};

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
    translator: Arc<LifecycleTranslator>,
}

impl AndroidLifecyclePlatform {
    pub fn new(translator: Arc<LifecycleTranslator>) -> Self {
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

// Global registry for Android Lifecycle Platform
static ANDROID_LIFECYCLE: OnceLock<Arc<AndroidLifecyclePlatform>> = OnceLock::new();

pub fn register_lifecycle(lifecycle: Arc<AndroidLifecyclePlatform>) -> Result<(), &'static str> {
    ANDROID_LIFECYCLE
        .set(lifecycle)
        .map_err(|_| "Lifecycle already registered")
}

pub fn get_lifecycle() -> Option<&'static Arc<AndroidLifecyclePlatform>> {
    ANDROID_LIFECYCLE.get()
}

// ABI-compatible raw types for JNI pointer exchange on Android
type JniEnvPtr = *mut std::ffi::c_void;
type JClassPtr = *mut std::ffi::c_void;

#[no_mangle]
pub extern "system" fn Java_com_hroniki_app_LifecycleBridge_onPause(
    _env: JniEnvPtr,
    _class: JClassPtr,
) {
    if let Some(lifecycle) = get_lifecycle() {
        lifecycle.handle_os_event(PlatformLifecycleEvent::Background);
    } else {
        tracing::warn!("JNI LifecycleBridge::onPause called before platform registration!");
    }
}

#[no_mangle]
pub extern "system" fn Java_com_hroniki_app_LifecycleBridge_onResume(
    _env: JniEnvPtr,
    _class: JClassPtr,
) {
    if let Some(lifecycle) = get_lifecycle() {
        lifecycle.handle_os_event(PlatformLifecycleEvent::Foreground);
    } else {
        tracing::warn!("JNI LifecycleBridge::onResume called before platform registration!");
    }
}

#[no_mangle]
pub extern "system" fn Java_com_hroniki_app_LifecycleBridge_onDestroy(
    _env: JniEnvPtr,
    _class: JClassPtr,
) {
    if let Some(lifecycle) = get_lifecycle() {
        lifecycle.handle_os_event(PlatformLifecycleEvent::Terminating);
    } else {
        tracing::warn!("JNI LifecycleBridge::onDestroy called before platform registration!");
    }
}

#[no_mangle]
pub extern "system" fn Java_com_hroniki_app_LifecycleBridge_onTrimMemory(
    _env: JniEnvPtr,
    _class: JClassPtr,
) {
    if let Some(lifecycle) = get_lifecycle() {
        lifecycle.handle_os_event(PlatformLifecycleEvent::MemoryPressure);
    } else {
        tracing::warn!("JNI LifecycleBridge::onTrimMemory called before platform registration!");
    }
}

#[no_mangle]
pub extern "system" fn Java_com_hroniki_app_LifecycleBridge_onLocked(
    _env: JniEnvPtr,
    _class: JClassPtr,
) {
    if let Some(lifecycle) = get_lifecycle() {
        lifecycle.handle_os_event(PlatformLifecycleEvent::Locked);
    } else {
        tracing::warn!("JNI LifecycleBridge::onLocked called before platform registration!");
    }
}
