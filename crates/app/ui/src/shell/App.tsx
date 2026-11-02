import { AppShell, Toaster, TransactionPreviewHost } from "@/design";
import { useLive, useUnread } from "@/ipc";
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
  useLive();
  const unread = useUnread().data?.first_seen ?? 0;
  const tabs = TABS.map((t) => (t.id === "activity" && unread > 0 ? { ...t, badge: unread } : t));
  const Page = PAGES[tab];
  return (
    <>
      <AppShell tabs={tabs} active={tab} onSelect={setTab} statusStrip={<StatusStrip />}>
        <Page />
      </AppShell>
      <TransactionPreviewHost />
      <Toaster />
    </>
  );
}
