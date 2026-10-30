//! bigmand: the per-user collector daemon. It owns the store and runs every check;
//! the CLI, desktop app and native host talk to it over a private Unix socket.
pub mod server;

use bigman_core::{
    checks::{canary, doctor},
    ipc::{DaemonHealth, ErrorBody, ProtectionStatus, Request, Response},
    model::now,
    runner,
    store::Store,
};

/// Shared daemon state handed to every connection.
pub struct Daemon {
    pub store: Store,
    pub started: u64,
}

impl Daemon {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            started: now(),
        }
    }
    pub fn handle(&self, request: Request) -> Response {
        match request {
            Request::Health => Response::Health(DaemonHealth {
                version: env!("CARGO_PKG_VERSION").into(),
                pid: std::process::id(),
                started: self.started,
                state_dir: self.store.root.clone(),
            }),
            Request::Status => match self.status() {
                Ok(status) => Response::Status(Box::new(status)),
                Err(e) => error(e),
            },
            Request::Doctor => match doctor::doctor(&self.store, &runner::cancel()) {
                Ok(report) => Response::Report(Box::new(report)),
                Err(e) => error(e),
            },
            Request::History { limit } => match self.store.history() {
                Ok(mut reports) => {
                    reports.truncate(limit as usize);
                    Response::History(reports)
                }
                Err(e) => error(e),
            },
        }
    }
    fn status(&self) -> anyhow::Result<ProtectionStatus> {
        let last_scan = self
            .store
            .history()?
            .into_iter()
            .find(|r| r.kind == "File scan");
        Ok(ProtectionStatus {
            checked_at: now(),
            tools: doctor::tools(),
            signatures: doctor::signatures(&self.store),
            monitor: canary::setup::health(&self.store),
            last_scan,
        })
    }
}

pub fn error(message: impl ToString) -> Response {
    Response::Error(ErrorBody {
        message: message.to_string(),
    })
}
