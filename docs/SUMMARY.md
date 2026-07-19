# Содержание (SUMMARY)

Добро пожаловать в базу знаний проекта **ХРОНИКИ**. Ниже представлена структура доступной документации:

---

## 📋 Общие сведения
- [Паспорт проекта (PROJECT.md)](file:///c:/bylo/docs/PROJECT.md) — общие принципы, структура репозитория, сборка и запуск.
- [Статус подсистем (PROJECT_STATUS.md)](file:///c:/bylo/docs/architecture/PROJECT_STATUS.md) — текущее состояние и планы внедрения модулей.

---

## 🏛️ Архитектура системы
- [Архитектурные слои (ARCHITECTURE.md)](file:///c:/bylo/docs/architecture/ARCHITECTURE.md) — слои приложения и жизненный цикл событий.
- [Карта зависимостей и процессов (DEPENDENCIES.md)](file:///c:/bylo/docs/architecture/DEPENDENCIES.md) — визуализация зависимостей, диаграммы последовательностей (Sequence) и Event Flow.

---

## 💾 Спецификации хранения
- [Схема базы данных (schema.md)](file:///c:/bylo/docs/database/schema.md) — описание SQLite таблиц, внешних ключей и лога миграций.

---

## ⚙️ Документация модулей
- [Учетные записи (account.md)](file:///c:/bylo/docs/modules/account.md) — авторизация, сессии и AccountProvider.
- [Права доступа (features.md)](file:///c:/bylo/docs/modules/features.md) — Feature Gate и SubscriptionPlan.
- [Резервное копирование (backup.md)](file:///c:/bylo/docs/modules/backup.md) — архив v2 с манифестом и контроль совместимости.
- [Безопасность (security.md)](file:///c:/bylo/docs/modules/security.md) — PIN-код, PBKDF2, Argon2id и AES-GCM.

---

## 📝 Журнал архитектурных решений (ADR)
- [ADR 0001: Система Feature Gate](file:///c:/bylo/docs/decisions/0001-feature-gate.md)
- [ADR 0002: Абстракция AccountProvider](file:///c:/bylo/docs/decisions/0002-account-provider.md)
- [ADR 0003: Манифест архива Backup v2](file:///c:/bylo/docs/decisions/0003-backup-v2.md)
- [ADR 0004: Tokio-broadcast Event Bus](file:///c:/bylo/docs/decisions/0004-event-bus.md)
