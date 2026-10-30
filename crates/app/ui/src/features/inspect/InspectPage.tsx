import { EmptyState, PageHeader } from "@/design";

export function InspectPage() {
  return (
    <>
      <PageHeader title="Inspect" />
      <div className="px-6 py-5">
        <EmptyState>Installers, packages, PDFs and repositories you inspect before running them are read here, without executing anything.</EmptyState>
      </div>
    </>
  );
}
