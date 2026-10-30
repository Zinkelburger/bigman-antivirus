import { EmptyState, PageHeader } from "@/design";

export function SetupPage() {
  return (
    <>
      <PageHeader title="Setup" />
      <div className="px-6 py-5">
        <EmptyState>Optional tools, background services and notification settings are installed and changed here, always previewed first.</EmptyState>
      </div>
    </>
  );
}
