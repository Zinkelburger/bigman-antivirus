//! Pending security updates for installed packages (plan D6, research doc
//! section 4). The distribution's own data decides affected and fixed; KEV only
//! raises severity. Nothing about installed packages leaves the machine: Fedora
//! is asked through dnf's own metadata, Debian and Arch data are public files.
pub mod backends;
mod gather;
pub mod version;

pub use gather::{advisories, Options};

use crate::model::{Report, Severity};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// What `/etc/os-release` says.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Distro {
    pub id: String,
    pub like: Vec<String>,
    pub version_id: String,
    pub codename: String,
    pub pretty: String,
}

pub fn os_release(text: &str) -> Distro {
    let field = |key: &str| {
        text.lines()
            .find_map(|l| l.strip_prefix(&format!("{key}=")))
            .map(|v| v.trim().trim_matches('"').trim_matches('\'').to_string())
            .unwrap_or_default()
    };
    Distro {
        id: field("ID"),
        like: field("ID_LIKE")
            .split_whitespace()
            .map(str::to_string)
            .collect(),
        version_id: field("VERSION_ID"),
        codename: field("VERSION_CODENAME"),
        pretty: field("PRETTY_NAME"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Family {
    Fedora,
    Debian,
    Arch,
}

impl Distro {
    /// The backend by the distribution's own id only; derivatives (Ubuntu, Mint,
    /// RHEL clones, Manjaro) ship their own security data and are not guessed at.
    pub fn family(&self) -> Option<Family> {
        match self.id.as_str() {
            "fedora" => Some(Family::Fedora),
            "debian" => Some(Family::Debian),
            "arch" => Some(Family::Arch),
            _ => None,
        }
    }
}

/// One installed package as the package database lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Installed {
    pub name: String,
    /// Full version in the distribution's scheme: rpm EVR, dpkg or pacman version.
    pub version: String,
    /// Debian's source package and version; advisories are keyed by source.
    #[serde(default)]
    pub source: Option<(String, String)>,
}

/// One package an advisory covers, installed and fixed versions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Affected {
    pub name: String,
    pub installed: String,
    /// Absent when the distribution has no fix yet.
    pub fixed: Option<String>,
}

/// One advisory that applies to this machine, already matched by a backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Applies {
    /// FEDORA-2026-…, a Debian source package, or an Arch AVG.
    pub id: String,
    pub packages: Vec<Affected>,
    pub cves: Vec<String>,
    /// The distribution's own rating ("Critical", "high", "Medium"), verbatim.
    pub severity: String,
    /// The update command to show; Bigman never runs it.
    pub command: String,
    /// Where the verdict came from, for evidence.
    pub source: String,
}

/// Everything the advisory rules look at, gathered by `backends` or a fixture.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inputs {
    pub distro: Distro,
    #[serde(default)]
    pub applies: Vec<Applies>,
    /// CVE ids in CISA's Known Exploited Vulnerabilities catalog.
    #[serde(default)]
    pub kev: BTreeSet<String>,
    /// Why a reboot is needed (e.g. "Running 7.2.8-200.fc44; 7.2.14-200.fc44 is installed").
    #[serde(default)]
    pub reboot: Option<String>,
    /// Packages no configured repository provides (AUR, local builds).
    #[serde(default)]
    pub untracked: Vec<String>,
    /// The release no longer receives security updates, with why.
    #[serde(default)]
    pub unsupported_release: Option<String>,
}

fn list(names: &[String]) -> String {
    match names {
        [] => String::new(),
        [one] => one.clone(),
        [a, b] => format!("{a} and {b}"),
        [a, b, rest @ ..] => format!("{a}, {b} and {} more", rest.len()),
    }
}

/// One command for every pending update, for the report's "install all" line.
pub fn install_all(family: Option<Family>, applies: &[Applies]) -> Option<String> {
    let fixable: Vec<&Applies> = applies
        .iter()
        .filter(|a| a.packages.iter().any(|p| p.fixed.is_some()))
        .collect();
    if fixable.is_empty() {
        return None;
    }
    Some(match family? {
        Family::Fedora => format!(
            "sudo dnf upgrade --advisories={}",
            fixable
                .iter()
                .map(|a| a.id.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ),
        Family::Debian => {
            let mut names: Vec<&str> = fixable
                .iter()
                .flat_map(|a| a.packages.iter().map(|p| p.name.as_str()))
                .collect();
            names.sort_unstable();
            names.dedup();
            format!(
                "sudo apt update && sudo apt install --only-upgrade {}",
                names.join(" ")
            )
        }
        Family::Arch => "sudo pacman -Syu".into(),
    })
}

/// The advisory rules. Findings are ordered most severe first by the caller.
pub fn assess(r: &mut Report, inputs: &Inputs) {
    for a in &inputs.applies {
        let names: Vec<String> = a.packages.iter().map(|p| p.name.clone()).collect();
        let versions = a
            .packages
            .iter()
            .map(|p| match &p.fixed {
                Some(f) => format!("{}: installed {}, fixed in {f}", p.name, p.installed),
                None => format!("{}: installed {}, no fix yet", p.name, p.installed),
            })
            .collect::<Vec<_>>()
            .join("\n");
        let exploited: Vec<&String> = a.cves.iter().filter(|c| inputs.kev.contains(*c)).collect();
        let cves = if a.cves.is_empty() {
            "No CVE ids listed".to_string()
        } else {
            a.cves.join(", ")
        };
        let evidence = format!(
            "{} ({})\n{versions}\nCVEs: {cves}{}\nFrom {}",
            a.id,
            a.severity,
            if exploited.is_empty() {
                String::new()
            } else {
                format!(
                    "\nIn CISA's Known Exploited Vulnerabilities catalog: {}",
                    exploited
                        .iter()
                        .map(|c| c.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
            a.source
        );
        let fixable = a.packages.iter().any(|p| p.fixed.is_some());
        if !fixable {
            r.add(
                "advisory.no-fix-yet",
                if exploited.is_empty() { Severity::Info } else { Severity::Review },
                &format!("{} is affected and {} has no fix yet", list(&names), inputs.distro.pretty_or_id()),
                evidence,
                "Nothing to install yet. Avoid the affected feature where you can; the fix will appear here when it is released.",
            );
            continue;
        }
        // The distribution's own rating stays in the evidence; the summary speaks
        // only Bigman's scale, so a row never carries two severities.
        if exploited.is_empty() {
            r.add_command(
                "advisory.update-available",
                Severity::Review,
                &format!("Security update for {}", list(&names)),
                evidence,
                "Install the update with this command, or from your desktop's software updater.",
                &a.command,
            );
        } else {
            r.add_command(
                "advisory.exploited",
                Severity::High,
                &format!("Security update for {} fixes a vulnerability known to be exploited", list(&names)),
                evidence,
                "Install this update before the others, with this command or your desktop's software updater.",
                &a.command,
            );
        }
    }
    if let Some(why) = &inputs.reboot {
        r.add(
            "advisory.reboot-required",
            Severity::Review,
            "Restart to finish installing updates",
            why.clone(),
            "Restart when convenient; until then the fixes in the newer kernel are not running.",
        );
    }
    if !inputs.untracked.is_empty() {
        r.add(
            "advisory.untracked-package",
            Severity::Info,
            &format!("{} not covered by your distribution's security data", if inputs.untracked.len() == 1 { "1 package is".to_string() } else { format!("{} packages are", inputs.untracked.len()) }),
            inputs.untracked.join("\n"),
            "These came from outside the distribution's repositories (for example the AUR or a local build). Follow their upstream for security updates.",
        );
    }
    if let Some(why) = &inputs.unsupported_release {
        r.add(
            "advisory.unsupported-release",
            Severity::Review,
            &format!(
                "{} no longer receives security updates",
                inputs.distro.pretty_or_id()
            ),
            why.clone(),
            "Upgrade to a supported release; updates found here may be incomplete.",
        );
    }
}

impl Distro {
    fn pretty_or_id(&self) -> String {
        if self.pretty.is_empty() {
            self.id.clone()
        } else {
            self.pretty.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_os_release() {
        let d = os_release("NAME=\"Fedora Linux\"\nID=fedora\nVERSION_ID=44\nPRETTY_NAME=\"Fedora Linux 44 (Workstation Edition)\"\n");
        assert_eq!(
            (d.id.as_str(), d.version_id.as_str(), d.family()),
            ("fedora", "44", Some(Family::Fedora))
        );
        let ubuntu = os_release("ID=ubuntu\nID_LIKE=debian\nVERSION_CODENAME=noble\n");
        assert_eq!(ubuntu.family(), None, "derivatives are not guessed at");
    }
}
