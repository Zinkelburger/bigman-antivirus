import type { ReactNode } from "react";

interface PageHeaderProps {
  title: string;
  /** Time-range picker or filters that scope the whole page. */
  controls?: ReactNode;
  /** The page's one primary action. */
  action?: ReactNode;
}

export function PageHeader({ title, controls, action }: PageHeaderProps) {
  return (
    <header className="flex min-h-14 flex-wrap items-center gap-x-4 gap-y-2 border-b border-rule px-6 py-3">
      <h1 className="m-0 mr-auto text-title font-semibold text-ink">{title}</h1>
      {controls}
      {action}
    </header>
  );
}
