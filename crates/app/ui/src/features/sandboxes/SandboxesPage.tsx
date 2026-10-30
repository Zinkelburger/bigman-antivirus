import { EmptyState, PageHeader } from "@/design";

export function SandboxesPage() {
  return (
    <>
      <PageHeader title="Sandboxes" />
      <div className="px-6 py-5">
        <EmptyState>Each of your running programs appears here with its confinement: Flatpak, Snap, container, systemd unit or none.</EmptyState>
      </div>
    </>
  );
}
