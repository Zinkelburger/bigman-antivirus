import { Check, Copy } from "lucide-react";
import { useState } from "react";
import { formatCount } from "../format";

interface EvidenceBlockProps {
  /** Already cleaned in Rust (control characters escaped). Rendered as text, never HTML. */
  text: string;
  label?: string | undefined;
  /** 1-based line numbers to highlight. */
  highlight?: readonly number[] | undefined;
  /** Number of the first line, for excerpts. */
  startLine?: number;
  /** A note beside a highlighted line, such as the rule that matched, in its severity's color. */
  notes?: Readonly<Record<number, { text: string; tone: "review" | "high" }>> | undefined;
  maxLines?: number;
  /** A long block with highlights opens folded to those lines and a few around them. */
  fold?: boolean;
}

/** Lines kept on each side of a highlighted line when folded. */
const CONTEXT = 3;
/** Blocks shorter than this are never folded. */
const FOLD_FROM = 30;

export function EvidenceBlock({
  text,
  label,
  highlight = [],
  startLine = 1,
  notes = {},
  maxLines = 2_000,
  fold = false,
}: EvidenceBlockProps) {
  const [copied, setCopied] = useState(false);
  const [expanded, setExpanded] = useState(false);
  const lines = text.split("\n");
  const shown = lines.slice(0, maxLines);
  const marked = new Set(highlight);
  const folded = fold && !expanded && highlight.length > 0 && lines.length >= FOLD_FROM;
  // Index into `shown` of each line to render; a gap between two kept lines
  // becomes one "N lines not shown" row.
  const kept = folded
    ? shown.map((_, i) => i).filter((i) => highlight.some((h) => Math.abs(h - (startLine + i)) <= CONTEXT))
    : shown.map((_, i) => i);
  const numbered = lines.length > 1 || highlight.length > 0;
  const copy = async () => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(true);
      setTimeout(() => setCopied(false), 1_500);
    } catch {
      setCopied(false);
    }
  };
  return (
    <figure className="m-0 flex min-w-0 flex-col overflow-hidden rounded-panel border border-rule bg-sunken">
      <figcaption className="flex items-center justify-between gap-2 border-b border-rule px-3 py-1">
        <span className="text-small font-semibold text-ink-muted">{label ?? "Evidence"}</span>
        <span className="flex items-center gap-1">
        {fold && highlight.length > 0 && lines.length >= FOLD_FROM && (
          <button
            type="button"
            onClick={() => setExpanded((e) => !e)}
            className="cursor-pointer rounded-control px-1.5 py-0.5 text-small text-ink-muted hover:bg-ground"
          >
            {expanded ? "Show matched lines only" : `Show all ${formatCount(lines.length)} lines`}
          </button>
        )}
        <button
          type="button"
          onClick={() => void copy()}
          className="inline-flex cursor-pointer items-center gap-1 rounded-control px-1.5 py-0.5 text-small text-ink-muted hover:bg-ground"
        >
          {copied ? <Check aria-hidden size={13} /> : <Copy aria-hidden size={13} />}
          {copied ? "Copied" : "Copy"}
        </button>
        </span>
      </figcaption>
      <pre className="relative m-0 overflow-x-auto py-2 font-mono text-small leading-5 text-ink">
        {kept.map((i, k) => {
          const line = shown[i] ?? "";
          const n = startLine + i;
          const hit = marked.has(n);
          const skipped = i - (k === 0 ? -1 : (kept[k - 1] ?? 0)) - 1;
          return (
            <div key={i}>
            {skipped > 0 && (
              <div className="px-3 py-0.5 font-sans text-small text-ink-muted">
                {skipped === 1 ? "1 line not shown" : `${formatCount(skipped)} lines not shown`}
              </div>
            )}
            <div
              className={`grid grid-cols-[auto_minmax(0,1fr)] gap-3 px-3 ${hit ? (notes[n]?.tone === "high" ? "bg-high-soft" : "bg-review-soft") : ""}`}
            >
              {numbered ? (
                <span aria-hidden className="w-8 text-right text-ink-muted select-none">
                  {n}
                </span>
              ) : (
                <span />
              )}
              <span className="break-words whitespace-pre-wrap">
                {line || " "}
                {hit && notes[n] && (
                  <span className={`ml-3 font-sans font-semibold ${notes[n].tone === "high" ? "text-high" : "text-review"}`}>
                    {notes[n].text}
                  </span>
                )}
              </span>
            </div>
            </div>
          );
        })}
        {folded && (kept.at(-1) ?? 0) < shown.length - 1 && (
          <div className="px-3 py-0.5 font-sans text-small text-ink-muted">
            {formatCount(shown.length - 1 - (kept.at(-1) ?? 0))} more lines not shown
          </div>
        )}
      </pre>
      {lines.length > shown.length && (
        <p className="border-t border-rule px-3 py-1 text-small text-ink-muted">
          Showing the first {formatCount(shown.length)} of {formatCount(lines.length)} lines. Copy or export
          the report for the rest.
        </p>
      )}
    </figure>
  );
}
