use super::{cursor, gap, set_cursor, Collected, Ctx, Gap, Source};
use crate::{
    checks::denials,
    model::{Observation, Severity, SkipReason, SourceId},
    platform::Journal,
};
use std::{collections::HashSet, time::Duration};

/// SELinux and AppArmor denials from the system journal, as plain sentences.
pub struct Denials {
    journal: Box<dyn Journal>,
}

impl Denials {
    pub fn new(journal: Box<dyn Journal>) -> Self {
        Denials { journal }
    }
}

impl Source for Denials {
    fn name(&self) -> &'static str {
        "denials"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(5)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let after = cursor(ctx.db, self.name());
        let Some((records, next)) = self
            .journal
            .denials_after(after.as_deref())
            .map_err(|e| gap(SkipReason::Failed, format!("{e:#}")))?
        else {
            return Err(gap(
                SkipReason::PermissionDenied,
                "Reading the system journal needs membership of systemd-journal, adm or wheel.",
            ));
        };
        let mut observations = vec![];
        // A program that hits the same wall repeatedly gives one entry per run.
        let mut this_run = HashSet::new();
        for record in records {
            let Some(d) = denials::parse(&record.message) else {
                continue;
            };
            if !this_run.insert(d.key.clone()) {
                continue;
            }
            observations.push(Observation {
                at: if record.at > 0 { record.at } else { ctx.now },
                source: SourceId::Denial,
                key: d.key.clone(),
                rule: Some(
                    if d.module == "selinux" {
                        "denial.selinux"
                    } else {
                        "denial.apparmor"
                    }
                    .into(),
                ),
                severity: Severity::Info,
                summary: denials::sentence(&d),
                evidence: d.record,
                link: None,
            });
        }
        if let Some(next) = next {
            set_cursor(ctx.db, self.name(), &next);
        }
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}
