# ADR 0017: Production Android Runtime & Kotlin JNI Bridge Topology

## Контекст и Проблема
В Спринте 5.12 была введена абстракция мобильного runtime и mock-адаптеры для тестирования на Desktop. Для перехода к реальной эксплуатации на устройствах Android необходимо развернуть производственную структуру Gradle-проекта, подключить нативные Kotlin-мосты (JNI Bridges) и настроить AndroidManifest с системными разрешениями и `BroadcastReceiver`'ами.

При этом необходимо строго сохранить чистоту доменного ядра Rust: код на Rust не должен содержать зависимостей от `Activity`, `Context`, `Intent` или системных Java-классов Android SDK.

## Рассматриваемые Варианты

1. **Прямые JNI-вызовы из Rust в системные Android API**:
   - *Плюсы*: Отсутствие промежуточного Kotlin-кода.
   - *Минусы*: Хрупкие C/JNI вызовы, сложности обработки исключений Java, жесткая привязка к версиям Android SDK в Rust коде.

2. **Строгая топология Kotlin JNI Bridge (Выбран)**:
   - *Архитектура*:
     ```
     Rust Domain ──► Platform Traits ──► Android Adapters ──► Kotlin JNI Bridges ──► Android OS
     ```
   - *Плюсы*: Kotlin-мосты скрывают особенности Android SDK (`NotificationManager`, `AlarmManager`, `KeyStore`, runtime permissions). Rust взаимодействует исключительно через тонкие C-JNI экспорты и DTO.

## Решение
Принято решение реализовать production runtime топологию:
- Создается структура Android проекта в `src-tauri/gen/android/`.
- Выделены изолированные Kotlin-мосты в пакете `com.hroniki.app`:
  - `KeyStoreBridge.kt`: Безопасные вызовы Android KeyStore (TEE/StrongBox).
  - `LifecycleBridge.kt`: Передача системных событий жизненного цикла.
  - `NotificationBridge.kt`: Создание каналов уведомлений (`hroniki_reminders`) и публикация сообщений.
  - `AlarmBridge.kt` + `ReminderReceiver.kt`: Планирование точных алармов `AlarmManager` и восстановление после перезагрузки (`BOOT_COMPLETED`).
  - `PermissionBridge.kt`: Запрос нативных разрешений Android.

## Последствия
- Полная изоляция Rust доменного ядра от специфики Android SDK.
- Поддержка компиляции и автоматизированного тестирования на Android Эмуляторах и реальных устройствах.
- Защищенность от изменений API Android SDK за счет абстракции Kotlin-мостов.
