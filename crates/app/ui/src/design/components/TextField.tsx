import { useId } from "react";

interface TextFieldProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  /** One line under the field: format or where to find the value. */
  hint?: string | undefined;
  placeholder?: string;
  mono?: boolean;
}

/** Free text that is not a path: URLs, extension IDs. Validated in core when used. */
export function TextField({ label, value, onChange, hint, placeholder, mono = false }: TextFieldProps) {
  const id = useId();
  return (
    <div className="flex flex-col gap-1">
      <label htmlFor={id} className="text-small font-semibold text-ink-muted">
        {label}
      </label>
      <input
        id={id}
        value={value}
        spellCheck={false}
        placeholder={placeholder}
        aria-describedby={hint ? `${id}-hint` : undefined}
        onChange={(e) => onChange(e.target.value)}
        className={`h-8 min-w-0 rounded-control border border-rule-strong bg-surface px-3 text-ink placeholder:text-ink-muted ${mono ? "font-mono text-small" : "text-body"}`}
      />
      {hint && (
        <span id={`${id}-hint`} className="text-small text-ink-muted">
          {hint}
        </span>
      )}
    </div>
  );
}
