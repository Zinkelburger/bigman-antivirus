//! What a project's own code does when it starts: the take-home repository lure.
//! A fake interview hands over a working app whose server or config file fetches
//! text, or decodes it from a file posing as a font, and runs it with Node's
//! `require`. Opening or starting the app is enough; nothing is installed.
//! These rules read the project's source as data and never run it.
use crate::{
    checks::repo::File,
    model::{clean, Report, Severity},
};
use regex::Regex;
use std::{path::Path, sync::LazyLock};

/// Source and configuration files whose text the rules read.
const CODE: [&str; 9] = ["js", "cjs", "mjs", "jsx", "ts", "cts", "mts", "tsx", "py"];
/// Files that are binary when genuine. Text in one is a payload in disguise.
const ASSETS: [&str; 17] = [
    "woff", "woff2", "ttf", "otf", "eot", "png", "jpg", "jpeg", "gif", "ico", "webp", "bmp",
    "avif", "mp3", "mp4", "wav", "webm",
];
/// Lockfiles are large, generated and full of base64 hashes; the lockfile rules read them.
const LOCKFILES: [&str; 4] = [
    "package-lock.json",
    "npm-shrinkwrap.json",
    "composer.lock",
    "bun.lock",
];
/// JSON storage services that Contagious Interview repositories fetch their next
/// stage from (GitLab Threat Intelligence, 2026-02-19; NVISO, 2025-11-13).
const PAYLOAD_HOSTS: [&str; 4] = ["jsonkeeper.com", "npoint.io", "jsonsilo.com", "mocki.io"];
/// How far apart, in lines, a fetch or decode and an `eval` that runs it may be.
/// Loaders keep them together; further apart, the two are usually unrelated.
const NEAR: usize = 5;
/// Bytes of an asset file read: enough to tell a font or image from text.
pub const ASSET_HEAD: u64 = 4096;

/// How much of a project file the rules need.
#[derive(Debug, PartialEq, Eq)]
pub enum Want {
    Text,
    /// Only the first `ASSET_HEAD` bytes.
    Head,
}

fn extension(path: &Path) -> String {
    path.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default()
}

fn is_env(name: &str) -> bool {
    name == ".env" || name.starts_with(".env.") || name.ends_with(".env")
}

/// Whether, and how much of, a file these rules read.
pub fn wanted(rel: &Path) -> Option<Want> {
    let name = rel
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    let ext = extension(rel);
    if ASSETS.contains(&ext.as_str()) {
        return Some(Want::Head);
    }
    // Minified and type-declaration files are vendored or generated, not the project's code.
    if name.ends_with(".min.js") || name.ends_with(".d.ts") {
        return None;
    }
    (CODE.contains(&ext.as_str())
        || is_env(&name)
        || (ext == "json" && !LOCKFILES.contains(&name.as_str())))
    .then_some(Want::Text)
}

/// Code that runs text it was handed. `Function.constructor`, and the Function
/// constructor given `require`, give that text the whole Node runtime.
static RUN_WITH_REQUIRE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\bFunction\s*\.\s*constructor\b|\bFunction\s*\(\s*['"`]require['"`]|\.constructor\s*\(\s*['"`]require['"`]"#)
        .expect("valid regex")
});
// `eval(` with something to run, not a method such as Playwright's `$$eval` or
// prose that mentions `eval()`.
static RUN_JS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:^|[^\w$.])eval\([^)]|\bnew\s+Function\s*\(|\bvm\s*\.\s*(?:runInThisContext|runInNewContext|runInContext|compileFunction)\s*\(|\bnew\s+vm\s*\.\s*Script\b")
        .expect("valid regex")
});
static RUN_PY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:^|[^\w.])(?:exec|eval)\([^)]").expect("valid regex"));
/// Where the text comes from: the network or an encoding. Reading a file counts
/// separately (`READ_JS`), and only for a file that is not code.
static FETCH_JS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\baxios\s*\.\s*\w+\s*\(|\baxios\s*\(|\bfetch\s*\(|\bhttps?\s*\.\s*(?:get|request)\s*\(|\bgot\s*\(|\bXMLHttpRequest\b|\batob\s*\(|['"](?:base64|hex)['"]"#)
        .expect("valid regex")
});
static READ_JS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\breadFile(?:Sync)?\s*\(").expect("valid regex"));
static CODE_FILE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\.(?:[cm]?js|ts|json)['"`]"#).expect("valid regex"));
static FETCH_PY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\brequests\s*\.\s*(?:get|post|request)\s*\(|\burlopen\s*\(|\bhttpx\s*\.\s*(?:get|post|request)\s*\(|\bb64decode\s*\(|\bfromhex\s*\(|\bcodecs\s*\.\s*decode\s*\(")
        .expect("valid regex")
});
static BASE64: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Za-z0-9+/_-]{16,}={0,2}").expect("valid regex"));
static HEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:[0-9a-fA-F]{2}){12,}\b").expect("valid regex"));

/// The rules for one project file.
pub fn assess(r: &mut Report, f: &File) {
    match wanted(&f.path) {
        Some(Want::Head) => disguised(r, f),
        Some(Want::Text) => {
            let ext = extension(&f.path);
            if ext == "py" {
                loader(r, f, &RUN_PY, &|l| FETCH_PY.is_match(l));
            } else if CODE.contains(&ext.as_str()) {
                loader(r, f, &RUN_JS, &|l| {
                    FETCH_JS.is_match(l) || (READ_JS.is_match(l) && !CODE_FILE.is_match(l))
                });
            }
            hidden_urls(r, f);
        }
        None => {}
    }
}

fn shown(line: &str) -> String {
    let line = line.trim();
    let cut: String = line.chars().take(200).collect();
    clean(&if cut.len() < line.len() {
        format!("{cut}…")
    } else {
        cut
    })
}

fn at(f: &File, lines: &[&str], n: usize) -> String {
    format!("{}, line {}: {}", f.path.display(), n + 1, shown(lines[n]))
}

/// Text run as code next to where it was fetched or decoded. One finding per file.
fn loader(r: &mut Report, f: &File, run: &Regex, fetch: &dyn Fn(&str) -> bool) {
    let lines: Vec<&str> = f.text.lines().collect();
    let strong = lines.iter().position(|l| RUN_WITH_REQUIRE.is_match(l));
    let pair = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| run.is_match(l) || RUN_WITH_REQUIRE.is_match(l))
        .find_map(|(i, _)| {
            let from = i.saturating_sub(NEAR);
            let to = (i + NEAR).min(lines.len().saturating_sub(1));
            // The nearest source, preferring one before the line that runs it.
            (from..=to)
                .filter(|&j| fetch(lines[j]))
                .min_by_key(|&j| (j.abs_diff(i), j > i))
                .map(|j| (i, j))
        });
    let (run_line, source_line) = match (pair, strong) {
        (Some((i, j)), _) => (i, Some(j)),
        (None, Some(i)) => (i, None),
        (None, None) => return,
    };
    let mut evidence = at(f, &lines, run_line);
    if let Some(j) = source_line.filter(|&j| j != run_line) {
        evidence.push_str(&format!("\nfed by line {}: {}", j + 1, shown(lines[j])));
    }
    r.add(
        "repo.code-loader",
        Severity::High,
        &format!("{} runs text it fetches or decodes as code", f.path.display()),
        evidence,
        "This is how fake take-home projects deliver malware: starting the app runs whatever the text says, with your permissions. Do not run the project on a computer that holds keys, wallets or work access; read where the text comes from first. To watch it run, use bigman try, which opens a disposable container without your files or keys.",
    );
}

fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    let (mut acc, mut bits) = (0u32, 0u32);
    for c in s.trim_end_matches('=').bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            _ => return None,
        };
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Some(out)
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok())
        .collect()
}

/// A URL, if the decoded bytes are one.
fn as_url(bytes: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(bytes).ok()?.trim();
    let lower = text.to_ascii_lowercase();
    (lower.starts_with("https://") || lower.starts_with("http://"))
        .then(|| text.chars().take(200).collect())
}

fn payload_host(text: &str) -> Option<&'static str> {
    let lower = text.to_ascii_lowercase();
    PAYLOAD_HOSTS.into_iter().find(|h| lower.contains(h))
}

/// URLs stored base64- or hex-encoded, and the JSON storage hosts these lures use.
/// At most one finding of each kind per file.
fn hidden_urls(r: &mut Report, f: &File) {
    let mut encoded = None;
    let mut host = None;
    for (i, line) in f.text.lines().enumerate() {
        if encoded.is_none() {
            encoded = BASE64
                .find_iter(line)
                .filter_map(|m| base64_decode(m.as_str()))
                .chain(HEX.find_iter(line).filter_map(|m| hex_decode(m.as_str())))
                .find_map(|b| as_url(&b))
                .map(|url| (i, url));
        }
        if host.is_none() {
            host = payload_host(line).map(|h| (i, h, None));
        }
        if encoded.is_some() && host.is_some() {
            break;
        }
    }
    // A storage host hidden by the encoding counts too.
    if host.is_none() {
        if let Some((i, url)) = &encoded {
            host = payload_host(url).map(|h| (*i, h, Some(url.clone())));
        }
    }
    let lines: Vec<&str> = f.text.lines().collect();
    if let Some((i, url)) = &encoded {
        r.add(
            "repo.encoded-url",
            Severity::Review,
            &format!("{} stores a web address in encoded form", f.path.display()),
            format!("{}\ndecodes to: {}", at(f, &lines, *i), clean(url)),
            "Projects rarely need to hide an address. Find the code that decodes it and what it does with the reply before you start the project.",
        );
    }
    if let Some((i, h, decoded)) = host {
        r.add(
            "repo.payload-host",
            Severity::Review,
            &format!("{} uses {h}, a JSON storage service fake interview projects load code from", f.path.display()),
            format!(
                "{}{}",
                at(f, &lines, i),
                decoded.map(|u| format!("\ndecodes to: {}", clean(&u))).unwrap_or_default()
            ),
            "Anyone can change what such a link returns at any time. Read what the project does with the reply; if it runs it, do not start the project.",
        );
    }
}

/// What text in an asset's place looks like.
fn describe(text: &str) -> &'static str {
    let body: Vec<char> = text.chars().filter(|c| !c.is_whitespace()).collect();
    let all = |p: fn(&char) -> bool| !body.is_empty() && body.iter().all(p);
    if all(char::is_ascii_hexdigit) {
        "hex-encoded data"
    } else if all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '=' | '-' | '_')) {
        "base64-encoded data"
    } else if ["function", "require(", "=>", "const ", "var ", "import "]
        .iter()
        .any(|k| text.contains(k))
    {
        "JavaScript"
    } else {
        "plain text"
    }
}

/// A font, image or media file that is text: a genuine one starts with binary
/// magic bytes, so readable text in its place is code or data posing as an asset.
fn disguised(r: &mut Report, f: &File) {
    let text = &f.text;
    // Git LFS keeps a small text pointer in place of large files.
    if text.starts_with("version https://git-lfs.github.com/spec/") {
        return;
    }
    let printable = text
        .chars()
        .filter(|c| c.is_ascii_graphic() || matches!(c, ' ' | '\n' | '\r' | '\t'))
        .count();
    let total = text.chars().count();
    if total < 32 || printable * 100 < total * 97 {
        return;
    }
    r.add(
        "repo.disguised-asset",
        Severity::High,
        &format!(
            "{} is {} named as a {} file",
            f.path.display(),
            describe(text),
            extension(&f.path)
        ),
        format!("{}\nstarts with: {}", f.path.display(), shown(text.lines().next().unwrap_or(""))),
        "A genuine font, image or media file is binary. Text in its place is usually a payload that another file in the project decodes and runs, or that an editor task starts. Do not open or run the project; find what reads this file.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn run(path: &str, text: &str) -> Vec<(String, Severity)> {
        let mut r = Report::new("t", "t");
        assess(
            &mut r,
            &File {
                path: PathBuf::from(path),
                text: text.into(),
            },
        );
        r.findings
            .into_iter()
            .map(|f| (f.rule, f.severity))
            .collect()
    }

    #[test]
    fn decodes_base64_and_hex_urls() {
        let url = "https://www.jsonkeeper.com/b/TEST1";
        assert_eq!(
            as_url(&base64_decode("aHR0cHM6Ly93d3cuanNvbmtlZXBlci5jb20vYi9URVNUMQ==").unwrap())
                .as_deref(),
            Some(url)
        );
        let hex: String = url.bytes().map(|b| format!("{b:02x}")).collect();
        assert_eq!(as_url(&hex_decode(&hex).unwrap()).as_deref(), Some(url));
    }

    #[test]
    fn the_function_constructor_with_require_fires_alone() {
        let found = run(
            "server/errorHandler.cjs",
            "const h = new (Function.constructor)('require', errCode);\nh(require);\n",
        );
        assert_eq!(found, vec![("repo.code-loader".into(), Severity::High)]);
    }

    #[test]
    fn eval_needs_a_fetch_or_decode_nearby() {
        assert!(run("src/calc.js", "export const f = (s) => eval(s);\n").is_empty());
        let mut far = String::from("const r = await fetch(u);\n");
        far.push_str(&"\n".repeat(NEAR + 5));
        far.push_str("eval(x);\n");
        assert!(run("src/a.js", &far).is_empty());
        assert_eq!(
            run(
                "src/a.js",
                "const r = await fetch(u);\neval(await r.text());\n"
            ),
            vec![("repo.code-loader".into(), Severity::High)]
        );
        assert_eq!(
            run(
                "setup.py",
                "import urllib.request\nexec(urllib.request.urlopen(u).read())\n"
            ),
            vec![("repo.code-loader".into(), Severity::High)]
        );
    }

    /// Shapes seen in real projects that must not fire.
    #[test]
    fn ordinary_eval_and_exec_do_not_fire() {
        for (path, text) in [
            // A test loading the project's own script.
            ("tests/content.test.js", "const fs = require('fs');\neval(fs.readFileSync(require.resolve('../content.js'), 'utf8'));\n"),
            // Playwright's $$eval and page.eval are methods, not eval.
            ("scripts/e2e.mjs", "const r = await fetch(u);\nconst n = await page.$$eval('#moves', (x) => x.length);\n"),
            // Prose that mentions exec or eval().
            ("tests/test_mount.py", "import requests\n# Reads /etc/os-release via exec (inside the namespace)\n# equivalent to running ``eval()`` on it\n"),
            // A config loader running a local file.
            ("conf.py", "with open(path) as f:\n    exec(f.read(), ns)\n"),
        ] {
            assert!(run(path, text).is_empty(), "{path}");
        }
    }

    #[test]
    fn a_real_font_or_an_lfs_pointer_is_not_disguised() {
        let woff2 = "wOF2\0\u{1}\0\0\u{FFFD}\u{FFFD}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
        assert!(run("public/fonts/a.woff2", woff2).is_empty());
        assert!(run(
            "img/hero.png",
            "version https://git-lfs.github.com/spec/v1\noid sha256:4d7a\nsize 12345\n"
        )
        .is_empty());
        let found = run("webfonts/fa-brands-regular.woff2", &"636f6e7374".repeat(20));
        assert_eq!(found, vec![("repo.disguised-asset".into(), Severity::High)]);
    }

    #[test]
    fn data_urls_and_hashes_are_not_urls() {
        assert!(run(
            "src/logo.js",
            "export default 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==';\n"
        )
        .is_empty());
        assert!(run(
            "config.json",
            "{\"sha\": \"e0b5edb035e5629dd83dcc03ac6be746d934c418\"}\n"
        )
        .is_empty());
    }

    #[test]
    fn skips_minified_vendor_files_and_lockfiles() {
        assert_eq!(wanted(Path::new("public/jquery.min.js")), None);
        assert_eq!(wanted(Path::new("package-lock.json")), None);
        assert_eq!(
            wanted(Path::new("server/config/.config.env")),
            Some(Want::Text)
        );
        assert_eq!(wanted(Path::new("fonts/x.WOFF2")), Some(Want::Head));
    }
}
