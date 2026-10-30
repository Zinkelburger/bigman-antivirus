export interface DiffLine {
  kind: "add" | "remove" | "context";
  text: string;
  /** Rule that matched this line, shown beside it. */
  rule?: string | undefined;
}

export interface DiffHunk {
  header: string;
  /** Line numbers of the hunk's first line in the old and new file. */
  oldStart: number;
  newStart: number;
  lines: DiffLine[];
}

const SIGN = { add: "+", remove: "−", context: " " } as const;
// Neutral change marks: the accent means first-seen and interaction, the review tint means a rule matched.
const ROW = { add: "bg-sunken", remove: "text-ink-muted", context: "" } as const;

/**
 * Before and after share one scale and fixed gutters, so the two versions line up
 * column for column.
 */
export function DiffView({ hunks, label = "Changes" }: { hunks: DiffHunk[]; label?: string }) {
  return (
    <figure className="m-0 overflow-hidden rounded-panel border border-rule bg-surface">
      <figcaption className="border-b border-rule px-3 py-1 text-small font-semibold text-ink-muted">{label}</figcaption>
      <div className="overflow-x-auto font-mono text-small leading-5">
        {hunks.map((hunk, h) => {
          let oldLine = hunk.oldStart;
          let newLine = hunk.newStart;
          return (
            <section key={h} aria-label={hunk.header}>
              <div className="border-y border-rule bg-sunken px-3 py-0.5 text-ink-muted">{hunk.header}</div>
              {hunk.lines.map((line, i) => {
                const o = line.kind === "add" ? "" : oldLine++;
                const n = line.kind === "remove" ? "" : newLine++;
                return (
                  <div
                    key={i}
                    className={`grid grid-cols-[2.5rem_2.5rem_1rem_minmax(0,1fr)] px-1 ${line.rule ? "bg-review-soft" : ROW[line.kind]}`}
                  >
                    <span aria-hidden className="pr-2 text-right text-ink-muted select-none">{o}</span>
                    <span aria-hidden className="pr-2 text-right text-ink-muted select-none">{n}</span>
                    <span className="text-ink-muted select-none">
                      <span aria-hidden>{SIGN[line.kind]}</span>
                      <span className="sr-only">{line.kind === "context" ? "unchanged" : line.kind === "add" ? "added" : "removed"}</span>
                    </span>
                    <span className="break-words whitespace-pre-wrap text-ink">
                      {line.text || " "}
                      {line.rule && <span className="ml-3 font-sans font-semibold text-review">{line.rule}</span>}
                    </span>
                  </div>
                );
              })}
            </section>
          );
        })}
      </div>
    </figure>
  );
}
