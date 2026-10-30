/**
 * The life-list mark: a ringed dot in the accent, the way a field notebook circles
 * a first sighting. Shape plus text, so it never depends on color alone.
 */
export function FirstSeenMarker({ compact = false }: { compact?: boolean }) {
  const mark = (
    <svg aria-hidden width="12" height="12" viewBox="0 0 12 12" className="shrink-0 text-accent">
      <circle cx="6" cy="6" r="4.75" fill="none" stroke="currentColor" strokeWidth="1.5" />
      <circle cx="6" cy="6" r="2" fill="currentColor" />
    </svg>
  );
  if (compact) {
    return (
      <span className="inline-flex items-center" title="First seen">
        {mark}
        <span className="sr-only">First seen</span>
      </span>
    );
  }
  return (
    <span className="inline-flex shrink-0 items-center gap-1 text-small font-semibold text-accent">
      {mark}
      First seen
    </span>
  );
}
