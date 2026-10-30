import { Search, X } from "lucide-react";
import { forwardRef } from "react";

interface SearchFieldProps {
  value: string;
  onChange: (value: string) => void;
  label?: string;
  placeholder?: string;
}

/** Filters the current view. The shell focuses it when `/` is pressed. */
export const SearchField = forwardRef<HTMLInputElement, SearchFieldProps>(function SearchField(
  { value, onChange, label = "Search", placeholder = "Search" },
  ref,
) {
  return (
    <div className="relative w-64 max-w-full">
      <Search aria-hidden size={14} className="pointer-events-none absolute top-1/2 left-2.5 -translate-y-1/2 text-ink-muted" />
      <input
        ref={ref}
        type="search"
        data-search-field
        aria-label={label}
        placeholder={`${placeholder}  /`}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === "Escape" && value) {
            e.stopPropagation();
            onChange("");
          }
        }}
        className="h-8 w-full rounded-control border border-rule-strong bg-surface pr-8 pl-8 text-body text-ink placeholder:text-ink-muted [&::-webkit-search-cancel-button]:hidden"
      />
      {value && (
        <button
          type="button"
          aria-label="Clear search"
          onClick={() => onChange("")}
          className="absolute top-1/2 right-1.5 -translate-y-1/2 cursor-pointer rounded-control p-1 text-ink-muted hover:bg-sunken"
        >
          <X aria-hidden size={13} />
        </button>
      )}
    </div>
  );
});
