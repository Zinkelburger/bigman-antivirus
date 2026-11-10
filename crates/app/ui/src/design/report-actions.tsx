// What a reader can do with a report, wherever it opens (the tab that ran it,
// Activity, a recent list): the same follow-ups for the same report.
import { useState } from "react";
import type { Quarantined, Report, ScanItem } from "@/ipc";
import { useQuarantined, useScanItems, useToolGroups } from "@/ipc";
import { ActionDialog } from "./action";
import { Disclosure } from "./components/Disclosure";
import { EvidenceBlock } from "./components/EvidenceBlock";
import { KeyValue } from "./components/KeyValue";
import { TextField } from "./components/TextField";
import { Toggle } from "./components/Toggle";
import { plural } from "./format";

/** The quarantine entry holding this scanned file, if it is still in quarantine. */
/** What to look at before accepting a startup-files baseline: the rated entries first. */
function BaselineAdvice({ report }: { report: Report }) {
  const high = report.findings.filter((f) => f.severity === "high").length;
  const review = report.findings.filter((f) => f.severity === "review").length;
  if (high + review === 0) {
    return (
      <p className="m-0 max-w-[56ch] text-ink-muted">
        Look through it for anything you do not recognise, such as a program started from your home folder, Downloads
        or /tmp, or an SSH key you did not add.
      </p>
    );
  }
  const rated = [high ? `${String(high)} High` : "", review ? `${String(review)} Review` : ""].filter(Boolean).join(" and ");
  return (
    <p className="m-0 max-w-[56ch] text-ink">
      {plural(high + review, "entry")} {high + review === 1 ? "is" : "are"} rated {rated}. Read {high + review === 1 ? "it" : "them"}{" "}
      above before you accept. If you do not recognise one, search its name, or ask whoever set up this computer.
    </p>
  );
}

export function heldIn(quarantine: readonly Quarantined[] | undefined, item: ScanItem): Quarantined | undefined {
  return quarantine?.find((q) => !q.restored && q.original === item.path && q.hash === item.hash);
}

/** Files a scan flagged, each with its own previewed quarantine, or restore once quarantined. */
function FlaggedFiles({ report }: { report: string }) {
  const items = useScanItems(report);
  const quarantine = useQuarantined();
  const flagged = (items.data ?? []).map((item, index) => ({ item, index })).filter(({ item }) => item.detected);
  if (flagged.length === 0) return null;
  return (
    <section aria-label="Flagged files" className="flex flex-col gap-3">
      <h3 className="m-0 text-small font-semibold text-ink-muted">Flagged files</h3>
      {flagged.map(({ item, index }) => (
        <div key={item.hash + item.path} className="flex flex-col gap-3 rounded-panel border border-rule p-3">
          <KeyValue
            items={[
              { label: "File", value: item.path, mono: true },
              { label: "Signature", value: item.signature, mono: true },
              { label: "SHA-256", value: item.hash, mono: true },
            ]}
          />
          <QuarantineControl report={report} index={index} held={heldIn(quarantine.data, item)} />
        </div>
      ))}
    </section>
  );
}

function QuarantineControl({ report, index, held }: { report: string; index: number; held: Quarantined | undefined }) {
  if (held) {
    return (
      <div className="flex flex-wrap items-center gap-3">
        <span className="text-small text-ink-muted">In quarantine; a recovery copy is kept.</span>
        <ActionDialog action={{ kind: "restore", id: held.id }} label="Restore…" variant="quiet" />
      </div>
    );
  }
  return (
    <div>
      <ActionDialog action={{ kind: "quarantine", report, index }} label="Quarantine…" />
    </div>
  );
}

/**
 * Level three for an installer: the exact script, every line a rule matched
 * highlighted in its severity's color with the rule IDs beside it.
 */
function InstallerSource({ report }: { report: Report }) {
  if (!report.details) return null;
  const lines = report.details.split("\n").length;
  const notes: Record<number, { text: string; tone: "review" | "high" }> = {};
  let outside = 0;
  for (const finding of report.findings) {
    const line = Number(/^Line (\d+):/.exec(finding.evidence)?.[1] ?? 0);
    if (!line || finding.severity === "info") continue;
    if (line > lines) {
      outside += 1;
      continue;
    }
    const existing = notes[line];
    notes[line] = {
      text: existing ? `${existing.text}, ${finding.rule}` : finding.rule,
      tone: existing?.tone === "high" || finding.severity === "high" ? "high" : "review",
    };
  }
  return (
    <section aria-label="The script" className="flex flex-col gap-2">
      <EvidenceBlock label="The script, as saved" text={report.details} highlight={Object.keys(notes).map(Number)} notes={notes} />
      {outside > 0 && (
        <p className="m-0 text-small text-ink-muted">
          {outside} finding line numbers fall outside the saved text and are not highlighted.
        </p>
      )}
    </section>
  );
}

/** Reports about a local folder or file someone may next want to run, and
 * whether the subject is a folder. */
const RUNNABLE: Record<string, "folder" | "file"> = {
  "Installer inspection": "file",
  "Package script inventory": "folder",
  "Project inspection": "folder",
  "Repository check": "folder",
  "Archive check": "file",
  "Launcher check": "file",
  "Package file": "file",
  "Package archive": "file",
  "PKGBUILD check": "file",
  "AppImage check": "file",
};

/** The port field's value as a port, null when empty, undefined when invalid. */
export function parsePort(text: string): number | null | undefined {
  const t = text.trim();
  if (t === "") return null;
  const n = /^\d{1,5}$/.test(t) ? Number(t) : 0;
  return n >= 1 && n <= 65535 ? n : undefined;
}

/**
 * Run the folder or file in a disposable Podman container. The daemon previews
 * what it can reach and the exact commands; confirming opens a terminal with the
 * container's shell. Under a High finding, or for an installer, the network
 * starts off.
 */
function OpenInContainer({ report, subject }: { report: Report; subject: "folder" | "file" }) {
  const tools = useToolGroups();
  const cautious = report.kind === "Installer inspection" || report.findings.some((f) => f.severity === "high");
  const [network, setNetwork] = useState(!cautious);
  const [port, setPort] = useState("");
  const parsed = parsePort(port);
  const podman = tools.data?.find((t) => t.id === "container");
  return (
    <section aria-label="Run it in a disposable container" className="flex max-w-[56ch] flex-col gap-3">
      <div className="flex flex-col gap-1">
        <h3 className="m-0 text-small font-semibold text-ink-muted">Run it in a disposable container</h3>
        <p className="m-0 text-small text-ink-muted">
          A terminal opens with a shell in a Podman container that holds a copy of this {subject} and none of your files or keys. You run it there yourself
          {report.kind === "Installer inspection" ? ", unlike the rehearsal, which runs the script once and records what it does" : ""}. Leaving the shell
          deletes the container.
        </p>
      </div>
      {podman && !podman.installed ? (
        <div className="flex flex-wrap items-center gap-3">
          <span className="text-small text-ink-muted">This needs Podman, which is not installed.</span>
          <ActionDialog action={{ kind: "install_tool", tool: "container" }} label="Install Podman…" />
        </div>
      ) : (
        <>
          <Toggle
            checked={network}
            onChange={setNetwork}
            label="Allow network"
            description={
              cautious
                ? "Off because of what this report found. Installing dependencies needs it."
                : "Installing dependencies needs it. Code inside can then also reach your local network."
            }
          />
          <Disclosure panel={false} summary={<span className="text-small font-semibold text-ink-muted">Open a port for a web app</span>}>
            <TextField
              label="Port inside the container"
              value={port}
              onChange={setPort}
              mono
              invalid={parsed === undefined}
              hint={
                parsed === undefined
                  ? "Use a number from 1 to 65535."
                  : parsed === null
                    ? "For example 3000. You then open it in your own browser at 127.0.0.1."
                    : `Open http://127.0.0.1:${parsed} in your browser. Only this computer can reach it.`
              }
            />
          </Disclosure>
          <div>
            <ActionDialog
              action={{ kind: "open_container", path: report.subject, offline: !network, port: parsed ?? null }}
              label="Open in a disposable container…"
              disabled={parsed === undefined}
            />
          </div>
        </>
      )}
    </section>
  );
}

/** A skill's SKILL.md, with the lines its findings cite highlighted. */
function SkillSource({ report }: { report: Report }) {
  const notes: Record<number, { text: string; tone: "review" | "high" }> = {};
  for (const f of report.findings) {
    if (f.severity === "info") continue;
    for (const line of f.evidence.split("\n")) {
      const m = /^(?:.*\/)?SKILL\.md, line (\d+)/.exec(line);
      if (!m) continue;
      const n = Number(m[1]);
      const existing = notes[n];
      notes[n] = { text: existing ? `${existing.text}, ${f.rule}` : f.rule, tone: existing?.tone === "high" || f.severity === "high" ? "high" : "review" };
    }
  }
  return <EvidenceBlock label="SKILL.md, as read" text={report.details} highlight={Object.keys(notes).map(Number)} notes={notes} fold />;
}

export function ReportFollowUp({ report }: { report: Report }) {
  const subject = report.subject.startsWith("/") ? RUNNABLE[report.kind] : undefined;
  return (
    <>
      <KindFollowUp report={report} />
      {subject && <OpenInContainer report={report} subject={subject} />}
    </>
  );
}

function KindFollowUp({ report }: { report: Report }) {
  switch (report.kind) {
    case "File scan":
      return <FlaggedFiles report={report.id} />;
    case "Installer inspection":
      return (
        <>
          <InstallerSource report={report} />
          {report.sha256 && (
            <div>
              <ActionDialog action={{ kind: "rehearse", report: report.id, sha256: report.sha256 }} label="Rehearse in the offline sandbox…" />
            </div>
          )}
        </>
      );
    case "Project inspection":
    case "Package script inventory":
      return report.subject.endsWith(".tgz") || report.subject.endsWith(".gz") ? null : (
        <div>
          <ActionDialog action={{ kind: "package_policy", path: report.subject }} label="Disable automatic install scripts…" />
        </div>
      );
    case "Security updates":
      return report.details ? (
        <section className="flex flex-col gap-2">
          <h3 className="m-0 text-small font-semibold text-ink-muted">Install all security updates</h3>
          <EvidenceBlock text={report.details} label="Command (Bigman does not run it)" />
          <p className="m-0 max-w-[56ch] text-small text-ink-muted">
            Your desktop&apos;s software updater (GNOME Software or Discover) installs the same updates.
          </p>
        </section>
      ) : null;
    case "PKGBUILD check":
    case "Launcher check":
      return report.details ? (
        <EvidenceBlock text={report.details} label={report.kind === "PKGBUILD check" ? "The PKGBUILD, as read" : "The launcher, as read"} />
      ) : null;
    case "AI skills and plugins":
      // Reviewed one at a time, with their files, in Protect > AI skills.
      return <p className="m-0 max-w-[56ch] text-ink-muted">Read and approve each one in Protect › AI skills.</p>;
    case "Skill check":
      return report.details ? <SkillSource report={report} /> : null;
    case "Persistence changes":
      return report.errors.length === 0 ? (
        <section className="flex flex-col gap-2">
          <p className="m-0 max-w-[56ch] text-ink-muted">
            Accepting records this list as the starting point, so the next comparison shows only what changed after it.
            It does not mark anything as trustworthy.
          </p>
          <BaselineAdvice report={report} />
          <div>
            <ActionDialog action={{ kind: "accept_baseline", report: report.id }} label="Accept as baseline…" />
          </div>
        </section>
      ) : null;
    default:
      return null;
  }
}
