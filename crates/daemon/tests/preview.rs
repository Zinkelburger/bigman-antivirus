//! The opt-in pre-transaction preview over the socket: a hook's receipt reaches
//! the app, the app's answer reaches the hook, and nobody answering continues.
use bigman_core::{
    checks::receipt::{AfterEffects, Receipt},
    ipc::{client::Client, Live, Request, Response},
    model::{Coverage, SourceId},
    platform::{ChangeKind, PackageChange},
    runner,
    store::Store,
};
use bigman_daemon::{server, Daemon};
use std::{
    path::PathBuf,
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

fn receipt(id: &str) -> Receipt {
    Receipt {
        id: id.into(),
        at: bigman_core::model::now(),
        manager: "dnf".into(),
        transaction: "preview-1".into(),
        command: "dnf install nix".into(),
        packages: vec![PackageChange {
            name: "nix".into(),
            version: "2.24.9-1.fc44".into(),
            kind: ChangeKind::Install,
        }],
        files: vec![],
        scripts: vec![],
        hits: vec![],
        services: vec![],
        repositories: Default::default(),
        after: AfterEffects::default(),
        findings: vec![],
        coverage: Coverage::default(),
        score: 0,
    }
}

fn daemon() -> (
    tempfile::TempDir,
    PathBuf,
    Arc<Daemon>,
    runner::Cancel,
    std::thread::JoinHandle<anyhow::Result<()>>,
) {
    let dir = tempfile::tempdir().unwrap();
    let runtime = tempfile::Builder::new()
        .prefix("bm")
        .tempdir_in(std::env::var_os("XDG_RUNTIME_DIR").map_or_else(std::env::temp_dir, Into::into))
        .unwrap();
    let socket = server::bind(&runtime.path().join("run/bigmand.sock")).unwrap();
    let path = socket.path.clone();
    let daemon =
        Arc::new(Daemon::new(Store::open(Some(dir.path().join("state"))).unwrap()).unwrap());
    let stop = runner::cancel();
    let (d, s) = (daemon.clone(), stop.clone());
    let server = std::thread::spawn(move || {
        let _keep = runtime;
        server::serve(&socket, d, &s)
    });
    (dir, path, daemon, stop, server)
}

#[test]
fn the_app_stops_a_previewed_transaction_and_nobody_answering_continues() {
    let (_dir, path, daemon, stop, server) = daemon();

    let (tx, rx) = std::sync::mpsc::channel();
    let sub = path.clone();
    std::thread::spawn(move || {
        let mut client = Client::connect_to(&sub).unwrap();
        let _ = client.subscribe(true, false, |live| tx.send(live).is_ok());
    });
    assert!(matches!(
        rx.recv_timeout(Duration::from_secs(5)).unwrap(),
        Live::Unread(_)
    ));

    // The hook asks; the app sees the preview and answers Abort.
    let hook_path = path.clone();
    let hook = std::thread::spawn(move || {
        Client::connect_to(&hook_path)
            .unwrap()
            .request(Request::PreviewTransaction {
                receipt: Box::new(receipt("r1")),
            })
            .unwrap()
    });
    let pushed = loop {
        if let Live::Preview(r) = rx.recv_timeout(Duration::from_secs(5)).unwrap() {
            break r;
        }
    };
    assert_eq!(pushed.receipt.packages[0].name, "nix");
    assert!(
        pushed.deadline > bigman_core::model::now(),
        "the push says when it goes ahead"
    );
    // A window that connects while it waits is shown it too.
    let (late_tx, late_rx) = std::sync::mpsc::channel();
    let late = path.clone();
    std::thread::spawn(move || {
        let mut client = Client::connect_to(&late).unwrap();
        let _ = client.subscribe(true, false, |live| late_tx.send(live).is_ok());
    });
    assert!(
        std::iter::from_fn(|| late_rx.recv_timeout(Duration::from_secs(5)).ok())
            .any(|l| matches!(l, Live::Preview(w) if w.receipt.id == "r1"))
    );
    let mut app = Client::connect_to(&path).unwrap();
    assert!(matches!(
        app.request(Request::Decide {
            id: "r1".into(),
            proceed: false
        })
        .unwrap(),
        Response::Done(_)
    ));
    let Response::Decision(d) = hook.join().unwrap() else {
        panic!("a decision")
    };
    assert_eq!((d.proceed, d.by.as_str()), (false, "you"));
    assert!(
        matches!(
            app.request(Request::Decide {
                id: "r1".into(),
                proceed: true
            })
            .unwrap(),
            Response::Error(_)
        ),
        "an answered preview is no longer waiting"
    );

    // Nobody answers: it continues.
    *daemon.timeline.decision_wait.lock().unwrap() = Duration::from_millis(300);
    let Response::Decision(d) = app
        .request(Request::PreviewTransaction {
            receipt: Box::new(receipt("r2")),
        })
        .unwrap()
    else {
        panic!("a decision")
    };
    assert_eq!((d.proceed, d.by.as_str()), (true, "timeout"));

    // Both are on the timeline, linked to their receipts.
    let Response::Events(events) = app
        .request(Request::Events {
            since: 0,
            until: None,
            sources: vec![SourceId::Receipt],
            limit: 10,
        })
        .unwrap()
    else {
        panic!("events")
    };
    let summaries: Vec<_> = events.iter().map(|e| e.summary.as_str()).collect();
    assert_eq!(
        summaries,
        [
            "Previewed before it ran: dnf installed nix; continued after a minute without an answer",
            "Previewed before it ran: dnf installed nix; stopped by you"
        ]
    );
    assert!(matches!(
        app.request(Request::Receipt { id: "r1".into() }).unwrap(),
        Response::Receipt(_)
    ));

    stop.store(true, Ordering::Relaxed);
    server.join().unwrap().unwrap();
}
