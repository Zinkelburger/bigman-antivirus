---
url: "https://www.wiz.io/blog/rust-supply-chain-attack-on-arrayref-significant-overlap-with-dprk-campaigns"
title: "Rust Supply Chain Attack on arrayref: Significant Overlap with DPRK Campaigns | Wiz Blog"
site: "wiz.io"
published: "2026-08-20T11:56:39-04:00"
captured: "2026-11-02T19:03:01Z"
via: "live"
sha256: "6235c09a5fe997aa983aa519c711938db3db4ae452afc5564d051dbd6b2347a0"
wayback: "https://web.archive.org/web/20260923182828/https://www.wiz.io/blog/rust-supply-chain-attack-on-arrayref-significant-overlap-with-dprk-campaigns"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Rust Supply Chain Attack on arrayref: Significant Overlap with DPRK Campaigns | Wiz Blog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.wiz.io/blog/rust-supply-chain-attack-on-arrayref-significant-overlap-with-dprk-campaigns>
> Snapshot: <https://web.archive.org/web/20260923182828/https://www.wiz.io/blog/rust-supply-chain-attack-on-arrayref-significant-overlap-with-dprk-campaigns>

On August 20, 2026, malicious versions of three Rust crates were published to crates.io: arrayref@0.3.10, internment@0.8.7, and append-only-vec@0.1.9. The malicious crates added a typosquatted dependency (proc-macro1) whose build script downloads and executes a remote binary. Notably, proc-macro1 was the first dependency added to arrayref in its ten-year history.

Because build scripts run during compilation, building an affected project was sufficient to execute the payload. arrayref can be found in over 35% of all environments. Even more notably, it's used in ¾ of all environments where Rust is present.

Prevalence of impacted Rust Packages (any version)

The Rust Security Response Team [deleted the malicious versions and locked the account, and assesses that the maintainer's machine or credentials were compromised](https://blog.rust-lang.org/2026/08/20/supply-chain-attack-on-arrayref/).

Wiz customers should review our Threat Intel Center advisory: [arrayref and Other Rust Crates Hijacked in Supply Chain Attack](https://app.wiz.io/boards/threat-center/wiz-adv-2026-151)

##

Technical Details

The impacted package versions add a malicious dependency to the Cargo.toml:

```
[dependencies]
proc-macro1 = "1.0.107"
```

proc-macro1 is a typosquat of the legitimate proc-macro2 crate (154M+ downloads). Its build.rs contains the malicious logic. Because Cargo executes build scripts at compile time, building a project that depends on proc-macro1 is sufficient to trigger the payload.

###

Malicious build.rs

At compile time, the build script:

-

Reconstructs a C2 URL from Base64 fragments (e.g. https://23.254.165[.]112:9089/)

-

Disables TLS certificate validation via a custom AcceptAll verifier

-

Downloads a platform-specific payload based on OS and architecture

-

Writes it to /tmp/rust-setup (Unix) or %TEMP%\rust-setup.ps1 (Windows)

-

Executes the payload, passing the C2 beacon address as an argument

The build otherwise completes and the package functions normally. The second stage payload is selected based on the platform, with support for x86_64 versions of Linux, Windows, and macOS, in addition to aarch64 macOS.

###

Second Stage Capabilities

Wiz Research was able to analyze the malicious Rust crates, [retrieved from Google Threat Intelligence](https://www.virustotal.com/gui/collection/3a675745859dc5905fd450855c24f127242c2a6ebfbb584633242b5da3967676/iocs). The implant is a featureful backdoor that:

-

Beacons to C2 via HTTPS POST (to the endpoint /49890878), exfiltrating host info and stolen credentials as Base64-encoded JSON

-

Collects hostname, username and operating system details, enumerates installed applications, and reads Chrome, Brave and Edge profiles for saved logins and extension settings, querying the browsers' SQLite credential stores directly. Edit: A prior version of this piece mistakenly stated that browser credentials were stolen. The queries only enumerate saved logins, they do not retrieve the encrypted credential material.

-

Persists via Registry Run key (Windows), LaunchAgent (macOS), or systemd user service (Linux)

-

Supports four commands: kill (terminate), minicfg (reconfigure C2 and beacon interval), startup (install persistence), and runscript (download and execute PowerShell or shell scripts, synchronously or in background)

-

Falls back to a Domain Generation Algorithm if the primary C2 is unreachable, generating 10 algorithmic .com domains every 5 days. Currently, the relevant domains do not appear to be registered.

Configuration is encrypted with AES-128-GCM using the hardcoded key i am botking. Commands are authenticated via an embedded RSA-2048 private key.

##

Overlap with DPRK Supply Chain Attacks

The arrayref infrastructure substantially overlaps with operations attributed to recent North Korean actors.

Shared C2 endpoint pattern: The arrayref payloads beacon to /49890878. This endpoint has been used in the Mastra campaign, [attributed by Microsoft to DPRK / Sapphire Sleet](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/). The IP address used in the arrayref beacon also shares an SSL issuer (WIN-A6QF8AHPQH1\Administrator@WIN-A6QF8AHPQH1) with 23.254.167[.]13 - also used in the Mastra campaign.

Victim-reported infrastructure overlap: A victim has[reported C2 traffic to 23.254.167[.]216](https://github.com/rustsec/advisory-db/issues/3161#issuecomment-5356130756). This IP appears in [Google Cloud Threat Intelligence's analysis of UNC1069's axios npm attack](https://cloud.google.com/blog/topics/threat-intelligence/north-korea-threat-actor-targets-axios-npm-package), which Mandiant links to North Korea.

Preferred Host: Both campaigns generally use the same 23.254.164.0/23 range of Hostwinds LLC infrastructure we see repeated in this incident.

##

Recommendations

Wiz customers should review our Threat Intel Center advisory: [arrayref and Other Rust Crates Hijacked in Supply Chain Attack](https://app.wiz.io/boards/threat-center/wiz-adv-2026-151).

-

Check your lockfiles and local cache: search Cargo.lock across your repositories for the malicious versions and for any of the six attacker-controlled crate names. Locally, run:

```
find ~/.cargo/registry/cache -type f \( \
  -name 'arrayref-0.3.10.crate' -o \
  -name 'internment-0.8.7.crate' -o \
  -name 'append-only-vec-0.1.9.crate' -o \
  -name 'proc-macro1-*.crate' -o \
  -name 'proc-macro-en-*.crate' -o \
  -name 'aovine-*.crate' -o \
  -name 'arone-*.crate' -o \
  -name 'aronenao-*.crate' -o \
  -name 'tinymember-*.crate' \
\) -print
```

-

Treat affected hosts as compromised: because the payload executes during cargo build, any developer workstation or CI runner that built an affected project must be treated as compromised. Rotate every credential, token and key reachable from it, including CI secrets and signing keys, and rebuild any artifacts produced after exposure from clean sources.

-

Rotate browser-stored credentials: the payload reads saved logins and extension data from Chrome, Brave and Edge profiles on the affected host. Reset those passwords and revoke the associated sessions.

-

Remove payload and persistence artifacts: delete /tmp/rust-setup, %TEMP%\rust-setup.ps1 and %TEMP%\rust-setup-launch.vbs where present, and check for unrecognized systemd user services, HKCU Run entries and LaunchAgents. Re-deploy affected workloads from a known clean state.

-

Do not resolve yank warnings by upgrading blindly: this attack used yanking to drive upgrades. Treat a sudden yank of multiple stable versions of a long-lived crate as a signal to investigate.

-

Review build-time dependencies: build scripts execute with full user privileges during compilation. Review any new or changed build-dependencies entry, particularly networking crates such as ureq, reqwest or rustls in a crate with no reason to make network calls.

##

Indicators of Compromise (IOCs)

| Category | Indicator | Details

| Package | arrayref @0.3.10 | Hijacked release

| Package | internment @0.8.7 | Hijacked release

| Package | append-only-vec @0.1.9 | Hijacked release

| Package | proc-macro1 | Typosquat of proc-macro2, all versions deleted

| Package | proc-macro-en | Attacker-controlled, all versions deleted

| Package | aovine | Attacker-controlled, all versions deleted

| Package | arone | Attacker-controlled, all versions deleted

| Package | aronenao | Attacker-controlled, all versions deleted

| Package | tinymember | Attacker-controlled, all versions deleted

| SHA256 | 25ad700976873c76af785cb99b33c48db7df8b81f21d1e9e06b3676b9a9373ae | arrayref-0.3.10.crate

| SHA256 | 61198155da51b838772eecf5bfaac6cbc4dcc388dccc56658fc28a8e831b34d4 | proc-macro1-1.0.107.crate

| SHA256 | b5c1b5b0763a8809a644a8f92224653f0aca623a98eecc714d27f74b80fbe436 | proc-macro1-1.0.106.crate

| SHA1 | f22e3e01e38bcdf001f0d15a2dbfdec5a1cf8eff | proc-macro1-1.0.107.crate

| SHA1 | f4767ad92cb61401fd69139cade563501c39b991 | rust-crate_0.1.0, Linux stage-2 payload

| SHA1 | fc0fdb978eac72f4484b48db058e4473f1bc516e | rust-crate_0.2.0, Windows stage-2 payload

| SHA1 | ff7e20cf642346bf893f1eca808df82035bb53d0 | rust-crate_0.4.0, macOS arm64 stage-2 payload

| IP | 23.254.165[.]112:9089 | Stage-2 payload host (Hostwinds VPS)

| IP | 23.254.165[.]112:443 | C2 address passed to the payload as argv[1]

| IP | 23.254.167[.]107:443 | Stage-2 C2, live at publication

| Domain | hwsrv-798836.hostwindsdns[.]com | Attacker infrastructure hostname

| Network | POST /49890878 | Stage-2 C2 request path

| File | /tmp/rust-setup | Stage-2 payload, Unix

| File | %TEMP%\rust-setup.ps1 | Stage-2 payload, Windows

| File | %TEMP%\rust-setup-launch.vbs | Hidden Windows launcher, executed via wscript.exe

| Binary | rust-crate_0.1.0 / _0.2.0 / _0.3.0 / _0.4.0 | Platform-specific stage-2 payloads on the payload host

| Account | dtolney | Impersonation account, publisher of proc-macro1

| Account | droundy | Legitimate maintainer; machine or credentials likely compromised, account locked as a precaution

| Email | rchaitm@gmail[.]com | Forged author metadata in proc-macro1

##

Tags

[#Research](https://www.wiz.io/blog/tag/research)[#Threat Intel](https://www.wiz.io/blog/tag/threat-intel)

https://www.wiz.io/blog/wiz-pen-test-findings-is-now-ga

[### Wiz Penetration Test Findings is now GA](https://www.wiz.io/blog/wiz-pen-test-findings-is-now-ga)

https://www.wiz.io/authors/roi-kesslerhttps://www.wiz.io/authors/shaked-rotlevi

[Roi Kessler](https://www.wiz.io/authors/roi-kessler), [Shaked Rotlevi](https://www.wiz.io/authors/shaked-rotlevi)

August 19, 2026

Transform point-in-time pen-tests into continuous exposure management with unified platform combining pen-test findings and real-time cloud context

https://www.wiz.io/blog/detecting-entra-device-registration-abuse

[###  How to Spot and Stop Rogue Device Joins](https://www.wiz.io/blog/detecting-entra-device-registration-abuse)

https://www.wiz.io/authors/shahar-dorfmanhttps://www.wiz.io/authors/sapir-federovsky

[Shahar Dorfman](https://www.wiz.io/authors/shahar-dorfman), [Sapir Federovsky](https://www.wiz.io/authors/sapir-federovsky)

August 18, 2026

Instead of leaving behind recognizable fingerprints from public tooling, adversaries can now generate realistic device names that blend naturally into enterprise environments. This blog explores how that changes Entra ID detection and what are the behavioral signals that still expose these attacks.

https://www.wiz.io/blog/2026-partner-award-winners

[### Announcing the 2026 Wiz Partner Alliance Award Winners](https://www.wiz.io/blog/2026-partner-award-winners)

https://www.wiz.io/authors/andy-ritchie

[Andy Ritchie](https://www.wiz.io/authors/andy-ritchie)

August 18, 2026

Recognizing the partners, integrators, and visionaries driving cloud security transformation, AI risk management, and SOC modernization across AMER, EMEA, and ANZ.
