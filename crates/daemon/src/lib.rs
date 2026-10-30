//! bigmand: the per-user collector daemon. It owns the store and runs every check;
//! the CLI, desktop app and native host talk to it over a private Unix socket.
pub mod server;

use bigman_core::{
    ipc::{DaemonHealth, ErrorBody, Request, Response},
    model::now,
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
        }
    }
}

pub fn error(message: impl ToString) -> Response {
    Response::Error(ErrorBody {
        message: message.to_string(),
    })
}
