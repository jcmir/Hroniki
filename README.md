<div align="center">

# ХРОНИКИ

**Приватная структурированная память — для людей и организаций**

[![CI](https://github.com/jcmir/Hroniki/actions/workflows/ci.yml/badge.svg)](https://github.com/jcmir/Hroniki/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.78+-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-2.x-blue.svg)](https://tauri.app)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

</div>

---

## Что такое ХРОНИКИ?

ХРОНИКИ — это **local-first, зашифрованное** приложение для ведения структурированной хроники событий вокруг объектов вашей жизни: людей, животных, недвижимости, техники, бизнеса.

Не дневник. Не заметки. **Структурированная память с полнотекстовым поиском, напоминаниями и зашифрованными бэкапами.**

```
Объект: "Яблоня в саду"
  │
  ├── 2024-03-15  Обрезка ветвей
  ├── 2024-05-01  Первое цветение
  ├── 2024-07-20  Обработка от вредителей  [фото]
  └── 2024-09-10  Урожай: 12 кг
```

---

## Ключевые возможности

| Возможность | Описание |
|-------------|----------|
| 🗂 **Объекты и категории** | Структурируйте хронику по объектам |
| 📝 **Записи с медиа** | Текст, фотографии, теги |
| 🔍 **Полнотекстовый поиск** | FTS5 с поддержкой русского языка |
| 🔔 **Напоминания** | Встроенный планировщик |
| 🔐 **PIN-защита** | PBKDF2, блокировка после 5 попыток |
| 💾 **Зашифрованный бэкап** | AES-256-GCM, ваш ключ — только у вас |
| 📦 **Offline-first** | Работает без интернета |
| 📊 **Аудит-лог** | Журнал критических действий |

---

## Архитектура

```
┌─────────────────────────────────────────┐
│           Frontend (Web/TS)             │
└───────────────────┬─────────────────────┘
                    │ Tauri IPC
┌───────────────────▼─────────────────────┐
│           Rust Backend                  │
│                                         │
│  account ── features ── audit           │
│     │           │          │            │
│  identity    subscription  EventBus     │
│     │                      │            │
│  security              search (FTS5)    │
│     │                                   │
│  storage (SQLite · 8 migrations)        │
└─────────────────────────────────────────┘
```

**Стек:**
- **Backend:** Rust + Tauri 2.x
- **База данных:** SQLite (sqlx) + FTS5
- **Шифрование:** AES-256-GCM (архивы), Argon2id (пароли), PBKDF2 (PIN)
- **Архитектура:** Event-Driven (tokio broadcast EventBus)
- **Frontend:** TypeScript + Web

---

## Тарифы

| Функция | Free | Pro | Family | Enterprise |
|---------|:----:|:---:|:------:|:----------:|
| Объектов | 10 | ∞ | ∞ | ∞ |
| Зашифрованный бэкап | ✅ | ✅ | ✅ | ✅ |
| Полнотекстовый поиск | ✅ | ✅ | ✅ | ✅ |
| Cloud Sync | ❌ | ✅ | ✅ | ✅ |
| AI-ассистент | ❌ | ✅ | ✅ | ✅ |
| Семейный архив | ❌ | ❌ | ✅ | ✅ |
| Аудит-лог | ❌ | ❌ | ❌ | ✅ |

---

## Быстрый старт

### Требования

- Rust 1.78+
- Node.js 20+
- Tauri CLI v2

### Запуск в режиме разработки

```bash
git clone https://github.com/jcmir/Hroniki.git
cd Hroniki
npm install
npm run tauri dev
```

### Тесты

```bash
cd src-tauri
cargo test        # запуск всех тестов
cargo clippy      # линтер
cargo fmt         # форматирование
```

---

## Документация

| Документ | Описание |
|----------|----------|
| [PROJECT.md](docs/PROJECT.md) | Паспорт проекта |
| [ARCHITECTURE.md](docs/architecture/ARCHITECTURE.md) | Архитектурный обзор |
| [SECURITY_MODEL.md](docs/security/SECURITY_MODEL.md) | Модель безопасности |
| [PRODUCT_SPEC.md](docs/product/PRODUCT_SPEC.md) | Продуктовая спецификация |
| [ROADMAP.md](docs/roadmap/ROADMAP.md) | Roadmap развития |
| [PROJECT_STATUS.md](docs/architecture/PROJECT_STATUS.md) | Статус подсистем |
| [decisions/](docs/decisions/) | Architecture Decision Records (ADR) |

---

## Статус разработки

Phase 1 (MVP) — ✅ **Завершена**

```
✅ Core (категории, объекты, записи)
✅ Медиавложения
✅ Полнотекстовый поиск (FTS5)
✅ Напоминания
✅ Identity (пользователи, сессии, recovery)
✅ Security (PIN, AES-256 бэкапы)
✅ Feature Gate (тарифы)
✅ Event Bus (Domain Events)
✅ Audit Log
```

Phase 2 (Monetization) — 🔄 В работе

---

## Безопасность

ХРОНИКИ реализует многоуровневую защиту:

- **Argon2id** — хеширование паролей
- **AES-256-GCM** — шифрование архивов (authenticated encryption)
- **PBKDF2-SHA256** — PIN-код
- **Zero-knowledge** — сервер никогда не видит незашифрованные данные
- **Параметризованные запросы** — защита от SQL Injection

[Подробная модель безопасности →](docs/security/SECURITY_MODEL.md)

---

## Contributing

Смотрите [PULL_REQUEST_TEMPLATE.md](.github/PULL_REQUEST_TEMPLATE.md).

CI автоматически проверяет:
- `cargo fmt`
- `cargo clippy -- -D warnings`
- `cargo test`
- `npm run build`

---

<div align="center">

Сделано с ❤️ для тех, кому важна собственная история.

</div>
