import { useMemo, useState } from "react";
import {
  ActionButton,
  DaemonGate,
  EmptyState,
  FilterChips,
  fold,
  IncompleteNotice,
  nowSeconds,
  PageHeader,
  ReportById,
  reportOccurrences,
  SearchField,
  Skeleton,
  Timeline,
  useDrilldown,
} from "@/design";
import { useCheck, useHistory, type Severity } from "@/ipc";

const SEVERITIES: readonly { value: Severity; label: string }[] = [
  { value: "high", label: "High" },
  { value: "review", label: "Review" },
  { value: "info", label: "Info" },
];

export function ActivityPage() {
  const history = useHistory();
  const checkup = useCheck();
  const open = useDrilldown((s) => s.open);
  const [search, setSearch] = useState("");
  const [sources, setSources] = useState<Set<string>>(new Set());
  const [severities, setSeverities] = useState<Set<Severity>>(new Set());
  const now = nowSeconds();

  const occurrences = useMemo(() => reportOccurrences(history.data ?? []), [history.data]);
  const folded = useMemo(() => fold(occurrences), [occurrences]);
  // Counts are of rows on screen, not of raw runs.
  const sourceOptions = useMemo(() => {
    const counts = new Map<string, number>();
    for (const e of folded) counts.set(e.source, (counts.get(e.source) ?? 0) + 1);
    return [...counts].sort((a, b) => b[1] - a[1]).map(([value, count]) => ({ value, label: value, count }));
  }, [folded]);
  const entries = useMemo(() => {
    const needle = search.trim().toLowerCase();
    return folded.filter(
      (e) =>
        (sources.size === 0 || sources.has(e.source)) &&
        (severities.size === 0 || severities.has(e.severity)) &&
        (!needle || e.summary.toLowerCase().includes(needle)),
    );
  }, [folded, search, sources, severities]);
  const filtering = search !== "" || sources.size > 0 || severities.size > 0;

  return (
    <>
      <PageHeader title="Activity" controls={<SearchField value={search} onChange={setSearch} label="Search activity" />} />
      <div className="flex flex-col gap-4 px-6 py-5">
        <DaemonGate>
        {history.isError ? (
          <IncompleteNotice title="Activity could not be loaded" errors={[history.error.message]} />
        ) : history.isPending ? (
          <Skeleton shape="rows" count={8} />
        ) : occurrences.length === 0 ? (
          <EmptyState
            action={
              <ActionButton busy={checkup.isPending} onClick={() => checkup.mutate({ type: "doctor" })}>
                Run checkup
              </ActionButton>
            }
          >
            Every check, scan and inspection you run is entered here by day, with repeats folded under their first run.
          </EmptyState>
        ) : (
          <>
            <div className="grid grid-cols-[max-content_minmax(0,1fr)] items-center gap-x-3 gap-y-2">
              <span aria-hidden className="text-small font-semibold text-ink-muted">Check</span>
              <FilterChips label="Filter by check" options={sourceOptions} selected={sources} onChange={setSources} />
              <span aria-hidden className="text-small font-semibold text-ink-muted">Severity</span>
              <FilterChips label="Filter by severity" options={SEVERITIES} selected={severities} onChange={setSeverities} />
            </div>
            <Timeline
              label="Activity"
              entries={entries}
              now={now}
              height={560}
              onOpen={(entry) => open({ title: entry.source, content: <ReportById id={entry.id} /> })}
              empty={
                filtering ? (
                  <>
                    <p className="m-0">Nothing matches these filters.</p>
                    <ActionButton
                      onClick={() => {
                        setSearch("");
                        setSources(new Set());
                        setSeverities(new Set());
                      }}
                    >
                      Clear filters
                    </ActionButton>
                  </>
                ) : null
              }
            />
          </>
        )}
        </DaemonGate>
      </div>
    </>
  );
}
