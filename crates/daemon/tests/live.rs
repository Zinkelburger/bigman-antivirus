//! Live collection end to end over the socket, with fake platforms: a new listener
//! reaches a subscriber within seconds, lands in the timeline and the unread count,
//! and throughput history survives for a later client.
use bigman_core::{
    ipc::{client::Client, Live, Request, Response},
    model::{Severity, SourceId},
    notify::NotificationSettings,
    platform::{
        InterfaceCounters, NetworkCounters, Notifier, ProcessInfo, ProcessSockets, Protocol, Socket,
    },
    runner,
    sources::{Listeners, Throughput},
    store::Store,
};
use bigman_daemon::{server, timeline, Daemon};
use std::{
    sync::{atomic::Ordering, Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone, Default)]
struct FakeSockets(Arc<Mutex<Vec<Socket>>>);
impl ProcessSockets for FakeSockets {
    fn sockets(&self) -> anyhow::Result<Vec<Socket>> {
        Ok(self.0.lock().unwrap().clone())
    }
}

#[derive(Clone, Default)]
struct FakeCounters(Arc<Mutex<u64>>);
impl NetworkCounters for FakeCounters {
    fn counters(&self) -> anyhow::Result<Vec<InterfaceCounters>> {
        let mut total = self.0.lock().unwrap();
        *total += 10_000;
        Ok(vec![InterfaceCounters {
            iface: "eth0".into(),
            rx_bytes: *total,
            tx_bytes: *total / 2,
        }])
    }
}

struct Quiet;
impl Notifier for Quiet {
    fn notify(&self, _: &str, _: &str) {}
}

fn listener(name: &str, local: &str) -> Socket {
    Socket {
        protocol: Protocol::Tcp,
        local: local.parse().unwrap(),
        remote: None,
        state: "listen".into(),
        uid: 1000,
        process: Some(ProcessInfo {
            pid: 99,
            name: name.into(),
            exe: None,
        }),
    }
}

#[test]
fn a_new_listener_reaches_subscribers_the_timeline_and_the_unread_count() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(Some(dir.path().join("state"))).unwrap();
    let runtime = tempfile::Builder::new()
        .prefix("bm")
        .tempdir_in(std::env::var_os("XDG_RUNTIME_DIR").map_or_else(std::env::temp_dir, Into::into))
        .unwrap();
    let socket = server::bind(&runtime.path().join("run/bigmand.sock")).unwrap();
    let path = socket.path.clone();

    let sockets = FakeSockets::default();
    sockets
        .0
        .lock()
        .unwrap()
        .push(listener("cupsd", "127.0.0.1:631"));
    let mut daemon = Daemon::new(store).unwrap();
    daemon.sockets = Box::new(sockets.clone());
    let daemon = Arc::new(daemon);
    let stop = runner::cancel();
    let collectors = timeline::start(
        &daemon,
        vec![
            Box::new(Listeners::new(Box::new(sockets.clone()))),
            Box::new(Throughput::new(Box::new(FakeCounters::default()))),
        ],
        Box::new(Quiet),
        &stop,
    );
    let server_stop = stop.clone();
    let server = std::thread::spawn(move || server::serve(&socket, daemon, &server_stop));

    // A subscriber on its own connection.
    let (tx, rx) = std::sync::mpsc::channel();
    let subscribe_path = path.clone();
    std::thread::spawn(move || {
        let mut client = Client::connect_to(&subscribe_path).unwrap();
        let _ = client.subscribe(true, false, |live| tx.send(live).is_ok());
    });
    assert!(
        matches!(rx.recv_timeout(Duration::from_secs(5)).unwrap(), Live::Unread(u) if u.first_seen == 0)
    );

    std::thread::sleep(Duration::from_millis(500));
    sockets
        .0
        .lock()
        .unwrap()
        .push(listener("dockerd", "0.0.0.0:2375"));
    let started = Instant::now();
    let event = loop {
        match rx
            .recv_timeout(Duration::from_secs(5))
            .expect("a pushed event within 5 s")
        {
            Live::Event(e) => break e,
            _ => continue,
        }
    };
    assert!(started.elapsed() < Duration::from_secs(5));
    assert_eq!(event.source, SourceId::Listener);
    assert_eq!(event.rule.as_deref(), Some("net.listener.docker_api"));
    assert_eq!(event.severity, Severity::High);

    let mut client = Client::connect_to(&path).unwrap();
    let Response::Events(events) = client
        .request(Request::Events {
            since: 0,
            until: None,
            sources: vec![],
            limit: 100,
        })
        .unwrap()
    else {
        panic!("events")
    };
    assert_eq!(events.iter().map(|e| e.id).collect::<Vec<_>>(), [event.id]);

    let Response::Unread(unread) = client.request(Request::Unread).unwrap() else {
        panic!("unread")
    };
    assert_eq!(
        (unread.first_seen, unread.to_review, unread.through),
        (1, 1, event.id)
    );
    let Response::Unread(after) = client
        .request(Request::MarkSeen {
            through: unread.through,
        })
        .unwrap()
    else {
        panic!("mark seen")
    };
    assert_eq!(after.first_seen, 0);

    let Response::Network(net) = client.request(Request::Network).unwrap() else {
        panic!("network")
    };
    assert_eq!(
        net.listeners[0].socket.local.port(),
        2375,
        "the most exposed listener comes first"
    );
    assert_eq!(net.listeners[0].findings[0].rule, "net.listener.docker_api");
    assert!(
        net.listeners[1].first_at.is_some(),
        "listeners present at start are counted as seen"
    );

    // Throughput was stored, so a client that connects later sees history.
    std::thread::sleep(Duration::from_millis(1500));
    let now = bigman_core::model::now();
    let Response::Samples(series) = client
        .request(Request::Samples {
            since: now - 300,
            until: now,
            points: 300,
        })
        .unwrap()
    else {
        panic!("samples")
    };
    assert!(
        !series.samples.is_empty()
            && series
                .samples
                .iter()
                .all(|s| s.iface == "eth0" && s.rx > 0.0)
    );

    let Response::Sources(statuses) = client.request(Request::Sources).unwrap() else {
        panic!("sources")
    };
    assert_eq!(
        statuses
            .iter()
            .map(|s| (s.name.as_str(), s.ok))
            .collect::<Vec<_>>(),
        [("listeners", true), ("throughput", true)]
    );

    let settings = NotificationSettings {
        new_listeners: true,
        ..Default::default()
    };
    client
        .request(Request::SetNotifications {
            settings: settings.clone(),
        })
        .unwrap();
    assert!(
        matches!(client.request(Request::Notifications).unwrap(), Response::Notifications(s) if s == settings)
    );

    stop.store(true, Ordering::Relaxed);
    server.join().unwrap().unwrap();
    for c in collectors {
        c.join().unwrap();
    }
}
