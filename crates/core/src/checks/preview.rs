//! Receipts before a transaction commits, from the downloaded package files.
//! The opt-in hooks (see `hooks`) run `bigman receipt --preview` as root; it reads
//! each archive's file list and install scripts without unpacking or running
//! anything, builds the same receipt the daemon builds afterwards, and asks your
//! daemon whether to continue. Every failure continues: Bigman never blocks the
//! package manager because of itself.
use crate::{
    checks::receipt::{self, Receipt},
    platform::{
        packagedb::parse_rpm_scripts, Attribution, ChangeKind, PackageChange, PackageDb,
        PackageScript, Transaction,
    },
    runner::{self, Cancel},
};
use anyhow::{bail, Context, Result};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
};

/// Seconds the app has to answer before the transaction continues on its own.
pub const DECISION_SECS: u64 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manager {
    Dnf,
    Apt,
    Pacman,
}

impl Manager {
    pub fn parse(s: &str) -> Option<Manager> {
        match s {
            "dnf" => Some(Manager::Dnf),
            "apt" => Some(Manager::Apt),
            "pacman" => Some(Manager::Pacman),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Manager::Dnf => "dnf",
            Manager::Apt => "apt",
            Manager::Pacman => "pacman",
        }
    }
}

/// A package about to be installed, and where its archive is.
#[derive(Debug, Clone)]
pub struct Pending {
    pub change: PackageChange,
    pub archive: Option<PathBuf>,
    /// The repository it comes from, when the package manager says (dnf does).
    pub repository: Option<String>,
}

/// Reads file lists and scripts from package archives rather than the database.
struct Archives {
    manager: Manager,
    by_name: BTreeMap<String, PathBuf>,
}

fn output(program: &str, args: &[&str], stop: &Cancel) -> Result<String> {
    let out = runner::run(Command::new(program).args(args), 60, stop)?;
    if !out.ok() {
        bail!(
            "{program} could not read the package: {}",
            out.stderr.trim()
        );
    }
    Ok(out.stdout)
}

impl Archives {
    fn archive(&self, package: &str) -> Result<&Path> {
        self.by_name
            .get(package)
            .map(PathBuf::as_path)
            .context("The downloaded package file was not found")
    }
}

impl PackageDb for Archives {
    fn manager(&self) -> &'static str {
        match self.manager {
            Manager::Dnf => "rpm archives",
            Manager::Apt => "deb archives",
            Manager::Pacman => "pacman archives",
        }
    }

    fn attribute(&self, _: &[PathBuf], _: &Cancel) -> Result<BTreeMap<PathBuf, Attribution>> {
        Ok(BTreeMap::new())
    }

    fn files(&self, package: &str, stop: &Cancel) -> Result<Vec<PathBuf>> {
        let archive = self.archive(package)?.to_string_lossy().into_owned();
        Ok(match self.manager {
            Manager::Dnf => output("rpm", &["-qpl", &archive], stop)?
                .lines()
                .filter(|l| l.starts_with('/'))
                .map(PathBuf::from)
                .collect(),
            Manager::Apt => parse_tar_listing(&output("dpkg-deb", &["-c", &archive], stop)?),
            Manager::Pacman => output("bsdtar", &["-tf", &archive], stop)?
                .lines()
                .filter(|l| !l.starts_with('.') && !l.ends_with('/'))
                .map(|l| Path::new("/").join(l))
                .collect(),
        })
    }

    fn scripts(&self, package: &str, stop: &Cancel) -> Result<Vec<PackageScript>> {
        let archive = self.archive(package)?.to_string_lossy().into_owned();
        Ok(match self.manager {
            Manager::Dnf => parse_rpm_scripts(
                package,
                &output("rpm", &["-qp", "--scripts", &archive], stop)?,
            ),
            Manager::Apt => {
                let mut scripts = vec![];
                for phase in ["preinst", "postinst", "prerm", "postrm", "config"] {
                    // `dpkg-deb -I archive member` prints one control member, or fails when absent.
                    let Ok(text) = output("dpkg-deb", &["-I", &archive, phase], stop) else {
                        continue;
                    };
                    let interpreter = text
                        .lines()
                        .next()
                        .and_then(|l| l.strip_prefix("#!"))
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    scripts.push(PackageScript {
                        package: package.into(),
                        phase: phase.into(),
                        interpreter,
                        text,
                    });
                }
                scripts
            }
            Manager::Pacman => match output("bsdtar", &["-xOf", &archive, ".INSTALL"], stop) {
                Ok(text) if !text.trim().is_empty() => {
                    vec![PackageScript {
                        package: package.into(),
                        phase: "install".into(),
                        interpreter: "/bin/sh".into(),
                        text,
                    }]
                }
                _ => vec![],
            },
        })
    }
}

/// `dpkg-deb -c` lists like `tar -tv`: the path is the last column, `./usr/bin/x`.
pub fn parse_tar_listing(out: &str) -> Vec<PathBuf> {
    out.lines()
        .filter_map(|l| l.split_whitespace().nth(5))
        .filter(|p| !p.ends_with('/'))
        .map(|p| Path::new("/").join(p.trim_start_matches("./")))
        .collect()
}

/// dnf5 actions give `${pkg.location}` relative to the repository; the downloaded
/// file sits in the cache under a directory named after the repository id.
pub fn dnf_cached(cache: &Path, repo: &str, location: &str) -> Option<PathBuf> {
    let file = Path::new(location).file_name()?;
    std::fs::read_dir(cache)
        .ok()?
        .flatten()
        .filter(|d| {
            d.file_name()
                .to_string_lossy()
                .starts_with(&format!("{repo}-"))
        })
        .map(|d| d.path().join("packages").join(file))
        .find(|p| p.is_file())
}

/// pacman's hook passes names; the newest cached archive whose name parses back
/// to that package is the one about to be installed.
pub fn pacman_cached(cache: &Path, name: &str) -> Option<PathBuf> {
    let mut found: Vec<(std::time::SystemTime, PathBuf)> = std::fs::read_dir(cache)
        .ok()?
        .flatten()
        .filter_map(|d| {
            let file = d.file_name().to_string_lossy().into_owned();
            let stem = file.split(".pkg.tar").next()?;
            // name-version-release-arch; the name itself may contain dashes.
            let parts: Vec<&str> = stem.rsplitn(4, '-').collect();
            (file.contains(".pkg.tar")
                && !file.ends_with(".sig")
                && parts.len() == 4
                && parts[3] == name)
                .then(|| Some((d.metadata().ok()?.modified().ok()?, d.path())))
                .flatten()
        })
        .collect();
    found.sort();
    found.pop().map(|(_, p)| p)
}

fn kind_of_dnf(action: &str) -> ChangeKind {
    match action {
        "U" => ChangeKind::Upgrade,
        "D" => ChangeKind::Downgrade,
        "R" => ChangeKind::Reinstall,
        "E" | "O" => ChangeKind::Remove,
        _ => ChangeKind::Install,
    }
}

/// One staged dnf package: `full_nevra repo_id location action`.
pub fn parse_dnf_stage(line: &str, cache: &Path) -> Option<Pending> {
    let mut f = line.split_whitespace();
    let (nevra, repo, location, action) = (f.next()?, f.next()?, f.next()?, f.next()?);
    // name-epoch:version-release.arch → name
    let name = nevra.rsplitn(3, '-').nth(2)?.to_string();
    let version = nevra.strip_prefix(&format!("{name}-"))?.to_string();
    Some(Pending {
        change: PackageChange {
            name,
            version,
            kind: kind_of_dnf(action),
        },
        // Local files named on the command line keep their own path.
        repository: (!repo.starts_with('@')).then(|| repo.to_string()),
        archive: match Path::new(location) {
            local if local.is_absolute() && local.is_file() => Some(local.to_path_buf()),
            _ => dnf_cached(cache, repo, location),
        },
    })
}

/// The preview receipt for packages about to change.
pub fn build(manager: Manager, pending: &[Pending], command: &str, stop: &Cancel) -> Receipt {
    let archives = Archives {
        manager,
        by_name: pending
            .iter()
            .filter_map(|p| Some((p.change.name.clone(), p.archive.clone()?)))
            .collect(),
    };
    let t = Transaction {
        id: format!("preview-{}", std::process::id()),
        at: crate::model::now(),
        manager: manager.name().into(),
        command: command.into(),
        changes: pending.iter().map(|p| p.change.clone()).collect(),
    };
    let none = |_: &str, _: &str| None;
    let mut r = receipt::build(
        uuid::Uuid::new_v4().to_string(),
        &t,
        &archives,
        &receipt::History { hook: &none },
        stop,
    );
    r.repositories = pending
        .iter()
        .filter_map(|p| Some((p.change.name.clone(), p.repository.clone()?)))
        .collect();
    r.coverage
        .notes
        .retain(|n| !n.starts_with("Built after the transaction"));
    r.coverage.push("A preview from the downloaded package files, before anything was installed. Nothing was unpacked or run.".into());
    r
}

/// The user whose daemon should see the preview: whoever ran sudo or pkexec.
pub fn invoking_uid() -> Option<u32> {
    ["SUDO_UID", "PKEXEC_UID"]
        .iter()
        .find_map(|v| std::env::var(v).ok()?.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_downloaded_packages_in_the_caches() {
        let dir = tempfile::tempdir().unwrap();
        let dnf = dir.path().join("dnf");
        std::fs::create_dir_all(dnf.join("fedora-0123456789abcdef/packages")).unwrap();
        std::fs::write(
            dnf.join("fedora-0123456789abcdef/packages/nix-2.24.9-1.fc44.x86_64.rpm"),
            "",
        )
        .unwrap();
        let staged = parse_dnf_stage(
            "nix-0:2.24.9-1.fc44.x86_64 fedora Packages/n/nix-2.24.9-1.fc44.x86_64.rpm I",
            &dnf,
        )
        .unwrap();
        assert_eq!(
            (staged.change.name.as_str(), staged.change.kind),
            ("nix", ChangeKind::Install)
        );
        assert!(staged
            .archive
            .unwrap()
            .ends_with("packages/nix-2.24.9-1.fc44.x86_64.rpm"));

        let pacman = dir.path().join("pacman");
        std::fs::create_dir_all(&pacman).unwrap();
        for f in [
            "python-foo-1.0-1-any.pkg.tar.zst",
            "python-foo-1.0-1-any.pkg.tar.zst.sig",
            "python-foo-bar-2.0-1-any.pkg.tar.zst",
        ] {
            std::fs::write(pacman.join(f), "").unwrap();
        }
        assert_eq!(
            pacman_cached(&pacman, "python-foo")
                .unwrap()
                .file_name()
                .unwrap(),
            "python-foo-1.0-1-any.pkg.tar.zst"
        );
    }

    #[test]
    fn reads_deb_file_listings() {
        let out = "drwxr-xr-x root/root         0 2026-10-01 10:00 ./usr/bin/\n-rwxr-xr-x root/root     14328 2026-10-01 10:00 ./usr/bin/tool\n";
        assert_eq!(parse_tar_listing(out), [PathBuf::from("/usr/bin/tool")]);
    }
}
