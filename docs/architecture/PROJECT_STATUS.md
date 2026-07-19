# Статус подсистем проекта ХРОНИКИ

Ниже представлена таблица текущего состояния реализации подсистем и планов развития платформы:

| Подсистема | Компонент | Статус | Версия | Описание |
| :--- | :--- | :--- | :--- | :--- |
| **Security** | PIN Lock Screen | ✅ Production | v1.0.0 | PBKDF2 хеширование, 5-fail lockout, 15s кулдаун |
| **Security** | Crypto Vault | ✅ Production | v1.0.0 | AES-256-GCM шифрование файлов архива |
| **Storage** | SQLite Engine | ✅ Production | v1.0.0 | SqlitePool, foreign keys, 9 миграций схемы |
| **Storage** | Media originals | ✅ Production | v1.0.0 | Директория media оригиналов и staging |
| **Identity** | Domain Entities | ✅ Production | v1.0.0 | Модели User, Session, RecoveryKey |
| **Account** | Provider Model | ✅ Production | v1.0.0 | Абстракция AccountProvider, LocalAccountProvider |
| **Account** | Active Sessions | 🚧 In Progress | v1.1.0 | Валидация сроков действия сессии и аудит устройств |
| **Feature Gate**| Licensing Gate | ✅ Production | v1.0.0 | FeatureService, SubscriptionPlan сопоставление |
| **Backup** | Archive Format v2 | ✅ Production | v1.0.0 | manifest.json с контролем схем БД при импорте |
| **Event Bus** | Pub/Sub Bus | ✅ Production | v1.0.0 | Tokio broadcast асинхронные Domain Events |
| **Audit Log** | Actions logger | 🚧 In Progress | v1.0.0 | Подписчик на EventBus для аудита изменений |
| **Search** | FTS5 Engine | ✅ Production | v1.0.0 | SQLite FTS5 + SearchSubscriber, unicode61 токенизатор |
| **Reminders** | Reminder Engine | ✅ Production | v1.0.0 | Фоновый планировщик напоминаний, абстракция NotificationProvider |
| **Platform** | Platform Core | ✅ Production | v1.0.0 | Абстракция PlatformContext, адаптерная архитектура, трансляция ЖЦ ОС |
| **Cloud Sync** | Sync Engine | 🚧 Planned | v1.3.0 | CloudAccountProvider для репликации в облако |
| **AI Engine** | RAG Search | 🚧 Planned | v1.4.0 | Векторизация записей, семантический поиск по ленте |
