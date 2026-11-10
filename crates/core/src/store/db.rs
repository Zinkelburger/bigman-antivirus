//! The daemon's SQLite store: timeline events with first-seen counts, throughput
//! samples, receipts and settings. Only bigmand opens it; clients ask the daemon.
use crate::model::{
    clean, Coverage, Event, Flow, FlowSample, Link, Observation, ProgramSeries, ProgramTotal,
    Sample, Severity, SourceId, TrafficReport, Unread,
};
use anyhow::{bail, Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::{path::Path, sync::Mutex};

/// Forward-only migrations, applied in order. Never edit a shipped one.
const MIGRATIONS: &[&str] = &[
    include_str!("migrations/0001_initial.sql"),
    include_str!("migrations/0002_traffic.sql"),
];

/// How many program, address and port combinations a traffic report lists.
pub const FLOW_LIMIT: usize = 2_000;
/// How many programs a traffic report gives a series of their own.
pub const SERIES_PROGRAMS: usize = 8;
/// Traffic rows become minute sums after two hours and hour sums after two days,
/// and go after thirty days.
const TRAFFIC_AGES: [(i64, i64, i64); 2] = [(5, 60, 7_200), (60, 3_600, 2 * 86_400)];
const TRAFFIC_KEPT: i64 = 30 * 86_400;

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
        for (from, to, age) in TRAFFIC_AGES {
            // Whole coarse intervals only, so one is never split across two rows.
            let cutoff = (now as i64 - age) / to * to;
            tx.execute(
                "INSERT INTO traffic (at, program, uid, remote, port, inbound, rx, tx, resolution)
                 SELECT (at / ?2) * ?2, program, uid, remote, port, inbound, sum(rx), sum(tx), ?2
                 FROM traffic WHERE resolution = ?1 AND at < ?3
                 GROUP BY at / ?2, program, uid, remote, port, inbound",
                params![from, to, cutoff],
            )?;
            tx.execute(
                "DELETE FROM traffic WHERE resolution = ?1 AND at < ?2",
                params![from, cutoff],
            )?;
        }
        tx.execute(
            "DELETE FROM traffic WHERE at < ?1",
            [now as i64 - TRAFFIC_KEPT],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn add_traffic(&self, flows: &[FlowSample]) -> Result<()> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        for f in flows {
            tx.execute(
                "INSERT INTO traffic (at, program, uid, remote, port, inbound, rx, tx, resolution)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    f.at as i64,
                    f.program,
                    f.uid,
                    f.remote.to_string(),
                    f.port,
                    f.inbound,
                    f.rx,
                    f.tx,
                    crate::sources::TRAFFIC_STEP as i64
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Traffic in `[since, until]` in about `points` intervals: every program's
    /// total, the largest program, address and port combinations, and bytes per
    /// interval for the programs that moved the most. An interval is never
    /// shorter than the coarsest stored row in the range, so older, summed rows
    /// do not show as spikes. Coverage and the account are left for the caller.
    pub fn traffic(&self, since: u64, until: u64, points: u64) -> Result<TrafficReport> {
        let conn = self.lock();
        let range = params![since as i64, until as i64];
        let coarsest: i64 = conn.query_row(
            "SELECT coalesce(max(resolution), ?3) FROM traffic WHERE at >= ?1 AND at <= ?2",
            params![
                since as i64,
                until as i64,
                crate::sources::TRAFFIC_STEP as i64
            ],
            |r| r.get(0),
        )?;
        let coarsest = coarsest.max(1) as u64;
        let wanted = (until.saturating_sub(since) / points.clamp(1, 2_000)).max(1);
        let bucket = wanted.div_ceil(coarsest) * coarsest;

        let mut stmt = conn.prepare(
            "SELECT program, uid, sum(rx), sum(tx) FROM traffic WHERE at >= ?1 AND at <= ?2
             GROUP BY program, uid ORDER BY sum(rx) + sum(tx) DESC, program, uid",
        )?;
        let programs: Vec<ProgramTotal> = stmt
            .query_map(range, |r| {
                Ok(ProgramTotal {
                    program: r.get(0)?,
                    uid: r.get(1)?,
                    rx: r.get(2)?,
                    tx: r.get(3)?,
                    first_at: None,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;

        let mut stmt = conn.prepare(
            "SELECT program, uid, remote, port, inbound, sum(rx), sum(tx) FROM traffic
             WHERE at >= ?1 AND at <= ?2 GROUP BY program, uid, remote, port, inbound
             ORDER BY sum(rx) + sum(tx) DESC, remote, port",
        )?;
        let mut flows: Vec<Flow> = stmt
            .query_map(range, |r| {
                Ok(Flow {
                    program: r.get(0)?,
                    uid: r.get(1)?,
                    remote: r.get(2)?,
                    port: r.get(3)?,
                    inbound: r.get(4)?,
                    rx: r.get(5)?,
                    tx: r.get(6)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;
        let flows_omitted = flows.len().saturating_sub(FLOW_LIMIT) as u32;
        flows.truncate(FLOW_LIMIT);

        let first = since / bucket * bucket;
        let at: Vec<u64> = (first..=until).step_by(bucket as usize).collect();
        let empty = || vec![0.0; at.len()];
        let mut series: Vec<ProgramSeries> = programs
            .iter()
            .take(SERIES_PROGRAMS)
            .map(|p| ProgramSeries {
                program: p.program.clone(),
                uid: p.uid,
                rx: empty(),
                tx: empty(),
            })
            .collect();
        let (mut rest_rx, mut rest_tx) = (empty(), empty());
        // The collector was running wherever throughput or traffic was written.
        let mut recorded = vec![false; at.len()];
        let mut stmt = conn.prepare(
            "SELECT DISTINCT (at / ?3) * ?3 FROM samples WHERE at >= ?1 AND at <= ?2
             UNION SELECT DISTINCT (at / ?3) * ?3 FROM traffic WHERE at >= ?1 AND at <= ?2",
        )?;
        let mut rows = stmt.query(params![first as i64, until as i64, bucket as i64])?;
        while let Some(r) = rows.next()? {
            let slot = (r.get::<_, i64>(0)? as u64).saturating_sub(first) / bucket;
            if let Some(seen) = recorded.get_mut(slot as usize) {
                *seen = true;
            }
        }
        drop(rows);
        let mut stmt = conn.prepare(
            "SELECT (at / ?3) * ?3, program, uid, sum(rx), sum(tx) FROM traffic
             WHERE at >= ?1 AND at <= ?2 GROUP BY 1, program, uid",
        )?;
        let mut rows = stmt.query(params![since as i64, until as i64, bucket as i64])?;
        while let Some(r) = rows.next()? {
            let start = r.get::<_, i64>(0)? as u64;
            let Some(slot) = start.checked_sub(first).map(|d| (d / bucket) as usize) else {
                continue;
            };
            if slot >= at.len() {
                continue;
            }
            let (program, uid): (Option<String>, u32) = (r.get(1)?, r.get(2)?);
            let (rx, tx): (f64, f64) = (r.get(3)?, r.get(4)?);
            match series
                .iter_mut()
                .find(|s| s.program == program && s.uid == uid)
            {
                Some(s) => {
                    s.rx[slot] += rx;
                    s.tx[slot] += tx;
                }
                None => {
                    rest_rx[slot] += rx;
                    rest_tx[slot] += tx;
                }
            }
        }
        Ok(TrafficReport {
            since,
            until,
            own_uid: 0,
            bucket,
            at,
            recorded,
            series,
            rest_rx,
            rest_tx,
            programs,
            flows,
            flows_omitted,
            coverage: Coverage::default(),
        })
    }

    /// Stores (or replaces, when after-effects are attached) one install receipt.
    pub fn save_receipt(&self, r: &crate::checks::receipt::Receipt) -> Result<()> {
        self.lock().execute(
            "INSERT INTO receipts (id, at, body) VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET body = excluded.body",
            params![r.id, r.at as i64, serde_json::to_string(r)?],
        )?;
        Ok(())
    }

    pub fn receipt(&self, id: &str) -> Result<Option<crate::checks::receipt::Receipt>> {
        let body: Option<String> = self
            .lock()
            .query_row("SELECT body FROM receipts WHERE id = ?1", [id], |r| {
                r.get(0)
            })
            .optional()?;
        body.map(|b| serde_json::from_str(&b).context("A stored receipt could not be read"))
            .transpose()
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

    fn flow(at: u64, program: Option<&str>, remote: &str, port: u16, rx: f64) -> FlowSample {
        FlowSample {
            at,
            program: program.map(String::from),
            uid: if program.is_some() { 1000 } else { 0 },
            remote: remote.parse().unwrap(),
            port,
            inbound: false,
            rx,
            tx: rx / 10.0,
        }
    }

    #[test]
    fn traffic_adds_up_by_program_flow_and_interval() {
        let d = tempfile::tempdir().unwrap();
        let db = Db::open(&d.path().join("events.sqlite")).unwrap();
        db.add_traffic(&[
            flow(1_000, Some("firefox"), "93.184.215.14", 443, 4_000.0),
            flow(1_005, Some("firefox"), "93.184.215.14", 443, 6_000.0),
            flow(1_005, Some("firefox"), "151.101.1.69", 443, 500.0),
            flow(1_010, Some("curl"), "1.1.1.1", 80, 200.0),
            flow(1_010, None, "192.168.1.20", 22, 50.0),
            flow(5_000, Some("curl"), "1.1.1.1", 80, 9e9),
        ])
        .unwrap();
        let r = db.traffic(1_000, 1_019, 4).unwrap();
        assert_eq!(
            (r.bucket, r.at.clone()),
            (5, vec![1_000, 1_005, 1_010, 1_015])
        );
        let names: Vec<_> = r.programs.iter().map(|p| p.program.as_deref()).collect();
        assert_eq!(names, [Some("firefox"), Some("curl"), None]);
        assert_eq!((r.programs[0].rx, r.programs[0].tx), (10_500.0, 1_050.0));
        assert_eq!(r.series[0].rx, [4_000.0, 6_500.0, 0.0, 0.0]);
        assert_eq!(r.series[1].rx, [0.0, 0.0, 200.0, 0.0]);
        assert!(
            r.rest_rx.iter().all(|v| *v == 0.0),
            "three programs all get a series"
        );
        assert_eq!(
            r.recorded,
            [true, true, true, false],
            "nothing was written after 1,010"
        );
        assert_eq!(r.flows.len(), 4);
        assert_eq!(
            (r.flows[0].remote.as_str(), r.flows[0].rx),
            ("93.184.215.14", 10_000.0)
        );
        assert_eq!(r.flows_omitted, 0);
        // A range with nothing in it is a row of zeros, not an error.
        db.add_samples(&[Sample {
            at: 2_031,
            iface: "eth0".into(),
            rx: 0.0,
            tx: 0.0,
        }])
        .unwrap();
        let quiet = db.traffic(2_000, 2_060, 6).unwrap();
        assert!(quiet.programs.is_empty() && quiet.at.len() == 7 && quiet.bucket == 10);
        // Throughput was being written at 2,031: the collector ran and nothing moved.
        assert_eq!(
            quiet.recorded,
            [false, false, false, true, false, false, false]
        );
    }

    #[test]
    fn traffic_is_summed_as_it_ages_and_intervals_never_undercut_stored_rows() {
        let d = tempfile::tempdir().unwrap();
        let db = Db::open(&d.path().join("events.sqlite")).unwrap();
        let start = 1_000_000 / 3_600 * 3_600;
        let rows: Vec<FlowSample> = (0..720)
            .map(|i| flow(start + i * 5, Some("firefox"), "93.184.215.14", 443, 100.0))
            .collect();
        db.add_traffic(&rows).unwrap();
        let count = |db: &Db| -> i64 {
            db.lock()
                .query_row("SELECT count(*) FROM traffic", [], |r| r.get(0))
                .unwrap()
        };
        db.downsample(start + 3 * 3_600).unwrap();
        assert_eq!(
            count(&db),
            60,
            "an hour of five-second rows became minute rows"
        );
        let r = db.traffic(start, start + 3_599, 360).unwrap();
        assert_eq!(
            r.bucket, 60,
            "ten-second intervals were asked for; minute rows are stored"
        );
        assert!(r.series[0].rx.iter().all(|v| *v == 1_200.0));
        assert_eq!(r.programs[0].rx, 72_000.0);
        db.downsample(start + 3 * 86_400).unwrap();
        assert_eq!(count(&db), 1);
        assert_eq!(
            db.traffic(0, start + 86_400, 100).unwrap().programs[0].rx,
            72_000.0
        );
        db.downsample(start + 40 * 86_400).unwrap();
        assert_eq!(count(&db), 0, "traffic older than thirty days is dropped");
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
