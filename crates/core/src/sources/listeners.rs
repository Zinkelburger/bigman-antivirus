use super::{gap, Collected, Ctx, Gap, Source};
use crate::{
    checks::network::{listener_findings, process_name, reach},
    model::{Observation, Report, Severity, SkipReason, SourceId},
    platform::{ProcessSockets, Protocol, Socket},
};
use std::{collections::HashSet, time::Duration};

/// The first-seen key for a listening socket.
pub fn listener_key(s: &Socket) -> String {
    format!("listener:{}:{}", process_name(s), s.local)
}

/// A program starting to listen for connections, rated by the listener rules.
pub struct Listeners {
    sockets: Box<dyn ProcessSockets>,
    previous: Option<HashSet<String>>,
}

impl Listeners {
    pub fn new(sockets: Box<dyn ProcessSockets>) -> Self {
        Listeners {
            sockets,
            previous: None,
        }
    }
}

impl Source for Listeners {
    fn name(&self) -> &'static str {
        "listeners"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(3)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let sockets = self
            .sockets
            .sockets()
            .map_err(|e| gap(SkipReason::Unreadable, e.to_string()))?;
        let mut current = HashSet::new();
        let mut observations = vec![];
        for s in sockets
            .iter()
            .filter(|s| s.protocol == Protocol::Tcp && s.listening())
        {
            let who = process_name(s);
            let key = listener_key(s);
            current.insert(key.clone());
            // The first run records what is already listening without reporting each as new.
            let Some(previous) = &self.previous else {
                continue;
            };
            if previous.contains(&key) {
                continue;
            }
            let mut r = Report::new("Listener", &key);
            listener_findings(&mut r, s);
            let worst = r.findings.iter().max_by_key(|f| f.severity);
            observations.push(Observation {
                at: ctx.now,
                source: SourceId::Listener,
                key,
                rule: worst.map(|f| f.rule.clone()),
                severity: worst.map_or(Severity::Info, |f| f.severity),
                summary: worst.map_or_else(
                    || {
                        format!(
                            "{who} started listening on port {}, {}",
                            s.local.port(),
                            reach(s)
                        )
                    },
                    |f| f.summary.clone(),
                ),
                evidence: worst.map_or_else(|| s.local.to_string(), |f| f.evidence.clone()),
                link: None,
            });
        }
        if self.previous.is_none() {
            for key in &current {
                let _ = ctx.db.touch(key, SourceId::Listener, ctx.now);
            }
        }
        self.previous = Some(current);
        Ok(Collected {
            observations,
            ..Default::default()
        })
    }
}
