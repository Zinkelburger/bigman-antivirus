//! Gathering for the advisory check: the package tools and the public data
//! files, cached in the private store. Every gap is coverage, never silence.
use super::{
    assess,
    backends::{self, Avg, DnfInfo, DnfItem},
    os_release,
    version::{compare, Scheme},
    Family, Inputs,
};
use crate::{
    fetch::{self, Limits},
    model::{now, Report, SkipReason, Skipped},
    runner::{self, Cancel},
    store::Store,
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{path::Path, process::Command};

const DAY: u64 = 86_400;
/// Older than this, cached data no longer counts as checking.
const STALE: u64 = 7 * DAY;

/// One public data file: where it is cached, what it is called, where it comes from.
struct DataFile {
    name: &'static str,
    label: &'static str,
    url: &'static str,
    /// Largest download accepted.
    bytes: u64,
}

const KEV: DataFile = DataFile {
    name: "kev.json",
    label: "CISA Known Exploited Vulnerabilities",
    url: "https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json",
    bytes: 16 * 1024 * 1024,
};
const DEBIAN: DataFile = DataFile {
    name: "debian-tracker.json",
    label: "Debian security tracker",
    url: "https://security-tracker.debian.org/tracker/data/json",
    bytes: 64 * 1024 * 1024,
};
const ARCH: DataFile = DataFile {
    name: "arch-all.json",
    label: "Arch Linux security tracker",
    url: "https://security.archlinux.org/issues/all.json",
    bytes: 16 * 1024 * 1024,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    /// Download the data again even when the cache is less than a day old.
    pub refresh: bool,
    /// Never use the network; cached data only.
    pub offline: bool,
}

#[derive(Serialize, Deserialize)]
struct Meta {
    url: String,
    fetched: u64,
    last_modified: Option<String>,
}

/// A cached public data file, refreshed when older than a day (or on request).
/// Returns the file and its age; a failed refresh keeps the old copy but says so.
fn cached(
    store: &Store,
    r: &mut Report,
    data: &DataFile,
    opts: Options,
    stop: &Cancel,
) -> Option<std::path::PathBuf> {
    let (name, label, url) = (data.name, data.label, data.url);
    let file = store.path(&format!("advisories/{name}")).ok()?;
    let meta_path = store.path(&format!("advisories/{name}.meta.json")).ok()?;
    let meta: Option<Meta> = std::fs::read(&meta_path)
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok());
    let age = meta.as_ref().map(|m| now().saturating_sub(m.fetched));
    let fresh = file.exists() && age.is_some_and(|a| a < DAY);
    if !opts.offline && (opts.refresh || !fresh) {
        let since = meta
            .as_ref()
            .filter(|_| file.exists())
            .and_then(|m| m.last_modified.clone());
        match fetch::download(
            url,
            &file,
            &Limits {
                bytes: data.bytes,
                seconds: 180,
            },
            since.as_deref(),
            stop,
        ) {
            Ok(got) => {
                let m = Meta {
                    url: url.into(),
                    fetched: now(),
                    last_modified: got.last_modified.or(since),
                };
                let _ = store.write(&format!("advisories/{name}.meta.json"), &m);
                r.coverage.push(format!(
                    "{label}: {} from {url}.",
                    if got.status == 304 {
                        "unchanged since the last download"
                    } else {
                        "downloaded"
                    }
                ));
                return Some(file);
            }
            Err(e) => r.coverage.skipped.push(Skipped {
                what: format!("{label} refresh"),
                why: SkipReason::Failed,
                detail: format!("{e:#}"),
            }),
        }
    }
    match age {
        Some(a) if file.exists() && a <= STALE => {
            r.coverage.push(format!(
                "{label}: cached copy from {} ago.",
                crate::model::age(a)
            ));
            Some(file)
        }
        Some(a) if file.exists() => {
            r.coverage.skipped.push(Skipped {
                what: label.into(),
                why: SkipReason::Unreadable,
                detail: format!(
                    "The cached copy is {} old; refresh it before trusting the result.",
                    crate::model::age(a)
                ),
            });
            None
        }
        _ => {
            if opts.offline {
                r.coverage.skipped.push(Skipped {
                    what: label.into(),
                    why: SkipReason::Unreadable,
                    detail: "Not downloaded yet, and the check ran offline.".into(),
                });
            }
            None
        }
    }
}

fn run(program: &str, args: &[&str], seconds: u64, stop: &Cancel) -> Result<runner::Output> {
    let out = runner::run(Command::new(program).args(args), seconds, stop)?;
    if out.interrupted || out.truncated {
        bail!("{program} did not finish: {}", out.stderr.trim());
    }
    Ok(out)
}

/// Running kernel against the newest installed one (Fedora) or the distribution's flag.
fn reboot(family: Family, stop: &Cancel) -> Option<String> {
    let running = std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .ok()?
        .trim()
        .to_string();
    match family {
        Family::Fedora => {
            let out = run("rpm", &["-q", "kernel-core", "--qf", "%{VERSION}-%{RELEASE}.%{ARCH}\\n"], 30, stop).ok()?;
            let newest = out.stdout.lines().max_by(|a, b| compare(Scheme::Rpm, a, b))?.to_string();
            (newest != running).then(|| format!("Running kernel {running}; kernel {newest} is installed and takes effect after a restart."))
        }
        Family::Debian => {
            let flag = Path::new("/run/reboot-required");
            flag.exists().then(|| {
                let packages = std::fs::read_to_string("/run/reboot-required.pkgs").unwrap_or_default();
                format!("/run/reboot-required is set{}", if packages.trim().is_empty() { String::new() } else { format!(" by: {}", packages.split_whitespace().collect::<Vec<_>>().join(", ")) })
            })
        }
        Family::Arch => (!Path::new("/usr/lib/modules").join(&running).exists())
            .then(|| format!("Running kernel {running}, whose modules are no longer installed: the kernel was updated.")),
    }
}

fn fedora(r: &mut Report, inputs: &mut Inputs, stop: &Cancel) -> Result<()> {
    let inventory = run(
        "rpm",
        &[
            "-qa",
            "--qf",
            "%{NAME}\\t%{EPOCHNUM}:%{VERSION}-%{RELEASE}\\n",
        ],
        60,
        stop,
    )?;
    let installed = backends::parse_rpm_inventory(&inventory.stdout);
    r.coverage
        .checked
        .push(format!("{} installed rpm packages", installed.len()));
    // dnf refreshes repository metadata into your cache, so this uses the network.
    let list = run(
        "dnf",
        &[
            "--assumeno",
            "advisory",
            "list",
            "--security",
            "--updates",
            "--json",
        ],
        300,
        stop,
    )?;
    if !list.ok() {
        bail!("dnf could not list advisories: {}", list.stderr.trim());
    }
    if list
        .stderr
        .lines()
        .any(|l| l.to_lowercase().contains("error") || l.contains("Failed"))
    {
        r.coverage.skipped.push(Skipped {
            what: "Some repositories".into(),
            why: SkipReason::Failed,
            detail: list.stderr.trim().into(),
        });
    }
    let items: Vec<DnfItem> = serde_json::from_str(
        list.stdout
            .trim()
            .strip_prefix('\u{feff}')
            .unwrap_or(list.stdout.trim()),
    )
    .context("dnf's advisory list was not JSON")?;
    let mut names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
    names.sort_unstable();
    names.dedup();
    let mut info: Vec<DnfInfo> = vec![];
    for chunk in names.chunks(50) {
        let mut args = vec!["--assumeno", "advisory", "info", "--json"];
        args.extend(chunk);
        let out = run("dnf", &args, 300, stop)?;
        info.extend(serde_json::from_str::<Vec<DnfInfo>>(&out.stdout).unwrap_or_default());
    }
    inputs.applies = backends::fedora(&items, &info, &installed);
    r.coverage
        .checked
        .push("Fedora updateinfo through dnf".into());
    let extras = run("dnf", &["--assumeno", "-q", "list", "--extras"], 120, stop)?;
    inputs.untracked = backends::parse_dnf_extras(&extras.stdout);
    Ok(())
}

fn debian(
    store: &Store,
    r: &mut Report,
    inputs: &mut Inputs,
    opts: Options,
    stop: &Cancel,
) -> Result<()> {
    let release = inputs.distro.codename.clone();
    if release.is_empty() {
        bail!("/etc/os-release names no VERSION_CODENAME");
    }
    let inventory = run("dpkg-query", &["-W", "-f", "${binary:Package}\\t${Version}\\t${source:Package}\\t${source:Version}\\t${db:Status-Abbrev}\\n"], 60, stop)?;
    let installed = backends::parse_dpkg_inventory(&inventory.stdout);
    r.coverage
        .checked
        .push(format!("{} installed deb packages", installed.len()));
    let Some(file) = cached(store, r, &DEBIAN, opts, stop) else {
        return Ok(());
    };
    let data = backends::parse_debian_tracker(
        std::io::BufReader::new(std::fs::File::open(file)?),
        &release,
    )
    .context("The Debian security tracker data could not be read")?;
    if data.is_empty() {
        inputs.unsupported_release = Some(format!(
            "The Debian security tracker has no entries for {release}."
        ));
    }
    inputs.applies = backends::debian(&data, &installed, &release);
    r.coverage
        .checked
        .push(format!("Debian security tracker, {release}"));
    Ok(())
}

fn arch(
    store: &Store,
    r: &mut Report,
    inputs: &mut Inputs,
    opts: Options,
    stop: &Cancel,
) -> Result<()> {
    let inventory = run("pacman", &["-Q"], 60, stop)?;
    let installed = backends::parse_pacman_inventory(&inventory.stdout);
    r.coverage
        .checked
        .push(format!("{} installed pacman packages", installed.len()));
    let foreign = run("pacman", &["-Qm"], 60, stop)?;
    inputs.untracked = foreign.stdout.lines().map(str::to_string).collect();
    let Some(file) = cached(store, r, &ARCH, opts, stop) else {
        return Ok(());
    };
    let groups: Vec<Avg> = serde_json::from_slice(&std::fs::read(file)?)
        .context("The Arch security data could not be read")?;
    inputs.applies = backends::arch(&groups, &installed);
    r.coverage
        .checked
        .push("Arch Linux security tracker".into());
    Ok(())
}

/// Checks installed packages against the distribution's security data.
pub fn advisories(store: &Store, opts: Options, stop: &Cancel) -> Result<Report> {
    let distro = os_release(&std::fs::read_to_string("/etc/os-release").unwrap_or_default());
    let mut r = Report::new(
        "Security updates",
        if distro.pretty.is_empty() {
            &distro.id
        } else {
            &distro.pretty
        },
    );
    r.coverage.push("The distribution's own security data decides what is affected and what fixes it; CISA's catalog of exploited vulnerabilities only raises the severity. Bigman shows the update command and never runs it. No package list leaves this computer.".into());
    let mut inputs = Inputs {
        distro: distro.clone(),
        ..Default::default()
    };
    let Some(family) = distro.family() else {
        r.coverage.skipped.push(Skipped {
            what: format!("Security data for {}", if distro.pretty.is_empty() { "this distribution" } else { &distro.pretty }),
            why: SkipReason::NotApplicable,
            detail: "Bigman reads Fedora, Debian and Arch Linux security data; other distributions are not supported yet.".into(),
        });
        store.save(&r)?;
        return Ok(r);
    };
    let gathered = match family {
        Family::Fedora => fedora(&mut r, &mut inputs, stop),
        Family::Debian => debian(store, &mut r, &mut inputs, opts, stop),
        Family::Arch => arch(store, &mut r, &mut inputs, opts, stop),
    };
    if let Err(e) = gathered {
        r.coverage.skipped.push(Skipped {
            what: "Installed-package check".into(),
            why: SkipReason::Failed,
            detail: format!("{e:#}"),
        });
    }
    if let Some(file) = cached(store, &mut r, &KEV, opts, stop) {
        match std::fs::read_to_string(file)
            .map_err(anyhow::Error::from)
            .and_then(|t| Ok(backends::parse_kev(&t)?))
        {
            Ok(kev) => {
                inputs.kev = kev;
                r.coverage
                    .checked
                    .push("CISA Known Exploited Vulnerabilities".into());
            }
            Err(e) => r.coverage.skipped.push(Skipped {
                what: "CISA Known Exploited Vulnerabilities".into(),
                why: SkipReason::Unreadable,
                detail: format!("{e:#}"),
            }),
        }
    }
    inputs.reboot = reboot(family, stop);
    assess(&mut r, &inputs);
    // The report's details carry one command for all pending updates.
    r.details = super::install_all(Some(family), &inputs.applies).unwrap_or_default();
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
    store.save(&r)?;
    Ok(r)
}
