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
  set: (current: OpenAction | null) => void;
}

const useOpenAction = create<ActionStore>((set, get) => ({
  current: null,
  opener: null,
  set: (current) => {
    if (current) {
      set({ current, opener: document.activeElement instanceof HTMLElement ? document.activeElement : null });
      return;
    }
    const { opener } = get();
    set({ current: null, opener: null });
    // The opener may have gone with the change (a restored file's button); fall back to the page heading.
    requestAnimationFrame(() => (opener?.isConnected ? opener : document.querySelector<HTMLElement>("main h1"))?.focus());
  },
}));

/** Closes the open action dialog, if any. */
export function closeActionDialog(): void {
  useOpenAction.getState().set(null);
}

function Plan({ plan }: { plan: ActionPlan }) {
  return (
    <>
      <p className="m-0 max-w-[56ch] text-ink">{plan.summary}</p>
      {plan.details && <EvidenceBlock text={plan.details} label="Exactly what changes" />}
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
  /** Called after the change succeeds, for views that react to it. The dialog itself shows the result. */
  onDone?: ((message: string) => void) | undefined;
}

/** A button that opens the previewed change in the shared dialog. */
export function ActionDialog({ action, label, variant = "secondary", disabled = false, onDone }: ActionDialogProps) {
  const open = useOpenAction((s) => s.set);
  return (
    <ActionButton variant={variant} disabled={disabled} onClick={() => open({ action, label, onDone })}>
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
      title={plan?.title ?? current.label.replace(/…$/, "")}
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
