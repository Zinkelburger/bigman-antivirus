//! A disposable container for an untrusted project or file: rootless Podman with a
//! copy of the target and nothing else from this computer, deleted when the shell
//! exits. `plan` is pure, so the daemon's preview and the terminal's run agree;
//! only the CLI process, never the daemon, starts Podman.
use crate::{
    runner::{self, Cancel},
    store::Store,
};
use anyhow::{bail, Context, Result};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

/// Every container Bigman starts carries this label, set to the starting
/// process's ID, so a later run can remove one whose session was killed.
pub const LABEL: &str = "io.bigman.try";
const UID: &str = "1000";
const HOME: &str = "/home/user";
const WORK: &str = "/work";
const MEMORY: &str = "4g";
const MEMORY_TEXT: &str = "4 GiB";
const PROCESSES: &str = "4096";

/// The image for the project files Bigman recognises, first match wins.
const IMAGES: &[(&[&str], &str)] = &[
    (&["package.json"], "docker.io/library/node:lts"),
    (
        &["pyproject.toml", "requirements.txt", "setup.py", "Pipfile"],
        "docker.io/library/python:3",
    ),
    (&["Cargo.toml"], "docker.io/library/rust:1"),
    (&["go.mod"], "docker.io/library/golang:1"),
    (&["Gemfile"], "docker.io/library/ruby:3"),
];
const FALLBACK: &str = "docker.io/library/debian:stable";
const SHELL: &str = "if command -v bash >/dev/null; then exec bash -l; else exec sh -l; fi";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// No network at all instead of the default outbound network.
    pub offline: bool,
    /// Container ports to reach from this computer, on 127.0.0.1 only.
    pub ports: Vec<u16>,
    /// A fully qualified image instead of the one chosen from the project files.
    pub image: Option<String>,
    /// Root inside the container, for installing system packages. Still not root
    /// on this computer, but an escape would act as your account.
    pub root: bool,
    /// What to run instead of an interactive shell.
    pub command: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub target: PathBuf,
    pub image: String,
    /// Why this image: the project file that chose it, or the option.
    pub image_reason: String,
    pub options: Options,
}

pub fn plan(target: &Path, options: Options) -> Result<Plan> {
    let target = target
        .canonicalize()
        .with_context(|| format!("Cannot open {}", target.display()))?;
    if !target.is_dir() && !target.is_file() {
        bail!("Choose a folder or a regular file");
    }
    if options.ports.contains(&0) {
        bail!("Ports run from 1 to 65535");
    }
    let (image, image_reason) = match &options.image {
        Some(image) => {
            check_image(image)?;
            (image.clone(), "you chose it with --image".into())
        }
        None => choose_image(&target),
    };
    Ok(Plan {
        target,
        image,
        image_reason,
        options,
    })
}

/// Podman resolves a short name through whichever registries are configured, so
/// only a name with its registry is accepted.
fn check_image(image: &str) -> Result<()> {
    let registry = image.split('/').next().unwrap_or_default();
    if image.starts_with('-')
        || !image.contains('/')
        || !image
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._-/:@".contains(c))
        || !(registry.contains('.') || registry.contains(':') || registry == "localhost")
    {
        bail!("Give the image's full name with its registry, such as docker.io/library/node:22. A short name can resolve to a different registry.");
    }
    Ok(())
}

fn choose_image(target: &Path) -> (String, String) {
    if target.is_dir() {
        for (files, image) in IMAGES {
            if let Some(f) = files.iter().find(|f| target.join(f).is_file()) {
                return ((*image).into(), format!("the project has {f}"));
            }
        }
    }
    (
        FALLBACK.into(),
        "Bigman recognises no project file here".into(),
    )
}

impl Plan {
    /// `--network`'s value. Both backends are told not to map the host's
    /// loopback, so services listening only on 127.0.0.1 stay out of reach.
    pub fn network(&self) -> String {
        if self.options.offline {
            "none".into()
        } else if runner::available("pasta") {
            "pasta:--no-map-gw".into()
        } else {
            "slirp4netns:allow_host_loopback=false".into()
        }
    }

    /// The arguments to `podman create`. `owner` is the process ID of the `bigman
    /// try` that removes the container.
    pub fn create_args(&self, name: &str, owner: &str, interactive: bool) -> Vec<String> {
        let mut a: Vec<String> = ["create", "--rm", "--name", name, "--hostname", "bigman-try"]
            .map(String::from)
            .into();
        a.push("--label".into());
        a.push(format!("{LABEL}={owner}"));
        if interactive {
            a.push("--interactive".into());
            a.push("--tty".into());
        }
        // Untrusted output stays out of your journal.
        a.extend(["--log-driver", "none"].map(String::from));
        if !self.options.root {
            a.extend(
                [
                    "--user",
                    &format!("{UID}:{UID}"),
                    "--cap-drop",
                    "all",
                    "--env",
                    &format!("HOME={HOME}"),
                ]
                .map(String::from),
            );
        }
        a.extend(
            [
                "--security-opt",
                "no-new-privileges",
                "--workdir",
                WORK,
                "--memory",
                MEMORY,
                "--pids-limit",
                PROCESSES,
                "--network",
                &self.network(),
            ]
            .map(String::from),
        );
        for p in &self.options.ports {
            a.push("--publish".into());
            a.push(format!("127.0.0.1:{p}:{p}"));
        }
        a.push(self.image.clone());
        if self.options.command.is_empty() {
            a.extend(["sh", "-c", SHELL].map(String::from));
        } else {
            a.extend(self.options.command.iter().cloned());
        }
        a
    }

    pub fn summary(&self) -> String {
        let name = self
            .target
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.target.display().to_string());
        let network = if self.options.offline {
            "Code inside has no network."
        } else {
            "Code inside can use the network."
        };
        format!(
            "Copy {name} into a new Podman container and open a shell there in a terminal window. The container gets that copy and none of your home folder, keys, browser data or environment variables. {network} The container, the copy and anything downloaded into it are deleted when the shell exits."
        )
    }

    /// What the container gets and can reach, as label and value.
    pub fn facts(&self) -> Vec<(String, String)> {
        let network = if self.options.offline {
            "None".to_string()
        } else {
            "On. Code inside can reach the internet, your local network, and services on this computer that listen on its network address, but not ones that listen only on 127.0.0.1.".to_string()
        };
        let user = if self.options.root {
            "Root inside the container, which is your account's user ID outside it. Use this only to install system packages.".to_string()
        } else {
            format!("{UID}, without added privileges. Outside the container it is an unused ID, not your account.")
        };
        let registry = self.image.split('/').next().unwrap_or_default();
        let mut facts = vec![
            ("Copied in".to_string(), self.target.display().to_string()),
            (
                "Image".to_string(),
                format!(
                    "{}, because {}. The first time, Podman downloads it from {registry} (often several hundred MB), even with no network inside, and keeps it.",
                    self.image, self.image_reason
                ),
            ),
            ("Network".to_string(), network),
        ];
        if !self.options.ports.is_empty() {
            let ports = self
                .options
                .ports
                .iter()
                .map(|p| format!("127.0.0.1:{p} on this computer goes to {p} inside"))
                .collect::<Vec<_>>()
                .join(", ");
            facts.push(("Port".to_string(), ports));
        }
        facts.push(("User inside".to_string(), user));
        facts.push((
            "Limits".to_string(),
            format!("{MEMORY_TEXT} of memory, {PROCESSES} processes"),
        ));
        facts
    }

    /// The same run as one `bigman try` command, for a terminal.
    pub fn cli(&self) -> String {
        let mut c = vec!["bigman".to_string(), "try".to_string()];
        if self.options.offline {
            c.push("--offline".into());
        }
        for p in &self.options.ports {
            c.extend(["--port".into(), p.to_string()]);
        }
        if let Some(image) = &self.options.image {
            c.extend(["--image".into(), image.clone()]);
        }
        if self.options.root {
            c.push("--root".into());
        }
        c.push(quote(&self.target.to_string_lossy()));
        if !self.options.command.is_empty() {
            c.push("--".into());
            c.extend(self.options.command.iter().map(|a| quote(a)));
        }
        c.join(" ")
    }

    /// The exact commands, shown verbatim. `interactive` is whether a terminal
    /// is attached, as it is from the app.
    pub fn commands(&self, interactive: bool) -> String {
        let shell = |a: &[String]| a.iter().map(|s| quote(s)).collect::<Vec<_>>().join(" ");
        let create = self.create_args("$NAME", "$PID", interactive);
        let copy = if self.target.is_dir() {
            format!(
                "podman cp {} $NAME:{WORK}",
                quote(&self.target.to_string_lossy())
            )
        } else {
            format!("podman cp $COPY $NAME:{WORK}")
        };
        let mut lines = vec![format!("# The same as: {}", self.cli())];
        lines.push("NAME=bigman-try-<8 random characters>".into());
        lines.push("PID=<the process ID of bigman try>".into());
        if !self.target.is_dir() {
            lines.push(format!(
                "COPY=<a new folder holding a copy of {}>",
                self.target.display()
            ));
        }
        lines.push(format!("podman {}", shell(&create)));
        lines.push(copy);
        if !self.options.root {
            lines.push(format!("podman cp <an empty folder> $NAME:{HOME}"));
        }
        lines.push(format!(
            "podman start --attach{} $NAME",
            if interactive { " --interactive" } else { "" }
        ));
        lines.push("podman rm --force --volumes --time 0 $NAME".into());
        lines.join("\n")
    }

    /// Summary, facts, the kernel note and the commands, for the CLI.
    pub fn text(&self, interactive: bool) -> String {
        let facts = self
            .facts()
            .into_iter()
            .map(|(l, v)| format!("{l}: {v}"))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "{}\n\n{facts}\n\n{KERNEL}\n\n{}",
            self.summary(),
            self.commands(interactive)
        )
    }
}

/// What a container cannot promise, said wherever one is offered.
pub const KERNEL: &str = "A container shares this computer's kernel, so a kernel flaw could let code out. For something you believe is malware, use a separate computer or a virtual machine.";

fn quote(s: &str) -> String {
    if !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./:=@$,".contains(c))
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\'', r"'\''"))
    }
}

fn podman(args: &[&str]) -> Command {
    let mut c = Command::new("podman");
    c.args(args);
    c
}

fn check(c: &mut Command, what: &str) -> Result<()> {
    let status = c
        .status()
        .with_context(|| format!("Cannot run Podman to {what}"))?;
    if !status.success() {
        bail!("Podman could not {what} (exit {:?})", status.code());
    }
    Ok(())
}

/// Removes containers whose starting `bigman try` is gone (a closed terminal, a
/// killed process). Returns their names.
pub fn sweep() -> Vec<String> {
    let Ok(out) = podman(&[
        "ps",
        "--all",
        "--filter",
        &format!("label={LABEL}"),
        "--format",
        &format!("{{{{.Names}}}} {{{{index .Labels \"{LABEL}\"}}}}"),
    ])
    .stderr(Stdio::null())
    .output() else {
        return vec![];
    };
    let mut removed = vec![];
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let Some((name, pid)) = line.split_once(' ') else {
            continue;
        };
        let alive = pid
            .parse::<i32>()
            .is_ok_and(|p| p > 0 && unsafe { libc::kill(p, 0) } == 0);
        if !alive
            && podman(&["rm", "--force", "--volumes", "--time", "0", name])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .is_ok_and(|s| s.success())
        {
            removed.push(name.to_string());
        }
    }
    removed
}

static HUNG_UP: AtomicBool = AtomicBool::new(false);

extern "C" fn hang_up(_: libc::c_int) {
    HUNG_UP.store(true, Ordering::Relaxed);
}

/// Creates the container, copies the target in, attaches this terminal and
/// removes the container afterwards, whatever happened. Returns the exit code of
/// the shell or command.
pub fn run(store: &Store, plan: &Plan, stop: &Cancel) -> Result<Option<i32>> {
    if unsafe { libc::geteuid() } == 0 {
        bail!("Run this as your own user. Bigman starts only rootless containers.");
    }
    if !runner::available("podman") {
        bail!("Podman is not installed. Install it from Setup (Disposable containers) or with your package manager.");
    }
    let interactive = std::io::IsTerminal::is_terminal(&std::io::stdin());
    if !interactive && plan.options.command.is_empty() {
        bail!("Open a shell from a terminal, or give a command after --.");
    }
    // A closed window hangs up only this process when it is the terminal's
    // command, and Podman keeps the container running, so the hang-up is noted
    // and the container removed below. A handler, unlike SIG_IGN, is reset for
    // Podman when it starts.
    let handler: extern "C" fn(libc::c_int) = hang_up;
    unsafe {
        libc::signal(libc::SIGHUP, handler as libc::sighandler_t);
        libc::signal(libc::SIGTERM, handler as libc::sighandler_t);
    }
    for name in sweep() {
        eprintln!("Removed {name}, left running by an earlier session that was closed.");
    }
    let name = format!(
        "bigman-try-{}",
        &uuid::Uuid::new_v4().simple().to_string()[..8]
    );
    let scratch = tempfile::tempdir_in(&store.root)?;
    let source = if plan.target.is_dir() {
        plan.target.clone()
    } else {
        let folder = scratch.path().join("work");
        std::fs::create_dir(&folder)?;
        let file = plan.target.file_name().context("The file has no name")?;
        std::fs::copy(&plan.target, folder.join(file))
            .with_context(|| format!("Cannot copy {}", plan.target.display()))?;
        folder
    };
    let empty = scratch.path().join("home");
    std::fs::create_dir(&empty)?;
    let create = plan.create_args(&name, &std::process::id().to_string(), interactive);
    let result = (|| {
        check(
            Command::new("podman").args(&create).stdout(Stdio::null()),
            "create the container",
        )?;
        let into = |dest: &str| format!("{name}:{dest}");
        check(
            Command::new("podman")
                .arg("cp")
                .arg(&source)
                .arg(into(WORK)),
            "copy the files in",
        )?;
        if !plan.options.root {
            check(
                Command::new("podman").arg("cp").arg(&empty).arg(into(HOME)),
                "make a home folder",
            )?;
        }
        let mut start = podman(&["start", "--attach"]);
        if interactive {
            start.arg("--interactive");
        }
        let mut child = start
            .arg(&name)
            .spawn()
            .context("Cannot run Podman to start the container")?;
        let mut removing = false;
        loop {
            if let Some(status) = child.try_wait()? {
                return Ok(status.code());
            }
            if !removing && (HUNG_UP.load(Ordering::Relaxed) || stop.load(Ordering::Relaxed)) {
                removing = true;
                let _ = remove(&name);
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    })();
    if !remove(&name) {
        eprintln!("Bigman could not confirm the container was removed. Check with: podman ps --all --filter label={LABEL}");
    }
    result
}

fn remove(name: &str) -> bool {
    podman(&[
        "rm",
        "--force",
        "--ignore",
        "--volumes",
        "--time",
        "0",
        name,
    ])
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .is_ok_and(|s| s.success())
}

/// Terminals Bigman can open a command in, each with the arguments before the
/// command. The desktop's own terminal comes first.
fn terminals() -> Vec<(&'static str, &'static [&'static str])> {
    let mut list: Vec<(&str, &[&str])> = vec![
        ("xdg-terminal-exec", &[]),
        ("ptyxis", &["--"]),
        ("gnome-terminal", &["--"]),
        ("konsole", &["-e"]),
        ("xfce4-terminal", &["-x"]),
        ("xterm", &["-e"]),
    ];
    if std::env::var("XDG_CURRENT_DESKTOP").is_ok_and(|d| d.contains("KDE")) {
        list.sort_by_key(|(t, _)| *t != "konsole" && *t != "xdg-terminal-exec");
    }
    list
}

/// The command that opens a terminal window running `bigman try` for this plan.
pub fn terminal_command(store: &Store, plan: &Plan) -> Result<Vec<String>> {
    let (terminal, prefix) = terminals()
        .into_iter()
        .find(|(t, _)| runner::available(t))
        .with_context(|| {
            format!(
                "No terminal program was found. Run this in a terminal instead: {}",
                plan.cli()
            )
        })?;
    let mut c: Vec<String> = vec![terminal.into()];
    c.extend(prefix.iter().map(|s| s.to_string()));
    c.push(runner::bigman_executable()?.to_string_lossy().into_owned());
    c.extend(["--state".into(), store.root.to_string_lossy().into_owned()]);
    c.extend(["try", "--yes", "--pause"].map(String::from));
    if plan.options.offline {
        c.push("--offline".into());
    }
    for p in &plan.options.ports {
        c.extend(["--port".into(), p.to_string()]);
    }
    c.push(plan.target.to_string_lossy().into_owned());
    Ok(c)
}

/// Opens the terminal through the user's service manager, so it does not inherit
/// the daemon's no-new-privileges flag, which rootless Podman cannot run under.
pub fn open_in_terminal(store: &Store, plan: &Plan) -> Result<String> {
    if !runner::available("podman") {
        bail!("Podman is not installed. Install it from Setup (Disposable containers) or with your package manager.");
    }
    let command = terminal_command(store, plan)?;
    if runner::available("systemd-run") {
        let unit = format!(
            "bigman-try-{}",
            &uuid::Uuid::new_v4().simple().to_string()[..8]
        );
        let out = Command::new("systemd-run")
            .args(["--user", "--collect", "--quiet", "--unit", &unit])
            .args(["--description", "Bigman disposable container", "--"])
            .args(&command)
            .stdin(Stdio::null())
            .output()
            .context("Cannot run systemd-run")?;
        if !out.status.success() {
            bail!(
                "The terminal did not open: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
    } else {
        runner::spawn_detached(Command::new(&command[0]).args(&command[1..]))?;
    }
    Ok(format!(
        "A terminal window is open ({}). The container starts there; Podman may first download the image. Leaving the shell deletes the container and the copy.",
        command[0]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project(files: &[&str]) -> tempfile::TempDir {
        let d = tempfile::tempdir().unwrap();
        for f in files {
            std::fs::write(d.path().join(f), "{}").unwrap();
        }
        d
    }

    #[test]
    fn the_project_file_picks_the_image() {
        let node = project(&["package.json", "pyproject.toml"]);
        let p = plan(node.path(), Options::default()).unwrap();
        assert_eq!(p.image, "docker.io/library/node:lts");
        assert_eq!(p.image_reason, "the project has package.json");
        let python = project(&["requirements.txt"]);
        assert_eq!(
            plan(python.path(), Options::default()).unwrap().image,
            "docker.io/library/python:3"
        );
        let file = node.path().join("package.json");
        assert_eq!(plan(&file, Options::default()).unwrap().image, FALLBACK);
    }

    #[test]
    fn only_full_image_names_are_accepted() {
        let d = project(&[]);
        for bad in [
            "node:22",
            "library/node",
            "--privileged",
            "docker.io/node;rm",
        ] {
            let o = Options {
                image: Some(bad.into()),
                ..Options::default()
            };
            assert!(plan(d.path(), o).is_err(), "{bad}");
        }
        for good in [
            "docker.io/library/node:22",
            "localhost/mine",
            "quay.io/a/b@sha256:00",
        ] {
            let o = Options {
                image: Some(good.into()),
                ..Options::default()
            };
            assert!(plan(d.path(), o).is_ok(), "{good}");
        }
    }

    #[test]
    fn the_container_gets_nothing_from_this_computer() {
        let d = project(&["go.mod"]);
        let p = plan(
            d.path(),
            Options {
                ports: vec![3000],
                ..Options::default()
            },
        )
        .unwrap();
        let args = p.create_args("n", "1", true);
        let a = args.join(" ");
        for expected in [
            "--user 1000:1000",
            "--cap-drop all",
            "--security-opt no-new-privileges",
            "--publish 127.0.0.1:3000:3000",
            "--memory 4g",
            "--log-driver none",
            "--label io.bigman.try=1",
        ] {
            assert!(a.contains(expected), "{expected} missing from {a}");
        }
        for forbidden in [
            "--volume",
            "-v",
            "--env-host",
            "--privileged",
            "--mount",
            "--userns",
        ] {
            assert!(!args.iter().any(|x| x == forbidden), "{forbidden} in {a}");
        }
        assert!(a.ends_with(&format!("docker.io/library/golang:1 sh -c {SHELL}")));
        let offline = plan(
            d.path(),
            Options {
                offline: true,
                root: true,
                command: vec!["npm".into(), "test".into()],
                ..Options::default()
            },
        )
        .unwrap();
        let a = offline.create_args("n", "1", false).join(" ");
        assert!(a.contains("--network none") && a.ends_with("npm test"));
        assert!(!a.contains("--user") && !a.contains("--tty"));
        assert!(offline
            .facts()
            .contains(&("Network".to_string(), "None".to_string())));
        assert!(offline.summary().contains("no network"));
        assert!(offline.cli().starts_with("bigman try --offline --root "));
        assert!(offline.cli().ends_with(" -- npm test"));
    }

    #[test]
    fn the_preview_names_what_is_copied_and_the_limits_of_a_container() {
        let d = project(&["Cargo.toml"]);
        let p = plan(d.path(), Options::default()).unwrap();
        let text = p.text(true);
        assert!(text.contains(&format!(
            "Copied in: {}",
            d.path().canonicalize().unwrap().display()
        )));
        assert!(text.contains("shares this computer's kernel"));
        assert!(text.contains("not ones that listen only on 127.0.0.1"));
        assert!(text.contains("downloads it from docker.io"));
        assert!(p
            .commands(true)
            .contains("podman start --attach --interactive $NAME"));
        assert!(p.summary().contains("can use the network"));
    }

    #[test]
    fn ports_must_be_real() {
        let d = project(&[]);
        let o = Options {
            ports: vec![0],
            ..Options::default()
        };
        assert!(plan(d.path(), o).is_err());
        assert!(plan(&d.path().join("missing"), Options::default()).is_err());
    }
}
