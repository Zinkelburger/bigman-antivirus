import { X } from "lucide-react";
import { create } from "zustand";

interface ToastMessage {
  id: number;
  text: string;
}

interface ToastStore {
  toasts: ToastMessage[];
  push: (text: string) => void;
  dismiss: (id: number) => void;
}

let nextId = 1;

/** Short confirmations of completed actions. Findings never appear as toasts. */
export const useToasts = create<ToastStore>((set) => ({
  toasts: [],
  push: (text) => {
    const id = nextId++;
    set((s) => ({ toasts: [...s.toasts.slice(-2), { id, text }] }));
    setTimeout(() => set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) })), 5_000);
  },
  dismiss: (id) => set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) })),
}));

export function Toast({ text, onDismiss }: { text: string; onDismiss: () => void }) {
  return (
    <div className="flex items-center gap-3 rounded-panel border border-rule bg-surface py-2 pr-2 pl-4 shadow-panel animate-[bigman-row-in_200ms_ease-out]">
      <span className="text-body text-ink">{text}</span>
      <button
        type="button"
        onClick={onDismiss}
        aria-label="Dismiss"
        className="cursor-pointer rounded-control p-1 text-ink-muted hover:bg-sunken"
      >
        <X aria-hidden size={14} />
      </button>
    </div>
  );
}

export function Toaster() {
  const { toasts, dismiss } = useToasts();
  return (
    <div aria-live="polite" className="pointer-events-none fixed right-4 bottom-12 z-50 flex flex-col gap-2">
      {toasts.map((t) => (
        <div key={t.id} className="pointer-events-auto">
          <Toast text={t.text} onDismiss={() => dismiss(t.id)} />
        </div>
      ))}
    </div>
  );
}
