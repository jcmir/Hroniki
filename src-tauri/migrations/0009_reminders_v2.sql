-- Migration: 0009_reminders_v2.sql
-- Description: Upgrades reminders table to support optional entry_id, title, body, and recurrence.

PRAGMA foreign_keys=OFF;

CREATE TABLE reminders_new (
    id TEXT PRIMARY KEY NOT NULL,
    entry_id TEXT,
    title TEXT NOT NULL,
    body TEXT,
    trigger_at TEXT NOT NULL,
    status TEXT NOT NULL,
    recurrence TEXT NOT NULL, -- 'Once', 'Daily', 'Weekly', 'Monthly'
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT,
    FOREIGN KEY(entry_id) REFERENCES entries(id) ON DELETE CASCADE
);

-- Copy existing data, populating title with a placeholder and mapping repeat_days to recurrence
INSERT INTO reminders_new (id, entry_id, title, body, trigger_at, status, recurrence, created_at, updated_at, completed_at)
SELECT 
    id, 
    entry_id, 
    'Напоминание о записи' AS title, 
    NULL AS body, 
    trigger_at, 
    status, 
    CASE 
        WHEN repeat_days IS NULL THEN 'Once'
        WHEN repeat_days = 1 THEN 'Daily'
        WHEN repeat_days = 7 THEN 'Weekly'
        ELSE 'Once'
    END AS recurrence,
    datetime('now') AS created_at,
    datetime('now') AS updated_at,
    completed_at
FROM reminders;

DROP TABLE reminders;
ALTER TABLE reminders_new RENAME TO reminders;

PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
