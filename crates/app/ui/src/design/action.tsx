// State changes, always previewed: the daemon describes exactly what an Action
// will do, the user confirms, and the result (with any report) is shown.
// One host renders the dialog, so a trigger that disappears after the change
// (a restored file, a disarmed canary) never takes the result with it.
import { useState, type ReactNode } from "react";
import { create } from "zustand";
import { useAction, usePreview, type Action, type ActionPlan, type Report } from "@/ipc";
import { ActionButton } from "./components/ActionButton";
import { ConfirmDanger } from "./components/ConfirmDanger";
import { EvidenceBlock } from "./components/EvidenceBlock";
import { KeyValue } from "./components/KeyValue";
import { PreviewDialog } from "./components/PreviewDialog";
import { Skeleton } from "./components/Skeleton";
import { nowSeconds } from "./format";
import { ReportPanel } from "./report";

interface OpenAction {
  action: Action;
  label: string;
  onDone?: ((message: string) => void) | undefined;
}

interface ActionStore {
  current: OpenAction | null;
  /** Where focus returns when the dialog closes. */
  opener: HTMLElement | null;
  /** The opener's own group of buttons, its neighbouring rows and its panel, for when the change removes the opener. */
  fallbacks: Element[];
  set: (current: OpenAction | null) => void;
}

const FOCUSABLE = "button:not([disabled]), a[href], [tabindex]:not([tabindex='-1'])";

const useOpenAction = create<ActionStore>((set) => ({
  current: null,
  opener: null,
  fallbacks: [],
  set: (current) => {
    if (current) {
      const opener = document.activeElement instanceof HTMLElement ? document.activeElement : null;
      const row = opener?.closest("li");
      // The opener's parent first: a change often swaps the button for its
      // counterpart in the same place ("Mark as known…" for "Take back the mark…").
      const fallbacks = [opener?.parentElement, row?.nextElementSibling, row?.previousElementSibling, opener?.closest("aside, section")];
      set({ current, opener, fallbacks: fallbacks.filter((e): e is Element => e != null) });
      return;
    }
    set({ current: null });
    // Radix returns focus through onCloseAutoFocus; this catches a close that bypassed it.
    requestAnimationFrame(() => {
      if (!document.activeElement || document.activeElement === document.body) restoreFocus();
    });
  },
}));

/**
 * Focus after the dialog: the button that opened it, or, when the change removed
 * that button (a fixed suggestion, a restored file, a row now marked), the
 * button that took its place, then the next row, then the opener's panel, then
 * the page heading.
 */
function restoreFocus(): void {
  const { opener, fallbacks } = useOpenAction.getState();
  const near = fallbacks
    .filter((e) => e.isConnected)
    .map((e) => e.querySelector<HTMLElement>(FOCUSABLE))
    .find((e) => e !== null);
  (opener?.isConnected ? opener : (near ?? document.querySelector<HTMLElement>("main h1")))?.focus();
}

/** Opens the previewed change in the shared dialog, for a trigger that is not a button of its own (a table row). */
export function openActionDialog(action: Action, label: string, onDone?: (message: string) => void): void {
  useOpenAction.getState().set({ action, label, onDone });
}

/** Closes the open action dialog, if any. */
export function closeActionDialog(): void {
  useOpenAction.getState().set(null);
}

/** Plan facts whose values are paths, commands or identifiers, set as machine text. */
const MACHINE_FACTS = new Set(["Copied in", "Program", "Process ID", "Command line", "Runs in"]);

function Plan({ plan }: { plan: ActionPlan }) {
  return (
    <>
      {/* The daemon separates paragraphs with a blank line: what it costs, then who runs it. */}
      {plan.summary.split("\n\n").map((paragraph) => (
        <p key={paragraph} className="m-0 max-w-[56ch] text-pretty text-ink">
          {paragraph}
        </p>
      ))}
      {plan.note && <p className="m-0 max-w-[56ch] text-pretty text-ink">{plan.note}</p>}
      {plan.facts.length > 0 && (
        <KeyValue items={plan.facts.map((f) => ({ label: f.label, value: f.value, mono: MACHINE_FACTS.has(f.label) }))} />
      )}
      {plan.details && <EvidenceBlock text={plan.details} label={plan.details_label ?? "Exactly what changes"} />}
      {plan.undo && (
        <p className="m-0 text-small text-ink-muted">
          <span className="font-semibold">To undo: </span>
          {plan.undo}
        </p>
      )}
    </>
  );
}

interface ActionDialogProps {
  action: Action;
  /** The button that opens the preview. */
  label: string;
  variant?: "primary" | "secondary" | "quiet";
  disabled?: boolean;
  /** The id of the element naming what this button acts on, when several buttons share a label. */
  describedBy?: string | undefined;
  /** Called after the change succeeds, for views that react to it. The dialog itself shows the result. */
  onDone?: ((message: string) => void) | undefined;
}

/** A button that opens the previewed change in the shared dialog. */
export function ActionDialog({ action, label, variant = "secondary", disabled = false, describedBy, onDone }: ActionDialogProps) {
  const open = useOpenAction((s) => s.set);
  return (
    <ActionButton variant={variant} disabled={disabled} aria-describedby={describedBy} onClick={() => open({ action, label, onDone })}>
      {label}
    </ActionButton>
  );
}

function Outcome({ message, report }: { message: string; report: Report | null }) {
  return (
    <div className="flex flex-col gap-4">
      <p className="m-0 text-ink">{message}</p>
      {report && <ReportPanel report={report} now={nowSeconds()} />}
    </div>
  );
}

function Host({ current }: { current: OpenAction }) {
  const close = useOpenAction((s) => s.set);
  const { apply } = useAction();
  // The daemon describes the change fresh each time the dialog opens.
  const preview = usePreview(current.action, true);
  const [undo, setUndo] = useState<Action | null>(null);
  const plan = preview.data ?? null;
  const problem = preview.error?.message ?? null;
  const planView: ReactNode = problem ? (
    <EvidenceBlock text={problem} label="Bigman cannot make this change" />
  ) : plan ? (
    <Plan plan={plan} />
  ) : (
    <Skeleton shape="block" />
  );
  const Dialog = plan?.careful ? ConfirmDanger : PreviewDialog;
  return (
    <Dialog
      open
      onOpenChange={(open) => {
        if (!open) close(null);
      }}
      onCloseAutoFocus={(event) => {
        event.preventDefault();
        restoreFocus();
      }}
      title={plan?.title ?? current.label.replace(/…$/, "")}
      // A plan with facts describes a run, not a change to this computer.
      planHeading={plan && plan.facts.length > 0 ? "What happens" : undefined}
      plan={planView}
      ready={plan !== null && problem === null}
      confirmLabel={plan?.verb ?? "Confirm"}
      undo={
        undo
          ? {
              label: "Undo",
              run: async () => {
                const reverted = await apply(undo);
                setUndo(null);
                return <Outcome message={reverted.message} report={reverted.report} />;
              },
            }
          : undefined
      }
      onConfirm={async () => {
        const outcome = await apply(current.action);
        setUndo(outcome.undo);
        current.onDone?.(outcome.message);
        return <Outcome message={outcome.message} report={outcome.report} />;
      }}
    />
  );
}

/** Mounted once by the shell (and by tests); renders whichever action is open. */
export function ActionDialogHost() {
  const current = useOpenAction((s) => s.current);
  // Keyed by the action so each opening starts at the plan step.
  return current ? <Host key={JSON.stringify(current.action)} current={current} /> : null;
}
