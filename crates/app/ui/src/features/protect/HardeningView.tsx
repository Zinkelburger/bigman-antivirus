// Protect > Hardening: settings you could change to close a way in, each with
// its evidence, what it costs, and a previewed change or the steps to make it.
import {
  AboutRule,
  ActionButton,
  ActionDialog,
  ago,
  CoverageList,
  hardeningSummary,
  EvidenceBlock,
  IncompleteNotice,
  nowSeconds,
  plural,
  reportStatus,
  SeverityBadge,
  Skeleton,
  StatusLine,
  useDrilldown,
} from "@/design";
import { KeyRound } from "lucide-react";
import { useId, type ReactNode } from "react";
import { useHardening, type Suggestion } from "@/ipc";

/** A titled group of rows; the heading names the list for screen readers. */
function Section({ title, children }: { title: string; children: ReactNode }) {
  const id = useId();
  return (
    <section aria-labelledby={id} className="flex flex-col gap-2">
      <h2 id={id} className="m-0 text-body font-semibold text-ink">
        {title}
      </h2>
      <ul aria-labelledby={id} className="m-0 flex list-none flex-col rounded-panel border border-rule bg-surface p-0">
        {children}
      </ul>
    </section>
  );
}

function Prose({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section>
      <h3 className="m-0 mb-1 text-small font-semibold text-ink-muted">{title}</h3>
      <p className="m-0 max-w-[56ch] text-ink">{children}</p>
    </section>
  );
}

/** A collapsed group whose summary is a heading, so the outline stays whole when it opens. */
function Collapsed({ title, children }: { title: string; children: ReactNode }) {
  return (
    <details className="rounded-panel border border-rule bg-surface p-4">
      <summary className="cursor-pointer text-ink">
        <h2 className="m-0 inline text-body font-semibold">{title}</h2>
      </summary>
      <div className="mt-3">{children}</div>
    </details>
  );
}

/** The fix's control: a previewed change, or nothing when Bigman will not make it. */
function FixControl({ suggestion, variant, describedBy }: { suggestion: Suggestion; variant: "primary" | "secondary"; describedBy?: string }) {
  const { fix } = suggestion;
  if (fix.kind !== "bigman") return null;
  return <ActionDialog action={{ kind: "harden", id: suggestion.id }} label={`${fix.verb}…`} variant={variant} describedBy={describedBy} />;
}

/** Says, before anything is clicked, that the change asks for an administrator password. */
function NeedsPassword({ suggestion }: { suggestion: Suggestion }) {
  if (suggestion.fix.kind !== "bigman" || !suggestion.fix.admin) return null;
  return (
    <span className="inline-flex items-center gap-1 text-small text-ink-muted">
      <KeyRound aria-hidden size={14} />
      Asks for your password
    </span>
  );
}

/** Level three: everything about one suggestion. Reads the review live, so a change shows at once. */
function SuggestionDetail({ id }: { id: string }) {
  const review = useHardening();
  const s = review.data?.suggestions.find((x) => x.id === id);
  if (review.isPending) return <Skeleton shape="rows" count={4} />;
  if (!s) {
    return <p className="m-0 max-w-[56ch] text-ink-muted">This suggestion no longer applies. The setting changed since it was listed.</p>;
  }
  const { fix } = s;
  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-start gap-3">
        <SeverityBadge severity={s.severity} />
        <p className="m-0 max-w-[56ch] text-ink">{s.summary}</p>
      </div>
      {/* What you can do comes right after what was found; the evidence and reasons follow. */}
      {fix.kind === "manual" && <Prose title="How to do it">{fix.steps}</Prose>}
      {fix.kind === "blocked" && <Prose title="Not now">{fix.reason}</Prose>}
      {fix.kind === "bigman" && (
        <p className="m-0 max-w-[56ch] text-small text-ink-muted">
          {fix.admin
            ? "Bigman shows the exact commands first, then polkit asks for your password. The change can be undone."
            : "Bigman shows the exact commands first. No password is needed, and the change can be undone."}
        </p>
      )}
      <div className="flex flex-wrap gap-2">
        <FixControl suggestion={s} variant="primary" />
        <ActionDialog
          action={{ kind: "keep_setting", id: s.id, keep: !s.kept }}
          label={s.kept ? "Suggest again…" : "Leave as it is…"}
          variant="quiet"
        />
      </div>
      {s.evidence && <EvidenceBlock text={s.evidence} label="Evidence" />}
      <Prose title="Why it matters">{s.why}</Prose>
      <Prose title="What it costs">{s.tradeoff}</Prose>
      <AboutRule id={s.rule} />
    </div>
  );
}

function SuggestionRow({ suggestion: s }: { suggestion: Suggestion }) {
  const open = useDrilldown((d) => d.open);
  const details = () => open({ title: s.title, content: <SuggestionDetail id={s.id} /> });
  const titleId = `suggestion-${s.id.replace(/[^a-z0-9]+/gi, "-")}`;
  return (
    <li className="grid grid-cols-[6.5rem_minmax(0,1fr)_auto] items-start gap-x-4 border-b border-rule px-4 py-3 last:border-b-0">
      <span className="pt-0.5">
        <SeverityBadge severity={s.severity} />
      </span>
      <span className="flex min-w-0 max-w-[72ch] flex-col gap-0.5">
        <h3 id={titleId} className="m-0 text-body font-semibold text-ink">
          {s.title}
        </h3>
        <span className="line-clamp-2 text-small text-ink-muted">{s.summary}</span>
        {s.fix.kind === "blocked" && <span className="text-small text-ink">Not now: {s.fix.reason}</span>}
        <NeedsPassword suggestion={s} />
      </span>
      <span className="flex items-center gap-2">
        <FixControl suggestion={s} variant="secondary" describedBy={titleId} />
        <ActionButton variant="quiet" aria-describedby={titleId} onClick={details}>
          {s.fix.kind === "manual" ? "How to do it" : "Details"}
        </ActionButton>
      </span>
    </li>
  );
}

export function HardeningView() {
  const review = useHardening();
  const now = nowSeconds();
  if (review.isPending) return <Skeleton shape="rows" count={5} />;
  if (review.isError) {
    return <IncompleteNotice title="Bigman could not read this computer's settings" errors={[review.error.message]} />;
  }
  const data = review.data;
  const { line, toReview, optional } = hardeningSummary(data);
  const kept = data.suggestions.filter((s) => s.kept);
  const incomplete = reportStatus(data.report) === "Incomplete";
  return (
    <div className="flex flex-col gap-5">
      <div className="flex flex-col gap-1">
        <div className="flex flex-wrap items-center justify-between gap-3">
          {line ? (
            <p className="m-0 text-body font-semibold text-ink">
              {line} <span className="font-normal text-ink-muted">Checked {ago(data.report.created, now)}.</span>
            </p>
          ) : (
            <StatusLine ranAt={data.report.created} now={now} />
          )}
          <ActionButton busy={review.isFetching} onClick={() => void review.refetch()}>
            Check again
          </ActionButton>
        </div>
        <p className="m-0 max-w-[64ch] text-small text-ink-muted">
          Settings you could change to close a way in. Each one says what it costs, and nothing changes until you confirm a
          preview of the exact commands.
        </p>
      </div>
      {incomplete && (
        <IncompleteNotice
          title={`${plural(data.report.coverage.skipped.length, "setting")} could not be checked`}
          coverage={{ ...data.report.coverage, checked: [], notes: [] }}
        />
      )}
      {toReview.length > 0 && (
        <Section title="To review">
          {toReview.map((s) => (
            <SuggestionRow key={s.id} suggestion={s} />
          ))}
        </Section>
      )}
      {optional.length > 0 && (
        <Section title="Optional">
          {optional.map((s) => (
            <SuggestionRow key={s.id} suggestion={s} />
          ))}
        </Section>
      )}
      {data.applied.length > 0 && (
        <Section title="Changed by Bigman">
          {data.applied.map((a) => (
            <li key={a.id} className="flex items-center justify-between gap-4 border-b border-rule px-4 py-2 last:border-b-0">
              <span className="min-w-0">
                <span className="text-ink">{a.title}</span>
                <span className="text-small text-ink-muted"> · {ago(a.at, now)}</span>
              </span>
              <ActionDialog action={{ kind: "undo_hardening", id: a.id }} label="Undo…" variant="quiet" />
            </li>
          ))}
        </Section>
      )}
      {kept.length > 0 && (
        <Section title="Left as it is">
          {kept.map((s) => (
            <li key={s.id} className="flex items-center justify-between gap-4 border-b border-rule px-4 py-2 last:border-b-0">
              <span className="min-w-0 text-ink">{s.title}</span>
              <ActionDialog action={{ kind: "keep_setting", id: s.id, keep: false }} label="Suggest again…" variant="quiet" />
            </li>
          ))}
        </Section>
      )}
      {data.in_place.length > 0 && (
        <Collapsed title={`Already set (${data.in_place.length})`}>
          <dl className="m-0 grid grid-cols-[minmax(10rem,16rem)_minmax(0,1fr)] gap-x-4 gap-y-2">
            {data.in_place.map((c) => (
              <div key={c.check} className="contents">
                <dt className="text-small font-semibold text-ink-muted">{c.check}</dt>
                <dd className="m-0 text-ink">{c.detail}</dd>
              </div>
            ))}
          </dl>
        </Collapsed>
      )}
      <Collapsed title="What Bigman checked">
        <CoverageList coverage={incomplete ? { ...data.report.coverage, skipped: [] } : data.report.coverage} />
      </Collapsed>
    </div>
  );
}
