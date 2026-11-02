//! Per-distribution advisory data: parsers and matchers (pure, fixture-tested)
//! and the gathering that runs the package tools and fetches public data.
use super::{
    version::{compare, Scheme},
    Affected, Applies, Installed,
};
use serde::{de, Deserialize};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt,
};

fn cve_ids(text: &str) -> Vec<String> {
    let re = regex::Regex::new(r"CVE-\d{4}-\d{4,}").expect("valid regex");
    re.find_iter(text).map(|m| m.as_str().to_string()).collect()
}

fn by_name(installed: &[Installed]) -> BTreeMap<&str, &Installed> {
    installed.iter().map(|i| (i.name.as_str(), i)).collect()
}

// ---- Fedora: dnf's updateinfo, through `dnf advisory` ----

#[derive(Debug, Deserialize)]
pub struct DnfItem {
    pub name: String,
    pub severity: String,
    pub nevra: String,
}

#[derive(Debug, Deserialize)]
pub struct DnfInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(default)]
    pub references: Vec<DnfReference>,
}

#[derive(Debug, Deserialize)]
pub struct DnfReference {
    #[serde(rename = "Title", default)]
    pub title: String,
    #[serde(rename = "Id", default)]
    pub id: String,
}

/// `name-[epoch:]version-release.arch` → (name, `epoch:version-release`).
pub fn split_nevra(nevra: &str) -> Option<(String, String)> {
    let (rest, _arch) = nevra.rsplit_once('.')?;
    let mut parts = rest.rsplitn(3, '-');
    let (release, version, name) = (parts.next()?, parts.next()?, parts.next()?);
    let evr = if version.contains(':') {
        format!("{version}-{release}")
    } else {
        format!("0:{version}-{release}")
    };
    Some((name.to_string(), evr))
}

/// Matches `dnf advisory list --security --updates --json` against the rpm
/// inventory; CVE ids come from `dnf advisory info --json` references.
pub fn fedora(list: &[DnfItem], info: &[DnfInfo], installed: &[Installed]) -> Vec<Applies> {
    let installed = by_name(installed);
    let cves: BTreeMap<&str, Vec<String>> = info
        .iter()
        .map(|i| {
            let mut ids: BTreeSet<String> = BTreeSet::new();
            for r in &i.references {
                ids.extend(cve_ids(&r.title));
                ids.extend(cve_ids(&r.id));
            }
            (i.name.as_str(), ids.into_iter().collect())
        })
        .collect();
    let mut advisories: BTreeMap<&str, Applies> = BTreeMap::new();
    for item in list {
        let Some((name, fixed)) = split_nevra(&item.nevra) else {
            continue;
        };
        let Some(have) = installed.get(name.as_str()) else {
            continue;
        };
        if compare(Scheme::Rpm, &have.version, &fixed) != Ordering::Less {
            continue;
        }
        let entry = advisories.entry(&item.name).or_insert_with(|| Applies {
            id: item.name.clone(),
            packages: vec![],
            cves: cves.get(item.name.as_str()).cloned().unwrap_or_default(),
            severity: item.severity.clone(),
            command: format!("sudo dnf upgrade --advisories={}", item.name),
            source: "dnf updateinfo (Fedora repository metadata)".into(),
        });
        if !entry.packages.iter().any(|p| p.name == name) {
            entry.packages.push(Affected {
                name,
                installed: have.version.clone(),
                fixed: Some(fixed),
            });
        }
    }
    // The distribution's own rating orders updates of equal Bigman severity.
    let rank = |s: &str| {
        ["critical", "important", "moderate", "low"]
            .iter()
            .position(|r| s.eq_ignore_ascii_case(r))
            .unwrap_or(4)
    };
    let mut out: Vec<Applies> = advisories.into_values().collect();
    out.sort_by_key(|a| (rank(&a.severity), a.id.clone()));
    out
}

/// `rpm -qa --qf '%{NAME}\t%{EPOCHNUM}:%{VERSION}-%{RELEASE}\n'`.
pub fn parse_rpm_inventory(out: &str) -> Vec<Installed> {
    out.lines()
        .filter_map(|l| l.split_once('\t'))
        .filter(|(name, _)| *name != "gpg-pubkey")
        .map(|(name, version)| Installed {
            name: name.into(),
            version: version.into(),
            source: None,
        })
        .collect()
}

/// `dnf list --extras`: `name.arch  version  origin` after a heading line.
pub fn parse_dnf_extras(out: &str) -> Vec<String> {
    out.lines()
        .filter_map(|l| {
            let mut f = l.split_whitespace();
            let (name_arch, version, origin) = (f.next()?, f.next()?, f.next()?);
            let (name, _) = name_arch.rsplit_once('.')?;
            Some(format!("{name} {version} (from {origin})"))
        })
        .collect()
}

// ---- Debian: the security tracker's JSON ----

/// One CVE's entry for one release in the tracker.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DebianRelease {
    pub status: String,
    #[serde(default)]
    pub fixed_version: Option<String>,
    #[serde(default)]
    pub urgency: String,
    #[serde(default)]
    pub nodsa_reason: Option<String>,
}

/// source package → CVE → that release's entry. Only `release` is kept while
/// parsing, so the 87 MB file never sits in memory whole.
pub type DebianData = BTreeMap<String, BTreeMap<String, DebianRelease>>;

pub fn parse_debian_tracker(
    reader: impl std::io::Read,
    release: &str,
) -> serde_json::Result<DebianData> {
    struct Sources<'a>(&'a str);
    impl<'de> de::DeserializeSeed<'de> for Sources<'_> {
        type Value = DebianData;
        fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_map(self)
        }
    }
    impl<'de> de::Visitor<'de> for Sources<'_> {
        type Value = DebianData;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("source packages")
        }
        fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut out = DebianData::new();
            while let Some(source) = map.next_key::<String>()? {
                let cves = map.next_value_seed(Cves(self.0))?;
                if !cves.is_empty() {
                    out.insert(source, cves);
                }
            }
            Ok(out)
        }
    }
    struct Cves<'a>(&'a str);
    impl<'de> de::DeserializeSeed<'de> for Cves<'_> {
        type Value = BTreeMap<String, DebianRelease>;
        fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_map(self)
        }
    }
    impl<'de> de::Visitor<'de> for Cves<'_> {
        type Value = BTreeMap<String, DebianRelease>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("CVEs")
        }
        fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            #[derive(Deserialize)]
            struct Cve {
                #[serde(default)]
                releases: BTreeMap<String, DebianRelease>,
            }
            let mut out = BTreeMap::new();
            while let Some(cve) = map.next_key::<String>()? {
                let mut entry: Cve = map.next_value()?;
                if let Some(r) = entry.releases.remove(self.0) {
                    out.insert(cve, r);
                }
            }
            Ok(out)
        }
    }
    let mut d = serde_json::Deserializer::from_reader(reader);
    de::DeserializeSeed::deserialize(Sources(release), &mut d)
}

/// `dpkg-query -W -f '${binary:Package}\t${Version}\t${source:Package}\t${source:Version}\t${db:Status-Abbrev}\n'`.
pub fn parse_dpkg_inventory(out: &str) -> Vec<Installed> {
    out.lines()
        .filter_map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            let [binary, version, source, source_version, status] = f.as_slice() else {
                return None;
            };
            status.starts_with("ii").then(|| Installed {
                name: binary.split(':').next().unwrap_or(binary).to_string(),
                version: (*version).into(),
                source: Some(((*source).into(), (*source_version).into())),
            })
        })
        .collect()
}

/// Resolved CVEs with a newer fixed version become one update per source
/// package; open ones that matter become "no fix yet".
pub fn debian(data: &DebianData, installed: &[Installed], release: &str) -> Vec<Applies> {
    let mut sources: BTreeMap<&str, (&str, Vec<&Installed>)> = BTreeMap::new();
    for i in installed {
        if let Some((source, version)) = &i.source {
            sources
                .entry(source.as_str())
                .or_insert((version.as_str(), vec![]))
                .1
                .push(i);
        }
    }
    let mut out = vec![];
    for (source, (version, binaries)) in sources {
        let Some(cves) = data.get(source) else {
            continue;
        };
        let mut fixed: Vec<(&str, &str)> = vec![];
        let mut open: Vec<&str> = vec![];
        let mut urgencies = BTreeSet::new();
        for (cve, r) in cves {
            match (r.status.as_str(), r.fixed_version.as_deref()) {
                ("resolved", Some(f))
                    if f != "0" && compare(Scheme::Dpkg, version, f) == Ordering::Less =>
                {
                    fixed.push((cve, f));
                    urgencies.insert(r.urgency.clone());
                }
                ("open", _)
                    if r.urgency != "unimportant"
                        && !matches!(r.nodsa_reason.as_deref(), Some("ignored")) =>
                {
                    open.push(cve)
                }
                _ => {}
            }
        }
        let names: Vec<String> = binaries.iter().map(|b| b.name.clone()).collect();
        let worst = ["high", "medium", "low", "unimportant", "not yet assigned"]
            .into_iter()
            .find(|u| urgencies.contains(*u))
            .unwrap_or("not yet assigned");
        if let Some(target) = fixed
            .iter()
            .map(|(_, f)| *f)
            .max_by(|a, b| compare(Scheme::Dpkg, a, b))
        {
            out.push(Applies {
                id: format!("{source} (Debian security tracker)"),
                packages: binaries
                    .iter()
                    .map(|b| Affected {
                        name: b.name.clone(),
                        installed: version.into(),
                        fixed: Some(target.into()),
                    })
                    .collect(),
                cves: fixed.iter().map(|(c, _)| c.to_string()).collect(),
                severity: format!("Debian urgency: {worst}"),
                command: format!(
                    "sudo apt update && sudo apt install --only-upgrade {}",
                    names.join(" ")
                ),
                source: format!("Debian security tracker, {release}"),
            });
        }
        if !open.is_empty() {
            out.push(Applies {
                id: format!("{source} (Debian security tracker)"),
                packages: binaries
                    .iter()
                    .map(|b| Affected {
                        name: b.name.clone(),
                        installed: version.into(),
                        fixed: None,
                    })
                    .collect(),
                cves: open.iter().map(|c| c.to_string()).collect(),
                severity: "open".into(),
                command: String::new(),
                source: format!("Debian security tracker, {release}"),
            });
        }
    }
    out
}

// ---- Arch: security.archlinux.org ----

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Avg {
    pub name: String,
    pub packages: Vec<String>,
    pub status: String,
    pub severity: String,
    pub affected: String,
    pub fixed: Option<String>,
    #[serde(default)]
    pub issues: Vec<String>,
}

/// `pacman -Q`: `name version`.
pub fn parse_pacman_inventory(out: &str) -> Vec<Installed> {
    out.lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(name, version)| Installed {
            name: name.into(),
            version: version.trim().into(),
            source: None,
        })
        .collect()
}

/// arch-audit's matching: fixed when installed is older than `fixed`; still
/// vulnerable with no fix when the group is open and installed is at least `affected`.
pub fn arch(groups: &[Avg], installed: &[Installed]) -> Vec<Applies> {
    let installed = by_name(installed);
    let mut out = vec![];
    for g in groups.iter().filter(|g| g.status != "Not affected") {
        let packages: Vec<Affected> = g
            .packages
            .iter()
            .filter_map(|p| installed.get(p.as_str()).map(|i| (p, *i)))
            .filter_map(|(p, i)| match &g.fixed {
                Some(f) if compare(Scheme::Pacman, &i.version, f) == Ordering::Less => {
                    Some(Affected {
                        name: p.clone(),
                        installed: i.version.clone(),
                        fixed: Some(f.clone()),
                    })
                }
                None if compare(Scheme::Pacman, &i.version, &g.affected) != Ordering::Less => {
                    Some(Affected {
                        name: p.clone(),
                        installed: i.version.clone(),
                        fixed: None,
                    })
                }
                _ => None,
            })
            .collect();
        if packages.is_empty() {
            continue;
        }
        out.push(Applies {
            id: g.name.clone(),
            packages,
            cves: g.issues.clone(),
            severity: g.severity.clone(),
            command: if g.fixed.is_some() {
                "sudo pacman -Syu".into()
            } else {
                String::new()
            },
            source: "Arch Linux security tracker".into(),
        });
    }
    out
}

// ---- CISA KEV ----

pub fn parse_kev(json: &str) -> serde_json::Result<BTreeSet<String>> {
    #[derive(Deserialize)]
    struct Catalog {
        vulnerabilities: Vec<Entry>,
    }
    #[derive(Deserialize)]
    struct Entry {
        #[serde(rename = "cveID")]
        cve: String,
    }
    Ok(serde_json::from_str::<Catalog>(json)?
        .vulnerabilities
        .into_iter()
        .map(|e| e.cve)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_fedora_nevras() {
        assert_eq!(
            split_nevra("firefox-159.0-1.fc44.x86_64"),
            Some(("firefox".into(), "0:159.0-1.fc44".into()))
        );
        assert_eq!(
            split_nevra("gcc-c++-16.2.1-3.fc44.x86_64"),
            Some(("gcc-c++".into(), "0:16.2.1-3.fc44".into()))
        );
        assert_eq!(
            split_nevra("vim-enhanced-2:9.1.1-1.fc44.x86_64"),
            Some(("vim-enhanced".into(), "2:9.1.1-1.fc44".into()))
        );
    }

    #[test]
    fn parses_only_the_wanted_debian_release() {
        let json = r#"{"openssl": {"CVE-2026-1": {"description": "x", "releases": {
            "bookworm": {"status": "resolved", "fixed_version": "3.0.20-1~deb12u2", "urgency": "medium", "repositories": {"bookworm": "3.0.20-1~deb12u2"}},
            "trixie": {"status": "resolved", "fixed_version": "3.5.4-1", "urgency": "medium"}}}},
            "xz-utils": {"CVE-2024-3094": {"releases": {"trixie": {"status": "resolved", "fixed_version": "0", "urgency": "unimportant"}}}}}"#;
        let data = parse_debian_tracker(json.as_bytes(), "bookworm").unwrap();
        assert_eq!(data.len(), 1, "xz-utils has no bookworm entry");
        assert_eq!(
            data["openssl"]["CVE-2026-1"].fixed_version.as_deref(),
            Some("3.0.20-1~deb12u2")
        );
    }

    #[test]
    fn reads_inventories() {
        assert_eq!(
            parse_rpm_inventory("gpg-pubkey\t0:abc-1\nfirefox\t0:158.0-2.fc44\n").len(),
            1
        );
        let deb = parse_dpkg_inventory("libssl3:amd64\t3.0.20-1~deb12u1\topenssl\t3.0.20-1~deb12u1\tii \nold\t1\told\t1\trc \n");
        assert_eq!(deb[0].name, "libssl3");
        assert_eq!(
            deb[0].source,
            Some(("openssl".into(), "3.0.20-1~deb12u1".into()))
        );
        assert_eq!(deb.len(), 1, "removed packages are skipped");
        assert_eq!(
            parse_dnf_extras("Extra packages\nchatgpt.x86_64   26.1-1   openai-chatgpt\n"),
            ["chatgpt 26.1-1 (from openai-chatgpt)"]
        );
    }
}
