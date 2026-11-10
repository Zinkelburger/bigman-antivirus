#!/usr/bin/env node
// WebDriver smoke test of the built desktop app: launch, visit each tab, open one
// drill-down, close it, quit. Needs tauri-driver and WebKitWebDriver on PATH and a
// graphical session (CI uses xvfb-run). Starts a real bigmand on a private socket.
//
//   node tests/app_smoke.mjs target/debug/bigman-app
import { spawn } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";

const app = resolve(process.argv[2] ?? "target/debug/bigman-app");
const TABS = ["Overview", "Network", "Traffic", "Activity", "Inspect", "Sandboxes", "Protect", "Setup"];
const DRIVER = "http://127.0.0.1:4444";
const ELEMENT = "element-6066-11e4-a52e-4f735466cecf"; // W3C WebDriver element reference key

const runtime = process.env.XDG_RUNTIME_DIR ?? tmpdir();
const work = mkdtempSync(join(runtime, "bigman-smoke-"));
const env = { ...process.env, BIGMAN_SOCKET: join(work, "run", "bigmand.sock") };
const children = [];
let session = null;
const start = (cmd, args) => {
  const child = spawn(cmd, args, { env, stdio: ["ignore", "inherit", "inherit"] });
  children.push(child);
  return child;
};
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function until(what, check, timeoutMs = 15_000) {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    try {
      const value = await check();
      if (value) return value;
    } catch {
      // Not ready yet.
    }
    if (Date.now() > deadline) throw new Error(`Timed out waiting for ${what}`);
    await sleep(200);
  }
}

async function wd(method, path, body) {
  const response = await fetch(DRIVER + path, {
    method,
    headers: { "content-type": "application/json" },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  const json = await response.json();
  if (!response.ok) throw new Error(`${method} ${path}: ${JSON.stringify(json.value)}`);
  return json.value;
}

try {
  start(join(dirname(app), "bigmand"), ["--state", join(work, "state")]);
  start("tauri-driver", []);
  await until("tauri-driver", async () => (await fetch(`${DRIVER}/status`)).ok);

  const { sessionId } = await wd("POST", "/session", {
    capabilities: { alwaysMatch: { "tauri:options": { application: app, args: ["--socket", env.BIGMAN_SOCKET] } } },
  });
  session = `/session/${sessionId}`;
  const s = session;
  const find = async (xpath) => (await wd("POST", `${s}/element`, { using: "xpath", value: xpath }))[ELEMENT];
  // WebKitWebDriver's element-text endpoint is unreliable mid-animation; read the DOM.
  const textOf = (css) =>
    wd("POST", `${s}/execute/sync`, { script: "return document.querySelector(arguments[0])?.textContent?.trim() ?? null", args: [css] });
  const click = async (id) => wd("POST", `${s}/element/${id}/click`, {});

  const tabButton = (tab) => find(`//nav//button[.//span[normalize-space()='${tab}']]`);
  try {
    await until("the app to render", () => tabButton("Overview"));
  } catch (error) {
    const page = await wd("POST", `${s}/execute/sync`, {
      script: "return location.href + '\\n' + document.documentElement.outerHTML.slice(0, 2000)",
      args: [],
    }).catch((e) => String(e));
    console.error(`Page at timeout:\n${page}`);
    throw error;
  }
  for (const tab of TABS) {
    await click(await tabButton(tab));
    await until(`${tab} page`, async () => (await textOf("main h1")) === tab);
    console.log(`visited ${tab}`);
  }

  await click(await find(`//nav//button[.//span[normalize-space()='Overview']]`));
  // The tools tile becomes a button once bigmand has answered.
  const tile = await until("daemon status", () => find("//main//button[.//span[normalize-space()=\"Bigman's tools\"]]"), 30_000);
  await click(tile);
  await until("drill-down", async () => (await textOf("aside[role=dialog] h2")) === "Bigman's tools");
  console.log("opened the Bigman's tools drill-down");
  await click(await find("//aside[@role='dialog']//button[@aria-label='Close panel']"));
  await until("drill-down to close", async () => {
    const found = await wd("POST", `${s}/elements`, { using: "xpath", value: "//aside[@role='dialog']" });
    return found.length === 0;
  });

  // Live collection reaches the webview: the Network tab lists this machine's
  // listeners (or says there are none) and the Activity tab shows its timeline.
  await click(await tabButton("Network"));
  await until("network tables", async () => (await textOf("#listeners")) === "Listening programs", 30_000);
  const hasListeners = await wd("POST", `${s}/execute/sync`, {
    script: "return !!document.querySelector('table[aria-label=\"Listening programs\"]') || /No program is listening/.test(document.body.textContent)",
    args: [],
  });
  if (!hasListeners) throw new Error("Network shows neither listeners nor the empty state");
  console.log("Network lists listening programs from the daemon");

  await wd("DELETE", s);
  session = null;
  console.log("PASS: launched, visited every tab, opened and closed a drill-down, read live network state, quit");
} finally {
  // Deleting the session closes the app window, even after a failure.
  if (session) await wd("DELETE", session).catch(() => undefined);
  for (const child of children.reverse()) child.kill("SIGINT");
  await sleep(300);
  rmSync(work, { recursive: true, force: true });
}
