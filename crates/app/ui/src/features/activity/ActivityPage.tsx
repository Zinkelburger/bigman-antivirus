import { useMemo, useState } from "react";
import {
  ActionButton,
  DaemonGate,
  EmptyState,
  EventPanel,
  eventOccurrence,
  FilterChips,
  fold,
  IncompleteNotice,
  nowSeconds,
  PageHeader,
  SearchField,
  Skeleton,
  SourceGapLine,
  Timeline,
  useDrilldown,
  useToasts,
} from "@/design";
import { useEvents, useMarkSeen, useUnread, type Event, type Severity } from "@/ipc";
import { useShell } from "@/shell/store";

const NEW = [{ value: "new", label: "New since your last look" }] as const;

const SEVERITIES: readonly { value: Severity; label: string }[] = [
  { value: "high", label: "High" },
  { value: "review", label: "Review" },
  { value: "info", label: "Info" },
];

export function ActivityPage() {
  const events = useEvents(0);
  const unread = useUnread();
  const markSeen = useMarkSeen();
  const open = useDrilldown((s) => s.open);
  const toast = useToasts((s) => s.push);
  const { unreadOnly, setUnreadOnly } = useShell();
  const [search, setSearch] = useState("");
  const [sources, setSources] = useState<Set<string>>(new Set());
  const [severities, setSeverities] = useState<Set<Severity>>(new Set());
  const now = nowSeconds();

  const byId = useMemo(() => new Map<string, Event>((events.data ?? []).map((e) => [String(e.id), e])), [events.data]);
  const seenThrough = unread.data?.seen_through ?? Infinity;
  const folded = useMemo(
    () => fold((events.data ?? []).map(eventOccurrence)).map((e) => ({ ...e, unread: Number(e.id) > seenThrough })),
    [events.data, seenThrough],
  );
  const newCount = folded.filter((e) => e.unread).length;
  // Counts are of rows on screen, not of raw events.
  const sourceOptions = useMemo(() => {
    const counts = new Map<string, number>();
    for (const e of folded) counts.set(e.source, (counts.get(e.source) ?? 0) + 1);
    // Alphabetical, so a chip stays put as counts change.
    return [...counts].sort((a, b) => a[0].localeCompare(b[0])).map(([value, count]) => ({ value, label: value, count }));
  }, [folded]);
  const entries = useMemo(() => {
    const needle = search.trim().toLowerCase();
    // Search reads what a row stands for: its sentence, evidence and rule.
    const matches = (id: string, summary: string) => {
      const e = byId.get(id);
      return [summary, e?.evidence ?? "", e?.rule ?? ""].some((text) => text.toLowerCase().includes(needle));
    };
    return folded.filter(
      (e) =>
        (!unreadOnly || e.unread) &&
        (sources.size === 0 || sources.has(e.source)) &&
        (severities.size === 0 || severities.has(e.severity)) &&
        (!needle || matches(e.id, e.summary)),
    );
  }, [folded, byId, search, sources, severities, unreadOnly]);
  const filtering = search !== "" || sources.size > 0 || severities.size > 0 || unreadOnly;
  const markAll = () => {
    if (!unread.data) return;
    const before = unread.data.seen_through;
    const count = newCount;
    markSeen.mutate(unread.data.through, {
      onSuccess: () => {
        setUnreadOnly(false);
        toast(`Marked ${count === 1 ? "1 entry" : `${String(count)} entries`} as seen.`, {
          label: "Undo",
          run: () => markSeen.mutate(before),
        });
      },
    });
  };

  return (
    <>
      <PageHeader
        title="Activity"
        controls={<SearchField value={search} onChange={setSearch} label="Search activity" />}
        action={
          unread.data && (
            <ActionButton
              disabled={unread.data.through <= unread.data.seen_through}
              busy={markSeen.isPending}
              onClick={markAll}
            >
              Mark all as seen
            </ActionButton>
          )
        }
      />
      <div className="flex flex-col gap-4 px-6 py-5">
        <DaemonGate>
          <SourceGapLine />
          {events.isError ? (
            <IncompleteNotice title="Activity could not be loaded" errors={[events.error.message]} />
          ) : events.isPending ? (
            <Skeleton shape="rows" count={8} />
          ) : folded.length === 0 ? (
            <EmptyState>
              Your programs&apos; first connections, new listening programs, package installs, SELinux and AppArmor
              denials and every check you run are entered here by day, with repeats folded under their first sighting.
            </EmptyState>
          ) : (
            <>
              <div className="grid grid-cols-[max-content_minmax(0,1fr)] items-center gap-x-3 gap-y-2">
                <span aria-hidden className="text-small font-semibold text-ink-muted">Show</span>
                <FilterChips
                  label="Show only new entries"
                  options={NEW.map((o) => ({ ...o, count: newCount }))}
                  selected={new Set(unreadOnly ? ["new"] : [])}
                  onChange={(next) => setUnreadOnly(next.has("new"))}
                />
                <span aria-hidden className="text-small font-semibold text-ink-muted">Source</span>
                <FilterChips label="Filter by source" options={sourceOptions} selected={sources} onChange={setSources} />
                <span aria-hidden className="text-small font-semibold text-ink-muted">Severity</span>
                <FilterChips label="Filter by severity" options={SEVERITIES} selected={severities} onChange={setSeverities} />
              </div>
              <Timeline
                label="Activity"
                entries={entries}
                now={now}
                height={560}
                onOpen={(entry) => {
                  const event = byId.get(entry.id);
                  if (event) open({ title: entry.summary, content: <EventPanel event={event} /> });
                }}
                empty={
                  filtering ? (
                    <>
                      <p className="m-0">Nothing matches these filters.</p>
                      <ActionButton
                        onClick={() => {
                          setSearch("");
                          setSources(new Set());
                          setSeverities(new Set());
                          setUnreadOnly(false);
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
