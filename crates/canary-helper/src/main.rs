//! bigman-canary-helper: the optional root service behind canary reader
//! identification. See the library docs for what it does and refuses.
use bigman_canary_helper::{server, SOCKET};
use std::{
    collections::HashMap,
    os::unix::{fs::PermissionsExt, net::UnixListener},
    path::Path,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

/// Connections one user may hold, and in total: each holds a fanotify group and
/// two threads. The unit's TasksMax allows for all of them.
const PER_USER: usize = 4;
const TOTAL: usize = 64;

type Open = Arc<Mutex<HashMap<u32, usize>>>;

/// The counts stay right whatever a connection's thread did, so a panic there
/// must not stop the service from accepting.
fn lock(open: &Open) -> MutexGuard<'_, HashMap<u32, usize>> {
    open.lock().unwrap_or_else(PoisonError::into_inner)
}

/// One of a user's connection slots, given back when the connection's thread
/// ends, also by panicking, and when the thread could not be started.
struct Slot {
    open: Open,
    uid: u32,
}

impl Slot {
    fn take(open: &Open, uid: u32) -> Option<Self> {
        let mut o = lock(open);
        let total: usize = o.values().sum();
        if o.get(&uid).copied().unwrap_or(0) >= PER_USER || total >= TOTAL {
            return None;
        }
        *o.entry(uid).or_default() += 1;
        Some(Self {
            open: open.clone(),
            uid,
        })
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        let mut o = lock(&self.open);
        if let Some(n) = o.get_mut(&self.uid) {
            *n = n.saturating_sub(1);
            // No entry for a user with no connection: the map stays small.
            if *n == 0 {
                o.remove(&self.uid);
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("Not running as root: the kernel will hide which process read a decoy.");
    }
    let socket = std::env::args_os()
        .nth(1)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| SOCKET.into());
    if let Some(dir) = socket.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let _ = std::fs::remove_file(&socket);
    let listener = UnixListener::bind(&socket)?;
    // Anyone may connect; what a connection may do depends on its peer UID.
    std::fs::set_permissions(&socket, std::fs::Permissions::from_mode(0o666))?;
    let config = Arc::new(server::Config::default());
    let open = Open::default();
    eprintln!("Listening on {}", Path::new(&socket).display());
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let Ok(uid) = server::peer_uid(&stream) else {
            continue;
        };
        let Some(slot) = Slot::take(&open, uid) else {
            let mut s = &stream;
            let _ = bigman_canary_helper::send(
                &mut s,
                &bigman_canary_helper::HelperMessage::Error {
                    message: "Too many connections".into(),
                },
            );
            continue;
        };
        let config = config.clone();
        // When no thread can be started the closure is dropped, and with it the
        // connection and its slot.
        let started = std::thread::Builder::new().spawn(move || {
            let _slot = slot;
            if let Err(e) = server::serve(&config, stream, uid) {
                eprintln!("Connection from UID {uid} ended: {e:#}");
            }
        });
        if let Err(e) = started {
            eprintln!("Cannot serve a connection from UID {uid}: {e}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_slot_is_given_back_when_its_thread_panics() {
        let open = Open::default();
        let slots: Vec<_> = (0..PER_USER)
            .map(|_| Slot::take(&open, 1000).unwrap())
            .collect();
        assert!(Slot::take(&open, 1000).is_none());
        assert!(Slot::take(&open, 1001).is_some());
        assert!(!lock(&open).contains_key(&1001), "an unused entry stays");
        for slot in slots {
            let open = open.clone();
            let panicked = std::thread::spawn(move || {
                let _slot = slot;
                // Poisons the mutex as well: a panic while holding it.
                let _held = open.lock().unwrap();
                panic!("a connection thread panics");
            })
            .join();
            assert!(panicked.is_err());
        }
        assert!(lock(&open).is_empty());
        assert!(Slot::take(&open, 1000).is_some());
    }

    #[test]
    fn the_total_is_capped_across_users() {
        let open = Open::default();
        let held: Vec<_> = (0..TOTAL as u32)
            .map(|uid| Slot::take(&open, uid).unwrap())
            .collect();
        assert!(Slot::take(&open, 5000).is_none());
        assert!(!lock(&open).contains_key(&5000));
        drop(held);
        assert!(Slot::take(&open, 5000).is_some());
    }
}
