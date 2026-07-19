-- Migration: 0008_fts5_search.sql
-- Description: Creates FTS5 virtual table for full-text search over entries.

CREATE VIRTUAL TABLE entries_fts USING fts5(
    entry_id UNINDEXED,
    object_id UNINDEXED,
    title,
    description,
    tags,
    tokenize='unicode61'
);

-- Trigger: auto-populate on INSERT
CREATE TRIGGER entries_fts_ai AFTER INSERT ON entries BEGIN
    INSERT INTO entries_fts(entry_id, object_id, title, description, tags)
    VALUES (new.id, new.object_id, new.title, COALESCE(new.description, ''), '');
END;

-- Trigger: auto-update on UPDATE
CREATE TRIGGER entries_fts_au AFTER UPDATE ON entries BEGIN
    DELETE FROM entries_fts WHERE entry_id = old.id;
    INSERT INTO entries_fts(entry_id, object_id, title, description, tags)
    VALUES (new.id, new.object_id, new.title, COALESCE(new.description, ''), '');
END;

-- Trigger: auto-remove on DELETE
CREATE TRIGGER entries_fts_ad AFTER DELETE ON entries BEGIN
    DELETE FROM entries_fts WHERE entry_id = old.id;
END;
