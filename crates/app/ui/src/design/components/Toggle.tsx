import { useId } from "react";

interface ToggleProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  label: string;
  description?: string | undefined;
  disabled?: boolean;
}

export function Toggle({ checked, onChange, label, description, disabled = false }: ToggleProps) {
  const id = useId();
  return (
    <div className="flex items-start justify-between gap-4">
      <div className="flex flex-col">
        <label htmlFor={id} className="text-body font-semibold text-ink">
          {label}
        </label>
        {description && (
          <span id={`${id}-description`} className="text-small text-ink-muted">
            {description}
          </span>
        )}
      </div>
      <button
        id={id}
        type="button"
        role="switch"
        aria-checked={checked}
        aria-describedby={description ? `${id}-description` : undefined}
        disabled={disabled}
        onClick={() => onChange(!checked)}
        className={`relative mt-0.5 h-5 w-9 shrink-0 cursor-pointer rounded-full border transition-colors duration-150 disabled:cursor-not-allowed disabled:opacity-50 ${checked ? "border-accent bg-accent" : "border-rule-strong bg-sunken"}`}
      >
        <span
          aria-hidden
          className={`absolute top-0.5 size-3.5 rounded-full transition-[left] duration-150 ${checked ? "left-[18px] bg-accent-ink" : "left-0.5 bg-ink-muted"}`}
        />
      </button>
    </div>
  );
}
