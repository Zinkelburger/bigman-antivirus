use anyhow::Result;
use bigman_core::{ipc, model, runner, store::Store};
use bigman_daemon::{server, Daemon};
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
    eprintln!("bigmand: listening on {}", socket.path.display());
    server::serve(&socket, Arc::new(Daemon::new(store)), &stop)
}
