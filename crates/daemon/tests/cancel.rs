//! A client that closes its side of the connection cancels the running check.
use bigman_core::{
    ipc::{read_frame, write_frame, ClientFrame, Request, Response, ServerFrame},
    runner,
    store::Store,
};
use bigman_daemon::{server, Daemon};
use std::{
    io::BufReader,
    net::Shutdown,
    os::unix::{fs::PermissionsExt, net::UnixStream},
    sync::{atomic::Ordering, Arc},
    time::{Duration, Instant},
};

#[test]
fn disconnect_cancels_a_running_scan() {
    let dir = tempfile::tempdir().unwrap();
    // A scanner that would take a minute; the test must not wait for it.
    let tools = dir.path().join("tools");
    std::fs::create_dir(&tools).unwrap();
    let clamscan = tools.join("clamscan");
    std::fs::write(&clamscan, "#!/bin/sh\nexec /usr/bin/sleep 60\n").unwrap();
    std::fs::set_permissions(&clamscan, std::fs::Permissions::from_mode(0o700)).unwrap();
    std::env::set_var("PATH", &tools);
    let target = dir.path().join("downloads");
    std::fs::create_dir(&target).unwrap();
    std::fs::write(target.join("file.txt"), "fixture").unwrap();

    let store = Store::open(Some(dir.path().join("state"))).unwrap();
    let socket = server::bind(&dir.path().join("run/bigmand.sock")).unwrap();
    let path = socket.path.clone();
    let stop = runner::cancel();
    let daemon_stop = stop.clone();
    let thread = std::thread::spawn(move || {
        server::serve(&socket, Arc::new(Daemon::new(store)), &daemon_stop)
    });

    let stream = UnixStream::connect(&path).unwrap();
    let mut writer = stream.try_clone().unwrap();
    write_frame(
        &mut writer,
        &ClientFrame {
            id: 1,
            request: Request::Scan { path: target },
        },
    )
    .unwrap();
    std::thread::sleep(Duration::from_millis(300));
    let started = Instant::now();
    stream.shutdown(Shutdown::Write).unwrap();
    let frame: ServerFrame = read_frame(&mut BufReader::new(&stream)).unwrap().unwrap();
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "cancel took {:?}",
        started.elapsed()
    );
    match frame.response {
        Response::Report(r) => {
            assert!(r.is_incomplete(), "a cancelled scan is incomplete");
            assert!(
                r.errors
                    .iter()
                    .any(|e| e.contains("cancelled/timed out true")),
                "{:?}",
                r.errors
            );
        }
        other => panic!("unexpected {other:?}"),
    }
    stop.store(true, Ordering::Relaxed);
    thread.join().unwrap().unwrap();
}
