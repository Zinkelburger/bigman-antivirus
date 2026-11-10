-- Schema version 2: bytes each program moved over TCP, by remote address and
-- service port. program is NULL when another account runs it. resolution is the
-- seconds each row covers: 5 while fresh, 60 after two hours, 3600 after two days.
CREATE TABLE traffic (
    at INTEGER NOT NULL,
    program TEXT,
    uid INTEGER NOT NULL,
    remote TEXT NOT NULL,
    port INTEGER NOT NULL,
    inbound INTEGER NOT NULL,
    rx REAL NOT NULL,
    tx REAL NOT NULL,
    resolution INTEGER NOT NULL
);
CREATE INDEX traffic_at ON traffic(at);

-- Throughput samples by time alone: they show when the collector was running.
CREATE INDEX samples_time ON samples(at);
