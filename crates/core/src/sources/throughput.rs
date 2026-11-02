use super::{gap, Collected, Ctx, Gap, Source};
use crate::{
    model::{Sample, SkipReason},
    platform::{InterfaceCounters, NetworkCounters},
};
use std::{collections::HashMap, time::Duration};

/// Bytes per second per interface, from cumulative counters, every second.
pub struct Throughput {
    counters: Box<dyn NetworkCounters>,
    last: HashMap<String, (u64, InterfaceCounters)>,
}

impl Throughput {
    pub fn new(counters: Box<dyn NetworkCounters>) -> Self {
        Throughput {
            counters,
            last: HashMap::new(),
        }
    }
}

impl Source for Throughput {
    fn name(&self) -> &'static str {
        "throughput"
    }
    fn interval(&self) -> Duration {
        Duration::from_secs(1)
    }
    fn collect(&mut self, ctx: &Ctx) -> Result<Collected, Gap> {
        let now = ctx.now;
        let current = self
            .counters
            .counters()
            .map_err(|e| gap(SkipReason::Unreadable, e.to_string()))?;
        let mut samples = vec![];
        for c in current {
            if let Some((at, prev)) = self.last.get(&c.iface) {
                let elapsed = now.saturating_sub(*at);
                // A counter that went backwards was reset (interface restarted); skip one sample.
                if elapsed > 0 && c.rx_bytes >= prev.rx_bytes && c.tx_bytes >= prev.tx_bytes {
                    samples.push(Sample {
                        at: now,
                        iface: c.iface.clone(),
                        rx: (c.rx_bytes - prev.rx_bytes) as f64 / elapsed as f64,
                        tx: (c.tx_bytes - prev.tx_bytes) as f64 / elapsed as f64,
                    });
                }
            }
            self.last.insert(c.iface.clone(), (now, c));
        }
        Ok(Collected {
            samples,
            ..Default::default()
        })
    }
}
