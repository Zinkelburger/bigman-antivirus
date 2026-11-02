use super::{Collected, Ctx, Gap, Source};
use crate::{
    checks::advisories::{advisories, Options},
    model::SourceId,
    runner,
    store::db::EventQuery,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ts_rs::TS;

const DAY: u64 = 86_400;
/// After a package transaction, wait for the package manager to finish.
const SETTLE: u64 = 120;

/// Choices that are not notifications.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Preferences {
    /// Check for security updates once a day and after package changes. This uses
    /// the network for public data only; no package list is sent.
    pub check_updates: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            check_updates: true,
        }
    }
}

impl Preferences {
    pub const KEY: &'static str = "preferences";

    pub fn load(db: &crate::store::db::Db) -> Preferences {
        db.setting(Self::KEY)
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_str(&v).ok())
            .unwrap_or_default()
    }
}

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
        if !Preferences::load(ctx.db).check_updates {
            return Ok(Collected::default());
        }
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
