use anyhow::{bail, Context, Result};
use bigman_core::{
    checks::{
        self, canary, changes, disposable, doctor, hardening, inspect, integrations, packages, pdf,
        route, scan, setup, shell,
    },
    ipc::{self, client::Client},
    model::{self, Report},
    platform, runner, store,
};
use clap::{Parser, Subcommand};
use std::{
    io::{IsTerminal, Read, Write},
    path::{Path, PathBuf},
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
    /// Installed skills and plugins for Claude Code, OpenCode and Codex: what each
    /// runs without asking, what it pre-approves, and whether you approved it.
    Skills {
        /// Approve the version a report listed: the report ID and the item's
        /// number (from the list) or key.
        #[arg(long, num_args = 2, value_names = ["REPORT_ID", "ITEM"])]
        approve: Option<Vec<String>>,
        /// Take back an approval by the item's key, restoring the one it replaced.
        #[arg(long, value_name = "KEY")]
        revoke: Option<String>,
    },
    /// The running system looked at more than one way: processes, kernel modules
    /// and sockets that one view shows and another does not, programs running from
    /// memory or deleted files, packet access, BPF programs, and who watches your
    /// credential folders.
    Running {
        /// Mark a row as known, pinned to what it is now: the report ID and the
        /// row's number (from the list) or key.
        #[arg(long, num_args = 2, value_names = ["REPORT_ID", "ROW"])]
        known: Option<Vec<String>>,
        /// Take a mark back by the row's key, restoring the one it replaced.
        #[arg(long, value_name = "KEY")]
        forget: Option<String>,
        /// List every row with its number, key and pinned details.
        #[arg(long)]
        rows: bool,
    },
    /// Stop one of your programs and watch for a minute for anything that starts
    /// it again. Without a process ID, lists your programs. Bigman never stops a
    /// program unless you ask, and shows what it will do first.
    StopWatch {
        /// The process ID from the list.
        pid: Option<u32>,
        /// Stop it and watch (asks it to stop with SIGTERM; never forces it).
        #[arg(long)]
        apply: bool,
        /// Start a program stopped earlier again: the Stop and watch report's ID.
        #[arg(long, value_name = "REPORT_ID", conflicts_with = "pid")]
        again: Option<String>,
    },
    /// Open a shell in a disposable Podman container that holds a copy of a folder
    /// or file and nothing else from this computer. The container and the copy are
    /// deleted when the shell exits.
    Try {
        path: PathBuf,
        /// No network inside the container.
        #[arg(long)]
        offline: bool,
        /// A port inside the container to reach at 127.0.0.1 on this computer.
        #[arg(long = "port")]
        ports: Vec<u16>,
        /// The image's full name, instead of the one picked from the project files.
        #[arg(long)]
        image: Option<String>,
        /// Root inside the container, to install system packages. An escape would
        /// then act as your account.
        #[arg(long)]
        root: bool,
        /// Start without asking.
        #[arg(long, short)]
        yes: bool,
        /// Wait for Enter before exiting, so a window the app opened stays readable.
        #[arg(long, hide = true)]
        pause: bool,
        /// What to run instead of a shell, after --.
        #[arg(last = true)]
        command: Vec<String>,
    },
    /// What a project folder could run on open (IDE tasks, direnv, devcontainers,
    /// git hooks and filters, AI-tool settings, instruction files, CI workflows) and
    /// whether its own code runs fetched or decoded text. Give a GitHub, GitLab,
    /// Codeberg or Bitbucket repository URL to read it before cloning.
    RepoCheck {
        /// A project folder, or a repository URL such as https://github.com/OWNER/REPO.
        path: String,
    },
    /// Non-registry sources, missing integrity hashes and install hooks in a
    /// lockfile (npm, Yarn, pnpm, Bun, Cargo).
    Lockfile {
        path: PathBuf,
        /// An older copy: report only what the change adds.
        #[arg(long)]
        against: Option<PathBuf>,
    },
    /// Where packages come from: apt, dnf, zypper, pacman, Flatpak and Nix sources,
    /// judged on signatures, HTTPS, key scope and priority over the distribution.
    PackageSources,
    /// Pending security updates for installed packages, from your distribution's
    /// own security data (Fedora, Debian, Arch). Shows the update command; never runs it.
    Advisories {
        /// Download the security data again even if it is less than a day old.
        #[arg(long)]
        refresh: bool,
        /// Use cached data only; report incomplete if it is too old.
        #[arg(long)]
        offline: bool,
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
    /// Called by the opt-in package-manager hooks (as root): preview a transaction's
    /// receipt in Bigman before it runs. Every failure lets the transaction continue.
    #[command(hide = true)]
    Receipt {
        /// dnf only: record one incoming package (full NEVRA, repo id, location, action).
        #[arg(long, num_args = 4, value_names = ["NEVRA", "REPO", "LOCATION", "ACTION"])]
        stage: Option<Vec<String>>,
        /// Build the preview and wait for Continue or Abort.
        #[arg(long)]
        preview: bool,
        /// dnf, apt or pacman.
        #[arg(long)]
        manager: Option<String>,
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
    /// Stop bigmand: the Downloads and project watches, Activity and Network
    /// collection and the daily updates check, until it is started again.
    Stop,
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
        #[command(flatten)]
        responses: ResponseArgs,
    },
    /// Replace what happens when a decoy is touched (all flags off: log and notify only).
    Respond {
        id: String,
        #[command(flatten)]
        responses: ResponseArgs,
    },
    /// A decoy's access log, newest first: when, what, which process, what ran.
    Log {
        id: String,
    },
    /// Show (or with --apply, make) the change that installs the optional root
    /// helper naming the process behind each access.
    Helper {
        #[arg(long)]
        apply: bool,
        #[arg(long)]
        remove: bool,
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
/// "2026-11-10 14:03:22" in local time.
fn local_time(seconds: u64) -> String {
    let t = seconds as libc::time_t;
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    if unsafe { libc::localtime_r(&t, &mut tm) }.is_null() {
        return seconds.to_string();
    }
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        tm.tm_year + 1900,
        tm.tm_mon + 1,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec
    )
}
/// What happens when a decoy is touched, beyond logging and a notification.
#[derive(clap::Args)]
struct ResponseArgs {
    /// Lock every session of yours.
    #[arg(long)]
    lock_screen: bool,
    /// End every session and program of yours, the monitor included (needs the helper).
    #[arg(long)]
    log_out: bool,
    /// Turn NetworkManager networking off.
    #[arg(long)]
    network_off: bool,
    /// Kill the process that touched the decoy (needs the helper).
    #[arg(long)]
    kill_reader: bool,
    /// Lock your local account when your account touched it (needs the helper).
    #[arg(long)]
    lock_account: bool,
    /// Run this executable with the event in BIGMAN_* environment variables.
    #[arg(long)]
    script: Option<PathBuf>,
}
impl From<ResponseArgs> for canary::Responses {
    fn from(a: ResponseArgs) -> Self {
        Self {
            lock_screen: a.lock_screen,
            log_out: a.log_out,
            network_off: a.network_off,
            kill_reader: a.kill_reader,
            lock_account: a.lock_account,
            script: a.script,
        }
    }
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
/// `bigman stop-watch` with no process ID: your programs, and why any is refused.
fn programs_text(list: &[bigman_core::checks::respawn::Program]) -> String {
    let mut out = String::from("Your programs. Stop one and watch with: bigman stop-watch PID\n\n");
    for p in list {
        let unit = p
            .unit
            .as_deref()
            .map(|u| format!("  [{u}]"))
            .unwrap_or_default();
        out.push_str(&format!(
            "{:>8}  {}{unit}\n          {}\n",
            p.pid, p.name, p.command
        ));
        if let Some(why) = &p.refused {
            out.push_str(&format!("          Not offered: {why}\n"));
        }
    }
    out
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
        Some(Commands::Try {
            path,
            offline,
            ports,
            image,
            root,
            yes,
            pause,
            command,
        }) => {
            let options = disposable::Options {
                offline,
                ports,
                image,
                root,
                command,
            };
            let result = try_container(&store, &path, options, yes, &stop);
            if pause {
                let mut err = std::io::stderr();
                if let Err(e) = &result {
                    let _ = writeln!(err, "Bigman: {}", model::clean(&e.to_string()));
                }
                let _ = writeln!(err, "\nPress Enter to close this window.");
                let _ = std::io::stdin().read_line(&mut String::new());
                if result.is_err() {
                    std::process::exit(2);
                }
            }
            match result? {
                Some(0) | None => None,
                Some(code) => std::process::exit(code),
            }
        }
        Some(Commands::Lockfile { path, against }) => {
            let r = checks::lockfile::lockfile(&path, against.as_deref())?;
            store.save(&r)?;
            Some(r)
        }
        Some(Commands::PackageSources) => {
            let r = checks::channels::channels();
            store.save(&r)?;
            Some(r)
        }
        Some(Commands::RepoCheck { path }) if checks::gitremote::repo_url(&path).is_some() => {
            Some(checks::gitremote::check(&store, &path, &stop)?)
        }
        Some(Commands::Skills { approve, revoke }) => match (approve, revoke) {
            (Some(args), _) => {
                let key = checks::skills::key_for(&store, &args[0], &args[1])?;
                println!("{}", checks::skills::approve(&store, &args[0], &key)?);
                None
            }
            (None, Some(key)) => {
                println!("{}", checks::skills::revoke(&store, &key)?);
                None
            }
            (None, None) => Some(checks::skills::inventory(&store)?),
        },
        Some(Commands::Running {
            known,
            forget,
            rows,
        }) => match (known, forget) {
            (Some(args), _) => {
                let key = checks::running::key_for(&store, &args[0], &args[1])?;
                println!("{}", checks::running::mark(&store, &args[0], &key, &stop)?);
                None
            }
            (None, Some(key)) => {
                println!("{}", checks::running::forget(&store, &key)?);
                None
            }
            (None, None) => {
                let (r, items) = checks::running::check(&store, &stop)?;
                if rows && !cli.json {
                    print!("{}", checks::running::rows_text(&items));
                }
                Some(r)
            }
        },
        Some(Commands::StopWatch { pid, apply, again }) => {
            use bigman_core::{actions::Action, checks::respawn};
            let action = match (pid, again) {
                (Some(pid), _) => {
                    let start = respawn::programs()
                        .into_iter()
                        .find(|p| p.pid == pid)
                        .map(|p| p.start)
                        .with_context(|| format!("No program of yours has process ID {pid}"))?;
                    Some(Action::StopAndWatch { pid, start })
                }
                (None, Some(report)) => Some(Action::StartAgain { report }),
                (None, None) => None,
            };
            match action {
                None => {
                    let list = respawn::programs();
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&list)?);
                    } else {
                        print!("{}", programs_text(&list));
                    }
                    None
                }
                Some(action) if apply => {
                    if matches!(action, Action::StopAndWatch { .. }) && !cli.json {
                        eprintln!(
                            "Watching for {} seconds; Ctrl+C ends the watch early.",
                            respawn::WATCH_SECONDS
                        );
                    }
                    // The CLI watches in the foreground; the daemon's watch runs
                    // in a thread that would end with this process.
                    if let Action::StopAndWatch { pid, start } = action {
                        let (report, watch) = respawn::stop_and_watch(&store, pid, start, &stop)?;
                        eprintln!("{}", respawn::outcome(&watch));
                        if respawn::can_start_again(&watch) {
                            eprintln!(
                                "To start it again: bigman stop-watch --again {} --apply",
                                report.id
                            );
                        }
                        Some(report)
                    } else {
                        let done = bigman_core::actions::apply(&action, &store, &stop)?;
                        eprintln!("{}", done.message);
                        None
                    }
                }
                Some(action) => {
                    let p = bigman_core::actions::preview(&action, &store)?;
                    println!("{}\n\n{}", p.title, p.summary);
                    if let Some(n) = &p.note {
                        println!("{n}");
                    }
                    if !p.facts.is_empty() {
                        println!();
                    }
                    for f in &p.facts {
                        println!("{}: {}", f.label, f.value);
                    }
                    println!(
                        "\n{}:\n{}",
                        p.details_label.as_deref().unwrap_or("Exactly what changes"),
                        p.details
                    );
                    println!("\nNothing has happened yet. Run again with --apply to do this.");
                    None
                }
            }
        }
        Some(Commands::RepoCheck { path }) => {
            let r = checks::repo::repo_check(Path::new(&path))?;
            store.save(&r)?;
            Some(r)
        }
        Some(Commands::Advisories { refresh, offline }) => Some(checks::advisories::advisories(
            &store,
            checks::advisories::Options { refresh, offline },
            &runner::cancel(),
        )?),
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
            CanaryCommands::Arm {
                path,
                session_only,
                responses,
            } => Some(canary::setup::arm(
                &store,
                &path.unwrap_or_else(canary::setup::default_path),
                session_only,
                responses.into(),
                &stop,
            )?),
            CanaryCommands::Respond { id, responses } => {
                let c = canary::set_responses(&store, &id, responses.into())?;
                let chosen = c.responses.describe();
                println!(
                    "Saved. When {} is touched: log and notify{}",
                    c.path.display(),
                    chosen
                        .iter()
                        .map(|r| format!(", then {}", r.to_lowercase()))
                        .collect::<String>()
                );
                None
            }
            CanaryCommands::Log { id } => {
                let log = canary::respond::log(&store, &id)?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&log)?);
                } else if log.is_empty() {
                    println!("No accesses recorded.");
                } else {
                    for a in &log {
                        println!("{}  {}", local_time(a.time), a.kinds.join(", "));
                        for r in &a.readers {
                            println!("  by {}: {}", r.label(), r.cmdline);
                        }
                        if a.self_test {
                            println!("  Bigman's own test read");
                        }
                        if let Some(why) = &a.skipped {
                            println!("  responses skipped: {why}");
                        }
                        for o in &a.outcomes {
                            println!(
                                "  {} {}: {}",
                                if o.ok { "ok  " } else { "FAIL" },
                                o.action,
                                o.detail.replace('\n', "\n       ")
                            );
                        }
                    }
                }
                None
            }
            CanaryCommands::Helper { apply, remove } => {
                let action = bigman_core::actions::Action::CanaryHelper { remove };
                if apply {
                    println!(
                        "{}",
                        bigman_core::actions::apply(&action, &store, &stop)?.message
                    );
                } else {
                    let p = bigman_core::actions::preview(&action, &store)?;
                    println!("{}\n\n{}\n", p.title, p.summary);
                    for f in &p.facts {
                        println!("{}: {}", f.label, f.value);
                    }
                    println!("\nThe commands run as root:\n{}", p.details);
                    println!("\nNothing has changed. Run again with --apply to make this change.");
                }
                None
            }
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
        Some(Commands::Receipt {
            stage,
            preview,
            manager,
        }) => {
            if let Some(stage) = stage {
                preview_stage(&stage)?;
            } else if preview {
                let manager = manager
                    .as_deref()
                    .and_then(bigman_core::checks::preview::Manager::parse)
                    .ok_or_else(|| anyhow::anyhow!("--manager must be dnf, apt or pacman"))?;
                std::process::exit(receipt_preview(manager));
            }
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
/// Shows the plan, asks unless `yes`, then runs the container in this terminal.
/// Returns the exit code of the shell or command, or None when nothing started.
fn try_container(
    store: &Store,
    path: &std::path::Path,
    options: disposable::Options,
    yes: bool,
    stop: &runner::Cancel,
) -> Result<Option<i32>> {
    let plan = disposable::plan(path, options)?;
    println!(
        "Open in a disposable container\n\n{}\n",
        plan.text(std::io::stdin().is_terminal())
    );
    if !yes {
        if !std::io::stdin().is_terminal() {
            bail!("Add --yes to start without a terminal to answer in");
        }
        eprint!("Open the container? [y/N] ");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer)?;
        if !answer.trim().eq_ignore_ascii_case("y") {
            println!("Nothing was started.");
            return Ok(None);
        }
    }
    let code = disposable::run(store, &plan, stop)?;
    // The window may be gone already; printing must not fail then.
    let _ = writeln!(
        std::io::stdout(),
        "\nThe container and its copy of {} are deleted, with anything downloaded into it. The image {} stays for next time (podman rmi {} removes it).",
        plan.target.display(),
        plan.image,
        plan.image
    );
    Ok(code)
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

fn daemon_stop() -> Result<()> {
    let Ok(mut client) = Client::connect() else {
        println!("bigmand is not running.");
        return Ok(());
    };
    client.set_timeout(Some(std::time::Duration::from_secs(5)))?;
    let pid = match client.request(ipc::Request::Health) {
        Ok(ipc::Response::Health(h)) => Some(h.pid),
        _ => None,
    };
    // The daemon may close the connection before its answer arrives; whether it
    // stopped is told by its process (or, without a pid, its socket), below.
    let _ = client.request(ipc::Request::Shutdown);
    let running = || match pid {
        // A zombie has exited; only its parent's reaping is left.
        Some(pid) => std::fs::read_to_string(format!("/proc/{pid}/stat")).is_ok_and(|stat| {
            stat.rsplit_once(") ")
                .is_some_and(|(_, rest)| !rest.starts_with('Z'))
        }),
        None => Client::connect().is_ok(),
    };
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while running() {
        if std::time::Instant::now() > deadline {
            bail!("bigmand was asked to stop but is still running after 30 seconds.");
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    println!("bigmand stopped. The desktop app starts it again whenever it is open, and so does your next login if the background services are installed.");
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
        DaemonCommands::Stop => daemon_stop(),
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

/// Where dnf's incoming packages are staged: root-only, or `BIGMAN_PREVIEW_DIR` in tests.
fn stage_dir() -> std::path::PathBuf {
    std::env::var_os("BIGMAN_PREVIEW_DIR").map_or_else(|| "/run/bigman-preview".into(), Into::into)
}

/// dnf stages incoming packages per transaction, keyed by the dnf process.
fn stage_file() -> std::path::PathBuf {
    stage_dir().join(format!("{}.list", std::os::unix::process::parent_id()))
}

fn preview_stage(fields: &[String]) -> Result<()> {
    use std::{
        io::Write,
        os::unix::fs::{DirBuilderExt, OpenOptionsExt},
    };
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(stage_dir())?;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .open(stage_file())?;
    writeln!(f, "{}", fields.join(" "))?;
    Ok(())
}

/// 0 to continue, 1 to stop the transaction. Anything that goes wrong continues.
fn receipt_preview(manager: bigman_core::checks::preview::Manager) -> i32 {
    match receipt_preview_inner(manager) {
        Ok(true) => 0,
        Ok(false) => {
            // dnf5's actions plugin reads `stop=` from the action's output.
            println!("stop=Stopped in Bigman's receipt preview");
            eprintln!("Bigman: the transaction was stopped from the receipt preview.");
            1
        }
        Err(e) => {
            eprintln!("Bigman: no receipt preview this time ({e:#}); the transaction continues.");
            0
        }
    }
}

fn incoming(
    manager: bigman_core::checks::preview::Manager,
) -> Result<Vec<bigman_core::checks::preview::Pending>> {
    use bigman_core::checks::preview::{pacman_cached, parse_dnf_stage, Manager, Pending};
    use bigman_core::platform::{ChangeKind, PackageChange};
    use std::io::Read;
    let stop = runner::cancel();
    let ran = |program: &str, args: &[&str]| {
        runner::run(std::process::Command::new(program).args(args), 30, &stop).ok()
    };
    Ok(match manager {
        Manager::Dnf => {
            let path = stage_file();
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            let _ = std::fs::remove_file(&path);
            text.lines()
                .filter_map(|l| parse_dnf_stage(l, std::path::Path::new("/var/cache/libdnf5")))
                .collect()
        }
        Manager::Apt => {
            let mut stdin = String::new();
            std::io::stdin().read_to_string(&mut stdin)?;
            stdin
                .lines()
                .map(str::trim)
                .filter(|l| l.ends_with(".deb"))
                .filter_map(|path| {
                    let fields = ran("dpkg-deb", &["-f", path, "Package", "Version"])?.stdout;
                    let field = |name: &str| {
                        fields
                            .lines()
                            .find_map(|l| l.strip_prefix(&format!("{name}: ")).map(str::to_string))
                    };
                    let name = field("Package")?;
                    let installed = ran("dpkg-query", &["-W", "-f", "${Status}", &name])
                        .is_some_and(|o| o.stdout.contains("installed"));
                    Some(Pending {
                        change: PackageChange {
                            name,
                            version: field("Version")?,
                            kind: if installed {
                                ChangeKind::Upgrade
                            } else {
                                ChangeKind::Install
                            },
                        },
                        archive: Some(path.into()),
                        repository: None,
                    })
                })
                .collect()
        }
        Manager::Pacman => {
            let mut stdin = String::new();
            std::io::stdin().read_to_string(&mut stdin)?;
            stdin
                .split_whitespace()
                .map(|name| {
                    let archive =
                        pacman_cached(std::path::Path::new("/var/cache/pacman/pkg"), name);
                    let version = archive
                        .as_ref()
                        .and_then(|a| {
                            a.file_name()?
                                .to_string_lossy()
                                .split(".pkg.tar")
                                .next()
                                .map(str::to_string)
                        })
                        .and_then(|stem| stem.strip_prefix(&format!("{name}-")).map(str::to_string))
                        .unwrap_or_default();
                    let installed = ran("pacman", &["-Q", name]).is_some_and(|o| o.ok());
                    Pending {
                        change: PackageChange {
                            name: name.into(),
                            version,
                            kind: if installed {
                                ChangeKind::Upgrade
                            } else {
                                ChangeKind::Install
                            },
                        },
                        archive,
                        repository: None,
                    }
                })
                .collect()
        }
    })
}

/// Daemons to ask: the one of whoever ran sudo or pkexec, else every running one.
fn preview_sockets() -> Vec<std::path::PathBuf> {
    if let Some(socket) = std::env::var_os("BIGMAN_SOCKET") {
        return vec![socket.into()];
    }
    if let Some(uid) = bigman_core::checks::preview::invoking_uid() {
        return vec![std::path::Path::new("/run/user")
            .join(uid.to_string())
            .join("bigman/bigmand.sock")];
    }
    std::fs::read_dir("/run/user")
        .map(|dirs| {
            dirs.flatten()
                .map(|d| d.path().join("bigman/bigmand.sock"))
                .filter(|p| p.exists())
                .collect()
        })
        .unwrap_or_default()
}

fn receipt_preview_inner(manager: bigman_core::checks::preview::Manager) -> Result<bool> {
    let pending = incoming(manager)?;
    if pending.is_empty() {
        return Ok(true);
    }
    let command = std::fs::read(format!(
        "/proc/{}/cmdline",
        std::os::unix::process::parent_id()
    ))
    .map(|b| {
        String::from_utf8_lossy(&b)
            .replace('\0', " ")
            .trim()
            .to_string()
    })
    .unwrap_or_default();
    let receipt =
        bigman_core::checks::preview::build(manager, &pending, &command, &runner::cancel());
    let mut announced = false;
    for socket in preview_sockets() {
        let Ok(mut client) = Client::connect_to(&socket) else {
            continue;
        };
        // Said once, in the terminal where the package manager is waiting.
        if !announced {
            eprintln!(
                "Bigman: showing this transaction's receipt; it continues in {} s unless you stop it in Bigman.",
                bigman_core::checks::preview::DECISION_SECS
            );
            announced = true;
        }
        client.set_timeout(Some(std::time::Duration::from_secs(
            bigman_core::checks::preview::DECISION_SECS + 15,
        )))?;
        if let ipc::Response::Decision(d) = client.request(ipc::Request::PreviewTransaction {
            receipt: Box::new(receipt.clone()),
        })? {
            if !d.proceed {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
