//! The rule registry: every finding Bigman can produce, with its stable ID,
//! default severity, plain explanation, remediation and the threats it covers.
//! `fixtures/<rule-id>/` holds a synthetic input that fires it (tests/fixtures.rs),
//! and `docs/rules.md` is generated from this table.
use crate::model::Severity::{self, High, Info, Review};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub struct Rule {
    pub id: &'static str,
    /// The usual severity. A few rules vary by context and say so in `explanation`.
    pub severity: Severity,
    pub title: &'static str,
    /// What the finding means, in plain language.
    pub explanation: &'static str,
    /// What the reader can do about it.
    pub remediation: &'static str,
    /// IDs from docs/threats this rule covers in part or whole.
    pub threats: &'static [&'static str],
    /// Operating systems the rule applies to.
    pub platforms: &'static [&'static str],
}

const LINUX: &[&str] = &["linux"];

macro_rules! rule {
    ($id:literal, $severity:expr, $title:literal, $explanation:literal, $remediation:literal, [$($threat:literal),*]) => {
        Rule {
            id: $id,
            severity: $severity,
            title: $title,
            explanation: $explanation,
            remediation: $remediation,
            threats: &[$($threat),*],
            platforms: LINUX,
        }
    };
}

pub const RULES: &[Rule] = &[
    rule!("shell.download", Review, "Downloads remote content",
        "The script fetches something from the network with curl or wget. What it fetches is not part of this inspection.",
        "Inspect each downloaded payload and confirm where it comes from before running the script.", ["DP-18"]),
    rule!("shell.execute-download", High, "Downloads and runs code",
        "The script pipes downloaded content straight into a shell, so whatever the server sends at that moment runs on your machine.",
        "Download the payload separately, inspect the exact bytes, and run only what you reviewed.", ["DP-18", "PH-17"]),
    rule!("shell.privilege", Review, "Asks for administrator rights",
        "The script runs commands with sudo, doas or su. Those commands can change anything on the system.",
        "Check why each privileged command is needed and whether it could run without administrator rights.", ["DP-18"]),
    rule!("shell.credentials", High, "Touches credentials or browser data",
        "The script refers to SSH keys, cloud or registry tokens, or browser profile data. Installers rarely need these.",
        "Run it without your real credentials present, and find out why it reads these paths.", ["CR-01", "CR-02", "CR-03", "CR-18", "SC-23"]),
    rule!("shell.persistence", Review, "Sets something up to run later",
        "The script writes cron jobs, systemd units, autostart entries, shell profiles or SSH authorized keys, so its effect outlasts the install.",
        "Review exactly what will run in future sessions and compare the startup files afterwards.", ["HI-02", "HI-03", "HI-04", "HI-05", "HI-10"]),
    rule!("shell.dynamic", Review, "Runs code that is built or decoded at run time",
        "The script uses eval, source, base64 or an interpreter on generated input. Static rules cannot see what that code will be.",
        "Decode or expand the dynamic part by hand and inspect it before running the script.", ["DP-18"]),
    rule!("shell.permissions", High, "Sets broad or privileged file permissions",
        "The script makes files world-writable or setuid. Either can let other programs or users gain access they should not have.",
        "Prefer the narrowest permissions the program needs, and never setuid a file you did not build.", ["HI-17"]),
    rule!("shell.delete", Review, "Deletes files recursively or by wildcard",
        "The script removes files with rm -r, rm -f or a wildcard. A wrong variable can expand to the wrong path.",
        "Check what each deleted path expands to before running the script.", ["DP-18"]),
    rule!("shell.network-listener", Review, "Uses a tool that can open or forward network connections",
        "The script runs nc, ncat, netcat or socat, which can listen for connections or relay a shell.",
        "Check which address and port it uses, and whether it listens to the whole network.", ["NE-14"]),
    rule!("shellcheck.warning", Review, "ShellCheck warning",
        "ShellCheck found a shell correctness problem. It is about how the script is written, not whether it is malicious.",
        "Read the ShellCheck code linked in the message to see what can go wrong.", ["DP-18"]),
    rule!("package.lifecycle", Review, "Package runs a script during install",
        "A preinstall, install, postinstall or prepare script runs automatically when the package is installed.",
        "Read the script and the files it calls. Turning off automatic install scripts for the project stops them running.", ["SC-01"]),
    rule!("package.script", Info, "Package defines a script",
        "A script that runs only when you ask for it, for example npm test.",
        "Read it before you run it.", ["SC-01"]),
    rule!("package.unresolved-hook", Review, "Lockfile lists an install script that is not on disk",
        "The lockfile says a dependency has an install script, but the package is not installed here, so its script could not be read.",
        "Inspect that package's archive without running its scripts.", ["SC-01", "SC-02"]),
    rule!("trace.sensitive-path", Review, "The rehearsal touched a sensitive path",
        "While running in the offline sandbox, the installer opened or looked for credential or browser paths. The sandbox has no real ones.",
        "Check the system call result; the access may have failed. Find out why the installer wanted that path.", ["CR-18", "CR-22"]),
    rule!("trace.network", Review, "The rehearsal tried to connect to the network",
        "While running in the offline sandbox, the installer attempted a network connection. The sandbox has no network.",
        "Find out what it was trying to reach and why.", ["NE-14"]),
    rule!("trace.process", Info, "The rehearsal started a program",
        "A program started while the installer ran in the offline sandbox.",
        "Review the program and its arguments.", ["DP-18"]),
    rule!("pdf.structure", Info, "The PDF contains an active or external element",
        "The document has an open action, JavaScript, a launch action, an embedded file, a link or encryption. JavaScript and launch actions are Review; the rest are Info.",
        "Check the parser output and the link destinations. A tag alone is not evidence of malware.", ["DS-12"]),
    rule!("clamav.match", High, "A file matches a ClamAV signature",
        "ClamAV recognised the file as matching a known-malware or heuristic signature. Limit or encryption signatures mean the content was not fully examined.",
        "If you did not expect the file, quarantine it; Bigman keeps a recovery copy you can restore.", ["DP-08", "MW-01"]),
    rule!("clamav.no-definitions", High, "No malware signatures found",
        "Neither Bigman's private signature folder nor /var/lib/clamav has ClamAV definitions, so file scans cannot run.",
        "Update signatures from Protect > Scans, or point ClamAV at your own database.", ["DP-20"]),
    rule!("clamav.stale", Review, "Malware signatures are out of date",
        "The newest ClamAV definitions file is more than three days old. File age is a freshness signal, not a verified version.",
        "Update signatures from Protect > Scans.", ["DP-20"]),
    rule!("tool.missing", Info, "An optional tool is not installed",
        "One of the tools Bigman can use is not installed. Only the checks that need it are affected.",
        "Install it from Setup if you want those checks.", []),
    rule!("monitor.stopped", Review, "Canary monitoring is not reporting",
        "The canary monitor has not written a heartbeat in the last ten seconds, so reads of decoy files are not being noticed. The background service also reports this live, once, when the heartbeat has been missing for a minute while the computer was awake.",
        "Start monitoring from Protect > Canaries, or install the background services from Setup.", ["CR-22", "HI-24"]),
    rule!("monitor.gap", Review, "Monitoring was off while the computer was awake",
        "The canary monitor or Bigman's background service was not running for a while, so nothing was watching in that window. Suspended time does not count, and a reboot starts over. A background-service gap after a normal stop (logout, systemctl --user stop) is Info; one after a crash or kill is Review. Malware running as you can stop these processes, which is why the gap itself is shown.",
        "If you did not stop monitoring, review Activity and the startup files for changes made in that window.", ["HI-24", "CR-22"]),
    rule!("ssh.key.added", High, "A new SSH key can log in to your account",
        "A key appeared in ~/.ssh/authorized_keys that was not there before. Whoever holds the matching private key can log in as you wherever sshd accepts it, without your password. The key's fingerprint and comment are shown, never the key itself; fingerprints published as indicators of compromise are named.",
        "If you did not add it, remove its line, then check ~/.ssh, your crontab and startup files for anything added at the same time. Turn sshd off if you do not need remote logins.", ["NE-05", "HI-10", "SC-19"]),
    rule!("ssh.tunnel.reverse", Review, "SSH is forwarding connections back to this computer",
        "One of your ssh or autossh processes uses -R or RemoteForward, so connections to a port on another computer arrive here. Developers do this on purpose. It is High when it forwards to this computer's SSH server, turns off host-key checking, or was started from a code editor, as the October 2026 tampered Cursor extension did.",
        "If you did not start it, stop the process, then look for what restarts it (crontab -l, ~/.config/systemd/user, autostart) and check ~/.ssh/authorized_keys.", ["NE-20", "NE-14", "SC-19"]),
    rule!("ssh.login.remote", Review, "Someone logged in over the network",
        "The login manager reports a session that came over the network, usually through sshd. It is High when it came from this computer's own address, which is how logins through a reverse tunnel arrive (or a deliberate ssh to localhost).",
        "If this was not you, end the session with loginctl terminate-session, remove keys you do not recognise from ~/.ssh/authorized_keys, and change your password.", ["NE-03", "NE-05", "NE-20"]),
    rule!("ssh.key.stray", Review, "An SSH private key is stored outside ~/.ssh",
        "A file directly inside a temporary folder, ~/.config, ~/.cache or ~/.local/share starts like an SSH private key. Tools rarely keep keys there; droppers do, to authenticate a tunnel. Only the first line is read.",
        "If no tool you use put it there, keep a copy for investigation, look for an ssh or autossh process using it, then delete it.", ["CR-03", "NE-20"]),
    rule!("git.plaintext-credentials", Review, "Git stores credentials in plain text",
        "Git's global credential helper is 'store', which writes tokens unencrypted to ~/.git-credentials. The file itself was not read.",
        "Switch to your desktop keyring's credential helper and revoke tokens you no longer use.", ["CR-01", "CR-08"]),
    rule!("persistence.change", Review, "A startup file changed",
        "A file that starts programs for you (a systemd unit, autostart entry, shell profile, cron file) is new or different from the baseline you accepted.",
        "Review the file and the package that owns it before accepting it into the baseline.", ["HI-01", "HI-02", "HI-03", "HI-04", "HI-05", "HI-10"]),
    rule!("persistence.owned_modified", Review, "A package's file was modified",
        "A startup or configuration file that a package installed no longer matches the package database. Configuration files are often edited on purpose; other files are not expected to change.",
        "Check the change is one you or your administrator made; otherwise reinstall the package.", ["HI-01", "HI-22", "DP-21", "MW-08"]),
    rule!("persistence.sensitive_modified", High, "An authentication or loader file was modified",
        "A PAM, dynamic-loader or SSH file installed by a package no longer matches the package database. These files decide who can log in and what code every program loads.",
        "Compare it with the package's version (rpm -V, dpkg --verify or pacman -Qkk) and restore it if you did not make the change.", ["HI-08", "HI-10", "HI-22", "MW-15"]),
    rule!("persistence.package_update", Info, "A package updated a startup file",
        "A startup or configuration file changed, and the package database shows it is exactly what the owning package installed.",
        "Nothing to do; the update explains the change.", ["HI-01", "HI-22"]),
    rule!("persistence.privilege", High, "A startup file gained elevated privileges",
        "A startup or configuration file now has the setuid or setgid bit or file capabilities, which lets it run with more rights than the user who starts it.",
        "Remove the setuid bit or capability unless you know why it is there.", ["HI-17", "HI-18"]),
    rule!("persistence.metadata", Review, "A startup file changed permissions or owner",
        "A startup or configuration file has the same content as before but a different mode or owner.",
        "Check why before accepting the new baseline; a writable startup file lets other users change what runs for you.", ["HI-01", "HI-05", "HI-19"]),
    rule!("persistence.removed", Review, "A startup file was removed",
        "A startup file in your baseline is gone.",
        "Check whether an update or your own change explains it.", ["HI-01", "HI-02", "HI-03", "HI-04", "HI-05"]),
    rule!("persistence.initial", Info, "Startup file recorded for the first baseline",
        "There is no baseline yet, so every startup file is listed for review.",
        "Review the list, then accept it as the baseline to compare future changes against.", ["HI-01", "HI-02", "HI-03", "HI-04", "HI-05"]),
    rule!("receipt.hook.sensitive_path", Review, "An install script writes to a startup or authentication path",
        "A package's install script writes to a path that decides what runs at login or boot, or how you authenticate (profile.d, PAM, sudoers, cron, systemd, udev, the loader, SSH, home directories).",
        "Read the line. Packages rarely need to change these paths from a script; if you did not expect it, reinstall from your distribution's repository or remove the package.", ["DP-09", "DP-11", "HI-05"]),
    rule!("receipt.hook.download", High, "An install script downloads something",
        "A package's install script calls curl, wget or a similar tool. Distribution packages ship their files, so a download at install time runs code nobody reviewed.",
        "Find out what it downloads before trusting the package.", ["DP-09", "DP-11", "DP-18"]),
    rule!("receipt.hook.obfuscation", Review, "An install script contains encoded or hidden code",
        "A package's install script uses base64 decoding, eval, a long encoded blob, escaped bytes or invisible Unicode, which hide what it does.",
        "Decode the line by hand, never by running it, and check what it does.", ["DP-01", "DP-09", "DP-11"]),
    rule!("receipt.hook.changed", Review, "An install script changed in an update",
        "A package's install script differs from the one recorded in an earlier receipt for the same package.",
        "Read what the script does now; a changed hook in an ordinary update is worth a look.", ["DP-01", "DP-09"]),
    rule!("receipt.file.privileged", Review, "A package installed a program with raised privileges",
        "An installed or updated package ships a file with the setuid or setgid bit or file capabilities, so it runs with more rights than the user who starts it.",
        "Check this is expected for the package.", ["HI-17", "HI-18"]),
    rule!("receipt.sensitive_package", Info, "A package that controls authentication, networking or boot changed",
        "One of the packages that decide how this computer authenticates, connects or boots (openssh, sudo, PAM, systemd, glibc, xz, curl, OpenSSL, the kernel, shells) was installed or updated.",
        "Nothing to do when the update came from your distribution; it is listed so the change is easy to find later.", ["DP-01", "DP-06"]),
    rule!("receipt.after.listener", Review, "A program started listening right after a package change",
        "Within a minute of a package transaction, a program began accepting network connections.",
        "Check whether the package is meant to run a server, and who can reach it (Network > Listening programs).", ["NE-01", "DP-09"]),
    rule!("canary.event", High, "A decoy file was opened or changed",
        "A canary decoy was read, changed, moved or removed while the monitor was running. inotify cannot say which program did it.",
        "Check what was running at that time. Backups, indexers and your own tests can also trigger decoys.", ["CR-22"]),
    rule!("canary.verified", Info, "Local read detection verified",
        "A deliberate read of the decoy produced a fresh receipt from the running monitor, so local detection works.",
        "Leave monitoring running.", ["CR-22"]),
    rule!("canary.notifications-unavailable", Info, "Desktop pop-ups are not available",
        "notify-send is not installed, so canary alerts appear in Activity but not as pop-ups.",
        "Install your distribution's desktop notification utilities if you want pop-ups.", []),
    rule!("net.listener.wildcard", Review, "Listens on every network interface",
        "A program accepts connections on all of this computer's network addresses, so other machines on the same network can reach it.",
        "If only this computer uses it, make it listen on 127.0.0.1, or stop it.", ["NE-01", "NE-22"]),
    rule!("net.listener.docker_api", High, "The container API is open on the network",
        "Docker's API is listening on TCP. Whoever reaches it can start containers with root access to this machine.",
        "Use the local socket instead, or require TLS client certificates.", ["NE-06", "MW-03"]),
    rule!("net.listener.debugger", High, "A remote debugger is reachable from the network",
        "A debugger port (Node.js inspector, Chromium DevTools, debugpy) is open beyond this computer. Debuggers let whoever connects run code in the program.",
        "Bind the debugger to 127.0.0.1, or stop it when you are done.", ["NE-11"]),
    rule!("net.listener.lan_service", Review, "A local AI or notebook server is reachable from the network",
        "A server that usually has no login (Ollama, Jupyter, Gradio and similar) listens beyond this computer.",
        "If only you use it, bind it to 127.0.0.1.", ["NE-09", "NE-10"]),
    rule!("denial.selinux", Info, "SELinux stopped or logged an access",
        "SELinux refused a program something its policy does not allow, or logged it when the program runs in a permissive domain. This is the system's own defense working.",
        "If the program needed this to work, a local policy module can allow it; Bigman shows the record and never changes policy for you.", ["DS-18"]),
    rule!("denial.apparmor", Info, "AppArmor stopped or logged an access",
        "AppArmor refused a confined program something its profile does not allow, or logged it in complain mode.",
        "If the program needed this to work, its profile can be extended; Bigman shows the record and never changes profiles for you.", ["DS-18"]),
    rule!("lynis.finding", Review, "Lynis recommendation",
        "Lynis reported a warning or suggestion. The line is shown exactly as Lynis wrote it.",
        "Look up the Lynis test ID before changing configuration.", ["HI-24"]),
    rule!("osv.advisory", Review, "A dependency has a published advisory",
        "OSV lists a known vulnerability for a version this project depends on. A vulnerability is not evidence of malware.",
        "Read the advisory's affected and fixed versions and update the dependency.", ["SC-05", "SC-10"]),
    rule!("yara.output", Review, "YARA-X rules matched",
        "Rules you supplied matched the target. A match is the result of those rules, not proof of malware.",
        "Check where the rules came from and what they look for.", ["MW-01"]),
];

pub fn get(id: &str) -> Option<&'static Rule> {
    RULES.iter().find(|r| r.id == id)
}

/// A rule as the UI shows it beside a finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct RuleInfo {
    pub id: String,
    pub severity: Severity,
    pub title: String,
    pub explanation: String,
    pub remediation: String,
    pub threats: Vec<String>,
}

impl From<&Rule> for RuleInfo {
    fn from(r: &Rule) -> Self {
        RuleInfo {
            id: r.id.into(),
            severity: r.severity,
            title: r.title.into(),
            explanation: r.explanation.into(),
            remediation: r.remediation.into(),
            threats: r.threats.iter().map(|t| (*t).into()).collect(),
        }
    }
}

pub fn infos() -> Vec<RuleInfo> {
    RULES.iter().map(RuleInfo::from).collect()
}

/// docs/rules.md, generated from the registry; a test keeps the file current.
pub fn markdown() -> String {
    let mut out = String::from(
        "# Bigman rules\n\n> Generated from `crates/core/src/rules/mod.rs`. Do not edit by hand: run `BIGMAN_UPDATE_RULES_DOC=1 cargo test -p bigman-core --test fixtures`.\n\nEvery finding Bigman shows comes from one of these rules. Each has a synthetic fixture in `fixtures/<rule-id>/` that must fire it.\n\n| Rule | Severity | Title | Threats |\n|---|---|---|---|\n",
    );
    for r in RULES {
        out.push_str(&format!(
            "| `{}` | {:?} | {} | {} |\n",
            r.id,
            r.severity,
            r.title,
            r.threats.join(", ")
        ));
    }
    for r in RULES {
        out.push_str(&format!(
            "\n## `{}`\n\n**{}** ({:?})\n\n{}\n\nWhat you can do: {}\n",
            r.id, r.title, r.severity, r.explanation, r.remediation
        ));
    }
    out
}
