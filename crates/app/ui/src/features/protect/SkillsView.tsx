// Protect > AI skills: every skill and plugin Claude Code, OpenCode and Codex
// load, what each runs without asking, and the version you approved. Approval
// records the exact files read; a later change to any of them is flagged.
import type { ColumnDef } from "@tanstack/react-table";
import { useEffect, useRef, useState } from "react";
import {
  ActionButton,
  ActionDialog,
  ago,
  CoverageList,
  DataTable,
  Disclosure,
  EmptyState,
  EvidenceBlock,
  FileTree,
  FindingEvidence,
  IncompleteNotice,
  KeyValue,
  maxSeverity,
  nowSeconds,
  PathText,
  plural,
  ProgressRow,
  reportStatus,
  SearchField,
  SEVERITY_RANK,
  SeverityBadge,
  Skeleton,
  useDrilldown,
  type FileTreeEntry,
  type FileTreeGroup,
} from "@/design";
import { useCheck, useHistory, useReport, useSkillItems, type Finding, type Report, type SkillItem } from "@/ipc";

export const INVENTORY = "AI skills and plugins";

const TOOL: Record<string, string> = { claude_code: "Claude Code", open_code: "OpenCode", codex: "Codex" };
const KIND: Record<string, string> = {
  skill: "Skill",
  command: "Command",
  agent: "Subagent",
  plugin: "Plugin",
  code_plugin: "Code plugin",
  npm_plugin: "npm plugin",
};

/** Who decided: the status always says it was you, never Bigman. */
function status(item: SkillItem, now: number): { line: string; detail: string | null } {
  const when = item.approved_at === null ? null : ago(item.approved_at, now);
  switch (item.status) {
    case "changed":
      return { line: "Changed", detail: when ? `since you approved it ${when}` : "since you approved it" };
    case "approved":
      return { line: "You approved it", detail: when };
    case "unreviewed":
      return { line: "You have not reviewed it", detail: null };
  }
}

function toReview(item: SkillItem): Finding[] {
  return item.findings.filter((f) => f.severity !== "info").sort((a, b) => SEVERITY_RANK[b.severity] - SEVERITY_RANK[a.severity]);
}

function tools(item: SkillItem): string {
  return item.tools.map((t) => TOOL[t] ?? t).join(", ") || "Inspected on its own";
}

/** "SKILL.md, line 14: …" → the files a finding cites, when they are the item's own. */
function citedPaths(finding: Finding, item: SkillItem): string[] {
  const paths = new Set<string>();
  for (const line of finding.evidence.split("\n")) {
    const path = /^(.+?)(?:, line \d+|:\s)/.exec(line)?.[1];
    if (path && item.files.some((f) => f.path === path)) paths.add(path);
  }
  return [...paths];
}

const twoLine = (line: string, detail: string | null) => (
  <span className="flex min-w-0 flex-col">
    <span className="truncate text-ink">{line}</span>
    {detail && <span className="truncate text-small text-ink-muted">{detail}</span>}
  </span>
);

const columns = (now: number): ColumnDef<SkillItem>[] => [
  {
    id: "name",
    header: "Name",
    accessorFn: (i) => `${i.name} ${KIND[i.kind] ?? i.kind} ${tools(i)}`,
    cell: ({ row }) => twoLine(row.original.name, `${KIND[row.original.kind] ?? row.original.kind} · ${tools(row.original)}`),
  },
  {
    id: "status",
    header: "Status",
    accessorFn: (i) => status(i, now).line,
    cell: ({ row }) => {
      const s = status(row.original, now);
      return twoLine(s.line, s.detail);
    },
    meta: { width: "15rem" },
  },
  {
    id: "runs",
    header: "Runs without asking",
    accessorFn: (i) => i.runs.length,
    cell: ({ row }) => (row.original.runs.length ? String(row.original.runs.length) : "None"),
    meta: { width: "10rem", align: "right" },
  },
  {
    id: "findings",
    header: "Findings",
    accessorFn: (i) => toReview(i).length,
    cell: ({ row }) => {
      const found = toReview(row.original);
      const severity = maxSeverity(found.map((f) => f.severity));
      if (!found.length || !severity) return "None";
      // The badge is the most severe one; the count is all of them, so "3" never reads as "3 High".
      return (
        <span className="inline-flex items-center gap-2">
          <SeverityBadge severity={severity} />
          <span className="text-small text-ink-muted">{plural(found.length, "finding")}</span>
        </span>
      );
    },
    meta: { width: "11rem", align: "right" },
  },
];

/** The newest inventory report, from this session's check or history. */
function useLatest(check: ReturnType<typeof useCheck>): Report | undefined {
  const last = useHistory().data?.find((r) => r.kind === INVENTORY);
  const fresh = check.data?.kind === INVENTORY ? check.data : undefined;
  if (fresh && (!last || fresh.created >= last.created)) return fresh;
  return last;
}

/** A skill's files, changed ones first, each with the badge of the findings that cite it. */
function fileGroups(item: SkillItem): FileTreeGroup[] {
  const change = new Map<string, FileTreeEntry["change"]>();
  for (const c of item.changed) {
    const m = /^(.+) \((added|changed|removed)\)$/.exec(c);
    if (m?.[1] && m[2]) change.set(m[1], m[2] as FileTreeEntry["change"]);
  }
  const severity = new Map<string, Finding["severity"]>();
  for (const f of item.findings) {
    for (const path of citedPaths(f, item)) {
      const prev = severity.get(path);
      if (!prev || SEVERITY_RANK[f.severity] > SEVERITY_RANK[prev]) severity.set(path, f.severity);
    }
  }
  const files: FileTreeEntry[] = item.files.map((f) => ({
    path: f.path,
    change: change.get(f.path),
    detail: f.executable ? "Executable" : undefined,
    severity: severity.get(f.path),
  }));
  const removed: FileTreeEntry[] = [...change].filter(([, c]) => c === "removed").map(([path]) => ({ path, change: "removed" }));
  const changed = [...files.filter((f) => f.change), ...removed];
  const rest = files.filter((f) => !f.change);
  return [
    ...(changed.length ? [{ category: "Changed since your approval", files: changed, open: true }] : []),
    { category: changed.length ? "Other files" : "Files", files: rest, open: !changed.length },
  ].filter((g) => g.files.length > 0);
}

/** The file to show first: a changed one, then one a finding cites, then SKILL.md. */
function firstFile(item: SkillItem): string | undefined {
  const changed = item.changed
    .map((c) => c.replace(/ \((added|changed)\)$/, ""))
    .find((p) => item.files.some((f) => f.path === p && f.text !== null));
  const cited = toReview(item).flatMap((f) => citedPaths(f, item))[0];
  const main = item.files.find((f) => f.path === "SKILL.md" || f.path === item.root.split("/").pop());
  return changed ?? cited ?? main?.path ?? item.files.find((f) => f.text !== null)?.path;
}

/**
 * One skill or plugin, read from the newest inventory each render so the panel
 * follows an approval, a re-check or a change Bigman noticed in the background.
 */
function SkillDetail({ itemKey }: { itemKey: string }) {
  const recheck = useCheck();
  const report = useLatest(recheck);
  const items = useSkillItems(report?.id ?? null);
  const push = useDrilldown((s) => s.push);
  const [file, setFile] = useState<string | null>(null);
  if (!report || items.isPending) return <Skeleton shape="rows" count={4} />;
  if (items.isError) return <IncompleteNotice title="This list is not available" errors={[items.error.message]} />;
  const item = items.data.find((i) => i.key === itemKey);
  if (!item) return <p className="m-0 text-ink-muted">It is no longer installed where it was.</p>;
  const now = nowSeconds();
  const found = toReview(item);
  const lead = found[0];
  const high = found.some((f) => f.severity === "high");
  const after = () => recheck.mutate({ type: "skills" });
  const shownPath = file ?? firstFile(item);
  const shown = item.files.find((f) => f.path === shownPath);
  const openFinding = (f: Finding) => {
    const cited = citedPaths(f, item)[0];
    if (cited) setFile(cited);
    push({ title: f.summary, content: <FindingEvidence finding={f} /> });
  };
  // Findings cite "path, line N"; mark those lines in the file shown.
  const notes: Record<number, { text: string; tone: "review" | "high" }> = {};
  if (shown) {
    for (const f of found) {
      for (const line of f.evidence.split("\n")) {
        const m = /^(.+?), line (\d+)/.exec(line);
        if (m?.[1] !== shown.path) continue;
        const n = Number(m[2]);
        const existing = notes[n];
        notes[n] = {
          text: existing ? `${existing.text}, ${f.rule}` : f.rule,
          tone: existing?.tone === "high" || f.severity === "high" ? "high" : "review",
        };
      }
    }
  }
  const s = status(item, now);
  return (
    <div className="flex flex-col gap-5">
      {/* The answer first: what changed, or the most severe finding and what to do. */}
      <section aria-label="What to look at first" className="flex flex-col gap-2">
        <p className="m-0 text-ink">
          <span className="font-semibold">{s.line}</span>
          {s.detail && <span className="text-ink-muted"> {s.detail}</span>}
          <span className="text-ink-muted">
            {" · "}
            {KIND[item.kind] ?? item.kind} loaded by {tools(item)}
          </span>
        </p>
        {item.status === "changed" && (
          <p className="m-0 max-w-[56ch] text-ink">
            {plural(item.changed.length, "file")} changed since you approved it. Read {item.changed.length === 1 ? "it" : "them"} below
            before using it again.
          </p>
        )}
        {lead ? (
          <div className="flex flex-col gap-2 rounded-panel border border-rule p-3">
            <div className="flex items-start gap-3">
              <SeverityBadge severity={lead.severity} />
              <p className="m-0 font-semibold text-ink">{lead.summary}</p>
            </div>
            <p className="m-0 max-w-[56ch] text-ink">{lead.action}</p>
            <div>
              <ActionButton variant="quiet" onClick={() => openFinding(lead)}>
                See the evidence
              </ActionButton>
            </div>
          </div>
        ) : (
          <p className="m-0 max-w-[56ch] text-ink">No findings from these rules. The wording of its instructions is not judged.</p>
        )}
      </section>

      <section aria-labelledby="skill-runs" className="flex flex-col gap-2">
        <h3 id="skill-runs" className="m-0 text-small font-semibold text-ink-muted">
          Runs without asking
        </h3>
        {item.runs.length ? (
          <EvidenceBlock text={item.runs.join("\n")} label="What runs" />
        ) : (
          <p className="m-0 max-w-[56ch] text-ink">None recognised. The agent may still run its scripts when its instructions tell it to.</p>
        )}
        {item.allowed_tools.length > 0 && (
          <KeyValue items={[{ label: "Approved in advance", value: item.allowed_tools.join(", "), mono: true }]} />
        )}
      </section>

      {found.length > 1 && (
        <section aria-labelledby="skill-findings" className="flex flex-col gap-2">
          <h3 id="skill-findings" className="m-0 text-small font-semibold text-ink-muted">
            All findings
          </h3>
          <ul className="m-0 flex list-none flex-col gap-1 p-0">
            {found.map((f, i) => (
              <li key={`${f.rule}-${String(i)}`}>
                <button
                  type="button"
                  aria-label={`${f.severity === "high" ? "High" : "Review"}: ${f.summary}`}
                  onClick={() => openFinding(f)}
                  className="flex w-full items-start gap-3 rounded-control px-2 py-1.5 text-left text-ink hover:bg-ground"
                >
                  <SeverityBadge severity={f.severity} />
                  <span className="min-w-0">
                    <span className="block">{f.summary}</span>
                    <span className="block truncate font-mono text-small text-ink-muted">{f.evidence.split("\n")[0]}</span>
                  </span>
                </button>
              </li>
            ))}
          </ul>
        </section>
      )}

      <section aria-labelledby="skill-files" className="flex flex-col gap-2">
        <h3 id="skill-files" className="m-0 text-small font-semibold text-ink-muted">
          Read it yourself
        </h3>
        {item.files.length > 1 && <FileTree groups={fileGroups(item)} selected={shownPath} onOpen={(f) => setFile(f.path)} />}
        {shown?.text != null ? (
          <EvidenceBlock text={shown.text} label={shown.path} highlight={Object.keys(notes).map(Number)} notes={notes} fold />
        ) : (
          <p className="m-0 max-w-[56ch] text-ink-muted">
            {shown
              ? `${shown.path} is not text, or is too large to show (${plural(shown.bytes, "byte")}).`
              : shownPath
                ? `${shownPath} is no longer there.`
                : "No files were read."}
          </p>
        )}
        {item.skipped.length > 0 && <EvidenceBlock text={item.skipped.join("\n")} label="Not read" />}
      </section>

      {(item.remove_command ?? item.remove_note) && (
        <section aria-labelledby="skill-remove" className="flex flex-col gap-2">
          <h3 id="skill-remove" className="m-0 text-small font-semibold text-ink-muted">
            If you do not want it
          </h3>
          {item.remove_note && <p className="m-0 max-w-[56ch] text-ink">{item.remove_note}</p>}
          {item.remove_command && <EvidenceBlock text={item.remove_command} label="Command (Bigman does not run it)" />}
        </section>
      )}

      <div className="flex flex-wrap items-center gap-2 border-t border-rule pt-4">
        {item.status !== "approved" && (
          <ActionDialog
            action={{ kind: "approve_skill", report: report.id, key: item.key }}
            label={item.status === "changed" ? "Approve this version…" : "Approve…"}
            // A High finding is something to read about, not a warning to clear.
            variant={high ? "secondary" : "primary"}
            onDone={after}
          />
        )}
        {item.approved_at !== null && (
          <ActionDialog action={{ kind: "revoke_skill_approval", key: item.key }} label="Take back approval…" variant="quiet" onDone={after} />
        )}
        <span role="status" className="text-small text-ink-muted">
          {recheck.isPending ? "Checking again…" : ""}
        </span>
      </div>

      <KeyValue
        items={[
          { label: "Where", value: <PathText path={item.root} className="text-small" /> },
          ...(item.source ? [{ label: "Installed as", value: item.source, mono: true }] : []),
          ...(item.description ? [{ label: "Says it does", value: item.description }] : []),
          { label: "Fingerprint of its files", value: item.hash, mono: true },
        ]}
      />
    </div>
  );
}

/** What the check covered, from the full report (history keeps a summary). */
function Coverage({ id }: { id: string }) {
  const full = useReport(id).data;
  if (!full) return null;
  // Gaps are never folded away: an incomplete check says so above the list.
  if (reportStatus(full) === "Incomplete") {
    return <IncompleteNotice title="Some skills or plugins were not read completely" coverage={full.coverage} errors={full.errors} />;
  }
  return (
    <Disclosure panel={false} summary={<span className="text-small text-ink-muted">What this check covers</span>}>
      <CoverageList coverage={full.coverage} errors={full.errors} />
    </Disclosure>
  );
}

const HOW = (
  <>
    Skills and plugins for Claude Code, OpenCode and Codex can run commands, start servers and approve tools for the
    agent as soon as they load. Bigman reads each one as data and lists what it runs without asking and what its rules
    find. Approve the version you read; if any of its files change afterwards, it is flagged here and in Activity.
    Nothing in a skill is run, and the wording of its instructions is not judged, so an approval is your decision, not
    a verdict on the skill.
  </>
);

export function SkillsView() {
  const check = useCheck();
  const report = useLatest(check);
  const items = useSkillItems(report?.id ?? null);
  const open = useDrilldown((s) => s.open);
  const [query, setQuery] = useState("");
  const heading = useRef<HTMLHeadingElement>(null);
  const progress = useRef<HTMLDivElement>(null);
  const pending = check.isPending;
  const resultId = check.data?.id;
  const loaded = items.data !== undefined;
  // The button that started the check changes its label; focus follows the
  // work: the progress row, then the list's heading once the list is in.
  useEffect(() => {
    if (pending) progress.current?.focus();
  }, [pending]);
  useEffect(() => {
    if (resultId && loaded) heading.current?.focus();
  }, [resultId, loaded]);
  const now = nowSeconds();
  const list = items.data ?? [];
  const count = (s: SkillItem["status"]) => list.filter((i) => i.status === s).length;
  const flagged = list.filter((i) => toReview(i).length > 0).length;
  const summary = [
    list.length === 1 ? "1 skill or plugin" : `${String(list.length)} skills and plugins`,
    count("changed") ? `${String(count("changed"))} changed since you approved them` : null,
    count("unreviewed") ? `${String(count("unreviewed"))} not reviewed` : null,
    flagged ? `${String(flagged)} with findings` : null,
  ]
    .filter(Boolean)
    .join(" · ");
  const recheck = () => check.mutate({ type: "skills" });
  return (
    <div className="flex flex-col gap-5">
      <section aria-label="AI skills and plugins" className="flex flex-col gap-3">
        {report ? (
          <div className="max-w-[56ch] text-small text-ink-muted">
            <Disclosure panel={false} summary="How this works">
              <p className="m-0">{HOW}</p>
            </Disclosure>
          </div>
        ) : (
          <>
            <p className="m-0 text-body text-ink-muted">Skills and plugins have not been checked yet.</p>
            <p className="m-0 max-w-[56ch] text-ink-muted">{HOW}</p>
          </>
        )}
        <div className="flex flex-wrap items-center gap-2">
          <ActionButton variant={report ? "secondary" : "primary"} disabled={pending} onClick={recheck}>
            {report ? "Check again" : "Check skills and plugins"}
          </ActionButton>
          {report && <span className="text-small text-ink-muted">Checked {ago(report.created, now)}. Bigman also notices new and changed ones on its own.</span>}
        </div>
      </section>

      {pending && (
        <div ref={progress} tabIndex={-1} className="outline-none">
          <ProgressRow label="Reading skills and plugins" progress={null} onCancel={check.cancel} />
        </div>
      )}
      {check.error && <IncompleteNotice title="The check did not finish" errors={[check.error.message]} />}

      {report && (
        <section aria-labelledby="skills-list" className="flex flex-col gap-3">
          <div className="flex flex-wrap items-end justify-between gap-3">
            <h2 id="skills-list" ref={heading} tabIndex={-1} className="m-0 rounded-control text-body font-semibold text-ink outline-none">
              {loaded ? summary : "Skills and plugins"}
            </h2>
            {list.length > 8 && <SearchField label="Search skills and plugins" value={query} onChange={setQuery} />}
          </div>
          {resultId && loaded && (
            <p className="sr-only" role="status">
              Check finished: {summary}.
            </p>
          )}
          <Coverage id={report.id} />
          {items.isPending ? (
            <Skeleton shape="rows" />
          ) : items.isError ? (
            <IncompleteNotice title="The list from this check is not available" errors={[items.error.message]} />
          ) : list.length ? (
            <DataTable
              label="Skills and plugins"
              columns={columns(now)}
              data={list}
              getRowId={(i) => i.key}
              onOpen={(i) => open({ title: i.name, content: <SkillDetail itemKey={i.key} /> })}
              filter={query}
              rowHeight={52}
              height={Math.min(520, 32 + list.length * 52)}
            />
          ) : (
            <EmptyState action={<ActionButton onClick={recheck}>Check again</ActionButton>}>
              No skills, commands, subagents or plugins were found for Claude Code, OpenCode or Codex.
            </EmptyState>
          )}
        </section>
      )}
    </div>
  );
}
