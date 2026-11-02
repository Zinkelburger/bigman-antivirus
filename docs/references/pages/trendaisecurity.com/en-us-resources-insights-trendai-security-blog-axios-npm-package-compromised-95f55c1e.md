---
url: "https://www.trendaisecurity.com/en-us/resources-insights/trendai-security-blog/axios-npm-package-compromised"
title: "Axios NPM Package Compromised: Supply Chain Attack Hits JavaScript HTTP Client with 100M+ Weekly Downloads | TrendAI (US)"
site: "TrendAI"
captured: "2026-11-02T19:00:39Z"
via: "live"
sha256: "f2cc385fee104c1cdcb4218f00914a36d6f4cd8a23fd2d89fc409d312681a64b"
wayback: "https://web.archive.org/web/20261025121422/https://www.trendaisecurity.com/en-us/resources-insights/trendai-security-blog/axios-npm-package-compromised"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Axios NPM Package Compromised: Supply Chain Attack Hits JavaScript HTTP Client with 100M+ Weekly Downloads | TrendAI (US)

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.trendaisecurity.com/en-us/resources-insights/trendai-security-blog/axios-npm-package-compromised>
> Snapshot: <https://web.archive.org/web/20261025121422/https://www.trendaisecurity.com/en-us/resources-insights/trendai-security-blog/axios-npm-package-compromised>

[Return to TrendAI™ Security Blog](https://www.trendaisecurity.com/en-us/resources-insights/trendai-security-blog)

Cyber crime

#  Axios NPM Package Compromised: Supply Chain Attack Hits JavaScript HTTP Client with 100M+ Weekly Downloads

A supply chain attack hit Axios when attackers used stolen npm credentials to publish malicious versions containing a phantom dependency. This triggered a cross-platform RAT during installation and replaced its files with clean decoys, making detection challenging.

Cyber threatsNews

March 31st, 2026   10 min read  [Peter Girnus](https://www.trendaisecurity.com/en-us/resources-insights/author/peter-girnus), [Jacob Santos](https://www.trendaisecurity.com/en-us/resources-insights/author/jacob-santos)

####  Key Takeaways

- Axios, a widely used JavaScript HTTP client with over 100 million weekly npm downloads, was compromised when an attacker hijacked the lead maintainer’s npm account and published two malicious versions (1.14.1 and 0.30.4) that deployed a cross-platform remote access trojan (RAT).
- The attack introduced a phantom dependency, plain-crypto-js@4.2.1, which executed a postinstall hook to deliver persistent malware on macOS, Windows, and Linux, and then erased evidence by replacing its own files with clean decoys.
- The attacker bypassed GitHub Actions’ OIDC Trusted Publisher safeguards by manually publishing poisoned versions using a stolen npm token, leaving no trace in the official GitHub repository.
- Automated npm security scanners flagged the malicious dependency within minutes, and npm administration removed the compromised packages shortly thereafter.
- Forensic analysis revealed sophisticated obfuscation, anti-forensics, and platform-specific payloads, highlighting critical gaps in supply chain security, especially around dependency pinning and CI/CD pipeline protections.

Axios, the JavaScript ecosystem’s most popular HTTP client with [over 100 million weekly npm downloads](https://www.npmjs.com/package/axios), was compromised on March 30, 2026, weaponized as a delivery vehicle for a cross-platform remote access trojan (RAT). The attacker hijacked the lead maintainer’s npm account, published two poisoned versions across both the 1.x and legacy 0.x release branches within 39 minutes of each other, and injected a phantom dependency whose sole purpose was to deploy persistent malware on macOS, Windows, and Linux. The malware self-destructed after execution, replacing its own evidence with a clean decoy.

## Attack timeline

The operation behind the compromise was pre-staged across ~18 hours, with the malicious dependency seeded on npm before the Axios releases to avoid "brand-new package" alarms:

| Timestamp (UTC)  | Event

| March 30, 2026 05:57 UTC  | plain-crypto-js@4.2.0 published by nrwise@proton.me — clean decoy to build registry history

| March 30, 2026 23:59 UTC  | plain-crypto-js@4.2.1 published — malicious payload with postinstall: "node setup.js"

| March 31, 2026 00:05 UTC  | Automated npm security scanners flag plain-crypto-js as malware (six-minute detection)

| March 31, 2026 00:21 UTC  | axios@1.14.1 published by compromised jasonsaayman account (email: ifstap@proton.me)

| March 31, 2026 01:00 UTC  | axios@0.30.4 published — 39 minutes later, poisoning the legacy 0.x branch

| March 31, 2026 ~03:30 UTC  | npm administration removes all compromised versions and revokes tokens

Table 1. Attack timeline of the Axios supply chain compromise

As of this writing, we observed in our telemetry activity consistent with this threat affecting organizations across multiple industries, including Government, Finance, Retail, Consulting, Entertainment, Manufacturing, Technology, Healthcare, and Utilities.

## How the maintainer account was hijacked

The attacker compromised [jasonsaayman](https://www.aikido.dev/blog/axios-npm-compromised-maintainer-hijacked-rat), the lead maintainer of the Axios project on npm. The account email was changed to ifstap@proton.me, an attacker-controlled ProtonMail address.

Every legitimate Axios 1.x release is published via GitHub Actions with npm’s OIDC Trusted Publisher mechanism, cryptographically binding the publish to a verified CI workflow. The malicious 1.14.1 was published manually with a stolen npm access token, with no OIDC binding and no gitHead.

There is no commit, tag, or release in the Axios GitHub repository that corresponds to 1.14.1 or 0.30.4. The releases exists only on npm.

When the Axios collaborator DigitalBrainJS attempted to respond, the attacker used the hijacked credentials (which had admin-level permissions) to unpin and delete disclosure issues. The collaborator [stated](https://github.com/axios/axios/issues/10604) publicly: "It’s pointless. Since access to git and the npm repository is compromised, and his git permissions are higher than mine. I’m a collaborator, not an admin. I can’t revoke his access. Whatever I fix, he will 'fix' it after me."

Key forensic signal — OIDC comparison:

```
// axios@1.14.0 — LEGITIMATE
"_npmUser": {
"name": "GitHub Actions",
"trustedPublisher": { "id": "github", "oidcConfigId": "..." }
}

// axios@1.14.1 — MALICIOUS
"_npmUser": {
"name": "jasonsaayman",
"email": "ifstap@proton.me"
// no trustedPublisher, no gitHead
}
```

Figure 1. GitHub issue #10604 — the Axios collaborator confirming the compromised maintainer has admin permissions.

## The phantom dependency: plain-crypto-js@4.2.1

The only change in both poisoned Axios versions is a single new dependency: plain-crypto-js@^4.2.1. This package is never imported or require()'d anywhere in the Axios source code. A grep across all 86 files confirms zero usage. It exists in package.json solely to trigger npm's automatic dependency resolution and execute its postinstall hook.

This is the defining characteristic of a phantom dependency attack — a package added to the manifest purely for its side effects during installation.

The plain-crypto-js@4.2.1 package was flagged as known malware with a 0% supply chain security score by multiple automated scanners within minutes of publication. The compromised axios@1.14.1 was similarly flagged and subsequently unpublished by npm.

## Technical analysis: The RAT dropper (setup.js)

Obfuscation architecture

All sensitive strings are stored as encoded values in an array stq[]. Two functions decode them at runtime:

- _trans_1(x, r) — XOR cipher: The key "OrDeR_7077" is parsed through JavaScript's Number(). Alphabetic characters produce NaN, which in bitwise operations becomes 0. Only the digits 7, 0, 7, 7 survive, giving an effective key of [0,0,0,0,0,0,7,0,7,7]. Each character is decoded as: charCode XOR key[(7*r*r) % 10] XOR 333
- _trans_2(x, r) — Outer layer: Reverses the encoded string, replaces _ with =, base64-decodes, then passes through _trans_1.

Fully decoded strings

| Index  | Decoded value  | Purpose

| stq[0]  | child_process  | Shell execution

| stq[1]  | os  | Platform detection

| stq[2]  | fs  | Filesystem operations

| stq[3]  | hxxp://sfrclak[.]com:8000/  | C&C base URL

| stq[5]  | win32  | Windows identifier

| stq[6]  | darwin  | macOS identifier

| stq[13]  | package.json  | Deleted after execution

| stq[14]  | package.md  | Clean stub renamed to package.json

Table 2. Decoded string values and their functions

## Annotated source: setup.js

The following walkthrough covers the complete malicious payload from plain-crypto-js@4.2.1, with every significant code path annotated.

Decoding functions

- _trans_1(x, r) splits the key "OrDeR_7077" into characters and maps each through Number(). Non-digit characters produce NaN, which becomes 0 in bitwise XOR. The effective key is [0,0,0,0,0,0,7,0,7,7]. Each character is decoded as: charCode XOR key[(7*r*r) % 10] XOR 333. The quadratic index (7r² mod 10) scrambles the key access pattern to defeat frequency analysis.
- _trans_2(x, r) reverses the encoded string, restores base64 padding (replacing _ with =), base64-decodes, then passes the result through _trans_1. Both functions are wrapped in try/catch {} — errors are silently swallowed so a partial failure never produces a visible stack trace.

Entry point: _entry("6202033")

The campaign ID "6202033" is passed as an argument and appended to the C&C URL. Before dispatching, the function decodes six base64-split placeholder tokens:

| Variable  | Encoded  | Decoded

| E  | atob("TE9DQUw=") + atob("X1BBVEg=")  | LOCAL_PATH

| S  | atob("UFM=") + atob("X1BBVEg=")  | PS_PATH

| a  | atob("U0NSXw==") + atob("TElOSw==")  | SCR_LINK

| c  | atob("UFNfQg==") + atob("SU5BUlk=")  | PS_BINARY

| s  | atob("d2hlcmUgcG93ZXJzaGVsbA==")  | where powershell

Table 3. Platform-specific placeholder tokens used in setup.js

Each atob() call is deliberately split across two halves to prevent grep-based detection of the full string. These tokens are template variables — the dropper decodes platform-specific shell command templates from stq[], then replaces these tokens with the actual C&C URL and local file paths.

The three core modules (fs, os, child_process) are loaded via dynamic require() calls that decode the module name at runtime from stq[]. This evades static analysis scanners that grep for literal require("child_process") patterns, making the imports invisible to automated code review.

Platform dispatch

The dropper uses a for(;;) { ... break } pattern — an infinite loop that executes exactly once. This is a deliberate obfuscation technique that avoids a clean if/else structure identifiable as a platform dispatcher in automated code review. Inside:

- macOS (darwin): Decodes an AppleScript template from stq[9], replaces SCR_LINK with the C&C URL and LOCAL_PATH with /tmp/6202033, writes it to disk, then executes via a command from stq[10].
- Windows (win32): Runs "where powershell" to locate the interpreter, copies it to %PROGRAMDATA%\wt.exe (disguised as Windows Terminal), writes a VBScript launcher to %TEMP%\6202033.vbs with the C&C URL and PowerShell paths injected, then executes.
- Linux (default): Decodes a curl + nohup command from stq[12], replaces SCR_LINK with the C&C URL, and executes directly via execSync.

Anti-forensics (final three operations)

```
F(n);                                  // execSync — RAT is now running
t.unlink(__filename, (x => {}));       // DELETE setup.js
t.unlink("package.json", (x => {}));   // DELETE malicious manifest
t.rename("package.md", "package.json", // SWAP clean stub into place
           (x => {}));
```

Every callback is (x => {}) — errors are silently swallowed. The entire _entry function is wrapped in try/catch {} for the same reason. The final line — _entry("6202033") — is the only unobfuscated call in the file, executing immediately when node setup.js is invoked by npm's postinstall hook.

## Platform-specific payloads

macOS: AppleScript dropper

C&C POST body: packages.npm.org/product0

The C&C returns a macOS binary saved to /Library/Caches/com.apple.act.mond — a path designed to mimic Apple system cache entries. The name "com.apple.act.mond" follows Apple's reverse-DNS daemon convention, likely abbreviating "Activity Monitor Daemon." Made executable with chmod 770 and launched via /bin/zsh in background. AppleScript file self-deletes after execution.

Windows: VBScript + PowerShell chain

C&C POST body: packages.npm.org/product1

Four-stage attack:

- Locates PowerShell via "where powershell"
- Copies PowerShell binary to %PROGRAMDATA%\wt.exe (disguised as Windows Terminal)
- VBScript executes hidden cmd.exe that fetches PowerShell RAT from C&C, runs with -ExecutionPolicy Bypass -WindowStyle Hidden, then self-deletes. Persistent artifact: %PROGRAMDATA%\wt.exe.
- The fetched script executes its final payload entirely in memory. The renamed PowerShell binary (wt.exe) runs the following command:

Figure 2. wt.exe executes the final payload entirely in memory, running the attacker’s script without writing it to disk.

Invoke-WebRequest POSTs the Windows platform identifier to the C&C and receives the second-stage script in the HTTP response body. That body is UTF-8 decoded, passed to [scriptblock]::Create(), and executed inline — entirely in the current PowerShell process's memory. No intermediate file is written for the final payload. The C&C URL is passed as an argument to the scriptblock, enabling the delivered code to call back for additional tasking without hardcoding the address.

Linux: Python RAT

C&C POST body: packages.npm.org/product2

Direct execution via execSync: downloads Python RAT to /tmp/ld.py and runs detached via nohup. The POST body prefix “packages.npm.org/” appears intentionally chosen to look npm-related during quick review and to blend into logs, even though it is not the official npm registry domain (registry.npmjs.org).

## C&C infrastructure analysis

Our analysis of the C&C infrastructure reveals the operational profile of the attacker's server:

| Property  | Value

| Domain registered  | March 30, 2026 16:03:46 UTC

| Registrar  | Namecheap Inc

| Name servers  | dns1.registrar-servers.com, dns2.registrar-servers.com

| Hosting provider  | Hostwinds LLC (AS54290)

| Hosting country  | United States

| DNS SPF record  | v=spf1 include:spf.efwd.registrar-servers.com ~all

| DNS MX records  | eforward[1-5].registrar-servers.com (Namecheap default email forwarding)

Table 4. The attacker’s C&C infrastructure

The domain was registered at 16:03 UTC on March 30, 10 hours after the decoy plain-crypto-js@4.2.0 was published (05:57 UTC) and eight hours before the malicious @4.2.1 payload went live (23:59 UTC). This confirms the infrastructure was provisioned on the same day as the attack staging. The use of Namecheap's default DNS and email forwarding configuration, combined with a newly registered domain on a commodity hosting provider, is consistent with disposable single-operation infrastructure.

Our threat intelligence platform indexes 10 URLs under sfrclak[.]com, including the platform-specific C&C endpoints:

| URL  | Context

| hxxp[://]sfrclak[.]com:8000/6202033  | Primary C&C callback (campaign ID in path)

| hxxp[://]sfrclak[.]com:8000/6202033.ps1  | Windows PowerShell payload download

| hxxp[://]sfrclak[.]com  | Base domain

Table 5. Key URLs associated with sfrclak[.]com C&C server

The low community reputation scores across these URLs indicate that network-level blocking based on domain and IP — rather than URL pattern matching — is the more reliable control at this time.

## Anti-forensics: Self-destruction sequence

After launching the payload, setup.js performs three cleanup steps:

- Deletes itself - fs.unlink(__filename) removes setup.js
- Deletes package.json - removes the manifest with the postinstall hook
- Renames package.md to package.json - a pre-staged clean stub (v4.2.0, no postinstall) takes its place

Post-infection, any inspection of node_modules/plain-crypto-js/package.json shows a completely clean manifest. The evidence has been swapped in-place.

Detection signal: The mere existence of node_modules/plain-crypto-js/ is sufficient evidence of compromise — this package is not a dependency of any legitimate Axios version.

## Runtime confirmation

The static analysis was validated by installing axios@1.14.1 inside a GitHub Actions runner instrumented with runtime security monitoring in audit mode, capturing every outbound connection, spawned process, and file write at the kernel level.

Two C&C connections were recorded:

- curl at 01:30:51Z - 1.1 seconds after npm install began
- nohup at 01:31:27Z - in a different workflow step entirely

The malware persisted as a detached background process orphaned to PID 1, independent of npm.

## Process tree: Full kill chain

```
PID 2366 bash (workflow script)
  └─ PID 2380 npm install axios@1.14.1
       └─ PID 2391 sh -c "node setup.js"
            └─ PID 2392 node setup.js
                 └─ PID 2399 /bin/sh -c "curl ... && nohup python3 ..."

PID 2401 curl -o /tmp/ld.py ... ppid: 2400
PID 2400 nohup python3 /tmp/ld.py ppid: 1 ← ORPHANED TO INIT
```

Figure 3. Infection flow of the malicious npm dependency leading to compromise

## Broader campaign: Related packages

- @shadanai/openclaw: A fork of OpenClaw AI gateway with plain-crypto-js hidden in a vendored path. Identical setup.js, same C&C, same payloads.
- @qqbrowser/openclaw-qbot@0.0.130: Ships a tampered axios@1.14.1 pre-baked in its node_modules/ directory with plain-crypto-js already injected. Different injection vector, same malware.

## MITRE ATT&CK TTPs

| ID  | Technique  | Description

| T1195.002  | Compromise Software Supply Chain  | Hijacked npm maintainer account published poisoned Axios versions bypassing OIDC Trusted Publisher

| T1059.005  | Command and Scripting Interpreter: VBScript  | 6202033.vbs launcher writes and executes PowerShell chain from %TEMP%

| T1059.001  | Command and Scripting Interpreter: PowerShell  | Fileless [scriptblock]::Create() execution from C&C HTTP response body via renamed wt.exe

| T1059.006  | Command and Scripting Interpreter: Python  | Linux RAT /tmp/ld.py executed detached via nohup, orphaned to PID 1

| T1027  | Obfuscated Files or Information  | XOR cipher (OrDeR_7077) + base64 reversal + string array rotation + split atob() calls

| T1036  | Masquerading  | wt.exe mimics Windows Terminal; com.apple.act.mond mimics Apple daemon; packages.npm.org/ prefix in POST body mimics npm registry traffic

| T1070.004  | Indicator Removal: File Deletion  | setup.js self-deletes, removes package.json, swaps in clean package.md stub

| T1082  | System Information Discovery  | os.platform() fingerprint determines macOS/Windows/Linux execution branch

| T1071.001  | Application Layer Protocol: Web Protocols  | HTTP POST to sfrclak[.]com:8000 with campaign ID 6202033 in URL path

| T1620  | Reflective Code Loading  | PowerShell [scriptblock]::Create() from C&C response content executed in-process without disk write

Table 6. TTPs used in the software supply chain attack

## Remediation

Rapid and thorough remediation is essential after a supply chain attack like the Axios compromise. The following steps can help in removing malicious components, restoring secure environments, and strengthening defenses against future incidents:

- Pin to safe versions: npm install axios@1.14.0 (1.x) or axios@0.30.3 (0.x)
- Add overrides/resolutions in package.json to prevent transitive resolution
- Remove plain-crypto-js: rm -rf node_modules/plain-crypto-js && npm install --ignore-scripts
- If RAT artifacts found: Do not clean in place. Rebuild from known-good state.
- Rotate ALL credentials: npm tokens, AWS keys, SSH keys, CI/CD secrets, .env values
- Audit CI/CD pipelines: Any workflow that ran npm install during exposure window — rotate all injected secrets
- Block C&C at network/DNS: Add sfrclak.com to blocklist
- Use npm ci --ignore-scripts in CI/CD as standing policy

## Lessons for developers, registry operators, and security teams

The Axios compromise reveals critical gaps in supply chain security, showing how a single account takeover can impact millions. This attack highlights the importance of strict dependency pinning, CI/CD safeguards, and vigilant registry controls. In examining the technical and operational failings exposed in this campaign, we can identify valuable insights that can help mitigate future risks:

For developers

- Pin dependency versions. Caret ranges (^1.14.0) silently pull the next minor/patch. Exact pinning with lockfile verification is the minimum defense.
- Use npm ci --ignore-scripts in CI/CD. Postinstall hooks are the primary execution vector for npm supply chain attacks.
- Audit lockfiles, not just package.json. The malicious dependency was transitive — only visible in package-lock.json.

For package registry operators

- OIDC Trusted Publisher adoption must become the norm. The malicious publish was trivially detectable as anomalous because it lacked the OIDC binding.
- Account takeover detection should flag email changes on high-download packages and trigger review holds.
- Cooldown windows on newly published packages, even 24 hours would have limited blast radius.

For security teams

- Monitor for phantom dependencies: packages in manifests but never imported in source code.
- Network egress monitoring in CI/CD catches what static analysis misses.
- Post-install evidence can be destroyed. Lockfile diffs and network logs are more reliable forensic sources.

## Proactive security with TrendAI Vision One™

[TrendAI Vision One™](https://www.trendaisecurity.com/en-us/platform) platform is the industry-leading AI cybersecurity platform that centralizes cyber risk exposure management, security operations, and robust layered protection.

## TrendAI Vision One™ Threat Intelligence Hub

[TrendAI Vision One™ Threat Intelligence Hub](https://www.trendaisecurity.com/en-us/platform/threat-intelligence) provides the latest insights on emerging threats and threat actors, exclusive strategic reports from TrendAI™ Research, and TrendAI Vision One™ Threat Intelligence Feed in the TrendAI Vision One™ platform.

Emerging Threats: [Axios Compromised: Inside the Supply Chain Attack That Hit 100M+ Weekly Downloads](https://portal.xdr.trendmicro.com/index.html#/app/ti/intelligence_insights?name=Axios%20Compromised%3A%20Inside%20the%20Supply%20Chain%20Attack%20That%20Hit%20100M%2B%20Weekly%20Downloads)

## TrendAI Vision One™ Intelligence Reports (IOC Sweeping)

[Axios Compromised: Inside the Supply Chain Attack That Hit 100M+ Weekly Downloads](https://portal.xdr.trendmicro.com/index.html#/app/ti/intelligence?intrusionSet=Axios%20Compromised%3A%20Inside%20the%20Supply%20Chain%20Attack%20That%20Hit%20100M%2B%20Weekly%20Downloads)

## Hunting Queries

### TrendAI Vision One™ Search App

TrendAI Vision One™ customers can use the Search App to match or hunt the malicious indicators mentioned in this blog post with data in their environment.

Detect campaign-specific temp file creation by npm postinstall dropper

eventSubId:101 AND objectFilePath:(*\Temp\6202033* OR */tmp/6202033*) AND parentFilePath:(*\node.exe OR */node)

Detect outbound connections to Axios supply chain attack C&C

eventSubId:204 AND dst:("sfrclak.com" OR "142.11.206.73")

More hunting queries are available for TrendAI Vision One™ with [Threat Intelligence Hub](https://www.trendaisecurity.com/en-us/platform/threat-intelligence) entitlement enabled.

## Indicators of compromise (IOCs)

| Indicator  | Value  | Detection

| Network Indicators  |    |

| C&C Domain  | sfrclak[.]com  | 91. C&C server

| C&C URL  | http[://]sfrclak[.]com:8000/6202033  | 79. Disease Vector

| C&C Domain  | callnrwise[.]com  | 91. C&C server

| C&C IP  | 142[.]11[.]206[.]73  | 91. C&C server

| POST body (macOS)  | packages.npm.org/product0  |

| POST body (Windows)  | packages.npm.org/product1  |

| POST body (Linux)  | packages.npm.org/product2  |

| Malicious NPM Packages  |    |

| axios@1.14.1  | 2553649f2322049666871cea80a5d0d6adc700ca  | Trojan.JS.TPCPSTEAL.A

| axios@0.30.4  | d6f3f62fd3b9f5432f5782b62d8cfd5247d5ee71  |

| plain-crypto-js@4.2.1  | 07d889e2dadce6f3910dcbc253317d28ca61c766  | Trojan.JS.TPCPSTEAL.A

| File Hashes  |    |

| setup.js — RAT dropper (plain-crypto-js@4.2.1 postinstall payload)  | e10b1fa84f1d6481625f741b69892780140d4e0e7769e7491e5f4d894c2e0e09  | [Trojan.JS.AXIOSDROP.THCCABF](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/Trojan.JS.AXIOSDROP.THCCABF)

| 6202033.vbs — Powershell RAT downloader  | 591a70e8b166265804c1e2add3f5554b38364a8750248a8c5be751c5cd9b1655  | [Trojan.VBS.AXIOSDROP.AA](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/Trojan.VBS.AXIOSDROP.AA)

| 6202033.ps1 — PowerShell RAT  | ed8560c1ac7ceb6983ba995124d5917dc1a00288912387a6389296637d5f815c  | [Backdoor.PS1.AXIOSRAT.THCCABF](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/Backdoor.PS1.AXIOSRAT.THCCABF)

| 6202033.ps1 — PowerShell RAT  | 617b67a8e1210e4fc87c92d1d1da45a2f311c08d26e89b12307cf583c900d101  | Trojan.PS1.AXIOSDROP.THCCABF

| system.bat — Windows fileless loader  | f7d335205b8d7b20208fb3ef93ee6dc817905dc3ae0c10a0b164f4e7d07121cd  | [Trojan.PS1.AXIOSDROP.THCCABG](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/Trojan.PS1.AXIOSDROP.THCCABG)

| com.apple.act.mond  | 92ff08773995ebc8d55ec4b8e1a225d0d1e51efa4ef88b8849d0071230c9645a  | Backdoor.MacOS.NUKESPED.MANP

| ld.py — Linux Python RAT  | fcb81618bb15edfdedfb638b4c08a2af9cac9ecfa551af135a8402bf980375cf  | [Backdoor.Python.AXIOSRAT.THCCABF](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/malware/Backdoor.Python.AXIOSRAT.THCCABF)

With contributions from Ian Kenefick.
