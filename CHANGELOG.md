# Changelog

Все значимые изменения в проекте ХРОНИКИ документируются здесь.

Формат основан на [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Проект следует [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned
- Sprint 5.7: Reminder Engine (NotificationService, recurrence rules)
- Sprint 5.8: Media Service (previews, EXIF, cleanup)
- Sprint 6.1: Cloud Sync Foundation

---

## [1.0.0] — 2026-07 — Phase 1 MVP

### Added

#### Infrastructure
- SQLite database with `sqlx` and connection pooling
- Tauri 2.x application framework (Rust backend + Web frontend)
- 8 database migrations (`0001`–`0008`)
- Event Bus: tokio broadcast channel for Domain Events (`EventBus`, `DomainEvent`)

#### Core Functionality
- Categories and chronicle objects (CRUD)
- Chronicle entries with date, title, description
- Media attachments (photos per entry)
- Reminders with built-in scheduler
- Tags for entries

#### Search
- Full-text search via SQLite FTS5 (`unicode61` tokenizer)
- `SearchSubscriber`: reactive indexing via EventBus
- Tag-aware reindexing
- `fts_search` Tauri command with optional object filter

#### Security
- PIN protection (PBKDF2-SHA256, 5-attempt lockout, 15s cooldown)
- Identity: Users, Sessions, Recovery Keys
- Encrypted backup (AES-256-GCM + ZIP archive)
- Archive Format v2: `manifest.json` with schema version checking
- Argon2id password hashing

#### Monetization Foundation
- `FeatureGate`: `SubscriptionPlan` enum, `Feature` enum
- `AccountProvider` abstraction (`LocalAccountProvider`)

#### Observability
- `AuditSubscriber`: async EventBus listener → `audit_log` table

#### Documentation (Sprint D1 + D2)
- `docs/product/PRODUCT_SPEC.md` — product specification
- `docs/product/PERSONAS.md` — user personas
- `docs/roadmap/ROADMAP.md` — 4-phase development roadmap
- `docs/security/SECURITY_MODEL.md` — security architecture
- `docs/architecture/ARCHITECTURE.md` — system architecture overview
- `docs/architecture/PROJECT_STATUS.md` — subsystem status tracker
- `docs/architecture/DEPENDENCIES.md` — component dependency graph
- `docs/database/schema.md` — database schema specification
- `docs/modules/` — 6 module guides (account, audit, backup, features, search, security)
- `docs/decisions/` — ADR 0001–0006
- `PROJECT.md` — project passport
- `SUMMARY.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`

#### Engineering
- GitHub Actions CI: fmt + clippy + test + npm build + cargo audit
- `.github/PULL_REQUEST_TEMPLATE.md`
- `.github/ISSUE_TEMPLATE/` (bug report, feature request)
- `.github/CODEOWNERS`

### Architecture Decisions
- ADR 0001: Feature Gate (SubscriptionPlan-based)
- ADR 0002: AccountProvider abstraction
- ADR 0003: Backup v2 with manifest
- ADR 0004: Event Bus (tokio broadcast)
- ADR 0005: Audit Log via EventBus subscriber
- ADR 0006: Full-text search via SQLite FTS5

---

## Links

[Unreleased]: https://github.com/jcmir/Hroniki/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/jcmir/Hroniki/releases/tag/v1.0.0
