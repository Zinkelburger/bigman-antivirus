/**
 * Field tally marks for repeats: four strokes and a bar per five, on one fixed
 * ramp. Beyond twenty the figure carries the count alone. Decorative; the
 * accompanying text states the number.
 */
export function Tally({ count }: { count: number }) {
  if (count < 2) return null;
  const marks = Math.min(count, 20);
  const groups = Math.ceil(marks / 5);
  const width = groups * 16 - 4;
  return (
    <svg aria-hidden width={width} height="12" viewBox={`0 0 ${width} 12`} className="shrink-0 text-ink-muted">
      {Array.from({ length: groups }, (_, g) => {
        const inGroup = Math.min(5, marks - g * 5);
        const x = g * 16;
        return (
          <g key={g} stroke="currentColor" strokeWidth="1.4" strokeLinecap="round">
            {Array.from({ length: Math.min(4, inGroup) }, (_, i) => (
              <line key={i} x1={x + 1 + i * 3} y1="1.5" x2={x + 1 + i * 3} y2="10.5" />
            ))}
            {inGroup === 5 && <line x1={x - 0.5} y1="9.5" x2={x + 11.5} y2="2.5" />}
          </g>
        );
      })}
    </svg>
  );
}
