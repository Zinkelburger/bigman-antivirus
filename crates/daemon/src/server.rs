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

fn connection(stream: UnixStream, daemon: &Daemon) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;
    loop {
        let frame: ClientFrame = match read_frame(&mut reader) {
            Ok(Some(frame)) => frame,
            Ok(None) => return Ok(()),
            Err(e) => {
                // A malformed frame gets one error reply, then the connection closes.
                write_frame(
                    &mut writer,
                    &ServerFrame {
                        id: 0,
                        response: error(format!("Invalid request: {e}")),
                    },
                )?;
                return Ok(());
            }
        };
        let response = daemon.handle(frame.request);
        write_frame(
            &mut writer,
            &ServerFrame {
                id: frame.id,
                response,
            },
        )?;
    }
}
