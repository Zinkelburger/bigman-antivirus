import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import type { ReactNode } from "react";
import { ActionDialogHost, closeActionDialog, Toaster, useDrilldown, useToasts } from "@/design";
import { Drilldown } from "@/design/components/Drilldown";
import { setDesktopShell, setTransport } from "@/ipc";
import { mockShell, mockTransport } from "@/mock/transport";

/** Renders a page against a fresh in-memory daemon with no simulated delays. */
export function renderWithDaemon(node: ReactNode, scenario = "findings") {
  setTransport(mockTransport(scenario, 0, 0));
  setDesktopShell(mockShell);
  useDrilldown.setState({ stack: [], opener: null });
  closeActionDialog();
  useToasts.setState({ toasts: [] });
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
  return {
    user: userEvent.setup(),
    ...render(
      <QueryClientProvider client={client}>
        {node}
        <Drilldown />
        <ActionDialogHost />
        <Toaster />
      </QueryClientProvider>,
    ),
  };
}
