import type { ReactNode } from "react";

interface EmptyStateProps {
  /** One sentence: what would appear here. */
  children: ReactNode;
  /** One action that makes it appear. */
  action?: ReactNode;
}

export function EmptyState({ children, action }: EmptyStateProps) {
  return (
    <div className="flex flex-col items-start gap-3 rounded-panel border border-dashed border-rule-strong px-5 py-6">
      <p className="max-w-[60ch] text-body text-ink-muted">{children}</p>
      {action}
    </div>
  );
}
