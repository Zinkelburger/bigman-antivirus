import { ChevronDown } from "lucide-react";
import { useId } from "react";

interface SelectProps<T extends string> {
  label: string;
  value: T;
  options: readonly { value: T; label: string }[];
  onChange: (value: T) => void;
  /** Hide the visible label when the context already names the control. */
  hideLabel?: boolean;
}

/** The platform's native select, styled to the tokens. */
export function Select<T extends string>({ label, value, options, onChange, hideLabel = false }: SelectProps<T>) {
  const id = useId();
  return (
    <div className="flex items-center gap-2">
      <label htmlFor={id} className={hideLabel ? "sr-only" : "text-small font-semibold text-ink-muted"}>
        {label}
      </label>
      <div className="relative">
        <select
          id={id}
          value={value}
          onChange={(e) => {
            const next = options.find((o) => o.value === e.target.value);
            if (next) onChange(next.value);
          }}
          className="h-8 cursor-pointer appearance-none rounded-control border border-rule-strong bg-surface py-0 pr-8 pl-3 text-body text-ink hover:bg-ground"
        >
          {options.map((o) => (
            <option key={o.value} value={o.value}>
              {o.label}
            </option>
          ))}
        </select>
        <ChevronDown aria-hidden size={14} className="pointer-events-none absolute top-1/2 right-2.5 -translate-y-1/2 text-ink-muted" />
      </div>
    </div>
  );
}
