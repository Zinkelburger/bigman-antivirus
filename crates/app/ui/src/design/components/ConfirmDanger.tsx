import { PreviewDialog, type PreviewDialogProps } from "./PreviewDialog";

/**
 * For changes that run code the user has not reviewed (the installer rehearsal).
 * Same plan/confirm/result flow, plus an explicit acknowledgment.
 */
export function ConfirmDanger(props: Omit<PreviewDialogProps, "acknowledgment" | "undo">) {
  return (
    <PreviewDialog
      {...props}
      acknowledgment="I have read what will change and want to continue."
    />
  );
}
