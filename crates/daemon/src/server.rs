//! Socket server: one thread per client, peer uid checked on accept.
use crate::{error, Daemon};
use anyhow::{bail, Context, Result};
use bigman_core::{
    ipc::{read_frame, write_frame, ClientFrame, ServerFrame},
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
            bail!("Another bigmand is already answering at {}", path.display());
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
    Done(ServerFrame),
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
            let response = daemon.handle(frame.request, &job_stop);
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
