import { EmptyState, PageHeader } from "@/design";

export function ProtectPage() {
  return (
    <>
      <PageHeader title="Protect" />
      <div className="px-6 py-5">
        <EmptyState>Malware scans, quarantine, the startup baseline, canaries and advisories live here.</EmptyState>
      </div>
    </>
  );
}
