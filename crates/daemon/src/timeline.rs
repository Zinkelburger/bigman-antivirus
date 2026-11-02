//! Live collection: every source on its own thread and interval, results into the
//! store, pushes to subscribers, and the notification batch.
use crate::Daemon;
use anyhow::Result;
use bigman_core::{
    checks::{
        preview::DECISION_SECS,
        receipt::{self, Receipt},
    },
    ipc::{Decision, Live, Waiting},
    model::{now, Event, Link, Observation, Severity, SkipReason, SourceId},
    notify::{Batch, NotificationSettings},
    platform::Notifier,
    platform::Transaction,
    runner::Cancel,
    sources::{label, Ctx, Source, SourceStatus},
    store::{db::Db, Store},
};
use std::{
    collections::{BTreeMap, HashMap},
    sync::{
        atomic::Ordering,
        mpsc::{sync_channel, Receiver, SyncSender, TrySendError},
        Arc, Mutex,
    },
    thread::JoinHandle,
    time::Duration,
};

/// Pushes a subscriber may fall behind by before it is dropped; it then
/// resubscribes and reloads rather than silently missing entries.
const BACKLOG: usize = 4096;

struct Subscriber {
    events: bool,
    samples: bool,
    tx: SyncSender<Live>,
}

pub struct Timeline {
    pub db: Db,
    subscribers: Mutex<Vec<Subscriber>>,
    statuses: Mutex<BTreeMap<&'static str, SourceStatus>>,
    batch: Mutex<Batch>,
    /// Previewed transactions waiting for an answer, by receipt id.
    decisions: Mutex<HashMap<String, (std::sync::mpsc::Sender<bool>, Waiting)>>,
    notifier: Mutex<Option<Arc<dyn Notifier>>>,
    /// How long a preview waits for an answer; tests shorten it.
    pub decision_wait: Mutex<Duration>,
}

impl Timeline {
    pub fn open(store: &Store) -> Result<Self> {
        Ok(Timeline {
            db: Db::open(&store.path("bigman.db")?)?,
            subscribers: Mutex::default(),
            statuses: Mutex::default(),
            batch: Mutex::default(),
            decisions: Mutex::default(),
            notifier: Mutex::default(),
            decision_wait: Mutex::new(Duration::from_secs(DECISION_SECS)),
        })
    }

    pub fn subscribe(&self, events: bool, samples: bool) -> Receiver<Live> {
        let (tx, rx) = sync_channel(BACKLOG);
        // A window opened after a preview arrived still gets to answer it.
        if events {
            for (_, waiting) in self.decisions.lock().unwrap().values() {
                let _ = tx.try_send(Live::Preview(Box::new(waiting.clone())));
            }
        }
        self.subscribers.lock().unwrap().push(Subscriber {
            events,
            samples,
            tx,
        });
        rx
    }

    pub fn publish(&self, push: Live) {
        self.subscribers.lock().unwrap().retain(|s| {
            let wanted = match push {
                Live::Event(_) | Live::Unread(_) | Live::Preview(_) | Live::PreviewDone(_) => {
                    s.events
                }
                Live::Samples(_) => s.samples,
            };
            !wanted
                || !matches!(
                    s.tx.try_send(push.clone()),
                    Err(TrySendError::Full(_) | TrySendError::Disconnected(_))
                )
        });
    }

    pub fn publish_unread(&self) {
        if let Ok(unread) = self.db.unread() {
            self.publish(Live::Unread(unread));
        }
    }

    /// Shows a transaction's receipt before it runs and waits up to a minute for
    /// the app's answer. Nobody answering, or the hook going away, continues.
    pub fn preview(&self, r: Receipt, stop: &Cancel) -> Decision {
        let (tx, rx) = std::sync::mpsc::channel();
        let wait = *self.decision_wait.lock().unwrap();
        let waiting = Waiting {
            receipt: r.clone(),
            deadline: now() + wait.as_secs(),
        };
        self.decisions
            .lock()
            .unwrap()
            .insert(r.id.clone(), (tx, waiting.clone()));
        let t = Transaction {
            id: r.transaction.clone(),
            at: r.at,
            manager: r.manager.clone(),
            command: r.command.clone(),
            changes: r.packages.clone(),
        };
        let sentence = receipt::sentence(&r, &t);
        let _ = self.db.save_receipt(&r);
        self.publish(Live::Preview(Box::new(waiting)));
        if let Some(n) = self.notifier.lock().unwrap().as_ref() {
            n.notify(
                "A package change is waiting",
                &format!("{sentence}\nIt continues in a minute unless you stop it in Bigman."),
            );
        }
        let deadline = std::time::Instant::now() + wait;
        let answer = loop {
            if stop.load(Ordering::Relaxed) {
                break None;
            }
            match rx.recv_timeout(Duration::from_millis(250)) {
                Ok(proceed) => break Some(proceed),
                Err(_) if std::time::Instant::now() >= deadline => break None,
                Err(_) => {}
            }
        };
        self.decisions.lock().unwrap().remove(&r.id);
        let decision = Decision {
            id: r.id.clone(),
            proceed: answer.unwrap_or(true),
            by: if answer.is_some() { "you" } else { "timeout" }.into(),
        };
        self.publish(Live::PreviewDone(decision.clone()));
        let outcome = match (decision.proceed, answer.is_some()) {
            (false, _) => "stopped by you",
            (true, true) => "continued by you",
            (true, false) => "continued after a minute without an answer",
        };
        let worst = r.findings.iter().max_by_key(|f| f.severity);
        let o = Observation {
            at: now(),
            source: SourceId::Receipt,
            key: format!("preview:{}", r.id),
            rule: worst.map(|f| f.rule.clone()),
            severity: worst.map_or(Severity::Info, |f| f.severity),
            summary: format!("Previewed before it ran: {sentence}; {outcome}"),
            evidence: r.command.clone(),
            link: Some(Link::Receipt { id: r.id.clone() }),
        };
        if let Ok(e) = self.db.record(&o) {
            self.publish(Live::Event(e));
            self.publish_unread();
        }
        decision
    }

    pub fn decide(&self, id: &str, proceed: bool) -> Result<()> {
        let waiting = self
            .decisions
            .lock()
            .unwrap()
            .get(id)
            .map(|(tx, _)| tx.clone());
        match waiting {
            Some(tx) if tx.send(proceed).is_ok() => Ok(()),
            _ => anyhow::bail!("That transaction is no longer waiting; it went ahead after a minute or was answered elsewhere."),
        }
    }

    pub fn statuses(&self) -> Vec<SourceStatus> {
        self.statuses.lock().unwrap().values().cloned().collect()
    }

    pub fn notifications(&self) -> NotificationSettings {
        self.db
            .setting(NotificationSettings::KEY)
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_str(&v).ok())
            .unwrap_or_default()
    }

    pub fn set_notifications(&self, settings: &NotificationSettings) -> Result<()> {
        self.db
            .set_setting(NotificationSettings::KEY, &serde_json::to_string(settings)?)
    }

    fn set_status(
        &self,
        name: &'static str,
        last_run: Option<u64>,
        gap: Option<(SkipReason, String)>,
    ) {
        let (why, detail) = gap.map_or((None, String::new()), |(why, detail)| (Some(why), detail));
        self.statuses.lock().unwrap().insert(
            name,
            SourceStatus {
                name: name.into(),
                label: label(name).into(),
                last_run,
                ok: why.is_none(),
                why,
                detail,
            },
        );
    }

    /// One run of one source: record, push, and queue notifications.
    pub fn run_source(&self, source: &mut dyn Source, store: &Store, now: u64) -> Vec<Event> {
        let collected = match source.collect(&Ctx {
            db: &self.db,
            store,
            now,
        }) {
            Ok(c) => c,
            Err(gap) => {
                self.set_status(source.name(), Some(now), Some((gap.why, gap.detail)));
                return vec![];
            }
        };
        let mut events = vec![];
        let mut problem = None;
        for o in &collected.observations {
            match self.db.record(o) {
                Ok(e) => events.push(e),
                Err(e) => {
                    problem = Some((
                        SkipReason::Failed,
                        format!("Could not save an entry: {e:#}"),
                    ))
                }
            }
        }
        if !collected.samples.is_empty() {
            if let Err(e) = self.db.add_samples(&collected.samples) {
                problem = Some((SkipReason::Failed, format!("Could not save samples: {e:#}")));
            }
            self.publish(Live::Samples(collected.samples));
        }
        self.set_status(source.name(), Some(now), problem);
        if !events.is_empty() {
            let settings = self.notifications();
            let mut batch = self.batch.lock().unwrap();
            for e in &events {
                batch.push(&settings, e, now);
                self.publish(Live::Event(e.clone()));
            }
            drop(batch);
            self.publish_unread();
        }
        events
    }
}

/// Sleeps up to `duration`, returning early (false) once `stop` is set.
fn wait(stop: &Cancel, duration: Duration) -> bool {
    let step = Duration::from_millis(100);
    let mut left = duration;
    while !left.is_zero() {
        if stop.load(Ordering::Relaxed) {
            return false;
        }
        let nap = left.min(step);
        std::thread::sleep(nap);
        left -= nap;
    }
    !stop.load(Ordering::Relaxed)
}

/// Starts every source and the notification and housekeeping thread.
pub fn start(
    daemon: &Arc<Daemon>,
    sources: Vec<Box<dyn Source>>,
    notifier: Box<dyn Notifier>,
    stop: &Cancel,
) -> Vec<JoinHandle<()>> {
    let notifier: Arc<dyn Notifier> = Arc::from(notifier);
    *daemon.timeline.notifier.lock().unwrap() = Some(notifier.clone());
    let mut threads = vec![];
    for mut source in sources {
        daemon.timeline.set_status(source.name(), None, None);
        let (daemon, stop) = (daemon.clone(), stop.clone());
        threads.push(std::thread::spawn(move || loop {
            daemon
                .timeline
                .run_source(source.as_mut(), &daemon.store, now());
            if !wait(&stop, source.interval()) {
                break;
            }
        }));
    }
    let (daemon, stop) = (daemon.clone(), stop.clone());
    threads.push(std::thread::spawn(move || {
        let mut last_downsample = 0;
        loop {
            let now = now();
            let settings = daemon.timeline.notifications();
            let notice = daemon.timeline.batch.lock().unwrap().due(&settings, now);
            if let Some(n) = notice {
                notifier.notify(&n.title, &n.body);
            }
            if now >= last_downsample + 3600 {
                if let Err(e) = daemon.timeline.db.downsample(now) {
                    eprintln!("bigmand: could not downsample throughput history: {e:#}");
                }
                last_downsample = now;
            }
            if !wait(&stop, Duration::from_secs(1)) {
                break;
            }
        }
    }));
    threads
}
