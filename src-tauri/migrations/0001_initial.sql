CREATE TABLE categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL
);


CREATE TABLE objects (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,

    FOREIGN KEY(category_id)
        REFERENCES categories(id)
);


CREATE TABLE entries (
    id TEXT PRIMARY KEY NOT NULL,
    object_id TEXT NOT NULL,
    occurred_at TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    FOREIGN KEY(object_id)
        REFERENCES objects(id)
);
