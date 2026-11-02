use super::{cursor, gap, set_cursor, Collected, Ctx, Gap, Source};
use crate::{
    checks::receipt::{self, Receipt, AFTER_SECS},
    model::{Event, Link, Observation, Severity, SkipReason, SourceId},
    platform::{ChangeKind, PackageDb, PackageHistory, Transaction},
    runner,
    store::{
        db::{Db, EventQuery},
        digest,
    },
};
use std::time::Duration;

/// Package-manager transactions, however they were started, each with a receipt
/// of what it installed and what its scripts do.
pub struct Packages {
    history: Box<dyn PackageHistory>,
    db: Option<Box<dyn PackageDb>>,
    /// Receipts still waiting for the minute after their transaction.
    pending: Vec<(String, u64, u64)>,
}

impl Packages {
    pub fn new(history: Box<dyn PackageHistory>, db: Option<Box<dyn PackageDb>>) -> Self {
        Packages {
            history,
            db,
            pending: vec![],
        }
    }

    fn receipt(&self, store: &Db, t: &Transaction) -> Option<Receipt> {
        let db = self.db.as_deref()?;
        let hook = |package: &str, phase: &str| {
            store
                .setting(&format!("hook.{package}/{phase}"))
                .ok()
                .flatten()
        };
        let r = receipt::build(
            uuid::Uuid::new_v4().to_string(),
            t,
            db,
            &receipt::History { hook: &hook },
            &runner::cancel(),
        );
        for s in &r.scripts {
            let _ = store.set_setting(
                &format!("hook.{}/{}", s.script.package, s.script.phase),
                &digest(s.script.text.as_bytes()),
            );
        }
        let _ = store.save_receipt(&r);
        Some(r)
    }

    /// Attaches what started listening in the minute after each due transaction.
    fn finish_due(&mut self, store: &Db, now: u64) {
        let (due, waiting): (Vec<_>, Vec<_>) = self
            .pending
            .drain(..)
            .partition(|(_, _, until)| now >= *until);
        self.pending = waiting;
        for (id, since, until) in due {
            let Ok(Some(mut r)) = store.receipt(&id) else {
                continue;
            };
            let listeners: Vec<Event> = store
                .events(&EventQuery {
                    since,
                    until: Some(until),
                    sources: vec![SourceId::Listener],
                    limit: 50,
                })
                .unwrap_or_default();
            receipt::attach_after(
                &mut r,
                listeners
                    .into_iter()
                    .filter(|e| e.seen_before == 0)
                    .map(|e| e.summary)
                    .collect(),
            );
            let _ = store.save_receipt(&r);
        }
    }
}

/// "dnf installed nix and 4 more packages" / "dnf updated 32 packages".
pub fn transaction_summary(t: &Transaction) -> String {
    let verb = |k: ChangeKind| match k {
        ChangeKind::Install => "installed",
        ChangeKind::Upgrade => "updated",
        ChangeKind::Downgrade => "downgraded",
        ChangeKind::Reinstall => "reinstalled",
        ChangeKind::Remove => "removed",
    };
    let Some(first) = t.changes.first() else {
        return format!("{} ran a transaction with no package changes", t.manager);
    };
    let same = t.changes.iter().all(|c| c.kind == first.kind);
    let what = if same { verb(first.kind) } else { "changed" };
    match t.changes.len() {
        1 => format!("{} {what} {}", t.manager, first.name),
        2 => format!(
            "{} {what} {} and {}",
            t.manager, first.name, t.changes[1].name
        ),
        n if n <= 5 => format!(
            "{} {what} {} and {} more packages",
            t.manager,
            first.name,
            n - 1
        ),
        n => format!("{} {what} {n} packages", t.manager),
    }
}

impl Source for Packages {
    fn name(&self) -> &'static str {
        "packages"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(10)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let after = cursor(ctx.db, self.name());
        let transactions = self
            .history
            .transactions_after(after.as_deref())
            .map_err(|e| gap(SkipReason::Unreadable, format!("{e:#}")))?;
        self.finish_due(ctx.db, ctx.now);
        let mut observations = vec![];
        for t in &transactions {
            // On the first run, look back a week instead of importing all history.
            if after.is_none() && t.at + 7 * 86_400 < ctx.now {
                continue;
            }
            // Older transactions are listed without a receipt: the package database
            // shows today's files, not what those transactions installed.
            let receipt = if after.is_some() {
                self.receipt(ctx.db, t)
            } else {
                None
            };
            let evidence = t
                .changes
                .iter()
                .map(|c| format!("{:?} {} {}", c.kind, c.name, c.version))
                .collect::<Vec<_>>()
                .join("\n");
            let worst = receipt
                .as_ref()
                .and_then(|r| r.findings.iter().max_by_key(|f| f.severity));
            if let Some(r) = &receipt {
                self.pending
                    .push((r.id.clone(), t.at.min(ctx.now), ctx.now + AFTER_SECS));
            }
            observations.push(Observation {
                at: if t.at > 0 { t.at } else { ctx.now },
                source: SourceId::Receipt,
                key: format!("transaction:{}:{}", t.manager, t.id),
                rule: worst.map(|f| f.rule.clone()),
                severity: worst.map_or(Severity::Info, |f| f.severity),
                summary: receipt
                    .as_ref()
                    .map_or_else(|| transaction_summary(t), |r| receipt::sentence(r, t)),
                evidence: if t.command.is_empty() {
                    evidence
                } else {
                    format!("{}\n{evidence}", t.command)
                },
                link: receipt.map(|r| Link::Receipt { id: r.id }),
            });
        }
        if let Some(last) = transactions.last() {
            set_cursor(ctx.db, self.name(), &last.id);
        }
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}
