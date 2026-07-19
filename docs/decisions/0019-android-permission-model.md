# ADR 0019: Гранулярная Модель Нативных Разрешений (Granular Permission Model)

## Контекст и Проблема
В Android система разрешений включает не только бинарные статусы (`Granted` / `Denied`), но и критические состояния, влияющие на UX приложения:
- `PermanentlyDenied` ("Больше не спрашивать" / "Don't ask again").
- `Unsupported` (отсутствие аппаратного чипа биометрии или неподдерживаемая версия Android SDK).

Для предоставления корректного UI/UX (например, перенаправления пользователя в настройки системы вместо бесконечных повторных запросов) платформенный слой Rust должен точно отражать эти состояния.

Кроме того, статус аппаратных возможностей (`PlatformCapabilities`) может изменяться во времени (например, после того как пользователь вручную включил разрешение на уведомления в настройках OS).

## Решение

### 1. Расширение `PermissionStatus`:
```rust
pub enum PermissionStatus {
    Granted,
    Denied,
    PermanentlyDenied,
    Unsupported,
}
```

### 2. Динамический `PlatformCapabilitiesProvider` (Debt-1):
Внедрить абстракцию `PlatformCapabilitiesProvider`:
```rust
#[async_trait]
pub trait PlatformCapabilitiesProvider: Send + Sync {
    async fn refresh(&self) -> Result<PlatformCapabilities, String>;
    fn current(&self) -> PlatformCapabilities;
}
```

## Последствия
- Позволяет UI корректно обрабатывать сценарии "Больше не спрашивать" с перенаправлением в системные настройки Android.
- Обеспечивает динамическое обновление доступных функций при изменении разрешений в OS.
