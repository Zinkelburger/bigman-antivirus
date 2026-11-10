// Protect > Running system: processes, kernel modules and sockets looked at two
// ways, with where the answers differ, and what ordinary tools do not list
// (programs running from memory, packet access, BPF programs, watchers of
// credential folders). A row you mark as known stops counting until one of its
// pinned details changes. Rows where two views disagree cannot be marked.
import type { ColumnDef } from "@tanstack/react-table";
import { useEffect, useRef, useState } from "react";
import {
  ActionButton,
  ActionDialog,
  ago,
  CoverageList,
  DataTable,
  Disclosure,
  EvidenceBlock,
  FindingEvidence,
  IncompleteNotice,
  KeyValue,
  nowSeconds,
  ProgressRow,
  reportStatus,
  SearchField,
  SEVERITY_RANK,
  SeverityBadge,
  Skeleton,
  StatusLine,
  useDrilldown,
} from "@/design";
import { useCheck, useHistory, usePrograms, useReport, useRunningItems, type Report, type RunningItem } from "@/ipc";
import { StopWatch, useStopProgram } from "./StopWatch";

export const RUNNING = "Running system";

const SECTION: Record<RunningItem["section"], string> = {
  disagreements: "Two views disagree",
  programs: "Programs",
  kernel: "Kernel",
  network: "Packet access",
  watches: "Watched files",
};

const STANDING_ORDER: Record<RunningItem["standing"], number> = { changed: 0, new: 1, known: 2 };

/** Who decided: a mark is always yours, never Bigman's. */
function standing(item: RunningItem, now: number): string {
  const when = item.known_at === null ? "" : ` ${ago(item.known_at, now)}`;
  switch (item.standing) {
    case "changed":
      return `Changed since you marked it${when}`;
    case "known":
      return `Marked${when}`;
    case "new":
      return "Not yet";
  }
}

/** Rows that count: not marked as known (or changed since), and rated Review or High. */
const counts = (item: RunningItem) => item.standing !== "known" && item.severity !== "info";

function ordered(items: RunningItem[]): RunningItem[] {
  return [...items].sort(
    (a, b) =>
      // Rows you marked as known come last; the rest lead with the most severe,
      // and a row that changed since your mark comes before a new one.
      Number(a.standing === "known") - Number(b.standing === "known") ||
      SEVERITY_RANK[b.severity] - SEVERITY_RANK[a.severity] ||
      STANDING_ORDER[a.standing] - STANDING_ORDER[b.standing] ||
      a.title.localeCompare(b.title),
  );
}

const ROW_HEIGHT = 68;

const columns = (now: number): ColumnDef<RunningItem>[] => [
  {
    id: "what",
    header: "What",
    accessorFn: (i) => `${i.title} · ${SECTION[i.section]}`,
    // The title is the finding: it wraps to two lines rather than being cut.
    cell: ({ row }) => (
      <span className="flex min-w-0 flex-col whitespace-normal">
        <span className="line-clamp-2 text-ink">{row.original.title}</span>
        <span className="truncate text-small text-ink-muted">{SECTION[row.original.section]}</span>
      </span>
    ),
  },
  {
    id: "standing",
    header: "Known to you",
    accessorFn: (i) => standing(i, now),
    sortingFn: (a, b) => STANDING_ORDER[a.original.standing] - STANDING_ORDER[b.original.standing],
    cell: ({ row }) => <span className="line-clamp-2 whitespace-normal text-ink">{standing(row.original, now)}</span>,
    meta: { width: "11rem" },
  },
  {
    id: "severity",
    header: "Severity",
    accessorFn: (i) => SEVERITY_RANK[i.severity],
    cell: ({ row }) => <SeverityBadge severity={row.original.severity} />,
    meta: { width: "7rem", align: "right" },
  },
];

/**
 * The newest Running system report, from this session's check or history.
 * History lists reports newest first, to finer than the second a report's
 * `created` holds, so once it lists this session's check its order decides:
 * a re-check from a row's panel can land in the same second as the check that
 * filled the list, and the list must follow it. Until history has caught up,
 * the check just made is the newest.
 */
function useLatest(check: ReturnType<typeof useCheck>): Report | undefined {
  const history = useHistory().data;
  const last = history?.find((r) => r.kind === RUNNING);
  const fresh = check.data?.kind === RUNNING ? check.data : undefined;
  const listed = fresh !== undefined && history?.some((r) => r.id === fresh.id);
  if (fresh && !listed && (!last || fresh.created >= last.created)) return fresh;
  return last ?? fresh;
}

/** Values that are paths, commands, identifiers or lists of names read as machine text. */
const MACHINE = new Set(["File", "Command line", "Process ID", "Process IDs", "Names", "Library", "Libraries", "Local", "Remote", "Memory file", "Bus name"]);
const LONG_VALUE = 200;

function Facts({ id, heading, facts }: { id: string; heading: string; facts: RunningItem["facts"] }) {
  if (!facts.length) return null;
  const short = facts.filter((f) => f.value.length <= LONG_VALUE);
  const long = facts.filter((f) => f.value.length > LONG_VALUE);
  return (
    <section aria-labelledby={id} className="flex flex-col gap-2">
      <h3 id={id} className="m-0 text-small font-semibold text-ink-muted">
        {heading}
      </h3>
      {short.length > 0 && (
        <KeyValue items={short.map((f) => ({ label: f.label, value: f.value, mono: MACHINE.has(f.label) || f.value.startsWith("/") }))} />
      )}
      {/* A long list reads one entry per line, and stays folded until asked for. */}
      {long.map((f) => (
        <Disclosure key={f.label} panel={false} summary={<span className="text-small text-ink">{f.label}</span>}>
          <EvidenceBlock text={f.value.split(", ").join("\n")} label={f.label} />
        </Disclosure>
      ))}
    </section>
  );
}

/** The one process a row is about, when the row names exactly one. */
function processOf(item: RunningItem): number | null {
  const fact = item.facts.find((f) => f.label === "Process ID" || f.label === "Process IDs");
  return fact && /^\d+$/.test(fact.value) ? Number(fact.value) : null;
}

/** "Stop and watch…" for a program row whose process is still one of yours. */
function StopThis({ pid }: { pid: number }) {
  const program = usePrograms().data?.find((p) => p.pid === pid);
  const stopProgram = useStopProgram();
  if (!program) return null;
  return (
    <ActionButton variant="secondary" onClick={() => stopProgram(program)}>
      Stop and watch…
    </ActionButton>
  );
}

/**
 * One row, read from the newest report each render so the panel follows a
 * mark, a re-check or a change Bigman noticed in the background.
 */
function RunningDetail({ itemKey }: { itemKey: string }) {
  const recheck = useCheck();
  const report = useLatest(recheck);
  const items = useRunningItems(report?.id ?? null);
  const item = items.data?.find((i) => i.key === itemKey);
  const actions = useRef<HTMLDivElement>(null);
  const mark = item?.standing;
  const marked = useRef(mark);
  // The re-check after a mark can finish once its dialog has closed; the button
  // that had focus is then replaced, so focus moves to the one in its place.
  useEffect(() => {
    const replaced = marked.current !== undefined && mark !== undefined && marked.current !== mark;
    marked.current = mark;
    if (replaced && (!document.activeElement || document.activeElement === document.body)) {
      actions.current?.querySelector("button")?.focus();
    }
  }, [mark]);
  if (!report || items.data === undefined) {
    if (items.isError) return <IncompleteNotice title="This list is not available" errors={[items.error.message]} />;
    return <Skeleton shape="rows" count={4} />;
  }
  if (!item) return <p className="m-0 text-ink-muted">It was not there at the last check.</p>;
  const after = () => recheck.mutate({ type: "running" });
  const markable = item.section !== "disagreements";
  const pid = item.section === "programs" ? processOf(item) : null;
  return (
    <div className="flex min-h-full flex-col gap-5">
      <section aria-label="Known to you" className="flex flex-col gap-2">
        <p className="m-0 text-ink">
          <span className="font-semibold">{!markable ? "Cannot be marked as known" : item.standing === "new" ? "Not marked as known" : standing(item, nowSeconds())}</span>
          <span className="text-ink-muted">
            {" · "}
            {SECTION[item.section]}
          </span>
        </p>
        {item.standing === "changed" && <EvidenceBlock text={item.changed.join("\n")} label="What changed since your mark" />}
      </section>

      <FindingEvidence
        finding={{ rule: item.rule, severity: item.severity, summary: item.title, evidence: item.evidence, action: item.action, ...(item.command ? { command: item.command } : {}) }}
      />

      {markable ? (
        <>
          <Facts id="running-pinned" heading="A mark holds while these stay the same" facts={item.facts.filter((f) => f.pinned)} />
          <Facts id="running-moving" heading="Not part of a mark" facts={item.facts.filter((f) => !f.pinned)} />
        </>
      ) : (
        <Facts id="running-details" heading="Details" facts={item.facts} />
      )}

      {/* Always in view: the panel is longer than the window at its smallest. */}
      <div ref={actions} className="sticky -bottom-4 -mx-4 -mb-4 mt-auto flex flex-wrap items-center gap-2 border-t border-rule bg-surface px-4 py-3">
        {!markable && (
          <>
            {/* Never disabled, so focus stays here while it runs; a second press waits for the first. */}
            <ActionButton variant="secondary" aria-describedby="running-unmarkable" onClick={() => !recheck.isPending && after()}>
              Check again
            </ActionButton>
            <p id="running-unmarkable" className="m-0 max-w-[56ch] basis-full text-pretty text-small text-ink-muted">
              Ordinary programs do not make two of the kernel's answers differ, so this row stays listed until it goes away. Checking again rules out a
              coincidence of timing.
            </p>
          </>
        )}
        {markable && item.standing !== "known" && (
          <ActionDialog
            action={{ kind: "mark_known", report: report.id, key: item.key }}
            label={item.standing === "changed" ? "Mark this version as known…" : "Mark as known…"}
            // A High row is something to read about, not a warning to clear.
            variant={item.severity === "high" ? "secondary" : "primary"}
            onDone={after}
          />
        )}
        {item.known_at !== null && <ActionDialog action={{ kind: "forget_known", key: item.key }} label="Take back the mark…" variant="quiet" onDone={after} />}
        {pid !== null && <StopThis pid={pid} />}
        <span role="status" className="text-small text-ink-muted">
          {recheck.isPending ? "Checking again…" : recheck.isSuccess ? `Checked ${ago(recheck.data.created, nowSeconds())}. Still listed.` : ""}
        </span>
      </div>
    </div>
  );
}

/** What the check covered, from the full report (history keeps a summary). */
function Coverage({ id }: { id: string }) {
  const full = useReport(id).data;
  if (!full) return null;
  // Gaps are never folded away: an incomplete check says so above the list.
  if (reportStatus(full) === "Incomplete") {
    return <IncompleteNotice title="Some of the running system could not be read" coverage={full.coverage} errors={full.errors} />;
  }
  return (
    <Disclosure panel={false} summary={<span className="text-small text-ink-muted">What this check compares, and what it cannot see</span>}>
      <CoverageList coverage={full.coverage} errors={full.errors} />
    </Disclosure>
  );
}

const HOW = [
  "Bigman asks your system three questions in two different ways: which programs are running, which kernel modules are loaded, and which network connections are open. Anything that is in one answer and not the other is listed.",
  "It also lists what ordinary tools leave out: programs running with no file on disk, programs reading network traffic directly, small programs loaded into the kernel (BPF), and programs watching the folders that hold your keys.",
  "When you recognise a row, mark it as known. It stays listed and stops counting until its details change. Rows where two views disagree cannot be marked: nothing ordinary causes them.",
  "A short list is not proof that nothing is hidden. Something that hides from every view at once hides from this check too.",
];

function How({ muted }: { muted: boolean }) {
  return (
    <div className={`flex max-w-[64ch] flex-col gap-2 ${muted ? "text-small text-ink-muted" : "text-ink"}`}>
      {HOW.map((p) => (
        <p key={p} className="m-0">
          {p}
        </p>
      ))}
    </div>
  );
}

export function RunningView() {
  const check = useCheck();
  const report = useLatest(check);
  const items = useRunningItems(report?.id ?? null);
  const full = useReport(report?.id ?? null).data;
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
  const list = ordered(items.data ?? []);
  const review = list.filter(counts).length;
  const known = list.filter((i) => i.standing === "known").length;
  const info = list.length - review - known;
  const incomplete = full !== undefined && reportStatus(full) === "Incomplete";
  // One of the three results leads; the counts follow it.
  const result = review ? `${String(review)} to review` : incomplete ? "Incomplete" : null;
  const summary = [result, info ? `${String(info)} listed for information` : null, known ? `${String(known)} marked as known` : null]
    .filter(Boolean)
    .join(" · ");
  const recheck = () => check.mutate({ type: "running" });
  return (
    <div className="flex flex-col gap-5">
      <section aria-label="Running system" className="flex flex-col gap-3">
        {report ? (
          <Disclosure panel={false} summary="How this works">
            <How muted />
          </Disclosure>
        ) : (
          <>
            <p className="m-0 text-body text-ink-muted">The running system has not been checked yet.</p>
            <How muted={false} />
          </>
        )}
        <div className="flex flex-wrap items-center gap-2">
          <ActionButton variant={report ? "secondary" : "primary"} disabled={pending} onClick={recheck}>
            {report ? "Check again" : "Check the running system"}
          </ActionButton>
          {report && (
            <span className="text-small text-ink-muted">
              Checked {ago(report.created, now)}. Bigman also checks every ten minutes while it runs in the background.
            </span>
          )}
        </div>
      </section>

      {pending && (
        <div ref={progress} tabIndex={-1} className="outline-none">
          <ProgressRow label="Reading processes, kernel modules and sockets" progress={null} onCancel={check.cancel} />
        </div>
      )}
      {check.error && <IncompleteNotice title="The check did not finish" errors={[check.error.message]} />}

      {report && (
        <section aria-labelledby="running-list" className="flex flex-col gap-3">
          {loaded && !result && <StatusLine ranAt={report.created} now={now} />}
          <div className="flex flex-wrap items-end justify-between gap-3">
            <h2 id="running-list" ref={heading} tabIndex={-1} className="m-0 rounded-control text-body font-semibold text-ink outline-none">
              {!loaded ? "Running system" : summary || "Nothing is listed from the parts that were read"}
            </h2>
            {list.length > 8 && <SearchField label="Search the running system" value={query} onChange={setQuery} />}
          </div>
          {resultId && loaded && (
            <p className="sr-only" role="status">
              Check finished: {result ?? "No findings from these checks"}
              {summary && result !== summary ? `. ${summary}` : ""}.
            </p>
          )}
          <Coverage id={report.id} />
          {!loaded && items.isError ? (
            <IncompleteNotice title="The list from this check is not available" errors={[items.error.message]} />
          ) : !loaded ? (
            <Skeleton shape="rows" />
          ) : (
            list.length > 0 && (
              <DataTable
                label="Running system"
                columns={columns(now)}
                data={list}
                getRowId={(i) => i.key}
                onOpen={(i) => open({ title: i.title, content: <RunningDetail itemKey={i.key} /> })}
                filter={query}
                rowHeight={ROW_HEIGHT}
                height={Math.min(520, 34 + list.length * ROW_HEIGHT)}
              />
            )
          )}
        </section>
      )}

      <StopWatch />
    </div>
  );
}
