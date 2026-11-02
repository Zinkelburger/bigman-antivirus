---
url: "https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack"
title: "Keyv and friends compromised in npm supply chain attack"
captured: "2026-11-02T18:49:36Z"
via: "live"
sha256: "14d6474b08f0370303fc4204ddd69a7bdc575a60d17ece461b8e06f781acfc0e"
wayback: "https://web.archive.org/web/20261015102553/https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Keyv and friends compromised in npm supply chain attack

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack>
> Snapshot: <https://web.archive.org/web/20261015102553/https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack>

[Blog](https://www.aikido.dev/blog)[Vulnerabilities & Threats](https://www.aikido.dev/category/vulnerabilities-threats)

Keyv and friends compromised in active Shai-Hulud supply chain attack

# Keyv and friends compromised in active Shai-Hulud supply chain attack

Written by

[Ilyas Makari](https://www.aikido.dev/team-members/ilyas-makari)

Published on:

Aug 4, 2026

Last updated on:

Aug 5, 2026

On August 4, 2026, attackers compromised the GitHub account of the maintainer behind keyv, a key-value storage library with roughly 127 million weekly npm downloads, and used that access to inject a credential-stealing worm across the entire package family. The same maintainer owns cacheable (29M downloads/month), flat-cache (565M downloads/month), file-entry-cache (557M downloads/month), and several other widely-used caching utilities, all of which were swept up in the same attack.

The compromise was carried out by pushing malicious files directly to the main branch and then immediately cutting a new release, meaning the poisoned versions were published to npm with valid provenance signed by GitHub Actions.

The compromised packages include:

- keyv 6.0.0 (604M/month)
- flat-cache 6.1.24 (580M/month)
- file-entry-cache 11.1.6 (571M/month)
- cacheable-request 13.0.20 (137M/month)
- cacheable 2.5.1 (30M/month)
- @cacheable/memory 2.2.1 (28M/month)
- cache-manager 7.2.10 (16M/month)
- @cacheable/node-cache 3.1.2 (6M/month)
- @cacheable/utils 2.5.1 (34M/month)
- @cacheable/net 2.1.1 (3.7K/month)
- ecto 5.0.1 (4.5K/month)

We are also also seeing very active community spread of this supply chain worm to other maintainers and packages, including major organizations:

- @deliveroo/reevent 1.0.1
- @or-sdk/invitations 1.4.9
- @picsart/ai-sdk 3.32.2
- @qlik/embed-runtime 1.6.4
- picasso.js 2.11.6

Update — August 5, 2026, 13:15 CEST: At least 444 packages (across 1381 versions) have been compromised by the worm, with a combined total of over 2 billion monthly installs at the time of writing.

## What happened

Every package in the family received two new files, setup.mjs and Math_Symbol.js, along with a "preinstall": "node setup.mjs" entry added to each package.json. Anyone who ran npm install against an affected version would have had setup.mjs execute automatically before their install completed.

setup.mjs is a heavily obfuscated dropper. Its only job is to silently download the Bun JavaScript runtime from github[.]com/oven-sh/bun/releases/download/bun-v1.3.13/ and use it to execute the real payload, Math_Symbol.js:

```
execFileSync(<bun binary>, ['<script_dir>/Math_Symbol.js'], {
  stdio: 'inherit',
  cwd: <script_dir>
})
```

The Math_Symbol.js is a heavily obfuscated 728 KB JavaScript file containing credential stealers that harvest secrets from the victim's environment, encrypt the findings, and exfiltrate them to a public GitHub repository whose description reads "Shai-Hulud: Here We Go Again". The payload also contains worm-like propagation functionality to infect packages of other maintainers that have installed one of the compromised packages.

## What it steals

The Math_Symbol.js file implements a set of credential extractors, each targeting a different secret store on the victim machine.

npm tokens

Reads ~/.npmrc and scans the filesystem for any other .npmrc files. Extracts authToken values and any //registry.*:_authToken=... entries. Validates each token live against registry.npmjs[.]org/-/whoami before exfiltrating.

GitHub tokens

Three token formats are targeted: classic PATs (ghp_...) and OAuth tokens (gho_...), GitHub App server-to-server tokens (ghs_...), and JWT OIDC tokens. Sources include ~/.config/gh/hosts.yml, environment variables, and a filesystem scan.

On GitHub Actions runners, the payload also executes a shell command that reads the runner process memory directly to dump the entire secret store. It reads ACTIONS_ID_TOKEN_REQUEST_TOKEN and ACTIONS_ID_TOKEN_REQUEST_URL to steal OIDC tokens used for npm publishing.

AWS credentials

- ~/.aws/credentials and ~/.aws/config, parsing all named profiles
- AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, and AWS_SESSION_TOKEN environment variables
- EC2 Instance Metadata Service at 169.254.169.254, trying IMDSv2 first with a fallback to IMDSv1
- ECS container metadata endpoint at 169.254.170.2
- AWS Secrets Manager, calling secretsmanager:ListSecrets across multiple regions to enumerate and exfiltrate all secrets stored there

Kubernetes secrets

Reads the service account token, CA certificate, and namespace from /var/run/secrets/kubernetes.io/serviceaccount/. Uses the service account token to query the Kubernetes API directly and retrieve all secrets in the namespace. Also targets KUBECONFIG and ~/.kube/config.

HashiCorp Vault tokens

Checks six sources in priority order: the VAULT_TOKEN environment variable, ~/.vault-token, the GitHub Actions runner path /home/runner/.vault-token, several well-known container paths, a Kubernetes auth login using the stolen service account JWT, and Vault's AWS IAM auth endpoint using any stolen AWS credentials. After obtaining a token, it enumerates all KV stores via /v1/sys/mounts and reads every secret from KV v1 and v2 paths.

Stripe and Slack tokens

Scans for Stripe API keys (both test and live, sk_ and pk_ prefixes) and Slack tokens (xox[baprs]-...) across all files touched by the filesystem scanner.

Generic filesystem scan

A platform-aware scanner (macOS vs Linux) runs roughly 200 glob patterns across the filesystem, targeting among other things:

- .env, .env.*, and .envrc files
- Private key files (*.pem, *.key, *.p12, *.pfx, *.jks)
- SSH keys and config (id_rsa, id_ed25519, .ssh/config)
- Terraform state files and .tfvars
- Docker registry credential files (docker/config.json)
- KeePass databases (*.kdbx)
- VPN configs (*.ovpn)
- IDE config files including .vscode/tasks.json and .claude/settings.json

Files over 5 MB are skipped. Up to 64 concurrent reads are used. A generic regex engine is also applied across all scanned files, flagging PEM private keys, SSH public keys, Azure storage keys, database connection strings with embedded credentials, and generic key=value patterns matching common secret field names.

## Exfiltration

Once credentials are harvested, the payload encrypts the entire bundle before sending it anywhere. Only the attacker, who holds the corresponding RSA private key, can decrypt what gets uploaded. This means the stolen data sits in plain sight on public infrastructure but is unreadable to anyone else.

The primary exfiltration destination is a public GitHub repository whose description contains the string "Shai-Hulud: Here We Go Again". At the time of writing, GitHub contains roughly 1,300 public repositories matching that string, each serving as a drop point for a victim's encrypted credential bundle.

If the GitHub upload fails, the payload falls back to https://npm-cache[.]com:443/router, a domain registered on 2026-05-22 that appears to serve no legitimate purpose. This domain is fetched dynamically from an Ethereum smart contract at 0xE1f2395ee43e45A1556EC6438a88c31B83493103, allowing the attacker to rotate infrastructure at any time without touching the payload.

## Self-replicating worm

Beyond stealing credentials, the payload actively uses them to spread the malware to other maintainers and repositories. It has two distinct infection vectors.

npm tarball infection

Using the stolen npm token, the payload calls https://registry.npmjs[.]org/-/npm/v1/tokens to list every package that token has publish rights to, then fetches and unpacks the current tarball for each one. Before republishing, it makes the following modifications:

- Bumps the patch version by one (e.g. 1.2.3 becomes 1.2.4)
- Adds "preinstall": "node setup.mjs" to the package scripts
- Injects setup.mjs and math_init.js (functionally identical to Math_Symbol.js) into the package

It then repacks and publishes the modified tarball to the registry.

This is how the worm propagates beyond the original maintainer. After the initial compromise of the keyv maintainer, we observed over 400 packages being infected through community spread. These second-generation infections are identifiable by the use of math_init.js rather than Math_Symbol.js, since they were seeded by the npm tarball injection worm.

GitHub repo infection

When the payload finds a ghs_ token, it commits into every branch it can reach, up to 50 branches per repo, working through the most recently active branches first and skipping dependabot and copilot branches. It adds malicious hooks to .claude/settings.json and .vscode/tasks.json so that the payload executes automatically the next time any developer opens the repository in VS Code or starts a Claude Code session inside it, with no npm install required. The commits are authored as claude with the email claude@users.noreply.github[.]com and carry the message chore: update config, blending in with real commits.

## How Aikido detects this

If you are an Aikido user, check your central feed and filter on malware issues. This will surface as a 100/100 critical issue. Aikido rescans nightly, but we recommend triggering a manual rescan now.

If you are not yet an Aikido user, you can [create an account](https://app.aikido.dev/login) and connect your repos. Our malware coverage is included in the free plan, no credit card required.

For broader coverage across your whole team, Aikido's [Device Protection](https://www.aikido.dev/protect/device-protection) gives you visibility and control over the software packages installed on your team's devices. It covers browser extensions, code libraries, IDE plugins, and build dependencies, all in one place. Stop malware before it gets installed.

For future protection, consider [Aikido Safe Chain](https://github.com/AikidoSec/safe-chain) (open source). Safe Chain sits in your existing workflow, intercepting npm, npx, yarn, pnpm, and pnpx commands and checking packages against [Aikido Intel](https://intel.aikido.dev/) before install.

## Indicators of Compromise (IOCs)

Files

- setup.mjs‍

- SHA-256 54dc7ea54a1317cca0e890a2770630cf7fa6c97813e0cb9d2caa93012b350668

- setup.mjs (community-spread version)

- SHA-256 fd3ca4007b225fdf8de7af4345a19179d5efa8c4bb9205f88cda806e5684b1eb

- Math_Symbol.js and math_init.js‍

- SHA-256 9fc2570b7cef51c1b8df116d144d11ff4096357be7d2c4c6367cfc2509cf1bcc

Network

- https://npm-cache[.]com:443/router — exfiltration endpoint
- eth-mainnet.nodereal[.]io request containing 0xE1f2395ee43e45A1556EC6438a88c31B83493103

Other

- Any public GitHub repository with description containing "Shai-Hulud: Here We Go Again" — these are the attacker-controlled repos used for credential exfiltration.

‍

Share:

https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack#https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack#https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack#

https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack

[Text Link](https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack#)

### Scan for malware

[Start for Free](https://app.aikido.dev/)

4.7/5

Tired of false positives?
Try Aikido like 100k others.

[Start Now](https://app.aikido.dev/login)

Get a personalized walkthrough

Trusted by 100k+ teams

[Book Now](https://www.aikido.dev/book/demo)

Scan your app for IDORs and real attack paths

Trusted by 100k+ teams

[Start Scanning](https://app.aikido.dev/login)

See how AI pentests your app

Trusted by 100k+ teams

[Start Testing](https://app.aikido.dev/login)

[Start Now](https://app.aikido.dev/login)

[Similar Posts](https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack#)[See all](https://www.aikido.dev/blog)

November 2, 2026

•

Vulnerabilities & Threats

## Critical Linux kernel TCP vulnerability can allow remote code execution (CVE-2026-98432)

CVE-2026-98432 is a critical Linux kernel TCP flaw rated 9.8 that can allow remote code execution. See who is affected, how to patch and what to do now.

https://www.aikido.dev/blog/linux-kernel-tcp-rce-vulnerability-cve-2026-98432

October 23, 2026

•

Vulnerabilities & Threats

## Node2Shell evolves to HTTP2Shell. Brotli flaw (CVE-2026-111497) blast radius affects many libraries & apps using HTTP.

The Brotli flaw behind Node2Shell (CVE-2026-111497) affects many libraries and applications using HTTP with Brotli or brotlicffi installed. Check your versions and upgrade to the fix.

#

Vulnerabilities

#

Pypi

https://www.aikido.dev/blog/python-packages-vulnerable-node2shell-brotli-cve-2026-111497

October 22, 2026

•

Vulnerabilities & Threats

## Node2Shell: Node.js Brotli vulnerability allows remote code execution (CVE-2026-111497)

Node.js CVE-2026-111497 (Node2Shell) is a Brotli flaw that can allow remote code execution. See who is affected, patched versions and the fix.

#

Malware

https://www.aikido.dev/blog/node2shell-nodejs-brotli-rce-cve-2026-111497

## Get secure today,
quickly and for free.

Secure your code, cloud, and runtime in one central system.
Connect a repo to discover what the reasoning agents find in your codebase.

[Start for Free](https://www.aikido.dev/blog/keyv-and-friends-compromised-in-npm-supply-chain-attack#)

No credit card required | Scan results in 32 seconds.

Trusted by 150k+ orgs
