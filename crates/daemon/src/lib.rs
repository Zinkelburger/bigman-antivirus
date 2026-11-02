//! bigmand: the per-user collector daemon. It owns the store and runs every check;
//! the CLI, desktop app and native host talk to it over a private Unix socket.
pub mod server;
pub mod timeline;

use bigman_core::{
    actions,
    checks::{canary, changes, doctor, inspect, integrations, packages, pdf, route, scan, setup},
    ipc::{DaemonHealth, ErrorBody, ProtectionStatus, Request, Response},
    model::{now, Report, SampleSeries},
    platform::{linux::ProcSockets, ProcessSockets},
    runner::Cancel,
    sources::network_snapshot,
    store::{db::EventQuery, Store},
};
use timeline::Timeline;

/// Shared daemon state handed to every connection.
pub struct Daemon {
    pub store: Store,
    pub timeline: Timeline,
    /// Where the Network tab's tables come from; tests substitute a fake.
    pub sockets: Box<dyn ProcessSockets>,
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
    /// Opens the timeline store. Collection starts separately, with `timeline::start`.
    pub fn new(store: Store) -> anyhow::Result<Self> {
        Ok(Self {
            timeline: Timeline::open(&store)?,
            sockets: Box::new(ProcSockets),
            store,
            started: now(),
        })
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
                Request::Advisories { refresh, offline } => {
                    report(bigman_core::checks::advisories::advisories(
                        store,
                        bigman_core::checks::advisories::Options { refresh, offline },
                        stop,
                    ))
                }
                Request::Changes => {
                    let r = changes::changes(store, stop)?;
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
                Request::Rules => Response::Rules(bigman_core::rules::infos()),
                Request::OpenTool { tool } => {
                    setup::open_tool(&tool)?;
                    Response::Done(Box::new(actions::ActionOutcome {
                        message: "Opened.".into(),
                        report: None,
                        undo: None,
                    }))
                }
                Request::Events {
                    since,
                    until,
                    sources,
                    limit,
                } => Response::Events(self.timeline.db.events(&EventQuery {
                    since,
                    until,
                    sources,
                    limit: limit.min(20_000),
                })?),
                Request::Unread => Response::Unread(self.timeline.db.unread()?),
                Request::PreviewTransaction { receipt } => {
                    Response::Decision(self.timeline.preview(*receipt, stop))
                }
                Request::Decide { id, proceed } => {
                    self.timeline.decide(&id, proceed)?;
                    Response::Done(Box::new(actions::ActionOutcome {
                        message: if proceed {
                            "The transaction continues."
                        } else {
                            "The transaction was stopped."
                        }
                        .into(),
                        report: None,
                        undo: None,
                    }))
                }
                Request::Receipt { id } => match self.timeline.db.receipt(&id)? {
                    Some(r) => Response::Receipt(Box::new(r)),
                    None => error(format!("No receipt {id}")),
                },
                Request::MarkSeen { through } => {
                    self.timeline.db.mark_seen(through)?;
                    self.timeline.publish_unread();
                    Response::Unread(self.timeline.db.unread()?)
                }
                Request::Samples {
                    since,
                    until,
                    points,
                } => {
                    let (bucket, samples) =
                        self.timeline
                            .db
                            .samples(since, until, u64::from(points.clamp(1, 2000)))?;
                    Response::Samples(SampleSeries { bucket, samples })
                }
                Request::Network => Response::Network(Box::new(network_snapshot(
                    self.sockets.as_ref(),
                    &self.timeline.db,
                    now(),
                ))),
                Request::Sources => Response::Sources(self.timeline.statuses()),
                Request::Notifications => Response::Notifications(self.timeline.notifications()),
                Request::Preferences => Response::Preferences(
                    bigman_core::sources::Preferences::load(&self.timeline.db),
                ),
                Request::SetPreferences { preferences } => {
                    self.timeline.db.set_setting(
                        bigman_core::sources::Preferences::KEY,
                        &serde_json::to_string(&preferences)?,
                    )?;
                    Response::Preferences(preferences)
                }
                Request::SetNotifications { settings } => {
                    self.timeline.set_notifications(&settings)?;
                    Response::Notifications(settings)
                }
                // Served by the connection loop, which streams it.
                Request::Subscribe { .. } => error("Subscribe needs a connection of its own"),
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
            receipt_hook: bigman_core::checks::hooks::installed().map(|(_, path)| path),
        })
    }
}
