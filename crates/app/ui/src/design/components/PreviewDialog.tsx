import * as Dialog from "@radix-ui/react-dialog";
import { X } from "lucide-react";
import { useState, type ReactNode } from "react";
import { ActionButton } from "./ActionButton";
import { EvidenceBlock } from "./EvidenceBlock";

export interface PreviewDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  /** Exactly what will change, shown before anything runs. */
  plan: ReactNode;
  confirmLabel: string;
  /** Runs the change and returns what happened, for the result step. */
  onConfirm: () => Promise<ReactNode>;
  /** Present when the subsystem can revert the change. */
  undo?: { label: string; run: () => Promise<ReactNode> } | undefined;
  /** ConfirmDanger requires an acknowledgment before confirming. */
  acknowledgment?: string | undefined;
  /** False until the plan has loaded; confirming is not possible before. */
  ready?: boolean;
  /** Heading above the plan; "What will change" by default. */
  planHeading?: string | undefined;
  /** The dismiss button's label; "Cancel" by default. */
  cancelLabel?: string | undefined;
  /** A second answer beside the confirm button, for two-way decisions. */
  alternative?: { label: string; run: () => Promise<ReactNode> } | undefined;
  /** Shown at the left of the footer while deciding, e.g. a countdown. */
  footer?: ReactNode;
  /** What a screen reader hears on open; "Review what will change before confirming." by default. */
  description?: string | undefined;
  /** For decisions nobody should lose to a stray click: the backdrop does not close it. */
  modalOnly?: boolean;
}

type Phase = { step: "plan" } | { step: "running" } | { step: "result"; result: ReactNode } | { step: "error"; message: string };

/** Every state-changing action: plan, confirm, result. Modal because focus must stay on the decision. */
export function PreviewDialog({
  open,
  onOpenChange,
  title,
  plan,
  confirmLabel,
  onConfirm,
  undo,
  acknowledgment,
  ready = true,
  planHeading = "What will change",
  cancelLabel = "Cancel",
  alternative,
  footer,
  description = "Review what will change before confirming.",
  modalOnly = false,
}: PreviewDialogProps) {
  const [phase, setPhase] = useState<Phase>({ step: "plan" });
  const [acknowledged, setAcknowledged] = useState(false);
  const run = async (action: () => Promise<ReactNode>) => {
    setPhase({ step: "running" });
    try {
      setPhase({ step: "result", result: await action() });
    } catch (error) {
      setPhase({ step: "error", message: error instanceof Error ? error.message : String(error) });
    }
  };
  const change = (next: boolean) => {
    if (phase.step === "running") return;
    if (!next) {
      setPhase({ step: "plan" });
      setAcknowledged(false);
    }
    onOpenChange(next);
  };
  return (
    <Dialog.Root open={open} onOpenChange={change}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 z-40 bg-scrim" />
        <Dialog.Content
          onPointerDownOutside={modalOnly ? (e) => e.preventDefault() : undefined}
          onOpenAutoFocus={(e) => {
            // Start at the title, so Enter cannot answer before the plan is read.
            e.preventDefault();
            (e.currentTarget as HTMLElement).querySelector<HTMLElement>("[data-dialog-title]")?.focus();
          }}
          className="fixed top-1/2 left-1/2 z-50 flex max-h-[85vh] w-[min(640px,calc(100vw-48px))] -translate-x-1/2 -translate-y-1/2 flex-col rounded-panel border border-rule bg-surface shadow-panel"
        >
          <div className="flex items-center justify-between gap-4 border-b border-rule px-5 py-3">
            <Dialog.Title data-dialog-title tabIndex={-1} className="text-title font-semibold text-ink outline-none">
              {title}
            </Dialog.Title>
            <Dialog.Close
              aria-label="Close dialog"
              disabled={phase.step === "running"}
              className="cursor-pointer rounded-control p-1 text-ink-muted hover:bg-sunken"
            >
              <X aria-hidden size={16} />
            </Dialog.Close>
          </div>
          <Dialog.Description className="sr-only">{description}</Dialog.Description>
          <div className="flex flex-col gap-4 overflow-y-auto px-5 py-4 text-body">
            {phase.step === "result" ? (
              <>
                <h3 className="font-semibold text-ink">Done</h3>
                {phase.result}
              </>
            ) : phase.step === "error" ? (
              <>
                <h3 className="font-semibold text-ink">This change did not complete</h3>
                <EvidenceBlock text={phase.message} label="Message from the system" />
              </>
            ) : (
              <>
                <h3 className="font-semibold text-ink">{planHeading}</h3>
                {plan}
                {acknowledgment && (
                  <label className="flex items-start gap-2 text-body text-ink">
                    <input
                      type="checkbox"
                      checked={acknowledged}
                      onChange={(e) => setAcknowledged(e.target.checked)}
                      className="mt-1 size-4 accent-[var(--accent)]"
                    />
                    {acknowledgment}
                  </label>
                )}
              </>
            )}
          </div>
          <div className="flex items-center justify-end gap-2 border-t border-rule px-5 py-3">
            {footer && phase.step !== "result" && phase.step !== "error" && (
              <div className="mr-auto min-w-0 text-small text-ink-muted">{footer}</div>
            )}
            {phase.step === "result" || phase.step === "error" ? (
              <>
                {phase.step === "result" && undo && (
                  <ActionButton onClick={() => void run(undo.run)}>{undo.label}</ActionButton>
                )}
                <ActionButton variant="primary" onClick={() => change(false)}>
                  Close
                </ActionButton>
              </>
            ) : (
              <>
                <ActionButton variant="quiet" disabled={phase.step === "running"} onClick={() => change(false)}>
                  {cancelLabel}
                </ActionButton>
                {alternative && (
                  <ActionButton disabled={phase.step === "running" || !ready} onClick={() => void run(alternative.run)}>
                    {alternative.label}
                  </ActionButton>
                )}
                <ActionButton
                  variant="primary"
                  busy={phase.step === "running"}
                  disabled={!ready || (acknowledgment !== undefined && !acknowledged)}
                  onClick={() => void run(onConfirm)}
                >
                  {confirmLabel}
                </ActionButton>
              </>
            )}
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
