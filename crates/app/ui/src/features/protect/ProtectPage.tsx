import { DaemonGate, PageHeader, SegmentedTabs } from "@/design";
import { AuditsView } from "./AuditsView";
import { CanariesView } from "./CanariesView";
import { HardeningView } from "./HardeningView";
import { QuarantineView } from "./QuarantineView";
import { RunningView } from "./RunningView";
import { ScansView } from "./ScansView";
import { SkillsView } from "./SkillsView";
import { StartupView } from "./StartupView";
import { UpdatesView } from "./UpdatesView";
import { useProtectView, type ProtectView } from "./store";

const VIEWS: readonly { id: ProtectView; label: string }[] = [
  { id: "hardening", label: "Hardening" },
  { id: "scans", label: "Scans" },
  { id: "updates", label: "Updates" },
  { id: "quarantine", label: "Quarantine" },
  { id: "startup", label: "Startup files" },
  { id: "running", label: "Running" },
  { id: "skills", label: "AI skills" },
  { id: "canaries", label: "Canaries" },
  { id: "audits", label: "Audits" },
];

const BODY: Record<ProtectView, () => React.JSX.Element> = {
  hardening: HardeningView,
  scans: ScansView,
  updates: UpdatesView,
  quarantine: QuarantineView,
  startup: StartupView,
  running: RunningView,
  skills: SkillsView,
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
