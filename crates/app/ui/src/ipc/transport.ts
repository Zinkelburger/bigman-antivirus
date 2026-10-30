import type { Request } from "./generated/Request";
import type { Response } from "./generated/Response";

/** How the UI reaches bigmand: the Tauri shell in the app, fixtures in browser dev mode. */
export interface Transport {
  request(request: Request): Promise<Response>;
}

/** bigmand could not be reached. Views show the Disconnected state. */
export class DisconnectedError extends Error {
  override name = "DisconnectedError";
}

/** bigmand answered with an error. The message is the daemon's, shown verbatim. */
export class DaemonError extends Error {
  override name = "DaemonError";
}

let current: Transport | null = null;

export function setTransport(transport: Transport): void {
  current = transport;
}

export function transport(): Transport {
  if (!current) throw new Error("No transport configured; call setTransport() at startup");
  return current;
}

type ValueOf<K extends Response["type"]> = Extract<Response, { type: K }> extends { value: infer V }
  ? V
  : never;

/** Sends a request and unwraps the expected response variant. */
export async function call<K extends Exclude<Response["type"], "error">>(
  request: Request,
  expect: K,
): Promise<ValueOf<K>> {
  const response = await transport().request(request);
  if (response.type === "error") throw new DaemonError(response.value.message);
  if (response.type !== expect) {
    throw new DaemonError(`Expected a ${expect} response, received ${response.type}`);
  }
  return response.value as ValueOf<K>;
}
