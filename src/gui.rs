use crate::{
    model::{Report, Severity},
    runner::{self, Cancel},
    store::Store,
};
use eframe::egui;
use std::{
    path::PathBuf,
    sync::{atomic::Ordering, mpsc, Arc},
    time::{Duration, Instant},
};
#[derive(Clone, PartialEq)]
enum Page {
    Overview,
    Files,
    Installers,
    Changes,
    Canaries,
    Tools,
    History,
    Quarantine,
}
#[derive(Clone)]
enum Action {
    Doctor,
    Scan(String),
    Pdf(String),
    Update,
    Inspect(String),
    Packages(String),
    Policy(String),
    Changes,
    AcceptBaseline(String),
    CreateCanary(String, Option<String>),
    ArmCanary(String),
    StartMonitor,
    StopMonitor,
    TestCanary(String),
    Disarm(String),
    Install(String),
    Services(bool),
    Sandbox(String, String),
    Quarantine(String, usize),
    Restore(String),
    Audit(String, String),
    Yara(String, String),
}
enum Outcome {
    Report(Report),
    Message(String),
}
fn perform(store: &Store, a: Action, stop: &Cancel) -> anyhow::Result<Outcome> {
    use Action::*;
    let report=match a{
 Doctor=>crate::system::doctor(store,stop)?,Scan(p)=>crate::scanner::scan(store,PathBuf::from(p).as_path(),stop)?,Pdf(p)=>crate::pdf_scanner::scan_pdf(store,PathBuf::from(p).as_path(),stop)?,Update=>crate::scanner::update(store,stop)?,Inspect(p)=>crate::inspect::inspect(store,&p,stop)?,Packages(p)=>crate::inspect::packages(store,PathBuf::from(p).as_path())?,Changes=>{let r=crate::system::changes(store)?;store.save(&r)?;r},
 AcceptBaseline(id)=>crate::system::accept_reviewed_baseline(store,&id)?,
 Sandbox(id,hash)=>crate::inspect::sandbox(store,&id,&hash,stop)?,Audit(tool,p)=>crate::system::integration(store,&tool,PathBuf::from(p).as_path(),stop)?,Yara(r,p)=>crate::system::yara(store,PathBuf::from(r).as_path(),PathBuf::from(p).as_path(),stop)?,
 Policy(p)=>return Ok(Outcome::Message(crate::inspect::policy(PathBuf::from(p).as_path(),true)?)),
 CreateCanary(p,url)=>return Ok(Outcome::Message(format!("Created decoy {}\nStart monitoring, then use Test and verify the resulting alert. Hosted URLs contact their provider only when activated by a reader.",crate::canary::create(store,PathBuf::from(p).as_path(),url.as_deref())?.path.display()))),
 ArmCanary(p)=>crate::canary_setup::arm(store,PathBuf::from(p).as_path(),false,stop)?,
 StartMonitor=>crate::canary_setup::start_monitor(store,false,stop)?,
 StopMonitor=>crate::canary_setup::stop_monitor(store,false,stop)?,
 TestCanary(id)=>crate::canary_setup::verify(store,&id,stop)?,
 Disarm(id)=>{crate::canary::disarm(store,&id)?;return Ok(Outcome::Message("Canary disarmed; its file was preserved.".into()));},
 Install(tool)=>return Ok(Outcome::Message(crate::setup::install_tool(&tool,true,stop)?)),Services(remove)=>return Ok(Outcome::Message(crate::setup::services(store,true,remove,stop)?)),
 Quarantine(id,index)=>return Ok(Outcome::Message(format!("Recovery copy saved; original quarantined. ID: {}",crate::scanner::quarantine(store,&id,index)?.id))),Restore(id)=>return Ok(Outcome::Message(format!("Restored without executable permissions: {}",crate::scanner::restore(store,&id)?.display()))),
 };
    Ok(Outcome::Report(report))
}
struct Job {
    rx: mpsc::Receiver<Result<Outcome, String>>,
    stop: Cancel,
}
struct App {
    store: Store,
    page: Page,
    path: String,
    installer: String,
    project: String,
    rules: String,
    decoy: String,
    hosted: String,
    report: Option<Arc<Report>>,
    history: Vec<Report>,
    message: String,
    job: Option<Job>,
    pending: Option<(String, Action)>,
    refresh: Instant,
    tool: String,
}
impl App {
    fn new(store: Store) -> Self {
        Self {
            decoy: crate::canary_setup::default_path().to_string_lossy().into(),
            history: store.history().unwrap_or_default(),
            store,
            page: Page::Overview,
            path: crate::setup::downloads().to_string_lossy().into(),
            installer: String::new(),
            project: String::new(),
            rules: String::new(),
            hosted: String::new(),
            report: None,
            message: String::new(),
            job: None,
            pending: None,
            refresh: Instant::now(),
            tool: "clamav".into(),
        }
    }
    fn start(&mut self, a: Action) {
        if self.job.is_some() {
            return;
        }
        let store = self.store.clone();
        let stop = runner::cancel();
        let cancel = stop.clone();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let result = perform(&store, a, &cancel).map_err(|e| e.to_string());
            let _ = tx.send(result);
        });
        self.job = Some(Job { rx, stop });
        self.message = "Working…".into();
    }
    fn confirm(&mut self, text: String, a: Action) {
        self.pending = Some((text, a));
    }
    fn results(&mut self, ui: &mut egui::Ui) {
        if let Some(r) = self.report.clone() {
            ui.separator();
            ui.heading(&r.kind);
            ui.label(crate::model::clean(&r.subject));
            if r.errors.is_empty() && r.findings.iter().any(|f| f.rule == "canary.verified") {
                ui.colored_label(egui::Color32::LIGHT_GREEN, "Local read detection verified");
            } else {
                ui.strong(r.status());
            }
            for error in &r.errors {
                ui.colored_label(
                    egui::Color32::from_rgb(225, 150, 65),
                    format!("Incomplete: {}", crate::model::clean(error)),
                );
            }
            for finding in &r.findings {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    let color = match finding.severity {
                        Severity::High => egui::Color32::from_rgb(245, 110, 100),
                        Severity::Review => egui::Color32::from_rgb(230, 185, 90),
                        Severity::Info => egui::Color32::from_rgb(125, 185, 220),
                    };
                    ui.colored_label(color, &finding.summary);
                    ui.label(finding.evidence.chars().take(4096).collect::<String>());
                    ui.label(format!("Next: {}", finding.action));
                    ui.small(&finding.rule);
                });
            }
            for c in &r.coverage {
                ui.small(c);
            }
            if let Some(hash) = &r.sha256 {
                ui.monospace(format!("SHA-256 {hash}"));
            }
            ui.collapsing("Technical details", |ui| {
                ui.monospace(crate::model::clean(
                    &r.details.chars().take(100_000).collect::<String>(),
                ));
                if r.details.chars().count() > 100_000 {
                    ui.label("Display truncated. Export the report for complete details.");
                }
            });
            ui.horizontal(|ui|{if ui.button("Export report…").clicked(){if let Some(p)=rfd::FileDialog::new().set_file_name(format!("bigman-{}.json",r.id)).save_file(){self.message=match serde_json::to_vec_pretty(r.as_ref()).map_err(anyhow::Error::from).and_then(|b|std::fs::write(p,b).map_err(Into::into)){Ok(_)=>"Report exported. It may contain local paths and script content.".into(),Err(e)=>e.to_string()};}}
 if r.kind=="Installer inspection"{if let Some(hash)=&r.sha256{if ui.button("Rehearse in restricted environment…").clicked(){self.confirm(format!("Execute saved installer {}\nSHA-256 {}\nUses a disposable offline filesystem with no home directory or credentials. May fail due to restrictions. This runs code; it does not establish safety or install onto the host.",r.id,hash),Action::Sandbox(r.id.clone(),hash.clone()));}}}});
            if r.kind == "File scan" {
                if let Ok(items) = crate::scanner::inventory(&self.store, &r.id) {
                    for (i, item) in items.iter().enumerate().filter(|(_, v)| v.detected) {
                        if ui
                            .button(format!(
                                "Quarantine {}…",
                                crate::model::clean(&item.path.to_string_lossy())
                            ))
                            .clicked()
                        {
                            self.confirm(format!("Save a private recovery copy and remove this unchanged file from its original location:\n{}\nSignature: {}\nHash: {}\nNo other files will be removed.",item.path.display(),item.signature,item.hash),Action::Quarantine(r.id.clone(),i));
                        }
                    }
                }
            }
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if let Some(job) = &self.job {
            match job.rx.try_recv() {
                Ok(result) => {
                    self.job = None;
                    match result {
                        Ok(Outcome::Report(r)) => {
                            self.message =
                                "Check completed. Review findings and coverage below.".into();
                            self.report = Some(Arc::new(r));
                        }
                        Ok(Outcome::Message(m)) => self.message = m,
                        Err(e) => self.message = format!("Could not complete: {e}"),
                    };
                    match self.store.history() {
                        Ok(h) => self.history = h,
                        Err(e) => self.message = format!("History unavailable: {e}"),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.job = None;
                    self.message = "Worker stopped unexpectedly; check incomplete.".into();
                }
                _ => {}
            }
        }
        if self.refresh.elapsed() > Duration::from_secs(3) {
            if let Ok(h) = self.store.history() {
                self.history = h;
            }
            self.refresh = Instant::now();
        }
        ctx.request_repaint_after(Duration::from_millis(500));
        egui::SidePanel::left("navigation")
            .resizable(false)
            .default_width(185.)
            .show(ctx, |ui| {
                ui.add_space(12.);
                ui.heading("Bigman");
                ui.label("Security companion");
                ui.separator();
                for (page, label) in [
                    (Page::Overview, "Check my system"),
                    (Page::Files, "Scan files"),
                    (Page::Installers, "Inspect installs"),
                    (Page::Changes, "Startup changes"),
                    (Page::Canaries, "Canary tripwires"),
                    (Page::Tools, "Tools & setup"),
                    (Page::History, "History"),
                    (Page::Quarantine, "Quarantine"),
                ] {
                    ui.selectable_value(&mut self.page, page, label);
                }
                ui.separator();
                ui.small("Deterministic local checks. No AI or automatic uploads.");
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let busy = self.job.is_some();
                if busy {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Check in progress");
                        if ui.button("Cancel job").clicked() {
                            if let Some(j) = &self.job {
                                j.stop.store(true, Ordering::Relaxed);
                            }
                        }
                    });
                }
                if !self.message.is_empty() {
                    ui.label(crate::model::clean(&self.message));
                    ui.separator();
                }
                ui.add_enabled_ui(!busy, |ui| self.draw_page(ui));
                self.results(ui);
            });
        });
        if let Some((description, action)) = self.pending.clone() {
            egui::Window::new("Review this change")
                .collapsible(false)
                .resizable(true)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(400.)
                        .show(ui, |ui| {
                            ui.label(description);
                        });
                    ui.horizontal(|ui| {
                        if ui.button("Apply").clicked() {
                            self.pending = None;
                            self.start(action);
                        }
                        if ui.button("Cancel").clicked() {
                            self.pending = None;
                        }
                    });
                });
        }
    }
}
impl App {
    fn draw_page(&mut self, ui: &mut egui::Ui) {
        match self.page {
            Page::Overview => {
                ui.heading("What needs attention?");
                ui.label("Check protection health, available tools, credential-storage configuration and listening applications.");
                if ui.button("Run read-only checkup").clicked() {
                    self.start(Action::Doctor);
                }
                ui.add_space(12.);
                ui.label("Getting started: check your system, install ClamAV if needed, update definitions, scan Downloads, then create and test a canary.");
                if let Some(last) = self.history.first() {
                    ui.label(format!(
                        "Latest activity: {} — {}",
                        last.kind,
                        last.status()
                    ));
                }
            }
            Page::Files => {
                ui.heading("Scan files");
                ui.label("ClamAV checks file snapshots. Nothing is deleted automatically.");
                ui.text_edit_singleline(&mut self.path);
                ui.horizontal(|ui| {
                    if ui.button("Choose file…").clicked() {
                        if let Some(p) = rfd::FileDialog::new().pick_file() {
                            self.path = p.to_string_lossy().into();
                        }
                    }
                    if ui.button("Choose folder…").clicked() {
                        if let Some(p) = rfd::FileDialog::new().pick_folder() {
                            self.path = p.to_string_lossy().into();
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Scan with ClamAV").clicked() {
                        self.start(Action::Scan(self.path.clone()));
                    }
                    if ui.button("Inspect PDF structure").clicked() {
                        self.start(Action::Pdf(self.path.clone()));
                    }
                    if ui.button("Update definitions").clicked() {
                        self.start(Action::Update);
                    }
                });
            }
            Page::Installers => {
                ui.heading("Understand an install before running it");
                ui.label("Enter an HTTPS installer URL or select a local script. Inspection never executes it.");
                ui.text_edit_singleline(&mut self.installer);
                ui.horizontal(|ui| {
                    if ui.button("Choose script…").clicked() {
                        if let Some(p) = rfd::FileDialog::new().pick_file() {
                            self.installer = p.to_string_lossy().into();
                        }
                    }
                    if ui.button("Inspect and save exact bytes").clicked() {
                        self.start(Action::Inspect(self.installer.clone()));
                    }
                });
                ui.separator();
                ui.label("Package scripts: select a project, installed node_modules tree or npm .tgz archive.");
                ui.text_edit_singleline(&mut self.project);
                ui.horizontal(|ui| {
                    if ui.button("Choose project…").clicked() {
                        if let Some(p) = rfd::FileDialog::new().pick_folder() {
                            self.project = p.to_string_lossy().into();
                        }
                    }
                    if ui.button("List package scripts").clicked() {
                        self.start(Action::Packages(self.project.clone()));
                    }
                    if ui.button("Disable automatic install scripts…").clicked() {
                        match crate::inspect::policy(PathBuf::from(&self.project).as_path(), false)
                        {
                            Ok(p) => self.confirm(p, Action::Policy(self.project.clone())),
                            Err(e) => self.message = e.to_string(),
                        }
                    }
                });
            }
            Page::Changes => {
                ui.heading("Startup and persistence changes");
                ui.label("Review new and changed startup entries. A baseline records what was present; it does not certify that those entries are trustworthy.");
                if ui.button("Compare current startup configuration").clicked() {
                    self.start(Action::Changes);
                }
                let reviewed = self
                    .report
                    .as_ref()
                    .filter(|r| r.kind == "Persistence changes" && r.errors.is_empty())
                    .map(|r| r.id.clone());
                if ui
                    .add_enabled(
                        reviewed.is_some(),
                        egui::Button::new("Accept reviewed configuration as baseline…"),
                    )
                    .clicked()
                {
                    self.confirm("Save the exact startup inventory in the report below as the new baseline. Bigman will refuse if the inventory changed after review. A baseline records prior state, not trust.".into(),Action::AcceptBaseline(reviewed.unwrap()));
                }
            }
            Page::Canaries => {
                ui.heading("Canary tripwires");
                ui.label("Synthetic decoy files alert when read, changed or removed while the monitor is running. They do not contain usable credentials. Backups, indexers and your tests can trigger them.");
                let health = self
                    .store
                    .read::<crate::canary::Health>("monitor-health.json")
                    .ok();
                let active = health
                    .as_ref()
                    .is_some_and(|h| crate::model::now().saturating_sub(h.updated) < 10);
                ui.strong(if active {
                    "Monitor is reporting"
                } else {
                    "Monitor is stopped or heartbeat is stale"
                });
                if let Some(h) = health {
                    ui.label(format!("{}/{} paths watched", h.watched, h.configured));
                    for e in h.errors {
                        ui.colored_label(egui::Color32::YELLOW, e);
                    }
                }
                if !active
                    && ui
                        .button("Start monitoring existing tokens at login")
                        .clicked()
                {
                    self.start(Action::StartMonitor);
                }
                if self.store.root.join("monitor-health.json").exists()
                    && ui.button("Stop monitoring…").clicked()
                {
                    self.confirm("Stop local canary and startup monitoring and remove its login service. Token files and history stay on disk. Reads while stopped cannot be detected.".into(), Action::StopMonitor);
                }
                ui.separator();
                ui.label("Local decoy location (existing unrelated files are never overwritten)");
                ui.text_edit_singleline(&mut self.decoy);
                ui.label("No account or API key needed. Creates a local token, enables monitoring at login when available, and verifies a real read event. Keep Bigman installed at its current path.");
                if ui.button("Create, monitor and test locally").clicked() {
                    self.start(Action::ArmCanary(self.decoy.clone()));
                }
                ui.collapsing("Advanced: existing hosted token", |ui| {
                    ui.label("Only needed for callbacks from another computer. Paste a provider-created HTTPS token URL and choose a new .url path above. This creates a shortcut without visiting the URL.");
                    ui.text_edit_singleline(&mut self.hosted);
                    if ui.add_enabled(!self.hosted.trim().is_empty(), egui::Button::new("Create hosted shortcut…")).clicked() {
                        self.confirm(format!("Create a shortcut at {} using the supplied token URL. Existing files are never overwritten.", self.decoy), Action::CreateCanary(self.decoy.clone(), Some(self.hosted.trim().into())));
                    }
                });
                match crate::canary::list(&self.store) {
                    Ok(cs) => {
                        for c in cs {
                            ui.group(|ui| {
                                ui.label(crate::model::clean(&c.path.to_string_lossy()));
                                ui.small(&c.id);
                                ui.horizontal(|ui| {
                                    if ui
                                        .add_enabled(active, egui::Button::new("Verify detection"))
                                        .clicked()
                                    {
                                        self.start(Action::TestCanary(c.id.clone()));
                                    }
                                    if ui.button("Disarm…").clicked() {
                                        self.confirm(
                                            format!(
                                                "Stop watching {}. The decoy file stays on disk.",
                                                c.path.display()
                                            ),
                                            Action::Disarm(c.id.clone()),
                                        );
                                    }
                                });
                            });
                        }
                    }
                    Err(e) => {
                        ui.label(format!("Cannot load canaries: {e}"));
                    }
                }
            }
            Page::Tools => {
                ui.heading("Tools and setup");
                ui.label("Install supported packages from your configured distribution repositories. Administrator actions use polkit authentication.");
                egui::ComboBox::from_label("Tool group")
                    .selected_text(&self.tool)
                    .show_ui(ui, |ui| {
                        for tool in [
                            "clamav",
                            "shellcheck",
                            "sandbox",
                            "lynis",
                            "pdf",
                            "firewall",
                        ] {
                            ui.selectable_value(&mut self.tool, tool.into(), tool);
                        }
                    });
                if ui.button("Preview installation…").clicked() {
                    match crate::setup::install_tool(&self.tool, false, &runner::cancel()) {
                        Ok(p) => self.confirm(p, Action::Install(self.tool.clone())),
                        Err(e) => self.message = e.to_string(),
                    }
                }
                ui.separator();
                if ui
                    .button("Enable monitoring and daily Downloads scan…")
                    .clicked()
                {
                    match crate::setup::services(&self.store, false, false, &runner::cancel()) {
                        Ok(p) => self.confirm(p, Action::Services(false)),
                        Err(e) => self.message = e.to_string(),
                    }
                }
                if ui.button("Disable and remove user services…").clicked() {
                    self.confirm("Stop and remove Bigman user monitoring and daily scan units. Existing report data and canary files remain.".into(),Action::Services(true));
                }
                ui.separator();
                ui.label("Optional deterministic tools must already be installed. OSV advisory scans use network services; other actions below run locally.");
                ui.text_edit_singleline(&mut self.project);
                ui.horizontal(|ui|{if ui.button("Lynis system audit").clicked(){self.start(Action::Audit("lynis".into(),".".into()));}
if ui.button("OSV dependency advisories…").clicked(){self.confirm(format!("Run OSV-Scanner on {}. It may query vulnerability services using dependency identifiers. No AI analysis is used.",self.project),Action::Audit("osv".into(),self.project.clone()));}
if ui.button("osquery process inventory").clicked(){self.start(Action::Audit("osquery".into(),".".into()));}
if ui.button("Open OpenSnitch").clicked(){if let Err(e)=crate::setup::open_tool("firewall"){self.message=e.to_string();}}});
                ui.collapsing("Upstream installation instructions", |ui| {
                    for (name, url) in [
                        (
                            "PDFiD / pdf-parser",
                            "https://blog.didierstevens.com/programs/pdf-tools/",
                        ),
                        ("OSV-Scanner", "https://google.github.io/osv-scanner/"),
                        ("osquery", "https://osquery.io/"),
                        ("YARA-X", "https://virustotal.github.io/yara-x/"),
                    ] {
                        ui.hyperlink_to(name, url);
                    }
                });
                ui.label("Reviewed YARA-X rules file");
                ui.text_edit_singleline(&mut self.rules);
                ui.label("Target");
                ui.text_edit_singleline(&mut self.path);
                if ui.button("Scan using these rules").clicked() {
                    self.start(Action::Yara(self.rules.clone(), self.path.clone()));
                }
            }
            Page::History => {
                ui.heading("History");
                ui.label("Select a report to inspect findings, limits and evidence. Reports stay on this computer unless you export them.");
                for r in self.history.clone() {
                    if ui
                        .selectable_label(
                            self.report.as_ref().is_some_and(|p| p.id == r.id),
                            format!(
                                "{} | {} | {}",
                                r.kind,
                                r.status(),
                                crate::model::clean(&r.subject)
                            ),
                        )
                        .clicked()
                    {
                        match self.store.report(&r.id) {
                            Ok(full) => self.report = Some(Arc::new(full)),
                            Err(e) => self.message = e.to_string(),
                        }
                    }
                }
            }
            Page::Quarantine => {
                ui.heading("Quarantine");
                ui.label("Recovery copies are private and non-executable. Restore refuses to overwrite existing paths.");
                match crate::scanner::quarantined(&self.store) {
                    Ok(qs) => {
                        for q in qs {
                            ui.group(|ui|{ui.label(crate::model::clean(&q.original.to_string_lossy()));ui.monospace(&q.hash);if q.restored{ui.label("Restored; recovery copy retained");}else if ui.button("Restore…").clicked(){self.confirm(format!("Restore {} with permissions 0600. This file was previously flagged. Existing files will not be overwritten.",q.original.display()),Action::Restore(q.id));}});
                        }
                    }
                    Err(e) => {
                        ui.label(e.to_string());
                    }
                }
            }
        }
    }
}
impl Drop for App {
    fn drop(&mut self) {
        if let Some(j) = &self.job {
            j.stop.store(true, Ordering::Relaxed);
        }
    }
}
pub fn run_gui(store: Store) -> Result<(), eframe::Error> {
    eframe::run_native(
        "Bigman Security Companion",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1100., 780.]),
            ..Default::default()
        },
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(App::new(store)))
        }),
    )
}
