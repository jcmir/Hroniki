CREATE TABLE reminders (
    id TEXT PRIMARY KEY NOT NULL,
    entry_id TEXT NOT NULL,
    trigger_at TEXT NOT NULL,
    status TEXT NOT NULL,
    repeat_days INTEGER,
    completed_at TEXT,

    FOREIGN KEY(entry_id)
        REFERENCES entries(id)
        ON DELETE CASCADE
);
