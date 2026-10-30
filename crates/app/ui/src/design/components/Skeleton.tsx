const BAR =
  "rounded-control bg-[linear-gradient(90deg,var(--rule)_0%,var(--rule-strong)_50%,var(--rule)_100%)] bg-[length:200%_100%] animate-[bigman-shimmer_1.4s_ease-in-out_infinite]";

/** Loading placeholder in the shape of the real content; never a spinner. */
export function Skeleton({ shape, count = 3 }: { shape: "tiles" | "rows" | "block"; count?: number }) {
  if (shape === "tiles") {
    return (
      <div aria-busy className="grid grid-cols-4 gap-3" aria-label="Loading">
        {Array.from({ length: count }, (_, i) => (
          <div key={i} className="flex min-h-28 min-w-0 flex-col gap-2 overflow-hidden rounded-panel border border-rule bg-surface p-4">
            <span className={`h-4 w-24 ${BAR}`} />
            <span className={`h-7 w-16 ${BAR}`} />
            <span className={`h-4 w-36 max-w-full ${BAR}`} />
          </div>
        ))}
      </div>
    );
  }
  if (shape === "rows") {
    return (
      <div aria-busy aria-label="Loading" className="flex flex-col">
        {Array.from({ length: count }, (_, i) => (
          <div key={i} className="flex items-center gap-3 border-b border-rule py-2.5">
            <span className={`h-4 w-12 ${BAR}`} />
            <span className={`h-4 flex-1 ${BAR}`} style={{ maxWidth: `${60 + ((i * 17) % 30)}%` }} />
            <span className={`h-4 w-14 ${BAR}`} />
          </div>
        ))}
      </div>
    );
  }
  return <div aria-busy aria-label="Loading" className={`h-40 w-full ${BAR}`} />;
}
