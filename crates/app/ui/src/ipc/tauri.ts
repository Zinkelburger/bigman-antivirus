import { Channel, invoke } from "@tauri-apps/api/core";
import type { Live } from "./generated/Live";
import type { Response } from "./generated/Response";
import type { DesktopShell } from "./shell";
import { DaemonError, DisconnectedError, type Transport } from "./transport";

/** The shell's `RequestError` (src-tauri/src/daemon.rs). */
interface ShellError {
  kind: "disconnected" | "failed";
  message: string;
}

function isShellError(value: unknown): value is ShellError {
  return typeof value === "object" && value !== null && "kind" in value && "message" in value;
}

function rethrow(error: unknown): never {
  if (isShellError(error) && error.kind === "failed") throw new DaemonError(error.message);
  throw new DisconnectedError(isShellError(error) ? error.message : String(error));
}

/** The desktop shell forwards each request to bigmand over its private socket. */
export const tauriTransport: Transport = {
  async request(request, token) {
    try {
      return await invoke<Response>("daemon_request", { request, token: token ?? null });
    } catch (error) {
      rethrow(error);
    }
  },
  async cancel(token) {
    await invoke("daemon_cancel", { token });
  },
  subscribe({ events, samples }, onPush) {
    const token = crypto.randomUUID();
    const channel = new Channel<Live>(onPush);
    const ended = invoke<null>("daemon_subscribe", { channel, events, samples, token }).then(
      () => undefined,
      rethrow,
    );
    return { ended, stop: () => void invoke("daemon_cancel", { token }) };
  },
};

export const tauriShell: DesktopShell = {
  choosePath: (kind, title) => invoke<string | null>("choose_path", { kind, title }),
  async exportReport(id) {
    try {
      return await invoke<string | null>("export_report", { id });
    } catch (error) {
      rethrow(error);
    }
  },
};
