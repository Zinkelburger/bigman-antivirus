use anyhow::{bail, Result};
use bigman_core::{
    checks::{
        canary, changes, doctor, hardening, inspect, integrations, packages, pdf, route, scan,
        setup, shell,
    },
    ipc::{self, client::Client},
    model::{self, Report},
    platform, runner, store,
};
use clap::{Parser, Subcommand};
use std::{
    io::{IsTerminal, Read},
    path::PathBuf,
    sync::atomic::Ordering,
};
use store::Store;
#[derive(Parser)]
#[command(
    version,
    about = "Deterministic Linux security companion. No AI, no automatic malware verdicts."
)]
struct Cli {
    #[arg(long, global = true)]
    state: Option<PathBuf>,
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// Open the desktop app.
    Gui,
    Doctor,
    /// Settings you could change to harden this computer. With an id, shows
    /// exactly what would change; nothing changes without --apply.
    Harden {
        /// A suggestion id from the list.
        id: Option<String>,
        /// Make the change (polkit asks for an administrator password when needed).
        #[arg(long, requires = "id", conflicts_with_all = ["undo", "keep", "suggest_again"])]
        apply: bool,
        /// Put back a change Bigman made.
        #[arg(long, requires = "id", conflicts_with_all = ["keep", "suggest_again"])]
        undo: bool,
        /// Stop suggesting this change; nothing on the computer changes.
        #[arg(long, requires = "id", conflicts_with = "suggest_again")]
        keep: bool,
        /// Suggest a kept change again.
        #[arg(long, requires = "id")]
        suggest_again: bool,
    },
    Inspect {
        target: String,
    },
    /// Inspect a file, folder or HTTPS URL with the inspector that fits it.
    Check {
        target: String,
    },
    Packages {
        path: PathBuf,
    },
    PackagePolicy {
        path: PathBuf,
        #[arg(long)]
        apply: bool,
    },
    Scan {
        path: Option<PathBuf>,
        #[arg(long)]
        notify: bool,
    },
    Pdf {
        path: PathBuf,
    },
    Update,
    Sandbox {
        report: String,
        #[arg(long)]
        approve_sha256: String,
    },
    Changes {
        #[arg(long)]
        accept_baseline: Option<String>,
    },
    History {
        report: Option<String>,
    },
    Canary {
        #[command(subcommand)]
        command: CanaryCommands,
    },
    Watch,
    Quarantine {
        report: String,
        index: usize,
    },
    Restore {
        id: String,
    },
    Quarantined,
    Audit {
        tool: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    Yara {
        rules: PathBuf,
        target: PathBuf,
    },
    Setup {
        tool: String,
        #[arg(long)]
        apply: bool,
    },
    Services {
        #[arg(long)]
        apply: bool,
        #[arg(long)]
        remove: bool,
    },
    NativeHost,
    NativeInstall {
        extension_id: String,
    },
    /// Query the bigmand collector daemon.
    Daemon {
        #[command(subcommand)]
        command: DaemonCommands,
    },
}
#[derive(Subcommand)]
enum DaemonCommands {
    /// Show whether bigmand answers on its socket.
    Status,
    /// Timeline entries, newest first.
    Events {
        /// Only the last N hours.
        #[arg(long, default_value_t = 24)]
        hours: u64,
        /// Only these sources (network, listener, receipt, denial, scan, canary, ...).
        #[arg(long = "source")]
        sources: Vec<String>,
        #[arg(long, default_value_t = 50)]
        limit: u32,
    },
    /// Print entries as the daemon records them, until interrupted.
    Follow,
    /// Current connections of your programs, and every listening program.
    Network,
    /// What each live source can and cannot read.
    Sources,
    /// One install receipt: what a package transaction installed and ran.
    Receipt { id: String },
}
#[derive(Subcommand)]
enum CanaryCommands {
    /// Generate a local decoy, start monitoring and verify an actual read event.
    Arm {
        path: Option<PathBuf>,
        /// Skip login service setup; monitor until logout/reboot or an explicit stop.
        #[arg(long)]
        session_only: bool,
    },
    /// Start monitoring existing tokens without creating another file.
    Start {
        #[arg(long)]
        session_only: bool,
    },
    /// Stop the monitor and remove its login service, preserving decoys and history.
    Stop {
        #[arg(long)]
        session_only: bool,
    },
    Create {
        path: PathBuf,
        #[arg(long)]
        hosted_url: Option<String>,
    },
    List,
    Test {
        id: String,
    },
    Disarm {
        id: String,
    },
}
fn output(r: &Report, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(r)?);
    } else {
        println!("{}", r.text());
    }
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("Bigman: {}", model::clean(&e.to_string()));
        std::process::exit(2);
    }
}
fn run() -> Result<()> {
    // Chromium supplies its validated origin as argv[1], not a CLI subcommand.
    let native = std::env::args()
        .nth(1)
        .is_some_and(|a| a.starts_with("chrome-extension://"));
    let cli = if native {
        Cli {
            state: None,
            json: false,
            command: Some(Commands::NativeHost),
        }
    } else {
        Cli::parse()
    };
    let store = Store::open(cli.state)?;
    let stop = runner::cancel();
    let signal = stop.clone();
    ctrlc::set_handler(move || {
        signal.store(true, Ordering::Relaxed);
    })?;
    let report = match cli.command {
        Some(Commands::Gui) => {
            open_app()?;
            None
        }
        None if std::io::stdin().is_terminal() => {
            open_app()?;
            None
        }
        None => {
            let mut bytes = vec![];
            std::io::stdin()
                .take(store::MAX_INPUT + 1)
                .read_to_end(&mut bytes)?;
            if bytes.len() as u64 > store::MAX_INPUT {
                bail!("Input limit exceeded");
            }
            let text = std::str::from_utf8(&bytes)?;
            let r = shell::analyze_content(text);
            store.bytes(&format!("artifacts/{}.sh", r.id), &bytes)?;
            store.save(&r)?;
            Some(r)
        }
        Some(Commands::Doctor) => Some(doctor::doctor(&store, &stop)?),
        Some(Commands::Harden {
            id,
            apply,
            undo,
            keep,
            suggest_again,
        }) => {
            let action = id.map(|id| {
                if undo {
                    bigman_core::actions::Action::UndoHardening { id }
                } else if keep || suggest_again {
                    bigman_core::actions::Action::KeepSetting { id, keep }
                } else {
                    bigman_core::actions::Action::Harden { id }
                }
            });
            match action {
                None => {
                    let review = hardening::review(&store, &stop)?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&review)?);
                    } else {
                        print!("{}", harden_text(&review));
                    }
                    exit_for(&review.report);
                    None
                }
                Some(action) if apply || undo || keep || suggest_again => {
                    let done = bigman_core::actions::apply(&action, &store, &stop)?;
                    println!("{}", done.message);
                    None
                }
                Some(action) => {
                    let p = bigman_core::actions::preview(&action, &store)?;
                    println!(
                        "{}\n\n{}\n\nExactly what changes:\n{}",
                        p.title, p.summary, p.details
                    );
                    if let Some(u) = p.undo {
                        println!("\nTo undo: {u}");
                    }
                    println!("\nNothing has changed. Run again with --apply to make this change.");
                    None
                }
            }
        }
        Some(Commands::Inspect { target }) => Some(inspect::inspect(&store, &target, &stop)?),
        Some(Commands::Check { target }) => Some(route::inspect_any(&store, &target, &stop)?),
        Some(Commands::Packages { path }) => Some(packages::packages(&store, &path)?),
        Some(Commands::PackagePolicy { path, apply }) => {
            println!("{}", packages::policy(&path, apply)?);
            None
        }
        Some(Commands::Scan { path, notify }) => {
            let result = scan::scan(&store, &path.unwrap_or_else(setup::downloads), &stop);
            if notify {
                match &result {
                    Ok(r) if !r.findings.is_empty() || !r.errors.is_empty() => {
                        canary::notify("Bigman scan needs attention", r.status())
                    }
                    Err(e) => canary::notify("Bigman scan failed", &e.to_string()),
                    _ => {}
                }
            }
            Some(result?)
        }
        Some(Commands::Pdf { path }) => Some(pdf::scan_pdf(&store, &path, &stop)?),
        Some(Commands::Update) => Some(scan::update(&store, &stop)?),
        Some(Commands::Sandbox {
            report,
            approve_sha256,
        }) => Some(inspect::sandbox(&store, &report, &approve_sha256, &stop)?),
        Some(Commands::Changes { accept_baseline }) => {
            let r = match accept_baseline {
                Some(id) => changes::accept_reviewed_baseline(&store, &id)?,
                None => changes::changes(&store, &runner::cancel())?,
            };
            store.save(&r)?;
            Some(r)
        }
        Some(Commands::History { report: Some(id) }) => Some(store.report(&id)?),
        Some(Commands::History { report: None }) => {
            let rs = store.history()?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&rs)?);
            } else {
                for r in rs {
                    println!(
                        "{} | {} | {} | {}",
                        r.id,
                        r.kind,
                        r.status(),
                        model::clean(&r.subject)
                    );
                }
            }
            None
        }
        Some(Commands::Canary { command }) => match command {
            CanaryCommands::Arm { path, session_only } => Some(canary::setup::arm(
                &store,
                &path.unwrap_or_else(canary::setup::default_path),
                session_only,
                &stop,
            )?),
            CanaryCommands::Start { session_only } => {
                Some(canary::setup::start_monitor(&store, session_only, &stop)?)
            }
            CanaryCommands::Stop { session_only } => {
                Some(canary::setup::stop_monitor(&store, session_only, &stop)?)
            }
            CanaryCommands::Test { id } => Some(canary::setup::verify(&store, &id, &stop)?),
            CanaryCommands::Create { path, hosted_url } => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&canary::create(
                        &store,
                        &path,
                        hosted_url.as_deref()
                    )?)?
                );
                None
            }
            CanaryCommands::List => {
                println!("{}", serde_json::to_string_pretty(&canary::list(&store)?)?);
                None
            }
            CanaryCommands::Disarm { id } => {
                canary::disarm(&store, &id)?;
                println!("Disarmed. Decoy file preserved.");
                None
            }
        },
        Some(Commands::Watch) => {
            canary::watch(&store, &stop)?;
            None
        }
        Some(Commands::Quarantine { report, index }) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&scan::quarantine(&store, &report, index)?)?
            );
            None
        }
        Some(Commands::Restore { id }) => {
            println!(
                "Restored without executable permissions: {}",
                scan::restore(&store, &id)?.display()
            );
            None
        }
        Some(Commands::Quarantined) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&scan::quarantined(&store)?)?
            );
            None
        }
        Some(Commands::Audit { tool, path }) => {
            Some(integrations::integration(&store, &tool, &path, &stop)?)
        }
        Some(Commands::Yara { rules, target }) => {
            Some(integrations::yara(&store, &rules, &target, &stop)?)
        }
        Some(Commands::Setup { tool, apply }) => {
            println!("{}", setup::install_tool(&tool, apply, &stop)?);
            None
        }
        Some(Commands::Services { apply, remove }) => {
            println!("{}", setup::services(&store, apply, remove, &stop)?);
            None
        }
        Some(Commands::NativeHost) => {
            bigman_native_host::host(&store, &stop)?;
            None
        }
        Some(Commands::Daemon {
            command: DaemonCommands::Status,
        }) => {
            daemon_status(cli.json)?;
            None
        }
        Some(Commands::Daemon { command }) => {
            daemon_live(command, cli.json)?;
            None
        }
        Some(Commands::NativeInstall { extension_id }) => {
            for p in setup::native_manifest_paths() {
                setup::write_native_manifest(&p, &extension_id)?;
                println!("Registered {}", p.display());
            }
            None
        }
    };
    if let Some(r) = report {
        output(&r, cli.json)?;
        exit_for(&r);
    }
    Ok(())
}
/// Exit 2 when incomplete, 1 when something is worth reviewing, otherwise return.
fn exit_for(r: &Report) {
    let code = if r.is_incomplete() {
        2
    } else if r
        .findings
        .iter()
        .any(|f| f.severity != model::Severity::Info)
    {
        1
    } else {
        0
    };
    if code != 0 {
        std::process::exit(code);
    }
}
/// The hardening review as text: suggestions first, then what is in place.
fn harden_text(review: &hardening::HardeningReview) -> String {
    let status = match review.report.status() {
        "Findings to review" => "Suggestions to review",
        "No findings from these checks" => "No suggestions from these checks",
        other => other,
    };
    let mut out = format!("Hardening review\n{status}\n");
    let indent = |text: &str| {
        text.lines()
            .map(|l| format!("  {}", model::clean(l)))
            .collect::<Vec<_>>()
            .join("\n")
    };
    for s in review.suggestions.iter().filter(|s| !s.kept) {
        out.push_str(&format!(
            "\n[{:?}] {} ({})\n{}\n",
            s.severity, s.title, s.id, s.summary
        ));
        if !s.evidence.is_empty() {
            out.push_str(&format!("{}\n", indent(&s.evidence)));
        }
        out.push_str(&format!("Why: {}\nCost: {}\n", s.why, s.tradeoff));
        out.push_str(&match &s.fix {
            hardening::Fix::Bigman { .. } => {
                format!("Next: bigman harden {} (shows the change)\n", s.id)
            }
            hardening::Fix::Manual { steps } => format!("Next, by hand: {steps}\n"),
            hardening::Fix::Blocked { reason } => format!("Not now: {reason}\n"),
        });
    }
    let kept: Vec<_> = review.suggestions.iter().filter(|s| s.kept).collect();
    if !kept.is_empty() {
        out.push_str("\nLeft as they are (bigman harden ID --suggest-again):\n");
        for s in kept {
            out.push_str(&format!("  {} ({})\n", s.title, s.id));
        }
    }
    if !review.applied.is_empty() {
        out.push_str("\nChanged by Bigman (bigman harden ID --undo):\n");
        for a in &review.applied {
            out.push_str(&format!("  {} ({})\n", a.title, a.id));
        }
    }
    if !review.in_place.is_empty() {
        out.push_str("\nAlready set:\n");
        for c in &review.in_place {
            out.push_str(&format!("  {}: {}\n", c.check, c.detail));
        }
    }
    for s in &review.report.coverage.skipped {
        out.push_str(&format!(
            "\nNot checked: {} ({:?}) {}",
            s.what,
            s.why,
            model::clean(&s.detail)
        ));
    }
    for n in &review.report.coverage.notes {
        out.push_str(&format!("\nCoverage: {n}"));
    }
    out.push('\n');
    out
}
/// The desktop app is a separate binary installed next to this one.
fn open_app() -> Result<()> {
    let app = runner::sibling_executable("bigman-app").map_err(|_| {
        anyhow::anyhow!(
            "The Bigman desktop app is not installed next to this binary. Install it with ./install.sh, or use the CLI commands (bigman --help)."
        )
    })?;
    runner::spawn_detached(&mut std::process::Command::new(app))
}
fn daemon_status(json: bool) -> Result<()> {
    let mut client = Client::connect()?;
    client.set_timeout(Some(std::time::Duration::from_secs(5)))?;
    match client.request(ipc::Request::Health)? {
        ipc::Response::Health(h) if json => println!("{}", serde_json::to_string_pretty(&h)?),
        ipc::Response::Health(h) => println!(
            "bigmand {} is running (pid {}) at {}\nState: {}",
            h.version,
            h.pid,
            ipc::client::describe_socket(),
            h.state_dir.display()
        ),
        ipc::Response::Error(e) => bail!("Daemon error: {}", e.message),
        other => bail!("Unexpected daemon response: {other:?}"),
    }
    Ok(())
}

fn daemon_request(request: ipc::Request) -> Result<ipc::Response> {
    let mut client = Client::connect()?;
    client.set_timeout(Some(std::time::Duration::from_secs(30)))?;
    match client.request(request)? {
        ipc::Response::Error(e) => bail!("Daemon error: {}", e.message),
        response => Ok(response),
    }
}

fn print_event(e: &model::Event) {
    let repeat = if e.seen_before == 0 {
        "first seen".to_string()
    } else {
        format!("seen {} times", e.seen_before + 1)
    };
    let severity = match e.severity {
        model::Severity::Info => String::new(),
        s => format!(" [{s:?}]"),
    };
    println!(
        "{}  {:<9} {}{severity} ({repeat})",
        e.at,
        e.source.as_str(),
        e.summary
    );
}

/// The timeline and live collection, read through the daemon that owns them.
fn daemon_live(command: DaemonCommands, json: bool) -> Result<()> {
    match command {
        DaemonCommands::Status => daemon_status(json),
        DaemonCommands::Events {
            hours,
            sources,
            limit,
        } => {
            let sources = sources
                .iter()
                .map(|s| {
                    model::SourceId::parse(s).ok_or_else(|| anyhow::anyhow!("Unknown source {s}"))
                })
                .collect::<Result<Vec<_>>>()?;
            let since = model::now().saturating_sub(hours * 3600);
            let ipc::Response::Events(events) = daemon_request(ipc::Request::Events {
                since,
                until: None,
                sources,
                limit,
            })?
            else {
                bail!("Unexpected daemon response");
            };
            if json {
                println!("{}", serde_json::to_string_pretty(&events)?);
            } else if events.is_empty() {
                println!("No entries in the last {hours} hours.");
            } else {
                events.iter().for_each(print_event);
            }
            Ok(())
        }
        DaemonCommands::Follow => {
            let mut client = Client::connect()?;
            client.subscribe(true, false, |live| {
                if let ipc::Live::Event(e) = live {
                    if json {
                        println!("{}", serde_json::to_string(&e).unwrap_or_default());
                    } else {
                        print_event(&e);
                    }
                }
                true
            })
        }
        DaemonCommands::Network => {
            let ipc::Response::Network(net) = daemon_request(ipc::Request::Network)? else {
                bail!("Unexpected daemon response");
            };
            if json {
                println!("{}", serde_json::to_string_pretty(&net)?);
                return Ok(());
            }
            let name = |s: &platform::Socket| {
                s.process
                    .as_ref()
                    .map_or("(another user)".to_string(), |p| p.name.clone())
            };
            println!("Listening programs");
            for l in &net.listeners {
                let worst = l.findings.iter().map(|f| f.severity).max();
                println!(
                    "  {:<22} {:<16} {:<24} {}",
                    l.socket.local.to_string(),
                    name(&l.socket),
                    l.reach,
                    worst.map_or(String::new(), |s| format!(
                        "[{s:?}] {}",
                        l.findings[0].summary
                    ))
                );
            }
            println!("Your programs' connections");
            for c in &net.connections {
                println!(
                    "  {:<16} {}",
                    name(&c.socket),
                    c.socket.remote.map_or(String::new(), |r| r.to_string())
                );
            }
            for note in &net.coverage.notes {
                println!("{note}");
            }
            Ok(())
        }
        DaemonCommands::Receipt { id } => {
            let ipc::Response::Receipt(r) = daemon_request(ipc::Request::Receipt { id })? else {
                bail!("Unexpected daemon response");
            };
            if json {
                println!("{}", serde_json::to_string_pretty(&r)?);
                return Ok(());
            }
            println!(
                "{} transaction {} · {}",
                r.manager, r.transaction, r.command
            );
            for p in &r.packages {
                println!("  {:?} {} {}", p.kind, p.name, p.version);
            }
            println!(
                "{} files, {} install scripts, {} services or autostart entries, {} suspicious lines",
                r.files.len(),
                r.scripts.len(),
                r.services.len(),
                r.hits.len()
            );
            for f in &r.findings {
                println!(
                    "[{:?}] {} ({})
  {}",
                    f.severity,
                    f.summary,
                    f.rule,
                    f.evidence.replace('\n', "\n  ")
                );
            }
            if !r.after.checked {
                println!("What started afterwards is checked a minute after the transaction.");
            }
            Ok(())
        }
        DaemonCommands::Sources => {
            let ipc::Response::Sources(sources) = daemon_request(ipc::Request::Sources)? else {
                bail!("Unexpected daemon response");
            };
            if json {
                println!("{}", serde_json::to_string_pretty(&sources)?);
            } else {
                for s in &sources {
                    let state = if s.ok {
                        "reading".to_string()
                    } else {
                        format!("not collected: {}", s.detail)
                    };
                    println!("{:<34} {state}", s.label);
                }
            }
            Ok(())
        }
    }
}
