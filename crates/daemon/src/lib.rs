//! bigmand: the per-user collector daemon. It owns the store and runs every check;
//! the CLI, desktop app and native host talk to it over a private Unix socket.
pub mod server;

use bigman_core::{
    actions,
    checks::{canary, changes, doctor, inspect, integrations, packages, pdf, route, scan, setup},
    ipc::{DaemonHealth, ErrorBody, ProtectionStatus, Request, Response},
    model::{now, Report},
    runner::Cancel,
    store::Store,
};

/// Shared daemon state handed to every connection.
pub struct Daemon {
    pub store: Store,
    pub started: u64,
}

pub fn error(message: impl ToString) -> Response {
    Response::Error(ErrorBody {
        message: message.to_string(),
    })
}

fn report(result: anyhow::Result<Report>) -> Response {
    match result {
        Ok(r) => Response::Report(Box::new(r)),
        Err(e) => error(format!("{e:#}")),
    }
}

impl Daemon {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            started: now(),
        }
    }

    /// Runs one request. `stop` is set when the client disconnects, so long checks
    /// end early and report themselves as incomplete.
    pub fn handle(&self, request: Request, stop: &Cancel) -> Response {
        let store = &self.store;
        let result = (|| -> anyhow::Result<Response> {
            Ok(match request {
                Request::Health => Response::Health(DaemonHealth {
                    version: env!("CARGO_PKG_VERSION").into(),
                    pid: std::process::id(),
                    started: self.started,
                    state_dir: store.root.clone(),
                }),
                Request::Status => Response::Status(Box::new(self.status()?)),
                Request::Doctor => report(doctor::doctor(store, stop)),
                Request::History { limit } => {
                    let mut reports = store.history()?;
                    reports.truncate(limit as usize);
                    Response::History(reports)
                }
                Request::Report { id } => report(store.report(&id)),
                Request::Scan { path } => report(scan::scan(store, &path, stop)),
                Request::UpdateSignatures => report(scan::update(store, stop)),
                Request::Pdf { path } => report(pdf::scan_pdf(store, &path, stop)),
                Request::InspectAny { target } => report(route::inspect_any(store, &target, stop)),
                Request::Inspect { target } => report(inspect::inspect(store, &target, stop)),
                Request::Packages { path } => report(packages::packages(store, &path)),
                Request::Changes => {
                    let r = changes::changes(store)?;
                    store.save(&r)?;
                    Response::Report(Box::new(r))
                }
                Request::Audit { tool, path } => {
                    report(integrations::integration(store, &tool, &path, stop))
                }
                Request::Yara { rules, target } => {
                    report(integrations::yara(store, &rules, &target, stop))
                }
                Request::CanaryTest { id } => report(canary::setup::verify(store, &id, stop)),
                Request::ScanItems { report } => {
                    Response::ScanItems(scan::inventory(store, &report)?)
                }
                Request::Quarantined => Response::Quarantined(scan::quarantined(store)?),
                Request::Canaries => Response::Canaries(Box::new(canary::state(store)?)),
                Request::ToolGroups => Response::ToolGroups(setup::tool_groups()),
                Request::OpenTool { tool } => {
                    setup::open_tool(&tool)?;
                    Response::Done(Box::new(actions::ActionOutcome {
                        message: "Opened.".into(),
                        report: None,
                        undo: None,
                    }))
                }
                Request::Preview { action } => {
                    Response::Plan(Box::new(actions::preview(&action, store)?))
                }
                Request::Apply { action } => {
                    Response::Done(Box::new(actions::apply(&action, store, stop)?))
                }
            })
        })();
        result.unwrap_or_else(|e| error(format!("{e:#}")))
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
            downloads: setup::downloads(),
        })
    }
}
