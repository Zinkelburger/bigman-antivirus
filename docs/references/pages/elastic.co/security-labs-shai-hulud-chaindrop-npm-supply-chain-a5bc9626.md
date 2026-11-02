---
url: "https://www.elastic.co/security-labs/shai-hulud-chaindrop-npm-supply-chain"
title: "Shai-Hulud strikes again: CHAINDROP worm hits 400+ npm packages"
captured: "2026-11-02T18:54:52Z"
via: "live"
source_url: "https://www.elastic.co/security-labs/threat-command/shai-hulud-chaindrop-npm-supply-chain"
sha256: "588d9bde03588720615a19347c275d6d24baf17ad7d7c6b751eac2fa24790ef3"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Shai-Hulud strikes again: CHAINDROP worm hits 400+ npm packages

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.elastic.co/security-labs/shai-hulud-chaindrop-npm-supply-chain>

[Threat Command](https://www.elastic.co/security-labs/threat-command)

# Shai-Hulud strikes again: CHAINDROP worm hits 400+ npm packages

Elastic Security Labs identified the return of Shai-Hulud. Attackers compromised the keyv maintainer and deployed CHAINDROP, a worm that uses stolen npm credentials to backdoor co-owned packages totaling over 1.3 billion monthly downloads.

August 06, 2026

[Elastic Security Labs](https://www.elastic.co/security-labs/author/elastic-security-labs)

[Threat Intelligence](https://www.elastic.co/security-labs/threat-command/category/threat-intelligence)

Jump to

Share

https://x.com/intent/post?text=Shai-Hulud%20strikes%20again%3A%20CHAINDROP%20worm%20hits%20400%2B%20npm%20packages&url=https%3A%2F%2Fwww.elastic.co%2Fsecurity-labs%2Fthreat-command%2Fshai-hulud-chaindrop-npm-supply-chainhttps://www.linkedin.com/shareArticle?mini=true&url=https%3A%2F%2Fwww.elastic.co%2Fsecurity-labs%2Fthreat-command%2Fshai-hulud-chaindrop-npm-supply-chain&title=Shai-Hulud%20strikes%20again%3A%20CHAINDROP%20worm%20hits%20400%2B%20npm%20packageshttps://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fwww.elastic.co%2Fsecurity-labs%2Fthreat-command%2Fshai-hulud-chaindrop-npm-supply-chain

On August 4, 2026, Elastic Security Labs identified a new Shai-Hulud campaign targeting the maintainer of keyv, a widely used key-value storage library. The attackers trojanized the monorepo and embedded a self-propagating worm called CHAINDROP that uses stolen npm credentials to automatically backdoor every other package the maintainer had publish rights to. The reach of this compromise is significant: keyv alone received over 600 million downloads last month, with related packages compounding the exposure: flat-cache near 580 million, cacheable-request at over 137 million, cacheable at over 30 million, and cache-manager at over 16 million monthly downloads.

Our team was alerted via Slack notification around 5:39 AM EST. We received a hit from a previously released supply-chain monitoring [project](https://github.com/elastic/supply-chain-monitor) used to detect malicious npm packages. At the time of this writing, over 400 unique npm packages have been compromised.

Supply Chain Monitor initial notifications

## How the CHAINDROP worm works

Execution is triggered via a preinstall hook in package.json. This abuses a legitimate npm feature that will run arbitrary commands before a package is installed, requiring no further interaction from the victim after the installation.

Preinstall hook in package.json

Every subpackage in the keyv repo was backdoored with a dropper (setup.mjs) that delivers the same underlying payload. The malware is cross-platform, targeting Linux, macOS, and Windows platforms, giving it broad impact.

The payload appears under two filenames: Math_Symbol.js in packages compromised directly from the keyv monorepo, and math_init.js in packages trojanized during worm propagation to other maintainers. Both files share the same SHA-256 hash, making the filename a reliable indicator of infection generation. Worm-generated commits can also be identified by the author name claude and the commit message chore: update config.

Trojanized files

There are two other execution paths that can be triggered by users using Claude Code or VS Code. The threat actor has placed a SessionStart hook in settings.json under the Claude directory that will run node .claude/setup.mjs if a new Claude session is started. For VS Code, a folderOpen task is used under tasks.json that will run node .vscode/setup.mjs when an infected repository is opened. When a GitHub App token is among the stolen credentials, the worm extends this further by committing malicious hooks to up to 50 branches per accessible repository, injecting .claude/settings.json and .vscode/tasks.json into each. This means a developer can still be infected by just opening the repository.

The dropper (setup.mjs) checks if the JavaScript runtime (bun) is already installed. If not, the dropper will detect the platform/architecture of the machine and download bun v1.3.13 directly from the official release [page](https://github.com/oven-sh/bun/releases#release-bun-v1.3.13/). It will extract bun, then use it to execute the payload (Math_Symbol.js). After execution, the bun temporary directory is deleted to cover its tracks.

The payload is heavily obfuscated at 711 kilobytes, employing control-flow flattening with a string encoding scheme using Base91. The payload contains strings with Dune-themed references and has similarities to previous Shai-Hulud [campaigns](https://www.elastic.co/blog/shai-hulud-worm-npm-supply-chain-compromise).

```

```
laza, kanly, ghola, mentat, lasgun, sietch, fedaykin, tleilaxu, sandworm, sardaukar, ornithopter, navigator
```

```

The payload has a component called collector that functions as a credential harvester, scanning over 300 unique patterns across many credential stores found on a developer machine. The more notable targeting is aimed at AI tooling credentials such as Anthropic, Claude, Codex, Cursor, OpenAI, and Gemini. The malware also targets cloud provider credentials from AWS, GCP, Azure, and Alibaba Cloud. Many other credentials are targeted, such as GitHub (PATs, JWT, session tokens), HashiCorp Vault tokens, SSH private keys, Kubernetes service account tokens, and npm tokens.

AI tooling targeted by credential stealer

After the credentials are stolen, they are gzip-compressed and then encrypted with a randomly generated AES-256-GCM key. That AES key is then RSA-encrypted with the attacker's hardcoded public key. Only the attacker who holds the corresponding private key can decrypt the data.

CHAINDROP does not hardcode a C2 domain; instead, it queries an Ethereum smart contract at address 0xE1f2395ee43e45A1556EC6438a88c31B83493103 to retrieve the current exfiltration endpoint at runtime. It uses multiple RPC providers as fallbacks. The practical effect is that the attacker can rotate C2 infrastructure by updating the smart contract without touching the payload. At the time of our detonation runs, we observed the following dead-drop domains (awqhnjewqjkl[.]icu, npm-cache[.]com). If the contract discovery fails, CHAINDROP searches GitHub commit history for a cryptographically signed marker (thebeautifulmarchoftime) and accepts a C2 domain only after validating it against an embedded RSA public key.

If these two previous paths fail, CHAINDROP will exfiltrate the data via a public GitHub repository based on Dune names using the compromised victim’s account with a description: Shai-Hulud: Here We Go Again.

GitHub repo propagation with Shai-Hulud: Here We Go Again

The worm component activates when the credential sweep turns up an npm token, but only if that token meets two specific requirements: package write permissions and the ability to publish without two-factor authentication (bypass_2fa). Once a qualifying token is found, the worm enumerates every package the victim has publish rights to. For each writable package, it performs the following steps:

- Downloads the latest published tarball from the npm registry
- Extracts it to a temporary directory
- Injects the malicious components (math_init.js/Math_Symbol.js, setup.mjs)
- Patches the package.json to add the preinstall hook and bumps the patch version
- Re-packages and publishes back to the npm registry

In this campaign, the commit message contains the stolen GitHub token alongside the string IfYouBlockThisAPIKeyItWillCrashTheLiveProductionServersOfAllThirdPartyClients. The wording is an attempt to intimidate developers not to revoke the GitHub token.

Commit intimidation string

## Elastic Defend’s protection and detection coverage

Elastic Defend has coverage for this supply-chain compromise through the following rules:

### Trojanized npm package install running its dropper script

- [Node.js Pre or Post-Install Script Execution](https://github.com/elastic/detection-rules/blob/main/rules/cross-platform/execution_nodejs_pre_or_post_install_script_execution.toml)
- [Elastic Defend Alert from Package Manager Install Ancestry](https://github.com/elastic/detection-rules/blob/main/rules/cross-platform/initial_access_elastic_defend_alert_package_manager_ancestor.toml)

### Node.js dropper shelling out to download and extract the bun runtime

- [Suspicious Windows Powershell Arguments](https://github.com/elastic/detection-rules/blob/main/rules/windows/execution_windows_powershell_susp_args.toml)
- [Suspicious Curl Execution via NodeJS](https://github.com/elastic/protections-artifacts/blob/main/behavior/rules/cross-platform/command_and_control_suspicious_curl_execution_via_nodejs.toml)
- [Curl or Wget Spawned via Node.js](https://github.com/elastic/detection-rules/blob/main/rules/cross-platform/command_and_control_curl_wget_spawn_via_nodejs_parent.toml)

### Bun executing the obfuscated JavaScript payload and reaching out to infrastructure

- [Uncommon DNS Request via Bun or Node.js](https://github.com/elastic/detection-rules/blob/main/rules/cross-platform/command_and_control_uncommon_dns_request_via_bun_or_nodejs.toml)
- [DNS Request to Crypto/DHT Services](https://github.com/elastic/protections-artifacts/blob/main/behavior/rules/linux/command_and_control_dns_request_to_crypto_dht_services.toml)
- [Suspicious Instance Metadata Service (IMDS) API Request](https://github.com/elastic/detection-rules/blob/main/rules/cross-platform/credential_access_suspicious_instance_metadata_service_api_request.toml)

### Staged script execution followed by deletion of the temporary directory

- [Node Script Execution and Immediate Deletion](https://github.com/elastic/protections-artifacts/blob/main/behavior/rules/cross-platform/defense_evasion_node_script_execution_and_immediate_deletion.toml)

### Enumeration/discovery

- [System Owner/User Discovery Linux](https://github.com/elastic/detection-rules/blob/main/rules_building_block/discovery_linux_system_owner_user_discovery.toml)

Elastic Defend - Uncommon DNS Request via Bun alert

### Hunting queries

The following hunting queries can be used to find this associated activity:

```

```
FROM logs-endpoint.events.process-*
| WHERE process.name IN ("node", "node.exe")
| WHERE process.command_line IN ("node setup.mjs", "node  setup.mjs")
```

```

```

```
FROM logs-endpoint.events.process-*
| WHERE process.name IN ("bun", "bun.exe")
| WHERE process.command_line LIKE "*Math_Symbol.js*" OR process.command_line LIKE "*math_init.js*"
```

```

```

```
FROM logs-endpoint.events.network-*
| WHERE process.name IN ("bun", "bun.exe")
| WHERE dns.question.name IN ("go.getblock.io", "eth.llamarpc.com", "npm-cache.com", "eth-mainnet.nodereal.io", "awqhnjewqjkl.icu"
)
```

```

## Recommendations: How to respond to the Shai-Hulud compromise

- Don’t pull package updates immediately; add a soak period where new versions sit before your builds adopt them. This gives the community time to catch compromises before they reach your pipelines and developer machines (previously mentioned [Navigating the Shai-Hulud Worm 2.0: Elastic's updated response to npm supply chain compromise](https://www.elastic.co/blog/shai-hulud-worm-2-0-updated-response) and [How we caught the Axios supply chain attack](https://www.elastic.co/security-labs/how-we-caught-the-axios-supply-chain-attack))
- Revoke all GitHub tokens (PATs, session tokens) for any impacted machines
- Revoke and regenerate npm tokens found on affected machines, especially automation tokens with write permission and bypass_2fa
- Check your GitHub repositories for unauthorized commits with the message chore: update config from claude@users.noreply.github.com
- Rotate any impacted credentials from cloud services, Kubernetes configs, Vault tokens, AI tooling, CI/CD secrets
- Upgrade to npm 12 or later, which blocks preinstall hooks by default
- Enable 2FA on all npm accounts and avoid automation tokens with bypass_2fa: true where possible

## Indicators of Compromise

The following observables were discussed in this research.

| Observable | Type | Name | Reference

| 9fc2570b7cef51c1b8df116d144d11ff4096357be7d2c4c6367cfc2509cf1bcc | SHA-256 | Math_Symbol.js |

| fd3ca4007b225fdf8de7af4345a19179d5efa8c4bb9205f88cda806e5684b1eb | SHA-256 | setup.mjs |

| 54dc7ea54a1317cca0e890a2770630cf7fa6c97813e0cb9d2caa93012b350668 | SHA-256 | setup.mjs |

| npm-cache[.]com | domain |  |

| 0xE1f2395ee43e45A1556EC6438a88c31B83493103 | Ethereum contract |  | C2 resolver

| npm-cache[.]com | domain-name |  | C2 server

| awqhnjewqjkl[.]icu | domain-name |  | C2 server

[Report an issue](https://discuss.elastic.co/c/elastic-community-ecosystem/elastic-labs/101)

### Related Content

[How CVE-2026-73570 turns a Zimbra log file into remote code execution](https://www.elastic.co/security-labs/threat-command/command-injection-zimbra-cve-2026-73570)

[Detection Engineering](https://www.elastic.co/security-labs/threat-command/category/detection-engineering)[Threat Intelligence](https://www.elastic.co/security-labs/threat-command/category/threat-intelligence)

### How CVE-2026-73570 turns a Zimbra log file into remote code execution

October 23, 2026[Eric Forte](https://www.elastic.co/security-labs/author/eric-forte)

[php2shell: Assessing Elastic detection coverage relevant to CVE-2026-109618 and CVE-2026-109619](https://www.elastic.co/security-labs/threat-command/php2shell-php-rce-detection)

[Detection Engineering](https://www.elastic.co/security-labs/threat-command/category/detection-engineering)[Threat Intelligence](https://www.elastic.co/security-labs/threat-command/category/threat-intelligence)

### php2shell: Assessing Elastic detection coverage relevant to CVE-2026-109618 and CVE-2026-109619

October 21, 2026[Ruben Groenewoud](https://www.elastic.co/security-labs/author/ruben-groenewoud)[+1](https://www.elastic.co/security-labs/threat-command/shai-hulud-chaindrop-npm-supply-chain#)

[Auditing the auditor: Unmasking Apex Accelerator](https://www.elastic.co/security-labs/threat-command/apex-accelerator-malicious-npm-packages)

[Malware Analysis](https://www.elastic.co/security-labs/threat-command/category/malware-analysis)[Threat Intelligence](https://www.elastic.co/security-labs/threat-command/category/threat-intelligence)[AI Security](https://www.elastic.co/security-labs/threat-command/category/ai-security)

### Auditing the auditor: Unmasking Apex Accelerator

October 12, 2026[Jia Yu Chan](https://www.elastic.co/security-labs/author/jia-yu-chan)[+1](https://www.elastic.co/security-labs/threat-command/shai-hulud-chaindrop-npm-supply-chain#)

[The extension you never installed: KREMLIN forges Chrome's own integrity checks to steal banking sessions](https://www.elastic.co/security-labs/threat-command/malicious-browser-extension-kremlin-banking-malware)

[Malware Analysis](https://www.elastic.co/security-labs/threat-command/category/malware-analysis)[Threat Intelligence](https://www.elastic.co/security-labs/threat-command/category/threat-intelligence)

### The extension you never installed: KREMLIN forges Chrome's own integrity checks to steal banking sessions

September 14, 2026[Cyril François](https://www.elastic.co/security-labs/author/cyril-francois)[+1](https://www.elastic.co/security-labs/threat-command/shai-hulud-chaindrop-npm-supply-chain#)

[REVSTEALER ramps up: analysis of up-and-coming infostealer](https://www.elastic.co/security-labs/threat-command/revstealer-credential-harvesting-infostealer)

[Detection Engineering](https://www.elastic.co/security-labs/threat-command/category/detection-engineering)[Malware Analysis](https://www.elastic.co/security-labs/threat-command/category/malware-analysis)[Threat Intelligence](https://www.elastic.co/security-labs/threat-command/category/threat-intelligence)

### REVSTEALER ramps up: analysis of up-and-coming infostealer

September 02, 2026[Daniel Stepanic](https://www.elastic.co/security-labs/author/daniel-stepanic)[+2](https://www.elastic.co/security-labs/threat-command/shai-hulud-chaindrop-npm-supply-chain#)

### Elastic Security Labs Newsletter

[Follow Elastic Security Labs on](https://x.com/elasticseclabs)
