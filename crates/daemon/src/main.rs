use anyhow::Result;
use bigman_core::{ipc, model, platform::linux::NotifySend, runner, sources, store::Store};
use bigman_daemon::{server, timeline, Daemon};
use clap::Parser;
use std::{path::PathBuf, sync::Arc};

#[derive(Parser)]
#[command(version, about = "Bigman's per-user collector daemon")]
struct Args {
    /// Private state directory (defaults to $XDG_DATA_HOME/bigman).
    #[arg(long)]
    state: Option<PathBuf>,
    /// Socket path (defaults to $BIGMAN_SOCKET or $XDG_RUNTIME_DIR/bigman/bigmand.sock).
    #[arg(long)]
    socket: Option<PathBuf>,
}

fn main() {
    if let Err(e) = run() {
        if let Some(running) = e.downcast_ref::<server::AlreadyRunning>() {
            eprintln!("bigmand: {running}; nothing to do.");
            return;
        }
        eprintln!("bigmand: {}", model::clean(&format!("{e:#}")));
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let store = Store::open(args.state)?;
    let socket = server::bind(&match args.socket {
        Some(p) => p,
        None => ipc::socket_path()?,
    })?;
    let stop = runner::cancel();
    let signal = stop.clone();
    ctrlc::set_handler(move || signal.store(true, std::sync::atomic::Ordering::Relaxed))?;
    let daemon = Arc::new(Daemon::new(store)?);
    let collectors = timeline::start(
        &daemon,
        sources::linux_sources(stop.clone()),
        Box::new(NotifySend),
        &stop,
    );
    eprintln!("bigmand: listening on {}", socket.path.display());
    let served = server::serve(&socket, daemon.clone(), &stop);
    stop.store(true, std::sync::atomic::Ordering::Relaxed);
    for thread in collectors {
        let _ = thread.join();
    }
    // Lets the next start tell a normal stop from a crash or kill (monitor.gap).
    let _ = daemon
        .timeline
        .db
        .set_setting(sources::DAEMON_STOPPED, &model::now().to_string());
    served
}
