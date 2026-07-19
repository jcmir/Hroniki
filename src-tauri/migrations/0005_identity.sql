-- Migration: 0005_identity.sql
-- Description: Establishes database structures for users, sessions, and recovery keys.

CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT UNIQUE,
    display_name TEXT,
    password_hash TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE user_sessions (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    device_name TEXT,
    created_at TEXT NOT NULL,
    expires_at TEXT,

    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE TABLE recovery_keys (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    key_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    used_at TEXT,

    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);
