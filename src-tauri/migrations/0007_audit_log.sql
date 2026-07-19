-- Migration: 0007_audit_log.sql
-- Description: Creates audit_logs table for user & security events tracking.

CREATE TABLE audit_logs (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT,
    event_type TEXT NOT NULL,
    details TEXT,
    created_at TEXT NOT NULL,

    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);
