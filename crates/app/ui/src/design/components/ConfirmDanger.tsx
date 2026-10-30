import { PreviewDialog, type PreviewDialogProps } from "./PreviewDialog";

/**
 * For changes the subsystem cannot undo (removing a file into quarantine, removing
 * a service). Same plan/confirm/result flow, plus an explicit acknowledgment.
 */
export function ConfirmDanger(props: Omit<PreviewDialogProps, "acknowledgment" | "undo">) {
  return (
    <PreviewDialog
      {...props}
      acknowledgment="I have read what will change and want to continue."
    />
  );
}
