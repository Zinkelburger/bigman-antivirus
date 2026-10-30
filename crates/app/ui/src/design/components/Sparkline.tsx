interface SparklineProps {
  values: readonly number[];
  /** Spoken summary, e.g. "Connections per hour, last 24 hours". */
  label: string;
  width?: number;
  height?: number;
}

/** A trend beside real figures; never a stand-in for them. */
export function Sparkline({ values, label, width = 96, height = 24 }: SparklineProps) {
  if (values.length < 2) return null;
  const max = Math.max(...values, 1);
  const step = width / (values.length - 1);
  const points = values.map((v, i) => `${(i * step).toFixed(1)},${(height - 1 - (v / max) * (height - 2)).toFixed(1)}`);
  return (
    <svg role="img" aria-label={label} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="shrink-0 text-ink-muted">
      <polyline points={points.join(" ")} fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinejoin="round" strokeLinecap="round" />
    </svg>
  );
}
