import { EmptyState, PageHeader } from "@/design";

export function SandboxesPage() {
  return (
    <>
      <PageHeader title="Sandboxes" />
      <div className="px-6 py-5">
        <EmptyState>
          Not built yet. This tab will list each of your running programs with its confinement (Flatpak, Snap,
          container, systemd unit or none), and what SELinux or AppArmor stopped it from doing. Until then, denials
          appear in Activity.
        </EmptyState>
      </div>
    </>
  );
}
