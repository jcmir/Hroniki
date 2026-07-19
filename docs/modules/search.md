# Документация модуля: Search (Полнотекстовый поиск)

Модуль `search` отвечает за полнотекстовое индексирование и поиск по записям хроники. Реализован с использованием встроенного движка SQLite FTS5 и реактивного EventBus-подписчика.

---

## 1. Назначение

- Мгновенный полнотекстовый поиск по заголовкам и описаниям записей.
- Поиск с учётом тегов записей.
- Фильтрация результатов по конкретному объекту хроники.
- Автоматическое индексирование при создании и обновлении записей (без ручного вызова).

---

## 2. Архитектура

### FTS5 Virtual Table

SQLite FTS5 предоставляет специальную виртуальную таблицу `entries_fts`, связанную с таблицей `entries` через нативные SQL-триггеры. Это обеспечивает базовую синхронизацию при INSERT / UPDATE / DELETE прямо на уровне БД.

Дополнительно `SearchSubscriber` подписывается на EventBus и при событии `EntryCreated` загружает теги записи и обновляет поле `tags` в индексе. Такое двухуровневое решение защищает от рассинхронизации и делает поиск по тегам мгновенным.

### Поток данных:

```
EntryCreated / EntryUpdated
        │
        ▼
    [ EventBus ]
        │
        ▼
[ SearchSubscriber ] — фоновый tokio task
        │
        ▼
[ SearchRepository.index_entry(id, title, description, tags) ]
        │
        ▼
  SQLite: entries_fts (FTS5 virtual table)
```

---

## 3. Основные сущности

- **`SearchQuery`**: Параметры запроса.
  - `text`: строка поиска (обрабатывается движком FTS5).
  - `object_id`: опциональный фильтр по конкретному объекту.

- **`SearchResult`**: Результат поиска.
  - `entry_id`: UUID записи.
  - `object_id`: UUID объекта.
  - `title`: заголовок записи.
  - `snippet`: контекстный фрагмент текста с подсветкой совпадений (FTS5 `snippet()`).
  - `rank`: релевантность (значение FTS5 `rank`).

---

## 4. Public API

### Tauri команда

```
search_entries(query: String, object_id: Option<String>) -> Vec<SearchResultDto>
```

### SearchService

```rust
impl SearchService {
    // Поиск по записям
    pub async fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>, String>;

    // Переиндексация записи (вызывается подписчиком)
    pub async fn reindex_entry(
        &self,
        entry_id: &str,
        object_id: &str,
        title: &str,
        description: Option<&str>,
        tags: &[String],
    ) -> Result<(), String>;

    // Удаление записи из индекса
    pub async fn remove_entry(&self, entry_id: &str) -> Result<(), String>;
}
```

---

## 5. Используемые сервисы

- `SearchRepository`: Выполняет FTS5-запросы и обновляет индекс в SQLite.
- `EventBus`: Источник событий `EntryCreated` и `EntryUpdated` для реактивного индексирования.

---

## 6. Будущие изменения

- Гибридный поиск: комбинирование FTS5-рейтинга с семантической близостью векторных эмбеддингов (Sprint 6.2).
- Поиск по метаданным фотографий и EXIF-данным (Sprint 5.8 / 6.x).
