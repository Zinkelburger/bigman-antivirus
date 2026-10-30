import type { ReactNode } from "react";

export interface KeyValueItem {
  label: string;
  value: ReactNode;
  mono?: boolean;
}

export function KeyValue({ items }: { items: KeyValueItem[] }) {
  return (
    <dl className="grid grid-cols-[max-content_minmax(0,1fr)] gap-x-6 gap-y-1.5 text-body">
      {items.map((item) => (
        <div key={item.label} className="contents">
          <dt className="text-small font-semibold text-ink-muted">{item.label}</dt>
          <dd className={`m-0 break-words text-ink ${item.mono ? "font-mono text-small" : ""}`}>{item.value}</dd>
        </div>
      ))}
    </dl>
  );
}
