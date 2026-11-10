use super::{Collected, Ctx, Gap, Source};
use crate::{
    checks::advisories::{advisories, Options},
    model::SourceId,
    runner,
    store::db::EventQuery,
};
use std::time::Duration;

const DAY: u64 = 86_400;
/// After a package transaction, wait for the package manager to finish.
const SETTLE: u64 = 120;

/// Re-checks security updates once a day and shortly after each package
/// transaction, so updates you installed stop being listed on their own. The
/// saved report reaches the timeline through the reports source.
#[derive(Default)]
pub struct Updates;

const LAST: &str = "advisories.last_run";

impl Source for Updates {
    fn name(&self) -> &'static str {
        "updates"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(60)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let Some(last) = ctx
            .db
            .setting(LAST)
            .ok()
            .flatten()
            .and_then(|v| v.parse::<u64>().ok())
        else {
            // A fresh install waits ten minutes before its first check, not at start.
            let _ = ctx
                .db
                .set_setting(LAST, &(ctx.now + 600).saturating_sub(DAY).to_string());
            return Ok(Collected::default());
        };
        let transaction = ctx
            .db
            .events(&EventQuery {
                since: last + 1,
                until: None,
                sources: vec![SourceId::Receipt],
                limit: 1,
            })
            .unwrap_or_default()
            .into_iter()
            .next();
        let after_change = transaction.is_some_and(|t| t.at + SETTLE <= ctx.now);
        if ctx.now < last + DAY && !after_change {
            return Ok(Collected::default());
        }
        let _ = ctx.db.set_setting(LAST, &ctx.now.to_string());
        advisories(ctx.store, Options::default(), &runner::cancel())
            .map_err(|e| super::gap(crate::model::SkipReason::Failed, format!("{e:#}")))?;
        Ok(Collected::default())
    }
}
