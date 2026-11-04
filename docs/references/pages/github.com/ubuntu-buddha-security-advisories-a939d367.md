---
url: "https://github.com/Ubuntu-Buddha/security-advisories"
title: "GitHub - Ubuntu-Buddha/security-advisories: Security advisories and malware-analysis writeups by Trig Bundgaard (@Ubuntu-Buddha)."
site: "GitHub"
captured: "2026-11-03T20:33:02Z"
via: "live"
sha256: "e65ef8291e08b42b5d9f8595740295998db6f2ab6d4534aeda1eace75fa24fa7"
cited_in:
  - "docs/research/interview-repo-lures.md"
  - "docs/threats/01-phishing-social-engineering.md"
  - "docs/threats/02-supply-chain-packages.md"
---

# GitHub - Ubuntu-Buddha/security-advisories: Security advisories and malware-analysis writeups by Trig Bundgaard (@Ubuntu-Buddha).

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/Ubuntu-Buddha/security-advisories>

# Security Advisories

https://github.com/Ubuntu-Buddha/security-advisories#security-advisories

Threat-analysis and vulnerability-disclosure writeups by Trygve Bundgaard (GitHub: [@Ubuntu-Buddha](https://github.com/Ubuntu-Buddha)).

Defensive security research: malware targeting software developers and vulnerabilities in my own AI-agent tooling (a Claude Code plugin). I also work on vulnerability disclosure for web3 / Solana / EVM platforms; no such writeup is published here yet. All indicators of compromise below are defanged (hxxp, [.]) so this page is safe to read and copy.

## Advisories

https://github.com/Ubuntu-Buddha/security-advisories#advisories

| Date  | Title  | Type  | Status

| 2026-10-23  | [Launch gate bypasses in the StrangerDanger Claude Code plugin v0.1.0 (GHSA-w42w-fc6x-88wr)](https://github.com/Ubuntu-Buddha/security-advisories/blob/main/advisories/2026-10-23-stranger-danger-launch-gate-bypass.md)  | Vulnerability / security-tool bypass  | Found in the maintainer's own review; patched in v0.1.1 (keep Monitor denied; see advisory); GHSA published 2026-10-23; CVE requested, none assigned as of 2026-10-30

| 2026-10-15  | [Hex payload hidden in a fake font file, run by a new Function loader (coinbucket/dex-mvp)](https://github.com/Ubuntu-Buddha/security-advisories/blob/main/advisories/2026-10-15-fake-font-hex-loader-dex-mvp.md)  | Malware / RCE  | Malware identified before execution by remote review; reported to GitLab and Upwork 2026-10-15, outcome pending

| 2026-01-30  | [IDE auto-execute RCE via malicious .vscode/tasks.json](https://github.com/Ubuntu-Buddha/security-advisories/blob/main/advisories/2026-01-30-ide-autorun-rce-vscode-tasks.md)  | Malware / RCE  | Malware identified before execution; one of the two repos reported to GitHub; both accounts now 404

| 2026-01-29  | [Runtime RCE backdoor hidden in an environment variable (Optu-Consulting)](https://github.com/Ubuntu-Buddha/security-advisories/blob/main/advisories/2026-01-29-env-var-runtime-rce-optu.md)  | Malware / RCE  | Malware identified before execution; account and repo now 404

| 2026-01-29  | [Fake "careers" page pushing a malicious "GAPI Update" .exe (Google Sites abuse)](https://github.com/Ubuntu-Buddha/security-advisories/blob/main/advisories/2026-fake-careers-page-gapi-update-exe.md)  | Malware / phishing  | Malware identified before execution; reportable to platform & Google

| 2026-01-27  | [RCE backdoor in a fake "Web3 developer" interview test (TechPro-1/W3GLFun)](https://github.com/Ubuntu-Buddha/security-advisories/blob/main/advisories/2026-01-27-contagious-interview-w3glfun-rce.md)  | Malware / RCE  | Reported; GitHub Trust & Safety took action against the account; account now 404

### Methodology

https://github.com/Ubuntu-Buddha/security-advisories#methodology

- [Safely triaging "interview test" repos: a field guide to developer-targeted malware](https://github.com/Ubuntu-Buddha/security-advisories/blob/main/advisories/2026-01-safely-triaging-interview-test-repos.md) — red-flag taxonomy, safe online-first inspection protocol, and the campaign's delivery-vector catalog.

## Contact

https://github.com/Ubuntu-Buddha/security-advisories#contact

Report malware or vulnerabilities to me via my GitHub profile.

## License

https://github.com/Ubuntu-Buddha/security-advisories#license

The writeups in this repository are licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://github.com/Ubuntu-Buddha/security-advisories/blob/main/LICENSE), © 2026 Trygve Bundgaard. Malicious code, attacker messages, and platform emails quoted for analysis belong to their authors and are not covered by this license.
