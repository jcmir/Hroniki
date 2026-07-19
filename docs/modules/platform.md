# Документация модуля: Platform (Платформенные абстракции)

Модуль `platform` абстрагирует платформозависимые системные возможности операционных систем от бизнес-логики приложения. Это исключает дублирование макросов условной компиляции `#[cfg]` и обеспечивает легкое мокирование для тестов.

---

## 1. Компоненты контекста платформы

Вся системная интеграция объединена в `PlatformContext`:

### `NotificationPlatform`
Трейт для вывода локальных уведомлений в ОС.
```rust
#[async_trait]
pub trait NotificationPlatform: Send + Sync {
    async fn show(&self, title: &str, body: Option<&str>) -> Result<(), String>;
}
```

### `SecureStoragePlatform`
Абстракция над системным Keyring/Keychain (Desktop) и Keystore (Android) для хранения секретов с поддержкой пространств имен (`SecretIdentifier`).
```rust
#[async_trait]
pub trait SecureStoragePlatform: Send + Sync {
    async fn store(&self, id: SecretIdentifier, value: &[u8]) -> Result<(), String>;
    async fn load(&self, id: SecretIdentifier) -> Result<Option<Vec<u8>>, String>;
    async fn delete(&self, id: SecretIdentifier) -> Result<(), String>;
}
```

### `PermissionPlatform`
Унифицированный интерфейс проверки и запроса прав ОС.
```rust
#[async_trait]
pub trait PermissionPlatform: Send + Sync {
    async fn check_permission(&self, kind: PermissionKind) -> Result<PermissionStatus, String>;
    async fn request_permission(&self, kind: PermissionKind) -> Result<PermissionStatus, String>;
}
```

---

## 2. Жизненный цикл ОС и События Домена

Модуль `lifecycle` содержит `LifecycleTranslator`, который переводит низкоуровневые события операционной системы в высокоуровневые доменные события, транслируемые через `EventBus`:

| Событие ОС (LifecycleEvent) | Доменное событие (DomainEvent) | Описание действия ядра |
|---|---|---|
| `AppStarted` | `ApplicationStarted` | Инициализация SQLite пула и миграций |
| `AppBackgrounded` | `ApplicationSuspended` | Блокировка UI PIN-кодом, сброс кэшей |
| `AppResumed` | `ApplicationResumed` | Проверка PIN-кода, возобновление шедулера |
| `AppClosed` | `ApplicationClosed` | Корректное закрытие пула БД |
