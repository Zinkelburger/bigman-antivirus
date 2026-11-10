//! Every state-changing workflow as an `Action`: `preview` says exactly what will
//! change without changing anything, `apply` does it. The CLI, the daemon and the
//! desktop app all go through these, so a preview can never drift from the change.
use crate::{
    checks::{
        canary, changes, disposable, hardening, hooks, inspect, packages, running, scan, setup,
        skills,
    },
    model::Report,
    runner::Cancel,
    store::Store,
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[ts(export)]
pub enum Action {
    /// Install an optional tool group from the distribution repositories via polkit.
    InstallTool {
        tool: String,
    },
    /// Install (or remove) the package-manager hook that previews receipts.
    ReceiptHook {
        remove: bool,
    },
    /// Join (or leave) systemd-journal so SELinux and AppArmor denials can be read.
    JournalAccess {
        remove: bool,
    },
    /// Install (or remove) the per-user monitor and daily scan/update units.
    Services {
        remove: bool,
    },
    /// Write `ignore-scripts=true` to a project's `.npmrc`, keeping a backup.
    PackagePolicy {
        path: PathBuf,
    },
    /// Save a reviewed startup inventory as the baseline.
    AcceptBaseline {
        report: String,
    },
    /// Put back the baseline the last acceptance replaced.
    RestorePreviousBaseline,
    /// Record the version of a skill or plugin an inventory report showed as
    /// reviewed. Refused when it changed since.
    ApproveSkill {
        report: String,
        key: String,
    },
    /// Take back a skill approval, restoring the one it replaced.
    RevokeSkillApproval {
        key: String,
    },
    /// Record a row of a Running system report as known to you, pinned to what
    /// it is now. Refused when it changed since the report.
    MarkKnown {
        report: String,
        key: String,
    },
    /// Take a mark back, restoring the one it replaced.
    ForgetKnown {
        key: String,
    },
    /// Move a scan finding into private quarantine.
    Quarantine {
        report: String,
        index: usize,
    },
    /// Put a quarantined file back at its original path.
    Restore {
        id: String,
    },
    /// Run an inspected installer, by exact hash, in the offline sandbox.
    Rehearse {
        report: String,
        sha256: String,
    },
    /// Open a terminal with a shell in a disposable container holding a copy of
    /// a folder or file.
    OpenContainer {
        path: PathBuf,
        offline: bool,
        port: Option<u16>,
    },
    /// Create (or reuse) a local decoy, start monitoring and verify a real read.
    ArmCanary {
        path: PathBuf,
        session_only: bool,
    },
    StartMonitor {
        session_only: bool,
    },
    StopMonitor {
        session_only: bool,
    },
    /// Save a provider's hosted token URL as a shortcut file.
    CreateHostedCanary {
        path: PathBuf,
        url: String,
    },
    /// Stop watching a decoy; the file stays on disk.
    DisarmCanary {
        id: String,
    },
    /// Register the browser extension's native-messaging host.
    RegisterNativeHost {
        extension_id: String,
    },
    /// Make a hardening suggestion's change. The daemon works out the change
    /// from the computer's current state; the id only says which suggestion.
    Harden {
        id: String,
    },
    /// Reverse a hardening change Bigman made, from its record.
    UndoHardening {
        id: String,
    },
    /// Stop (or resume) suggesting a hardening change.
    KeepSetting {
        id: String,
        keep: bool,
    },
}

/// What an action will do, shown before anything happens.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct ActionPlan {
    pub title: String,
    /// The confirm button's label: "Quarantine", "Install", "Accept".
    pub verb: String,
    /// One plain sentence: what changes.
    pub summary: String,
    /// Exact commands, file contents or paths, shown verbatim.
    pub details: String,
    /// How to undo it, when the subsystem allows.
    pub undo: Option<String>,
    /// True when the change runs code the user has not reviewed (the installer
    /// rehearsal); the UI asks for an explicit acknowledgment.
    pub careful: bool,
    /// Plain facts the decision rests on, shown above `details`.
    pub facts: Vec<PlanFact>,
    /// One plain sentence after the summary: a limit the reader should know.
    pub note: Option<String>,
    /// The label over `details`, when "Exactly what changes" does not fit.
    pub details_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct PlanFact {
    pub label: String,
    pub value: String,
}

/// What an action did.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ActionOutcome {
    pub message: String,
    pub report: Option<Report>,
    /// The action that reverses this one, offered as Undo in the result.
    pub undo: Option<Action>,
}

fn plan(
    title: &str,
    verb: &str,
    summary: impl Into<String>,
    details: impl Into<String>,
) -> ActionPlan {
    ActionPlan {
        title: title.into(),
        verb: verb.into(),
        summary: summary.into(),
        details: details.into(),
        undo: None,
        careful: false,
        facts: vec![],
        note: None,
        details_label: None,
    }
}

fn message(text: impl Into<String>) -> ActionOutcome {
    ActionOutcome {
        message: text.into(),
        report: None,
        undo: None,
    }
}

fn with_report(text: impl Into<String>, report: Report) -> ActionOutcome {
    ActionOutcome {
        report: Some(report),
        ..message(text)
    }
}

fn undoable(outcome: ActionOutcome, undo: Action) -> ActionOutcome {
    ActionOutcome {
        undo: Some(undo),
        ..outcome
    }
}

/// The startup entries a persistence report would accept, one per line.
fn baseline_entries(report: &Report) -> String {
    let label = |rule: &str| match rule {
        "persistence.change" | "persistence.package_update" => "Changed",
        "persistence.owned_modified" | "persistence.sensitive_modified" => {
            "Differs from its package"
        }
        "persistence.privilege" => "Gained privileges",
        "persistence.metadata" => "Permissions changed",
        "persistence.removed" => "Removed",
        "persistence.initial" => "Recorded",
        "persistence.extension-new" => "New extension",
        "persistence.extension-updated" => "Extension updated",
        "persistence.extension-modified" => "Extension code changed",
        "persistence.shadowed-command" => "Hides a system command",
        _ => "Entry",
    };
    // A first inventory lists every entry once; the attribution notes repeat paths.
    let initial = report
        .findings
        .iter()
        .any(|f| f.rule == "persistence.initial");
    report
        .findings
        .iter()
        .filter(|f| !initial || f.rule == "persistence.initial")
        .map(|f| {
            let path = f.evidence.lines().next().unwrap_or_default();
            format!("{}: {path}", label(&f.rule))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn scan_item(store: &Store, report: &str, index: usize) -> Result<scan::ScanItem> {
    let items = scan::inventory(store, report)?;
    let item = items.get(index).context("That scan item does not exist")?;
    if !item.detected {
        bail!("Only files with a scan finding can be quarantined");
    }
    Ok(item.clone())
}

fn quarantined(store: &Store, id: &str) -> Result<scan::Quarantined> {
    scan::quarantined(store)?
        .into_iter()
        .find(|q| q.id == id)
        .context("Unknown quarantine ID")
}

fn container_plan(
    path: &std::path::Path,
    offline: bool,
    port: Option<u16>,
) -> Result<disposable::Plan> {
    disposable::plan(
        path,
        disposable::Options {
            offline,
            ports: port.into_iter().collect(),
            ..Default::default()
        },
    )
}

fn paths(list: &[PathBuf]) -> String {
    list.iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn preview(action: &Action, store: &Store) -> Result<ActionPlan> {
    Ok(match action {
        Action::ReceiptHook { remove } => {
            let (manager, path, content, script) = hooks::plan(*remove)?;
            ActionPlan {
                undo: Some(if *remove {
                    "Turn the preview on again from Setup.".into()
                } else {
                    "Undo removes the hook file again.".into()
                }),
                ..plan(
                    if *remove { "Stop previewing package changes" } else { "Preview package changes before they happen" },
                    if *remove { "Remove the hook" } else { "Install the hook" },
                    if *remove {
                        format!("Delete {}. {} stops asking Bigman before a transaction; receipts afterwards continue.", path.display(), manager.name())
                    } else {
                        format!(
                            "Add a hook so {} shows each transaction's receipt in Bigman first, with Continue and Abort. It continues on its own after a minute, so unattended upgrades never wait, and it continues whenever Bigman is not running or cannot read a package. Administrator approval is asked for by polkit.",
                            manager.name()
                        )
                    },
                    if *remove {
                        format!("pkexec /bin/sh -c \"{script}\"")
                    } else {
                        format!("{}\n\n{content}\npkexec /bin/sh -c \"{script}\"", path.display())
                    },
                )
            }
        }
        Action::Harden { id } => {
            let (s, change, _) = hardening::find(store, id, &crate::runner::cancel())?;
            let verb = match &s.fix {
                hardening::Fix::Bigman { verb, .. } => verb.clone(),
                _ => "Change".into(),
            };
            ActionPlan {
                undo: Some(format!(
                    "Offered right after the change, and later under Changed by Bigman (bigman harden {id} --undo), each with its own preview."
                )),
                // Paragraphs: what it costs, then who runs the commands below.
                ..plan(
                    &s.title,
                    &verb,
                    format!("{}\n\n{}", s.tradeoff, change.who()),
                    change.describe(),
                )
            }
        }
        Action::UndoHardening { id } => {
            let a = hardening::applied_change(store, id)?;
            plan(
                &format!("Undo: {}", a.title),
                "Undo",
                format!(
                    "Put the setting back the way it was before Bigman changed it.\n\n{}",
                    a.revert.who()
                ),
                a.undo,
            )
        }
        Action::KeepSetting { id, keep } => {
            let title = hardening::review(store, &crate::runner::cancel())?
                .suggestions
                .into_iter()
                .find(|s| s.id == *id)
                .map(|s| s.title)
                .context("This suggestion no longer applies: the setting may have changed since you last looked")?;
            ActionPlan {
                undo: Some(if *keep {
                    format!("Suggest again… under Left as it is (bigman harden {id} --suggest-again).")
                } else {
                    format!("Leave as it is… on the suggestion (bigman harden {id} --keep).")
                }),
                ..plan(
                    &if *keep {
                        format!("Stop suggesting: {title}")
                    } else {
                        format!("Suggest again: {title}")
                    },
                    if *keep { "Stop suggesting" } else { "Suggest again" },
                    if *keep {
                        "Bigman stops counting this suggestion and lists it under Left as it is. Nothing on this computer changes."
                    } else {
                        "Bigman counts this suggestion again. Nothing on this computer changes."
                    },
                    "",
                )
            }
        },
        Action::JournalAccess { remove } => {
            let command = setup::journal_group_plan(*remove)?;
            ActionPlan {
                undo: Some(if *remove {
                    "Join the group again from Setup.".into()
                } else {
                    "Undo removes you from the group again.".into()
                }),
                ..plan(
                    if *remove { "Stop reading the system journal" } else { "Read SELinux and AppArmor denials" },
                    if *remove { "Leave the group" } else { "Join the group" },
                    if *remove {
                        "Remove your account from systemd-journal. Bigman stops collecting denials from your next login."
                    } else {
                        "Add your account to systemd-journal, the group that may read the system journal. Bigman then turns denials into plain sentences in Activity. Your account can also read other system logs; nothing gains write access. Administrator approval is asked for by polkit, and it takes effect at your next login."
                    },
                    command.join(" "),
                )
            }
        }
        Action::InstallTool { tool } => {
            let command = setup::tool_plan(tool)?;
            ActionPlan {
                undo: Some("Remove the packages with your package manager if you no longer want them.".into()),
                ..plan(
                    "Install optional tools",
                    "Install",
                    format!(
                        "Install {} from your distribution's configured repositories. Administrator approval is asked for by polkit.",
                        command[4..].join(", ")
                    ),
                    command.join(" "),
                )
            }
        }
        Action::Services { remove } => ActionPlan {
            undo: Some(if *remove {
                "Install the services again from Setup.".into()
            } else {
                "Undo removes them again.".into()
            }),
            ..plan(
                if *remove { "Remove background services" } else { "Install background services" },
                if *remove { "Remove" } else { "Install" },
                if *remove {
                    "Stop and remove Bigman's per-user collector, monitor and daily update units. Reports, the timeline and canary files stay."
                } else {
                    "Write and enable per-user units: the collector daemon that keeps Activity and Network history while the window is closed, which also checks each new file in Downloads as it arrives, the canary and startup monitor, and daily signature updates. Nothing system-wide."
                },
                setup::services(store, false, *remove, &crate::runner::cancel())?,
            )
        },
        Action::PackagePolicy { path } => ActionPlan {
            undo: Some("Delete the added lines from .npmrc; an existing file is backed up next to it first.".into()),
            ..plan(
                "Disable automatic install scripts",
                "Write .npmrc",
                "Add ignore-scripts=true to this project's .npmrc so npm and pnpm do not run lifecycle scripts during install.",
                packages::policy(path, false)?,
            )
        },
        Action::AcceptBaseline { report } => {
            let r = store.report(report)?;
            if r.kind != "Persistence changes" || !r.errors.is_empty() {
                bail!("Choose a complete startup inventory report first");
            }
            let recorded = r.findings.iter().filter(|f| f.rule == "persistence.initial").count();
            let changed = if recorded > 0 { 0 } else { r.findings.len() };
            ActionPlan {
                undo: Some(if store.root.join("baseline.json").exists() {
                    "Bigman keeps the baseline this replaces; Undo puts it back.".into()
                } else {
                    "Compare again and accept a newer inventory at any time.".into()
                }),
                ..plan(
                    "Accept startup baseline",
                    "Accept",
                    if changed > 0 {
                        let entries = if changed == 1 { "the changed startup entry".to_string() } else { format!("the {changed} changed startup entries") };
                        format!("Treat {entries} below as expected from now on. Accept only if you know why it changed. Bigman refuses if anything changed since this report.")
                    } else {
                        format!("Record these {recorded} startup entries as the baseline to compare against. Bigman refuses if anything changed since this report.")
                    },
                    baseline_entries(&r),
                )
            }
        }
        Action::ApproveSkill { report, key } => {
            let (summary, details) = skills::approval_plan(store, report, key)?;
            ActionPlan {
                undo: Some("Undo takes the approval back, restoring any earlier one.".into()),
                ..plan("Approve this version", "Approve", summary, details)
            }
        }
        Action::RevokeSkillApproval { key } => plan(
            "Take back the approval",
            "Take back",
            format!(
                "Stop treating {} as reviewed. An earlier approval comes back if there was one; otherwise it is listed as not reviewed.",
                skills::approved_name(store, key).unwrap_or_else(|| "this skill or plugin".into())
            ),
            key.clone(),
        ),
        Action::MarkKnown { report, key } => {
            let (summary, details, note) = running::mark_plan(store, report, key)?;
            ActionPlan {
                undo: Some("Take the mark back at any time; an earlier mark is restored.".into()),
                note,
                details_label: Some("What the mark records".into()),
                ..plan("Mark as known", "Mark as known", summary, details)
            }
        }
        Action::ForgetKnown { key } => plan(
            "Take back the mark",
            "Take back",
            format!(
                "Stop treating \"{}\" as something you recognise. An earlier mark comes back if there was one; otherwise it counts as something to review again.",
                crate::checks::known::title(store, key).unwrap_or_else(|| "this row".into())
            ),
            "",
        ),
        Action::RestorePreviousBaseline => plan(
            "Restore the previous baseline",
            "Restore",
            "Put back the startup baseline that the last acceptance replaced.",
            "",
        ),
        Action::Quarantine { report, index } => {
            let item = scan_item(store, report, *index)?;
            ActionPlan {
                undo: Some("Undo restores it, as long as nothing new has taken its path.".into()),
                ..plan(
                    "Quarantine a file",
                    "Quarantine",
                    "Save a private recovery copy, check the file is unchanged since the scan, then remove it from its folder. No other file is touched.",
                    format!(
                        "{}\nSignature {}\nSHA-256 {}",
                        item.path.display(),
                        item.signature,
                        item.hash
                    ),
                )
            }
        }
        Action::Restore { id } => {
            let q = quarantined(store, id)?;
            plan(
                "Restore a quarantined file",
                "Restore",
                "Write the recovery copy back to its original path with permissions 0600. Bigman refuses to overwrite an existing file. This file was flagged by a scan.",
                format!("{}\nSHA-256 {}", q.original.display(), q.hash),
            )
        }
        Action::Rehearse { report, sha256 } => {
            let r = store.report(report)?;
            ActionPlan {
                careful: true,
                ..plan(
                    "Rehearse the installer",
                    "Run in the sandbox",
                    "Run the saved installer in a disposable offline sandbox with no home directory, credentials or network. It may fail because of these limits. This runs the code; it does not install anything on your system.",
                    format!("{}\nSHA-256 {sha256}", r.subject),
                )
            }
        }
        Action::OpenContainer {
            path,
            offline,
            port,
        } => {
            let p = container_plan(path, *offline, *port)?;
            if !crate::runner::available("podman") {
                bail!("Podman is not installed. Install it from Setup, under Podman.");
            }
            ActionPlan {
                facts: p
                    .facts()
                    .into_iter()
                    .map(|(label, value)| PlanFact { label, value })
                    .collect(),
                note: Some(disposable::KERNEL.into()),
                details_label: Some("The commands Bigman runs".into()),
                ..plan(
                    "Open in a disposable container",
                    "Open in a disposable container",
                    p.summary(),
                    p.commands(true),
                )
            }
        }
        Action::ArmCanary { path, session_only } => ActionPlan {
            undo: Some("Undo stops watching the decoy; the file stays where it is.".into()),
            ..plan(
                "Set up a canary file",
                "Create and test",
                if *session_only {
                    "Create a decoy file (or reuse a registered one), start monitoring until you log out, and read it once to prove detection works."
                } else {
                    "Create a decoy file (or reuse a registered one), start monitoring now and at each login, and read it once to prove detection works."
                },
                format!(
                    "{}\nThe file contains a synthetic token, not a working credential. Existing files are never overwritten.",
                    path.display()
                ),
            )
        },
        Action::StartMonitor { session_only } => plan(
            "Start canary monitoring",
            "Start",
            if *session_only {
                "Start the monitor for this session for the registered decoys. No files are created."
            } else {
                "Start the monitor for the registered decoys and enable it at login. No files are created."
            },
            paths(&canary::list(store)?.into_iter().map(|c| c.path).collect::<Vec<_>>()),
        ),
        Action::StopMonitor { session_only } => ActionPlan {
            undo: Some("Start monitoring again from Protect > Canaries.".into()),
            ..plan(
                "Stop canary monitoring",
                "Stop",
                if *session_only {
                    "Stop the running monitor. The login service, if enabled, starts it again at the next login."
                } else {
                    "Stop the monitor and remove its login service. Decoy files and history stay. Reads while stopped are not noticed."
                },
                "",
            )
        },
        Action::CreateHostedCanary { path, url } => plan(
            "Save a hosted canary shortcut",
            "Save shortcut",
            "Write a shortcut file that points at your provider's token URL. Bigman does not visit the URL.",
            format!("{}\n{url}", path.display()),
        ),
        Action::DisarmCanary { id } => {
            let c = canary::list(store)?
                .into_iter()
                .find(|c| &c.id == id)
                .context("Unknown canary ID")?;
            plan(
                "Stop watching a decoy",
                "Stop watching",
                "Remove this decoy from Bigman's list. The file itself stays where it is.",
                c.path.display().to_string(),
            )
        }
        Action::RegisterNativeHost { extension_id } => ActionPlan {
            undo: Some("Delete the manifest files listed above.".into()),
            ..plan(
                "Connect the browser extension",
                "Connect",
                "Write native-messaging host manifests that let only this extension ID talk to Bigman.",
                format!("Extension {extension_id}\n{}", paths(&setup::native_manifest_paths())),
            )
        },
    })
}

pub fn apply(action: &Action, store: &Store, stop: &Cancel) -> Result<ActionOutcome> {
    Ok(match action {
        Action::Harden { id } => undoable(
            message(hardening::apply(store, id, stop)?),
            Action::UndoHardening { id: id.clone() },
        ),
        Action::UndoHardening { id } => message(hardening::undo(store, id, stop)?),
        Action::KeepSetting { id, keep } => {
            hardening::keep(store, id, *keep)?;
            undoable(
                message(if *keep {
                    "Left as it is. Bigman no longer counts it."
                } else {
                    "Bigman suggests it again."
                }),
                Action::KeepSetting {
                    id: id.clone(),
                    keep: !*keep,
                },
            )
        }
        Action::InstallTool { tool } => message(setup::install_tool(tool, true, stop)?),
        Action::ReceiptHook { remove } => {
            let done = message(hooks::apply(*remove, stop)?);
            if *remove {
                done
            } else {
                undoable(done, Action::ReceiptHook { remove: true })
            }
        }
        Action::JournalAccess { remove } => {
            let done = message(setup::journal_group(*remove, stop)?);
            if *remove {
                done
            } else {
                undoable(done, Action::JournalAccess { remove: true })
            }
        }
        Action::Services { remove } => {
            let done = message(setup::services(store, true, *remove, stop)?);
            if *remove {
                done
            } else {
                undoable(done, Action::Services { remove: true })
            }
        }
        Action::PackagePolicy { path } => message(packages::policy(path, true)?),
        Action::AcceptBaseline { report } => {
            let saved = with_report(
                "Baseline saved.",
                changes::accept_reviewed_baseline(store, report)?,
            );
            if changes::has_previous_baseline(store) {
                undoable(saved, Action::RestorePreviousBaseline)
            } else {
                saved
            }
        }
        Action::ApproveSkill { report, key } => undoable(
            message(skills::approve(store, report, key)?),
            Action::RevokeSkillApproval { key: key.clone() },
        ),
        Action::RevokeSkillApproval { key } => message(skills::revoke(store, key)?),
        Action::MarkKnown { report, key } => undoable(
            message(running::mark(store, report, key, stop)?),
            Action::ForgetKnown { key: key.clone() },
        ),
        Action::ForgetKnown { key } => message(running::forget(store, key)?),
        Action::RestorePreviousBaseline => {
            changes::restore_previous_baseline(store)?;
            message("The previous baseline is back.")
        }
        Action::Quarantine { report, index } => {
            let q = scan::quarantine(store, report, *index)?;
            undoable(
                message(format!(
                    "A recovery copy is saved and {} was removed from its folder.",
                    q.original.display()
                )),
                Action::Restore { id: q.id },
            )
        }
        Action::Restore { id } => message(format!(
            "Restored without executable permissions: {}",
            scan::restore(store, id)?.display()
        )),
        Action::Rehearse { report, sha256 } => with_report(
            "Rehearsal finished.",
            inspect::sandbox(store, report, sha256, stop)?,
        ),
        Action::OpenContainer {
            path,
            offline,
            port,
        } => message(disposable::open_in_terminal(
            store,
            &container_plan(path, *offline, *port)?,
        )?),
        Action::ArmCanary { path, session_only } => {
            let report = canary::setup::arm(store, path, *session_only, stop)?;
            let armed = canary::list(store)?
                .into_iter()
                .find(|c| report.subject == c.path.display().to_string());
            let done = with_report("Canary setup finished.", report);
            match armed {
                Some(c) => undoable(done, Action::DisarmCanary { id: c.id }),
                None => done,
            }
        }
        Action::StartMonitor { session_only } => with_report(
            "Monitor startup finished.",
            canary::setup::start_monitor(store, *session_only, stop)?,
        ),
        Action::StopMonitor { session_only } => with_report(
            "Monitor stopped.",
            canary::setup::stop_monitor(store, *session_only, stop)?,
        ),
        Action::CreateHostedCanary { path, url } => {
            let c = canary::create(store, path, Some(url))?;
            message(format!("Shortcut saved at {}", c.path.display()))
        }
        Action::DisarmCanary { id } => {
            canary::disarm(store, id)?;
            message("Decoy removed from the list; its file was kept.")
        }
        Action::RegisterNativeHost { extension_id } => {
            let manifests = setup::native_manifest_paths();
            for p in &manifests {
                setup::write_native_manifest(p, extension_id)?;
            }
            message(format!(
                "Registered {}",
                paths(&manifests).replace('\n', ", ")
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn previews_change_nothing() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let project = d.path().join("project");
        std::fs::create_dir(&project).unwrap();
        let action = Action::PackagePolicy {
            path: project.clone(),
        };
        let p = preview(&action, &store).unwrap();
        assert!(p.details.contains("Would update"));
        assert!(!project.join(".npmrc").exists());
        apply(&action, &store, &crate::runner::cancel()).unwrap();
        assert!(std::fs::read_to_string(project.join(".npmrc"))
            .unwrap()
            .contains("ignore-scripts=true"));
        let missing = Action::Quarantine {
            report: uuid::Uuid::new_v4().to_string(),
            index: 0,
        };
        assert!(preview(&missing, &store).is_err());
    }

    #[test]
    fn accepting_a_baseline_can_be_undone() {
        let d = tempfile::tempdir().unwrap();
        let store = Store::open(Some(d.path().join("state"))).unwrap();
        let stop = crate::runner::cancel();
        let first = changes::changes(&store, &crate::runner::cancel()).unwrap();
        store.save(&first).unwrap();
        let accept = Action::AcceptBaseline { report: first.id };
        assert!(
            apply(&accept, &store, &stop).unwrap().undo.is_none(),
            "nothing to undo to yet"
        );
        let second = changes::changes(&store, &crate::runner::cancel()).unwrap();
        store.save(&second).unwrap();
        let outcome = apply(&Action::AcceptBaseline { report: second.id }, &store, &stop).unwrap();
        assert_eq!(outcome.undo, Some(Action::RestorePreviousBaseline));
        apply(&Action::RestorePreviousBaseline, &store, &stop).unwrap();
        assert!(!changes::has_previous_baseline(&store));
    }
}
