import { formatTime, signatureAge } from "../format";

interface StatusLineProps {
  /** When the checks ran (Unix seconds). */
  ranAt: number;
  /** Newest signature file time, when signatures were part of the checks. */
  signaturesUpdated?: number | null | undefined;
  now: number;
}

/**
 * The only calm state Bigman shows: "No findings from these checks." with when
 * they ran. It never says the machine is safe.
 */
export function StatusLine({ ranAt, signaturesUpdated, now }: StatusLineProps) {
  const signatures =
    signaturesUpdated != null ? ` with signatures ${signatureAge(signaturesUpdated, now)}` : "";
  return (
    <p className="flex items-center gap-2 text-body text-ink">
      <span aria-hidden className="size-2 shrink-0 rounded-full bg-calm" />
      <span>
        No findings from these checks. Ran at {formatTime(ranAt)}
        {signatures}.
      </span>
    </p>
  );
}
