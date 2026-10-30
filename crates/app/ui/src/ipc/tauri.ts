import { invoke } from "@tauri-apps/api/core";
import type { Response } from "./generated/Response";
import { DaemonError, DisconnectedError, type Transport } from "./transport";

/** The shell's `RequestError` (src-tauri/src/daemon.rs). */
interface ShellError {
  kind: "disconnected" | "failed";
  message: string;
}

function isShellError(value: unknown): value is ShellError {
  return typeof value === "object" && value !== null && "kind" in value && "message" in value;
}

/** The desktop shell forwards each request to bigmand over its private socket. */
export const tauriTransport: Transport = {
  async request(request) {
    try {
      return await invoke<Response>("daemon_request", { request });
    } catch (error) {
      if (isShellError(error) && error.kind === "failed") throw new DaemonError(error.message);
      throw new DisconnectedError(isShellError(error) ? error.message : String(error));
    }
  },
};
