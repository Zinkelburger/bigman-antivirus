# Data and development

> Part of the [Bigman README](../README.md).

State defaults to `$XDG_DATA_HOME/bigman` or `~/.local/share/bigman`. Use an absolute `--state /path/to/private-directory` for separate profiles/tests. Directories are mode 0700 and report/recovery files mode 0600. Writes use temporary files and atomic replacement. Existing broad-permission or symlinked state directories are refused. This protects against other ordinary users, not compromise of the same account or root.

The Cargo workspace has five crates. `crates/core` holds the report model, private store, bounded process runner, daemon contract (`ipc`) and one module per check under `checks/`; it has no binaries and no UI. `crates/cli` is the `bigman` command; `bigman gui` opens the desktop app. `crates/daemon` is `bigmand`, the per-user daemon that runs every check for the app: it listens on `$XDG_RUNTIME_DIR/bigman/bigmand.sock` (mode 0600, peer uid checked; `BIGMAN_SOCKET` overrides the path), and a client that disconnects cancels its running check. `bigman daemon status` queries it. `crates/app` is the desktop app. `crates/native-host` is the browser native-messaging bridge. Rules have stable IDs and local evidence: `crates/core/src/rules/mod.rs` registers every rule with its explanation, remediation and the threats it covers, [docs/rules.md](rules.md) is generated from it, and `fixtures/<rule-id>/` holds a synthetic input that must fire exactly its listed rules (`cargo test -p bigman-core --test fixtures`). There is no telemetry from Bigman; explicitly launched upstream tools may have their own network behavior.

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked
python3 tests/cli_integration.py target/debug/bigman
python3 tests/canary_onboarding.py target/debug/bigman
python3 tests/cli_golden.py target/debug/bigman
python3 tests/daemon_integration.py target/debug
cd browser-extension && npm run check
```

### Desktop app

The desktop app lives in `crates/app`: a Tauri 2 shell (`src-tauri`) and a React + TypeScript UI (`ui`). It is a thin client of `bigmand`; it never runs a check itself and starts the daemon if it is not answering. Every state change is a previewed action (`crates/core/src/actions.rs`): the daemon describes exactly what will change, you confirm, then it applies.

```sh
# Fedora build dependencies for the Tauri shell
sudo dnf install webkit2gtk4.1-devel libsoup3-devel javascriptcoregtk4.1-devel libappindicator-gtk3-devel librsvg2-devel openssl-devel

cd crates/app/ui
npm ci --ignore-scripts
npm run dev:browser          # UI in a normal browser against fixture data (?scenario=calm|incomplete|empty|disconnected)
npm run check                # typecheck, lint, architecture rules, vitest
npm run build                # bundle the UI into crates/app/ui/dist
cd ../../..
cargo build                  # bigman, bigmand
cargo run -p bigman-app      # the desktop app, using the built UI
# Hot reload: npm --prefix crates/app/ui run dev, then cargo run -p bigman-app --no-default-features
```

`http://127.0.0.1:1420/#/gallery` shows every design-system component in every state, light and dark. The TypeScript types for the daemon contract are generated from the Rust types by `cargo test` into `crates/app/ui/src/ipc/generated`; never edit them by hand. `tests/app_smoke.mjs` drives the built app through `tauri-driver` (install it with `cargo install tauri-driver`; it needs `WebKitWebDriver`). `cargo deny check` enforces the licence policy in `deny.toml`.

`tests/cli_integration.py` uses synthetic files and a scanner contract fixture. `tests/cli_golden.py` pins the exact text and JSON output of `doctor`, `inspect`, piped inspection and `packages` against `tests/golden/`; rerun it with `--update` only after an intentional output change. `tests/daemon_integration.py` drives the desktop app's workflows (scan, previewed quarantine and restore, inspection, package policy, baseline, canaries, history) against a real `bigmand` over its socket. `tests/canary_onboarding.py` exercises real monitoring, fresh-read verification, repeat setup, failure fallback and stop behavior; its service manager is a fixture so tests do not change the host’s login configuration. Rust tests exercise real inotify events and filesystem safeguards. Browser tests cover host parsing, Unicode, mismatches, exceptions, ClickFix rules and DOM mutations. `tests/browser_smoke.cjs` additionally exercises the built bundle with real Chrome click events using Playwright (set `BIGMAN_PLAYWRIGHT_MODULE` and optionally `BIGMAN_CHROME`). CI runs Rust/CLI and extension checks and builds an extension artifact.

The historical [review](../REVIEW.md) explains the original prototype and proposals. The README and these docs describe the implemented interfaces and their limits. Advanced host-wide credential-read attribution, privileged on-access blocking, incident cleanup, Windows/macOS support, provider-side token provisioning and enterprise monitoring servers are not implemented or implied by this release.
