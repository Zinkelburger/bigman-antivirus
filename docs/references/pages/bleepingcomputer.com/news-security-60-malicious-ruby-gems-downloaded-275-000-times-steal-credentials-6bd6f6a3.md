---
url: "https://www.bleepingcomputer.com/news/security/60-malicious-ruby-gems-downloaded-275-000-times-steal-credentials/"
title: "60 malicious Ruby gems downloaded 275,000 times steal credentials"
site: "BleepingComputer"
captured: "2026-11-02T18:52:24Z"
via: "live"
sha256: "0df78a41f068596161ddfd2fef28aad809c7e4e1aa9824be95a2bd2bb1edc4b7"
wayback: "https://web.archive.org/web/20261023025309/https://www.bleepingcomputer.com/news/security/60-malicious-ruby-gems-downloaded-275-000-times-steal-credentials/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# 60 malicious Ruby gems downloaded 275,000 times steal credentials

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.bleepingcomputer.com/news/security/60-malicious-ruby-gems-downloaded-275-000-times-steal-credentials/>
> Snapshot: <https://web.archive.org/web/20261023025309/https://www.bleepingcomputer.com/news/security/60-malicious-ruby-gems-downloaded-275-000-times-steal-credentials/>

# 60 malicious Ruby gems downloaded 275,000 times steal credentials

By

###### [Bill Toulas](https://www.bleepingcomputer.com/author/bill-toulas/)

- August 9, 2025
- 10:11 AM
- [0](https://www.bleepingcomputer.com/news/security/60-malicious-ruby-gems-downloaded-275-000-times-steal-credentials/#comment_form)

Sixty malicious Ruby gems containing credential-stealing code have been downloaded over 275,000 times since March 2023, targeting developer accounts.

The malicious Ruby gems were discovered by Socket, which reports they targeted primarily South Korean users of automation tools for Instagram, TikTok, Twitter/X, Telegram, Naver, WordPress, and Kakao.

RubyGems is the official package manager for the Ruby programming language, enabling the distribution, installation, and management of Ruby libraries, known as gems, much like npm for JavaScript or PyPI for Python.

https://www.wiz.io/lp/ai-security-starter-pack?utm_source=bleepingcomputer&utm_medium=display&utm_campaign=FY27Q2_INB_FORM_AI-Security-Starter-Kit&sfcid=701Vh00000cn7aRIAQ&utm_term=FY27Q4-bleepingcomputer-970x250-Nov&utm_content=AI-Security-Bundle

The malicious gems in this campaign were published onto RubyGems.org under various aliases over the years. The offending publishers are zon, nowon, kwonsoonje, and soonje, spreading the activity over multiple accounts to make the activity harder to trace and block.

The full list of the malicious packages can be found in [Socket's report](https://socket.dev/blog/60-malicious-ruby-gems-used-in-targeted-credential-theft-campaign), but below are some notable cases of deceptively named or typosquatted packages:

- WordPress-style automators: wp_posting_duo, wp_posting_zon
- Telegram-style bots: tg_send_duo, tg_send_zon
- SEO/backlink tools: backlink_zon, back_duo
- Blog platform mimics: nblog_duo, nblog_zon, tblog_duopack, tblog_zon
- Naver Café interaction tools: cafe_basics[_duo], cafe_buy[_duo], cafe_bey, *_blog_comment, *_cafe_comment

All 60 gems highlighted in the Socket report present a graphical user interface (GUI) that appears legitimate, as well as the advertised functionality.

In practice, however, they act as phishing tools that exfiltrate the credentials users enter on the login form to the attackers on a hardcoded command-and-control (C2) address (programzon[.]com, appspace[.]kr, marketingduo[.]co[.]kr).

Malicious code snippet present in the 60 gems
Source: Socket

The harvested data includes usernames and passwords in plaintext, device MAC addresses for fingerprinting, and the package name for campaign performance tracking.

In some cases, the tools respond with a fake success or failure message, although no real login or API call to the actual service is made.

Socket has found credential logs on Russian-speaking darknet markets that appear to derive from these gems, based on interactions with marketingduo[.]co[.]kr, a dubious marketing tool site tied to the attacker.

Infostealer logs linked to the campaign
Source: Socket

The researchers say that at least 16 of the 60 malicious Ruby gems remain available, although they have reported them all to the RubyGems team upon discovery.

Supply chain attacks on RubyGems aren't unprecedented, and they have been going on [for several years now](https://www.bleepingcomputer.com/news/security/malicious-rubygems-packages-used-in-cryptocurrency-supply-chain-attack/).

In June, Socket [reported another case](https://www.bleepingcomputer.com/news/security/malicious-rubygems-pose-as-fastlane-to-steal-telegram-api-data/) of malicious Ruby gems that typosquatted Fastlane, a legitimate open-source plugin that serves as an automation tool for mobile app developers, targeting Telegram bot developers specifically.

Developers should scrutinize libraries they source from open-source repositories for signs of suspicious code like obfuscated parts, consider the publisher's reputation and release history, and lock dependencies to 'known to be safe' versions.

https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY

##  [Uncover email threats your existing defenses may miss.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

Adaptive Security's AI agents analyze email intent, context, and behavior to uncover phishing, business email compromise, and invoice fraud.

See what's getting through with a 14-day read-only scan of Microsoft 365 or Google Workspace, with no MX changes.

[Start your free email security scan.](https://www.adaptivesecurity.com/lp/nb/email-scan?utm_source=display_network&utm_medium=paid_display&utm_campaign=2026_NA_Q4_Bleepingcomputer_Inadunit_October2026_scan&utm_id=701Rd00000sqmWCIAY)

### Related Articles:

[LiteLLM release pipeline hacked to distribute info-stealer malware](https://www.bleepingcomputer.com/news/security/litellm-release-pipeline-hacked-to-distribute-info-stealer-malware/)

[Hackers hijack Convertr analytics script to steal payment card data](https://www.bleepingcomputer.com/news/security/hackers-hijack-convertr-analytics-script-to-steal-payment-card-data/)

[WordPress SEO plugin Rank Math compromised to backdoor websites](https://www.bleepingcomputer.com/news/security/wordpress-seo-plugin-rank-math-compromised-to-backdoor-websites/)

[Ledger asks sellers across Asia to halt sales after supply-chain attack](https://www.bleepingcomputer.com/news/security/ledger-asks-sellers-across-asia-to-halt-sales-after-supply-chain-attack/)

[Ledger confirms spy chip implant behind wallet crypto thefts](https://www.bleepingcomputer.com/news/security/ledger-confirms-hardware-wallet-spy-implant-behind-crypto-thefts/)
