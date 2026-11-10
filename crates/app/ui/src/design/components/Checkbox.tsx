import { useId, type ReactNode, type Ref } from "react";

interface CheckboxProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  label: string;
  /** Under the label: what choosing it does, or why it is unavailable. */
  description?: ReactNode;
  /**
   * The choice cannot be changed now. The box stays in the tab order, so the
   * description that says why is reached and read out with it.
   */
  disabled?: boolean;
  ref?: Ref<HTMLInputElement> | undefined;
}

/** One choice in a set the user picks several of; a Toggle is for a setting that applies at once. */
export function Checkbox({ checked, onChange, label, description, disabled = false, ref }: CheckboxProps) {
  const id = useId();
  return (
    <div className="flex items-start gap-2">
      <input
        ref={ref}
        id={id}
        type="checkbox"
        checked={checked}
        aria-disabled={disabled || undefined}
        aria-describedby={description ? `${id}-description` : undefined}
        onChange={(e) => {
          if (!disabled) onChange(e.target.checked);
        }}
        className="mt-1 size-4 shrink-0 cursor-pointer accent-[var(--accent)] aria-disabled:cursor-not-allowed aria-disabled:opacity-60"
      />
      <div className="flex min-w-0 flex-col">
        {/* Dimmed through the muted ink, not opacity, so the reason below keeps its contrast. */}
        <label htmlFor={id} className={`text-body ${disabled ? "cursor-not-allowed text-ink-muted" : "text-ink"}`}>
          {label}
        </label>
        {description && (
          <span id={`${id}-description`} className="text-small text-ink-muted">
            {description}
          </span>
        )}
      </div>
    </div>
  );
}
