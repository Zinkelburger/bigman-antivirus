use super::{cursor, gap, set_cursor, Collected, Ctx, Gap, Source};
use crate::{
    model::{Observation, Severity, SkipReason, SourceId},
    platform::{ChangeKind, PackageHistory, Transaction},
};
use std::time::Duration;

/// Package-manager transactions, however they were started.
pub struct Packages {
    history: Box<dyn PackageHistory>,
}

impl Packages {
    pub fn new(history: Box<dyn PackageHistory>) -> Self {
        Packages { history }
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
        let mut observations = vec![];
        for t in &transactions {
            // On the first run, look back a week instead of importing all history.
            if after.is_none() && t.at + 7 * 86_400 < ctx.now {
                continue;
            }
            let evidence = t
                .changes
                .iter()
                .map(|c| format!("{:?} {} {}", c.kind, c.name, c.version))
                .collect::<Vec<_>>()
                .join("\n");
            observations.push(Observation {
                at: if t.at > 0 { t.at } else { ctx.now },
                source: SourceId::Receipt,
                key: format!("transaction:{}:{}", t.manager, t.id),
                rule: None,
                severity: Severity::Info,
                summary: transaction_summary(t),
                evidence: if t.command.is_empty() {
                    evidence
                } else {
                    format!("{}\n{evidence}", t.command)
                },
                link: None,
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
