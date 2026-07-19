pub mod android;
pub mod desktop;

pub use android::{
    AndroidNotificationPlatform, AndroidPermissionPlatform, AndroidSecureStoragePlatform,
};
pub use desktop::{
    DesktopNotificationPlatform, DesktopPermissionPlatform, MemorySecureStoragePlatform,
};
