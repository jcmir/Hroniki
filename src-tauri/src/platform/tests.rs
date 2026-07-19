use super::adapters::android::backend::{
    KeyStoreBackend, KeyStoreError, MemoryKeyStoreBackend, JniKeyStoreBackend, KeyStoreState, KeyStoreJniBridge,
};
use super::adapters::android::storage::WrappedSecret;
use async_trait::async_trait;
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
    assert_eq!(bad_nonce.validate().unwrap_err(), KeyStoreError::InvalidSecretFormat);

    // 2. Tag length != 16 validation
    let bad_tag = WrappedSecret {
        version: 1,
        algorithm: "AES-GCM-NoPadding".to_string(),
        nonce: vec![0u8; 12],
        ciphertext: vec![1, 2, 3],
        tag: vec![0u8; 15], // 15 bytes instead of 16
    };
    assert_eq!(bad_tag.validate().unwrap_err(), KeyStoreError::InvalidSecretFormat);

    // 3. Algorithm validation
    let bad_algo = WrappedSecret {
        version: 1,
        algorithm: "AES-CBC".to_string(), // unsupported algorithm
        nonce: vec![0u8; 12],
        ciphertext: vec![1, 2, 3],
        tag: vec![0u8; 16],
    };
    assert_eq!(bad_algo.validate().unwrap_err(), KeyStoreError::InvalidSecretFormat);

    // 4. Version validation
    let bad_version = WrappedSecret {
        version: 999, // unsupported version
        algorithm: "AES-GCM-NoPadding".to_string(),
        nonce: vec![0u8; 12],
        ciphertext: vec![1, 2, 3],
        tag: vec![0u8; 16],
    };
    assert_eq!(bad_version.validate().unwrap_err(), KeyStoreError::InvalidVersion(999));

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
    assert_eq!(result_decrypt.unwrap_err(), KeyStoreError::BackendUnavailable);

    // Failed state -> wrap/unwrap must fail with BackendUnavailable
    jni_backend.set_state(KeyStoreState::Failed("TEE error".to_string())).await;
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
        Java_com_hroniki_app_LifecycleBridge_onPause,
        Java_com_hroniki_app_LifecycleBridge_onResume,
        Java_com_hroniki_app_LifecycleBridge_onLocked
    };

    let dummy_env = std::ptr::null_mut();
    let dummy_class = std::ptr::null_mut();

    // Call callbacks.
    Java_com_hroniki_app_LifecycleBridge_onPause(dummy_env, dummy_class);
    Java_com_hroniki_app_LifecycleBridge_onResume(dummy_env, dummy_class);
    Java_com_hroniki_app_LifecycleBridge_onLocked(dummy_env, dummy_class);
    
    // Success: did not panic.
}
