import { Check } from "lucide-react";

interface FilterChipsProps<T extends string> {
  label: string;
  options: readonly { value: T; label: string; count?: number | undefined }[];
  /** Selected values; an empty set means "all". */
  selected: ReadonlySet<T>;
  onChange: (selected: Set<T>) => void;
}

/** Toggle filters over a list. Selected chips show a check, not only a color. */
export function FilterChips<T extends string>({ label, options, selected, onChange }: FilterChipsProps<T>) {
  return (
    <div role="group" aria-label={label} className="flex flex-wrap items-center gap-1.5">
      {options.map((option) => {
        const on = selected.has(option.value);
        return (
          <button
            key={option.value}
            type="button"
            aria-pressed={on}
            onClick={() => {
              const next = new Set(selected);
              if (on) next.delete(option.value);
              else next.add(option.value);
              onChange(next);
            }}
            className={`inline-flex h-7 cursor-pointer items-center gap-1 rounded-full border px-2.5 text-small transition-colors duration-150 ${on ? "border-ink bg-ink text-surface" : "border-rule-strong bg-surface text-ink hover:bg-ground"}`}
          >
            {on && <Check aria-hidden size={12} strokeWidth={2.5} />}
            {option.label}
            {option.count !== undefined && (
              <span className={on ? "text-surface" : "text-ink-muted"}>
                <span className="sr-only">, </span>
                {option.count}
              </span>
            )}
          </button>
        );
      })}
    </div>
  );
}
