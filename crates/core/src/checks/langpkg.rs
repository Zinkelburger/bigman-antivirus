//! Language packages beyond npm (SC-03, SC-04, SC-19): Python wheels and
//! sdists, Rust crates and VS Code extensions, read from their archive listing
//! or a source folder. Nothing is built, installed or imported.
use crate::{
    checks::archive::{Entry, Group, Kind, Listing},
    model::{Report, Severity},
};
use regex::Regex;
use serde_json::Value;
use std::{path::Path, sync::LazyLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ecosystem {
    Wheel,
    Sdist,
    Crate,
    Vsix,
}

impl Ecosystem {
    pub fn label(self) -> &'static str {
        match self {
            Ecosystem::Wheel => "Python wheel",
            Ecosystem::Sdist => "Python source package",
            Ecosystem::Crate => "Rust crate",
            Ecosystem::Vsix => "VS Code extension",
        }
    }
}

fn depth(name: &str) -> usize {
    name.trim_end_matches('/').matches('/').count()
}

fn base(name: &str) -> &str {
    name.rsplit('/').next().unwrap_or(name)
}

/// The ecosystem an archive belongs to, from its entry names.
pub fn detect(file_name: &str, names: &[&str]) -> Option<Ecosystem> {
    let at_top = |file: &str| names.iter().any(|n| depth(n) == 1 && base(n) == file);
    if names
        .iter()
        .any(|n| *n == "extension.vsixmanifest" || *n == "extension/package.json")
    {
        Some(Ecosystem::Vsix)
    } else if names.iter().any(|n| {
        depth(n) == 1
            && n.split('/')
                .next()
                .is_some_and(|d| d.ends_with(".dist-info"))
            && base(n) == "WHEEL"
    }) {
        Some(Ecosystem::Wheel)
    } else if file_name.ends_with(".crate") || (at_top("Cargo.toml") && at_top("Cargo.toml.orig")) {
        Some(Ecosystem::Crate)
    } else if at_top("PKG-INFO") && (at_top("setup.py") || at_top("pyproject.toml")) {
        Some(Ecosystem::Sdist)
    } else {
        None
    }
}

/// Entries whose text the rules read.
pub fn wanted(name: &str) -> bool {
    let b = base(name);
    b.ends_with(".pth")
        || matches!(
            b,
            "sitecustomize.py"
                | "usercustomize.py"
                | "entry_points.txt"
                | "setup.py"
                | "pyproject.toml"
                | "Cargo.toml"
                | "build.rs"
                | "PKG-INFO"
        )
        || (b == "METADATA" && name.contains(".dist-info/"))
        || name == "extension/package.json"
        || (name.starts_with("extension/")
            && !name.contains("/node_modules/")
            && [".js", ".cjs", ".mjs"].iter().any(|e| b.ends_with(e)))
}

/// Commands a console script could take the place of on PATH.
const COMMON: &[&str] = &[
    "python", "python3", "pip", "pip3", "git", "ssh", "sudo", "su", "ls", "cd", "cat", "curl",
    "wget", "npm", "node", "npx", "bash", "sh", "cargo", "make", "gpg", "docker", "kubectl",
];

static SETUP_CALLS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(subprocess|os\.system|os\.popen|os\.exec\w*|urllib|urlopen|requests\.|http\.client|socket\.|base64|b64decode|exec\(|eval\(|__import__|marshal\.loads|zlib\.decompress)").expect("valid regex")
});
/// `build = "path"` or `build = false` in a Cargo.toml.
static BUILD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)^\s*build\s*=\s*(?:"([^"]+)"|(false|true))"#).expect("valid regex")
});
/// What lifts a .pth import line to High. Plain `__import__` and `importlib`
/// are how setuptools' own startup lines adjust imports.
static PTH_REACH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(subprocess|os\.system|os\.popen|os\.exec\w*|urllib|urlopen|requests\.|http\.client|socket\.|base64|b64decode|exec\(|eval\(|compile\(|marshal\.loads|zlib\.decompress|codecs\.decode)").expect("valid regex")
});
/// What lifts a build script to High: network access, or your home folder and
/// credentials. Build scripts legitimately start rustc, git, make and the like.
static BUILD_REACH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(TcpStream|UdpSocket|std::net::|\breqwest::|\bureq::|\bhyper::|\battohttpc::|\bcurl::|Command::new\("(curl|wget)"\)|env::var(_os)?\("HOME"\)|home_dir\(|\.ssh/|\.aws/|\.gnupg|\.npmrc|\.cargo/credentials)"#).expect("valid regex")
});
static BUILD_SPAWN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Command::new\(").expect("valid regex"));

/// A run of invisible characters long enough to carry code (GlassWorm-style):
/// variation selectors, tag characters and zero-width marks. Emoji flags are
/// tag runs after U+1F3F4 and do not count.
fn invisible_run(text: &str) -> Option<usize> {
    let invisible = |c: char| matches!(c, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}' | '\u{E0000}'..='\u{E007F}' | '\u{200B}'..='\u{200D}' | '\u{2060}'..='\u{2064}');
    let mut longest = 0;
    let mut run = 0;
    let mut after_flag = false;
    let mut prev = '\0';
    for c in text.chars() {
        if invisible(c) {
            if run == 0 {
                after_flag = prev == '\u{1F3F4}';
            }
            run += 1;
            if !after_flag {
                longest = longest.max(run);
            }
        } else {
            run = 0;
        }
        prev = c;
    }
    (longest >= 4).then_some(longest)
}

fn line_matches(text: &str, re: &Regex) -> Vec<String> {
    text.lines()
        .enumerate()
        .filter(|(_, l)| !l.trim_start().starts_with('#') && !l.trim_start().starts_with("//"))
        .filter(|(_, l)| re.is_match(l))
        .map(|(n, l)| format!("line {}: {}", n + 1, l.trim()))
        .collect()
}

/// What the package says it is, as "Key: value" lines like a .deb's:
/// METADATA or PKG-INFO, Cargo.toml's [package], or the extension manifest.
pub fn identity(eco: Ecosystem, listing: &Listing) -> Option<String> {
    let text = |pred: &dyn Fn(&str) -> bool| {
        listing
            .entries
            .iter()
            .filter(|e| pred(&e.name))
            .min_by_key(|e| depth(&e.name))
            .and_then(|e| e.content.clone())
    };
    // Name, version, then who and where, each only when present.
    let lines = |name: &str, version: &str, who: (&str, String), home: String| {
        let mut out = format!("Package: {name} {version}").trim_end().to_string();
        for (label, value) in [who, ("Homepage", home)] {
            if !value.is_empty() {
                out.push_str(&format!("\n{label}: {value}"));
            }
        }
        out
    };
    match eco {
        Ecosystem::Wheel | Ecosystem::Sdist => {
            let meta = text(&|n| {
                (base(n) == "METADATA" && n.contains(".dist-info/"))
                    || (base(n) == "PKG-INFO" && depth(n) == 1)
            })?;
            let field = |key: &str| {
                meta.lines()
                    .take_while(|l| !l.is_empty())
                    .find_map(|l| l.strip_prefix(key)?.strip_prefix(':'))
                    .map(|v| v.trim().to_string())
                    .unwrap_or_default()
            };
            let who = [field("Author-email"), field("Author"), field("Maintainer")]
                .into_iter()
                .find(|v| !v.is_empty())
                .unwrap_or_default();
            let home = Some(field("Home-page"))
                .filter(|v| !v.is_empty())
                .unwrap_or_else(|| {
                    meta.lines()
                        .filter_map(|l| l.strip_prefix("Project-URL:"))
                        .find_map(|v| v.split_once(',').map(|(_, url)| url.trim().to_string()))
                        .unwrap_or_default()
                });
            Some(lines(
                &field("Name"),
                &field("Version"),
                ("Maintainer", who),
                home,
            ))
        }
        Ecosystem::Crate => {
            let toml = text(&|n| base(n) == "Cargo.toml")?;
            let get = |key: &str| {
                let re = Regex::new(&format!(r#"(?m)^\s*{key}\s*=\s*"([^"]*)""#)).ok()?;
                re.captures(&toml).map(|c| c[1].to_string())
            };
            let authors = Regex::new(r#"(?m)^\s*authors\s*=\s*\[\s*"([^"]*)""#)
                .ok()
                .and_then(|re| re.captures(&toml).map(|c| c[1].to_string()))
                .unwrap_or_default();
            let home = get("repository")
                .or_else(|| get("homepage"))
                .unwrap_or_default();
            Some(lines(
                &get("name")?,
                &get("version").unwrap_or_default(),
                ("Maintainer", authors),
                home,
            ))
        }
        Ecosystem::Vsix => {
            let manifest: Value =
                serde_json::from_str(&text(&|n| n == "extension/package.json")?).ok()?;
            let s = |v: &Value| v.as_str().unwrap_or("").to_string();
            let home = manifest["repository"]["url"]
                .as_str()
                .or(manifest["repository"].as_str())
                .or(manifest["homepage"].as_str())
                .unwrap_or("")
                .to_string();
            let id = format!("{}.{}", s(&manifest["publisher"]), s(&manifest["name"]));
            Some(lines(
                &id,
                &s(&manifest["version"]),
                ("Publisher", s(&manifest["publisher"])),
                home,
            ))
        }
    }
}

pub fn notes(eco: Ecosystem) -> &'static str {
    match eco {
        Ecosystem::Wheel => "Reads the wheel's .pth files, startup modules and entry points as text. Installing a wheel runs no code, but Python runs .pth import lines at every start.",
        Ecosystem::Sdist => "Reads setup.py and pyproject.toml as text. Installing a source package runs its build backend, so what those files can do is reported, not what they will do.",
        Ecosystem::Crate => "Reads Cargo.toml and build scripts as text. Cargo runs a build script, and rustc runs a proc-macro, whenever the crate is built or an editor analyses the project.",
        Ecosystem::Vsix => "Reads the extension's manifest and JavaScript as text. Extensions run with your permissions in the editor.",
    }
}

pub fn assess(r: &mut Report, eco: Ecosystem, listing: &Listing) {
    let files: Vec<&Entry> = listing
        .entries
        .iter()
        .filter(|e| e.kind == Kind::File)
        .collect();
    match eco {
        Ecosystem::Wheel | Ecosystem::Sdist => python(r, eco, &files),
        Ecosystem::Crate => rust(r, &files),
        Ecosystem::Vsix => vsix(r, &files),
    }
    r.findings.sort_by_key(|f| std::cmp::Reverse(f.severity));
}

fn python(r: &mut Report, eco: Ecosystem, files: &[&Entry]) {
    let mut startup = Group::default();
    let mut severity = Severity::Review;
    for e in files {
        let b = base(&e.name);
        if b.ends_with(".pth") {
            if let Some(text) = &e.content {
                // site.py runs lines that start with "import" followed by a space or tab.
                for (n, line) in text.lines().enumerate() {
                    if line.starts_with("import ") || line.starts_with("import\t") {
                        let shown: String = line.chars().take(160).collect();
                        let more = if line.chars().count() > 160 {
                            format!(" … ({} characters)", line.chars().count())
                        } else {
                            String::new()
                        };
                        startup.push(format!("{}, line {}: {shown}{more}", e.name, n + 1));
                        if line.len() > 1000 || PTH_REACH.is_match(line) {
                            severity = Severity::High;
                        }
                    }
                }
            }
        } else if b == "sitecustomize.py" || b == "usercustomize.py" {
            startup.push(format!("{} (Python imports it at every start)", e.name));
            if e.content.as_deref().is_some_and(|t| PTH_REACH.is_match(t)) {
                severity = Severity::High;
            }
        }
    }
    if !startup.0.is_empty() {
        r.add(
            "py.startup-code",
            severity,
            if severity == Severity::High {
                "The package runs hidden or far-reaching code every time Python starts"
            } else {
                "The package runs code every time Python starts"
            },
            startup.evidence(),
            "Once installed, this runs in every Python program, pip included, without being imported. Tools such as setuptools do this to adjust imports; code that decodes, executes or connects has no such reason.",
        );
    }
    let mut shadow = Group::default();
    for e in files.iter().filter(|e| base(&e.name) == "entry_points.txt") {
        let Some(text) = &e.content else { continue };
        // pip installing `pip` names itself, not something else.
        let own = e
            .name
            .split('-')
            .next()
            .unwrap_or("")
            .to_lowercase()
            .replace('_', "-");
        let mut scripts = false;
        for line in text.lines().map(str::trim) {
            if line.starts_with('[') {
                scripts = line == "[console_scripts]" || line == "[gui_scripts]";
                continue;
            }
            if let Some((name, target)) = line.split_once('=').filter(|_| scripts) {
                let name = name.trim();
                let digits = |c: char| c.is_ascii_digit() || c == '.';
                if COMMON.contains(&name) && name.trim_end_matches(digits) != own {
                    shadow.push(format!("{name} = {}", target.trim()));
                }
            }
        }
    }
    if !shadow.0.is_empty() {
        r.add(
            "py.entrypoint-shadow",
            Severity::Review,
            "The package installs a command named like a common one",
            shadow.evidence(),
            "Where its scripts folder comes first on PATH, typing that command runs the package instead. Check why it needs the name.",
        );
    }
    if eco == Ecosystem::Sdist {
        let mut calls = Group::default();
        for e in files.iter().filter(|e| depth(&e.name) == 1) {
            let Some(text) = &e.content else { continue };
            match base(&e.name) {
                "setup.py" => {
                    for m in line_matches(text, &SETUP_CALLS) {
                        calls.push(format!("{}, {m}", e.name));
                    }
                }
                "pyproject.toml" if text.contains("backend-path") => {
                    calls.push(format!("{}: builds with its own backend (backend-path), which runs from the package", e.name));
                }
                _ => {}
            }
        }
        if !calls.0.is_empty() {
            r.add(
                "py.setup-exec",
                Severity::Review,
                "Installing it runs build code that can start programs, fetch or decode",
                calls.evidence(),
                "pip runs this when it installs the source package. Read these lines, or install a wheel instead.",
            );
        }
    }
}

fn rust(r: &mut Report, files: &[&Entry]) {
    let manifest = files
        .iter()
        .filter(|e| base(&e.name) == "Cargo.toml")
        .min_by_key(|e| depth(&e.name));
    let root = manifest
        .map(|m| m.name.trim_end_matches("Cargo.toml").to_string())
        .unwrap_or_default();
    let toml = manifest.and_then(|m| m.content.as_deref()).unwrap_or("");
    static PROC: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?m)^\s*proc[-_]macro\s*=\s*true").expect("valid regex"));
    let declared = BUILD.captures(toml);
    let build_path = match declared.as_ref() {
        Some(c) if c.get(2).is_some_and(|v| v.as_str() == "false") => None,
        Some(c) => Some(c.get(1).map_or("build.rs", |m| m.as_str()).to_string()),
        None => Some("build.rs".to_string()),
    };
    if let Some(rel) = build_path {
        let path = format!("{root}{rel}");
        if let Some(script) = files.iter().find(|e| e.name == path) {
            let text = script.content.as_deref().unwrap_or("");
            let reach = line_matches(text, &BUILD_REACH);
            let spawn = line_matches(text, &BUILD_SPAWN);
            let mut evidence = format!("{path} runs on every build");
            for c in reach.iter().chain(spawn.iter().take(3)).take(6) {
                evidence.push_str(&format!("\n{c}"));
            }
            let severity = if reach.is_empty() {
                Severity::Review
            } else {
                Severity::High
            };
            if script.content.is_none() {
                evidence.push_str(" (too large to read)");
            }
            r.add(
                "rust.build-script",
                severity,
                if severity == Severity::High {
                    "The crate's build script opens network connections or reads your home folder"
                } else {
                    "The crate runs a build script"
                },
                evidence,
                "Cargo runs it with your permissions on cargo build, check or test, and rust-analyzer runs it when you open the project. Read it first.",
            );
        }
    }
    if PROC.is_match(toml) {
        r.add(
            "rust.proc-macro",
            Severity::Info,
            "The crate is a procedural macro",
            format!("{root}Cargo.toml: proc-macro = true"),
            "Its code runs inside the compiler whenever a crate using it is built or analysed. That is expected for macro crates; check this one is meant to be one.",
        );
    }
}

fn vsix(r: &mut Report, files: &[&Entry]) {
    let manifest = files
        .iter()
        .find(|e| e.name == "extension/package.json")
        .and_then(|e| e.content.as_deref())
        .and_then(|t| serde_json::from_str::<Value>(t).ok())
        .unwrap_or(Value::Null);
    let id = format!(
        "{}.{}",
        manifest["publisher"].as_str().unwrap_or("?"),
        manifest["name"].as_str().unwrap_or("?")
    );
    let events: Vec<&str> = manifest["activationEvents"]
        .as_array()
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let eager: Vec<&str> = events
        .iter()
        .copied()
        .filter(|e| *e == "*" || *e == "onStartupFinished")
        .collect();
    if !eager.is_empty() {
        r.add(
            "vsix.startup-activation",
            Severity::Review,
            &format!("{id} starts whenever the editor starts"),
            format!("activationEvents: {}", eager.join(", ")),
            "Most extensions start only for the files or commands they handle. Check why this one needs to run all the time.",
        );
    }
    let publisher = manifest["publisher"].as_str().unwrap_or("").to_lowercase();
    let mut chain = Group::default();
    for key in ["extensionPack", "extensionDependencies"] {
        for other in manifest[key]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            if other.split('.').next().unwrap_or("").to_lowercase() != publisher {
                chain.push(format!("{key}: {other}"));
            }
        }
    }
    if !chain.0.is_empty() {
        r.add(
            "vsix.dependency-chain",
            Severity::Review,
            &format!("Installing {id} also installs other publishers' extensions"),
            chain.evidence(),
            "Each listed extension installs and updates with this one. Check each publisher too.",
        );
    }
    let mut hidden = Group::default();
    let mut native = Group::default();
    for e in files {
        if let Some(n) = e.content.as_deref().and_then(invisible_run) {
            hidden.push(format!("{}: {n} invisible characters in a row", e.name));
        }
        if e.name.ends_with(".node")
            || e.name.ends_with(".so")
            || e.name.ends_with(".dll")
            || e.name.ends_with(".exe")
        {
            native.push(e.name.clone());
        }
    }
    if !hidden.0.is_empty() {
        r.add(
            "vsix.invisible-code",
            Severity::High,
            &format!("{id} hides text in invisible characters"),
            hidden.evidence(),
            "Long runs of invisible characters can encode code an editor never shows, as in the GlassWorm extensions. Do not install it.",
        );
    }
    if !native.0.is_empty() {
        r.add(
            "vsix.native-binary",
            Severity::Info,
            &format!("{id} ships native programs"),
            native.evidence(),
            "Native code is not visible in the extension's JavaScript. Common for language tools; check the extension is meant to include it.",
        );
    }
}

/// A source folder: a crate (Cargo.toml) or a Python project (setup.py or
/// pyproject.toml) at its top, read like an archive of the same files.
pub fn folder(dir: &Path) -> Option<(Ecosystem, Listing)> {
    let mut listing = Listing::default();
    let mut add = |rel: &str| {
        let path = dir.join(rel);
        if path.is_file() {
            let content = crate::store::read_file(&path, crate::store::MAX_INPUT)
                .ok()
                .map(|b| String::from_utf8_lossy(&b).into_owned());
            listing.entries.push(Entry {
                name: format!("./{rel}"),
                kind: Kind::File,
                target: String::new(),
                mode: 0o644,
                size: 0,
                content,
            });
        }
    };
    let eco = if dir.join("Cargo.toml").is_file() {
        add("Cargo.toml");
        let toml = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap_or_default();
        let script = BUILD
            .captures(&toml)
            .and_then(|c| c.get(1))
            .map_or("build.rs".to_string(), |m| m.as_str().to_string());
        add(&script);
        Ecosystem::Crate
    } else if dir.join("setup.py").is_file() || dir.join("pyproject.toml").is_file() {
        add("setup.py");
        add("pyproject.toml");
        Ecosystem::Sdist
    } else {
        return None;
    };
    Some((eco, listing))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(name: &str, text: &str) -> Entry {
        Entry {
            name: name.into(),
            kind: Kind::File,
            target: String::new(),
            mode: 0o644,
            size: text.len() as u64,
            content: Some(text.into()),
        }
    }

    fn fired(eco: Ecosystem, entries: Vec<Entry>) -> Vec<(String, Severity)> {
        let mut r = Report::new("t", "t");
        assess(
            &mut r,
            eco,
            &Listing {
                compressed: 1,
                entries,
                truncated: None,
            },
        );
        r.findings
            .into_iter()
            .map(|f| (f.rule, f.severity))
            .collect()
    }

    #[test]
    fn recognises_ecosystems_from_entry_names() {
        assert_eq!(
            detect("x.whl", &["pkg/__init__.py", "pkg-1.0.dist-info/WHEEL"]),
            Some(Ecosystem::Wheel)
        );
        assert_eq!(
            detect("x.tar.gz", &["pkg-1.0/PKG-INFO", "pkg-1.0/setup.py"]),
            Some(Ecosystem::Sdist)
        );
        assert_eq!(
            detect("serde-1.0.0.crate", &["serde-1.0.0/Cargo.toml"]),
            Some(Ecosystem::Crate)
        );
        assert_eq!(
            detect(
                "x.vsix",
                &["extension.vsixmanifest", "extension/package.json"]
            ),
            Some(Ecosystem::Vsix)
        );
        assert_eq!(detect("photos.zip", &["a.jpg"]), None);
    }

    #[test]
    fn only_import_lines_in_pth_files_run() {
        let got = fired(
            Ecosystem::Wheel,
            vec![
                file("paths.pth", "lib/extra\n# import nothing\n"),
                file("pkg-1.0.dist-info/WHEEL", "Wheel-Version: 1.0\n"),
            ],
        );
        assert!(got.is_empty());
        let got = fired(
            Ecosystem::Wheel,
            vec![file(
                "litellm_init.pth",
                "import os, base64; exec(base64.b64decode('...'))\n",
            )],
        );
        assert_eq!(got, [("py.startup-code".to_string(), Severity::High)]);
        let setuptools = "import os; var = 'SETUPTOOLS_USE_DISTUTILS'; enabled = os.environ.get(var, 'local') == 'local'; enabled and __import__('_distutils_hack').add_shim(); \n";
        let got = fired(
            Ecosystem::Wheel,
            vec![file("distutils-precedence.pth", setuptools)],
        );
        assert_eq!(got, [("py.startup-code".to_string(), Severity::Review)]);
        let entry = "[console_scripts]\npip = pip._internal.cli.main:main\npip3 = pip._internal.cli.main:main\n";
        assert!(fired(
            Ecosystem::Wheel,
            vec![file("pip-26.0.1.dist-info/entry_points.txt", entry)]
        )
        .is_empty());
        let got = fired(
            Ecosystem::Wheel,
            vec![file("helper-1.0.dist-info/entry_points.txt", entry)],
        );
        assert_eq!(
            got,
            [("py.entrypoint-shadow".to_string(), Severity::Review)]
        );
    }

    #[test]
    fn a_build_script_is_high_only_when_it_reaches_out() {
        let plain = fired(
            Ecosystem::Crate,
            vec![
                file("c-1.0/Cargo.toml", "[package]\nname = \"c\"\n"),
                file(
                    "c-1.0/build.rs",
                    "fn main() { println!(\"cargo:rerun-if-changed=build.rs\"); }\n",
                ),
            ],
        );
        assert_eq!(plain, [("rust.build-script".to_string(), Severity::Review)]);
        let reaching = fired(
            Ecosystem::Crate,
            vec![
                file(
                    "c-1.0/Cargo.toml",
                    "[package]\nbuild = \"src/gen.rs\"\n[lib]\nproc-macro = true\n",
                ),
                file(
                    "c-1.0/src/gen.rs",
                    "fn main() { std::net::TcpStream::connect(\"192.0.2.1:80\"); }\n",
                ),
            ],
        );
        assert_eq!(
            reaching,
            [
                ("rust.build-script".to_string(), Severity::High),
                ("rust.proc-macro".to_string(), Severity::Info)
            ]
        );
        let probe = fired(
            Ecosystem::Crate,
            vec![
                file("c-1.0/Cargo.toml", "[package]\n"),
                file(
                    "c-1.0/build.rs",
                    "fn main() { Command::new(rustc).arg(\"--version\"); }\n",
                ),
            ],
        );
        assert_eq!(probe, [("rust.build-script".to_string(), Severity::Review)]);
        let none = fired(
            Ecosystem::Crate,
            vec![
                file("c-1.0/Cargo.toml", "[package]\nbuild = false\n"),
                file("c-1.0/build.rs", "fn main() {}\n"),
            ],
        );
        assert!(none.is_empty());
    }

    #[test]
    fn identity_reads_what_each_ecosystem_declares() {
        let listing = |entries| Listing {
            compressed: 1,
            entries,
            truncated: None,
        };
        let wheel = listing(vec![file("demo-1.2.dist-info/METADATA", "Metadata-Version: 2.1\nName: demo\nVersion: 1.2\nAuthor-email: Dee <d@example.invalid>\nProject-URL: Source, https://example.invalid/demo\n\nLong description: Name: not this\n")]);
        assert_eq!(identity(Ecosystem::Wheel, &wheel).unwrap(), "Package: demo 1.2\nMaintainer: Dee <d@example.invalid>\nHomepage: https://example.invalid/demo");
        let krate = listing(vec![file("c-1.0.0/Cargo.toml", "[package]\nname = \"c\"\nversion = \"1.0.0\"\nauthors = [\"Ann\"]\nrepository = \"https://example.invalid/c\"\n")]);
        assert_eq!(
            identity(Ecosystem::Crate, &krate).unwrap(),
            "Package: c 1.0.0\nMaintainer: Ann\nHomepage: https://example.invalid/c"
        );
        let vsix = listing(vec![file(
            "extension/package.json",
            r#"{"publisher":"acme","name":"tool","version":"0.3.1"}"#,
        )]);
        assert_eq!(
            identity(Ecosystem::Vsix, &vsix).unwrap(),
            "Package: acme.tool 0.3.1\nPublisher: acme"
        );
    }

    #[test]
    fn invisible_runs_but_not_emoji() {
        assert_eq!(
            invisible_run(&format!("var a = '{}';", "\u{FE0F}".repeat(50))),
            Some(50)
        );
        assert_eq!(invisible_run("❤\u{FE0F} 👨\u{200D}👩\u{200D}👧"), None);
        assert_eq!(
            invisible_run("\u{1F3F4}\u{E0067}\u{E0062}\u{E0065}\u{E006E}\u{E0067}\u{E007F}"),
            None
        );
    }

    #[test]
    fn a_vsix_manifest_is_read() {
        let got = fired(
            Ecosystem::Vsix,
            vec![
                file(
                    "extension/package.json",
                    r#"{"publisher":"acme","name":"tool","activationEvents":["*"],"extensionPack":["acme.helper","other.pub-ext"]}"#,
                ),
                file(
                    "extension/out/extension.js",
                    &format!("const s='{}';", "\u{FE0F}".repeat(50)),
                ),
            ],
        );
        assert_eq!(
            got.iter().map(|g| g.0.as_str()).collect::<Vec<_>>(),
            [
                "vsix.invisible-code",
                "vsix.startup-activation",
                "vsix.dependency-chain"
            ]
        );
    }
}
