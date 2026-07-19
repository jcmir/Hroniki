use super::adapters::{
    DesktopNotificationPlatform, DesktopPermissionPlatform, MemorySecureStoragePlatform,
};
use super::context::PlatformContext;
use super::lifecycle::{LifecycleEvent, LifecycleTranslator};
use super::permissions::PermissionKind;
use super::storage::{SecretIdentifier, SecretKind, SecureStoragePlatform};
use crate::events::DomainEvent;
use crate::events::EventBus;
use std::sync::Arc;

#[tokio::test]
async fn test_storage_isolation() {
    let storage = MemorySecureStoragePlatform::new();

    let id_db = SecretIdentifier {
        kind: SecretKind::DatabaseKey,
        namespace: "user_a".to_string(),
    };
    let id_token = SecretIdentifier {
        kind: SecretKind::SessionToken,
        namespace: "user_a".to_string(),
    };
    let id_other_user = SecretIdentifier {
        kind: SecretKind::DatabaseKey,
        namespace: "user_b".to_string(),
    };

    storage
        .store(id_db.clone(), b"db_secret_key")
        .await
        .unwrap();
    storage
        .store(id_token.clone(), b"token_secret")
        .await
        .unwrap();
    storage
        .store(id_other_user.clone(), b"other_user_db_key")
        .await
        .unwrap();

    // Verify isolation by Kind
    let loaded_db = storage.load(id_db).await.unwrap().unwrap();
    assert_eq!(loaded_db, b"db_secret_key");

    let loaded_token = storage.load(id_token).await.unwrap().unwrap();
    assert_eq!(loaded_token, b"token_secret");

    // Verify isolation by Namespace (User)
    let loaded_other = storage.load(id_other_user).await.unwrap().unwrap();
    assert_eq!(loaded_other, b"other_user_db_key");
}

#[tokio::test]
async fn test_lifecycle_event_translation() {
    let event_bus = Arc::new(EventBus::new());
    let mut rx = event_bus.subscribe();

    let translator = LifecycleTranslator::new(event_bus.clone());
    translator.translate(LifecycleEvent::AppSuspended);

    // Receive translation
    let event = rx.recv().await.unwrap();
    if let DomainEvent::ApplicationSuspended = event {
        // Success
    } else {
        panic!("Expected ApplicationSuspended, received {:?}", event);
    }
}

#[tokio::test]
async fn test_platform_context_initialization() {
    let notifications = Arc::new(DesktopNotificationPlatform);
    let storage = Arc::new(MemorySecureStoragePlatform::new());
    let permissions = Arc::new(DesktopPermissionPlatform);

    let context = PlatformContext::new(notifications, storage, permissions);

    // Call and check permissions
    let status = context
        .permissions
        .check_permission(PermissionKind::Notifications)
        .await
        .unwrap();
    assert_eq!(status, super::permissions::PermissionStatus::Granted);
}
