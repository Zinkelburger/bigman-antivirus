import { Binoculars, Boxes, Fence, Network, NotebookTabs, ScanSearch, Wrench } from "lucide-react";
import type { ShellTab } from "@/design";
import type { TabId } from "./store";

export const TABS: readonly ShellTab<TabId>[] = [
  { id: "overview", label: "Overview", icon: NotebookTabs },
  { id: "network", label: "Network", icon: Network },
  { id: "activity", label: "Activity", icon: Binoculars },
  { id: "inspect", label: "Inspect", icon: ScanSearch },
  { id: "sandboxes", label: "Sandboxes", icon: Boxes },
  { id: "protect", label: "Protect", icon: Fence },
  { id: "setup", label: "Setup", icon: Wrench },
];
