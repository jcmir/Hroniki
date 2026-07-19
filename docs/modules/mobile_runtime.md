# Спецификация модуля: Mobile Runtime Integration

Модуль `mobile_runtime` отвечает за интеграцию системных сервисов мобильной операционной системы Android с платформенно-независимым доменным ядром приложения.

---

## 1. Архитектура Мобильного Runtime

```
                           Domain Core
                                │
                         Platform Context
                                │
      ┌─────────────────────────┼─────────────────────────┐
      ▼                         ▼                         ▼
Notification Delivery      Schedule Service          Storage & SAF Backup
(AndroidNotification)     (AndroidSchedule)         (AndroidStorageAdapter)
      │                         │                         │
      └─────────────────────────┼─────────────────────────┘
                                ▼
                       Android OS Runtime
```

### Разделение Ответственностей:
1. **Notification Delivery (`AndroidNotificationPlatform`)**: Доставка мгновенных/локальных уведомлений через `NotificationChannel`.
2. **Schedule Service (`AndroidSchedulePlatform`)**: Планирование системных будильников и напоминаний через `AlarmManager`.
3. **Storage & SAF Boundary (`AndroidStorageAdapter`)**: Безопасный экспорт/импорт шифрованных архивов без просачивания `content://` URI в доменный слой.
4. **Session Lock (`SessionManager`)**: Безопасная очистка RAM-токенов и разшифрованного кэша при событии блокировки (`DomainEvent::ApplicationLocked`).

---

## 2. Разрешения Android (Native Permissions)

Система разрешений маппится через перечисление `PermissionKind`:

| `PermissionKind` | Системное разрешение Android | Назначение |
|---|---|---|
| `Notifications` | `android.permission.POST_NOTIFICATIONS` | Отправка локальных и Push уведомлений (Android 13+) |
| `Storage` | `READ_EXTERNAL_STORAGE` / SAF | Выбор и сохранение архивов бэкапов |
| `ExactAlarms` | `android.permission.SCHEDULE_EXACT_ALARM` | Планирование точных будильников `AlarmManager` |
| `Biometrics` | `USE_BIOMETRIC` | Биометрическая аутентификация |
| `Camera` | `android.permission.CAMERA` | Съемка фото к записям |

---

## 3. Сброс Памяти при Блокировке (Session Locking)

При блокировке экрана Android генерирует системное событие `PlatformLifecycleEvent::Locked`, которое маппится на `DomainEvent::ApplicationLocked`.

```
ApplicationLocked ──► SessionManager ──► Clear RAM tokens & decrypted cache
```

### Правила безопасность session lock:
- Вся оперативная память, содержащая активные Session Tokens и расшифрованные структуры данных, сбрасывается.
- Главные ключи шифрования в KeyStore и зашифрованный файл базы данных SQLite на диске **не затрагиваются**.
