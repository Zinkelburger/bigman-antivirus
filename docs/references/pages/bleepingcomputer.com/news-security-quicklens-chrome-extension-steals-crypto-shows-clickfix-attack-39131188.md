---
url: "https://www.bleepingcomputer.com/news/security/quicklens-chrome-extension-steals-crypto-shows-clickfix-attack/"
title: "QuickLens Chrome extension steals crypto, shows ClickFix attack"
site: "BleepingComputer"
captured: "2026-11-02T18:54:52Z"
via: "live"
sha256: "469a79de46b1e9c14bb4e49786add0e8083e3fcf198cb6b5ab0113d1b6f47696"
wayback: "https://web.archive.org/web/20261024092827/https://www.bleepingcomputer.com/news/security/quicklens-chrome-extension-steals-crypto-shows-clickfix-attack/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# QuickLens Chrome extension steals crypto, shows ClickFix attack

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.bleepingcomputer.com/news/security/quicklens-chrome-extension-steals-crypto-shows-clickfix-attack/>
> Snapshot: <https://web.archive.org/web/20261024092827/https://www.bleepingcomputer.com/news/security/quicklens-chrome-extension-steals-crypto-shows-clickfix-attack/>

# QuickLens Chrome extension steals crypto, shows ClickFix attack

By

###### [Lawrence Abrams](https://www.bleepingcomputer.com/author/lawrence-abrams/)

- February 28, 2026
- 02:18 PM
- [0](https://www.bleepingcomputer.com/news/security/quicklens-chrome-extension-steals-crypto-shows-clickfix-attack/#comment_form)

A Chrome extension named "QuickLens - Search Screen with Google Lens" has been removed from the Chrome Web Store after it was compromised to push malware and attempt to steal crypto from thousands of users.

QuickLens was initially published as a Chrome extension that lets users run Google Lens searches directly in their browser. The extension grew to roughly 7,000 users and, at one point, received a featured badge from Google.

However, on February 17, 2026, a new version 5.8 was released that contained malicious scripts that introduced ClickFix attacks and info-stealing functionality for those using the extension.

https://www.wiz.io/lp/ai-threat-readiness-playbook?utm_source=bleepingcomputer&utm_medium=display&utm_campaign=FY27Q2_INB_FORM_AI-Threat-Readiness-Playbook&sfcid=701Vh00000cnU0lIAE&utm_term=FY27Q4-bleepingcomputer-970x250-Nov&utm_content=AITR-Playbook

## The malicious QuickLens extension

Security researchers at Annex [first reported](https://annex.security/blog/pixel-perfect/) that the extension had recently changed ownership after being listed for sale on ExtensionHub, a marketplace where developers sell browser extensions.

Annex says that on February 1, 2026, the owner changed to support@doodlebuggle.top under "LLC Quick Lens," with a new privacy policy hosted on a barely functional domain. Just over two weeks later, the malicious update was pushed to users.

Annex's analysis shows that version 5.8 requested new browser permissions, including declarativeNetRequestWithHostAccess and webRequest.

It also included a rules.json file that stripped browser security headers, such as Content-Security-Policy (CSP), X-Frame-Options, and X-XSS-Protection, from all pages and frames. These headers would have made it more difficult to run malicious scripts on websites.

The update also introduced communication with a command-and-control (C2) server at api.extensionanalyticspro[.]top. According to Annex, the extension generated a persistent UUID, fingerprinted the victim's country using Cloudflare's trace endpoint, identified the browser and OS, and then polled the C2 server every five minutes for instructions.

BleepingComputer learned about the extension this week after seeing numerous users [[1](https://community.brave.app/t/google-chrome-update-message-keeps-popping-up/649721), [2](https://www.reddit.com/r/ArcBrowser/comments/1r8wxep/whats_that/)] reporting fake Google Update alerts on every web page they visited.

"That is appearing in every site i go, i through it could be because Chrome wasn't updated, but even after uptading it continues to appear," a user seeking help said on [Reddit](http://www.reddit.com/r/antivirus/comments/1rco595/fake_captcha_appearing_in_every_site_on_chrome/).

"Of course i will not run the code that it copy on my clipboard on the run box but it keeps appearing in every site, making it impossible to interact with anything."

BleepingComputer's analysis of the extension showed it connected to a C2 server at https://api.extensionanalyticspro[.]top/extensions/callback?uuid=[uuid]&extension=kdenlnncndfnhkognokgfpabgkgehoddto, where it received an array of malicious JavaScript scripts.

These payloads were then executed on every page load using a technique that Annex described as a "1x1 GIF pixel onload trick."

Array of malicious JavaScript payloads
Source: BleepingComputer

Because the extension stripped CSP headers on all visited sites, this inline JavaScript execution worked even on sites that would normally block it.

The first payload contacts google-update[.]icu, where it receives an additional payload that displays a fake Google Update prompt. Clicking the update button would display a ClickFix attack, prompting users to perform a verification by running code on their computers.

Fake Google Update alert leading to a ClickFix attack
Source: Reddit [[1](https://www.reddit.com/r/ArcBrowser/comments/1r8wxep/whats_that/), [2](https://www.reddit.com/r/antivirus/comments/1rco595/fake_captcha_appearing_in_every_site_on_chrome/)]

For Windows users, this led to the download of a malicious executable named "googleupdate.exe" [[VirusTotal](https://www.virustotal.com/gui/file/099d63e692457bfccc2cf59278ae6a268cb03964f18d0d27f536027b43c89896/telemetry)] that was signed with a certificate from "Hubei Da'e Zhidao Food Technology Co., Ltd."

Upon execution, the malware launched a hidden PowerShell command that spawned a second PowerShell instance to connect to drivers[.]solutions/META-INF/xuoa.sys using a custom "Katzilla" user agent.

The response was piped into Invoke-Expression for execution. However, by the time BleepingComputer analyzed the payloads, the second-stage URL was no longer serving any malicious content.

Another malicious JavaScript "agent" delivered by the https://api.extensionanalyticspro[.]top C2 was used to steal cryptocurrency wallets and credentials.

The extension would detect if MetaMask, Phantom, Coinbase Wallet, Trust Wallet, Solflare, Backpack, Brave Wallet, Exodus, Binance Chain Wallet, WalletConnect, and the Argon crypto wallets were installed. If so, it would attempt to steal activity and seed phrases, which would be used to hijack wallets and steal their assets.

Another script captured login credentials, payment information, and other sensitive form data.

Additional payloads were used to scrape Gmail inbox contents, extract Facebook Business Manager advertising account data, and collect YouTube channel information.

A review of the now-removed Chrome extension page claims that macOS users were targeted with the AMOS (Atomic Stealer) infostealer. BleepingComputer has not been able to independently verify if these claims are true.

Google has since removed QuickLens from the Chrome Web Store, and Chrome now automatically disables it for affected users.

QuickLens disabled and flagged as malware by Chrome
Source: BleepingComputer

Users who installed QuickLens - Search Screen with Google Lens should ensure the extension is fully removed, scan their device for malware, and reset passwords for any credentials stored in the browser.

If you use any of the mentioned cryptocurrency wallets, you should transfer your funds to a new wallet.

This extension is not the first to be used in ClickFix attacks. Last month, Huntress discovered a browser extension that [intentionally crashed browsers](https://www.bleepingcomputer.com/news/security/fake-ad-blocker-extension-crashes-the-browser-for-clickfix-attacks/) and then displayed fake fixes that installed the ModeloRAT malware.

https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY

##  [Uncover email threats your existing defenses may miss.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

Adaptive Security's AI agents analyze email intent, context, and behavior to uncover phishing, business email compromise, and invoice fraud.

See what's getting through with a 14-day read-only scan of Microsoft 365 or Google Workspace, with no MX changes.

[Start your free email security scan.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

### Related Articles:

[Chrome extensions with 13.4 million installs caught hijacking searches](https://www.bleepingcomputer.com/news/security/chrome-extensions-with-134-million-installs-caught-hijacking-searches/)

[Polymarket halts trading after crypto-draining script breach, $2.5M traced](https://www.bleepingcomputer.com/news/security/polymarket-halts-trading-after-crypto-draining-script-breach-25m-traced/)

[Google Chrome adds Gemini-powered scam detection, warns users of ClickFix attacks](https://www.bleepingcomputer.com/news/google/google-chrome-adds-gemini-powered-scam-detection-warns-users-of-clickfix-attacks/)

[ClickFix has never been bigger, and AI tools are its most convincing lure yet](https://www.bleepingcomputer.com/news/security/clickfix-has-never-been-bigger-and-ai-tools-are-its-most-convincing-lure-yet/)

[Hackers abuse Google Ads, Bing redirects to push Claude ClickFix attacks](https://www.bleepingcomputer.com/news/security/hackers-abuse-google-ads-bing-redirects-to-push-claude-clickfix-attacks/)
