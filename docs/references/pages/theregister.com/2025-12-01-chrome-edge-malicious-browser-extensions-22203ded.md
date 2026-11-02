---
url: "https://www.theregister.com/2025/12/01/chrome_edge_malicious_browser_extensions/"
title: "Browser extensions pushed malware to 4.3M Chrome, Edge users"
site: "theregister"
published: "2025-12-01T19:14:12.000Z"
captured: "2026-11-02T19:04:36Z"
via: "live"
source_url: "https://www.theregister.com/security/2025/12/01/browser-extensions-pushed-malware-to-43m-chrome-edge-users/2720642"
sha256: "68ead63fc7786ad1bb9d5c127158cc72cede8534f4314adb927de59d923f7b30"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Browser extensions pushed malware to 4.3M Chrome, Edge users

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.theregister.com/2025/12/01/chrome_edge_malicious_browser_extensions/>

[Cyber-crime](https://www.theregister.com/tag/cyber-crime)

#  Stealthy browser extensions waited years before infecting 4.3M Chrome, Edge users with backdoors and spyware

And some are still active in the Microsoft Edge store

Jessica Lyons [Jessica Lyons](https://www.theregister.com/author/jessica-lyons)

Published mon 1 Dec 2025 // 19:14 UTC

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642  https://bsky.app/intent/compose?text=Browser%20extensions%20pushed%20malware%20to%204.3M%20Chrome%2C%20Edge%20users%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642&title=Browser%20extensions%20pushed%20malware%20to%204.3M%20Chrome%2C%20Edge%20users https://api.whatsapp.com/send?text=Browser%20extensions%20pushed%20malware%20to%204.3M%20Chrome%2C%20Edge%20users%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642&text=Browser%20extensions%20pushed%20malware%20to%204.3M%20Chrome%2C%20Edge%20users [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

A seven-year malicious browser extension campaign infected 4.3 million Google Chrome and Microsoft Edge users with malware, including backdoors and spyware sending people's data to servers in China. And, according to Koi researchers, five of the extensions with more than 4 million installs are still live in the Edge marketplace.

The attackers, which Koi named ShadyPanda, played the long game: publishing legitimate extensions, accumulating thousands or sometimes millions of downloads over several years, and then pushing a malware-laden update that auto updates across the entire user base.

Because both marketplaces review extensions upon submission – it's not an ongoing process – these seemingly stellar productivity tools, some with Featured and Verified status alongside glowing user reviews and high install counts, were allowed to track people's behavior and steal sensitive info silently for years.

REG AD

"No phishing. No social engineering. Just trusted extensions with quiet version bumps that turned productivity tools into surveillance platforms," the threat hunting team [said](https://www.koi.ai/blog/4-million-browsers-infected-inside-shadypanda-7-year-malware-campaign) in a Monday blog.

REG AD

Microsoft did not respond to The Register's requests for comment. A Google spokesperson confirmed none of the extensions are available on the Chrome Web Store, and we are aware that Google screens every single update to extensions in the Chrome store, no matter how minor the change.

Koi tracked ShadyPanda's activity in multiple phases, and says two campaigns are still active.

One of these campaigns included five extensions that infected 300,000 users with a remote-code-execution enabling backdoor. Three of the five were uploaded between 2018 and 2019 and achieved Featured and Verified status. One of those extensions, called Clean Master and published by Starlab Technology, has more than 200,000 installs.

In mid-2024, after being downloaded more than 300,000 times, ShadyPanda pushed a malicious update containing a backdoor across all five running on Chrome and Edge. While the extensions have since been removed from both marketplaces, "the infrastructure for full-scale attacks remains deployed on all infected browsers," the researchers wrote.

The malware allows complete browser surveillance, checking api.extensionplay[.]com for new instructions every hour, downloading arbitrary JavaScript, and executing it with full browser API access. It can also inject malicious content into any website, including HTTPS connections.

Clean Master then sends all of this stolen data - every URL visited, HTTP referrers showing navigation patterns, timestamps for activity profiling, persistent UUID4 identifiers, and complete browser fingerprints - to ShadyPanda-controlled servers.

Plus, the malware contains anti-analysis capabilities and switches to benign behavior if a researcher opens developer tools.

An additional five extensions from the same publisher launched on Edge around 2023 and now have more than four million combined installs. According to Koi, all five are still live on the Edge marketplace, and two of these install spyware on users' machines.

REG AD

One of these five, WeTab, has three million installs. It's a surveillance platform disguised as a productivity tool that snarfs all sorts of user data: every URL visited, search queries, mouse-click tracking, browser fingerprinting, page interaction data, and storage access – and then sends all of this, in real time, to 17 different domains (8 Baidu servers in China, 7 WeTab servers in China, and Google Analytics).

"The extension already has dangerous permissions including access to all URLs and cookies, users are downloading them right now," the researchers wrote. "ShadyPanda can push updates at any time, weaponizing 4 million browsers with the same RCE backdoor framework [from Clean Master] or something even worse."

## MORE CONTEXT

-  [### Compromised Amazon Q extension told AI to delete everything – and it shipped](https://www.theregister.com/2025/07/24/amazon_q_ai_prompt/)
-  [### Your AI conversations are a secret new treasure trove for marketers](https://www.theregister.com/2025/09/29/profound_browser_extension_privacy_concern/)
-  [### Devs are writing VS Code extensions that blab secrets by the bucketload](https://www.theregister.com/2025/10/15/vc_code_extension_leaks/)
-  [### PostHog admits Shai-Hulud 2.0 was its biggest ever security bungle](https://www.theregister.com/2025/11/28/posthog_shaihulud/)

Koi also traced ShadyPanda to a couple of earlier, now inactive, campaigns. One of these, which occurred during 2023, included 20 Chrome Web Store extensions and 125 on Microsoft Edge, all disguised as wallpaper or productivity apps.

This one worked by silently tracking and monetizing users' browsing data. When someone clicked on eBay, Amazon, or [Booking.com](http://booking.com), the extensions injected affiliate tracking codes and Google Analytics trackers, which were then logged and used to sell people's website visits and search queries.

A second inactive campaign from early 2023 was also disguised as a new tab productivity tool called Infinity V+. It redirected every user's search to browser hijacking website [trovi.com](http://trovi.com), exfiltrated cookies, and logged users' keystrokes in the search box, sending all of this info to external servers.

According to the researchers, all of these ShadyPanda campaigns illustrate a problem in the way marketplaces manage extensions. "They don't watch what happens after approval," they wrote. ®

## Updated to add at 23:10 GMT on December 2, 2025

After the story posted, a Microsoft spokesperson told us: “We have removed all the extensions identified as malicious on the Edge Add-on store. When we become aware of instances that violate our policies, we take appropriate action that includes, but is not limited to, the removal of prohibited content or termination of our publishing agreement.”

[chrome](https://www.theregister.com/tag/chrome) [web browser](https://www.theregister.com/tag/web%20browser) [security](https://www.theregister.com/tag/security) [microsoft edge](https://www.theregister.com/tag/microsoft%20edge) [cybercrime](https://www.theregister.com/tag/cybercrime) [cyber-crime](https://www.theregister.com/tag/cyber-crime)

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642  https://bsky.app/intent/compose?text=Browser%20extensions%20pushed%20malware%20to%204.3M%20Chrome%2C%20Edge%20users%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642&title=Browser%20extensions%20pushed%20malware%20to%204.3M%20Chrome%2C%20Edge%20users https://api.whatsapp.com/send?text=Browser%20extensions%20pushed%20malware%20to%204.3M%20Chrome%2C%20Edge%20users%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F12%2F01%2Fbrowser-extensions-pushed-malware-to-43m-chrome-edge-users%2F2720642&text=Browser%20extensions%20pushed%20malware%20to%204.3M%20Chrome%2C%20Edge%20users [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

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
