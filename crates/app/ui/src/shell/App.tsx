import { AppShell, Toaster } from "@/design";
import { ActivityPage } from "@/features/activity/ActivityPage";
import { InspectPage } from "@/features/inspect/InspectPage";
import { NetworkPage } from "@/features/network/NetworkPage";
import { OverviewPage } from "@/features/overview/OverviewPage";
import { ProtectPage } from "@/features/protect/ProtectPage";
import { SandboxesPage } from "@/features/sandboxes/SandboxesPage";
import { SetupPage } from "@/features/setup/SetupPage";
import { useGlobalKeys } from "./keyboard";
import { StatusStrip } from "./StatusStrip";
import { useShell, type TabId } from "./store";
import { TABS } from "./tabs";

const PAGES: Record<TabId, () => React.JSX.Element> = {
  overview: OverviewPage,
  network: NetworkPage,
  activity: ActivityPage,
  inspect: InspectPage,
  sandboxes: SandboxesPage,
  protect: ProtectPage,
  setup: SetupPage,
};

export function App() {
  const { tab, setTab } = useShell();
  useGlobalKeys();
  const Page = PAGES[tab];
  return (
    <>
      <AppShell tabs={TABS} active={tab} onSelect={setTab} statusStrip={<StatusStrip />}>
        <Page />
      </AppShell>
      <Toaster />
    </>
  );
}
