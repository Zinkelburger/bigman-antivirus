//! Rules, fixtures and the rules document stay in step (plan Part H):
//! - every registered rule has `fixtures/<rule-id>/expected.json`;
//! - each fixture's input fires exactly the rules it lists, and no others;
//! - every rule ID written in core's source is registered, and every registered
//!   rule is used somewhere;
//! - `docs/rules.md` matches the registry.
use bigman_core::{
    checks::{self, receipt},
    platform::{Attribution, PackageDb, PackageScript, Transaction},
};
use bigman_core::{
    checks::{
        canary, changes, denials, doctor, inspect, integrations, network, packages, pdf, scan,
        shell,
    },
    model::Report,
    platform::{linux::parse_proc_net, Protocol},
    rules, runner,
    store::Store,
};
use serde::Deserialize;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[derive(Deserialize)]
struct Expected {
    driver: String,
    fires: Vec<String>,
    input: Option<String>,
    /// The older file a diff-mode fixture compares with.
    #[serde(default)]
    before: Option<String>,
    covered_by: Option<String>,
}

fn rules_fired(report: &Report) -> BTreeSet<String> {
    report.findings.iter().map(|f| f.rule.clone()).collect()
}

/// A package database answering from a fixture: files and scripts per package.
#[derive(Deserialize)]
struct FixtureDb {
    #[serde(default)]
    files: std::collections::BTreeMap<String, Vec<PathBuf>>,
    #[serde(default)]
    scripts: std::collections::BTreeMap<String, Vec<PackageScript>>,
}

impl PackageDb for FixtureDb {
    fn manager(&self) -> &'static str {
        "fixture"
    }
    fn attribute(
        &self,
        _: &[PathBuf],
        _: &runner::Cancel,
    ) -> anyhow::Result<std::collections::BTreeMap<PathBuf, Attribution>> {
        Ok(Default::default())
    }
    fn files(&self, package: &str, _: &runner::Cancel) -> anyhow::Result<Vec<PathBuf>> {
        Ok(self.files.get(package).cloned().unwrap_or_default())
    }
    fn scripts(&self, package: &str, _: &runner::Cancel) -> anyhow::Result<Vec<PackageScript>> {
        Ok(self.scripts.get(package).cloned().unwrap_or_default())
    }
}

/// A receipt fixture: the transaction, what the database holds, earlier hook
/// digests, files to create setuid, and what started listening afterwards.
#[derive(Deserialize)]
struct ReceiptFixture {
    transaction: Transaction,
    #[serde(flatten)]
    db: FixtureDb,
    #[serde(default)]
    previous_hooks: std::collections::BTreeMap<String, String>,
    #[serde(default)]
    setuid: Vec<String>,
    #[serde(default)]
    after: Option<Vec<String>>,
}

fn receipt_rules(input: &Path) -> BTreeSet<String> {
    let mut f: ReceiptFixture = serde_json::from_str(&fs::read_to_string(input).unwrap()).unwrap();
    // setuid bits do not survive git; create those files here.
    let dir = tempfile::tempdir().unwrap();
    for (name, files) in f.db.files.iter_mut() {
        for file in f.setuid.iter().filter(|s| *s == name) {
            let path = dir.path().join(file);
            fs::write(&path, "").unwrap();
            fs::set_permissions(&path, std::os::unix::fs::PermissionsExt::from_mode(0o4755))
                .unwrap();
            files.push(path);
        }
    }
    let hooks = f.previous_hooks;
    let lookup =
        move |package: &str, phase: &str| hooks.get(&format!("{package}/{phase}")).cloned();
    let mut r = receipt::build(
        "fixture".into(),
        &f.transaction,
        &f.db,
        &receipt::History { hook: &lookup },
        &runner::cancel(),
    );
    if let Some(after) = f.after {
        receipt::attach_after(&mut r, after);
    }
    r.findings.iter().map(|f| f.rule.clone()).collect()
}

/// Runs one fixture through the check its driver names and returns what fired.
fn run(dir: &Path, expected: &Expected) -> BTreeSet<String> {
    let input = dir.join(expected.input.as_deref().expect("fixture input"));
    let text = || fs::read_to_string(&input).unwrap();
    let json = || serde_json::from_str::<serde_json::Value>(&text()).unwrap();
    let state = tempfile::tempdir().unwrap();
    let store = Store::open(Some(state.path().join("state"))).unwrap();
    let mut r = Report::new("Fixture", dir.display());
    match expected.driver.as_str() {
        "shell" => return rules_fired(&shell::analyze_content(&text())),
        "packages" => return rules_fired(&packages::packages(&store, &input).unwrap()),
        "pdf" => return rules_fired(&pdf::scan_pdf(&store, &input, &runner::cancel()).unwrap()),
        "trace" => inspect::trace_findings(&mut r, &text()),
        "shellcheck" => inspect::shellcheck_findings(&mut r, &text()).unwrap(),
        "clamav" => {
            let snapshot = PathBuf::from("/fixture/snapshot-0");
            let mut items = vec![scan::ScanItem {
                path: "/home/user/Downloads/sample.bin".into(),
                hash: "0".repeat(64),
                detected: false,
                signature: String::new(),
            }];
            let output = text().replace("{snapshot}", &snapshot.to_string_lossy());
            scan::record_matches(&mut r, &mut items, &[snapshot], &output);
        }
        "doctor" => doctor::assess(&mut r, &serde_json::from_value(json()).unwrap()),
        "changes" => {
            let value = json();
            let old: Option<changes::Baseline> =
                serde_json::from_value(value["old"].clone()).unwrap();
            let current: changes::Baseline =
                serde_json::from_value(value["current"].clone()).unwrap();
            let attribution =
                serde_json::from_value(value["attribution"].clone()).unwrap_or_default();
            changes::compare(&mut r, old.as_ref(), &current, &attribution);
        }
        "canary" => {
            let mask = json()["mask"]
                .as_array()
                .unwrap()
                .iter()
                .map(|m| match m.as_str().unwrap() {
                    "IN_ACCESS" => libc::IN_ACCESS,
                    "IN_MODIFY" => libc::IN_MODIFY,
                    "IN_ATTRIB" => libc::IN_ATTRIB,
                    "IN_DELETE" => libc::IN_DELETE,
                    other => panic!("unknown inotify mask {other}"),
                })
                .fold(0, |a, b| a | b);
            canary::event_finding(&mut r, "fixture", &canary::event_kinds(mask));
        }
        "listener" => {
            for (socket, _) in parse_proc_net(&text(), Protocol::Tcp) {
                network::listener_findings(&mut r, &socket);
            }
        }
        "receipt" => return receipt_rules(&input),
        "bin-links" => {
            // node_modules is not committed, so the project is built here.
            #[derive(Deserialize)]
            struct Bin {
                link: Option<String>,
                file: Option<String>,
            }
            #[derive(Deserialize)]
            struct Project {
                pnpm: bool,
                files: std::collections::BTreeMap<String, String>,
                bin: std::collections::BTreeMap<String, Bin>,
            }
            let p: Project = serde_json::from_value(json()).unwrap();
            let project = tempfile::tempdir().unwrap();
            let root = project.path();
            fs::write(
                root.join("package.json"),
                r#"{"name":"fixture","version":"1.0.0"}"#,
            )
            .unwrap();
            fs::create_dir_all(root.join("node_modules/.bin")).unwrap();
            if p.pnpm {
                fs::create_dir_all(root.join("node_modules/.pnpm")).unwrap();
            }
            for (path, text) in &p.files {
                let path = root.join(path);
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                fs::write(path, text).unwrap();
            }
            for (name, bin) in &p.bin {
                let path = root.join("node_modules/.bin").join(name);
                match (&bin.link, &bin.file) {
                    (Some(target), _) => std::os::unix::fs::symlink(target, &path).unwrap(),
                    (_, Some(text)) => fs::write(&path, text).unwrap(),
                    _ => panic!("bin entry {name} needs a link or a file"),
                }
            }
            return rules_fired(&packages::inventory(root).unwrap());
        }
        "lockfile" => {
            let before = expected.before.as_ref().map(|b| dir.join(b));
            return rules_fired(&checks::lockfile::lockfile(&input, before.as_deref()).unwrap());
        }
        "repo" => {
            #[derive(Deserialize)]
            struct Repo {
                files: std::collections::BTreeMap<PathBuf, String>,
                git_config: Option<String>,
            }
            let repo: Repo = serde_json::from_value(json()).unwrap();
            let files: Vec<checks::repo::File> = repo
                .files
                .into_iter()
                .map(|(path, text)| checks::repo::File { path, text })
                .collect();
            checks::repo::assess(&mut r, &files, repo.git_config.as_deref());
        }
        "channels" => {
            let value = json();
            let now = value["now"].as_u64().unwrap_or(0);
            checks::channels::assess(&mut r, &serde_json::from_value(value).unwrap(), now)
        }
        "archive" => checks::archive::assess(&mut r, &serde_json::from_value(json()).unwrap()),
        "langpkg" => {
            #[derive(Deserialize)]
            struct Fixture {
                ecosystem: String,
                entries: Vec<checks::archive::Entry>,
            }
            let f: Fixture = serde_json::from_value(json()).unwrap();
            let eco = match f.ecosystem.as_str() {
                "wheel" => checks::langpkg::Ecosystem::Wheel,
                "sdist" => checks::langpkg::Ecosystem::Sdist,
                "crate" => checks::langpkg::Ecosystem::Crate,
                "vsix" => checks::langpkg::Ecosystem::Vsix,
                other => panic!("unknown ecosystem {other}"),
            };
            let listing = checks::archive::Listing {
                compressed: 1,
                entries: f.entries,
                truncated: None,
            };
            checks::langpkg::assess(&mut r, eco, &listing);
        }
        "package" => checks::pkgfile::assess(&mut r, &serde_json::from_value(json()).unwrap()),
        "pkgbuild" => {
            let text = text();
            let install = fs::read_dir(dir)
                .unwrap()
                .map(|e| e.unwrap().path())
                .find(|p| p.extension().is_some_and(|e| e == "install"));
            let script = install.as_ref().map(|p| fs::read_to_string(p).unwrap());
            let name = install
                .as_ref()
                .map(|p| p.file_name().unwrap().to_string_lossy().into_owned());
            checks::pkgfile::assess_pkgbuild(&mut r, &text, name.as_deref().zip(script.as_deref()));
        }
        "appimage" => {
            let sections: std::collections::BTreeMap<String, String> =
                serde_json::from_value(json()).unwrap();
            let sections: Vec<(String, Vec<u8>)> = sections
                .into_iter()
                .map(|(k, v)| (k, v.into_bytes()))
                .collect();
            checks::pkgfile::assess_appimage(&mut r, &sections);
        }
        "desktop" => {
            let name = input.file_name().unwrap().to_string_lossy();
            checks::desktop::assess(&mut r, &input.display().to_string(), &name, &text(), false);
        }
        "advisories" => {
            checks::advisories::assess(&mut r, &serde_json::from_value(json()).unwrap())
        }
        "denial" => {
            for line in text().lines() {
                denials::finding(&mut r, &denials::parse(line).expect("a denial record"));
            }
        }
        "lynis" => integrations::lynis_findings(&mut r, &text()),
        "osv" => assert!(integrations::osv_findings(&mut r, &text())),
        "yara" => integrations::yara_findings(&mut r, &text()),
        other => panic!("{}: unknown driver {other}", dir.display()),
    }
    rules_fired(&r)
}

#[test]
fn every_rule_has_a_fixture_that_fires_exactly_its_rules() {
    let mut problems = vec![];
    for rule in rules::RULES {
        let dir = repo().join("fixtures").join(rule.id);
        let Ok(raw) = fs::read_to_string(dir.join("expected.json")) else {
            problems.push(format!(
                "{}: no fixtures/{}/expected.json",
                rule.id, rule.id
            ));
            continue;
        };
        let expected: Expected = serde_json::from_str(&raw).unwrap();
        if expected.driver == "integration" {
            // Rules that need a real monitor are proved by a named integration test.
            let test = expected.covered_by.as_deref().expect("covered_by");
            let source = fs::read_to_string(repo().join(test)).unwrap_or_default();
            if !source.contains(rule.id) {
                problems.push(format!("{}: {test} does not assert this rule", rule.id));
            }
            continue;
        }
        let fired = run(&dir, &expected);
        let wanted: BTreeSet<String> = expected.fires.iter().cloned().collect();
        if !wanted.contains(rule.id) {
            problems.push(format!(
                "{}: its fixture does not list the rule itself",
                rule.id
            ));
        }
        if fired != wanted {
            problems.push(format!("{}: fired {fired:?}, expected {wanted:?}", rule.id));
        }
    }
    assert!(problems.is_empty(), "\n{}", problems.join("\n"));
}

#[test]
fn denials_read_as_plain_sentences() {
    let selinux = fs::read_to_string(repo().join("fixtures/denial.selinux/input.txt")).unwrap();
    let d = denials::parse(selinux.trim()).unwrap();
    assert_eq!(
        denials::sentence(&d),
        "firefox, confined by the mozilla policy, tried to read .ssh (dir) and was stopped."
    );
    let apparmor = fs::read_to_string(repo().join("fixtures/denial.apparmor/input.txt")).unwrap();
    let d = denials::parse(apparmor.trim()).unwrap();
    assert_eq!(
        denials::sentence(&d),
        "firefox tried to read /home/user/.ssh/id_ed25519 and was stopped."
    );
    // Real records from a Fedora machine running the NetBird VPN.
    let proc_read = "AVC avc:  denied  { read } for  pid=1713531 comm=\"ps\" name=\"1\" dev=\"proc\" ino=12289 scontext=system_u:system_r:netbird_t:s0 tcontext=system_u:system_r:init_t:s0 tclass=dir permissive=0";
    let d = denials::parse(proc_read).unwrap();
    assert_eq!(
        denials::sentence(&d),
        "ps, confined by the netbird policy, tried to read other processes' details in /proc and was stopped."
    );
    assert_eq!(
        d.key, "denial:selinux:ps:read:dir:proc",
        "every pid folds into one entry"
    );
    let dbus = "USER_AVC pid=1407 uid=81 msg='avc:  denied  { send_msg } for  scontext=system_u:system_r:netbird_t:s0 tcontext=system_u:system_r:firewalld_t:s0 tclass=dbus permissive=0 exe=\"/usr/bin/dbus-broker\"'";
    assert_eq!(
        denials::sentence(&denials::parse(dbus).unwrap()),
        "A program confined by the netbird policy tried to message firewalld over D-Bus and was stopped."
    );
    let noise = "avc:  denied  { signull } for  pid=1130 comm=\"systemd-oomd\" scontext=system_u:system_r:systemd_oomd_t:s0 tcontext=system_u:system_r:unconfined_service_t:s0 tclass=process permissive=1";
    let d = denials::parse(noise).unwrap();
    assert!(d.allowed && denials::known_noise(&d));
    assert!(denials::sentence(&d).ends_with("This one is known and harmless."));
}

#[test]
fn every_fixture_belongs_to_a_registered_rule() {
    let stray: Vec<String> = fs::read_dir(repo().join("fixtures"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| rules::get(name).is_none())
        .collect();
    assert!(stray.is_empty(), "fixtures without a rule: {stray:?}");
}

fn source_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            source_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") && !path.ends_with("rules/mod.rs") {
            out.push(path);
        }
    }
}

#[test]
fn rule_ids_in_source_and_registry_match() {
    let families: BTreeSet<&str> = rules::RULES
        .iter()
        .map(|r| r.id.split('.').next().unwrap())
        .collect();
    let literal = regex::Regex::new(r#""([a-z]+(?:\.[a-z][a-z0-9_-]*)+)""#).unwrap();
    let mut files = vec![];
    source_files(&repo().join("crates/core/src"), &mut files);
    let mut used = BTreeSet::new();
    for file in files {
        for cap in literal.captures_iter(&fs::read_to_string(&file).unwrap()) {
            let id = cap[1].to_string();
            // File and host names that share a family's first word.
            let file_name = [
                ".json", ".lock", ".log", ".dat", ".sh", ".py", ".desktop", ".com", ".org",
            ]
            .iter()
            .any(|s| id.ends_with(s));
            if !file_name && families.contains(id.split('.').next().unwrap()) {
                used.insert(id);
            }
        }
    }
    let registered: BTreeSet<String> = rules::RULES.iter().map(|r| r.id.to_string()).collect();
    let unregistered: Vec<_> = used.difference(&registered).collect();
    let unused: Vec<_> = registered.difference(&used).collect();
    assert!(
        unregistered.is_empty(),
        "rule IDs used but not registered: {unregistered:?}"
    );
    assert!(
        unused.is_empty(),
        "registered rules never produced: {unused:?}"
    );
    let ids: BTreeSet<_> = rules::RULES.iter().map(|r| r.id).collect();
    assert_eq!(ids.len(), rules::RULES.len(), "duplicate rule IDs");
}

#[test]
fn ui_rules_snapshot_is_current() {
    // Browser dev mode has no daemon; its mock serves this copy of the registry.
    let path = repo().join("crates/app/ui/src/ipc/generated/rules.json");
    let generated = serde_json::to_string_pretty(&rules::infos()).unwrap() + "\n";
    if std::env::var_os("BIGMAN_UPDATE_RULES_DOC").is_some() {
        fs::write(&path, &generated).unwrap();
    }
    assert_eq!(fs::read_to_string(&path).unwrap_or_default(), generated, "rules.json is stale; run BIGMAN_UPDATE_RULES_DOC=1 cargo test -p bigman-core --test fixtures");
}

#[test]
fn rules_document_is_current() {
    let path = repo().join("docs/rules.md");
    let generated = rules::markdown();
    if std::env::var_os("BIGMAN_UPDATE_RULES_DOC").is_some() {
        fs::write(&path, &generated).unwrap();
    }
    assert_eq!(
        fs::read_to_string(&path).unwrap_or_default(),
        generated,
        "docs/rules.md is stale; run BIGMAN_UPDATE_RULES_DOC=1 cargo test -p bigman-core --test fixtures"
    );
}
