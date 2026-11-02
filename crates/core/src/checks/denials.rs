//! SELinux and AppArmor denial records, translated into one plain sentence each.
use crate::model::{Report, Severity};

/// One denial, parsed from an AVC or AppArmor record.
#[derive(Debug, Clone, PartialEq)]
pub struct Denial {
    /// "selinux" or "apparmor".
    pub module: &'static str,
    pub program: String,
    /// The SELinux policy confining the program, when it is a specific one.
    pub confined_by: Option<String>,
    pub action: String,
    pub target: String,
    /// Logged but not enforced (SELinux permissive domain, AppArmor complain mode).
    pub allowed: bool,
    /// Groups repeats of the same denial.
    pub key: String,
    pub record: String,
}

/// Recurring denials that are known to be harmless policy gaps.
const KNOWN_NOISE: [(&str, &str); 2] = [("systemd-oomd", "signull"), ("gnome-shell", "getattr")];

fn field<'a>(record: &'a str, name: &str) -> Option<&'a str> {
    let start = record.find(&format!("{name}="))? + name.len() + 1;
    let rest = &record[start..];
    Some(match rest.strip_prefix('"') {
        Some(quoted) => &quoted[..quoted.find('"')?],
        None => rest.split_whitespace().next()?,
    })
}

/// SELinux context `user:role:type:level` → its type.
fn context_type(context: &str) -> &str {
    context.split(':').nth(2).unwrap_or(context)
}

fn verb(permission: &str) -> &'static str {
    match permission {
        "read" | "open" | "getattr" | "search" | "read_file" | "r" => "read",
        "write" | "append" | "create" | "add_name" | "remove_name" | "unlink" | "rename"
        | "setattr" | "w" | "a" => "change",
        "execute" | "execute_no_trans" | "map" | "x" | "m" => "run",
        "connectto" | "name_connect" | "name_bind" | "sendto" | "connect" => "connect to",
        "send_msg" => "message",
        "signull" | "signal" | "sigkill" | "sigstop" | "ptrace" => "signal or inspect",
        _ => "use",
    }
}

/// Domains every desktop has; naming them explains nothing.
const GENERIC_DOMAINS: [&str; 8] = [
    "unconfined",
    "unconfined_service",
    "user",
    "staff",
    "sysadm",
    "init",
    "kernel",
    "system_dbusd",
];

/// `netbird_t` → "netbird"; `None` for the everyday domains.
fn policy_name(context_type: &str) -> Option<String> {
    let name = context_type.strip_suffix("_t")?;
    (!GENERIC_DOMAINS.contains(&name)).then(|| name.to_string())
}

pub fn parse(record: &str) -> Option<Denial> {
    if let Some(rest) = record.split("avc:  denied  {").nth(1) {
        let permissions: Vec<&str> = rest.split('}').next()?.split_whitespace().collect();
        let program = field(record, "comm").unwrap_or("A program").to_string();
        let tclass = field(record, "tclass").unwrap_or("object");
        let mut tcontext = context_type(field(record, "tcontext").unwrap_or("")).to_string();
        let name = field(record, "name");
        // /proc/<pid>: one entry per process looked at, which reads (and folds) as one thing.
        let process_entry = field(record, "dev") == Some("proc")
            && name.is_some_and(|n| n.chars().all(|c| c.is_ascii_digit()));
        let target = if process_entry {
            tcontext = "proc".into();
            "other processes' details in /proc".to_string()
        } else if tclass == "dbus" {
            format!(
                "{} over D-Bus",
                tcontext.strip_suffix("_t").unwrap_or(&tcontext)
            )
        } else {
            match (name, field(record, "path")) {
                (_, Some(path)) => path.to_string(),
                (Some(name), None) => format!("{name} ({tclass})"),
                (None, None) => format!("a {tclass} labelled {tcontext}"),
            }
        };
        let first = permissions.first().copied().unwrap_or("access");
        return Some(Denial {
            module: "selinux",
            confined_by: field(record, "scontext")
                .map(context_type)
                .and_then(policy_name),
            key: format!(
                "denial:selinux:{program}:{}:{tclass}:{tcontext}",
                permissions.join(",")
            ),
            action: verb(first).into(),
            allowed: field(record, "permissive") == Some("1"),
            program,
            target,
            record: record.into(),
        });
    }
    if record.contains("apparmor=\"DENIED\"") || record.contains("apparmor=\"ALLOWED\"") {
        let program = field(record, "comm").unwrap_or("A program").to_string();
        let mask = field(record, "denied_mask")
            .or_else(|| field(record, "requested_mask"))
            .unwrap_or("");
        let target = field(record, "name").unwrap_or("a resource").to_string();
        let profile = field(record, "profile").unwrap_or("");
        return Some(Denial {
            module: "apparmor",
            confined_by: None,
            key: format!("denial:apparmor:{profile}:{program}:{mask}:{target}"),
            action: verb(mask.get(0..1).unwrap_or("")).into(),
            allowed: record.contains("apparmor=\"ALLOWED\""),
            program,
            target,
            record: record.into(),
        });
    }
    None
}

pub fn known_noise(d: &Denial) -> bool {
    KNOWN_NOISE
        .iter()
        .any(|(program, permission)| d.program == *program && d.key.contains(permission))
}

/// "firefox, confined by the mozilla policy, tried to read .ssh (dir) and was stopped."
/// Program names stay as the system spells them.
pub fn sentence(d: &Denial) -> String {
    let outcome = if d.allowed {
        "and was allowed because the policy only logs it"
    } else {
        "and was stopped"
    };
    let noise = if known_noise(d) {
        " This one is known and harmless."
    } else {
        ""
    };
    let who = match &d.confined_by {
        Some(policy) if d.program == "A program" => {
            format!("A program confined by the {policy} policy")
        }
        Some(policy) => format!("{}, confined by the {policy} policy,", d.program),
        None => d.program.clone(),
    };
    format!("{who} tried to {} {} {outcome}.{noise}", d.action, d.target)
}

pub fn finding(r: &mut Report, d: &Denial) {
    let rule = if d.module == "selinux" {
        "denial.selinux"
    } else {
        "denial.apparmor"
    };
    r.add(
        rule,
        Severity::Info,
        &sentence(d),
        &d.record,
        crate::rules::get(rule).map_or("", |r| r.remediation),
    );
}
