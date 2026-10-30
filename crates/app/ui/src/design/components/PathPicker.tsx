import { FolderOpen } from "lucide-react";
import { useId } from "react";
import { ActionButton } from "./ActionButton";

interface PathPickerProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  /** Opens the desktop's file chooser; absent in browser dev mode. */
  onBrowse?: (() => void) | undefined;
  placeholder?: string;
}

/** A path typed or chosen. Validation happens in core, exactly as for the CLI. */
export function PathPicker({ label, value, onChange, onBrowse, placeholder }: PathPickerProps) {
  const id = useId();
  return (
    <div className="flex flex-col gap-1">
      <label htmlFor={id} className="text-small font-semibold text-ink-muted">
        {label}
      </label>
      <div className="flex gap-2">
        <input
          id={id}
          value={value}
          spellCheck={false}
          placeholder={placeholder}
          onChange={(e) => onChange(e.target.value)}
          className="h-8 min-w-0 flex-1 rounded-control border border-rule-strong bg-surface px-3 font-mono text-small text-ink placeholder:text-ink-muted"
        />
        {onBrowse && (
          <ActionButton icon={FolderOpen} onClick={onBrowse}>
            Browse…
          </ActionButton>
        )}
      </div>
    </div>
  );
}
