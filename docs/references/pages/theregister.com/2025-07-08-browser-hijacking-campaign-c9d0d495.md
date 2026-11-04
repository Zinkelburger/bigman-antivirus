---
url: "https://www.theregister.com/2025/07/08/browser_hijacking_campaign/"
title: "Browser hijacking campaign infects 2.3M Chrome, Edge users"
site: "theregister"
published: "2025-07-08T19:07:27.000Z"
captured: "2026-11-02T19:03:33Z"
via: "live"
source_url: "https://www.theregister.com/security/2025/07/08/browser-hijacking-campaign-infects-23m-chrome-edge-users/1236258"
sha256: "7ad9fd43591873070fa68a5937450209646b7b0b6afca0c8982b6f8f105223e6"
wayback: "https://web.archive.org/web/20260302012238/https://www.theregister.com/2025/07/08/browser_hijacking_campaign/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Browser hijacking campaign infects 2.3M Chrome, Edge users

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.theregister.com/2025/07/08/browser_hijacking_campaign/>
> Snapshot: <https://web.archive.org/web/20260302012238/https://www.theregister.com/2025/07/08/browser_hijacking_campaign/>

[Research](https://www.theregister.com/tag/research)

#  Massive browser hijacking campaign infects 2.3M Chrome, Edge users

These extensions weren't malware-laced from the start, researcher says

Jessica Lyons [Jessica Lyons](https://www.theregister.com/author/jessica-lyons)

Published tue 8 Jul 2025 // 20:07 UTC

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258  https://bsky.app/intent/compose?text=Browser%20hijacking%20campaign%20infects%202.3M%20Chrome%2C%20Edge%20users%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258&title=Browser%20hijacking%20campaign%20infects%202.3M%20Chrome%2C%20Edge%20users https://api.whatsapp.com/send?text=Browser%20hijacking%20campaign%20infects%202.3M%20Chrome%2C%20Edge%20users%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258&text=Browser%20hijacking%20campaign%20infects%202.3M%20Chrome%2C%20Edge%20users [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

UPDATED A Chrome and Edge extension with more than 100,000 downloads that displays Google's verified badge does what it purports to do: It delivers a color picker to users. Unfortunately, it also hijacks every browser session, tracks activities across websites, and backdoors victims' web browsers, according to Koi Security researchers.

Color pickers let users select any color from a website and copy it into a clipboard for later use - helpful for designing apps, websites, and the like. This particular extension from Geco is still available for download via both [Microsoft's](https://microsoftedge.microsoft.com/addons/detail/color-picker-tool-geco/cinphehlcbllinepbccomahegigmgjja) and [Google's](https://chromewebstore.google.com/detail/colour-picker-tool-geco/eokjikchkppnkdipbiggnmlkahcdkikp) respective stores at press time. Following publication, Microsoft subsequently took steps to disable these extensions and remove them from its store; Google has not yet responded.

The Geco extension has more than 800 reviews on the Chrome Web Store, 4.2 stars (out of 5), and "featured" placement. Microsoft's Edge Add-ons shows similarly glowing write-ups from its 1,000-plus users, and it looks like a perfectly safe extension.

REG AD

"This isn't some obvious scam extension thrown together in a weekend," [said](https://blog.koi.security/google-and-microsoft-trusted-them-2-3-million-users-installed-them-they-were-malware-fb4ed4f40ff5) Koi Security analyst Idan Dardikman in a Tuesday blog. "This is a carefully crafted Trojan horse."

REG AD

The Register also reached out to the developer for comment but did not receive a response.

The Geco color picker, according to Koi Security, is "just the tip of the iceberg," and part of a much larger browser-hijacking campaign dubbed RedDirection. The campaign consists of 18 malicious extensions spanning both Chrome and Edge stores that all share the same snooping capabilities. All 18 extensions are listed at the bottom of this story.

"Combined, these eighteen extensions have infected over 2.3 million users across both browsers, creating one of the largest browser hijacking operations we've documented," Dardikman wrote.

## MORE CONTEXT

-  [### Netflix, Apple, BofA websites hijacked with fake help-desk numbers](https://www.theregister.com/2025/06/20/netflix_apple_bofa_websites_hijacked/)
-  [### Sneaky SnakeKeylogger slithers into Windows inboxes to steal sensitive secrets](https://www.theregister.com/2024/08/05/snakekeylogger_malware_windows/)
-  [### Chrome users – get an alert when extensions are in danger of falling into wrong hands](https://www.theregister.com/2024/03/07/chrome_extension_changes/)
-  [### Suspected Scattered Spider domains target everyone from manufacturers to Chipotle](https://www.theregister.com/2025/07/08/suspected_scattered_spider_domains_target/)

The extensions offer all sorts of capabilities: emoji keyboards, weather forecasts, video speed controllers, VPN proxies for Discord and TikTok, dark themes, volume boosters, and YouTube unblockers (useful if your employer, school, or government blocks the popular video site). But in addition to providing these legitimate functions, they secretly surveil users' web browsing activity, capturing URLs, sending this info to a remote attacker-controlled server along with the victim's unique tracking ID, and even redirecting people's browsers if instructed, according to the researchers.

What makes this even sneakier — and likely explains the Google verified badge — is that these extensions weren't laced with malware from the start.

comme

According to Dardikman, the code started out clean and sometimes remained that way for years before the malware was introduced during version updates. "Due to how Google and Microsoft handle browser extension updates, these malicious versions auto-installed silently for over 2.3 million users across both platforms, most of whom never clicked anything," he said.

If you've installed any of the extensions listed below, uninstall now, clear your browser data, and keep an eye on your accounts for any suspicious activity.

REG AD

"No phishing. No social engineering. Just trusted extensions with quiet version bumps that turned productivity tools into surveillance malware," the blog warns. ®

##

Extension IDs

Chrome:

- kgmeffmlnkfnjpgmdndccklfigfhajen — [Emoji keyboard online — copy&past your emoji.]
- dpdibkjjgbaadnnjhkmmnenkmbnhpobj — [Free Weather Forecast]
- gaiceihehajjahakcglkhmdbbdclbnlf — [Video Speed Controller — Video manager]
- mlgbkfnjdmaoldgagamcnommbbnhfnhf — [Unlock Discord — VPN Proxy to Unblock Discord Anywhere]
- eckokfcjbjbgjifpcbdmengnabecdakp — [Dark Theme — Dark Reader for Chrome]
- mgbhdehiapbjamfgekfpebmhmnmcmemg — [Volume Max — Ultimate Sound Booster]
- cbajickflblmpjodnjoldpiicfmecmif — [Unblock TikTok — Seamless Access with One-Click Proxy]
- pdbfcnhlobhoahcamoefbfodpmklgmjm — [Unlock YouTube VPN]
- eokjikchkppnkdipbiggnmlkahcdkikp — [Color Picker, Eyedropper — Geco colorpick]
- ihbiedpeaicgipncdnnkikeehnjiddck — [Weather]

Edge:

- jjdajogomggcjifnjgkpghcijgkbcjdi — [Unlock TikTok]
- mmcnmppeeghenglmidpmjkaiamcacmgm — [Volume Booster — Increase your sound]
- ojdkklpgpacpicaobnhankbalkkgaafp — [Web Sound Equalizer]
- lodeighbngipjjedfelnboplhgediclp — [Header Value]
- hkjagicdaogfgdifaklcgajmgefjllmd — [Flash Player — games emulator]
- gflkbgebojohihfnnplhbdakoipdbpdm — [Youtube Unblocked]
- kpilmncnoafddjpnbhepaiilgkdcieaf — [SearchGPT — ChatGPT for Search Engine]
- caibdnkmpnjhjdfnomfhijhmebigcelo — [Unlock Discord]

Updated 18:42 GMT on July 10: Microsoft wrote us to tell us "These extensions have been removed from the store and disabled for Edge browser users. Microsoft’s security software will also block the download of these extensions." No word from Google yet.

[web browser](https://www.theregister.com/tag/web%20browser) [cybercrime](https://www.theregister.com/tag/cybercrime) [malware](https://www.theregister.com/tag/malware) [research](https://www.theregister.com/tag/research) [security](https://www.theregister.com/tag/security)

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258  https://bsky.app/intent/compose?text=Browser%20hijacking%20campaign%20infects%202.3M%20Chrome%2C%20Edge%20users%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258&title=Browser%20hijacking%20campaign%20infects%202.3M%20Chrome%2C%20Edge%20users https://api.whatsapp.com/send?text=Browser%20hijacking%20campaign%20infects%202.3M%20Chrome%2C%20Edge%20users%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F07%2F08%2Fbrowser-hijacking-campaign-infects-23m-chrome-edge-users%2F1236258&text=Browser%20hijacking%20campaign%20infects%202.3M%20Chrome%2C%20Edge%20users [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

[## Marvell wants to be a copper topper, bets $115B on photonics leader to push optical networking

IP house believes optics demand is going to explode as Nvidia and others eye bigger systems](https://www.theregister.com/systems/2026/11/02/marvell-wants-to-be-a-copper-topper-bets-115b-on-photonics-leader-to-push-optical-networking/5307028)

[## Oracle gobbles $20 billion borrowing limit a week after setting it

In the middle of a datacenter capex binge, vendor promises no more borrowing until the end of 2030](https://www.theregister.com/paas-and-iaas/2026/11/02/oracle-gobbles-20-billion-borrowing-limit-a-week-after-setting-it/5307014)

[## From expertise to intelligence: NISCO deploys world's first swarm-intelligence steel model at scale

PARTNER CONTENT: 300+ AI applications delivering surges in core metrics and annual cost reduction of over CNY100m](https://www.theregister.com/ai-and-ml/2026/11/02/partner-content-from-expertise-to-intelligence-nisco-deploys-worlds-first-swarm-intelligence-steel-model-at-scale/5306614)

[## Ericsson says it can detect, track drones using only mobile networks

ISAC technology adds detection ability to existing cell networks, enabling them to track low-flying small drones](https://www.theregister.com/networks/2026/11/02/ericsson-says-it-can-detect-track-drones-using-only-mobile-networks/5306958)

[OPINION

## Microsoft's moolah mountain is more massive than ever. It just doesn't want the people who built it

Flash the cash, flush the cache](https://www.theregister.com/columnists/2026/11/02/microsofts-moolah-mountain-is-more-massive-than-ever-it-just-doesnt-want-the-people-who-built-it/5306576)

[## Thieves pilfer environment variables from Trigger AI workflow tool

So, who's ready to clear the dust out of that server closet and move things back on-prem?](https://www.theregister.com/security/2026/11/02/thieves-pilfer-environment-variables-from-trigger-ai-workflow-tool/5306998)
