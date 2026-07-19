# Документация модуля: Android Runtime

Модуль `android_runtime` изолирует все особенности интеграции с платформой Android за абстрактными трейтами Rust.

---

## 1. Абстракция Жизненного Цикла (PlatformLifecycleEvent)

Низкоуровневые коллбэки операционной системы транслируются в нейтральный доменный енам `PlatformLifecycleEvent`:

```rust
pub enum PlatformLifecycleEvent {
    Background,     // Приложение свернуто (onPause / onStop)
    Foreground,     // Приложение развернуто (onResume)
    Terminating,    // Приложение завершает работу (onDestroy)
    MemoryPressure, // Сигнал от ОС о нехватке памяти (onTrimMemory)
}
```

### Трансляция:
1. Kotlin-Activity перехватывает системное событие жизненного цикла Android.
2. Kotlin вызывает JNI-метод Rust, передавая тип события.
3. Адаптер `platform/adapters/android/lifecycle.rs` маппит это событие на `PlatformLifecycleEvent`.
4. `LifecycleTranslator` публикует соответствующее событие `DomainEvent` (`ApplicationSuspended`, `ApplicationResumed`, `ApplicationClosed`) на шину `EventBus`.

---

## 2. Изоляция Android SDK

Все вызовы к Android Framework (KeyStore, KeyGenParameterSpec, Cipher) выполняются исключительно внутри Kotlin-классов в Gradle-модуле. Нативное Rust-ядро не содержит условной логики для работы с Java-объектами и взаимодействует с Kotlin-мостом через строгие бинарные контракты.
