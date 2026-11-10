//! One Bigman app per session. The app stays in the tray after its window closes,
//! so opening Bigman again from the application menu must bring that window back,
//! not start a second app with a second tray icon. A connection to the app's own
//! socket, next to bigmand's, is the whole message: "show your window".
use bigman_core::{ipc::socket_path, store::private_dir};
use std::{
    os::unix::{
        fs::PermissionsExt,
        net::{UnixListener, UnixStream},
    },
    path::{Path, PathBuf},
};

pub enum Claim {
    /// This is the only app. The listener is absent when the socket could not be
    /// made; the app still runs, without the hand-over from later launches.
    First(Option<UnixListener>),
    /// Another app answered and has been asked to show its window.
    AlreadyRunning,
}

/// Beside bigmand's socket, so `--socket` profiles and tests stay separate apps.
fn path() -> Option<PathBuf> {
    Some(socket_path().ok()?.parent()?.join("app.sock"))
}

pub fn claim() -> Claim {
    match path() {
        Some(path) => claim_at(&path),
        None => Claim::First(None),
    }
}

fn claim_at(path: &Path) -> Claim {
    if UnixStream::connect(path).is_ok() {
        return Claim::AlreadyRunning;
    }
    // Nothing answered: a socket file here was left by an app that was killed.
    let _ = std::fs::remove_file(path);
    let listener = path
        .parent()
        .and_then(|dir| private_dir(dir).ok())
        .and_then(|()| UnixListener::bind(path).ok());
    if listener.is_some() {
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
    }
    Claim::First(listener)
}

/// Calls `show` for each later launch, for as long as the app runs.
pub fn serve(listener: UnixListener, show: impl Fn() + Send + 'static) {
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            if stream.is_ok() {
                show();
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    /// Unix socket paths are limited to 107 bytes, so the test socket lives in the
    /// runtime directory when there is one.
    fn scratch(name: &str) -> PathBuf {
        let base = std::env::var_os("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        let dir = base.join(format!("bigman-app-test-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir.join("app.sock")
    }

    #[test]
    fn a_second_launch_finds_the_first_and_asks_it_to_show() {
        let path = scratch("second");
        let Claim::First(Some(listener)) = claim_at(&path) else {
            panic!("the first launch should own the socket");
        };
        let (shown, was_shown) = mpsc::channel();
        serve(listener, move || {
            let _ = shown.send(());
        });
        assert!(matches!(claim_at(&path), Claim::AlreadyRunning));
        was_shown
            .recv_timeout(Duration::from_secs(5))
            .expect("the first app is asked to show its window");
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn a_socket_left_by_a_killed_app_is_replaced() {
        let path = scratch("stale");
        let Claim::First(Some(listener)) = claim_at(&path) else {
            panic!("the first launch should own the socket");
        };
        // Dropping the listener leaves the file behind, as a killed app does.
        drop(listener);
        assert!(path.exists());
        assert!(matches!(claim_at(&path), Claim::First(Some(_))));
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }
}
