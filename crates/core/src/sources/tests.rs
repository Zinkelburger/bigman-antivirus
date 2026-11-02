//! Each source against a fake platform: what it reports, and what it stays quiet about.
use super::*;
use crate::{
    model::{Event, Link, Report, Severity, SourceId},
    platform::{
        ChangeKind, InterfaceCounters, Journal, JournalRecord, NetworkCounters, PackageChange,
        PackageHistory, ProcessInfo, ProcessSockets, Protocol, Socket, Transaction,
    },
};
use std::sync::{Arc, Mutex};

struct World {
    _dir: tempfile::TempDir,
    db: Db,
    store: Store,
}

fn world() -> World {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(Some(dir.path().join("state"))).unwrap();
    let db = Db::open(&dir.path().join("bigman.db")).unwrap();
    World {
        _dir: dir,
        db,
        store,
    }
}

/// Runs a source once and records its observations the way the daemon does.
fn run(w: &World, source: &mut dyn Source, now: u64) -> Result<(Vec<Event>, Vec<Sample>), Gap> {
    let collected = source.collect(&Ctx {
        db: &w.db,
        store: &w.store,
        now,
    })?;
    let events = collected
        .observations
        .iter()
        .map(|o| w.db.record(o).unwrap())
        .collect();
    Ok((events, collected.samples))
}

/// A fake whose output the test changes between runs.
#[derive(Clone, Default)]
struct Shared<T>(Arc<Mutex<T>>);
impl<T: Clone> Shared<T> {
    fn set(&self, value: T) {
        *self.0.lock().unwrap() = value;
    }
    fn get(&self) -> T {
        self.0.lock().unwrap().clone()
    }
}

impl NetworkCounters for Shared<Vec<InterfaceCounters>> {
    fn counters(&self) -> anyhow::Result<Vec<InterfaceCounters>> {
        Ok(self.get())
    }
}
impl ProcessSockets for Shared<Vec<Socket>> {
    fn sockets(&self) -> anyhow::Result<Vec<Socket>> {
        Ok(self.get())
    }
}
impl PackageHistory for Shared<Vec<Transaction>> {
    fn transactions_after(&self, cursor: Option<&str>) -> anyhow::Result<Vec<Transaction>> {
        let all = self.get();
        let start = cursor
            .and_then(|c| all.iter().position(|t| t.id == c))
            .map_or(0, |i| i + 1);
        Ok(all[start..].to_vec())
    }
}
impl Journal for Shared<Option<Vec<JournalRecord>>> {
    fn denials_after(
        &self,
        cursor: Option<&str>,
    ) -> anyhow::Result<Option<(Vec<JournalRecord>, Option<String>)>> {
        Ok(self.get().map(|records| {
            let start: usize = cursor.and_then(|c| c.parse().ok()).unwrap_or(0);
            (
                records[start.min(records.len())..].to_vec(),
                Some(records.len().to_string()),
            )
        }))
    }
}

fn counters(rx: u64, tx: u64) -> Vec<InterfaceCounters> {
    vec![InterfaceCounters {
        iface: "wlp2s0".into(),
        rx_bytes: rx,
        tx_bytes: tx,
    }]
}

#[test]
fn throughput_reports_bytes_per_second_and_skips_counter_resets() {
    let w = world();
    let fake = Shared::default();
    let mut source = Throughput::new(Box::new(fake.clone()));
    fake.set(counters(1_000, 500));
    assert!(
        run(&w, &mut source, 100).unwrap().1.is_empty(),
        "the first run only sets the baseline"
    );
    fake.set(counters(5_000, 1_500));
    let (_, samples) = run(&w, &mut source, 102).unwrap();
    assert_eq!((samples[0].rx, samples[0].tx), (2_000.0, 500.0));
    fake.set(counters(10, 10));
    assert!(
        run(&w, &mut source, 103).unwrap().1.is_empty(),
        "a reset counter is not a negative rate"
    );
}

fn connection(name: &str, remote: &str) -> Socket {
    Socket {
        protocol: Protocol::Tcp,
        local: "192.168.1.20:51000".parse().unwrap(),
        remote: Some(remote.parse().unwrap()),
        state: "established".into(),
        uid: 1000,
        process: Some(ProcessInfo {
            pid: 4242,
            name: name.into(),
            exe: None,
        }),
    }
}

#[test]
fn connections_log_first_sightings_only() {
    let w = world();
    let fake = Shared::default();
    let mut source = Connections::new(Box::new(fake.clone()));
    fake.set(vec![connection("firefox", "142.250.80.46:443")]);
    assert!(
        run(&w, &mut source, 90).unwrap().0.is_empty(),
        "the first run ever learns what is already connected"
    );
    fake.set(vec![
        connection("curl", "93.184.215.14:443"),
        connection("curl", "151.101.1.69:443"),
    ]);
    let (events, _) = run(&w, &mut source, 100).unwrap();
    assert_eq!(
        events.len(),
        1,
        "one entry for a program's first time online: {events:?}"
    );
    assert_eq!(
        events[0].summary,
        "curl made its first network connection, to 93.184.215.14:443"
    );
    assert_eq!(events[0].source, SourceId::Network);

    // Still connected, and a neighbour in the same range: nothing new.
    fake.set(vec![
        connection("curl", "93.184.215.14:443"),
        connection("curl", "93.184.215.99:443"),
    ]);
    assert!(run(&w, &mut source, 102).unwrap().0.is_empty());

    // Reconnecting later to a range it has used is not news either; a new range is.
    fake.set(vec![]);
    run(&w, &mut source, 104).unwrap();
    fake.set(vec![
        connection("curl", "93.184.215.14:443"),
        connection("curl", "140.82.112.3:443"),
    ]);
    let (events, _) = run(&w, &mut source, 106).unwrap();
    assert_eq!(events.len(), 1, "{events:?}");
    assert_eq!(
        events[0].summary,
        "curl connected to a new address range, 140.82.112.x port 443"
    );

    // A restart does not repeat what the store already knows.
    let mut restarted = Connections::new(Box::new(fake.clone()));
    fake.set(vec![
        connection("firefox", "142.250.80.46:443"),
        connection("curl", "140.82.112.3:443"),
    ]);
    assert!(run(&w, &mut restarted, 110).unwrap().0.is_empty());

    // A restart does not repeat what the store already knows.
    let mut restarted = Connections::new(Box::new(fake.clone()));
    fake.set(vec![
        connection("firefox", "142.250.80.46:443"),
        connection("curl", "140.82.112.3:443"),
    ]);
    assert!(run(&w, &mut restarted, 110).unwrap().0.is_empty());

    // Loopback is not the network.
    fake.set(vec![connection("node", "127.0.0.1:5173")]);
    assert!(run(&w, &mut source, 108).unwrap().0.is_empty());
}

fn listener(name: &str, local: &str) -> Socket {
    Socket {
        protocol: Protocol::Tcp,
        local: local.parse().unwrap(),
        remote: None,
        state: "listen".into(),
        uid: 1000,
        process: Some(ProcessInfo {
            pid: 7,
            name: name.into(),
            exe: None,
        }),
    }
}

#[test]
fn listeners_rate_new_listeners_and_stay_quiet_about_existing_ones() {
    let w = world();
    let fake = Shared::default();
    let mut source = Listeners::new(Box::new(fake.clone()));
    fake.set(vec![listener("cupsd", "127.0.0.1:631")]);
    assert!(
        run(&w, &mut source, 100).unwrap().0.is_empty(),
        "what was listening at start is not news"
    );

    fake.set(vec![
        listener("cupsd", "127.0.0.1:631"),
        listener("dockerd", "0.0.0.0:2375"),
    ]);
    let (events, _) = run(&w, &mut source, 103).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].rule.as_deref(), Some("net.listener.docker_api"));
    assert_eq!(events[0].severity, Severity::High);

    fake.set(vec![
        listener("cupsd", "127.0.0.1:631"),
        listener("vite", "127.0.0.1:5173"),
    ]);
    let (events, _) = run(&w, &mut source, 106).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(
        (events[0].severity, events[0].rule.as_deref()),
        (Severity::Info, None)
    );
    assert!(
        events[0]
            .summary
            .starts_with("vite started listening on port 5173"),
        "{}",
        events[0].summary
    );
}

fn transaction(id: &str, at: u64, names: &[&str], kind: ChangeKind) -> Transaction {
    Transaction {
        id: id.into(),
        at,
        manager: "dnf".into(),
        command: format!("dnf install {}", names.join(" ")),
        changes: names
            .iter()
            .map(|n| PackageChange {
                name: (*n).into(),
                version: "1.0-1.fc44".into(),
                kind,
            })
            .collect(),
    }
}

#[test]
fn packages_resume_from_their_cursor_and_look_back_one_week_at_first() {
    let w = world();
    let now = 100 * 86_400;
    let fake = Shared::default();
    let mut source = Packages::new(Box::new(fake.clone()));
    fake.set(vec![
        transaction("1", now - 30 * 86_400, &["vim"], ChangeKind::Install),
        transaction("2", now - 3600, &["nix", "nix-libs"], ChangeKind::Install),
    ]);
    let (events, _) = run(&w, &mut source, now).unwrap();
    assert_eq!(
        events
            .iter()
            .map(|e| e.summary.as_str())
            .collect::<Vec<_>>(),
        ["dnf installed nix and nix-libs"]
    );
    assert_eq!(events[0].source, SourceId::Receipt);

    let mut all = fake.get();
    let many: Vec<String> = (0..32).map(|i| format!("pkg{i}")).collect();
    all.push(transaction(
        "3",
        now,
        &many.iter().map(String::as_str).collect::<Vec<_>>(),
        ChangeKind::Upgrade,
    ));
    fake.set(all);
    let (events, _) = run(&w, &mut source, now + 10).unwrap();
    assert_eq!(
        events
            .iter()
            .map(|e| e.summary.as_str())
            .collect::<Vec<_>>(),
        ["dnf updated 32 packages"]
    );
    assert!(run(&w, &mut source, now + 20).unwrap().0.is_empty());
}

#[test]
fn denials_say_when_the_journal_is_unreadable_and_fold_repeats_within_a_run() {
    let w = world();
    let fake: Shared<Option<Vec<JournalRecord>>> = Shared::default();
    let mut source = Denials::new(Box::new(fake.clone()));
    let gap = run(&w, &mut source, 100).unwrap_err();
    assert_eq!(
        gap.why,
        SkipReason::PermissionDenied,
        "an unreadable journal is a gap, not silence"
    );

    let record = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/denial.selinux/input.txt"
    ))
    .unwrap();
    let line = JournalRecord {
        at: 90,
        message: record.trim().into(),
    };
    fake.set(Some(vec![line.clone(), line.clone(), line]));
    let (events, _) = run(&w, &mut source, 100).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(
        events[0].summary,
        "firefox, confined by the mozilla policy, tried to read .ssh (dir) and was stopped."
    );
    assert_eq!(events[0].rule.as_deref(), Some("denial.selinux"));
    assert!(
        run(&w, &mut source, 105).unwrap().0.is_empty(),
        "the cursor moves past what was read"
    );
}

#[test]
fn every_saved_report_enters_the_timeline_once() {
    let w = world();
    let now = crate::model::now();
    let mut old = Report::new("File scan", "/home/sam/Old");
    old.created = now - 30 * 86_400;
    w.store.save(&old).unwrap();
    let mut source = Reports::default();
    assert!(
        run(&w, &mut source, now).unwrap().0.is_empty(),
        "the first run looks back one week"
    );

    let mut scan = Report::new("File scan", "/home/sam/Downloads");
    // A long scan is saved after a quicker check that started later.
    scan.created = now - 300;
    w.store.save(&Report::new("Monitor started", "")).unwrap();
    w.store
        .save(&Report::new("PDF triage", "/home/sam/invoice.pdf"))
        .unwrap();
    w.store.save(&scan).unwrap();
    let (events, _) = run(&w, &mut source, now).unwrap();
    let summaries: Vec<_> = events.iter().map(|e| e.summary.as_str()).collect();
    assert_eq!(
        summaries,
        [
            "Scanned /home/sam/Downloads · no findings",
            "Read the PDF /home/sam/invoice.pdf · no findings"
        ]
    );
    assert_eq!(
        events[0].link,
        Some(Link::Report {
            id: scan.id.clone()
        })
    );
    assert!(
        run(&w, &mut Reports::default(), now).unwrap().0.is_empty(),
        "a restart does not repeat them"
    );
}
