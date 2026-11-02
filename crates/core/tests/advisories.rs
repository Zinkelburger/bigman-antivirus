//! Milestone 6 exit test: fixture package sets on Fedora, Debian and Arch data
//! yield exactly the expected updates, with the right fixed versions.
use bigman_core::{
    checks::advisories::{assess, backends, install_all, os_release, Family, Inputs},
    model::{Report, Severity},
};
use std::{fs, path::Path};

fn data(name: &str) -> String {
    fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/advisories")
            .join(name),
    )
    .unwrap()
}

/// (rule, packages with installed → fixed) per finding, most severe first.
fn outcome(inputs: &Inputs) -> Vec<(String, Severity, String)> {
    let mut r = Report::new("Security updates", "fixture");
    assess(&mut r, inputs);
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
    r.findings
        .iter()
        .map(|f| {
            let versions: Vec<&str> = f
                .evidence
                .lines()
                .filter(|l| l.contains(": installed "))
                .collect();
            (f.rule.clone(), f.severity, versions.join("; "))
        })
        .collect()
}

fn kev() -> std::collections::BTreeSet<String> {
    backends::parse_kev(&data("kev.json")).unwrap()
}

#[test]
fn fedora_lists_what_dnf_updateinfo_fixes() {
    let installed = backends::parse_rpm_inventory(&data("fedora-rpm.txt"));
    let list: Vec<backends::DnfItem> = serde_json::from_str(&data("fedora-list.json")).unwrap();
    let info: Vec<backends::DnfInfo> = serde_json::from_str(&data("fedora-info.json")).unwrap();
    let applies = backends::fedora(&list, &info, &installed);
    let gcc = applies
        .iter()
        .find(|a| a.id == "FEDORA-2026-d732fb97a0")
        .unwrap();
    assert_eq!(gcc.cves, ["CVE-2026-102010", "CVE-2026-115455"]);
    assert_eq!(
        gcc.command,
        "sudo dnf upgrade --advisories=FEDORA-2026-d732fb97a0"
    );
    assert_eq!(
        install_all(Some(Family::Fedora), &applies).as_deref(),
        Some("sudo dnf upgrade --advisories=FEDORA-2026-31601794bf,FEDORA-2026-d732fb97a0,FEDORA-2026-77aa11bb22")
    );
    let inputs = Inputs {
        distro: os_release("ID=fedora\nPRETTY_NAME=\"Fedora Linux 44\"\n"),
        applies,
        kev: kev(),
        ..Default::default()
    };
    assert_eq!(
        outcome(&inputs),
        [
            (
                "advisory.update-available".into(),
                Severity::Review,
                "firefox: installed 0:158.0-2.fc44, fixed in 0:159.0-1.fc44; firefox-langpacks: installed 0:158.0-2.fc44, fixed in 0:159.0-1.fc44".into()
            ),
            (
                "advisory.update-available".into(),
                Severity::Review,
                "gcc: installed 0:16.2.1-2.fc44, fixed in 0:16.2.1-3.fc44; cpp: installed 0:16.2.1-2.fc44, fixed in 0:16.2.1-3.fc44".into()
            ),
            (
                "advisory.update-available".into(),
                Severity::Review,
                "vim-enhanced: installed 1:9.1.1-1.fc44, fixed in 2:9.1.1-1.fc44".into()
            ),
        ],
        "zlib-ng is already newer than its advisory"
    );
}

#[test]
fn debian_lists_resolved_cves_by_source_package() {
    let installed = backends::parse_dpkg_inventory(&data("debian-dpkg.txt"));
    let tracker =
        backends::parse_debian_tracker(data("debian-tracker.json").as_bytes(), "trixie").unwrap();
    let applies = backends::debian(&tracker, &installed, "trixie");
    let openssl = applies
        .iter()
        .find(|a| a.id.starts_with("openssl"))
        .unwrap();
    assert_eq!(
        openssl.cves,
        ["CVE-2026-1001"],
        "CVE-2026-1002 is fixed in the installed version already"
    );
    assert_eq!(
        openssl.command,
        "sudo apt update && sudo apt install --only-upgrade openssl libssl3t64"
    );
    let inputs = Inputs {
        distro: os_release("ID=debian\nVERSION_CODENAME=trixie\nPRETTY_NAME=\"Debian 13\"\n"),
        applies,
        kev: kev(),
        ..Default::default()
    };
    assert_eq!(
        outcome(&inputs),
        [
            ("advisory.exploited".into(), Severity::High, "openssl: installed 3.5.4-1~deb13u1, fixed in 3.5.4-1~deb13u2; libssl3t64: installed 3.5.4-1~deb13u1, fixed in 3.5.4-1~deb13u2".into()),
            ("advisory.no-fix-yet".into(), Severity::Info, "libtiff6: installed 4.7.0-3, no fix yet".into()),
        ],
        "xz-utils was never affected, curl is fixed, the unimportant libtiff CVE is hidden"
    );
}

#[test]
fn arch_matches_like_arch_audit() {
    let installed = backends::parse_pacman_inventory(&data("arch-pacman.txt"));
    let groups: Vec<backends::Avg> = serde_json::from_str(&data("arch-all.json")).unwrap();
    let inputs = Inputs {
        distro: os_release("ID=arch\nPRETTY_NAME=\"Arch Linux\"\n"),
        applies: backends::arch(&groups, &installed),
        kev: kev(),
        ..Default::default()
    };
    assert_eq!(
        outcome(&inputs),
        [
            (
                "advisory.exploited".into(),
                Severity::High,
                "openssh: installed 10.0p1-1, fixed in 10.0p1-2".into()
            ),
            (
                "advisory.no-fix-yet".into(),
                Severity::Info,
                "libtiff: installed 4.7.0-2, no fix yet".into()
            ),
        ],
        "vim is already at the fixed version and curl is not affected"
    );
}
