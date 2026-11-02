-- Bigman event store, schema version 1. Forward-only: later changes are new files.
CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    at INTEGER NOT NULL,
    source TEXT NOT NULL,
    key TEXT NOT NULL,
    rule TEXT,
    severity TEXT NOT NULL,
    summary TEXT NOT NULL,
    evidence TEXT NOT NULL,
    seen_before INTEGER NOT NULL,
    first_at INTEGER NOT NULL,
    link TEXT
);
CREATE INDEX events_at ON events(at);
CREATE INDEX events_key ON events(key);

-- How often each key has been seen; drives first-seen marks and folding.
CREATE TABLE first_seen (
    key TEXT PRIMARY KEY,
    source TEXT NOT NULL,
    first_at INTEGER NOT NULL,
    count INTEGER NOT NULL
);

-- Throughput, bytes per second. resolution is the seconds each row covers:
-- 1 for live samples, 60 after a day, 3600 after a month.
CREATE TABLE samples (
    at INTEGER NOT NULL,
    iface TEXT NOT NULL,
    rx REAL NOT NULL,
    tx REAL NOT NULL,
    resolution INTEGER NOT NULL
);
CREATE INDEX samples_at ON samples(iface, at);

-- Install receipts (milestone 5) are stored whole as JSON.
CREATE TABLE receipts (
    id TEXT PRIMARY KEY,
    at INTEGER NOT NULL,
    body TEXT NOT NULL
);

-- User settings and per-source cursors.
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
