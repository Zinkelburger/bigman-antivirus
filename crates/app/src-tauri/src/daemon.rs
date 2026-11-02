//! Forwards UI requests to bigmand. The app never runs a check itself.
use bigman_core::{
    ipc::{
        client::{Canceller, Client},
        Request, Response,
    },
    runner,
};
use serde::Serialize;
use std::{
    collections::HashMap,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
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
    /// Requests the UI may cancel, by the token it chose.
    running: Mutex<HashMap<String, Canceller>>,
}

const RESTART_COOLDOWN: Duration = Duration::from_secs(10);

impl Launcher {
    /// One connection per request: a long scan never blocks a health poll.
    pub fn request(&self, request: Request) -> Result<Response, RequestError> {
        self.request_with_token(request, None)
    }

    pub fn request_with_token(
        &self,
        request: Request,
        token: Option<String>,
    ) -> Result<Response, RequestError> {
        let mut client = match Client::connect() {
            Ok(client) => client,
            Err(e) => {
                self.start_daemon();
                return Err(RequestError::Disconnected(format!("{e:#}")));
            }
        };
        if let (Some(token), Ok(canceller)) = (&token, client.canceller()) {
            self.lock_running().insert(token.clone(), canceller);
        }
        let result = client
            .request(request)
            .map_err(|e| RequestError::Failed(format!("{e:#}")));
        if let Some(token) = token {
            self.lock_running().remove(&token);
        }
        result
    }

    pub fn cancel(&self, token: &str) {
        if let Some(canceller) = self.lock_running().get(token) {
            canceller.cancel();
        }
    }

    fn lock_running(&self) -> std::sync::MutexGuard<'_, HashMap<String, Canceller>> {
        self.running.lock().unwrap_or_else(|p| p.into_inner())
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
    launcher: tauri::State<'_, Arc<Launcher>>,
    request: Request,
    token: Option<String>,
) -> Result<Response, RequestError> {
    let launcher = launcher.inner().clone();
    tauri::async_runtime::spawn_blocking(move || launcher.request_with_token(request, token))
        .await
        .map_err(|e| RequestError::Failed(e.to_string()))?
}

/// Cancels the request started with this token; its cancelled report still arrives.
#[tauri::command]
pub fn daemon_cancel(launcher: tauri::State<'_, Arc<Launcher>>, token: String) {
    launcher.cancel(&token);
}
