pub mod android;
pub mod desktop;

pub use android::{
    AndroidLifecyclePlatform, AndroidNotificationPlatform, AndroidPermissionPlatform,
    AndroidSchedulePlatform, AndroidSecureStoragePlatform, AndroidStorageAdapter,
};
pub use desktop::{
    DesktopNotificationPlatform, DesktopPermissionPlatform, DesktopSchedulePlatform,
    MemorySecureStoragePlatform,
};
