import { EmptyState, PageHeader } from "@/design";

export function NetworkPage() {
  return (
    <>
      <PageHeader title="Network" />
      <div className="px-6 py-5">
        <EmptyState>Live throughput per interface, the connections your programs make and new listening sockets appear here.</EmptyState>
      </div>
    </>
  );
}
