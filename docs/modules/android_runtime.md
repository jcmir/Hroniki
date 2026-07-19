# Документация модуля: Android Runtime

Модуль `android_runtime` изолирует нативную интеграцию с платформой Android за абстрактными трейтами Rust и нативными Kotlin-мостами.

---

## 1. Топология Вызовов

```
Rust Domain ──► Platform Traits ──► Android Adapters ──► Kotlin JNI Bridges ──► Android OS API
```

1. **Rust Domain**: Вызывает методы платформенного контекста (`NotificationPlatform`, `SchedulePlatform`, `SecureStoragePlatform`, `PermissionPlatform`).
2. **Android Adapters**: Выполняют JNI-вызовы к нативным Kotlin-классам или их мок-эмуляторам.
3. **Kotlin Bridges**: Взаимодействуют с Android SDK (`NotificationManager`, `AlarmManager`, `KeyStore`, `BiometricPrompt`).

---

## 2. Kotlin Мосты (`gen/android/app/src/main/java/com/hroniki/app/`)

- `KeyStoreBridge.kt`: Защищенный мастер-ключ в Android KeyStore (TEE / StrongBox).
- `LifecycleBridge.kt`: Трансляция системных событий жизненного цикла (`onPause`, `onResume`, `onLocked`).
- `NotificationBridge.kt`: Создание каналов уведомлений (`hroniki_reminders`) и показ мгновенных оповещений.
- `AlarmBridge.kt` + `ReminderReceiver.kt`: Планирование системных алармов в `AlarmManager` и восстановление после перезагрузки устройства (`BOOT_COMPLETED`).
- `PermissionBridge.kt`: Проверка и запрос разрешений Android 13+ (`POST_NOTIFICATIONS`, `SCHEDULE_EXACT_ALARM`, `READ_EXTERNAL_STORAGE`).

---

## 3. Требования к AndroidManifest.xml

Все нативные разрешения декларируются в `AndroidManifest.xml`:
```xml
<uses-permission android:name="android.permission.POST_NOTIFICATIONS" />
<uses-permission android:name="android.permission.SCHEDULE_EXACT_ALARM" />
<uses-permission android:name="android.permission.USE_BIOMETRIC" />
<uses-permission android:name="android.permission.RECEIVE_BOOT_COMPLETED" />
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
```
