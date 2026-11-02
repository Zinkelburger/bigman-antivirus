//! The opt-in pre-transaction hooks (plan Part F, "Active"): one file per package
//! manager that makes it run `bigman receipt --preview` as root before a
//! transaction commits. Installing one is a previewed action approved through
//! polkit; removing it is the same file deleted.
use super::preview::Manager;
use crate::runner::{self, Cancel};
use anyhow::{bail, Context, Result};
use std::{
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::Command,
};

/// Where each manager reads hooks from.
pub fn hook_path(manager: Manager) -> PathBuf {
    PathBuf::from(match manager {
        Manager::Dnf => "/etc/dnf/libdnf5-plugins/actions.d/bigman-preview.actions",
        Manager::Apt => "/etc/apt/apt.conf.d/80bigman-preview",
        Manager::Pacman => "/etc/pacman.d/hooks/bigman-preview.hook",
    })
}

/// The hook file for `manager`, calling the system-wide `bigman` at `exe`.
pub fn hook_file(manager: Manager, exe: &Path) -> String {
    let exe = exe.display();
    match manager {
        // One action per incoming package records it; the last one previews them all
        // (dnf5 gives no local path, so the archives are found in its cache).
        Manager::Dnf => format!(
            "# Bigman install-receipt preview (opt-in from Bigman's Setup). Delete this file to turn it off.\n\
             pre_transaction:*:in::{exe} receipt --stage ${{pkg.full_nevra}} ${{pkg.repo_id}} ${{pkg.location}} ${{pkg.action}}\n\
             pre_transaction:::raise_error=1:{exe} receipt --preview --manager dnf\n"
        ),
        Manager::Apt => format!(
            "// Bigman install-receipt preview (opt-in from Bigman's Setup). Delete this file to turn it off.\n\
             DPkg::Pre-Install-Pkgs {{ \"{exe} receipt --preview --manager apt\"; }};\n"
        ),
        Manager::Pacman => format!(
            "# Bigman install-receipt preview (opt-in from Bigman's Setup). Delete this file to turn it off.\n\
             [Trigger]\nOperation = Install\nOperation = Upgrade\nType = Package\nTarget = *\n\n\
             [Action]\nDescription = Bigman install-receipt preview\nWhen = PreTransaction\n\
             Exec = {exe} receipt --preview --manager pacman\nNeedsTargets\nAbortOnFail\n"
        ),
    }
}

/// The package manager this system uses, from the package database Bigman reads.
pub fn manager() -> Option<Manager> {
    match crate::platform::packagedb::package_db()?.manager() {
        "rpm" if runner::available("dnf5") || runner::available("dnf") => Some(Manager::Dnf),
        "dpkg" if runner::available("apt-get") => Some(Manager::Apt),
        "pacman" => Some(Manager::Pacman),
        _ => None,
    }
}

/// The package manager runs the hook as root, so the program it calls must not be
/// changeable by anyone but root: otherwise any process running as you could edit
/// it and gain root at the next update.
pub fn trusted_binary(exe: &Path) -> Result<()> {
    let mut path = Some(exe);
    while let Some(p) = path {
        let meta = p
            .metadata()
            .with_context(|| format!("Cannot inspect {}", p.display()))?;
        if meta.uid() != 0 || meta.mode() & 0o022 != 0 {
            bail!(
                "The package manager runs this hook as root, so Bigman must be installed system-wide (owned by root, not writable by others). {} is {}.",
                p.display(),
                if meta.uid() != 0 { "not owned by root" } else { "writable by others" }
            );
        }
        path = p.parent().filter(|parent| !parent.as_os_str().is_empty());
    }
    Ok(())
}

pub fn installed() -> Option<(Manager, PathBuf)> {
    [Manager::Dnf, Manager::Apt, Manager::Pacman]
        .into_iter()
        .map(|m| (m, hook_path(m)))
        .find(|(_, p)| p.exists())
}

fn quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}

/// The one root command that installs (or removes) the hook, shown in the preview.
pub fn plan(remove: bool) -> Result<(Manager, PathBuf, String, String)> {
    let manager = manager().context("Bigman found no dnf, apt or pacman to add a hook to")?;
    let path = hook_path(manager);
    if remove {
        return Ok((
            manager,
            path.clone(),
            String::new(),
            format!("rm -f {}", quote(&path.to_string_lossy())),
        ));
    }
    let exe = runner::bigman_executable()?;
    let exe = exe.canonicalize().unwrap_or(exe);
    trusted_binary(&exe)?;
    let content = hook_file(manager, &exe);
    let mut script = String::new();
    if manager == Manager::Dnf && !rpm_installed("libdnf5-plugin-actions") {
        script.push_str("dnf install -y libdnf5-plugin-actions && ");
    }
    script.push_str(&format!(
        "install -D -m 0644 -o root -g root /dev/stdin {}",
        quote(&path.to_string_lossy())
    ));
    Ok((manager, path, content, script))
}

fn rpm_installed(name: &str) -> bool {
    runner::run(
        Command::new("rpm").args(["-q", name]),
        10,
        &runner::cancel(),
    )
    .is_ok_and(|o| o.ok())
}

/// Runs the plan through pkexec; the hook text goes in on standard input.
pub fn apply(remove: bool, stop: &Cancel) -> Result<String> {
    let (manager, path, content, script) = plan(remove)?;
    let mut cmd = Command::new("pkexec");
    cmd.args(["/bin/sh", "-c", &script]);
    let out = runner::run_with_input(&mut cmd, content.as_bytes(), 600, stop)?;
    if !out.ok() {
        bail!(
            "The hook was not {}: {}",
            if remove { "removed" } else { "installed" },
            out.stderr.trim()
        );
    }
    Ok(if remove {
        format!(
            "Removed {}. {} no longer asks Bigman before a transaction.",
            path.display(),
            manager.name()
        )
    } else {
        format!(
            "Installed {}. Before each transaction {} now shows the receipt in Bigman and waits up to a minute for Continue or Abort.",
            path.display(),
            manager.name()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hook_files_call_the_given_binary() {
        let exe = Path::new("/usr/bin/bigman");
        let dnf = hook_file(Manager::Dnf, exe);
        assert!(dnf.contains("pre_transaction:*:in::/usr/bin/bigman receipt --stage ${pkg.full_nevra} ${pkg.repo_id} ${pkg.location} ${pkg.action}"));
        assert!(dnf.contains(
            "pre_transaction:::raise_error=1:/usr/bin/bigman receipt --preview --manager dnf"
        ));
        assert!(hook_file(Manager::Apt, exe).contains(
            "DPkg::Pre-Install-Pkgs { \"/usr/bin/bigman receipt --preview --manager apt\"; };"
        ));
        let pacman = hook_file(Manager::Pacman, exe);
        assert!(
            pacman.contains("When = PreTransaction")
                && pacman.contains("NeedsTargets")
                && pacman.contains("AbortOnFail")
        );
    }

    #[test]
    fn a_binary_you_can_change_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("bigman");
        std::fs::write(&exe, "").unwrap();
        let err = trusted_binary(&exe).unwrap_err().to_string();
        assert!(err.contains("must be installed system-wide"), "{err}");
        assert!(
            trusted_binary(Path::new("/usr/bin/sh")).is_ok() || !Path::new("/usr/bin/sh").exists()
        );
    }
}
