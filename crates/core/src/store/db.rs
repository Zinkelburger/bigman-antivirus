//! The daemon's SQLite store: timeline events with first-seen counts, throughput
//! samples, receipts and settings. Only bigmand opens it; clients ask the daemon.
use crate::model::{clean, Event, Link, Observation, Sample, Severity, SourceId, Unread};
use anyhow::{bail, Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::{path::Path, sync::Mutex};

/// Forward-only migrations, applied in order. Never edit a shipped one.
const MIGRATIONS: &[&str] = &[include_str!("migrations/0001_initial.sql")];

pub const SCHEMA_VERSION: u32 = MIGRATIONS.len() as u32;

pub struct Db {
    conn: Mutex<Connection>,
}

/// Which events to return, newest first.
#[derive(Debug, Clone, Default)]
pub struct EventQuery {
    pub since: u64,
    pub until: Option<u64>,
    pub sources: Vec<SourceId>,
    pub limit: u32,
}

fn severity_str(s: Severity) -> &'static str {
    match s {
        Severity::Info => "info",
        Severity::Review => "review",
        Severity::High => "high",
    }
}

fn severity_parse(s: &str) -> Severity {
    match s {
        "high" => Severity::High,
        "review" => Severity::Review,
        _ => Severity::Info,
    }
}

fn event_from_row(row: &rusqlite::Row) -> rusqlite::Result<Event> {
    let source: String = row.get(2)?;
    let link: Option<String> = row.get(10)?;
    Ok(Event {
        id: row.get(0)?,
        at: row.get::<_, i64>(1)? as u64,
        source: SourceId::parse(&source).unwrap_or(SourceId::Import),
        key: row.get(3)?,
        rule: row.get(4)?,
        severity: severity_parse(&row.get::<_, String>(5)?),
        summary: row.get(6)?,
        evidence: row.get(7)?,
        seen_before: row.get::<_, i64>(8)? as u32,
        first_at: row.get::<_, i64>(9)? as u64,
        link: link.and_then(|l| serde_json::from_str::<Link>(&l).ok()),
    })
}

impl Db {
    pub fn open(path: &Path) -> Result<Self> {
        let conn =
            Connection::open(path).with_context(|| format!("Cannot open {}", path.display()))?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let db = Db {
            conn: Mutex::new(conn),
        };
        db.migrate()?;
        Ok(db)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn migrate(&self) -> Result<()> {
        let mut conn = self.lock();
        conn.execute_batch("CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL)")?;
        let current: u32 = conn
            .query_row("SELECT max(version) FROM schema_version", [], |r| {
                r.get::<_, Option<u32>>(0)
            })?
            .unwrap_or(0);
        if current > SCHEMA_VERSION {
            bail!("The event store was written by a newer Bigman (schema {current}); this version understands up to {SCHEMA_VERSION}");
        }
        for (index, sql) in MIGRATIONS.iter().enumerate().skip(current as usize) {
            let tx = conn.transaction()?;
            tx.execute_batch(sql)?;
            tx.execute(
                "INSERT INTO schema_version (version) VALUES (?1)",
                [index as u32 + 1],
            )?;
            tx.commit()?;
        }
        Ok(())
    }

    pub fn schema_version(&self) -> Result<u32> {
        Ok(self
            .lock()
            .query_row("SELECT max(version) FROM schema_version", [], |r| r.get(0))?)
    }

    /// Numbers an observation and counts how often its key was seen before.
    pub fn record(&self, o: &Observation) -> Result<Event> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        let previous: Option<(i64, i64)> = tx
            .query_row(
                "SELECT count, first_at FROM first_seen WHERE key = ?1",
                [&o.key],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        let (seen_before, first_at) = previous.unwrap_or((0, o.at as i64));
        tx.execute(
            "INSERT INTO first_seen (key, source, first_at, count) VALUES (?1, ?2, ?3, 1)
             ON CONFLICT(key) DO UPDATE SET count = count + 1",
            params![o.key, o.source.as_str(), first_at],
        )?;
        let link = o.link.as_ref().map(serde_json::to_string).transpose()?;
        let evidence = clean(&o.evidence);
        tx.execute(
            "INSERT INTO events (at, source, key, rule, severity, summary, evidence, seen_before, first_at, link)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![o.at as i64, o.source.as_str(), o.key, o.rule, severity_str(o.severity), o.summary, evidence, seen_before, first_at, link],
        )?;
        let id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(Event {
            id,
            at: o.at,
            source: o.source,
            key: o.key.clone(),
            rule: o.rule.clone(),
            severity: o.severity,
            summary: o.summary.clone(),
            evidence,
            seen_before: seen_before as u32,
            first_at: first_at as u64,
            link: o.link.clone(),
        })
    }

    /// Counts a sighting of `key` without adding a timeline row, for things noticed
    /// alongside another entry (a new program's first address range).
    pub fn touch(&self, key: &str, source: SourceId, at: u64) -> Result<()> {
        self.lock().execute(
            "INSERT INTO first_seen (key, source, first_at, count) VALUES (?1, ?2, ?3, 1)
             ON CONFLICT(key) DO UPDATE SET count = count + 1",
            params![key, source.as_str(), at as i64],
        )?;
        Ok(())
    }

    /// Whether a key has ever been recorded, without recording it.
    pub fn seen(&self, key: &str) -> Result<bool> {
        Ok(self
            .lock()
            .query_row("SELECT 1 FROM first_seen WHERE key = ?1", [key], |_| Ok(()))
            .optional()?
            .is_some())
    }

    pub fn events(&self, q: &EventQuery) -> Result<Vec<Event>> {
        let conn = self.lock();
        // Source names are our own enum's, so they are safe to inline as literals.
        let filter = if q.sources.is_empty() {
            String::new()
        } else {
            let names: Vec<String> = q
                .sources
                .iter()
                .map(|s| format!("'{}'", s.as_str()))
                .collect();
            format!("AND source IN ({})", names.join(","))
        };
        let mut stmt = conn.prepare(&format!(
            "SELECT id, at, source, key, rule, severity, summary, evidence, seen_before, first_at, link
             FROM events WHERE at >= ?1 AND at <= ?2 {filter} ORDER BY at DESC, id DESC LIMIT ?3"
        ))?;
        let rows = stmt.query_map(
            params![
                q.since as i64,
                q.until.map_or(i64::MAX, |u| u as i64),
                q.limit.max(1) as i64
            ],
            event_from_row,
        )?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// What arrived since the user last marked everything as seen.
    pub fn unread(&self) -> Result<Unread> {
        let through = self
            .setting("activity.seen_through")?
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0);
        let (first_seen, to_review, latest) = self.lock().query_row(
            "SELECT coalesce(sum(seen_before = 0), 0), coalesce(sum(severity != 'info'), 0), max(id)
             FROM events WHERE id > ?1",
            [through],
            |r| Ok((r.get(0)?, r.get(1)?, r.get::<_, Option<i64>>(2)?)),
        )?;
        Ok(Unread {
            first_seen,
            to_review,
            through: latest.unwrap_or(through),
            seen_through: through,
        })
    }

    /// Marks everything up to event `through` as seen: the id the client showed, so
    /// an event arriving meanwhile stays unread. A lower id (Undo) marks entries
    /// after it unread again.
    pub fn mark_seen(&self, through: i64) -> Result<()> {
        self.set_setting("activity.seen_through", &through.max(0).to_string())
    }

    /// When `key` was first seen, if ever.
    pub fn first_at(&self, key: &str) -> Result<Option<u64>> {
        Ok(self
            .lock()
            .query_row(
                "SELECT first_at FROM first_seen WHERE key = ?1",
                [key],
                |r| r.get::<_, i64>(0),
            )
            .optional()?
            .map(|at| at as u64))
    }

    pub fn add_samples(&self, samples: &[Sample]) -> Result<()> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        for s in samples {
            tx.execute(
                "INSERT INTO samples (at, iface, rx, tx, resolution) VALUES (?1, ?2, ?3, ?4, 1)",
                params![s.at as i64, s.iface, s.rx, s.tx],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Samples in `[since, until]`, averaged into buckets so a range returns at most
    /// about `points` rows per interface. A missing bucket is a gap in collection.
    pub fn samples(&self, since: u64, until: u64, points: u64) -> Result<(u64, Vec<Sample>)> {
        let bucket = ((until.saturating_sub(since)) / points.max(1)).max(1);
        let conn = self.lock();
        let mut stmt = conn.prepare(
            "SELECT (at / ?3) * ?3 AS b, iface, avg(rx), avg(tx) FROM samples
             WHERE at >= ?1 AND at <= ?2 GROUP BY iface, b ORDER BY b",
        )?;
        let rows = stmt.query_map(params![since as i64, until as i64, bucket as i64], |r| {
            Ok(Sample {
                at: r.get::<_, i64>(0)? as u64,
                iface: r.get(1)?,
                rx: r.get(2)?,
                tx: r.get(3)?,
            })
        })?;
        Ok((bucket, rows.collect::<rusqlite::Result<_>>()?))
    }

    /// Keeps the samples table small: live rows become minute averages after a day,
    /// minute rows become hour averages after a month, hour rows go after a year.
    pub fn downsample(&self, now: u64) -> Result<()> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        for (from, to, age) in [(1i64, 60i64, 86_400i64), (60, 3_600, 30 * 86_400)] {
            let cutoff = now as i64 - age;
            tx.execute(
                "INSERT INTO samples (at, iface, rx, tx, resolution)
                 SELECT (at / ?2) * ?2, iface, avg(rx), avg(tx), ?2 FROM samples
                 WHERE resolution = ?1 AND at < ?3 GROUP BY iface, at / ?2",
                params![from, to, cutoff],
            )?;
            tx.execute(
                "DELETE FROM samples WHERE resolution = ?1 AND at < ?2",
                params![from, cutoff],
            )?;
        }
        tx.execute(
            "DELETE FROM samples WHERE at < ?1",
            [now as i64 - 365 * 86_400],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn setting(&self, key: &str) -> Result<Option<String>> {
        Ok(self
            .lock()
            .query_row("SELECT value FROM settings WHERE key = ?1", [key], |r| {
                r.get(0)
            })
            .optional()?)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.lock().execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observation(key: &str, at: u64) -> Observation {
        Observation {
            at,
            source: SourceId::Network,
            key: key.into(),
            rule: None,
            severity: Severity::Info,
            summary: format!("saw {key}"),
            evidence: String::new(),
            link: None,
        }
    }

    #[test]
    fn counts_repeats_and_keeps_first_sighting() {
        let d = tempfile::tempdir().unwrap();
        let db = Db::open(&d.path().join("events.sqlite")).unwrap();
        assert_eq!(db.schema_version().unwrap(), SCHEMA_VERSION);
        let first = db.record(&observation("firefox", 100)).unwrap();
        let again = db.record(&observation("firefox", 200)).unwrap();
        assert_eq!((first.seen_before, first.first_at), (0, 100));
        assert_eq!((again.seen_before, again.first_at), (1, 100));
        let unread = db.unread().unwrap();
        assert_eq!((unread.first_seen, unread.to_review), (1, 0));
        db.mark_seen(unread.through).unwrap();
        assert_eq!(db.unread().unwrap().first_seen, 0);
        db.mark_seen(0).unwrap();
        assert_eq!(
            db.unread().unwrap().first_seen,
            1,
            "undo marks them unread again"
        );
        let events = db
            .events(&EventQuery {
                since: 0,
                limit: 10,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            events.iter().map(|e| e.at).collect::<Vec<_>>(),
            vec![200, 100]
        );
        let other = db
            .events(&EventQuery {
                since: 0,
                limit: 10,
                sources: vec![SourceId::Canary],
                ..Default::default()
            })
            .unwrap();
        assert!(other.is_empty());
    }

    #[test]
    fn samples_downsample_without_losing_the_range() {
        let d = tempfile::tempdir().unwrap();
        let db = Db::open(&d.path().join("events.sqlite")).unwrap();
        let day = 86_400;
        let old: Vec<Sample> = (0..120)
            .map(|i| Sample {
                at: 1_000 + i,
                iface: "eth0".into(),
                rx: 10.0,
                tx: 2.0,
            })
            .collect();
        db.add_samples(&old).unwrap();
        db.downsample(1_000 + day * 2).unwrap();
        let (_, rows) = db.samples(0, 1_000 + day * 2, 1_000).unwrap();
        assert!(!rows.is_empty() && rows.iter().all(|s| (s.rx - 10.0).abs() < 1e-9));
        let count: i64 = db
            .lock()
            .query_row("SELECT count(*) FROM samples", [], |r| r.get(0))
            .unwrap();
        assert!(count <= 3, "120 one-second rows became {count} minute rows");
    }

    #[test]
    fn refuses_a_newer_schema() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("events.sqlite");
        drop(Db::open(&path).unwrap());
        Connection::open(&path)
            .unwrap()
            .execute("INSERT INTO schema_version (version) VALUES (99)", [])
            .unwrap();
        assert!(Db::open(&path).is_err());
    }
}
