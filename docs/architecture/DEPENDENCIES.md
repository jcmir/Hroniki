# Карта зависимостей и процессов

Данный документ содержит визуальные диаграммы архитектурных связей, потоков событий и сценариев выполнения ключевых процессов в приложении ХРОНИКИ.

---

## 1. Карта зависимостей компонентов (Dependency Graph)

Компоненты ядра связаны с соблюдением инверсии зависимостей. Бизнес-логика зависит от абстрактных интерфейсов (Traits), а хранилище реализует их:

```mermaid
graph TD
    UI[UI: Svelte 5 +page.svelte]
    CMD[Commands: Tauri IPC]
    AS[AccountService]
    AP[AccountProvider Trait]
    LAP[LocalAccountProvider]
    IR[IdentityRepository Trait]
    SIR[SqliteIdentityRepository]
    EB[EventBus: tokio broadcast]
    FS[FeatureService]

    UI -->|safeInvoke| CMD
    CMD -->|State AppState| AS
    AS --> AP
    LAP -.->|implements| AP
    LAP --> IR
    LAP --> EB
    SIR -.->|implements| IR
    FS --> SIR
```

---

## 2. Потоки событий (Event Flow Pipeline)

Асинхронные события домена (`DomainEvent`) публикуются в шину и рассылаются фоновым подписчикам:

```mermaid
flowchart LR
    subgraph Publishers [Издатели событий]
        LAP[LocalAccountProvider]
        BP[Backup Commands]
        FS[FeatureService]
    end

    subgraph Bus [Шина событий]
        EB[EventBus: tokio broadcast]
    end

    subgraph Subscribers [Фоновые слушатели]
        AL[Audit Logger - Planned]
        CS[Cloud Sync Engine - Planned]
        AI[AI Embedder - Planned]
    end

    Publishers -->|publish| Bus
    Bus -->|subscribe / recv| Subscribers
```

---

## 3. Диаграмма последовательности: Авторизация (Auth Sequence)

Процесс аутентификации локального пользователя:

```mermaid
sequenceDiagram
    autonumber
    participant UI as UI Screen
    participant CMD as Tauri Command
    participant AS as AccountService
    participant AP as LocalAccountProvider
    participant IR as SqliteIdentityRepository
    participant DB as SQLite DB

    UI->>CMD: invoke("login_account", { email, password })
    CMD->>AS: authenticate(email, password)
    AS->>AP: authenticate(email, password)
    AP->>IR: find_user_with_hash(email)
    IR->>DB: SELECT password_hash FROM users
    DB-->>IR: user row & hash
    IR-->>AP: User, password_hash
    Note over AP: verify_password(password, hash) via Argon2id
    AP-->>AS: User (if verified)
    AS-->>CMD: User
    CMD-->>UI: User Response
```

---

## 4. Диаграмма последовательности: Экспорт архива (Backup Export Sequence)

Процесс безопасного создания зашифрованной резервной копии v2:

```mermaid
sequenceDiagram
    autonumber
    participant UI as Svelte UI
    participant CMD as export_archive Command
    participant DB as SQLite Pool
    participant ZIP as ZipWriter (in memory)
    participant CRY as Crypto (security::password)
    participant FS as File System

    UI->>CMD: click export & input password
    Note over CMD: Show Save File Dialog (rfd)
    CMD->>DB: VACUUM INTO 'temp_backup.sqlite'
    CMD->>ZIP: start_file("chronology.sqlite")
    CMD->>ZIP: write_all(temp_db_bytes)
    Note over CMD: Generate manifest.json (v2 metadata)
    CMD->>ZIP: start_file("manifest.json")
    CMD->>ZIP: write_all(manifest_json)
    CMD->>ZIP: write_all(media_originals_folder)
    ZIP-->>CMD: zip_buffer
    CMD->>CRY: encrypt_data(zip_buffer, password)
    Note over CRY: PBKDF2 Key Derivation & AES-256-GCM
    CRY-->>CMD: encrypted_bytes
    CMD->>FS: write(target_path, encrypted_bytes)
    CMD-->>UI: "Export completed successfully"
```
