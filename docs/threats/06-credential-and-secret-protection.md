# Credential and secret protection on Linux developer and maintainer workstations

> Scope: the secrets that live in a Linux developer's or open-source maintainer's home directory and session. That covers plaintext token files for git, package registries, clouds and clusters; `.env` files and shell history; SSH and GPG private keys and `ssh-agent`; browser password and cookie stores; crypto wallets; desktop keyrings; and secrets committed to repositories or published artifacts. It also covers the mitigations that reduce what a thief can take: keyring-backed helpers, hardware keys, short-lived and OIDC tokens, secret scanning, and canaries/honeytokens. Everything here is written from the defender's side. It covers what to inventory, what a risky state looks like, and what Bigman could check deterministically without reading or exporting secret values. How malicious packages get onto the machine is covered in [supply-chain](./02-supply-chain-packages.md), and persistence and permission auditing in general in [host integrity](./04-host-integrity-and-hardening-audit.md). Last researched: 2026-10-29.

**The main point for Bigman.** Every 2024-2026 credential-harvesting campaign below ran as the user's own Unix account, from a package install hook, a VS Code extension, a CI step or an AI CLI. Against code running as the same user, file permissions and an unlocked desktop keyring give almost no protection. So the most useful defences are these four. Fewer plaintext long-lived secrets on disk (inventory plus migration guidance). Secrets that are short-lived or bound to hardware. Secret files that cannot be read or used silently (passphrases, FIDO keys, canaries). Fast rotation once a harvest is suspected. Bigman can drive all four without root and without ever reading a secret value into a report.

## Summary table

| ID | Attack | Severity for a Linux desktop/dev user | Bigman today | Proposed |
|---|---|---|---|---|
| CR-01 | Git plaintext credential store and `~/.netrc` | High | Partial (`git.plaintext-credentials`, global `credential.helper=store` only) | Check every config scope and leftover `~/.git-credentials`; guide the move to libsecret/GCM |
| CR-02 | Package-registry publish tokens in dotfiles (`~/.npmrc`, `~/.pypirc`, `~/.cargo/credentials.toml`, `~/.gem/credentials`, `~/.yarnrc.yml`) | Critical for maintainers | Gap (only `shell.credentials`/`trace.sensitive-path` mention `.npmrc`) | `cred.plaintext-store` inventory by key name; push trusted publishing, `cargo:libsecret`, twine keyring |
| CR-03 | Cloud and cluster credentials (`~/.aws`, `~/.config/gcloud`, `~/.azure`, `~/.kube/config`, `~/.vault-token`, Terraform) | Critical | Gap (`.aws/` only in static/trace rules) | Inventory static vs short-lived; recommend SSO/exec plugins/aws-vault |
| CR-04 | Container-registry and forge CLI tokens (`~/.docker/config.json` `auths`, `~/.config/gh/hosts.yml`) | High | Gap | Detect inline `auth` fields vs `credsStore`; detect gh plaintext fallback |
| CR-05 | `.env` files and project-local secrets (including AI/MCP tool configs) | High | Gap | Bounded `.env` discovery with `git ls-files`/`check-ignore` status; redacted pattern scan |
| CR-06 | Tokens in shell and REPL history | Medium | Gap | Redacted token-pattern scan of history files; `HISTCONTROL` guidance |
| CR-07 | Secrets in environment variables and process memory (`/proc/<pid>/environ`, `/proc/<pid>/mem`) | High | Partial (ptrace_scope) | Check `ptrace_scope` and token-like env var names (names only) in shell rc files |
| CR-08 | Passphrase-less SSH private keys | High | Gap | `ssh.key-unencrypted` from header/cipher-name parse only |
| CR-09 | `ssh-agent` forwarding and agent-socket abuse | High (maintainers with server access) | Gap (`~/.ssh/config` not in the startup baseline) | `ssh.agent-forwarding` config check; add `~/.ssh/config` to baseline; recommend `ProxyJump`, `ssh-add -c/-t/-h` |
| CR-10 | Weak permissions and ownership on credential paths | Medium (High on shared or multi-user hosts) | Partial (home folder) | `cred.permissions` for a fixed path list (mode, owner, symlink) |
| CR-11 | Browser saved passwords and cookie databases (infostealers) | High | Gap (extension does phishing only) | Detect `basic`/v10 password store use, no Firefox primary password; profile inventory |
| CR-12 | Session-token theft and MFA bypass (stolen cookies, npm/GitHub sessions) | High | Gap | Guidance and canaries; extension cannot protect cookies; feed-driven advisories |
| CR-13 | Crypto wallet files and browser wallet extensions | High (for holders) | Gap | `wallet.present` inventory (existence/mode only); wallet-extension inventory; decoy wallet canary |
| CR-14 | Secrets committed to git (history, forks, deleted commits) | High for maintainers | Gap | Optional `bigman audit gitleaks DIR` integration (redacted, no network verification) |
| CR-15 | Secrets in published artifacts and logs (npm tarballs, sdists, images, CI logs) | High for maintainers | Partial (`bigman packages` reads `.tgz` without extracting, no secret rules) | Run the redacted pattern set over package tarballs before publishing |
| CR-16 | GPG signing keys without passphrase or hardware backing | Medium | Gap | `gpg.key-unprotected` from keyformat token (`private-key` / `protected-private-key` / `shadowed-private-key`) |
| CR-17 | Desktop keyring exposure and silent plaintext fallbacks | Medium | Gap | Report keyring provider availability; detect tools that fell back to plaintext |
| CR-18 | Supply-chain payloads that harvest developer credentials (Shai-Hulud, TeamPCP, TrapDoor, sckit) | Critical | Partial (installer/package rules, sandbox `trace.sensitive-path`, canary) | Extend sensitive-path lists; feed-driven IOC checks; post-incident rotation checklist |
| CR-19 | AI coding agents used to enumerate secrets (s1ngularity, poisoned `CLAUDE.md`/`.cursorrules`) | High | Gap | Inventory AI CLI configs holding tokens; flag permission-skipping agent settings; canaries catch agent reads |
| CR-20 | Long-lived, never-rotated tokens (stale leak reuse) | High | Gap | Report file mtime/age of plaintext stores; guidance to short-lived/OIDC/fine-grained tokens |
| CR-21 | Password-manager CLI sessions and vault exports (`BW_SESSION`, exported vault files) | Medium | Gap | Detect session variables persisted in rc files and plaintext vault exports by filename |
| CR-22 | No tripwires: credential theft goes unnoticed | High (detection gap) | Partial (`bigman canary arm`: `.env` decoy and hosted URL) | More decoy types (SSH key, kubeconfig, `.git-credentials`, wallet, `.npmrc`); optional `IN_OPEN` |

## Attacks

### CR-01 Git plaintext credential store and `~/.netrc`
- **What it is:** With `credential.helper=store`, Git saves HTTPS credentials (usually a GitHub/GitLab personal access token) unencrypted in `~/.git-credentials` or `$XDG_CONFIG_HOME/git/credentials`, one `https://user:token@host` line per remote. The upstream docs say the file is "protected only by filesystem permissions". `~/.netrc` is an older plaintext store that git (via curl), `ftp` and some CLIs also read. Any process running as the user can read both files, and the token often has `repo` or `workflow` scope, enough to push to every project the maintainer owns.
- **Real-world examples:** The s1ngularity/Nx compromise (Aug 2025) and the Shai-Hulud worms (Sep and Nov 2025) collected GitHub tokens from developer machines and used them to create exfiltration repositories or flip private repos to public. Wiz reported that about 90% of the 1,000+ leaked GitHub tokens were still valid days later (https://www.wiz.io/blog/s1ngularitys-aftermath). sckit (Sep 2026) and TrapDoor (May 2026) also target GitHub tokens.
- **What a defender inspects (indicators):** `git config --show-origin --get-all credential.helper` (all scopes, shows which file sets `store`); whether `~/.git-credentials` or `~/.config/git/credentials` exists even after the helper was changed (stale files are common); `stat -c '%a %U %n' ~/.git-credentials ~/.netrc`; count of lines without printing them: `wc -l ~/.git-credentials`. Suspicious state: a store file present, readable by group/other, or older than the token rotation policy.
- **Bigman today:** Partial. `bigman doctor` emits `git.plaintext-credentials` when the *global* `credential.helper` is `store`. It does not check `--system` or repo-local config, does not follow `include`/`includeIf` (Git's `--global` lookup ignores includes unless `--includes` is passed), does not detect a leftover `~/.git-credentials` file, and does not check `~/.netrc`.
- **Proposed mitigation / detection:** CLI check `git.plaintext-credentials` v2: run `git config --show-origin --includes --get-all credential.helper` (global and system), stat both store files and `~/.netrc`, and report existence, mode, owner, mtime and line count. Never read values. Guidance: switch to `git-credential-libsecret` (Fedora ships it as a package; Debian/Ubuntu have historically shipped only sources under `/usr/share/doc/git/contrib/credential/libsecret`, unverified for current releases) or Git Credential Manager with `credential.credentialStore=secretservice` (or `gpg`/`pass`, or `cache` for in-memory only), then revoke the old token and delete the file. For GitHub, prefer SSH with a FIDO key (CR-08) or `gh auth` with keyring storage.
- **Test case idea:** Temp `HOME` with `.gitconfig` containing `[credential] helper = store` inside an `includeIf` file, plus a `.git-credentials` containing `https://user:not-a-token@example.invalid` at mode 0644. Expect `git.plaintext-credentials` and `cred.permissions`, and assert the report contains no substring of the line.
- **References:**
  - https://git-scm.com/docs/git-credential-store
  - https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md
  - https://www.wiz.io/blog/s1ngularitys-aftermath
  - MITRE ATT&CK T1552.001 Credentials In Files: https://attack.mitre.org/techniques/T1552/001/

### CR-02 Package-registry publish tokens in dotfiles
- **What it is:** Maintainers keep registry credentials in plaintext: `//registry.npmjs.org/:_authToken=` in `~/.npmrc`, `npmAuthToken` in `~/.yarnrc.yml`, `password = pypi-...` in `~/.pypirc`, `~/.cargo/credentials.toml` (the default `cargo:token` provider is plaintext), and `~/.gem/credentials`. A stolen publish token is what lets a worm spread: the payload republishes every package the victim can publish, with itself added.
- **Real-world examples:** Shai-Hulud (Sep 2025) and Sha1-Hulud 2.0 (Nov 2025) used npm tokens to self-propagate across hundreds of packages (https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack; Unit 42 timeline https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/). CanisterSprawl (Apr 2026) searched for npm publish tokens and spread to PyPI when it found a PyPI token (https://blog.gitguardian.com/three-supply-chain-campaigns-hit-npm-pypi-and-docker-hub-in-48-hours/). sckit (Sep 2026) explicitly targets `.npmrc`, `NPM_TOKEN` and `PYPI_API_TOKEN` (https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html). In response, npm permanently revoked classic tokens on 2025-12-09 and moved local use to short-lived session tokens (https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/).
- **What a defender inspects (indicators):** Presence of these files and of auth *keys* without printing values, e.g. `grep -c '_authToken' ~/.npmrc`, `grep -c -E '^\s*password\s*=' ~/.pypirc`, `test -f ~/.cargo/credentials.toml`; project-level `.npmrc` files containing `_authToken` (these get committed); `cargo` `global-credential-providers` in `~/.cargo/config.toml`. Suspicious state: a write-scoped token at rest on a laptop when the project could use trusted publishing.
- **Bigman today:** Gap. `.npmrc` appears only in the installer rule `shell.credentials` and the sandbox list `trace.sensitive-path`. `bigman package-policy` writes `ignore-scripts=true` to a project `.npmrc` but does not inspect the user's.
- **Proposed mitigation / detection:** Add a `cred.plaintext-store` inventory with a fixed table of (path, key-name regex, kind). It reports only "token-like key present: yes/no", line number and mtime. Guidance per tool: npm and PyPI trusted publishing (OIDC) from CI instead of local publish tokens; `cargo:libsecret` provider; twine `keyring` support; delete `.pypirc` passwords. Flag project `.npmrc` with `_authToken` that is tracked by git as High.
- **Test case idea:** Fixture `.npmrc` with `//registry.example.invalid/:_authToken=npm_` followed by 36 `x` characters, a `.pypirc` with `password = pypi-FAKE`, and `credentials.toml` with `token = "fake"`. Assert one finding per file, values absent from JSON output.
- **References:**
  - https://docs.npmjs.com/about-access-tokens
  - https://docs.npmjs.com/trusted-publishers
  - https://docs.pypi.org/trusted-publishers/
  - https://twine.readthedocs.io/en/stable/
  - https://doc.rust-lang.org/cargo/reference/registry-authentication.html
  - https://github.blog/changelog/2025-09-29-strengthening-npm-security-important-changes-to-authentication-and-token-management/
  - T1552.001

### CR-03 Cloud and cluster credentials
- **What it is:** `~/.aws/credentials` holds static access keys (`AKIA...`). `~/.config/gcloud/` holds `credentials.db`, `access_tokens.db` and `application_default_credentials.json` (a refresh token). `~/.azure/` holds MSAL token caches. `~/.kube/config` may embed client certificates, keys or bearer tokens. `~/.vault-token` and `~/.terraform.d/credentials.tfrc.json` are plaintext. All of these let an attacker work from their own machine, and leaked static cloud keys are typically abused within minutes.
- **Real-world examples:** The KICS/TeamPCP payload (Apr 2026) collected AWS, Azure and GCP credentials plus npm config and SSH keys (GitGuardian, above). The TanStack/"Mini Shai-Hulud" wave (May 2026) harvested AWS, GCP, Azure, Kubernetes service-account and Vault tokens (https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/). sckit names `.vault-token` and `credentials.db` (THN, above). Snowflake/UNC5537 (2024) shows how long reuse of stolen credentials can continue: Mandiant traced some to infostealer infections dating back to 2020 (https://cloud.google.com/blog/topics/threat-intelligence/unc5537-snowflake-data-theft-extortion).
- **What a defender inspects (indicators):** `ls -la ~/.aws ~/.config/gcloud ~/.azure ~/.kube`; `grep -c aws_secret_access_key ~/.aws/credentials` (count only); whether `~/.aws/config` uses `sso_session` (short-lived) vs static keys; whether kubeconfig users use `exec:` plugins (OIDC) vs `client-key-data`/`token:` (check key names only, e.g. `grep -c -E 'client-key-data|^\s*token:' ~/.kube/config`); file ages.
- **Bigman today:** Gap. `.aws/` is only a string in `shell.credentials` and `trace.sensitive-path`; gcloud, Azure, kube, Vault and Terraform paths are absent from both lists.
- **Proposed mitigation / detection:** Extend `cred.plaintext-store` with these paths, classifying each as "static long-lived", "refresh token" or "short-lived cache" by key name. Guidance: AWS IAM Identity Center (`aws sso login`) or `aws-vault` (keyring-backed); gcloud with short sessions or workload identity; kubeconfig via `kubelogin`/OIDC exec plugins; Vault token helper backed by a keyring. Add these paths to `trace.sensitive-path` and `shell.credentials`.
- **Test case idea:** Temp `HOME/.aws/credentials` with AWS's documented example key `AKIAIOSFODNN7EXAMPLE`; kubeconfig fixture with a `client-key-data: ZmFrZQ==` field; assert classification and redaction.
- **References:**
  - https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html
  - https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sso.html
  - https://cloud.google.com/docs/authentication/application-default-credentials
  - https://learn.microsoft.com/en-us/cli/azure/msal-based-azure-cli
  - https://kubernetes.io/docs/reference/access-authn-authz/authentication/
  - https://github.com/int128/kubelogin
  - https://github.com/99designs/aws-vault
  - https://developer.hashicorp.com/vault/docs/commands/token-helper
  - T1552.001; T1528 Steal Application Access Token: https://attack.mitre.org/techniques/T1528/

### CR-04 Container-registry and forge CLI tokens
- **What it is:** Without a credential helper, `docker login` writes `user:password` base64-encoded (not encrypted) into `~/.docker/config.json` under `auths.<registry>.auth`. Podman/Buildah use `${XDG_RUNTIME_DIR}/containers/auth.json` and sometimes `~/.config/containers/auth.json`. The GitHub CLI stores its OAuth token in the system keyring, but silently falls back to plaintext (`~/.config/gh/hosts.yml`) when no keyring is reachable, or when the user passed `--insecure-storage`.
- **Real-world examples:** s1ngularity used an authenticated `gh` CLI to create `s1ngularity-repository*` repos for exfiltration, and advisories told victims to revoke the GitHub CLI OAuth app because that is the only way to invalidate gh tokens (https://github.com/nrwl/nx/security/advisories/GHSA-cxm3-wv7p-598c). GitGuardian found 100,000 valid secrets in 15 million public Docker images (https://www.gitguardian.com/state-of-secrets-sprawl-report-2025).
- **What a defender inspects (indicators):** `jq 'keys, (.auths // {} | map_values(has("auth")))' ~/.docker/config.json` (prints only whether inline auth exists per registry), presence of `credsStore`/`credHelpers`; `gh auth status` (reports where the token is stored); `grep -c oauth_token ~/.config/gh/hosts.yml`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** `cred.plaintext-store` entries for these JSON/YAML files, parsed with serde and reporting only key presence. Guidance: `docker-credential-secretservice` or `docker-credential-pass` (Docker looks for `pass` first on Linux, then falls back to `secretservice`); re-run `gh auth login` without `--insecure-storage` once a keyring is available.
- **Test case idea:** Fixture `config.json` with `{"auths":{"registry.example.invalid":{"auth":"ZmFrZTpmYWtl"}}}` → finding; second fixture with `"credsStore":"secretservice"` and empty auths → no finding.
- **References:**
  - https://docs.docker.com/reference/cli/docker/login/
  - https://github.com/docker/docker-credential-helpers
  - https://cli.github.com/manual/gh_auth_login
  - https://github.com/nrwl/nx/security/advisories/GHSA-cxm3-wv7p-598c
  - T1552.001

### CR-05 `.env` files and project-local secrets
- **What it is:** Twelve-factor style `.env`, `.env.local` and `.env.production` files hold API keys and database URLs next to the code. They get committed by accident, copied into container images, and read by every dependency's install script and by AI coding tools. MCP/AI tool configs (e.g. `mcp.json`-style files) increasingly hold API keys in plaintext too.
- **Real-world examples:** The s1ngularity prompt told AI CLIs to enumerate `.env` files along with wallets and SSH keys (https://socket.dev/blog/nx-packages-compromised). GitGuardian's 2026 report counted 24,008 unique secrets in MCP-related config files on public GitHub, 2,117 verified valid, and an 81% rise in AI-service credential leaks (https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html).
- **What a defender inspects (indicators):** Within project roots, bounded `find ~/src ~/Projects -maxdepth 4 -name '.env*' -type f`; for each, `git -C <repo> ls-files --error-unmatch <file>` (tracked = bad) and `git check-ignore -q <file>` (not ignored = risky); mode and owner.
- **Bigman today:** Gap. Bigman's own canary *imitates* an `.env` file (`~/Documents/backup-access.env`), but nothing inventories real ones.
- **Proposed mitigation / detection:** `secret.env-file` check with user-chosen project roots (default `~/Projects`, `~/src`, `~/code` if present), depth and count limits like `bigman scan`. Severity: High if tracked by git, Review if untracked and not ignored, info otherwise. Optionally run the redacted pattern scanner (CR-06) to say which token *types* are present. Guidance: use a secret manager or keyring-backed injection (`pass`, `secret-tool`, 1Password/Bitwarden CLI, Infisical, `sops`), and add `.env*` to the global gitignore.
- **Test case idea:** Temp git repo with a committed `.env` containing `API_KEY=fake_value_for_tests` and an untracked, ignored `.env.local`; expect High for the first, info for the second.
- **References:**
  - https://socket.dev/blog/nx-packages-compromised
  - https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html
  - Basak et al., secret management practices (arXiv 2208.11280): https://arxiv.org/abs/2208.11280
  - T1552.001

### CR-06 Tokens in shell and REPL history
- **What it is:** Commands like `export GITHUB_TOKEN=...`, `curl -H "Authorization: Bearer ..."`, `docker login -p ...` or `mysql -p...` end up in `~/.bash_history`, `~/.zsh_history`, `~/.local/share/fish/fish_history`, and REPL histories (`~/.python_history`, `~/.node_repl_history`, `~/.psql_history`, `~/.mysql_history`). Harvesters and generic secret scanners sweep these files.
- **Real-world examples:** Shai-Hulud ran TruffleHog over the home/runner filesystem, which covers history files (https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack). Shell History is its own MITRE sub-technique, T1552.003.
- **What a defender inspects (indicators):** File presence, size and mode; `HISTCONTROL`/`HISTIGNORE` settings in rc files; a *redacted* count of token-shaped strings (see proposal), never echoing them.
- **Bigman today:** Gap. `~/.bashrc`, `~/.zshrc` and `~/.profile` are hashed by `bigman changes`, but history files are not inspected.
- **Proposed mitigation / detection:** `secret.pattern` scanner over a fixed list of history files and dotfiles. High-confidence, prefix-anchored rules only, derived from gitleaks' MIT-licensed `config/gitleaks.toml`: GitHub `ghp_/gho_/ghu_/ghs_/ghr_/github_pat_`, npm `npm_`, PyPI `pypi-`, AWS `AKIA/ASIA`, GitLab `glpat-`, Slack `xox[abposr]-`, Hugging Face `hf_`, Docker Hub `dckr_pat_`, Anthropic `sk-ant-`, OpenAI `sk-proj-`, Stripe `sk_live_`, and PEM `-----BEGIN ... PRIVATE KEY-----`. Output: rule ID, file, line number, and a truncated SHA-256 of the match so the user can correlate across reports without exposure. Never print any characters of the match and never verify tokens online (verification sends the secret to a third party, which conflicts with Bigman's no-network default). Guidance: rotate first, then remove the lines; set `HISTCONTROL=ignorespace` and use a leading space or a secret manager for one-off tokens.
- **Test case idea:** History fixture containing `export GH_TOKEN=ghp_` plus 36 `A` characters and an AWS documented example key; assert two findings, the report bytes contain neither string, and the hash prefix is stable across runs.
- **References:**
  - https://attack.mitre.org/techniques/T1552/003/
  - https://github.com/gitleaks/gitleaks/blob/master/config/gitleaks.toml
  - https://github.blog/engineering/platform-security/behind-githubs-new-authentication-token-formats/
  - https://docs.github.com/en/code-security/secret-scanning/introduction/supported-secret-scanning-patterns

### CR-07 Secrets in environment variables and process memory
- **What it is:** Tokens exported in `~/.bashrc`, `~/.profile` or `~/.config/environment.d/*.conf` live in every child process's environment. Any same-user process can read them via `/proc/<pid>/environ`, and any install script inherits them directly. With a permissive `ptrace` policy, same-user processes can also read other processes' memory, which is how in-memory OIDC and session tokens get stolen.
- **Real-world examples:** Shai-Hulud dumped the whole `process.env`, capturing `GITHUB_TOKEN`, `NPM_TOKEN` and AWS keys (Wiz, StepSecurity coverage above). In the TanStack wave (May 2026), attacker binaries on a GitHub Actions runner read OIDC tokens from the runner's memory via `/proc/<pid>/mem` and published malicious versions with valid provenance (Orca, above). CircleCI's 2023 incident report notes the attacker "extracted encryption keys from a running process" (https://circleci.com/blog/jan-4-2023-incident-report/).
- **What a defender inspects (indicators):** `cat /proc/sys/kernel/yama/ptrace_scope` (0 = any same-user process can attach; 1 = restricted to descendants). The kernel's built-in default is 1 and Ubuntu and Arch keep it, but Fedora (the `elfutils-default-yama-scope` package's `/usr/lib/sysctl.d/10-default-yama-scope.conf`) and Debian (a kernel patch, `yama-disable-by-default.patch`) ship 0. Yama only governs attach-mode access, so it protects `/proc/<pid>/mem`, `process_vm_readv` and `ptrace` attach, not `/proc/<pid>/environ`, which any same-user process can read at every setting; token-like variable *names* (`*_TOKEN`, `*_SECRET*`, `AWS_SECRET_ACCESS_KEY`, `NPM_TOKEN`, `BW_SESSION`) assigned in rc files or `environment.d`; `env | cut -d= -f1` for names only.
- **Bigman today:** Partial. `harden.sysctl.ptrace_scope` (Review) reports `ptrace_scope` = 0, names the sysctl file and package that set it, and offers a previewed, reversible change to 1 (see [hardening research](../research/hardening.md)). Exported token names in rc files are not checked yet.
- **Proposed mitigation / detection:** `secret.env-export` check: parse rc files and `environment.d` for exported names matching a fixed list, reporting names and line numbers only. Report `ptrace_scope` = 0 as Review (cross-ref [host integrity](./04-host-integrity-and-hardening-audit.md)). Guidance: load tokens on demand from a keyring (`secret-tool lookup`, `pass`) in a wrapper, not globally.
- **Test case idea:** `.bashrc` fixture with `export NPM_TOKEN=fake`; assert the finding names `NPM_TOKEN` and line 1 without the value; mock `ptrace_scope` path via an injectable root.
- **References:**
  - https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html
  - https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/
  - https://circleci.com/blog/jan-4-2023-incident-report/
  - T1003.007 OS Credential Dumping: Proc Filesystem: https://attack.mitre.org/techniques/T1003/007/

### CR-08 Passphrase-less SSH private keys
- **What it is:** An unencrypted `~/.ssh/id_*` file is a complete credential: copying it gives push access to GitHub/GitLab and login to every server whose `authorized_keys` trusts it. Passphrases (bcrypt-KDF in the OpenSSH format) make a stolen copy much less useful. FIDO2 keys (`ed25519-sk`/`ecdsa-sk`, OpenSSH 8.2+) make it nearly useless without the physical token, and `verify-required` (8.4+) also demands a PIN.
- **Real-world examples:** Nx/s1ngularity (2025), TrapDoor (May 2026, which also attempted SSH-based lateral movement), the KICS and xinference payloads (Apr 2026) and sckit (`id_ecdsa`, Sep 2026) all list SSH keys as targets (sources in CR-02/CR-03 and https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html).
- **What a defender inspects (indicators):** For each private key file, the header only. In the OpenSSH format (`-----BEGIN OPENSSH PRIVATE KEY-----`), the base64 body starts with the magic `openssh-key-v1\0` followed by the length-prefixed `ciphername`; `none` means unencrypted. Legacy PEM keys show `Proc-Type: 4,ENCRYPTED` when encrypted; PKCS#8 uses `BEGIN ENCRYPTED PRIVATE KEY`. The key type (e.g. `sk-ssh-ed25519@openssh.com`) can be read from the matching `.pub` file. Read-only commands: `ssh-keygen -l -f ~/.ssh/id_ed25519.pub`; `stat -c '%a' ~/.ssh/id_*`.
- **Bigman today:** Gap. `.ssh/` appears only as a string in `shell.credentials` and `trace.sensitive-path`. The startup baseline hashes `~/.ssh/authorized_keys` but not private keys.
- **Proposed mitigation / detection:** `ssh.key-unencrypted` check. Read at most the first ~200 bytes of each candidate in `~/.ssh` (files whose first line is a PEM/OpenSSH private-key header), base64-decode only the leading block needed to reach `ciphername`, and stop. Never decode the private section, and never call `ssh-keygen -y`, which derives key material. Severity High for software keys with `none`, Review for `sk-` keys with `none` (the file holds only a key handle), no finding for encrypted keys. Guidance: `ssh-keygen -p` to add a passphrase, or migrate to `ed25519-sk` with `-O verify-required`; register FIDO keys on GitHub/GitLab.
- **Test case idea:** Generate synthetic keys at test time into a temp dir (`ssh-keygen -t ed25519 -N '' -f k1`, and `-N 'test-pass' -f k2`). Expect a finding for k1 only. Also feed a truncated or garbage body to prove the parser fails closed (reported as "incomplete").
- **References:**
  - https://github.com/openssh/openssh-portable/blob/master/PROTOCOL.key
  - https://www.openssh.org/txt/release-8.2
  - https://www.openssh.org/txt/release-8.4
  - https://man.openbsd.org/ssh-keygen
  - https://www.yubico.com/blog/github-now-supports-ssh-security-keys/
  - T1552.004 Private Keys: https://attack.mitre.org/techniques/T1552/004/

### CR-09 `ssh-agent` forwarding and agent-socket abuse
- **What it is:** `ForwardAgent yes` (especially under `Host *`) exposes the local agent socket on every server the user connects to. Anyone with root on that server, or anyone who controls it, can use all loaded keys for as long as the session lasts. Locally, any same-user process can use `SSH_AUTH_SOCK` to sign with loaded keys. Confirmation prompts (`ssh-add -c`) do not show the destination and can be phished, so OpenSSH recommends avoiding forwarding (use `ProxyJump`) or using destination-restricted keys (`ssh-add -h`, OpenSSH 8.9+).
- **Real-world examples:** Matrix.org (2019): an attacker who had compromised a Jenkins worker hijacked a forwarded SSH agent to reach production; the post-mortem's remediations include disabling agent forwarding (https://matrix.org/blog/2019/05/08/post-mortem-and-remediations-for-apr-11-security-incident/). CVE-2023-38408 (OpenSSH < 9.3p2): a forwarded agent could be made to load PKCS#11 libraries and run code on the *client*; 9.3p2 disabled remote PKCS#11 loading by default (https://www.openssh.org/txt/release-9.3p2, https://nvd.nist.gov/vuln/detail/CVE-2023-38408).
- **What a defender inspects (indicators):** `grep -n -i -E '^\s*(Host|ForwardAgent|AddKeysToAgent)' ~/.ssh/config /etc/ssh/ssh_config /etc/ssh/ssh_config.d/*`; `ssh -G <host> | grep -i forwardagent`; `ssh-add -l` (fingerprints only; many long-lived loaded keys is a risk); OpenSSH version (`ssh -V`) against 9.3p2.
- **Bigman today:** Gap. `~/.ssh/config` is listed in the brief as not yet baselined by `bigman changes`.
- **Proposed mitigation / detection:** `ssh.agent-forwarding` check (Review when `ForwardAgent yes` applies to `Host *` or broad wildcards); `ssh.agent-keys` info listing loaded fingerprint count and whether any are non-FIDO; add `~/.ssh/config` and `~/.ssh/config.d/` to the startup baseline so new forwarding or `ProxyCommand` entries appear as changes. Feed-driven advisory: OpenSSH version below a fixed-version list. Guidance: `ProxyJump`, `AddKeysToAgent confirm`, `ssh-add -t <lifetime>`, destination constraints, FIDO keys.
- **Test case idea:** `ssh_config` fixture with `Host *\n  ForwardAgent yes` → finding; `Host bastion\n  ForwardAgent yes` → lower severity; parser ignores comments.
- **References:**
  - https://www.openssh.org/agent-restrict.html
  - https://www.openssh.org/txt/release-8.9
  - https://man.openbsd.org/ssh_config
  - https://matrix.org/blog/2019/05/08/post-mortem-and-remediations-for-apr-11-security-incident/
  - T1563.001 Remote Service Session Hijacking: SSH Hijacking: https://attack.mitre.org/techniques/T1563/001/

### CR-10 Weak permissions and ownership on credential paths
- **What it is:** Group- or world-readable `~`, `~/.ssh`, `~/.gnupg`, `~/.aws` or token files expose secrets to other local accounts, to services running as other users, to shared NFS homes and to backups. OpenSSH refuses keys with loose permissions and GnuPG warns about unsafe home permissions, but most other tools (AWS CLI, npm, Docker) do not. Permissions do **not** protect against same-user malware, which is why this is Medium on a single-user laptop.
- **Real-world examples:** Classic misconfiguration rather than a named incident. Git's own docs say the store file is protected "only by filesystem permissions" (CR-01). GCM notes that its plaintext store directory gets `700` on creation but existing directories are not changed.
- **What a defender inspects (indicators):** `stat -c '%a %U:%G %F %n' ~ ~/.ssh ~/.ssh/* ~/.gnupg ~/.aws/* ~/.kube/config ~/.docker/config.json ~/.npmrc ~/.pypirc ~/.git-credentials ~/.netrc`; owner ≠ user; any of these being a symlink out of `$HOME`; group/other bits set.
- **Bigman today:** Partial. `harden.home.readable` reports a home folder other accounts can list or open (Review when other people's accounts exist) and offers `chmod o-rwx` with an undo to the previous mode ([hardening research](../research/hardening.md)). Individual credential paths are not checked yet. Bigman's own canary files are created `0600` with `O_NOFOLLOW`, which is good practice to reuse.
- **Proposed mitigation / detection:** `cred.permissions` as part of the inventory: fixed path list, `lstat` only, flag mode & 0o077 ≠ 0, foreign owner, or symlinks. Suggest `chmod` commands as text; never change modes automatically (consistent with "no automatic cleanup").
- **Test case idea:** Temp `HOME` with `.ssh` at 0755 and `id_test` at 0644, plus a `.npmrc` symlink to `/tmp/x`; expect three findings with exact mode strings.
- **References:**
  - https://man.openbsd.org/ssh
  - https://git-scm.com/docs/git-credential-store
  - https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md
  - Cross-ref [host integrity](./04-host-integrity-and-hardening-audit.md)

### CR-11 Browser saved passwords and cookie databases
- **What it is:** Chromium-family browsers keep `Login Data` (passwords) and `Cookies` in SQLite under `~/.config/<browser>/<profile>/`. On Linux the values are encrypted with a key held in GNOME Keyring/KWallet (prefix `v11`). If no keyring is available, Chromium falls back to the `basic` store, where values use a key hardcoded in the source (prefix `v10`): obfuscation rather than encryption. Firefox keeps `logins.json` and `key4.db`, protected only if the user sets a Primary Password. Infostealers target exactly these files.
- **Real-world examples:** Mandiant tied the 2024 Snowflake intrusions (about 165 organizations notified) to credentials from Vidar, RedLine, Raccoon, Lumma and other infostealers, some infected in 2020 (UNC5537 report, above). TrapDoor (2026) lists "browser data" among its targets. Chrome's App-Bound Encryption (2024) and Device Bound Session Credentials (Windows GA in Chrome 146, April 2026) do not cover Linux, and the DBSC intent to ship did not commit to all platforms (https://security.googleblog.com/2024/07/improving-security-of-chrome-cookies-on.html; https://thehackernews.com/2026/04/google-rolls-out-dbsc-in-chrome-146-to.html).
- **What a defender inspects (indicators):** Which browser profiles exist; whether the browser runs with `--password-store=basic` (check `.desktop` overrides, `~/.config/chrome-flags.conf`/`chromium-flags.conf`, VS Code `argv.json` `password-store`); whether a Secret Service provider is on the session bus; whether Firefox profiles have a Primary Password. Directly checking the `v10`/`v11` prefix would require opening the SQLite database, which Bigman should not do by default (see open questions).
- **Bigman today:** Gap. The browser extension handles phishing and download scanning; `Login Data`, `Cookies` and `.mozilla` are only strings in `shell.credentials`/`trace.sensitive-path`.
- **Proposed mitigation / detection:** `browser.password-store-basic` check: scan launcher and flags files for `password-store=basic` and report whether `org.freedesktop.secrets` is available. `browser.profile-inventory` (info) lists which browsers and profiles hold saved credentials, by file presence and size only. Guidance: use a dedicated password manager with autofill, enable a Firefox Primary Password, sign out of high-value sessions when idle, and separate browser profiles for maintainer accounts. Cross-ref [desktop and sandboxing](./08-desktop-and-sandboxing.md) for Flatpak/Snap-confined browsers.
- **Test case idea:** Fixture `chromium-flags.conf` containing `--password-store=basic` → finding; fixture profile dir with zero-byte `Login Data` → inventory row, no file content read.
- **References:**
  - https://chromium.googlesource.com/chromium/src/+/HEAD/docs/linux/password_storage.md
  - https://code.visualstudio.com/docs/configure/settings-sync
  - https://support.mozilla.org/en-US/kb/use-primary-password-protect-stored-logins
  - https://cloud.google.com/blog/topics/threat-intelligence/unc5537-snowflake-data-theft-extortion
  - T1555.003 Credentials from Web Browsers: https://attack.mitre.org/techniques/T1555/003/

### CR-12 Session-token theft and MFA bypass
- **What it is:** Session cookies and registry/forge session tokens are post-MFA artifacts. Replaying them gives the attacker a logged-in session without the password or second factor. For maintainers this means npm, PyPI, GitHub and cloud consoles. Phishing kits that proxy logins in real time capture the same tokens (cross-ref [phishing](./01-phishing-social-engineering.md)).
- **Real-world examples:** CircleCI (Jan 2023): malware on an engineer's laptop, missed by antivirus, stole a 2FA-backed SSO session (https://circleci.com/blog/jan-4-2023-incident-report/). Ledger Connect Kit (Dec 2023): a former employee was phished and the attacker used their npm *session token* to bypass 2FA and publish a wallet drainer; about $600k was lost (https://www.ledger.com/blog/security-incident-report). The chalk/debug compromise (Sep 2025) began with a phishing email from `npmjs.help` and shipped a browser wallet-address interceptor (https://www.aikido.dev/blog/npm-debug-and-chalk-packages-compromised).
- **What a defender inspects (indicators):** Active sessions and tokens on the account side (GitHub "Sessions", npm token list); cookie database presence on the host (CR-11); unusual logins in provider audit logs. On the host, little can be seen until something reads the cookie store.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** Mostly guidance and tripwires. A post-incident "rotate everything" checklist generated from the CR-01..CR-05 inventory, giving per-provider revoke URLs and the order of operations. Decoys (CR-22) to detect reads. A feed-driven advisory when a phishing domain imitating a registry is published (links to [phishing](./01-phishing-social-engineering.md)). Encourage phishing-resistant MFA (WebAuthn/passkeys) on registry accounts.
- **Test case idea:** Snapshot test of the generated rotation checklist from a synthetic inventory (git store + `.npmrc` + `.aws`): ordering and URLs, with no secrets present.
- **References:**
  - https://attack.mitre.org/techniques/T1539/
  - https://attack.mitre.org/techniques/T1550/004/
  - https://circleci.com/blog/jan-4-2023-incident-report/
  - https://www.ledger.com/blog/security-incident-report
  - https://socket.dev/blog/ledger-connect-kit-supply-chain-attack-wallet-drainer

### CR-13 Crypto wallet files and browser wallet extensions
- **What it is:** Desktop wallets (e.g. `~/.bitcoin/wallets`, `~/.electrum/wallets`, `~/.config/Exodus`), developer keypairs (Solana CLI `~/.config/solana/id.json` is an unencrypted keypair by default; Ethereum keystores under `~/.ethereum/keystore`) and browser wallet extensions (stored under `<profile>/Local Extension Settings/<extension-id>/`, e.g. MetaMask `nkbihfbeogaeaoehlefnkodbefgpgknn`) are high-value, immediately cashable targets. Stealers copy the vault and attack the password offline. Clippers swap addresses at signing time instead.
- **Real-world examples:** GlassWorm (Oct 2025 onward) targeted 49 wallet extensions, plus GitHub/npm/OpenVSX credentials, from VS Code/OpenVSX extensions (https://www.koi.ai/blog/glassworm-first-self-propagating-worm-using-invisible-code-hits-openvsx-marketplace). s1ngularity explicitly searched for wallet artifacts. TrapDoor and xinference (2026) list crypto wallets. The chalk/debug payload rewrote wallet addresses in the browser.
- **What a defender inspects (indicators):** Presence, mode and mtime of known wallet paths; extension IDs present in each Chromium profile's `Local Extension Settings`; unexpected readers (only visible with canaries or auditd).
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** `wallet.present` inventory (existence/mode only) with guidance: hardware wallet, encrypted keystore, keep hot-wallet extensions in a separate browser profile, and do not keep keypairs for mainnet in `~/.config/solana/id.json`. Offer a wallet-shaped decoy (CR-22). Add wallet paths to `trace.sensitive-path` and `shell.credentials`.
- **Test case idea:** Temp profile tree with `Local Extension Settings/nkbihfbeogaeaoehlefnkodbefgpgknn/` (empty) → inventory row naming the extension; `~/.config/solana/id.json` with `[0,0,0]` at 0644 → permissions finding.
- **References:**
  - https://www.koi.ai/blog/glassworm-first-self-propagating-worm-using-invisible-code-hits-openvsx-marketplace
  - https://www.aikido.dev/blog/npm-debug-and-chalk-packages-compromised
  - https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html
  - T1005 Data from Local System: https://attack.mitre.org/techniques/T1005/; T1555.005 Password Managers: https://attack.mitre.org/techniques/T1555/005/

### CR-14 Secrets committed to git
- **What it is:** Keys pasted into config, tests or notebooks get committed and pushed. Deleting the file, or even the commit, does not remove the secret. On GitHub, commits in deleted forks and in private repos sharing a fork network can stay reachable by hash ("Cross Fork Object Reference"). Leaked secrets stay valid for years because nobody rotates them.
- **Real-world examples:** GitGuardian counted about 28.65 million new hardcoded secrets in public GitHub commits in 2025 (+34%), and found that 64% of secrets confirmed valid in 2022 were still valid in 2026 (THN summary above). Truffle Security (Jul 2024) showed deleted-fork and private-fork data remains accessible and found 40 valid API keys in deleted forks of three AI-company repos (https://trufflesecurity.com/blog/anyone-can-access-deleted-and-private-repo-data-github). GitHub turned on push protection by default for pushes to public repos at the user level in Feb/Mar 2024; repository-level push protection remains opt-in (https://docs.github.com/en/code-security/secret-scanning/introduction/about-push-protection).
- **What a defender inspects (indicators):** Run a scanner over history in redact mode, e.g. `gitleaks git --redact <repo>` or `trufflehog git file://<repo> --no-verification`; check for a pre-commit hook or `pre-commit` config that runs a scanner; check push protection on the user and repo settings.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** An optional integration `bigman audit gitleaks DIR`, alongside the existing `bigman audit osv`, that parses gitleaks JSON with `--redact`. Bigman should use gitleaks (MIT) as an external tool rather than vendoring TruffleHog (AGPL-3.0, which matches Bigman's AGPL licence, but its default verification makes network calls). A `git.secret-scan-hook` check reports whether a repo has a secret-scanning pre-commit hook. Guidance: a leaked secret must be **revoked**; history rewriting alone is insufficient.
- **Test case idea:** Temp repo with a commit adding `AKIAIOSFODNN7EXAMPLE`, then a commit deleting it. With a stub gitleaks JSON fixture, assert Bigman reports file, commit and rule with the value redacted.
- **References:**
  - https://github.com/gitleaks/gitleaks
  - https://github.com/trufflesecurity/trufflehog
  - https://trufflesecurity.com/blog/anyone-can-access-deleted-and-private-repo-data-github
  - https://docs.github.com/en/code-security/secret-scanning/introduction/about-push-protection
  - https://www.ndss-symposium.org/ndss-paper/how-bad-can-it-git-characterizing-secret-leakage-in-public-github-repositories/
  - T1552.001; T1213.003 Code Repositories: https://attack.mitre.org/techniques/T1213/003/

### CR-15 Secrets in published artifacts and logs
- **What it is:** A clean git history does not guarantee a clean release: `npm pack` includes untracked files that are not in `.npmignore`/`files`, sdists and wheels can include `.env` files or local configs, container images bake in build-time secrets, and CI logs print environment variables.
- **Real-world examples:** GitGuardian's scan of 15 million public Docker images found 100,000 valid secrets (2025 report, above). tj-actions/changed-files (CVE-2025-30066, Mar 2025) dumped CI secrets into public workflow logs (https://nvd.nist.gov/vuln/detail/CVE-2025-30066).
- **What a defender inspects (indicators):** `npm pack --dry-run` file list; `tar -tzf` of the tarball for `.env`, `.npmrc`, `id_*`, `*.pem`, `credentials*`; `python -m zipfile -l` on wheels; image layers with a scanner in redact mode.
- **Bigman today:** Partial. `bigman packages` reads `.tgz` contents without extracting, but only applies lifecycle-script rules.
- **Proposed mitigation / detection:** Add a `package.secret-file` rule (sensitive filenames inside tarballs) and run the CR-06 redacted pattern set over text members of `.tgz`/`.whl`/`.crate` within existing size limits. Position it as a maintainer pre-publish check. Cross-ref [supply-chain](./02-supply-chain-packages.md).
- **Test case idea:** Build a `.tgz` fixture in-test containing `package/.npmrc` with a fake `_authToken` and `package/index.js`; expect `package.secret-file` and `secret.pattern` findings, values redacted.
- **References:**
  - https://www.gitguardian.com/state-of-secrets-sprawl-report-2025
  - https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised
  - https://nvd.nist.gov/vuln/detail/CVE-2025-30066

### CR-16 GPG signing keys without passphrase or hardware backing
- **What it is:** Maintainers sign commits, tags and release artifacts with GPG or SSH keys. An unprotected secret key under `~/.gnupg/private-keys-v1.d/` lets a thief produce "verified" malicious releases. GnuPG's key file format makes the protection state visible from the first tokens of the file: `(private-key` is unprotected, `(protected-private-key` is passphrase-protected, and `(shadowed-private-key` means the key lives on a smartcard/YubiKey, with only a stub on disk.
- **Real-world examples:** No widely reported 2024-2026 incident of stolen maintainer GPG keys was found (unverified). Several 2025-2026 harvesters sweep the whole home directory, which would include `~/.gnupg`. GitHub shows signed commits as "Verified", so a stolen signing key turns into apparent legitimacy.
- **What a defender inspects (indicators):** `gpg --list-secret-keys --with-keygrip` (keys listed with `>` or `card-no` are on a card); `stat -c '%a' ~/.gnupg ~/.gnupg/private-keys-v1.d`; the first line of each `*.key` file (`Key: (shadowed-private-key ...` vs `(protected-private-key` vs `(private-key`); `git config --get user.signingkey`, `gpg.format`.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** `gpg.key-unprotected` check: read at most the first 128 bytes of each `.key` file and match the three S-expression tokens; stop reading after the token. Report keygrip (filename) and status. For SSH-format signing (`gpg.format=ssh`), reuse CR-08. Guidance: passphrase, or move the signing subkey to a YubiKey/OpenPGP card. Avoid forwarding the gpg-agent to untrusted hosts (see the GnuPG wiki on agent forwarding).
- **Test case idea:** Three fixture files whose first line starts `Key: (private-key`, `Key: (protected-private-key` and `Key: (shadowed-private-key` respectively, padded with zeros; expect findings only for the first and assert the reader never consumes past byte 128.
- **References:**
  - https://github.com/gpg/gnupg/blob/master/doc/keyformat.txt
  - https://wiki.gnupg.org/AgentForwarding
  - https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification
  - T1552.004

### CR-17 Desktop keyring exposure and silent plaintext fallbacks
- **What it is:** GNOME Keyring and KWallet implement the freedesktop Secret Service over D-Bus. Once the login keyring is unlocked at login (via `pam_gnome_keyring`/`pam_kwallet`), any unsandboxed process in the session can ask for items without a prompt. GNOME's position is that untrusted apps must not reach the session bus (CVE-2018-19358, disputed). Keyring helpers therefore protect against offline disk theft, backups and other users, not against malware running as the user. The bigger practical problem is silent fallback: when no keyring is reachable (headless, SSH, minimal WMs), gh, Docker, Chromium and VS Code fall back to plaintext or hardcoded-key storage.
- **Real-world examples:** CVE-2018-19358 (https://nvd.nist.gov/vuln/detail/CVE-2018-19358). Documented fallbacks: gh "falls back to plaintext" (gh manual), Docker falls back to base64 `config.json`, Chromium to `basic`, and VS Code warns its `basic` store is "obfuscation" (sources in CR-04/CR-11).
- **What a defender inspects (indicators):** Whether `org.freedesktop.secrets` is owned on the session bus (`busctl --user list | grep -i secrets`); which provider (gnome-keyring-daemon, kwalletd, KeePassXC); whether the login keyring auto-unlocks; plaintext artifacts from fallbacks (CR-04, CR-11).
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** `keyring.provider` info finding (provider name, or "none: helpers will fall back to plaintext"). Correlate it with the inventory: if no provider exists and plaintext stores exist, explain the cause. Guidance: KeePassXC as the Secret Service provider with per-access confirmation enabled; a separate keyring that does not auto-unlock for high-value tokens; Flatpak apps use the Secret portal (per-app secrets) instead of the full bus.
- **Test case idea:** Abstract the D-Bus query behind a trait; unit-test the "no provider + `hosts.yml` present" correlation message with a fake.
- **References:**
  - https://specifications.freedesktop.org/secret-service-spec/latest/
  - https://wiki.archlinux.org/title/GNOME/Keyring
  - https://wiki.archlinux.org/title/KDE_Wallet
  - https://keepassxc.org/docs/KeePassXC_UserGuide#_secret_service_integration
  - https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html
  - https://nvd.nist.gov/vuln/detail/CVE-2018-19358

### CR-18 Supply-chain payloads that harvest developer credentials
- **What it is:** The dominant 2025-2026 pattern is install-time code (npm `preinstall`/`postinstall`, Python import, Rust `build.rs`, VS Code extensions, CI actions). It collects tokens from dotfiles and the environment, often by running a real secret scanner (TruffleHog), validates them, exfiltrates to GitHub repos or gists, and republishes with stolen publish tokens. This document covers what is stolen; how the package gets in is covered in [supply-chain](./02-supply-chain-packages.md).
- **Real-world examples (token types harvested, verified against reports):**
  - Shai-Hulud (Sep 2025) and Sha1-Hulud 2.0 (Nov 2025): npm, GitHub, AWS/GCP/Azure via TruffleHog plus `process.env` and IMDS (Wiz; Unit 42).
  - s1ngularity/Nx (Aug 2025): GitHub and npm tokens, SSH keys, `.env`, wallets; 2,180 accounts and 7,200 repos across three phases (Wiz; GHSA-cxm3-wv7p-598c).
  - GlassWorm (Oct 2025 onward): GitHub, npm and OpenVSX credentials plus 49 wallet extensions (Koi).
  - Apr 21-23, 2026 wave (KICS images/extensions, CanisterSprawl `pgserve`, xinference, Bitwarden CLI 2026.4.0): GitHub, AWS, Azure, GCP, npm config, SSH, env vars, wallets (GitGuardian; https://thehackernews.com/2026/04/bitwarden-cli-compromised-in-ongoing.html).
  - TeamPCP "Mini Shai-Hulud" TanStack wave (May 2026): GitHub/npm tokens, cloud, Kubernetes, Vault, OIDC tokens from runner memory; persistence as `~/.config/systemd/user/gh-token-monitor.service`, which wipes `$HOME` if the stolen token is revoked (Orca).
  - TrapDoor (May 2026, npm/PyPI/crates.io): secrets, wallets, SSH keys, cloud, browser data; validates AWS and GitHub tokens (THN/Socket).
  - sckit/MemTensor (Sep 2026): `.npmrc`, `.vault-token`, `id_ecdsa`, `credentials.db`, AWS, GitHub/GitLab, npm/PyPI, Hugging Face, Vault, Slack, Stripe, SendGrid, JWTs (THN).
- **What a defender inspects (indicators):** Unexpected GitHub repos or gists in the account (`s1ngularity-repository*`, "Shai-Hulud" descriptions); new user systemd units (`gh-token-monitor.service`); `~/.claude/`, `.vscode/` payload files (`router_runtime.js`, `setup.mjs` per Orca); a TruffleHog binary in caches; provider audit logs showing token validation calls. Important: the TanStack persistence destroys data when tokens are revoked, so check for and disable persistence *before* revoking (from Orca's write-up).
- **Bigman today:** Partial. `bigman inspect` (`shell.credentials`), `bigman packages` (`package.lifecycle`), sandbox `trace.sensitive-path`, and `bigman changes` would surface a new `~/.config/systemd/user/*.service`. The canary catches harvesters that read its decoy.
- **Proposed mitigation / detection:** (1) Extend the `trace.sensitive-path` and `shell.credentials` lists to the full CR-01..CR-05 path table and wallet paths. (2) Feed-driven IOC checks: a versioned local rule file of filenames and unit names from advisories, checked read-only. (3) A "suspected harvest" runbook in the UI: check persistence first, then rotate in order (registry publish tokens, GitHub, cloud, SSH), using the inventory as the checklist. (4) Recommend `ignore-scripts` and `bigman package-policy` (already implemented).
- **Test case idea:** Fixture home with `~/.config/systemd/user/gh-token-monitor.service` whose `ExecStart=/bin/true` → IOC finding plus a `bigman changes` diff; installer fixture `cat ~/.config/gcloud/credentials.db` → extended `shell.credentials` hit.
- **References:**
  - https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack
  - https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/
  - https://www.wiz.io/blog/s1ngularitys-aftermath
  - https://orca.security/resources/blog/tanstack-npm-supply-chain-worm/
  - https://blog.gitguardian.com/three-supply-chain-campaigns-hit-npm-pypi-and-docker-hub-in-48-hours/
  - https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html
  - https://thehackernews.com/2026/09/compromised-memtensor-packages-deliver.html
  - T1552.001, T1528, T1555

### CR-19 AI coding agents used to enumerate secrets
- **What it is:** Developers run AI CLIs/IDE agents with broad file access, sometimes with permission prompts disabled. Malicious code can invoke these agents locally with a prompt asking them to find secrets, and poisoned repo instruction files (`CLAUDE.md`, `.cursorrules`) can try to steer an agent into reading or exposing them. The agent's own config often stores API keys in plaintext as well.
- **Real-world examples:** s1ngularity (Aug 2025) invoked Claude Code, Gemini CLI and Amazon Q with permission-skipping flags and a prompt to enumerate wallets, SSH keys and `.env` files. About half of victims had an AI CLI installed, and AI-mediated exfiltration succeeded in under 25% of cases (Wiz, Socket, Snyk). TrapDoor (May 2026) opened PRs adding `.cursorrules`/`CLAUDE.md` with hidden instructions for a "security scan" that would expose secrets (THN/Socket).
- **What a defender inspects (indicators):** Agent config files and whether they store API keys or MCP server tokens in plaintext (paths vary by tool and version; verify per tool); settings that disable confirmation prompts; unexpected instruction files in freshly cloned repos.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** Add known AI-CLI and MCP config paths to the `cred.plaintext-store` table (paths maintained as data, since they change often). An `agent.unsafe-permissions` Review finding when a config disables approval prompts globally. Treat repo-supplied agent instruction files as a `bigman packages`-style inspection target (cross-ref [supply-chain](./02-supply-chain-packages.md)). Canaries (CR-22) catch agents that read decoys.
- **Test case idea:** Fixture JSON config with an `"env": {"API_KEY": "fake"}` MCP server block → `cred.plaintext-store`; fixture settings with a permissions-bypass flag → Review.
- **References:**
  - https://socket.dev/blog/nx-packages-compromised
  - https://snyk.io/blog/weaponizing-ai-coding-agents-for-malware-in-the-nx-malicious-package
  - https://www.wiz.io/blog/s1ngularitys-aftermath
  - https://thehackernews.com/2026/05/trapdoor-supply-chain-attack-spreads.html

### CR-20 Long-lived, never-rotated tokens
- **What it is:** Classic PATs, static cloud keys and registry tokens with no expiry turn every leak into a durable compromise. Short-lived credentials (OIDC trusted publishing, SSO sessions, 7- to 90-day fine-grained tokens) shrink the window. The TanStack wave showed that even OIDC tokens can be taken from memory on a compromised runner. Provenance attestations then look valid, so short lifetimes have to be paired with runner hygiene.
- **Real-world examples:** 64% of secrets valid in 2022 were still valid in 2026 (GitGuardian). Snowflake victims' credentials had not been rotated since infostealer infections years earlier (Mandiant). npm revoked all classic tokens on 2025-12-09 and shortened write-token lifetimes.
- **What a defender inspects (indicators):** mtime of each plaintext store as a proxy for token age; token types (`ghp_` classic vs `github_pat_` fine-grained) via redacted classification; whether CI uses OIDC (`id-token: write`, trusted publisher configured) instead of stored secrets.
- **Bigman today:** Gap.
- **Proposed mitigation / detection:** Add "age" and "class" columns to the inventory (`classic-pat`, `fine-grained-pat`, `static-cloud-key`, `session-cache`). Review when a long-lived class is older than a configurable threshold (default 90 days). Guidance links to GitHub OIDC hardening, npm/PyPI trusted publishing and AWS SSO.
- **Test case idea:** Fixture `.git-credentials` with a classic-format fake PAT and mtime set to 400 days ago → `cred.stale-long-lived`.
- **References:**
  - https://docs.github.com/en/actions/security-for-github-actions/security-hardening-your-deployments/about-security-hardening-with-openid-connect
  - https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/
  - https://docs.pypi.org/trusted-publishers/
  - https://docs.github.com/en/rest/authentication/keeping-your-api-credentials-secure

### CR-21 Password-manager CLI sessions and vault exports
- **What it is:** CLI password managers often unlock into a session token. Bitwarden CLI uses `BW_SESSION` in the environment; users put it in rc files for convenience, which hands the whole vault to any process. Unencrypted vault exports (`*.csv`/`*.json` exported from Bitwarden, 1Password, KeePass) left in `~/Downloads` are full plaintext credential dumps. The CLI itself is a supply-chain target too.
- **Real-world examples:** `@bitwarden/cli` 2026.4.0 was malicious on npm for about 93 minutes on 2026-04-22 and harvested developer credentials. Bitwarden said vault data was not affected (THN, above). Password-manager theft is MITRE T1555.005.
- **What a defender inspects (indicators):** `BW_SESSION`, `OP_SESSION_*` assignments in rc files (names only); export-like filenames (`bitwarden_export_*.json`, `*passwords*.csv`) in `~/Downloads`/`~/Documents`; installed CLI versions against advisories.
- **Bigman today:** Gap (the daily Downloads scan is ClamAV-only).
- **Proposed mitigation / detection:** Extend `secret.env-export` (CR-07) with session variable names; `secret.vault-export` filename heuristic in the Downloads scan (filename and size only, user reviews). Feed-driven advisory for compromised CLI versions.
- **Test case idea:** `~/Downloads/bitwarden_export_20260101.json` fixture with `{}` → Review finding; `.zshrc` with `export BW_SESSION=fake` → finding naming the variable only.
- **References:**
  - https://bitwarden.com/help/cli/
  - https://thehackernews.com/2026/04/bitwarden-cli-compromised-in-ongoing.html
  - https://attack.mitre.org/techniques/T1555/005/

### CR-22 No tripwires: credential theft goes unnoticed
- **What it is:** Most harvesters copy files and leave. Without a tripwire, the user learns only when the provider or the attacker acts. Honeytokens are fake credentials or files with no legitimate use, so any read or use is a high-signal alert. There are two kinds. Local decoys detect *reads* on the host (inotify). Hosted tokens such as Thinkst Canarytokens detect *use* elsewhere: a fake AWS key or kubeconfig alerts when someone tries it.
- **Real-world examples:** Thinkst's open-source Canarytokens server includes token types for AWS and Azure keys, kubeconfig, WireGuard configs, MySQL dumps, Office documents, PDFs, SAML and MCP (from the repository's `canarytokens/` modules; GPL-3.0 with Thinkst clarifications). Harvesters that validate stolen AWS keys (TrapDoor and Shai-Hulud validate tokens) would exercise a hosted AWS-key canary. No public report was found confirming a canarytoken firing during a specific 2025-2026 campaign (unverified; web search budget was exhausted before this could be checked).
- **What a defender inspects (indicators):** Canary alerts; correlation with backup or indexing jobs (Tracker, Baloo, `updatedb`, Deja Dup) that legitimately read files.
- **Bigman today:** Partial. `bigman canary arm` creates a `0600`, `O_NOFOLLOW`, never-overwriting decoy (`local-decoy` kind: an `.env`-style file with `BIGMAN_CANARY_ID` and a `bigman_decoy_` token; default `~/Documents/backup-access.env`) or a `hosted-url` kind that wraps a user-supplied HTTPS canary URL as an `[InternetShortcut]` file. The monitor watches the decoy's *parent directory* with inotify (`IN_ACCESS | IN_MODIFY | IN_ATTRIB | IN_DELETE | IN_MOVED_* | IN_CREATE | IN_DELETE_SELF | IN_MOVE_SELF`, `IN_DONT_FOLLOW`), reattaches after directory replacement, writes durable alerts (`canary.event`, High) and desktop notifications, and verifies kernel read receipts. Limits, from the code: no reader PID (inotify cannot attribute); `IN_OPEN` is not in the mask, so an open-and-`stat` or `mmap`-only access may not fire, though ordinary `read(2)` does; it only works while the systemd `--user` monitor runs; and there is a single decoy kind with no network callback unless the user supplies one. Decoys are excluded from the startup baseline snapshot so Bigman itself does not trip them.
- **Proposed mitigation / detection:**
  - New local decoy kinds, all clearly synthetic, unusable and never placed where real tools auto-load them:
    - SSH-key-shaped file at a non-default name (`~/.ssh/id_ed25519_backup` with an OpenSSH header and invalid body; ssh only auto-loads default names);
    - `.git-credentials`-shaped and `.npmrc`-shaped files inside a decoy directory such as `~/Projects/old-release-tooling/`, never in `$HOME` itself where git/npm would read them;
    - kubeconfig-shaped `~/.kube/config.bak`;
    - AWS-profile-shaped `~/.aws/credentials.old` (hosted AWS-key canary optional);
    - wallet-shaped `~/.config/solana/backup-keypair.json` or `wallet.dat` in a decoy folder;
    - a `.env.production` in a decoy project.
  - Add an optional `IN_OPEN` mode (noisier: file managers and indexers open files) and an "indexer exclusion" helper that tells the user how to exclude decoy dirs from Tracker/Baloo.
  - When an alert fires, show the CR-18 runbook (persistence check, then rotation).
  - Optional root-only attribution via auditd (`-w <path> -p r`) or fanotify, documented, not default.
- **Test case idea:** Reuse the existing `real_kernel_read_event_and_no_overwrite` pattern for each new decoy kind; assert that real tools ignore the decoy: `git credential fill` against a temp HOME does not consult the decoy directory, and `ssh -G` identity list excludes the decoy name.
- **References:**
  - https://github.com/thinkst/canarytokens
  - https://docs.canarytokens.org/guide/
  - https://github.com/thinkst/opencanary
  - https://man7.org/linux/man-pages/man7/inotify.7.html
  - Bigman source: `src/canary.rs` (create/watch), `src/system.rs` (decoy exclusion in `snapshot`, `git.plaintext-credentials`)

## Proposed Bigman checks (consolidated, deterministic, rootless)

| Rule ID (proposed) | Component | Reads | Never does |
|---|---|---|---|
| `cred.plaintext-store` | `bigman doctor` / new `bigman secrets` | Fixed path table; JSON/YAML/INI parsed for *key names* only | Print or store values |
| `cred.permissions` | same | `lstat` mode/owner/symlink | Change modes |
| `cred.stale-long-lived` | same | mtime plus token *class* from prefix | Online verification |
| `git.plaintext-credentials` v2 | doctor | `git config --show-origin --includes`, store-file stat | Read store contents |
| `ssh.key-unencrypted` | doctor | ≤200 bytes per key: header plus `ciphername` | Decode private section, call `ssh-keygen -y` |
| `ssh.agent-forwarding`, `ssh.agent-keys` | doctor | `ssh_config` parse, `ssh-add -l` fingerprints | Use the agent |
| `gpg.key-unprotected` | doctor | ≤128 bytes per `.key` | Read key material |
| `secret.pattern` | doctor / scan | History, dotfiles, `.env`, package tarballs; prefix-anchored regexes | Show any matched characters; only rule, file, line, hash prefix |
| `secret.env-file`, `secret.env-export` | doctor | Bounded project roots; rc files for variable *names* | Read beyond limits |
| `browser.password-store-basic`, `keyring.provider` | doctor | Flags files, D-Bus name ownership | Open browser databases |
| `wallet.present` | doctor | Path existence/mode | Read wallet files |
| `package.secret-file` | `bigman packages` | Tarball member names, text members within limits | Extract to disk |
| New canary kinds | `bigman canary arm --kind ...` | n/a | Overwrite files, place decoys where tools auto-load |

**Migration guidance table (for the UI).** git → `git-credential-libsecret` or GCM `secretservice`/`gpg`/`cache`. Docker → `credsStore: secretservice` or `pass`. Cargo → `cargo:libsecret`. gh → keyring (re-login without `--insecure-storage`). npm → trusted publishing, short-lived session login, granular read-only tokens for installs. PyPI → trusted publishing; twine with `keyring`. AWS → IAM Identity Center or `aws-vault`. Kubernetes → OIDC exec plugin. SSH → passphrase or `ed25519-sk` + `verify-required`, no blanket `ForwardAgent`. GPG → passphrase or smartcard subkeys.

## Tools and scripts worth mining for rules

| Tool | URL | License | What we can borrow | Caveats |
|---|---|---|---|---|
| gitleaks | https://github.com/gitleaks/gitleaks | MIT | `config/gitleaks.toml` prefix-anchored rules and keywords; `--redact` JSON output to parse in an `audit` integration | Generic/entropy rules are noisy (46% precision in Basak et al. 2023); import only prefix-anchored rules |
| TruffleHog | https://github.com/trufflesecurity/trufflehog | AGPL-3.0 | Detector list as a catalogue of token types; `--no-verification` mode for offline runs | Verification contacts providers by default; it is also the tool Shai-Hulud used to harvest |
| detect-secrets (Yelp) | https://github.com/Yelp/detect-secrets | Apache-2.0 | Baseline-file workflow (accept known findings), keyword plugin ideas | Python; last push Apr 2026 |
| ggshield (GitGuardian) | https://github.com/GitGuardian/ggshield | MIT | Pre-commit/pre-push hook patterns | Detection runs on GitGuardian's API (sends content to a service) |
| Kingfisher (MongoDB) | https://github.com/mongodb/kingfisher | Apache-2.0 | Rule YAML with token prefixes, fast Rust scanning ideas | Validation makes network calls |
| Nosey Parker (Praetorian) | https://github.com/praetorian-inc/noseyparker | Apache-2.0 | Rule set and Rust/Hyperscan design | Repository archived |
| secretlint | https://github.com/secretlint/secretlint | MIT | Per-provider rule packages | Node-based |
| awslabs/git-secrets | https://github.com/awslabs/git-secrets | Apache-2.0 | Simple AWS patterns and hook installer | AWS-focused, little maintenance |
| Thinkst Canarytokens | https://github.com/thinkst/canarytokens | GPL-3.0 with Thinkst clarifications | Token type catalogue (AWS/Azure keys, kubeconfig, WireGuard, MySQL dump, docs, MCP); hosted URLs Bigman can wrap | Self-hosting needs a public server; licence clarifications on derivative works |
| OpenCanary | https://github.com/thinkst/opencanary | BSD-3-Clause | Service-honeypot ideas (network side; cross-ref [network exposure](./07-network-exposure-and-remote-access.md)) | Not file-based |
| docker-credential-helpers | https://github.com/docker/docker-credential-helpers | MIT | Names of helpers to recommend and detect (`secretservice`, `pass`) | n/a |
| aws-vault | https://github.com/99designs/aws-vault | MIT | Keyring-backed AWS credential workflow to recommend | n/a |
| Git Credential Manager | https://github.com/git-ecosystem/git-credential-manager | MIT | Credential store options and plaintext warnings | n/a |

## Papers and long-form references

- Meli, McNiece, Reaves, 2019, "How Bad Can It Git? Characterizing Secret Leakage in Public GitHub Repositories", NDSS. https://www.ndss-symposium.org/ndss-paper/how-bad-can-it-git-characterizing-secret-leakage-in-public-github-repositories/ — Takeaway: leaked secrets on GitHub are widespread and found quickly; prefix-distinct key formats make detection reliable.
- Basak, Neil, Reaves, Williams, 2022, "What are the Practices for Secret Management in Software Artifacts?", IEEE SecDev. https://arxiv.org/abs/2208.11280 — Takeaway: 24 practices in six categories; local environment variables and external secret managers are the most recommended.
- Basak et al., 2023, "SecretBench: A Dataset of Software Secrets", MSR. https://arxiv.org/abs/2303.06729 — Takeaway: a labelled benchmark usable for testing Bigman's regex set for false positives.
- Basak et al., 2023, "A Comparative Study of Software Secrets Reporting by Secret Detection Tools", ESEM. https://arxiv.org/abs/2307.00714 — Takeaway: best precision was GitHub's scanner (75%) and best recall gitleaks (88%); false positives come from generic regexes and poor entropy checks. This supports prefix-anchored rules only.
- Ladisa et al., 2023, "SoK: Taxonomy of Attacks on Open-Source Software Supply Chains", IEEE S&P. https://arxiv.org/abs/2204.04008 — Takeaway: maintainer credential theft is a root node for many supply-chain attack paths.
- GitGuardian, 2025 and 2026, "The State of Secrets Sprawl". https://www.gitguardian.com/state-of-secrets-sprawl-report-2025 and summary https://thehackernews.com/2026/03/the-state-of-secrets-sprawl-2026-9.html — Takeaway: 28.65M new public secrets in 2025; 64% of 2022-valid secrets still valid; MCP configs are a new leak source.
- Mandiant, 2024, "UNC5537 Targets Snowflake Customer Instances for Data Theft and Extortion". https://cloud.google.com/blog/topics/threat-intelligence/unc5537-snowflake-data-theft-extortion — Takeaway: infostealer-sourced credentials stay usable for years without rotation and MFA.
- OpenSSH project, "SSH agent restriction". https://www.openssh.org/agent-restrict.html — Takeaway: forwarding is equivalent to lending your keys; prefer `ProxyJump` or destination constraints.
- Truffle Security, 2024, "Anyone can Access Deleted and Private Repository Data on GitHub". https://trufflesecurity.com/blog/anyone-can-access-deleted-and-private-repo-data-github — Takeaway: deleting a commit or fork is not remediation; revoke.

## Open questions for the maintainer

1. **Reading boundaries.** Is header-only parsing (≤200 bytes for SSH, ≤128 bytes for GPG) acceptable under Bigman's "no credentials read" coverage statement, or should that statement change to "credential *values* are never read into reports"? The current `doctor` coverage text says "no credentials read".
2. **Pattern scanning of history and dotfiles** necessarily reads secret-bearing files into memory. Is an in-memory scan that emits only rule, file, line and hash prefix acceptable, and should it be opt-in?
3. **Hash prefixes in reports.** Is a truncated SHA-256 of a matched token acceptable? It allows correlation, but for a known-format token a full hash is a guessable oracle only if the attacker already holds the token. Alternative: no hash at all.
4. **Browser databases.** Should Bigman ever open `Cookies`/`Login Data` read-only to count `v10` vs `v11` entries, or stay with flags-and-keyring inference?
5. **Decoy placement.** Placing decoys inside `~/.ssh`, `~/.aws` and `~/.kube` is more realistic but touches directories real tools use. Is a dedicated decoy-project directory acceptable as the default, with in-tool-directory placement opt-in?
6. **`IN_OPEN` in the canary mask.** Add it as an option, accepting more false positives from file managers and indexers?
7. **External scanner integration.** Integrate gitleaks (MIT) as an optional external tool like OSV-Scanner, or reimplement a small prefix-rule set in Rust (attributing gitleaks rules under MIT)?
8. **AI-CLI config paths** change often. Should they live in a data file updated through the planned feed workflow rather than in code?
9. **Root-only attribution.** Should Bigman document (not automate) an auditd/fanotify recipe so users can learn *which process* read a canary?
10. **Rotation runbook.** Should Bigman generate provider-specific revoke links, given the TanStack payload wipes `$HOME` when its stolen token is revoked, so persistence must be checked first?
