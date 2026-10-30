//! Forwards UI requests to bigmand. The app never runs a check itself.
use bigman_core::{
    ipc::{client::Client, Request, Response},
    runner,
};
use serde::Serialize;
use std::{
    process::{Command, Stdio},
    sync::Mutex,
    time::{Duration, Instant},
};

/// A failed request, as the UI sees it: the daemon was unreachable, or it was
/// reachable and the exchange failed.
#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "message", rename_all = "snake_case")]
pub enum RequestError {
    Disconnected(String),
    Failed(String),
}

/// Starts bigmand when it does not answer, at most once per cooldown so a daemon
/// that keeps exiting is not restarted in a loop.
#[derive(Default)]
pub struct Launcher {
    last_start: Mutex<Option<Instant>>,
}

const RESTART_COOLDOWN: Duration = Duration::from_secs(10);

impl Launcher {
    /// One connection per request: a long scan never blocks a health poll.
    pub fn request(&self, request: Request) -> Result<Response, RequestError> {
        match Client::connect() {
            Ok(mut client) => client
                .request(request)
                .map_err(|e| RequestError::Failed(format!("{e:#}"))),
            Err(e) => {
                self.start_daemon();
                Err(RequestError::Disconnected(format!("{e:#}")))
            }
        }
    }

    fn start_daemon(&self) {
        let mut last = self.last_start.lock().unwrap_or_else(|p| p.into_inner());
        if last.is_some_and(|t| t.elapsed() < RESTART_COOLDOWN) {
            return;
        }
        *last = Some(Instant::now());
        let started = runner::sibling_executable("bigmand").and_then(|exe| {
            let mut cmd = Command::new(exe);
            cmd.stdin(Stdio::null()).stdout(Stdio::null());
            runner::spawn_detached(&mut cmd)
        });
        if let Err(e) = started {
            eprintln!("bigman-app: could not start bigmand: {e:#}");
        }
    }
}

#[tauri::command]
pub async fn daemon_request(
    launcher: tauri::State<'_, std::sync::Arc<Launcher>>,
    request: Request,
) -> Result<Response, RequestError> {
    let launcher = launcher.inner().clone();
    tauri::async_runtime::spawn_blocking(move || launcher.request(request))
        .await
        .map_err(|e| RequestError::Failed(e.to_string()))?
}
