# Спецификация SQLite таблиц и миграций

Вся база данных приложения хранится локально в файле `chronology.sqlite` внутри пользовательской директории AppData. База данных спроектирована со строгой проверкой внешних ключей (`foreign_keys = true`).

---

## 1. Схема таблиц базы данных

### 1.1 Таблица `users`
Хранит учетные записи локальных пользователей.
- `id` (TEXT PRIMARY KEY, NOT NULL): Уникальный UUID пользователя.
- `email` (TEXT UNIQUE): Электронная почта.
- `display_name` (TEXT): Имя пользователя.
- `password_hash` (TEXT): Хеш пароля, выработанный по схеме `Argon2id`.
- `created_at` (TEXT NOT NULL): Дата создания в формате ISO-8601.
- `updated_at` (TEXT NOT NULL): Дата обновления в формате ISO-8601.

### 1.2 Таблица `user_sessions`
Сессии авторизованных клиентских устройств.
- `id` (TEXT PRIMARY KEY, NOT NULL): Уникальный UUID сессии.
- `user_id` (TEXT NOT NULL): Связь с `users.id` (внешний ключ `ON DELETE CASCADE`).
- `device_name` (TEXT): Имя/тип устройства.
- `created_at` (TEXT NOT NULL): Дата открытия сессии.
- `expires_at` (TEXT): Дата истечения сессии.

### 1.3 Таблица `recovery_keys`
Хеш ключей восстановления доступа к зашифрованным контейнерам.
- `id` (TEXT PRIMARY KEY, NOT NULL): Уникальный UUID записи.
- `user_id` (TEXT NOT NULL): Связь с `users.id` (внешний ключ `ON DELETE CASCADE`).
- `key_hash` (TEXT NOT NULL): Хеш ключа восстановления.
- `created_at` (TEXT NOT NULL): Дата генерации ключа.
- `used_at` (TEXT): Дата применения ключа восстановления.

### 1.4 Таблица `subscriptions`
Биллинг-тарифы и Feature Gate права пользователей.
- `user_id` (TEXT PRIMARY KEY, NOT NULL): Связь с `users.id` (внешний ключ `ON DELETE CASCADE`).
- `plan` (TEXT NOT NULL): Выбранный тариф (`free`, `pro`, `family`, `enterprise`).
- `status` (TEXT NOT NULL): Статус подписки (`Active`, `Canceled`, `Expired`).
- `expires_at` (TEXT): Дата окончания действия подписки.
- `updated_at` (TEXT NOT NULL): Дата обновления статуса.

### 1.5 Таблица `categories`
Категории объектов хронологии (Сад, Дом, Здоровье и т.д.).
- `id` (TEXT PRIMARY KEY, NOT NULL): Уникальный UUID категории.
- `name` (TEXT UNIQUE, NOT NULL): Уникальное наименование категории на русском языке.
- `created_at` (TEXT NOT NULL): Дата создания.

### 1.6 Таблица `objects`
Объекты хроники (Яблоня, Автомобиль, Здоровье кошки).
- `id` (TEXT PRIMARY KEY, NOT NULL): Уникальный UUID объекта.
- `category_id` (TEXT NOT NULL): Связь с `categories.id` (внешний ключ `ON DELETE RESTRICT`).
- `name` (TEXT NOT NULL): Наименование объекта.
- `description` (TEXT): Описание объекта.
- `created_at` (TEXT NOT NULL): Дата заведения хроники.

### 1.7 Таблица `entries`
События хронологической ленты.
- `id` (TEXT PRIMARY KEY, NOT NULL): Уникальный UUID события.
- `object_id` (TEXT NOT NULL): Связь с `objects.id` (внешний ключ `ON DELETE CASCADE`).
- `occurred_at` (TEXT NOT NULL): Дата и время совершения события.
- `title` (TEXT NOT NULL): Краткий заголовок события.
- `description` (TEXT): Подробное текстовое описание события.
- `created_at` (TEXT NOT NULL): Дата записи.
- `updated_at` (TEXT NOT NULL): Дата обновления.

### 1.8 Таблица `entry_photos`
Связи записей событий с медиафайлами.
- `id` (TEXT PRIMARY KEY, NOT NULL): Уникальный UUID фотографии.
- `entry_id` (TEXT NOT NULL): Связь с `entries.id` (внешний ключ `ON DELETE CASCADE`).
- `file_path` (TEXT NOT NULL): Относительный путь к оригинальному файлу изображения.
- `thumbnail_path` (TEXT NOT NULL): Относительный путь к уменьшенной копии (эскизу).
- `created_at` (TEXT NOT NULL): Дата добавления.

### 1.9 Таблица `reminders`
События и триггеры плановых напоминаний.
- `entry_id` (TEXT PRIMARY KEY, NOT NULL): Связь с `entries.id` (внешний ключ `ON DELETE CASCADE`).
- `due_date` (TEXT NOT NULL): Дата планового срабатывания напоминания.
- `interval_days` (INTEGER): Интервал повторения в днях (для периодических напоминаний).
- `completed_at` (TEXT): Дата выполнения напоминания.

### 1.10 Таблицы `tags` и `entry_tags`
Тегирование ленты событий.
- `tags`:
  - `id` (TEXT PRIMARY KEY, NOT NULL): Уникальный UUID тега.
  - `name` (TEXT UNIQUE, NOT NULL): Наименование тега (например, `удобрение`, `ремонт`).
- `entry_tags` (Связующая таблица многие-ко-многим):
  - `entry_id` (TEXT NOT NULL): Связь с `entries.id` (внешний ключ `ON DELETE CASCADE`).
  - `tag_id` (TEXT NOT NULL): Связь с `tags.id` (внешний ключ `ON DELETE CASCADE`).
  - *Ограничение*: Составной первичный ключ `PRIMARY KEY(entry_id, tag_id)`.

---

## 2. История миграций базы данных

Миграции управляются встроенным инструментом `sqlx::migrate!` при инициализации пула бэкенда:

- **`0001_initial.sql`**: Создает базовые структуры категорий, объектов и записей событий (`categories`, `objects`, `entries`).
- **`0002_photos.sql`**: Добавляет фото-репозиторий и таблицу `entry_photos`.
- **`0003_add_reminders.sql`**: Добавляет поддержку планировщика напоминаний (`reminders`).
- **`0004_tags.sql`**: Создает таблицы тегов и связей событий с тегами (`tags`, `entry_tags`).
- **`0005_identity.sql`**: Закладывает фундамент подсистемы идентификации: таблицы пользователей (`users`), сессий устройств (`user_sessions`) и ключей восстановления (`recovery_keys`).
- **`0006_subscription.sql`**: Создает таблицу подписок (`subscriptions`) для управления уровнями Feature Gate прав.
