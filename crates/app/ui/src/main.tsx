import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { lazy, StrictMode, Suspense } from "react";
import { createRoot } from "react-dom/client";
import "./design/tokens.css";
import { applyTheme, storedTheme } from "./design/theme";
import { DisconnectedError, setTransport, type Transport } from "./ipc";
import { App } from "./shell/App";

const Gallery = lazy(() => import("./design/gallery/Gallery"));

async function chooseTransport(): Promise<Transport> {
  if (import.meta.env.MODE === "browser") {
    const mock = await import("./mock/transport");
    return mock.mockTransport(mock.scenarioFromLocation());
  }
  return (await import("./ipc/tauri")).tauriTransport;
}

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // A stopped daemon shows the Disconnected state at once; polling (see ipc/hooks)
      // brings the view back when it answers. Other errors get two quick retries.
      retry: (count, error) => !(error instanceof DisconnectedError) && count < 2,
      retryDelay: 1_000,
      refetchOnWindowFocus: true,
    },
  },
});

setTransport(await chooseTransport());
applyTheme(storedTheme());

const root = document.getElementById("root");
if (!root) throw new Error("Missing #root");
createRoot(root).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      {window.location.hash === "#/gallery" ? (
        <Suspense>
          <Gallery />
        </Suspense>
      ) : (
        <App />
      )}
    </QueryClientProvider>
  </StrictMode>,
);
