use bigman_core::{
    ipc::{client::Client, Request, Response},
    runner,
    store::Store,
};
use bigman_daemon::{server, Daemon};
use std::{
    io::{BufRead, BufReader, Write},
    os::unix::{fs::PermissionsExt, net::UnixStream},
    sync::{atomic::Ordering, Arc},
};

#[test]
fn answers_health_on_a_private_socket() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(Some(dir.path().join("state"))).unwrap();
    let socket = server::bind(&dir.path().join("run/bigmand.sock")).unwrap();
    let path = socket.path.clone();
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert!(
        server::bind(&path).is_err(),
        "a second daemon must not take over the socket"
    );

    let stop = runner::cancel();
    let daemon_stop = stop.clone();
    let daemon = Arc::new(Daemon::new(store));
    let thread = std::thread::spawn(move || server::serve(&socket, daemon, &daemon_stop));

    let mut client = Client::connect_to(&path).unwrap();
    match client.request(Request::Health).unwrap() {
        Response::Health(h) => {
            assert_eq!(h.pid, std::process::id());
            assert!(h.state_dir.ends_with("state"));
        }
        other => panic!("unexpected {other:?}"),
    }
    // Two requests on one connection keep their ids.
    assert!(matches!(
        client.request(Request::Health).unwrap(),
        Response::Health(_)
    ));

    let mut raw = UnixStream::connect(&path).unwrap();
    raw.write_all(b"{\"id\":1,\"request\":{\"type\":\"run_shell\"}}\n")
        .unwrap();
    let mut line = String::new();
    BufReader::new(&raw).read_line(&mut line).unwrap();
    assert!(line.contains("\"error\""), "{line}");

    stop.store(true, Ordering::Relaxed);
    thread.join().unwrap().unwrap();
    assert!(!path.exists(), "socket is removed on shutdown");
}
