//! A hardening change as a short list of steps. The same steps render the
//! preview and run, so what the preview prints is exactly what runs. Steps that
//! need an administrator run as one script through polkit, so there is one prompt.
use crate::runner::{self, Cancel};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "step", rename_all = "snake_case")]
pub enum Step {
    /// A program with fixed arguments.
    Run { argv: Vec<String> },
    /// Replace a file's contents (mode 0644 when Bigman runs it as administrator).
    Write { path: PathBuf, content: String },
    /// Delete a file Bigman wrote earlier; nothing happens when it is already gone.
    Remove { path: PathBuf },
}

/// What one fix (or its undo) does.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Change {
    /// Runs as root through polkit.
    pub admin: bool,
    pub steps: Vec<Step>,
}

pub fn run(argv: &[&str]) -> Step {
    Step::Run {
        argv: argv.iter().map(|a| a.to_string()).collect(),
    }
}

/// Single-quoted for /bin/sh.
pub fn quote(s: &str) -> String {
    if !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./=:@%+,".contains(c))
    {
        return s.into();
    }
    format!("'{}'", s.replace('\'', r"'\''"))
}

fn sh_line(step: &Step) -> String {
    match step {
        Step::Run { argv } => argv.iter().map(|a| quote(a)).collect::<Vec<_>>().join(" "),
        Step::Write { path, content } => format!(
            "printf '%s' {} > {}",
            quote(content),
            quote(&path.to_string_lossy())
        ),
        Step::Remove { path } => format!("rm -f -- {}", quote(&path.to_string_lossy())),
    }
}

impl Change {
    pub fn admin(steps: Vec<Step>) -> Self {
        Self { admin: true, steps }
    }
    pub fn user(steps: Vec<Step>) -> Self {
        Self {
            admin: false,
            steps,
        }
    }

    /// Who runs the steps, in one sentence for the preview's prose.
    pub fn who(&self) -> &'static str {
        if self.admin {
            "Polkit asks for your password, then these commands run as administrator."
        } else {
            "These commands run as you; no password is needed."
        }
    }

    /// The steps as a reader sees them: one command per line, file contents indented.
    pub fn describe(&self) -> String {
        let mut out = String::new();
        for step in &self.steps {
            match step {
                Step::Write { path, content } => {
                    out.push_str(&format!("write {}:\n", path.display()));
                    for line in content.lines() {
                        out.push_str(&format!("    {line}\n"));
                    }
                }
                other => {
                    out.push_str(&sh_line(other));
                    out.push('\n');
                }
            }
        }
        out.trim_end().into()
    }

    /// The command that performs an administrator change: the program itself
    /// when it is one command, otherwise one fixed script.
    pub fn admin_command(&self) -> Vec<String> {
        let mut cmd = vec!["pkexec".to_string()];
        match self.steps.as_slice() {
            [Step::Run { argv }] => cmd.extend(argv.iter().cloned()),
            steps => {
                let mut script = vec!["set -e".to_string(), "umask 022".to_string()];
                script.extend(steps.iter().map(sh_line));
                cmd.extend(["/bin/sh".into(), "-c".into(), script.join("\n")]);
            }
        }
        cmd
    }

    pub fn apply(&self, stop: &Cancel) -> Result<()> {
        if self.steps.is_empty() {
            bail!("Nothing to change");
        }
        if self.admin {
            let cmd = self.admin_command();
            let o = runner::run(Command::new(&cmd[0]).args(&cmd[1..]), 300, stop)?;
            if !o.ok() {
                // pkexec exits 126 when the password dialog is dismissed.
                if o.code == Some(126) || o.code == Some(127) {
                    bail!(
                        "Administrator approval was not given, so nothing changed.\n{}",
                        o.stderr.trim()
                    );
                }
                bail!(
                    "The change did not complete (exit {:?}).\n{}\n{}",
                    o.code,
                    o.stdout.trim(),
                    o.stderr.trim()
                );
            }
            return Ok(());
        }
        for step in &self.steps {
            match step {
                Step::Run { argv } => {
                    let o = runner::run(Command::new(&argv[0]).args(&argv[1..]), 60, stop)?;
                    if !o.ok() {
                        bail!(
                            "{} did not complete (exit {:?}).\n{}\n{}",
                            argv.join(" "),
                            o.code,
                            o.stdout.trim(),
                            o.stderr.trim()
                        );
                    }
                }
                Step::Write { path, content } => fs::write(path, content)
                    .with_context(|| format!("writing {}", path.display()))?,
                Step::Remove { path } => match fs::remove_file(path) {
                    Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
                        return Err(e).with_context(|| format!("removing {}", path.display()))
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_command_runs_directly_and_several_run_as_one_script() {
        let one = Change::admin(vec![run(&[
            "systemctl",
            "disable",
            "--now",
            "sshd.service",
        ])]);
        assert_eq!(
            one.admin_command(),
            ["pkexec", "systemctl", "disable", "--now", "sshd.service"]
        );
        let two = Change::admin(vec![
            Step::Write {
                path: "/etc/sysctl.d/90-bigman-ptrace-scope.conf".into(),
                content: "# it's Bigman's\nkernel.yama.ptrace_scope = 1\n".into(),
            },
            run(&["sysctl", "-w", "kernel.yama.ptrace_scope=1"]),
        ]);
        let cmd = two.admin_command();
        assert_eq!(&cmd[..3], ["pkexec", "/bin/sh", "-c"]);
        assert!(cmd[3].starts_with("set -e\numask 022\nprintf '%s' '# it'\\''s Bigman'\\''s\n"));
        assert!(cmd[3].ends_with("\nsysctl -w kernel.yama.ptrace_scope=1"));
        let syntax = std::process::Command::new("sh")
            .args(["-n", "-c", &cmd[3]])
            .status()
            .unwrap();
        assert!(syntax.success(), "the script parses");
        assert_eq!(
            two.describe(),
            "write /etc/sysctl.d/90-bigman-ptrace-scope.conf:\n    # it's Bigman's\n    kernel.yama.ptrace_scope = 1\nsysctl -w kernel.yama.ptrace_scope=1"
        );
    }

    #[test]
    fn your_own_changes_run_without_a_shell() {
        let d = tempfile::tempdir().unwrap();
        let file = d.path().join("x");
        let c = Change::user(vec![
            Step::Write {
                path: file.clone(),
                content: "a".into(),
            },
            run(&["true"]),
        ]);
        c.apply(&runner::cancel()).unwrap();
        assert_eq!(fs::read_to_string(&file).unwrap(), "a");
        Change::user(vec![Step::Remove { path: file.clone() }])
            .apply(&runner::cancel())
            .unwrap();
        assert!(!file.exists());
        Change::user(vec![Step::Remove { path: file }])
            .apply(&runner::cancel())
            .unwrap();
        assert_eq!(quote("a b"), "'a b'");
        assert_eq!(quote("kernel.x=1"), "kernel.x=1");
    }
}
