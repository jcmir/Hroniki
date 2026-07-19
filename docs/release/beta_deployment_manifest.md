# Beta Deployment Manifest — HRONIKI 0.2.1-beta

## Метаданные выпуска

| Параметр | Значение |
|---|---|
| Имя приложения | **ХРОНИКИ** |
| Идентификатор (Package ID) | `app.hroniki.mobile` |
| Версия (versionName) | `0.2.1-beta` |
| Сборка (versionCode) | `21` |
| Дата релиза | `2026-07-20` |
| Требуемый Android | Android 7.0+ (API level 24+) |
| Целевой Android | Android 14 (API level 34) |
| Архитектуры (ABI) | `arm64-v8a`, `x86_64` |

---

## Профиль Безопасности & Хранения

- **Local-first**: Все данные хранятся локально в шифруемом файле SQLite `chronology.sqlite`.
- **Сеть**: Сетевые соединения отсутствуют. Приложение полностью автономно (zero telemetry, zero analytics).
- **Медиа**: Фотографии сохраняются во внутреннем приватном хранилище приложения без публичной видимости.
- **Резервное копирование**: Поддерживается шифрованный экспорт/импорт всего архива в формате `.hroniki` (AES-256-GCM + PBKDF2 / Argon2id).

---

## Чек-лист Развёртывания на Устройстве

1. **Сборка APK**:
   ```bash
   tauri android build --target aarch64-linux-android
   ```
2. **Установка**:
   ```bash
   adb install -r src-tauri/gen/android/app/build/outputs/apk/release/app-release.apk
   ```
3. **Проверка маркера сборки**:
   - Открыть Настройки → нижняя карточка.
   - Убедиться, что выводится: `Beta 0.2.1`, `Сборка #21 (2026-07-20)`, `app.hroniki.mobile`.
4. **Валидация QA**:
   - Выполнить [smoke_flow_script.md](file:///c:/bylo/docs/testing/smoke_flow_script.md)
   - Выполнить [ux_bug_checklist.md](file:///c:/bylo/docs/testing/ux_bug_checklist.md)
   - Выполнить [backup_restore_device_test.md](file:///c:/bylo/docs/testing/backup_restore_device_test.md)

---

## Обратная связь от тестировщиков

При возникновении проблем попросите тестировщика отправить:
1. Значение маркера сборки из **Настроек**.
2. Аварийный лог из раздела **Настройки → 🛡 Диагностика** (если приложение закрылось со сбоем).
