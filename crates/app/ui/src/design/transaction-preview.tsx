// A package transaction waiting in the package manager (the opt-in hooks). It goes
// ahead on its own when the minute is up, so the one real answer is to stop it.
// Hiding the dialog keeps the preview waiting; the status strip can reopen it.
import { useEffect, useState } from "react";
import { create } from "zustand";
import { decide, setPreviewHandler, type Decision, type Waiting } from "@/ipc";
import { ActionButton } from "./components/ActionButton";
import { PreviewDialog } from "./components/PreviewDialog";
import { useToasts } from "./components/Toast";
import { nowSeconds, plural } from "./format";
import { ReceiptPanel } from "./receipt";

interface Entry {
  waiting: Waiting;
  /** Hidden with "Hide"; still waiting, reopened from the status strip. */
  hidden: boolean;
  /** Answered from this window; its dialog shows the result until closed. */
  answered: boolean;
}

interface PreviewStore {
  entries: Entry[];
  add: (waiting: Waiting) => void;
  show: (id: string) => void;
  hide: (id: string) => void;
  answered: (id: string) => void;
  /** The daemon says a preview is over: answered here, elsewhere, or timed out. */
  done: (decision: Decision) => void;
  remove: (id: string) => void;
}

export const usePreviews = create<PreviewStore>((set, get) => ({
  entries: [],
  add: (waiting) =>
    set((s) =>
      s.entries.some((e) => e.waiting.receipt.id === waiting.receipt.id)
        ? s
        : { entries: [...s.entries, { waiting, hidden: false, answered: false }] },
    ),
  show: (id) => set((s) => ({ entries: s.entries.map((e) => (e.waiting.receipt.id === id ? { ...e, hidden: false } : e)) })),
  hide: (id) => set((s) => ({ entries: s.entries.map((e) => (e.waiting.receipt.id === id ? { ...e, hidden: true } : e)) })),
  answered: (id) => set((s) => ({ entries: s.entries.map((e) => (e.waiting.receipt.id === id ? { ...e, answered: true } : e)) })),
  done: (decision) => {
    const entry = get().entries.find((e) => e.waiting.receipt.id === decision.id);
    if (!entry || entry.answered) return;
    const manager = entry.waiting.receipt.manager;
    useToasts
      .getState()
      .push(
        decision.by === "timeout"
          ? `${manager} went ahead after a minute without an answer. The receipt is in Activity.`
          : decision.proceed
            ? `${manager} went ahead; it was answered in another window.`
            : `${manager} stopped the transaction; it was answered in another window.`,
      );
    get().remove(decision.id);
  },
  remove: (id) => set((s) => ({ entries: s.entries.filter((e) => e.waiting.receipt.id !== id) })),
}));

setPreviewHandler((live) => {
  if (live.type === "preview") usePreviews.getState().add(live.value);
  else usePreviews.getState().done(live.value);
});

/** Seconds left, ticking; re-renders once a second while anything waits. */
function useSecondsLeft(deadline: number | undefined) {
  const [now, setNow] = useState(nowSeconds);
  useEffect(() => {
    if (deadline === undefined) return;
    const timer = setInterval(() => setNow(nowSeconds()), 1_000);
    return () => clearInterval(timer);
  }, [deadline]);
  return deadline === undefined ? 0 : Math.max(0, deadline - now);
}

/** "dnf is waiting for you · Open" in the status strip while a preview is hidden. */
export function WaitingIndicator() {
  const hidden = usePreviews((s) => s.entries.find((e) => e.hidden && !e.answered));
  const show = usePreviews((s) => s.show);
  if (!hidden) return null;
  return (
    <span className="inline-flex items-center gap-2 font-semibold text-ink">
      {hidden.waiting.receipt.manager} is waiting for you
      <ActionButton variant="quiet" onClick={() => show(hidden.waiting.receipt.id)}>
        Open
      </ActionButton>
    </span>
  );
}

function packages(names: string[]): string {
  return names.length <= 3 ? names.join(", ") : `${names.slice(0, 2).join(", ")} and ${String(names.length - 2)} more`;
}

export function TransactionPreviewHost() {
  const entries = usePreviews((s) => s.entries);
  const { answered, hide, remove } = usePreviews();
  const visible = entries.filter((e) => !e.hidden);
  const current = visible[0];
  const left = useSecondsLeft(current?.waiting.deadline);
  if (!current) return null;
  const { receipt } = current.waiting;
  const manager = receipt.manager;
  const names = packages(receipt.packages.map((p) => p.name));
  const queue = entries.length > 1 ? ` (1 of ${String(entries.length)})` : "";
  const stop = async () => {
    answered(receipt.id);
    await decide(receipt.id, false);
    return (
      <p className="m-0 max-w-[56ch] text-ink">
        {manager} stops the transaction and reports that it was stopped from Bigman&apos;s receipt preview. No package
        was changed.
      </p>
    );
  };
  const fraction = Math.min(1, left / 60);
  return (
    <PreviewDialog
      key={receipt.id}
      open
      modalOnly
      onOpenChange={(open) => {
        if (open) return;
        // Closing after an answer clears it; closing before keeps it waiting.
        if (current.answered) remove(receipt.id);
        else hide(receipt.id);
      }}
      title={`A package change is waiting${queue}`}
      description={`${manager} wants to change ${plural(receipt.packages.length, "package")}: ${names}. It goes ahead in ${plural(left, "second")} unless you stop it.`}
      planHeading={`${manager} wants to change ${plural(receipt.packages.length, "package")}: ${names}`}
      plan={<ReceiptPanel receipt={receipt} expanded />}
      cancelLabel="Hide"
      confirmLabel="Stop the transaction"
      ready={left > 0}
      onConfirm={stop}
      footer={
        <div className="flex flex-col gap-1">
          <span className="font-semibold text-ink">
            {left > 0 ? `Goes ahead in ${plural(left, "second")} unless you stop it` : `${manager} is going ahead`}
          </span>
          <span aria-hidden className="block h-1 w-48 overflow-hidden rounded-full bg-sunken">
            <span className="block h-full rounded-full bg-ink transition-[width] duration-1000 ease-linear" style={{ width: `${String(fraction * 100)}%` }} />
          </span>
          {/* Announced at 30 and 10 seconds only, not every tick. */}
          <span className="sr-only" aria-live="polite">
            {left === 30 || left === 10 ? `${plural(left, "second")} left to stop ${manager}` : ""}
          </span>
        </div>
      }
    />
  );
}
