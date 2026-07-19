# Документация модуля: Account

Модуль `account` отвечает за идентификацию пользователей, сессии устройств и авторизацию. Спроектирован с поддержкой смены провайдеров (`AccountProvider`) для будущей интеграции внешних учетных записей (Google, Apple, Cloud Sync).

---

## 1. Назначение

- Управление локальными и облачными учетными записями.
- Сопровождение жизненного цикла сессий авторизованных устройств.
- Хранение метаданных и разграничение прав доступа.

---

## 2. Основные сущности

- **`User`**: Модель данных пользователя. Содержит идентификатор, необязательный email, отображаемое имя и дату создания. Пароли в структуре `User` не хранятся!
- **`Session`**: Модель сессии устройства. Включает уникальный UUID сессии, `user_id` владельца, имя устройства (`device_name`) и дату авторизации.
- **`RecoveryKey`**: Запись криптографического ключа восстановления, привязанного к аккаунту.

---

## 3. Public API

Интерфейс `AccountService` предоставляет следующие методы:

```rust
impl AccountService {
    // Регистрация новой локальной учетной записи
    pub async fn create_account(
        &self,
        email: Option<String>,
        display_name: Option<String>,
        password: &str,
    ) -> Result<User, IdentityError>;

    // Аутентификация пользователя (сверка хеша Argon2id)
    pub async fn authenticate(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, IdentityError>;

    // Открытие сессии для устройства
    pub async fn open_session(
        &self,
        user_id: &str,
        device_name: Option<String>,
    ) -> Result<Session, IdentityError>;

    // Закрытие/удаление сессии (выход из аккаунта)
    pub async fn close_session(
        &self,
        session_id: &str,
    ) -> Result<(), IdentityError>;
}
```

---

## 4. Используемые и предоставляемые сервисы

- **Использует**:
  - `IdentityRepository`: Для выборки и записи данных в таблицы SQLite.
  - `security::password`: Хелперы хеширования и сверки паролей на базе Argon2id.
- **Предоставляет**:
  - Учетные данные пользователя для работы Feature Gate (`FeatureService`).
  - События для шины данных (например, `DomainEvent::UserRegistered`).

---

## 5. Будущие изменения

- Добавление `CloudAccountProvider` для авторизации на центральном сервере синхронизации ХРОНИКИ.
- Добавление `AppleProvider` / `GoogleProvider` для OAuth авторизации.
- Включение полей `last_used_at` и `revoked_at` в сессии для аудита активных устройств.
