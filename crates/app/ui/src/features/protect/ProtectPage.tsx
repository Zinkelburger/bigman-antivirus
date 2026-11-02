import { DaemonGate, PageHeader, SegmentedTabs } from "@/design";
import { AuditsView } from "./AuditsView";
import { CanariesView } from "./CanariesView";
import { QuarantineView } from "./QuarantineView";
import { ScansView } from "./ScansView";
import { StartupView } from "./StartupView";
import { useProtectView, type ProtectView } from "./store";

const VIEWS: readonly { id: ProtectView; label: string }[] = [
  { id: "scans", label: "Scans" },
  { id: "quarantine", label: "Quarantine" },
  { id: "startup", label: "Startup files" },
  { id: "canaries", label: "Canaries" },
  { id: "audits", label: "Audits" },
];

const BODY: Record<ProtectView, () => React.JSX.Element> = {
  scans: ScansView,
  quarantine: QuarantineView,
  startup: StartupView,
  canaries: CanariesView,
  audits: AuditsView,
};

export function ProtectPage() {
  const { view, setView } = useProtectView();
  const Body = BODY[view];
  return (
    <>
      <PageHeader title="Protect" />
      <SegmentedTabs label="Protect" tabs={VIEWS} active={view} onSelect={setView} />
      <div role="tabpanel" aria-label={VIEWS.find((v) => v.id === view)?.label} className="px-6 py-5">
        <DaemonGate>
          <Body />
        </DaemonGate>
      </div>
    </>
  );
}
