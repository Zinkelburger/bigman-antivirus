---
url: "https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/"
title: "Typosquatted npm packages used to steal cloud and CI/CD secrets | Microsoft Security Blog"
site: "Microsoft Security Blog"
published: "2026-05-29T03:04:52+00:00"
captured: "2026-11-02T18:57:51Z"
via: "live"
sha256: "cc5fa019ec556fa1f17ad6b5b576d7250a43a7c572e23f3b52fbe5b2f429af65"
wayback: "https://web.archive.org/web/20261022093944/https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Typosquatted npm packages used to steal cloud and CI/CD secrets | Microsoft Security Blog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/>
> Snapshot: <https://web.archive.org/web/20261022093944/https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/>

[Research](https://www.microsoft.com/en-us/security/blog/content-type/research/)    May 28    9 min read

#  Typosquatted npm packages used to steal cloud and CI/CD secrets

By [Microsoft Defender Security Research Team, view all posts by this author](https://www.microsoft.com/en-us/security/blog/author/windows-defender-research/)

/

## Share

-  [Link copied to clipboard!](https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/)
-  https://www.facebook.com/sharer/sharer.php?u=https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/
-  https://twitter.com/intent/tweet?url=https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/&text=Typosquatted+npm+packages+used+to+steal+cloud+and+CI%2FCD+secrets
-  https://www.linkedin.com/sharing/share-offsite/?url=https://www.microsoft.com/en-us/security/blog/2026/05/28/typosquatted-npm-packages-used-steal-cloud-ci-cd-secrets/

## Tags

- [AWS](https://www.microsoft.com/en-us/security/blog/tag/aws/)
- [Linux](https://www.microsoft.com/en-us/security/blog/tag/linux/)
- [npm](https://www.microsoft.com/en-us/security/blog/tag/npm/)

## Content types

- [Research](https://www.microsoft.com/en-us/security/blog/content-type/research/)

## Products and services

- [Microsoft Defender](https://www.microsoft.com/en-us/security/blog/product/microsoft-defender/)

## Topics

- [Actionable threat insights](https://www.microsoft.com/en-us/security/blog/topic/actionable-threat-insights/)
- [Defending against advanced tactics](https://www.microsoft.com/en-us/security/blog/topic/defending-against-advanced-tactics/)

Microsoft has identified an active supply chain attack targeting the npm package ecosystem. On May 28, 2026, a single threat actor operating under the newly created maintainer alias vpmdhaj (a39155771@gmail[.]com) published 14 malicious packages within a four-hour window. The packages typosquat well-known OpenSearch, ElasticSearch, DevOps, and environment-configuration libraries, and several spoof the upstream OpenSearch project’s repository URL in their package.json to appear legitimate. Once installed, the packages harvest AWS credentials, HashiCorp Vault tokens, and CI/CD pipeline secrets from the host environment.

All packages in the cluster ship the same install-time stager and the same Bun-compiled second-stage payload – a ~195 KB credential harvester purpose-built for cloud and CI/CD environments. The payload runs silently during npm install and targets credentials across Amazon Web Services, HashiCorp Vault, GitHub Actions, and the npm registry itself, enabling both cloud lateral movement and downstream supply-chain pivoting through stolen npm publish tokens. Based on our investigation and feedback to the npm team these repos and users were taken down.

Key capabilities observed in the campaign include automatic execution via npm lifecycle hooks, two distinct stager generations (an HTTP-C2 variant and a stealthier variant that abuses the legitimate Bun runtime distribution), AWS Instance Metadata Service (IMDSv2) and ECS task-role theft, AWS Secrets Manager enumeration across 16+ regions, HashiCorp Vault token harvesting, and theft of npm publish tokens for follow-on supply-chain attacks.

## Attack chain overview

The vpmdhaj cluster spans 14 scoped and unscoped packages that all mimic the @opensearch / @elastic ecosystem. The attack proceeds through:

- Publication of 14 typosquat packages under a single actor identity
- Automatic payload execution through a preinstall hook during npm install
- Execution chain (Gen-1): node -> preinstall.js -> HTTP C2 -> payload.bin (detached)
- Execution chain (Gen-2): node -> setup.mjs -> download legitimate Bun runtime -> run bundled stage-2
- Cloud credential theft (AWS IMDS, ECS metadata, Vault, Secrets Manager) and npm publish-token theft for downstream supply-chain pivot

Figure 1. vpmdhaj npm supply chain attack flow.

### The lure: typosquats and spoofed metadata

The actor adopted three social-engineering techniques designed to drive installs by mistake or trust transference. First, lookalike naming – names such as opensearch-setup, opensearch-setup-tool, opensearch-config-utility, elastic-opensearch-helper, search-engine-setup, and env-config-manager mimic well-known cluster-management and configuration libraries. Second, spoofed upstream metadata – every unscoped package sets its package.json homepage, repository, and bugs fields to the legitimate github.com/opensearch-project/opensearch-js project. Third, inflated version numbers – releases jump straight to 1.0.7265, 1.0.9108, or 2.1.9201 to suggest a long, mature release history.

Figure 2. npm.js package page for @vpmdhaj/elastic-helper showing the inflated 1.0.7269 version and the spoofed OpenSearch repository link.

### Execution: npm lifecycle hook abuse

Every package in the cluster declares an automatic install-time hook in package.json. The malicious code executes the moment a victim runs npm install – no require() from victim code is needed. Two stager variants were observed:

- Gen-1 (versions <= 1.0.7265): install, preinstall, and postinstall hooks all invoke preinstall.js / index.js
- Gen-2 (versions >= 1.0.7266): a single preinstall hook invokes setup.mjs (newer, stealthier loader)

Figure 3. The malicious package.json. A single preinstall hook is enough to gain code execution on every npm install.

### Gen-1 stager: HTTP C2 beacon and payload drop

preinstall.js collects rich host context – hostname, platform, arch, Node version, USER/USERNAME, cwd, INIT_CWD, npm_package_name, npm_package_version – base64-encodes the JSON, and POSTs it to the actor’s C2 with a campaign-unique header X-Supply: 1. The same C2 endpoint then serves a gunzip-compressed second-stage binary, which is written to payload.bin in the package install directory, chmod 0755’d, and spawned detached.

Figure 4. Stage-1 C2 beacon. The X-Supply: 1 header is a high-confidence detection signal in proxy logs.

Figure 5. Stage-2 download, decompression, +x, and detached spawn. __DAEMONIZED=1 lets the payload distinguish itself from npm.

The package’s index.js re-launches the same payload.bin on every subsequent require() of the module – a quiet persistence mechanism that survives across CI build stages and developer rebuild loops. The module also exports a benign-looking object falsely identifying itself as @opensearch/setup.

Figure 6. Persistence shim. The malicious module exports benign-looking metadata and silently re-spawns the payload every time it is require()’d.

### Gen-2 stager: abusing the legitimate Bun runtime as a loader

In newer versions, the actor replaced the noisy HTTP-C2 design with a stealthier loader that eliminates the install-time C2 round-trip entirely. setup.mjs (a) checks whether bun is already present on the host; (b) if not, downloads the legitimate Bun runtime v1.3.13 from github.com/oven-sh/bun/releases for the correct platform/arch (Linux x64/musl/aarch64, macOS x64/arm64, Windows x64/arm64); (c) extracts the ZIP using unzip, PowerShell Expand-Archive, or a hand-rolled ZIP parser; and (d) executes the pre-bundled second-stage payload (opensearch_init.js or ai_init.js) that ships inside the npm tarball.

This design reduces visibility for defenders that primarily monitor unusual outbound traffic during package installation.

Figure 7. Gen-2 loader. The actor abuses a legitimate GitHub Release of the Bun runtime to execute a pre-bundled payload that ships inside the npm tarball.

### Credential theft

The second-stage binary is a single-file Bun-compiled JavaScript binary of approximately 195 KB, purpose-built for cloud and CI/CD secret theft. Static review of the bundle identifies routines that target secrets across five platforms:

- AWS: queries EC2 Instance Metadata Service v2 (169.254.169[.]254), Elastic Container Service task metadata (169.254.170[.]2), reads AWS env credentials, calls STS GetCallerIdentity / AssumeRole, and enumerates Secrets Manager (ListSecrets / GetSecretValue) across 16+ regions with a bundled SigV4 signer.
- HashiCorp Vault: reads VAULT_TOKEN and VAULT_AUTH_TOKEN environment variables.
- npm: validates tokens through /-/whoami and enumerates publish access through /-/npm/v1/tokens.
- GitHub Actions: collects GITHUB_REPOSITORY and RUNNER_OS context to identify build environments for prioritized exploitation.
- CI/CD environment: respects __DAEMONIZED=1 to avoid re-entry, and explicitly resets CI=false to mislead build-aware code paths.

Figure 8. String evidence from the Bun-compiled stage-2 payload. The same binary is dropped by both Gen-1 and Gen-2 stagers.

### Impact and blast radius

- Stolen AWS STS sessions and Secrets Manager material enable cloud lateral movement and data theft.
- Stolen GitHub Actions tokens enable repo manipulation and CI/CD pipeline tampering.
- Stolen npm publish tokens enable downstream supply-chain pivoting – pushing malicious updates to packages owned by hijacked maintainer identities, expanding the campaign beyond the initial 14 packages.
- All 14 packages target the OpenSearch / ElasticSearch ecosystem keywords, suggesting the actor likely chose a developer audience to have AWS and Elastic cloud credentials in their environments.

## Mitigation and protection guidance

Microsoft recommends the following mitigations to reduce the impact of this threat:

- Identify systems that installed or built affected package versions on or after May 28, 2026.
- Pin known-good package versions where possible and avoid automatic dependency upgrades until validation is complete.
- Disable pre- and post-installation script execution by running npm install with –ignore-scripts (or setting npm config set ignore-scripts true globally). Apply equivalent settings for pnpm and yarn.
- Rotate AWS IAM/STS, HashiCorp Vault, npm publish, and GitHub Actions tokens that may have been exposed to affected runners or developer workstations.
- Block egress to aab.sportsontheweb[.]net at proxy, firewall, and DNS layers. Alert on any HTTP request carrying the header X-Supply: 1.
- Hunt CloudTrail for anomalous sts:GetCallerIdentity rapidly followed by sts:AssumeRole, and for secretsmanager:ListSecrets or GetSecretValue in cross-region succession from build infrastructure or developer IP space.
- Audit CI/CD logs for unexpected outbound network connections, Bun runtime downloads from GitHub Releases by Node.js processes, and detached child processes spawned with __DAEMONIZED=1.
- Review npm package lockfiles (package-lock.json, yarn.lock, pnpm-lock.yaml), build logs, and artifact provenance for evidence of compromised package versions.
- Enable cloud-delivered protection in Microsoft Defender Antivirus or equivalent antivirus protection.
- Use Microsoft Defender XDR to investigate suspicious activity across endpoints, identities, cloud apps, and developer environments.
- Use Microsoft Defender Vulnerability Management to search for the affected packages across your estate.

### How Microsoft Defender helps

Microsoft Defender Antivirus detects and blocks the malicious components on access. During reproduction in our analysis environment, setup.mjs was automatically quarantined the moment the tarball was extracted to disk.

Figure 9. Microsoft Defender auto-quarantine of setup.mjs at extract time.

### Microsoft Defender XDR Detections

Microsoft Defender XDR customers can refer to the list of applicable detections below. Microsoft Defender XDR coordinates detection, prevention, investigation, and response across endpoints, identities, email, and apps to provide integrated protection against attacks like the threat discussed in this blog.

| Tactic | Observed activity | Microsoft Defender coverage

| Initial Access / Execution | Suspicious script execution during npm install or package lifecycle activity | Microsoft Defender Antivirus
-Trojan:JS/ShaiWorm
-Trojan:JS/ObfusNpmJs
-Backdoor:JS/SupplyChain

Microsoft Defender for Endpoint
– Suspicious usage of Bun runtime
– Suspicious installation of Bun runtime
– Suspicious Node.js process behavior

Microsoft Defender XDR
– Suspicious file creation in temporary directory by node.exe
– Suspicious Bun execution from Node.js process

| Credential Access | Potential harvesting of AWS, Vault, GitHub Actions, and npm tokens from CI/CD runners | Microsoft Defender for Endpoint
– Credential access attempt
– Suspicious cloud credential access by npm-cached binary
– AWS Instance Metadata Service access from suspicious process

Microsoft Defender for Cloud
– Possible IMDS abuse from container workload
– Anomalous Secrets Manager enumeration across regions

| Command and Control | Outbound HTTP beacon with X-Supply: 1 header to attacker-controlled C2 | Microsoft Defender for Endpoint
– Connection to a custom network indicator (aab.sportsontheweb[.]net)
– Suspicious outbound HTTP from npm install context

| Persistence | Re-spawn of payload.bin on every require() of compromised package | Microsoft Defender for Endpoint
– Detached child process spawned by node.exe with __DAEMONIZED=1

### Advanced hunting

The following sample queries let you search for a week’s worth of events. To explore up to 30 days of raw data, go to the Advanced Hunting page > Query tab, and update the time range to Last 30 days.

Hunt for suspicious npm lifecycle script execution involving vpmdhaj packages.

```

DeviceProcessEvents
| where Timestamp > ago(7d)
| where FileName in~ ("node.exe", "node", "npm.cmd", "npm.exe", "npx.cmd", "npx.exe")
| where ProcessCommandLine has_any ("preinstall", "postinstall", "install")
| where ProcessCommandLine has_any (
    "@vpmdhaj", "opensearch-setup", "opensearch-setup-tool",
    "opensearch-config-utility", "opensearch-security-scanner",
    "search-engine-setup", "search-cluster-setup",
    "elastic-opensearch-helper", "vpmdhaj-opensearch-setup",
    "env-config-manager", "app-config-utility")
| project Timestamp, DeviceName, FileName, ProcessCommandLine,
          InitiatingProcessFileName, InitiatingProcessCommandLine, AccountName

```

Hunt for the stage-2 payload artifact on disk.

```

DeviceFileEvents
| where Timestamp > ago(7d)
| where FileName =~ "payload.bin"
| where FolderPath has "node_modules"
| project Timestamp, DeviceName, FolderPath, FileName,
          InitiatingProcessFileName, InitiatingProcessCommandLine, AccountName

```

Hunt for detached payload execution with the campaign environment marker.

```

DeviceProcessEvents
| where Timestamp > ago(7d)
| where ProcessCommandLine has "__DAEMONIZED=1"
   or InitiatingProcessCommandLine has "__DAEMONIZED=1"
| project Timestamp, DeviceName, FileName, ProcessCommandLine,
          InitiatingProcessFileName, InitiatingProcessCommandLine

```

Hunt for Gen-2 loader: Bun runtime download from GitHub Releases by Node.js.

```

DeviceNetworkEvents
| where Timestamp > ago(7d)
| where InitiatingProcessFileName in~ ("node.exe", "node")
| where RemoteUrl has "github.com/oven-sh/bun/releases/download"
| project Timestamp, DeviceName, RemoteUrl, RemoteIP,
          InitiatingProcessFileName, InitiatingProcessCommandLine, AccountName

```

Hunt for C2 beacon to attacker infrastructure.

```

DeviceNetworkEvents
| where Timestamp > ago(30d)
| where RemoteUrl has "aab.sportsontheweb.net"
   or RemoteUrl has "sportsontheweb.net"
| project Timestamp, DeviceName, RemoteUrl, RemoteIP,
          InitiatingProcessFileName, InitiatingProcessCommandLine, AccountName

```

Hunt for AWS IMDS / ECS metadata access from Node.js processes.

```

DeviceNetworkEvents
| where Timestamp > ago(7d)
| where InitiatingProcessFileName in~ ("node.exe", "node", "bun.exe", "bun")
| where RemoteIP in ("169.254.169.254", "169.254.170.2")
| project Timestamp, DeviceName, RemoteIP, RemoteUrl,
          InitiatingProcessFileName, InitiatingProcessCommandLine, AccountName

```

### Indicators of Compromise (IOC)

Affected npm packages – all published by maintainer vpmdhaj on 2026-05-28:

| Indicator | Type | Description

| @vpmdhaj/elastic-helper (1.0.7269) | Package | Typosquat – ElasticSearch/OpenSearch helper

| @vpmdhaj/devops-tools (1.0.7267) | Package | Typosquat – DevOps tools / OpenSearch setup

| @vpmdhaj/opensearch-setup (1.0.7267) | Package | Typosquat – OpenSearch setup utility

| @vpmdhaj/search-setup (1.0.7268) | Package | Typosquat – search engine setup

| opensearch-security-scanner (1.0.10) | Package | Unscoped lookalike – security scanner

| opensearch-setup (1.0.9103) | Package | Unscoped lookalike – spoofs opensearch-project repo URL

| opensearch-setup-tool (1.0.9108) | Package | Unscoped lookalike – spoofs opensearch-project repo URL

| opensearch-config-utility (1.0.9106) | Package | Unscoped lookalike – spoofs opensearch-project repo URL

| search-engine-setup (1.0.9108) | Package | Unscoped lookalike – spoofs opensearch-project repo URL

| search-cluster-setup (1.0.9104) | Package | Unscoped lookalike – spoofs opensearch-project repo URL

| elastic-opensearch-helper (1.0.9108) | Package | Unscoped lookalike – spoofs opensearch-project repo URL

| vpmdhaj-opensearch-setup (1.0.9102) | Package | Unscoped – author-named OpenSearch setup

| env-config-manager (2.1.9201) | Package | Typosquat – dotenv-style config manager

| app-config-utility (1.0.9300) | Package | Typosquat – generic app config utility

#### Actor, network, and file IOCs

| Indicator | Type | Description

| vpmdhaj | npm maintainer alias | Threat actor publishing all 14 packages

| a39155771@gmail.com | Email | Maintainer contact email registered on npm

| aab.sportsontheweb[.]net | Domain | Stage-1 C2 (Gen-1 packages)

| hxxp://aab.sportsontheweb[.]net/x.php | URL | Beacon + stage-2 payload endpoint (port 80)

| X-Supply: 1 | HTTP header | Campaign-unique marker – high-confidence proxy detection

| 169.254.169.254 | IP | AWS EC2 IMDSv2 endpoint queried by stage-2

| 169.254.170.2 | IP | AWS ECS task metadata endpoint queried by stage-2

| 638788AFC4F1B5860A328312CAF5895ABD5F5632D28A4F2A85B09076E270D15D | SHA-256 | preinstall.js (Gen-1 stager)

| 77D92EFE7AF3547F71FD41D4A884872D66B1BE9499EAA637E91EAC866911694D | SHA-256 | setup.mjs (Gen-2 stager)

| BFA149694EC6411C23936311A999163ADE54D6F38E2F4B0E3CFB8CB67BD7CFAA | SHA-256 | payload.gz (gzipped Bun stage-2)

| opensearch_init.js | Filename | Bun-compiled stage-2 credential harvester (~195 KB)

| ai_init.js | Filename | Alternate stage-2 filename used by some Gen-2 packages

| payload.bin | Filename | Dropped stage-2 binary in node_modules install dir

| __DAEMONIZED=1 | Env var | Marker set by stager when spawning detached payload

## References

- https://www.npmjs.com/~vpmdhaj – npm maintainer profile (all 14 packages)
- https://www.npmjs.com/package/@vpmdhaj/elastic-helper
- https://www.npmjs.com/package/@vpmdhaj/devops-tools
- https://docs.npmjs.com/cli/v10/using-npm/scripts – npm lifecycle scripts documentation
- https://bun.sh – Bun runtime (abused by Gen-2 stager as a loader)
- https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-use-IMDSv2.html – IMDSv2 hardening guidance

This research is provided by Microsoft Defender Security Research with contributions from members of Microsoft Threat Intelligence.

## Learn more

For the latest security research from the Microsoft Threat Intelligence community, check out the [Microsoft Threat Intelligence Blog](https://aka.ms/threatintelblog).

To get notified about new publications and to join discussions on social media, follow us on [LinkedIn](https://www.linkedin.com/showcase/microsoft-threat-intelligence), [X (formerly Twitter)](https://x.com/MsftSecIntel), and [Bluesky](https://bsky.app/profile/threatintel.microsoft.com).

To hear stories and insights from the Microsoft Threat Intelligence community about the ever-evolving threat landscape, listen to the [Microsoft Threat Intelligence podcast](https://thecyberwire.com/podcasts/microsoft-threat-intelligence).

Review our documentation to learn more about our real-time protection capabilities and see how to enable them within your organization.

- [Microsoft 365 Copilot AI security documentation](https://learn.microsoft.com/en-us/copilot/microsoft-365/microsoft-365-copilot-ai-security)
- [How Microsoft discovers and mitigates evolving attacks against AI guardrails](https://www.microsoft.com/en-us/security/blog/2024/04/11/how-microsoft-discovers-and-mitigates-evolving-attacks-against-ai-guardrails/)
- Learn more about [securing Copilot Studio agents with Microsoft Defender](https://learn.microsoft.com/en-us/defender-cloud-apps/ai-agent-protection)
- Evaluate your AI readiness with our latest [Zero Trust for AI workshop](https://microsoft.github.io/zerotrustassessment/).
- Learn more about [Protect your agents in real-time during runtime (Preview)](https://learn.microsoft.com/en-us/defender-cloud-apps/real-time-agent-protection-during-runtime)
- Explore [how to build and customize agents with Copilot Studio Agent Builder](https://eurppc-word-edit.officeapps.live.com/we/%E2%80%A2%09https:/learn.microsoft.com/en-us/microsoft-365-copilot/extensibility/copilot-studio-agent-builder)

##  Microsoft Defender Security Research Team

[See Microsoft Defender Security Research Team posts](https://www.microsoft.com/en-us/security/blog/author/windows-defender-research/)

## Related posts

-

October 30    11 min read

###  [Inside a malicious Claude Code-branded extension and its persistent SSH backdoor](https://www.microsoft.com/en-us/security/blog/2026/10/30/inside-malicious-claude-code-branded-cursor-extension-persistent-ssh-backdoor/)

Microsoft analyzes a malicious Claude Code–branded extension, its persistent SSH backdoor, and guidance to detect and remediate it.

-

October 30    16 min read

###  [From help desk impersonation to AD CS abuse: Investigating ESC1 and ESC8 intrusion paths](https://www.microsoft.com/en-us/security/blog/2026/10/30/help-desk-impersonation-ad-cs-abuse-investigating-esc1-esc8-intrusion-paths/)

Teams support impersonation led to remote access, malicious runtime execution, and Certificate Services abuse through authentication relay and Kerberos.

-

October 29    14 min read

###  [Tracking AI adoption in North Korean threat actor tradecraft](https://www.microsoft.com/en-us/security/blog/2026/10/29/tracking-ai-adoption-in-north-korean-threat-actor-tradecraft/)

As Microsoft Threat Intelligence continues to track how threat actors operationalize AI to increase the speed and efficiency of attacks, we’ve identified North Korean threat actors expanding their use of AI with varying levels of adoption.
