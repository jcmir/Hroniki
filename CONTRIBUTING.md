# Contributing to ХРОНИКИ

Спасибо за интерес к проекту. Это руководство описывает процесс участия в разработке.

---

## Принципы

1. **Документация до кода (DDD)**: Прежде чем писать код нового модуля, создайте документацию и ADR.
2. **Тесты обязательны**: Каждый новый публичный метод должен иметь интеграционный тест.
3. **Clippy — закон**: `cargo clippy -- -D warnings` должен проходить без ошибок.
4. **Архитектура важнее скорости**: Если не уверен — создай ADR и обсуди.

---

## Процесс разработки

### 1. Планирование (для значительных изменений)
- Создай Issue с описанием проблемы или фичи.
- Если это архитектурное решение — создай ADR в `docs/decisions/`.
- Для нового модуля — создай документацию в `docs/modules/`.

### 2. Ветвление
```bash
# Для фичи
git checkout -b feature/sprint-X.Y-description

# Для исправления
git checkout -b fix/short-description

# Для документации
git checkout -b docs/description
```

### 3. Разработка
```bash
# Запуск тестов
cd src-tauri && cargo test

# Проверка clippy (0 предупреждений)
cargo clippy -- -D warnings

# Форматирование
cargo fmt
```

### 4. Создание PR
Используй шаблон [`.github/PULL_REQUEST_TEMPLATE.md`](.github/PULL_REQUEST_TEMPLATE.md).

---

## Формат коммитов

Мы используем [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(scope): краткое описание

[опциональное тело]

[опциональный footer]
```

### Типы:
| Тип | Когда использовать |
|-----|--------------------|
| `feat` | Новая функция |
| `fix` | Исправление ошибки |
| `docs` | Только документация |
| `refactor` | Рефакторинг без изменения поведения |
| `test` | Добавление или исправление тестов |
| `chore` | Зависимости, CI, конфиги |
| `perf` | Улучшение производительности |

### Примеры:
```
feat(search): add FTS5 full-text search with unicode61 tokenizer
fix(backup): prevent pool reconnection on import failure
docs(adr): add ADR-0006 for FTS5 search decision
refactor(identity): extract Crypto errors to IdentityError::Crypto
```

---

## Архитектурные принципы

### Слоистая архитектура
```
commands/   ← Tauri IPC handlers (тонкий слой, только маппинг)
     │
services/   ← Бизнес-логика
     │
repositories/ ← Абстракция доступа к данным (trait)
     │
storage/    ← SQLite реализация
```

Правила:
- `commands` не содержит бизнес-логики.
- `service` не импортирует `sqlx` напрямую.
- `storage` реализует traits из `repository`.

### Event-Driven
Побочные эффекты (аудит, поиск, будущий sync) реализуются через `EventBus`, а не прямыми вызовами.

### Новый модуль — чек-лист
- [ ] `docs/modules/<name>.md` создан
- [ ] ADR создан (если архитектурное решение)
- [ ] Trait определён отдельно от реализации
- [ ] SQLite-реализация в `storage/`
- [ ] Миграция создана (если нужна)
- [ ] Интеграционные тесты написаны
- [ ] `PROJECT_STATUS.md` обновлён

---

## Стиль Rust

- Используй `async_trait` для асинхронных трейтов.
- Ошибки — `String` на публичном API команд, кастомный `Error` enum внутри модулей.
- Никогда не используй `unwrap()` в production-коде (только в тестах).
- Предпочитай `Arc<dyn Trait>` над дженериками в структурах состояния.

---

## CI обязателен

Все PR проходят через GitHub Actions:
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `npm run build`
- `cargo audit` (non-blocking)

PR не может быть слит, если CI красный.
