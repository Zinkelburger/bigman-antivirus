use crate::{
    runner::{self, Cancel},
    store::{home, Store},
};
use anyhow::{bail, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
pub fn downloads() -> PathBuf {
    home().join("Downloads")
}
pub fn tool_plan(tool: &str) -> Result<Vec<String>> {
    let os = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let fedora = os.lines().any(|l| l == "ID=fedora");
    let debian = os.lines().any(|l| l == "ID=ubuntu" || l == "ID=debian");
    let packages:Vec<&str>=match tool{"clamav"=>vec!["clamav"],"shellcheck"=>vec!["ShellCheck"],"sandbox"=>vec!["bubblewrap","strace"],"lynis"=>vec!["lynis"],"pdf"=>vec!["poppler-utils"],"firewall"=>vec!["opensnitch"],_=>bail!("Install this optional tool using its upstream instructions; no verified package mapping is configured.")};
    if !fedora && !debian {
        bail!("Automatic package setup supports Fedora, Debian and Ubuntu. Use your distribution's package manager on this system.");
    }
    let mut cmd = if fedora {
        vec!["pkexec", "dnf", "install", "-y"]
    } else {
        vec!["pkexec", "apt-get", "install", "-y"]
    }
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();
    for p in packages {
        cmd.push(
            if debian && p == "ShellCheck" {
                "shellcheck"
            } else {
                p
            }
            .into(),
        );
    }
    Ok(cmd)
}
pub fn install_tool(tool: &str, apply: bool, stop: &Cancel) -> Result<String> {
    let plan = tool_plan(tool)?;
    if !apply {
        return Ok(format!("Install from configured distribution repositories:\n{}\nAdministrator authentication will be handled by polkit.",plan.join(" ")));
    }
    let o = runner::run(Command::new(&plan[0]).args(&plan[1..]), 600, stop)?;
    if !o.ok() {
        bail!(
            "Installation did not complete: {:?}\n{}\n{}",
            o.code,
            o.stdout,
            o.stderr
        );
    }
    Ok(format!("{}\n{}", o.stdout, o.stderr))
}
fn quote(s: &str) -> Result<String> {
    if s.contains(['\n', '\r', '\0']) {
        bail!("Invalid service argument");
    }
    Ok(format!(
        "\"{}\"",
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('%', "%%")
            .replace('$', "$$")
    ))
}
pub fn service_plan(store: &Store) -> Result<Vec<(String, String)>> {
    let exe = std::env::current_exe()?;
    let base = format!(
        "{} --state {}",
        quote(&exe.to_string_lossy())?,
        quote(&store.root.to_string_lossy())?
    );
    let monitor=format!("[Unit]\nDescription=Bigman local canary and persistence monitor\n[Service]\nType=simple\nExecStart={base} watch\nRestart=on-failure\nRestartSec=5\nNoNewPrivileges=true\nUMask=0077\n[Install]\nWantedBy=default.target\n");
    let scan=format!("[Unit]\nDescription=Bigman daily Downloads scan\n[Service]\nType=oneshot\nExecStart={base} scan {} --notify\nNoNewPrivileges=true\nUMask=0077\n",quote(&downloads().to_string_lossy())?);
    let timer="[Unit]\nDescription=Daily Bigman scan\n[Timer]\nOnCalendar=daily\nPersistent=true\nRandomizedDelaySec=15m\n[Install]\nWantedBy=timers.target\n";
    let update=format!("[Unit]\nDescription=Bigman private signature update\n[Service]\nType=oneshot\nExecStart={base} update\nNoNewPrivileges=true\nUMask=0077\n");
    let update_timer="[Unit]\nDescription=Daily Bigman definitions update\n[Timer]\nOnCalendar=daily\nPersistent=true\nRandomizedDelaySec=5m\n[Install]\nWantedBy=timers.target\n";
    Ok(vec![
        ("bigman-update.service".into(), update),
        ("bigman-update.timer".into(), update_timer.into()),
        ("bigman-monitor.service".into(), monitor),
        ("bigman-scan.service".into(), scan),
        ("bigman-scan.timer".into(), timer.into()),
    ])
}
pub fn services(store: &Store, apply: bool, remove: bool, stop: &Cancel) -> Result<String> {
    let plan = service_plan(store)?;
    let root = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
        .join("systemd/user");
    if !apply {
        return Ok(format!(
            "{} per-user units in {}\n{}",
            if remove {
                "Remove"
            } else {
                "Install and enable"
            },
            root.display(),
            plan.iter()
                .map(|(n, c)| format!("\n{n}\n{c}"))
                .collect::<String>()
        ));
    }
    fs::create_dir_all(&root)?;
    if remove {
        let o = runner::run(
            Command::new("systemctl").args([
                "--user",
                "disable",
                "--now",
                "bigman-monitor.service",
                "bigman-scan.timer",
                "bigman-update.timer",
            ]),
            30,
            stop,
        )?;
        if !o.ok() {
            bail!("Could not disable units: {}", o.stderr);
        }
        for (n, _) in &plan {
            let p = root.join(n);
            if p.exists() {
                fs::remove_file(p)?;
            }
        }
    } else {
        for (n, c) in &plan {
            let p = root.join(n);
            if p.exists() {
                let existing = fs::read_to_string(&p)?;
                if existing != *c {
                    fs::copy(&p, root.join(format!("{n}.{}.bak", crate::model::now())))?;
                }
            }
            let mut tmp = tempfile::NamedTempFile::new_in(&root)?;
            use std::io::Write;
            tmp.write_all(c.as_bytes())?;
            tmp.persist(p).map_err(|e| e.error)?;
        }
    }
    let o = runner::run(
        Command::new("systemctl").args(["--user", "daemon-reload"]),
        30,
        stop,
    )?;
    if !o.ok() {
        bail!("Units written, but daemon reload failed: {}", o.stderr);
    }
    if !remove {
        let o = runner::run(
            Command::new("systemctl").args([
                "--user",
                "enable",
                "--now",
                "bigman-monitor.service",
                "bigman-scan.timer",
                "bigman-update.timer",
            ]),
            30,
            stop,
        )?;
        if !o.ok() {
            bail!("Units written, but enable failed: {}", o.stderr);
        }
    }
    Ok(if remove{"User services removed"}else{"User monitoring, daily scan and definition updates enabled. Services follow your user session; no system-wide service or lingering was enabled."}.into())
}
pub fn open_tool(tool: &str) -> Result<()> {
    match tool {
        "firewall" => {
            Command::new("opensnitch-ui").spawn()?;
        }
        _ => bail!("Unsupported application"),
    };
    Ok(())
}
pub fn write_native_manifest(path: &Path, extension: &str) -> Result<()> {
    if extension.len() != 32 || !extension.bytes().all(|c| (b'a'..=b'p').contains(&c)) {
        bail!("Chrome extension ID must contain exactly 32 letters a–p");
    }
    let manifest = serde_json::json!({"name":"com.bigman.pdf_scanner","description":"Bigman deterministic local security checks","path":std::env::current_exe()?,"type":"stdio","allowed_origins":[format!("chrome-extension://{extension}/")]});
    fs::create_dir_all(
        path.parent()
            .ok_or_else(|| anyhow::anyhow!("Missing parent"))?,
    )?;
    fs::write(path, serde_json::to_vec_pretty(&manifest)?)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn service_argument_escaping() {
        assert_eq!(quote("/home/a%/my tool").unwrap(), "\"/home/a%%/my tool\"");
        assert!(quote("bad\nExecStart=x").is_err());
    }
}
