pub mod android;
pub mod desktop;

pub use android::{
    AndroidLifecyclePlatform, AndroidNotificationPlatform, AndroidSecureStoragePlatform,
};
pub use desktop::{
    DesktopNotificationPlatform, DesktopPermissionPlatform, MemorySecureStoragePlatform,
};
