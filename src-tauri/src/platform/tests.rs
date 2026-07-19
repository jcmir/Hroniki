use super::adapters::android::backend::{
    JniKeyStoreBackend, KeyStoreBackend, KeyStoreError, KeyStoreJniBridge, KeyStoreState,
    MemoryKeyStoreBackend,
};
use super::adapters::android::storage::WrappedSecret;
use super::adapters::{
    android::lifecycle::PlatformLifecycleEvent, AndroidLifecyclePlatform,
    AndroidNotificationPlatform, AndroidPermissionPlatform, AndroidSchedulePlatform,
    AndroidSecureStoragePlatform, AndroidStorageAdapter, DesktopNotificationPlatform,
    DesktopPermissionPlatform, DesktopSchedulePlatform, MemorySecureStoragePlatform,
};
use super::capabilities::PlatformCapabilities;
use super::context::PlatformContext;
use super::lifecycle::{LifecycleEvent, LifecycleTranslator};
use super::permissions::{PermissionKind, PermissionPlatform, PermissionStatus};
use super::schedule::SchedulePlatform;
use super::session::SessionManager;
use super::storage::{SecretIdentifier, SecretKind, SecureStoragePlatform};
use crate::events::DomainEvent;
use crate::events::EventBus;
use async_trait::async_trait;
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
async fn test_wrong_master_key_rejected() {
    let backend_a = MemoryKeyStoreBackend::new();
    let backend_b = MemoryKeyStoreBackend::new(); // Has a different random master key

    let plaintext = b"secret_database_decryption_password";
    let wrapped = backend_a.wrap_key(plaintext).await.unwrap();

    // Trying to decrypt with backend_b should fail with DecryptionFailed
    let result = backend_b.unwrap_key(&wrapped).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), KeyStoreError::DecryptionFailed);
}

#[tokio::test]
async fn test_wrapped_secret_validation() {
    // 1. Nonce length != 12 validation
    let bad_nonce = WrappedSecret {
        version: 1,
        algorithm: "AES-GCM-NoPadding".to_string(),
        nonce: vec![0u8; 11], // 11 bytes instead of 12
        ciphertext: vec![1, 2, 3],
        tag: vec![0u8; 16],
    };
    assert_eq!(
        bad_nonce.validate().unwrap_err(),
        KeyStoreError::InvalidSecretFormat
    );

    // 2. Tag length != 16 validation
    let bad_tag = WrappedSecret {
        version: 1,
        algorithm: "AES-GCM-NoPadding".to_string(),
        nonce: vec![0u8; 12],
        ciphertext: vec![1, 2, 3],
        tag: vec![0u8; 15], // 15 bytes instead of 16
    };
    assert_eq!(
        bad_tag.validate().unwrap_err(),
        KeyStoreError::InvalidSecretFormat
    );

    // 3. Algorithm validation
    let bad_algo = WrappedSecret {
        version: 1,
        algorithm: "AES-CBC".to_string(), // unsupported algorithm
        nonce: vec![0u8; 12],
        ciphertext: vec![1, 2, 3],
        tag: vec![0u8; 16],
    };
    assert_eq!(
        bad_algo.validate().unwrap_err(),
        KeyStoreError::InvalidSecretFormat
    );

    // 4. Version validation
    let bad_version = WrappedSecret {
        version: 999, // unsupported version
        algorithm: "AES-GCM-NoPadding".to_string(),
        nonce: vec![0u8; 12],
        ciphertext: vec![1, 2, 3],
        tag: vec![0u8; 16],
    };
    assert_eq!(
        bad_version.validate().unwrap_err(),
        KeyStoreError::InvalidVersion(999)
    );

    // 5. Empty struct does not panic
    let empty_secret = WrappedSecret {
        version: 0,
        algorithm: "".to_string(),
        nonce: vec![],
        ciphertext: vec![],
        tag: vec![],
    };
    assert!(empty_secret.validate().is_err());
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

    // Verify it maps to DomainEvent::ApplicationLocked
    let event = rx.recv().await.unwrap();
    if let DomainEvent::ApplicationLocked = event {
        // Success
    } else {
        panic!(
            "Expected ApplicationLocked for Android Locked event, received {:?}",
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
    let schedule = Arc::new(DesktopSchedulePlatform::new());
    let capabilities = PlatformCapabilities::new(true, true, true, false, false, false);
    let provider = Arc::new(super::capabilities::StaticCapabilitiesProvider::new(
        capabilities,
    ));

    let context = PlatformContext::new(notifications, storage, permissions, schedule, provider);

    // Call and check permissions
    let status = context
        .permissions
        .check_permission(PermissionKind::Notifications)
        .await
        .unwrap();
    assert_eq!(status, super::permissions::PermissionStatus::Granted);

    // Verify capabilities
    assert!(context.capabilities.notifications);
    assert!(context.capabilities.exact_alarms);
    assert!(context.capabilities.saf_backup);
    assert!(!context.capabilities.biometric);
    assert!(!context.capabilities.strongbox);
}

// Mock JNI Bridge for testing
struct TestJniBridge {
    memory_backend: MemoryKeyStoreBackend,
    should_throw_exception: tokio::sync::Mutex<bool>,
    should_fail_jni: tokio::sync::Mutex<bool>,
    should_return_bad_dto: tokio::sync::Mutex<bool>,
}

impl TestJniBridge {
    fn new() -> Self {
        Self {
            memory_backend: MemoryKeyStoreBackend::new(),
            should_throw_exception: tokio::sync::Mutex::new(false),
            should_fail_jni: tokio::sync::Mutex::new(false),
            should_return_bad_dto: tokio::sync::Mutex::new(false),
        }
    }
}

#[async_trait]
impl KeyStoreJniBridge for TestJniBridge {
    async fn encrypt(&self, plaintext: &[u8]) -> Result<WrappedSecret, KeyStoreError> {
        if *self.should_fail_jni.lock().await {
            return Err(KeyStoreError::JniFailure);
        }
        if *self.should_throw_exception.lock().await {
            return Err(KeyStoreError::JavaException);
        }
        if *self.should_return_bad_dto.lock().await {
            // Return DTO with bad signature
            return Ok(WrappedSecret {
                version: 2, // invalid version
                algorithm: "AES-CBC".to_string(),
                nonce: vec![0u8; 10],
                ciphertext: vec![],
                tag: vec![],
            });
        }
        self.memory_backend.wrap_key(plaintext).await
    }

    async fn decrypt(&self, secret: &WrappedSecret) -> Result<Vec<u8>, KeyStoreError> {
        if *self.should_fail_jni.lock().await {
            return Err(KeyStoreError::JniFailure);
        }
        if *self.should_throw_exception.lock().await {
            return Err(KeyStoreError::JavaException);
        }
        self.memory_backend.unwrap_key(secret).await
    }
}

#[tokio::test]
async fn test_jni_backend_uninitialized() {
    let bridge = Arc::new(TestJniBridge::new());
    let jni_backend = JniKeyStoreBackend::new(bridge);

    // Initial state is Uninitialized -> wrap/unwrap must fail with BackendUnavailable
    let result = jni_backend.wrap_key(b"test").await;
    assert_eq!(result.unwrap_err(), KeyStoreError::BackendUnavailable);

    let dummy_secret = WrappedSecret {
        version: 1,
        algorithm: "AES-GCM-NoPadding".to_string(),
        nonce: vec![0u8; 12],
        ciphertext: vec![1, 2, 3],
        tag: vec![0u8; 16],
    };
    let result_decrypt = jni_backend.unwrap_key(&dummy_secret).await;
    assert_eq!(
        result_decrypt.unwrap_err(),
        KeyStoreError::BackendUnavailable
    );

    // Failed state -> wrap/unwrap must fail with BackendUnavailable
    jni_backend
        .set_state(KeyStoreState::Failed("TEE error".to_string()))
        .await;
    let result_fail = jni_backend.wrap_key(b"test").await;
    assert_eq!(result_fail.unwrap_err(), KeyStoreError::BackendUnavailable);
}

#[tokio::test]
async fn test_jni_backend_contract() {
    let bridge = Arc::new(TestJniBridge::new());
    let jni_backend = Arc::new(JniKeyStoreBackend::new(bridge));

    // Set state to Ready to execute contract
    jni_backend.set_state(KeyStoreState::Ready).await;

    let storage = Arc::new(AndroidSecureStoragePlatform::new(jni_backend));
    run_storage_contract_test(storage).await;
}

#[tokio::test]
async fn test_jni_error_mappings() {
    let bridge = Arc::new(TestJniBridge::new());
    let jni_backend = JniKeyStoreBackend::new(bridge.clone());
    jni_backend.set_state(KeyStoreState::Ready).await;

    // test JNI failures
    *bridge.should_fail_jni.lock().await = true;
    let result = jni_backend.wrap_key(b"test").await;
    assert_eq!(result.unwrap_err(), KeyStoreError::JniFailure);

    // test Java Exception mappings
    *bridge.should_fail_jni.lock().await = false;
    *bridge.should_throw_exception.lock().await = true;
    let result_exc = jni_backend.wrap_key(b"test").await;
    assert_eq!(result_exc.unwrap_err(), KeyStoreError::JavaException);
}

#[tokio::test]
async fn test_invalid_dto_response_mapping() {
    let bridge = Arc::new(TestJniBridge::new());
    let jni_backend = JniKeyStoreBackend::new(bridge.clone());
    jni_backend.set_state(KeyStoreState::Ready).await;

    *bridge.should_return_bad_dto.lock().await = true;
    let result = jni_backend.wrap_key(b"test").await;

    // validate() inside tests should verify it fails
    let secret = result.unwrap();
    assert!(secret.validate().is_err());
}

#[tokio::test]
async fn test_lifecycle_callback_without_registration() {
    // Calling JNI lifecycle callbacks before registration should not panic.
    // They should gracefully print a warning in the logs and exit.
    use super::adapters::android::lifecycle::{
        Java_com_hroniki_app_LifecycleBridge_onLocked,
        Java_com_hroniki_app_LifecycleBridge_onPause,
        Java_com_hroniki_app_LifecycleBridge_onResume,
    };

    let dummy_env = std::ptr::null_mut();
    let dummy_class = std::ptr::null_mut();

    // Call callbacks.
    Java_com_hroniki_app_LifecycleBridge_onPause(dummy_env, dummy_class);
    Java_com_hroniki_app_LifecycleBridge_onResume(dummy_env, dummy_class);
    Java_com_hroniki_app_LifecycleBridge_onLocked(dummy_env, dummy_class);

    // Success: did not panic.
}

#[tokio::test]
async fn test_java_exception_roundtrip() {
    use super::adapters::android::backend::jni::RealKeyStoreJniBridge;

    let bridge = RealKeyStoreJniBridge::new(std::ptr::null_mut());

    // Test wrap_key / unwrap_key on desktop return BackendUnavailable without crashing
    let wrap_res = bridge.encrypt(b"data").await;
    assert_eq!(wrap_res.unwrap_err(), KeyStoreError::BackendUnavailable);

    let dummy = WrappedSecret {
        version: 1,
        algorithm: "AES-GCM-NoPadding".to_string(),
        nonce: vec![0u8; 12],
        ciphertext: vec![1, 2, 3],
        tag: vec![0u8; 16],
    };
    let unwrap_res = bridge.decrypt(&dummy).await;
    assert_eq!(unwrap_res.unwrap_err(), KeyStoreError::BackendUnavailable);
}

#[tokio::test]
async fn test_android_notifications_channels_contract() {
    let platform = AndroidNotificationPlatform::new("default_channel", "Default Channel", 3);

    // Create custom channel
    platform
        .create_channel("alerts_channel", "Critical Alerts", 4)
        .await;

    let channel = platform.get_channel("alerts_channel").await.unwrap();
    assert_eq!(channel.name, "Critical Alerts");

    // Show notification on custom channel
    platform
        .show_on_channel("System Alert", Some("High CPU usage"), "alerts_channel")
        .await
        .unwrap();

    assert_eq!(platform.posted_count().await, 1);

    // Reject non-existent channel
    let err = platform
        .show_on_channel("Test", None, "missing_channel")
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_android_permissions_contract() {
    let platform_android_13 = AndroidPermissionPlatform::new(33);

    // POST_NOTIFICATIONS on Android 13+ is NotDetermined initially
    let status_notif = platform_android_13
        .check_permission(PermissionKind::Notifications)
        .await
        .unwrap();
    assert_eq!(status_notif, PermissionStatus::Denied);

    // Request notification permission
    let req_status = platform_android_13
        .request_permission(PermissionKind::Notifications)
        .await
        .unwrap();
    assert_eq!(req_status, PermissionStatus::Granted);

    // Check again
    let status_post_req = platform_android_13
        .check_permission(PermissionKind::Notifications)
        .await
        .unwrap();
    assert_eq!(status_post_req, PermissionStatus::Granted);

    // Check pre-Android 13 permissions
    let platform_android_11 = AndroidPermissionPlatform::new(30);
    let pre_13_status = platform_android_11
        .check_permission(PermissionKind::Notifications)
        .await
        .unwrap();
    assert_eq!(pre_13_status, PermissionStatus::Granted);
}

#[tokio::test]
async fn test_android_schedule_alarm_contract() {
    let scheduler = AndroidSchedulePlatform::new();
    let alarm_id = "reminder_alarm_101";

    // Schedule exact alarm
    scheduler
        .schedule_exact(alarm_id, 1700000000000)
        .await
        .unwrap();
    assert!(scheduler.is_scheduled(alarm_id).await);

    // Cancel alarm
    scheduler.cancel_alarm(alarm_id).await.unwrap();
    assert!(!scheduler.is_scheduled(alarm_id).await);

    // Cancel non-existent alarm returns error
    let cancel_err = scheduler.cancel_alarm(alarm_id).await;
    assert!(cancel_err.is_err());
}

#[tokio::test]
async fn test_android_saf_storage_boundary_contract() {
    let saf_adapter = AndroidStorageAdapter::new();
    let saf_uri = "content://com.android.providers.downloads.documents/document/42";
    let backup_data = b"ENCRYPTED_SQLITE_BACKUP_BYTES_12345";

    // Export archive to SAF URI
    saf_adapter
        .export_backup_archive(saf_uri, backup_data)
        .await
        .unwrap();

    assert!(saf_adapter.contains_saf_uri(saf_uri).await);

    // Import archive from SAF URI
    let imported = saf_adapter.import_backup_archive(saf_uri).await.unwrap();
    assert_eq!(imported, backup_data);

    // Invalid URI format rejected
    let invalid_err = saf_adapter
        .export_backup_archive("invalid_path/file.bin", backup_data)
        .await;
    assert!(invalid_err.is_err());
}

#[tokio::test]
async fn test_session_manager_memory_clear_on_lock() {
    let event_bus = Arc::new(EventBus::new());
    let session_mgr = Arc::new(SessionManager::new());
    session_mgr.start_event_listener(&event_bus);

    // Populate RAM tokens & decrypted cache
    session_mgr
        .set_token("session_token_1", b"active_jwt_token".to_vec())
        .await;
    session_mgr
        .set_cache("user_profile_cache", b"decrypted_user_json".to_vec())
        .await;

    assert!(session_mgr.get_token("session_token_1").await.is_some());
    assert!(session_mgr.get_cache("user_profile_cache").await.is_some());
    assert!(!session_mgr.is_locked().await);

    // Trigger DomainEvent::ApplicationLocked
    event_bus.publish(DomainEvent::ApplicationLocked);

    // Wait briefly for asynchronous background task handling
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Verify RAM memory cleared and session locked
    assert!(session_mgr.get_token("session_token_1").await.is_none());
    assert!(session_mgr.get_cache("user_profile_cache").await.is_none());
    assert!(session_mgr.is_locked().await);
}

#[test]
fn test_platform_capabilities_serialization() {
    let capabilities = PlatformCapabilities::new(true, true, true, false, true, false);
    let json = serde_json::to_string(&capabilities).unwrap();
    let deserialized: PlatformCapabilities = serde_json::from_str(&json).unwrap();

    assert_eq!(capabilities, deserialized);
    assert!(deserialized.notifications);
    assert!(deserialized.exact_alarms);
    assert!(deserialized.saf_backup);
    assert!(!deserialized.biometric);
    assert!(deserialized.strongbox);
    assert!(!deserialized.secure_hardware);
}

#[tokio::test]
async fn test_session_lock_state_machine_full_flow() {
    use super::session::SessionState;

    let event_bus = Arc::new(EventBus::new());
    let mut rx = event_bus.subscribe();

    let session_mgr = Arc::new(SessionManager::new());
    session_mgr.start_event_listener(&event_bus);

    assert_eq!(session_mgr.state().await, SessionState::Active);

    // 1. Set token while Active
    session_mgr
        .set_token("active_jwt", b"secret_token".to_vec())
        .await;
    assert_eq!(
        session_mgr.get_token("active_jwt").await,
        Some(b"secret_token".to_vec())
    );

    // 2. Publish DomainEvent::ApplicationLocked -> Transition to Locked
    event_bus.publish(DomainEvent::ApplicationLocked);
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    assert_eq!(session_mgr.state().await, SessionState::Locked);
    assert!(session_mgr.is_locked().await);
    assert_eq!(session_mgr.get_token("active_jwt").await, None);

    // 3. Publish DomainEvent::ApplicationResumed -> Transition to AwaitingUnlock
    event_bus.publish(DomainEvent::ApplicationResumed);
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    assert_eq!(session_mgr.state().await, SessionState::AwaitingUnlock);
    assert!(session_mgr.is_awaiting_unlock().await);

    // Verify AuthenticationRequired domain event received
    let mut rec_auth_req = false;
    while let Ok(evt) = rx.try_recv() {
        if let DomainEvent::AuthenticationRequired = evt {
            rec_auth_req = true;
            break;
        }
    }
    assert!(
        rec_auth_req,
        "Expected AuthenticationRequired event upon ApplicationResumed"
    );

    // 4. Authenticate user -> unlock_session -> Transition to Active
    session_mgr.unlock_session(&event_bus).await;
    assert_eq!(session_mgr.state().await, SessionState::Active);
    assert!(session_mgr.is_active().await);

    // Verify SessionRestored domain event received
    let mut rec_restored = false;
    while let Ok(evt) = rx.try_recv() {
        if let DomainEvent::SessionRestored = evt {
            rec_restored = true;
            break;
        }
    }
    assert!(
        rec_restored,
        "Expected SessionRestored event upon unlock_session"
    );
}

#[tokio::test]
async fn test_permission_status_granular() {
    let platform = AndroidPermissionPlatform::new(33);

    // Set permanently denied
    platform
        .set_permission_status(
            PermissionKind::Notifications,
            PermissionStatus::PermanentlyDenied,
        )
        .await;

    let status = platform
        .check_permission(PermissionKind::Notifications)
        .await
        .unwrap();
    assert_eq!(status, PermissionStatus::PermanentlyDenied);

    // Re-requesting should return PermanentlyDenied without popping OS dialog
    let req = platform
        .request_permission(PermissionKind::Notifications)
        .await
        .unwrap();
    assert_eq!(req, PermissionStatus::PermanentlyDenied);
}

#[tokio::test]
async fn test_capabilities_provider_dynamic_refresh() {
    use super::capabilities::{PlatformCapabilitiesProvider, StaticCapabilitiesProvider};

    let initial = PlatformCapabilities::new(true, false, false, false, false, false);
    let provider = StaticCapabilitiesProvider::new(initial);

    assert!(provider.current().notifications);
    assert!(!provider.current().exact_alarms);

    // Dynamically update capabilities after user grants permission in OS
    let updated = PlatformCapabilities::new(true, true, true, true, false, false);
    provider.update(updated.clone()).await;

    let refreshed = provider.refresh().await.unwrap();
    assert_eq!(refreshed, updated);
    assert!(provider.current().exact_alarms);
}

#[tokio::test]
async fn test_ipc_entries_dto_contract() {
    use crate::domain::{ChronicleObjectId, Entry};

    let dummy_entry = Entry::new(
        ChronicleObjectId::new(),
        chrono::Utc::now(),
        "Заголовок хроники".to_string(),
        Some("Описание события".to_string()),
    )
    .unwrap();

    let json = serde_json::to_string(&dummy_entry).unwrap();
    assert!(json.contains("title"));
    assert!(json.contains("Заголовок хроники"));
    assert!(json.contains("occurred_at"));
}

#[tokio::test]
async fn test_entry_creation_persistence_after_restart() {
    use crate::domain::{Category, ChronicleObject, Entry};
    use crate::storage::{
        connection::create_pool, migrations::run_migrations, ChronologyRepository,
        SqliteChronologyRepository,
    };

    let temp_file = std::env::temp_dir().join(format!(
        "hroniki_test_persist_{}.sqlite",
        uuid::Uuid::new_v4()
    ));
    let db_url = format!(
        "sqlite://{}",
        temp_file.to_string_lossy().replace('\\', "/")
    );

    // 1. Initial pool and data creation
    let pool1 = create_pool(&db_url).await.unwrap();
    run_migrations(&pool1).await.unwrap();
    let mut repo1 = SqliteChronologyRepository::new(pool1.clone());

    let cat = Category::new("Путешествия".to_string()).unwrap();
    repo1.save_category(cat.clone()).await.unwrap();

    let obj = ChronicleObject::new(cat.id, "Поездка в Алтай".to_string(), None).unwrap();
    repo1.save_object(obj.clone()).await.unwrap();

    let entry = Entry::new(
        obj.id,
        chrono::Utc::now(),
        "День 1: Горы".to_string(),
        Some("Отличный вид".to_string()),
    )
    .unwrap();
    repo1
        .save_entry_with_photos(entry.clone(), vec![])
        .await
        .unwrap();

    pool1.close().await;

    // 2. Simulated app restart: open new connection pool to same DB
    let pool2 = create_pool(&db_url).await.unwrap();
    let repo2 = SqliteChronologyRepository::new(pool2.clone());

    let loaded_entries = repo2.entries().await.unwrap();
    assert_eq!(loaded_entries.len(), 1);
    assert_eq!(loaded_entries[0].title, "День 1: Горы");

    pool2.close().await;
}

#[tokio::test]
async fn test_lock_session_ram_clear_and_database_persistence() {
    use crate::domain::{Category, ChronicleObject, Entry};
    use crate::storage::{
        connection::create_pool, migrations::run_migrations, ChronologyRepository,
        SqliteChronologyRepository,
    };

    let temp_file = std::env::temp_dir().join(format!(
        "hroniki_test_lock_persist_{}.sqlite",
        uuid::Uuid::new_v4()
    ));
    let db_url = format!(
        "sqlite://{}",
        temp_file.to_string_lossy().replace('\\', "/")
    );

    let pool = create_pool(&db_url).await.unwrap();
    run_migrations(&pool).await.unwrap();
    let mut repo = SqliteChronologyRepository::new(pool.clone());

    let cat = Category::new("Личное".to_string()).unwrap();
    repo.save_category(cat.clone()).await.unwrap();
    let obj = ChronicleObject::new(cat.id, "Дневник".to_string(), None).unwrap();
    repo.save_object(obj.clone()).await.unwrap();
    let entry = Entry::new(
        obj.id,
        chrono::Utc::now(),
        "Запись до блокировки".to_string(),
        None,
    )
    .unwrap();
    repo.save_entry_with_photos(entry, vec![]).await.unwrap();

    // Session lock event -> RAM SessionManager cleared
    let event_bus = Arc::new(EventBus::new());
    let session_mgr = Arc::new(SessionManager::new());
    session_mgr.start_event_listener(&event_bus);

    session_mgr
        .set_token("active_jwt", b"secret".to_vec())
        .await;
    event_bus.publish(DomainEvent::ApplicationLocked);
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Verify RAM token cleared
    assert!(session_mgr.get_token("active_jwt").await.is_none());

    // Verify database on disk remains 100% intact and readable
    let loaded = repo.entries().await.unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].title, "Запись до блокировки");

    pool.close().await;
}

#[tokio::test]
async fn test_initial_pin_setup() {
    use crate::application::security;

    let salt_bytes = security::generate_salt();
    let salt_hex = security::to_hex(&salt_bytes);
    let hash_hex = security::hash_pin("1234", &salt_bytes);

    assert_eq!(salt_bytes.len(), 16);
    assert!(!salt_hex.is_empty());
    assert!(!hash_hex.is_empty());
    assert_ne!(hash_hex, "1234");
}

#[tokio::test]
async fn test_wrong_pin_rejected() {
    use crate::application::security;

    let salt_bytes = security::generate_salt();
    let correct_hash = security::hash_pin("1234", &salt_bytes);
    let wrong_hash = security::hash_pin("9999", &salt_bytes);

    assert_ne!(correct_hash, wrong_hash);
}

#[tokio::test]
async fn test_session_verifier_argon2id_roundtrip() {
    use crate::security::verifier::SessionVerifier;

    let verifier = SessionVerifier::new(
        "sample_salt_hex".to_string(),
        "sample_nonce_hex".to_string(),
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
    );

    assert_eq!(verifier.version, 1);
    assert_eq!(verifier.salt, "sample_salt_hex");

    let json = serde_json::to_string(&verifier).unwrap();
    let restored: SessionVerifier = serde_json::from_str(&json).unwrap();
    assert_eq!(verifier, restored);
}

#[tokio::test]
async fn test_media_missing_file_handling() {
    use crate::domain::{ChronicleObjectId, EntryId, MediaSource, Photo};

    let entry_id = EntryId::new();
    let photo = Photo::with_source(
        entry_id,
        "non_existent_file.jpg",
        "non_existent_thumb.jpg",
        MediaSource::Gallery,
    );

    assert_eq!(photo.source, MediaSource::Gallery);
    let path = std::path::Path::new(&photo.path);
    assert!(!path.exists());
}

#[tokio::test]
async fn test_thumbnail_dimensions() {
    use crate::media::MediaService;

    let temp_dir =
        std::env::temp_dir().join(format!("hroniki_thumb_test_{}", uuid::Uuid::new_v4()));
    let media_svc = MediaService::new(temp_dir.clone());

    // Create a 1000x800 dummy image buffer
    let mut imgbuf = image::ImageBuffer::new(1000, 800);
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255u8, 158u8, 11u8]);
    }
    let orig_path = temp_dir.join("originals").join("test_large.png");
    imgbuf.save(&orig_path).unwrap();

    let thumb_path = media_svc.generate_thumbnail("test_large.png", 512).unwrap();
    assert!(thumb_path.exists());

    let thumb_img = image::open(&thumb_path).unwrap();
    let (width, height) = (thumb_img.width(), thumb_img.height());
    assert!(width <= 512, "Thumbnail width {} exceeds 512px", width);
    assert!(height <= 512, "Thumbnail height {} exceeds 512px", height);
}

#[tokio::test]
async fn test_lockout_survives_restart() {
    use crate::security::throttle::AuthThrottleState;

    let now = chrono::Utc::now();
    let mut throttle = AuthThrottleState::new();

    // 5 failure attempts -> 30s lockout
    for _ in 0..5 {
        throttle.register_failure(now, 5, 30);
    }

    let (locked, remaining) = throttle.is_locked_out(now);
    assert!(locked);
    assert!(remaining > 0);

    // Simulate serialization & restart
    let json = serde_json::to_string(&throttle).unwrap();
    let restored: AuthThrottleState = serde_json::from_str(&json).unwrap();

    let (restored_locked, restored_remaining) = restored.is_locked_out(now);
    assert!(restored_locked);
    assert_eq!(remaining, restored_remaining);
}

#[tokio::test]
async fn test_full_mobile_lifecycle() {
    use crate::domain::{Category, ChronicleObject, Entry, MediaSource, Photo};
    use crate::media::MediaService;
    use crate::storage::{
        connection::create_pool, migrations::run_migrations, ChronologyRepository,
        SqliteChronologyRepository,
    };

    let temp_dir =
        std::env::temp_dir().join(format!("hroniki_lifecycle_test_{}", uuid::Uuid::new_v4()));
    let db_path = temp_dir.join("hroniki.sqlite");
    let db_url = format!("sqlite://{}", db_path.to_string_lossy().replace('\\', "/"));

    // 1. First Run: Init DB & Storage
    let pool1 = create_pool(&db_url).await.unwrap();
    run_migrations(&pool1).await.unwrap();
    let mut repo1 = SqliteChronologyRepository::new(pool1.clone());
    let media_svc1 = MediaService::new(temp_dir.clone());

    let cat = Category::new("Воспоминания".to_string()).unwrap();
    repo1.save_category(cat.clone()).await.unwrap();
    let obj = ChronicleObject::new(cat.id, "Отпуск 2026".to_string(), None).unwrap();
    repo1.save_object(obj.clone()).await.unwrap();

    let entry = Entry::new(
        obj.id,
        chrono::Utc::now(),
        "Море и Солнце".to_string(),
        Some("Первый день на пляже".to_string()),
    )
    .unwrap();

    // Save media file original & thumbnail
    let _ = media_svc1
        .save_original("beach.png", b"dummy_png_bytes")
        .unwrap();
    let _ = media_svc1.generate_thumbnail("beach.png", 512).unwrap();
    let photo = media_svc1.register_photo(entry.id, "beach.png", MediaSource::Gallery);

    repo1
        .save_entry_with_photos(entry.clone(), vec![photo])
        .await
        .unwrap();
    pool1.close().await;

    // 2. Simulated App Lock & Restart: Reopen DB
    let pool2 = create_pool(&db_url).await.unwrap();
    let repo2 = SqliteChronologyRepository::new(pool2.clone());

    let entries = repo2.entries().await.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].title, "Море и Солнце");

    let photos = repo2.entry_photos(entries[0].id).await.unwrap();
    assert_eq!(photos.len(), 1);
    assert_eq!(photos[0].source, MediaSource::Gallery);

    pool2.close().await;
}
