import { LoaderCircle, type LucideIcon } from "lucide-react";
import type { ButtonHTMLAttributes } from "react";

interface ActionButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "quiet";
  icon?: LucideIcon | undefined;
  busy?: boolean;
}

const VARIANT = {
  primary: "bg-accent text-accent-ink border-accent hover:brightness-110",
  secondary: "bg-surface text-ink border-rule-strong hover:bg-ground",
  quiet: "bg-transparent text-ink border-transparent hover:bg-sunken",
};

/** Every action control. Names its action; shows progress inline while busy. */
export function ActionButton({
  variant = "secondary",
  icon: Icon,
  busy = false,
  disabled,
  children,
  className = "",
  type = "button",
  ...rest
}: ActionButtonProps) {
  return (
    <button
      type={type}
      disabled={disabled || busy}
      aria-busy={busy || undefined}
      className={`inline-flex h-8 shrink-0 cursor-pointer items-center justify-center gap-1.5 rounded-control border px-3 text-body font-semibold whitespace-nowrap transition-[background-color,filter] duration-150 disabled:cursor-not-allowed disabled:opacity-50 ${VARIANT[variant]} ${className}`}
      {...rest}
    >
      {busy ? (
        <LoaderCircle aria-hidden size={15} className="animate-spin" />
      ) : (
        Icon && <Icon aria-hidden size={15} strokeWidth={2} />
      )}
      {children}
    </button>
  );
}
