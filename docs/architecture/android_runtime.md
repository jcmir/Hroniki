# Архитектура JNI-мостов и Android Runtime

Этот документ описывает структуру слоев и потоков данных в мобильном рантайме Android приложения ХРОНИКИ.

---

## 1. Схема каталогов и слоев

```
[ Domain Layer (Rust Core) ]
            │
            ▼ (Трейты)
[ platform::adapters::android ]  <-- Содержит Rust JNI bindings и вызовы Kotlin-мостов
            │
      (Tauri Mobile IPC)
            │
            ▼ (Kotlin)
   [ com.hroniki.security ]      <-- KeyStoreBridge.kt, LifecycleBridge.kt
```

---

## 2. Изоляция Вызовов
Rust-код не зависит напрямую от пакетов `android.security.*` или `java.security.*`. Адаптер `platform/adapters/android/storage.rs` передает и принимает данные через `JNIEnv` в виде сырых массивов байтов. Вся логика создания Cipher, обработки KeyStore и проверки отпечатков пальцев пишется на стороне Kotlin.

---

## 3. Версионирование алгоритмов
Адаптер шифрования оперирует структурой `WrappedSecret`. При дешифровании адаптер считывает поле `version` и передает `ciphertext` и параметры алгоритма в Kotlin Bridge, что позволяет менять алгоритмы шифрования без изменения API взаимодействия.
