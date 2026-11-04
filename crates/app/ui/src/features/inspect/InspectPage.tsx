import type { ColumnDef } from "@tanstack/react-table";
import { useState } from "react";
import {
  ActionButton,
  ago,
  baseName,
  CheckStatus,
  DaemonGate,
  DataTable,
  Disclosure,
  KeyValue,
  nowSeconds,
  PageHeader,
  PathPicker,
  PathText,
  PreviewDialog,
  ReportById,
  reportName,
  reportSeverity,
  reportStatus,
  SeverityBadge,
  useDrilldown,
} from "@/design";
import { desktopShell, useCheck, useHistory, type Report } from "@/ipc";

const KINDS = new Set(["Installer inspection", "Restricted installer rehearsal", "Package script inventory", "Project inspection", "Repository check", "Lockfile check", "Archive check", "Launcher check", "Package file", "Package archive", "PKGBUILD check", "AppImage check", "Skill check", "PDF triage", "YARA-X scan", "osv integration"]);

const columns = (now: number): ColumnDef<Report>[] => [
  { id: "when", header: "When", accessorFn: (r) => r.created, cell: ({ row }) => ago(row.original.created, now), meta: { width: "9rem" } },
  { id: "kind", header: "Inspection", accessorFn: (r) => reportName(r), meta: { width: "12rem" } },
  { accessorKey: "subject", header: "Of", cell: ({ row }) => <PathText path={row.original.subject} className="text-small" /> },
  {
    id: "result",
    header: "Result",
    accessorFn: (r) => reportStatus(r),
    cell: ({ row }) => {
      const severity = reportSeverity(row.original);
      return severity && severity !== "info" ? <SeverityBadge severity={severity} /> : reportStatus(row.original);
    },
    meta: { width: "14rem" },
  },
];

type More = { type: "yara"; target: string; rules: string } | { type: "osv"; target: string } | { type: "lockfile"; target: string; against: string };

/** Inspections that need a second input or send data off the machine. */
function MoreInspections({ onRun, busy }: { onRun: (request: More) => void; busy: boolean }) {
  const [rules, setRules] = useState("");
  const [target, setTarget] = useState("");
  const [lockfile, setLockfile] = useState("");
  const [older, setOlder] = useState("");
  const [sending, setSending] = useState(false);
  const choose = async (kind: "file" | "directory", title: string, set: (v: string) => void) => {
    const chosen = await desktopShell().choosePath(kind, title);
    if (chosen) set(chosen);
  };
  return (
    <Disclosure summary="More inspections: lockfile changes, YARA-X rules and dependency advisories">
      <div className="flex flex-col gap-6">
        <section aria-labelledby="lockfile-changes" className="flex flex-col gap-3">
          <h3 id="lockfile-changes" className="m-0 text-body font-semibold text-ink">
            What a lockfile change adds
          </h3>
          <p className="m-0 max-w-[56ch] text-small text-ink-muted">
            Compares a lockfile with an older copy, such as the one from before a pull request, and checks only what is
            new or changed: dependencies from outside the registry, missing hashes, new install scripts, and packages
            whose download address or hash changed without a new version.
          </p>
          <PathPicker label="Lockfile" value={lockfile} onChange={setLockfile} onBrowse={() => void choose("file", "Choose the lockfile", setLockfile)} placeholder="/home/you/project/package-lock.json" />
          <PathPicker label="Older copy of it" value={older} onChange={setOlder} onBrowse={() => void choose("file", "Choose the older lockfile", setOlder)} />
          <div>
            <ActionButton disabled={busy || !lockfile || !older} onClick={() => onRun({ type: "lockfile", target: lockfile, against: older })}>
              Compare lockfiles
            </ActionButton>
          </div>
        </section>
        <section aria-labelledby="rules-and-advisories" className="flex flex-col gap-3">
          <h3 id="rules-and-advisories" className="m-0 text-body font-semibold text-ink">
            YARA-X rules and dependency advisories
          </h3>
          <PathPicker label="File or folder" value={target} onChange={setTarget} onBrowse={() => void choose("directory", "Choose what to scan", setTarget)} />
          <PathPicker label="YARA-X rules file you reviewed" value={rules} onChange={setRules} onBrowse={() => void choose("file", "Choose a YARA-X rules file", setRules)} />
          <p className="m-0 max-w-[56ch] text-small text-ink-muted">
            Rule matches are results of those rules, not proof of malware. Dependency advisories run OSV-Scanner on a
            project folder and send its package names and versions to osv.dev.
          </p>
          <div className="flex gap-2">
            <ActionButton disabled={busy || !target || !rules} onClick={() => onRun({ type: "yara", target, rules })}>
              Scan with these rules
            </ActionButton>
            <ActionButton disabled={busy || !target} onClick={() => setSending(true)}>
              Check dependency advisories…
            </ActionButton>
          </div>
          <PreviewDialog
            open={sending}
            onOpenChange={setSending}
            title="Check dependency advisories"
            planHeading="What will be sent"
            description="Review what leaves this computer before confirming."
            plan={
              <div className="flex flex-col gap-3">
                <KeyValue
                  items={[
                    { label: "To", value: "https://api.osv.dev, the OSV vulnerability database", mono: false },
                    { label: "From", value: target, mono: true },
                  ]}
                />
                <p className="m-0 max-w-[56ch] text-ink">
                  OSV-Scanner reads the lockfiles and manifests in this folder and sends the name, version and ecosystem
                  of each dependency it finds. For git checkouts it can also send their commit. File contents are not
                  sent.
                </p>
              </div>
            }
            confirmLabel="Send and check"
            onConfirm={() => {
              setSending(false);
              onRun({ type: "osv", target });
              return Promise.resolve(null);
            }}
          />
        </section>
      </div>
    </Disclosure>
  );
}

export function InspectPage() {
  const check = useCheck();
  const history = useHistory();
  const open = useDrilldown((s) => s.open);
  const [target, setTarget] = useState("");
  const [ranOn, setRanOn] = useState("");
  const [again, setAgain] = useState(false);
  const now = nowSeconds();
  const recent = (history.data ?? []).filter((r) => KINDS.has(r.kind));

  const browse = async () => {
    const chosen = await desktopShell().choosePath("file", "Choose what to inspect");
    if (chosen) setTarget(chosen);
  };
  const inspect = () => {
    setRanOn(target);
    check.mutate({ type: "inspect_any", target });
  };
  const showForm = !check.data && !check.isPending;

  return (
    <>
      <PageHeader title="Inspect" />
      <div className="flex flex-col gap-5 px-6 py-5">
        <DaemonGate>
          {showForm ? (
            <form
              aria-label="Inspect"
              className="flex flex-col gap-3"
              onSubmit={(e) => {
                e.preventDefault();
                if (target) inspect();
              }}
            >
              <p className="m-0 max-w-[56ch] text-ink-muted">
                Bigman reads it and says what it can do, without running it. Installer scripts are saved byte for
                byte; packages, projects, AI skills and plugins, archives, launchers, .deb and .rpm files and PDFs are read as data. HTTPS URLs are fetched once, without your
                proxies or credentials. A GitHub, GitLab, Codeberg or Bitbucket repository address is read before you clone it, with nothing checked out.
              </p>
              <PathPicker label="File, folder or HTTPS URL" value={target} onChange={setTarget} onBrowse={() => void browse()} placeholder="https://example.org/install.sh" autoFocus={again} />
              <div>
                <ActionButton type="submit" variant="primary" disabled={!target}>
                  Inspect
                </ActionButton>
              </div>
            </form>
          ) : (
            <div className="flex flex-wrap items-center gap-3 rounded-panel border border-rule bg-surface px-4 py-2">
              <PathText path={ranOn} className="flex-1 text-small text-ink" />
              <ActionButton
                variant="quiet"
                disabled={check.isPending}
                onClick={() => {
                  setAgain(true);
                  check.reset();
                }}
              >
                Inspect something else
              </ActionButton>
            </div>
          )}
          <CheckStatus check={check} running={`Inspecting ${ranOn}`} subjectShown />
          {showForm && (
            <MoreInspections
              busy={check.isPending}
              onRun={(more) => {
                setRanOn(more.type === "lockfile" ? `${more.target}, compared with ${more.against}` : more.target);
                check.mutate(
                  more.type === "yara"
                    ? { type: "yara", rules: more.rules, target: more.target }
                    : more.type === "lockfile"
                      ? { type: "lockfile", path: more.target, against: more.against }
                      : { type: "audit", tool: "osv", path: more.target },
                );
              }}
            />
          )}
          {recent.length > 0 && (
            <section aria-labelledby="recent-inspections" className="flex flex-col gap-2">
              <h2 id="recent-inspections" className="m-0 text-body font-semibold text-ink">
                Recent inspections
              </h2>
              <DataTable
                label="Recent inspections"
                columns={columns(now)}
                data={recent}
                getRowId={(r) => r.id}
                initialSorting={[{ id: "when", desc: true }]}
                onOpen={(r) => open({ title: `${reportName(r)}: ${baseName(r.subject)}`, content: <ReportById id={r.id} /> })}
                height={280}
              />
            </section>
          )}
        </DaemonGate>
      </div>
    </>
  );
}
