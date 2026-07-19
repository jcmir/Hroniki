-- Migration: 0006_subscription.sql
-- Description: Establishes subscription tables for user subscription management.

CREATE TABLE subscriptions (
    user_id TEXT PRIMARY KEY NOT NULL,
    plan TEXT NOT NULL,
    status TEXT NOT NULL,
    expires_at TEXT,
    updated_at TEXT NOT NULL,

    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);
