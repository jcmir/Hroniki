# ХРОНИКИ — Roadmap

**Версия:** 1.0  
**Обновлён:** 2026-07

---

## Обзор

```
Phase 1 — MVP          ✅ Завершена
Phase 2 — Monetization 🔄 В работе
Phase 3 — Enterprise   📋 Planned
Phase 4 — Ecosystem    🔮 Vision
```

---

## ✅ Phase 1 — MVP (Завершена)

Цель: рабочее приложение с core-функционалом, готовое к первым пользователям.

### Инфраструктура
- [x] SQLite база данных с миграциями (8 схем)
- [x] Tauri (Rust backend + Web frontend)
- [x] Криптографический vault (AES-256-GCM)
- [x] Event Bus (tokio broadcast, Domain Events)

### Core функционал
- [x] Категории и объекты хроники
- [x] Записи с датой, описанием, фотографиями
- [x] Напоминания с планировщиком
- [x] Теги для записей
- [x] Полнотекстовый поиск (SQLite FTS5, unicode61)

### Безопасность
- [x] PIN-защита (PBKDF2, 5-fail lockout)
- [x] Identity: пользователи, сессии, recovery keys
- [x] Зашифрованный бэкап (AES-256-GCM + ZIP)
- [x] Archive Format v2 с manifest.json и миграцией БД

### Монетизация (фундамент)
- [x] Feature Gate: SubscriptionPlan, Feature enum
- [x] AccountProvider абстракция

### Наблюдаемость
- [x] Audit Log: асинхронный подписчик на EventBus

### Документация
- [x] Passport проекта (`PROJECT.md`)
- [x] Architecture overview (`ARCHITECTURE.md`)
- [x] Database schema (`schema.md`)
- [x] Module guides (6 модулей)
- [x] ADR 0001–0006
- [x] PROJECT_STATUS.md, DEPENDENCIES.md

---

## 🔄 Phase 2 — Monetization (В работе)

Цель: превратить бесплатный продукт в платформу с устойчивой монетизацией.

### Sprint 5.7 — Reminder Engine (Planned)
- [ ] `NotificationService` — интеграция с системными уведомлениями Tauri
- [ ] Webhook-напоминания (Email / Telegram bot)
- [ ] Повторяющиеся напоминания (recurrence rules)

### Sprint 5.8 — Media Service (Planned)
- [ ] Preview генерация (thumbnails)
- [ ] EXIF-метаданные (дата, геолокация)
- [ ] Cleanup orphaned media
- [ ] Лимиты медиа по тарифу

### Sprint 6.1 — Cloud Sync Foundation (Planned)
- [ ] `CloudAccountProvider` абстракция
- [ ] Sync Protocol: conflict resolution (last-write-wins v1)
- [ ] End-to-end encryption перед отправкой на сервер
- [ ] REST API v1 (Rust axum или Go)

### Sprint 6.2 — AI Foundation / RAG (Planned)
- [ ] Embedding pipeline: записи → векторы
- [ ] Vector Store: SQLite-vss или DuckDB
- [ ] Local inference: Ollama интеграция
- [ ] `SearchService` расширение: семантический поиск
- [ ] AI-ответ с источником (citation)

### Sprint 6.3 — Smart Search (Planned)
- [ ] Гибридный поиск: FTS5 + векторная близость
- [ ] Фильтры: дата, объект, теги, тип медиа
- [ ] История поисковых запросов

---

## 📋 Phase 3 — Enterprise (Planned)

Цель: войти в корпоративный сегмент с compliance-возможностями.

### Organization Accounts
- [ ] Tenant isolation (организации как root-сущность)
- [ ] SSO-интеграция (SAML 2.0 / OIDC)
- [ ] Управление ролями (RBAC)

### Audit Compliance
- [ ] Tamper-evident Audit Log (цепочки хэшей)
- [ ] GDPR Data Export / Right to Erasure
- [ ] SOC 2 Type II подготовка

### Encryption Policies
- [ ] Bring-Your-Own-Key (BYOK)
- [ ] Policy enforcement: требования к PIN, сессиям
- [ ] Hardware Security Module (HSM) поддержка

### Device Management
- [ ] Remote wipe сессии
- [ ] Trusted device registry
- [ ] Geo-restricted access

---

## 🔮 Phase 4 — Ecosystem (Vision)

Цель: ХРОНИКИ как платформа для партнёров и интеграций.

### Mobile
- [ ] Android (Tauri Mobile)
- [ ] iOS (Tauri Mobile)
- [ ] Offline-first sync с конфликт-резолюцией

### API & Integrations
- [ ] Public REST API v2
- [ ] Webhooks (события в сторонние системы)
- [ ] Zapier / n8n интеграция

### Ecosystem
- [ ] Plugin SDK (сторонние провайдеры хранения)
- [ ] Marketplace расширений
- [ ] White-label для партнёров

---

## Метрики успеха по фазам

| Метрика | Phase 1 | Phase 2 | Phase 3 |
|---------|---------|---------|---------|
| MAU | 100 | 10 000 | 100 000 |
| Платящих пользователей | — | 500 | 5 000 |
| MRR | $0 | $3 000 | $40 000 |
| Uptime (Cloud) | N/A | 99.5% | 99.9% |
