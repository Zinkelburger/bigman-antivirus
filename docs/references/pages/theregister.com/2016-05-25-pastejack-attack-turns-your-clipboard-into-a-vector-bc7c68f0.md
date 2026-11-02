---
url: "https://www.theregister.com/2016/05/25/pastejack_attack_turns_your_clipboard_into_a_vector/"
title: "Pastejack attack turns your clipboard into a threat"
site: "theregister"
published: "2016-05-25T01:58:44.000Z"
captured: "2026-11-02T19:02:19Z"
via: "live"
source_url: "https://www.theregister.com/security/2016/05/25/pastejack-attack-turns-your-clipboard-into-a-threat/502661"
sha256: "15d48c655680fdcd9360e1a66d44c45d205083720b49e2786cf20857ed049f2f"
wayback: "https://web.archive.org/web/20260411085844/https://www.theregister.com/2016/05/25/pastejack_attack_turns_your_clipboard_into_a_vector/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Pastejack attack turns your clipboard into a threat

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.theregister.com/2016/05/25/pastejack_attack_turns_your_clipboard_into_a_vector/>
> Snapshot: <https://web.archive.org/web/20260411085844/https://www.theregister.com/2016/05/25/pastejack_attack_turns_your_clipboard_into_a_vector/>

[Security](https://www.theregister.com/tag/security)

#  Pastejack attack turns your clipboard into a threat

CTRL-V vector turns "not evil" into "evil" without the target noticing

Richard Chirgwin [Richard Chirgwin](https://www.theregister.com/author/Richard-Chirgwin)

Published wed 25 May 2016 // 02:58 UTC

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661  https://bsky.app/intent/compose?text=Pastejack%20attack%20turns%20your%20clipboard%20into%20a%20threat%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661&title=Pastejack%20attack%20turns%20your%20clipboard%20into%20a%20threat https://api.whatsapp.com/send?text=Pastejack%20attack%20turns%20your%20clipboard%20into%20a%20threat%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661&text=Pastejack%20attack%20turns%20your%20clipboard%20into%20a%20threat [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

Once, you could use HTML/CSS to manipulate the clipboard, but it was not a good way to do so. Now a security bod has worked out how to do it in JavaScript and reckons it's a lot more dangerous.

At first glance, it looks like purely a stunt-attack, except for this: a phishing e-mail purporting to be from tech support could trick a victim into dropping a message into a terminal window and executing it – not realising that what's in the clipboard got changed on the way.

In the simple proof-of-concept of [“Pastjacking” here](https://security.love/Pastejacking/), you think you're copying the command echo “not evil” into the terminal, but what lands in the clipboard is echo “evil”\n.

REG AD

The newline appended to what lands in the clipboard means the user doesn't get the chance to review what appears on the terminal command line: as soon as they paste from the clipboard to the terminal, it executes.

REG AD

Dylan Ayrey, who [published the exploit](https://github.com/dxa4481/Pastejacking) at GitHub, explains: “If a user attempts to copy the text with keyboard shortcuts, i.e. ctrl+c or command+c, an 800ms timer gets set that will override the user's clipboard with malicious code”.

Another example he gives is:

touch ~/.evil
clear
echo "not evil"

“This command will create an evil file in your home directory and clear the terminal out. The victim appears to have the command they intended to copy, nicely pasted into the terminal”, he writes.

Since it's the act of copying that triggers the JavaScript, copying a single letter of a crafted string is sufficient to get the malicious content dropped in the clipboard, and Ayrey notes that the attack can be used to exploit Vim as well. ®

[security](https://www.theregister.com/tag/security)

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661  https://bsky.app/intent/compose?text=Pastejack%20attack%20turns%20your%20clipboard%20into%20a%20threat%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661&title=Pastejack%20attack%20turns%20your%20clipboard%20into%20a%20threat https://api.whatsapp.com/send?text=Pastejack%20attack%20turns%20your%20clipboard%20into%20a%20threat%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2016%2F05%2F25%2Fpastejack-attack-turns-your-clipboard-into-a-threat%2F502661&text=Pastejack%20attack%20turns%20your%20clipboard%20into%20a%20threat [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

REG AD

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

###  TOP STORIES

-

https://www.theregister.com/networks/2026/11/01/landline-phones-will-switch-off-across-britland-in-100-days-no-extensions-openreach/5306673

[#### Landline phones will switch off across Britland in 100 days, no extensions – Openreach](https://www.theregister.com/networks/2026/11/01/landline-phones-will-switch-off-across-britland-in-100-days-no-extensions-openreach/5306673)

-

https://www.theregister.com/ai-and-ml/2026/10/27/anthropic-now-lets-claude-users-jump-out-of-plane-without-parachute/5305807

[#### Anthropic now lets Claude users jump out of plane without parachute](https://www.theregister.com/ai-and-ml/2026/10/27/anthropic-now-lets-claude-users-jump-out-of-plane-without-parachute/5305807)

-

https://www.theregister.com/os-platforms/2026/11/01/linus-torvalds-says-linux-kernel-is-out-of-stabilization-phase-and-using-much-healthier-model-for-ai-contributions/5306852

[#### Linus Torvalds says Linux kernel is out of 'stabilization phase' and using 'much healthier model' for AI contributions](https://www.theregister.com/os-platforms/2026/11/01/linus-torvalds-says-linux-kernel-is-out-of-stabilization-phase-and-using-much-healthier-model-for-ai-contributions/5306852)

-

https://www.theregister.com/ai-and-ml/2026/10/29/google-struggling-to-make-ai-mode-searches-pay/5306181

[#### Google struggling to make AI Mode searches pay](https://www.theregister.com/ai-and-ml/2026/10/29/google-struggling-to-make-ai-mode-searches-pay/5306181)

-

https://www.theregister.com/ai-and-ml/2026/10/26/alibabas-qwen-ai-caught-trying-to-contact-server-in-china/5305561

[#### Alibaba's Qwen AI caught trying to contact server in China](https://www.theregister.com/ai-and-ml/2026/10/26/alibabas-qwen-ai-caught-trying-to-contact-server-in-china/5305561)

-

https://www.theregister.com/science/2026/10/30/voyager-1-gives-controllers-the-silent-treatment/5306637

[#### Voyager 1 gives controllers the silent treatment](https://www.theregister.com/science/2026/10/30/voyager-1-gives-controllers-the-silent-treatment/5306637)

### [AI](https://beta.theregister.com/tag/ai)

-  [#### Marvell wants to be a copper topper, bets $115B on photonics leader to push optical networking

IP house believes optics demand is going to explode as Nvidia and others eye bigger systems](https://www.theregister.com/systems/2026/11/02/marvell-wants-to-be-a-copper-topper-bets-115b-on-photonics-leader-to-push-optical-networking/5307028)
-  [OPINION

#### Microsoft's moolah mountain is more massive than ever. It just doesn't want the people who built it

Flash the cash, flush the cache](https://www.theregister.com/columnists/2026/11/02/microsofts-moolah-mountain-is-more-massive-than-ever-it-just-doesnt-want-the-people-who-built-it/5306576)
-  [#### Amazon sends chills through its workforce with mixed message memo about AI use

A lot of software engineers feel bleak about the future of the profession](https://www.theregister.com/ai-and-ml/2026/10/31/amazon-sends-chills-through-its-workforce-with-mixed-message-memo-about-ai-use-corey-quinn/5306804)
-  [#### Backbone of a beast – charting Nvidia's rise to networking titan

How a $6.9B bet on Mellanox made Nvidia bigger than Cisco, and the world's most valuable company to boot](https://www.theregister.com/networks/2026/10/31/backbone-of-a-beast-charting-nvidias-rise-to-networking-titan/5306499)
-  [#### Domestic droid gone amok? Don't worry, you're covered

Insurance firms look to insulate domestic humanoid robot owners against the inevitable chaos they will bring](https://www.theregister.com/ai-and-ml/2026/10/30/domestic-droid-gone-amok-dont-worry-youre-covered/5306396)

### [Infosec](https://beta.theregister.com/security)

-  [Security

#### Russians are posing as Signal support to launch phishing attacks

PLUS: US takes down Iranian propaganda sites; Marketing company asks 'Why Do We Have Your Information?' And more!](https://www.theregister.com/security/2026/03/22/russians-posing-as-signal-support-to-launch-phishing-raids/5221189)
-  [Security

#### Microsoft patches failed to fix on-prem SharePoint, which is now under zero-day attack

PLUS: China upgrades smartphone surveillance tools; Ring eases anti-snooping stance; and more](https://www.theregister.com/security/2025/07/21/microsoft-warns-on-prem-sharepoint-users-of-a-zero-day/550805)
-  [Black Hat and DEF CON

#### DEF CON Franklin project enlists hackers to harden critical infrastructure

Voting village reports have been so successful, says Jeff Moss, that the whole of DEF CON will now be included](https://www.theregister.com/special-features/2024/08/12/def-con-launches-public-policy-report-volunteer-program/408600)
-  [Security

#### EQT buys majority share in Swiss cybersecurity biz Acronis

Went at equivalent of $3.5B+ valuation for entire firm, though portion sold not specified](https://www.theregister.com/security/2024/08/07/eqt-buys-majority-share-in-cybersecurity-outfit-acronis/853853)
-  [Malware Month

#### Ten years since the first corp ransomware, Mikko Hyppönen sees no end in sight

On the plus side, infosec's a good bet for a long, stable career](https://www.theregister.com/special-features/2024/05/08/10-years-since-the-first-corp-ransomware-and-no-end-in-sight/1259096)

### [FOSS](https://beta.theregister.com/tag/FOSS)

-  [#### AlpineBSD announced – an AI-free FOSS OS

Combining the NetBSD kernel with the Alpine Linux userland](https://www.theregister.com/os-platforms/2026/10/30/alpinebsd-announced-an-ai-free-foss-os/5306589)
-  [#### Ubuntu 27.04 may bring a preview of the long-awaited Core Desktop

Canonical has a long way to catch up, but it knows who it's chasing](https://www.theregister.com/os-platforms/2026/10/29/ubuntu-2704-may-bring-a-preview-of-the-long-awaited-core-desktop/5306426)
-  [#### Systemd adds support for Rust

What's even better than Rust in the Linux kernel? Rust in systemd!](https://www.theregister.com/os-platforms/2026/10/29/systemd-adds-support-for-rust/5306264)
-  [#### FreeBSD founder ports Omarchy to FreeBSD in ten days, ten releases

Daemonarchy is unaffiliated with the Linux original, built with AI help, and could complicate FreeBSD's debate over AI contributions](https://www.theregister.com/os-platforms/2026/10/27/freebsd-founder-ports-omarchy-to-freebsd-in-ten-days-ten-releases/5305446)
-  [#### SparkyOS: Lightweight general-purpose Arch, on a pendrive

Interesting new project from the minds behind SparkyLinux](https://www.theregister.com/os-platforms/2026/10/23/sparkyos-lightweight-general-purpose-arch-on-a-pendrive/5304984)
-  [#### Red Hat helps teach about tech writing

But learns tough lessons about AI in the process](https://www.theregister.com/os-platforms/2026/10/19/red-hat-helps-teach-about-tech-writing/5303868)
