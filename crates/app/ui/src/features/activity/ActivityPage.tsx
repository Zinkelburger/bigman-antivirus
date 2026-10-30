import { EmptyState, PageHeader } from "@/design";

export function ActivityPage() {
  return (
    <>
      <PageHeader title="Activity" />
      <div className="px-6 py-5">
        <EmptyState>Everything Bigman notices is entered here by day, first sightings on top, with every scan and inspection you run.</EmptyState>
      </div>
    </>
  );
}
