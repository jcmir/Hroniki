pub mod adapters;
pub mod capabilities;
pub mod context;
pub mod lifecycle;
pub mod notifications;
pub mod permissions;
pub mod schedule;
pub mod session;
pub mod storage;

pub use capabilities::PlatformCapabilities;
pub use context::PlatformContext;
pub use lifecycle::{LifecycleEvent, LifecycleTranslator};
pub use notifications::NotificationPlatform;
pub use permissions::{PermissionKind, PermissionPlatform, PermissionStatus};
pub use schedule::SchedulePlatform;
pub use session::SessionManager;
pub use storage::{SecretIdentifier, SecretKind, SecureStoragePlatform};

#[cfg(test)]
mod tests;
