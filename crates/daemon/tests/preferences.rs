//! Turning an automatic check off in Setup: it shows as off at once, checks
//! nothing while off, and does not catch up on what arrived meanwhile when it is
//! turned back on. And `Shutdown` stops the daemon the way a signal does.
use bigman_core::{
    ipc::{client::Client, Request, Response},
    model::now,
    runner,
    sources::{Downloads, Preferences, SourceStatus},
    store::Store,
};
use bigman_daemon::{server, Daemon};
use std::sync::Arc;

const LAUNCHER: &str =
    "[Desktop Entry]\nType=Application\nName=Invoice.pdf\nExec=sh -c \"echo fixture\"\nIcon=application-pdf\n";

fn downloads_status(daemon: &Daemon) -> SourceStatus {
    daemon
        .timeline
        .statuses()
        .into_iter()
        .find(|s| s.name == "downloads")
        .unwrap()
}

fn set(daemon: &Daemon, preferences: Preferences) {
    let response = daemon.handle(Request::SetPreferences { preferences }, &runner::cancel());
    assert!(matches!(response, Response::Preferences(_)), "{response:?}");
}

#[test]
fn a_watch_turned_off_checks_nothing_and_does_not_catch_up() {
    let dir = tempfile::tempdir().unwrap();
    let daemon = Daemon::new(Store::open(Some(dir.path().join("state"))).unwrap()).unwrap();
    let folder = dir.path().join("Downloads");
    std::fs::create_dir(&folder).unwrap();
    let mut source = Downloads::new(folder.clone(), runner::cancel());
    let mut events = vec![];
    let mut tick = |daemon: &Daemon, at: u64| {
        events.extend(daemon.timeline.run_source(&mut source, &daemon.store, at));
    };

    tick(&daemon, now());
    assert!(!downloads_status(&daemon).off);

    set(
        &daemon,
        Preferences {
            watch_downloads: false,
            ..Preferences::default()
        },
    );
    let off = downloads_status(&daemon);
    assert!(off.off && off.ok, "off at once, and not a gap: {off:?}");
    std::fs::write(folder.join("while-off.desktop"), LAUNCHER).unwrap();
    // The cursor is whole seconds; step past the second the file arrived in.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    tick(&daemon, now());
    tick(&daemon, now() + 10);
    assert!(downloads_status(&daemon).off);

    set(&daemon, Preferences::default());
    let on = downloads_status(&daemon);
    assert!(!on.off && on.last_run.is_none(), "{on:?}");
    tick(&daemon, now());
    std::fs::write(folder.join("after.desktop"), LAUNCHER).unwrap();
    tick(&daemon, now());
    tick(&daemon, now() + 10);

    let seen = format!("{events:?}");
    assert!(!seen.contains("while-off"), "{seen}");
    assert_eq!(events.len(), 1, "{seen}");
    assert!(seen.contains("after.desktop"), "{seen}");
}

#[test]
fn stored_preferences_from_before_the_watch_switches_keep_their_choice() {
    let dir = tempfile::tempdir().unwrap();
    let daemon = Daemon::new(Store::open(Some(dir.path().join("state"))).unwrap()).unwrap();
    daemon
        .timeline
        .db
        .set_setting(Preferences::KEY, r#"{"check_updates":false}"#)
        .unwrap();
    assert_eq!(
        Preferences::load(&daemon.timeline.db),
        Preferences {
            check_updates: false,
            watch_downloads: true,
            watch_projects: true,
        }
    );
}

#[test]
fn shutdown_stops_serving_and_removes_the_socket() {
    let dir = tempfile::tempdir().unwrap();
    let daemon =
        Arc::new(Daemon::new(Store::open(Some(dir.path().join("state"))).unwrap()).unwrap());
    let socket = server::bind(&dir.path().join("run/bigmand.sock")).unwrap();
    let path = socket.path.clone();
    let stop = daemon.stop.clone();
    let thread = std::thread::spawn(move || server::serve(&socket, daemon, &stop));

    let mut client = Client::connect_to(&path).unwrap();
    let response = client.request(Request::Shutdown).unwrap();
    assert!(matches!(response, Response::Done(_)), "{response:?}");
    thread.join().unwrap().unwrap();
    assert!(!path.exists());
}
