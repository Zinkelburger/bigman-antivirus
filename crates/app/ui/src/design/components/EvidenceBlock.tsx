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
}

export function EvidenceBlock({
  text,
  label,
  highlight = [],
  startLine = 1,
  notes = {},
  maxLines = 2_000,
}: EvidenceBlockProps) {
  const [copied, setCopied] = useState(false);
  const lines = text.split("\n");
  const shown = lines.slice(0, maxLines);
  const marked = new Set(highlight);
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
        <button
          type="button"
          onClick={() => void copy()}
          className="inline-flex cursor-pointer items-center gap-1 rounded-control px-1.5 py-0.5 text-small text-ink-muted hover:bg-ground"
        >
          {copied ? <Check aria-hidden size={13} /> : <Copy aria-hidden size={13} />}
          {copied ? "Copied" : "Copy"}
        </button>
      </figcaption>
      <pre className="m-0 overflow-x-auto py-2 font-mono text-small leading-5 text-ink">
        {shown.map((line, i) => {
          const n = startLine + i;
          const hit = marked.has(n);
          return (
            <div
              key={i}
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
          );
        })}
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
