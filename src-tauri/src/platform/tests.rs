use super::adapters::android::backend::{KeyStoreBackend, KeyStoreError, MemoryKeyStoreBackend};
use super::adapters::android::storage::WrappedSecret;
use super::adapters::{
    android::lifecycle::PlatformLifecycleEvent, AndroidLifecyclePlatform,
    AndroidSecureStoragePlatform, DesktopNotificationPlatform, DesktopPermissionPlatform,
    MemorySecureStoragePlatform,
};
use super::capabilities::PlatformCapabilities;
use super::context::PlatformContext;
use super::lifecycle::{LifecycleEvent, LifecycleTranslator};
use super::permissions::PermissionKind;
use super::storage::{SecretIdentifier, SecretKind, SecureStoragePlatform};
use crate::events::DomainEvent;
use crate::events::EventBus;
use std::sync::Arc;

// 1. Storage Contract Verification Helper
async fn run_storage_contract_test(storage: Arc<dyn SecureStoragePlatform>) {
    let id_db = SecretIdentifier {
        kind: SecretKind::DatabaseKey,
        namespace: "user_contract".to_string(),
    };
    let id_token = SecretIdentifier {
        kind: SecretKind::SessionToken,
        namespace: "user_contract".to_string(),
    };

    // Store & Load
    storage
        .store(id_db.clone(), b"contract_db_key")
        .await
        .unwrap();
    storage
        .store(id_token.clone(), b"contract_token")
        .await
        .unwrap();

    let loaded_db = storage.load(id_db.clone()).await.unwrap().unwrap();
    assert_eq!(loaded_db, b"contract_db_key");

    let loaded_token = storage.load(id_token.clone()).await.unwrap().unwrap();
    assert_eq!(loaded_token, b"contract_token");

    // Delete
    storage.delete(id_db.clone()).await.unwrap();
    let loaded_db_post = storage.load(id_db).await.unwrap();
    assert!(loaded_db_post.is_none());

    // Token must remain unaffected
    let loaded_token_post = storage.load(id_token).await.unwrap().unwrap();
    assert_eq!(loaded_token_post, b"contract_token");
}

#[tokio::test]
async fn test_memory_storage_contract() {
    let storage = Arc::new(MemorySecureStoragePlatform::new());
    run_storage_contract_test(storage).await;
}

#[tokio::test]
async fn test_android_storage_contract() {
    let storage = Arc::new(AndroidSecureStoragePlatform::default());
    run_storage_contract_test(storage).await;
}

#[tokio::test]
async fn test_nonce_uniqueness_and_ciphertext_safety() {
    let backend = MemoryKeyStoreBackend::new();
    let plaintext = b"sensitive_password_to_encrypt";

    let mut nonces = std::collections::HashSet::new();
    let mut ciphertexts = std::collections::HashSet::new();

    // Perform 100 encryptions of the same plaintext and ensure unique nonces and unique ciphertexts
    for _ in 0..100 {
        let wrapped = backend.wrap_key(plaintext).await.unwrap();

        assert_eq!(wrapped.nonce.len(), 12);
        assert!(
            nonces.insert(wrapped.nonce.clone()),
            "Duplicate nonce generated!"
        );
        assert!(
            ciphertexts.insert(wrapped.ciphertext.clone()),
            "Duplicate ciphertext generated for same plaintext!"
        );

        // Decrypt and confirm roundtrip
        let decrypted = backend.unwrap_key(&wrapped).await.unwrap();
        assert_eq!(decrypted, plaintext);
    }
}

#[tokio::test]
async fn test_unknown_version_rejection() {
    let backend = MemoryKeyStoreBackend::new();
    let bad_secret = WrappedSecret {
        version: 999,
        algorithm: "AES-GCM-NoPadding".to_string(),
        nonce: vec![0u8; 12],
        ciphertext: vec![1, 2, 3],
    };

    let result = backend.unwrap_key(&bad_secret).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), KeyStoreError::InvalidVersion(999));
}

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
async fn test_storage_thread_safety() {
    let storage = Arc::new(MemorySecureStoragePlatform::new());
    let mut tasks = vec![];

    // Spawn 100 concurrent tasks performing read/write/verify/delete on their own namespaces
    for i in 0..100 {
        let storage_clone = storage.clone();
        tasks.push(tokio::spawn(async move {
            let namespace = format!("namespace_{}", i);
            let id = SecretIdentifier {
                kind: SecretKind::DatabaseKey,
                namespace,
            };
            let payload = format!("payload_{}", i);

            // 1. Store
            storage_clone
                .store(id.clone(), payload.as_bytes())
                .await
                .unwrap();

            // 2. Load and verify
            let loaded = storage_clone.load(id.clone()).await.unwrap().unwrap();
            assert_eq!(loaded, payload.as_bytes());

            // 3. Delete
            storage_clone.delete(id.clone()).await.unwrap();

            // 4. Confirm deleted
            let post_delete = storage_clone.load(id).await.unwrap();
            assert!(post_delete.is_none());
        }));
    }

    for task in tasks {
        task.await.unwrap();
    }
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
async fn test_android_lifecycle_event_translation() {
    let event_bus = Arc::new(EventBus::new());
    let mut rx = event_bus.subscribe();

    let translator = Arc::new(LifecycleTranslator::new(event_bus.clone()));
    let android_lifecycle = AndroidLifecyclePlatform::new(translator);

    // Trigger Android-specific OS event Background (onPause)
    android_lifecycle.handle_os_event(PlatformLifecycleEvent::Background);

    // Verify it maps to DomainEvent::ApplicationSuspended
    let event = rx.recv().await.unwrap();
    if let DomainEvent::ApplicationSuspended = event {
        // Success
    } else {
        panic!(
            "Expected ApplicationSuspended for Android Background event, received {:?}",
            event
        );
    }
}

#[tokio::test]
async fn test_android_lifecycle_locked_translation() {
    let event_bus = Arc::new(EventBus::new());
    let mut rx = event_bus.subscribe();

    let translator = Arc::new(LifecycleTranslator::new(event_bus.clone()));
    let android_lifecycle = AndroidLifecyclePlatform::new(translator);

    // Trigger Android Locked event (Screen Off)
    android_lifecycle.handle_os_event(PlatformLifecycleEvent::Locked);

    // Verify it maps to DomainEvent::ApplicationSuspended
    let event = rx.recv().await.unwrap();
    if let DomainEvent::ApplicationSuspended = event {
        // Success
    } else {
        panic!(
            "Expected ApplicationSuspended for Android Locked event, received {:?}",
            event
        );
    }
}

#[tokio::test]
async fn test_unknown_lifecycle_event_translation() {
    let event_bus = Arc::new(EventBus::new());
    let mut rx = event_bus.subscribe();

    let translator = LifecycleTranslator::new(event_bus.clone());
    translator.translate(LifecycleEvent::Unknown("onNewAndroidCallback".to_string()));

    // Verify no event is translated or published to domain bus
    tokio::select! {
        val = rx.recv() => {
            panic!("Expected no event to be published for Unknown lifecycle callback, got {:?}", val);
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
            // Success: timed out without receiving any event
        }
    }
}

#[tokio::test]
async fn test_platform_context_initialization() {
    let notifications = Arc::new(DesktopNotificationPlatform);
    let storage = Arc::new(MemorySecureStoragePlatform::new());
    let permissions = Arc::new(DesktopPermissionPlatform);
    let capabilities = PlatformCapabilities::new(true, false, false, true);

    let context = PlatformContext::new(notifications, storage, permissions, capabilities);

    // Call and check permissions
    let status = context
        .permissions
        .check_permission(PermissionKind::Notifications)
        .await
        .unwrap();
    assert_eq!(status, super::permissions::PermissionStatus::Granted);

    // Verify capabilities
    assert!(context.capabilities.notifications);
    assert!(!context.capabilities.biometric);
}
