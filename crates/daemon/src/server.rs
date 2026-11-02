//! Socket server: one thread per client, peer uid checked on accept.
use crate::{error, Daemon};
use anyhow::{bail, Context, Result};
use bigman_core::{
    ipc::{read_frame, write_frame, ClientFrame, Request, Response, ServerFrame},
    runner::Cancel,
    store::private_dir,
};
use std::{
    io::{BufReader, ErrorKind},
    os::{
        fd::AsRawFd,
        unix::{
            fs::PermissionsExt,
            net::{UnixListener, UnixStream},
        },
    },
    path::{Path, PathBuf},
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

/// Another bigmand already answers on the socket. Not a failure: the service
/// manager should not restart a daemon that is simply not needed.
#[derive(Debug)]
pub struct AlreadyRunning(pub PathBuf);

impl std::fmt::Display for AlreadyRunning {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Another bigmand is already answering at {}",
            self.0.display()
        )
    }
}

impl std::error::Error for AlreadyRunning {}

/// A bound socket that is removed when dropped.
pub struct Socket {
    pub listener: UnixListener,
    pub path: PathBuf,
}

impl Drop for Socket {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

pub fn bind(path: &Path) -> Result<Socket> {
    // sockaddr_un holds 108 bytes including the terminating NUL.
    if path.as_os_str().len() > 107 {
        bail!(
            "Socket path is longer than the 107 bytes Unix sockets allow: {}",
            path.display()
        );
    }
    private_dir(
        path.parent()
            .context("Socket path has no parent directory")?,
    )?;
    if path.exists() {
        if UnixStream::connect(path).is_ok() {
            return Err(AlreadyRunning(path.into()).into());
        }
        std::fs::remove_file(path)?;
    }
    let listener =
        UnixListener::bind(path).with_context(|| format!("Cannot bind {}", path.display()))?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    listener.set_nonblocking(true)?;
    Ok(Socket {
        listener,
        path: path.into(),
    })
}

fn peer_uid(stream: &UnixStream) -> std::io::Result<u32> {
    let mut cred = libc::ucred {
        pid: 0,
        uid: 0,
        gid: 0,
    };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    let rc = unsafe {
        libc::getsockopt(
            stream.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            (&mut cred as *mut libc::ucred).cast(),
            &mut len,
        )
    };
    if rc != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(cred.uid)
}

/// Accepts clients until `stop` is set.
pub fn serve(socket: &Socket, daemon: Arc<Daemon>, stop: &Cancel) -> Result<()> {
    let own_uid = unsafe { libc::geteuid() };
    while !stop.load(Ordering::Relaxed) {
        match socket.listener.accept() {
            Ok((stream, _)) => {
                if peer_uid(&stream).ok() != Some(own_uid) {
                    continue;
                }
                stream.set_nonblocking(false)?;
                let daemon = daemon.clone();
                std::thread::spawn(move || {
                    if let Err(e) = connection(stream, &daemon) {
                        eprintln!("bigmand: client connection ended: {e}");
                    }
                });
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

enum Event {
    Frame(ClientFrame),
    /// The client closed its side, or sent something unreadable.
    Closed(Option<String>),
    /// A subscription push; the job keeps running.
    Push(ServerFrame),
    Done(ServerFrame),
}

/// Forwards timeline pushes until the client goes away or falls too far behind.
fn subscription(
    daemon: &Daemon,
    id: u64,
    events: bool,
    samples: bool,
    stop: &Cancel,
    tx: &std::sync::mpsc::Sender<Event>,
) -> Response {
    use std::sync::mpsc::RecvTimeoutError;
    let pushes = daemon.timeline.subscribe(events, samples);
    if events {
        // Start from the current unread count, so a client needs no separate request.
        if let Ok(unread) = daemon.timeline.db.unread() {
            let _ = tx.send(Event::Push(ServerFrame {
                id,
                response: Response::Live(bigman_core::ipc::Live::Unread(unread)),
            }));
        }
    }
    while !stop.load(Ordering::Relaxed) {
        match pushes.recv_timeout(Duration::from_millis(250)) {
            Ok(push) => {
                if tx
                    .send(Event::Push(ServerFrame {
                        id,
                        response: Response::Live(push),
                    }))
                    .is_err()
                {
                    break;
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                return error(
                    "The subscription fell behind and was ended; subscribe again and reload.",
                );
            }
        }
    }
    Response::Done(Box::new(bigman_core::actions::ActionOutcome {
        message: "Subscription ended.".into(),
        report: None,
        undo: None,
    }))
}

/// One client. A reader thread turns frames into events; each request runs on
/// its own thread with a cancel flag that a disconnect sets, so a closed window
/// or a Cancel button stops a long scan. Requests on one connection run in order.
fn connection(stream: UnixStream, daemon: &Arc<Daemon>) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel::<Event>();
    let reader_tx = tx.clone();
    let mut reader = BufReader::new(stream.try_clone()?);
    std::thread::spawn(move || loop {
        let event = match read_frame::<ClientFrame>(&mut reader) {
            Ok(Some(frame)) => Event::Frame(frame),
            Ok(None) => Event::Closed(None),
            Err(e) => Event::Closed(Some(format!("Invalid request: {e}"))),
        };
        let closed = matches!(event, Event::Closed(_));
        if reader_tx.send(event).is_err() || closed {
            break;
        }
    });

    let mut writer = stream;
    let mut queue = std::collections::VecDeque::new();
    let mut running: Option<Cancel> = None;
    let mut closed = false;
    let start = |frame: ClientFrame, tx: &std::sync::mpsc::Sender<Event>| -> Cancel {
        let stop = bigman_core::runner::cancel();
        let (daemon, job_stop, tx) = (daemon.clone(), stop.clone(), tx.clone());
        std::thread::spawn(move || {
            let response = match frame.request {
                Request::Subscribe { events, samples } => {
                    subscription(&daemon, frame.id, events, samples, &job_stop, &tx)
                }
                request => daemon.handle(request, &job_stop),
            };
            let _ = tx.send(Event::Done(ServerFrame {
                id: frame.id,
                response,
            }));
        });
        stop
    };
    for event in rx {
        match event {
            Event::Frame(frame) if running.is_some() => queue.push_back(frame),
            Event::Frame(frame) => running = Some(start(frame, &tx)),
            Event::Closed(problem) => {
                if let Some(message) = problem {
                    // A malformed frame gets one error reply, then the connection closes.
                    let _ = write_frame(
                        &mut writer,
                        &ServerFrame {
                            id: 0,
                            response: error(message),
                        },
                    );
                }
                closed = true;
                match &running {
                    Some(stop) => stop.store(true, Ordering::Relaxed),
                    None => break,
                }
            }
            Event::Push(frame) => {
                if write_frame(&mut writer, &frame).is_err() {
                    closed = true;
                    if let Some(stop) = &running {
                        stop.store(true, Ordering::Relaxed);
                    }
                }
            }
            Event::Done(frame) => {
                running = None;
                // A client that half-closed to cancel still reads the cancelled report.
                if write_frame(&mut writer, &frame).is_err() {
                    closed = true;
                }
                if closed {
                    break;
                }
                if let Some(next) = queue.pop_front() {
                    running = Some(start(next, &tx));
                }
            }
        }
    }
    Ok(())
}
