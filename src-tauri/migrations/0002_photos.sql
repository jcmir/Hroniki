CREATE TABLE app_metadata (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
);

CREATE TABLE photos (
    id TEXT PRIMARY KEY NOT NULL,
    entry_id TEXT NOT NULL,
    path TEXT NOT NULL,
    thumbnail TEXT NOT NULL,
    created_at TEXT NOT NULL,

    FOREIGN KEY(entry_id)
        REFERENCES entries(id)
        ON DELETE CASCADE
);
