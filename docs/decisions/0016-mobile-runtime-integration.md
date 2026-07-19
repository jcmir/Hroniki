# ADR 0016: Интеграция Мобильного Runtime (Notifications, AlarmManager, SAF & Session Locking)

## Контекст и Проблема
Мобильная платформа Android требует взаимодействия с системными сервисами OS:
1. Доставка мгновенных/локальных уведомлений через каналы уведомлений (`NotificationChannel`).
2. Точное и приближенное планирование системных будильников и напоминаний (`AlarmManager`).
3. Запрос и проверка нативных разрешений Android (`POST_NOTIFICATIONS` в Android 13+, `SCHEDULE_EXACT_ALARM`, доступ к хранилищу).
4. Резервное копирование и восстановление данных через Storage Access Framework (SAF) без проникновения специфичных системных URI (`content://`) в чистое Rust-ядро (Domain Core).
5. Очистка sensitive данных в оперативной памяти (RAM) при блокировке устройства / экрана (`DomainEvent::ApplicationLocked`).

## Рассматриваемые Варианты

1. **Прямое проникновение Android Java/JNI API во все слои приложения**:
   - *Плюсы*: Быстрый прототип.
   - *Минусы*: Нарушение принципа чистой архитектуры. Код становится непригодным для кросс-платформенной компиляции под Desktop и тестирования без эмулятора.

2. **Строгая слоистая изоляция Platform Adapters (Выбран)**:
   - *Notification Delivery (`AndroidNotificationPlatform`)*: отвечает только за мгновенную доставку через каналы уведомлений.
   - *Schedule Service (`AndroidSchedulePlatform`)*: управляет точными/приближенными алармами `AlarmManager`.
   - *Storage Access Framework (`AndroidStorageAdapter`)*: выполняет роль границы для SAF URI, работая исключительно с байтовыми потоками/буферами без просачивания `content://` в домен.
   - *Session Lock (`SessionManager`)*: при получении `DomainEvent::ApplicationLocked` полностью очищает RAM-токены и расшифрованные данные из RAM, оставляя ключи и БД на диске нетронутыми.

## Решение
Принято решение реализовать интеграцию системных сервисов Android через изолированные платформенные адаптеры с расширением флагов `PlatformCapabilities` (`notifications`, `exact_alarms`, `saf_backup`, `biometric`, `strongbox`, `secure_hardware`).

### Ключевые компоненты:
- **`PlatformCapabilities`**: расширен булевыми флагами поддержки точных алармов (`exact_alarms`) и SAF бэкапов (`saf_backup`).
- **`SchedulePlatform` & `AndroidSchedulePlatform`**: абстракция и адаптер для точного планирования системных алармов.
- **`PermissionPlatform` & `AndroidPermissionPlatform`**: поддержка `POST_NOTIFICATIONS` и `Storage`.
- **`AndroidStorageAdapter`**: изолированная граница бэкапов SAF.
- **`SessionManager`**: подписчик на `EventBus`, очищающий RAM-токены и кэши при вызове `DomainEvent::ApplicationLocked`.

## Последствия
- Полная тестируемость доменного ядра на Desktop и в CI/CD без использования эмулятора Android.
- Строгое соблюдение Android 13+ требований безопасности (кастомные каналы уведомлений, гранулярные разрешения).
- Защищенность данных пользователя в памяти при блокировке экрана.
