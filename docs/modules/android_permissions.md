# Документация модуля: Android Permissions & Capabilities

Модуль определяет систему управления разрешениями операционной системы и динамический реестр возможностей платформы `PlatformCapabilitiesProvider`.

---

## 1. Состояния Разрешений (`PermissionStatus`)

```rust
pub enum PermissionStatus {
    Granted,
    Denied,
    PermanentlyDenied,
    Unsupported,
}
```

- `Granted`: Разрешение предоставлено пользователем.
- `Denied`: Разрешение отклонено, возможно повторное запрашивание.
- `PermanentlyDenied`: Пользователь выбрал "Больше не спрашивать". UI перенаправляет в настройки OS.
- `Unsupported`: Фича не поддерживается устройством или версией Android.

---

## 2. Динамический Провайдер Возможностей (`PlatformCapabilitiesProvider`)

Динамический провайдер позволяет обновлять доступные способности приложения (`PlatformCapabilities`) во время работы (например, при предоставлении разрешения на уведомления в системных настройках):

```rust
pub trait PlatformCapabilitiesProvider: Send + Sync {
    async fn refresh(&self) -> Result<PlatformCapabilities, String>;
    fn current(&self) -> PlatformCapabilities;
}
```
