import { ActionButton } from "./ActionButton";

interface ProgressRowProps {
  label: string;
  detail?: string | undefined;
  /** 0 to 1 when known; null shows an indeterminate bar. */
  progress: number | null;
  onCancel?: (() => void) | undefined;
}

/** A long-running job (scan, update, rehearsal) with a cancel action. */
export function ProgressRow({ label, detail, progress, onCancel }: ProgressRowProps) {
  const percent = progress == null ? null : Math.round(Math.min(1, Math.max(0, progress)) * 100);
  return (
    <div className="flex items-center gap-4 rounded-panel border border-rule bg-surface px-4 py-3">
      <div className="flex min-w-0 flex-1 flex-col gap-1.5">
        <div className="flex items-baseline justify-between gap-3">
          <span className="truncate font-semibold text-ink">{label}</span>
          {percent != null && <span className="text-small text-ink-muted">{percent}%</span>}
        </div>
        <div
          role="progressbar"
          aria-label={label}
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={percent ?? undefined}
          className="h-1 overflow-hidden rounded-full bg-sunken"
        >
          {percent == null ? (
            <div className="h-full w-1/3 rounded-full bg-ink-muted animate-[bigman-indeterminate_1.4s_ease-in-out_infinite]" />
          ) : (
            <div
              className="h-full origin-left rounded-full bg-ink transition-transform duration-300"
              style={{ transform: `scaleX(${percent / 100})` }}
            />
          )}
        </div>
        {detail && <span className="truncate text-small text-ink-muted">{detail}</span>}
      </div>
      {onCancel && (
        <ActionButton variant="quiet" onClick={onCancel}>
          Cancel
        </ActionButton>
      )}
    </div>
  );
}
