---
url: "https://www.theregister.com/2024/03/28/canonical_snap_store_scams/"
title: "Canonical to manually approve new Snap package names"
site: "theregister"
published: "2024-03-28T11:45:13.000Z"
captured: "2026-11-02T19:02:34Z"
via: "live"
source_url: "https://www.theregister.com/security/2024/03/28/canonical-to-manually-approve-new-snap-package-names/1127641"
sha256: "7da4e48fac3c791e42649272a4fda6ff1cdce9f15de63b19a7d50add9df9bdef"
wayback: "https://web.archive.org/web/20260331103627/https://www.theregister.com/2024/03/28/canonical_snap_store_scams/"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Canonical to manually approve new Snap package names

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.theregister.com/2024/03/28/canonical_snap_store_scams/>
> Snapshot: <https://web.archive.org/web/20260331103627/https://www.theregister.com/2024/03/28/canonical_snap_store_scams/>

[Security](https://www.theregister.com/tag/security)

#  Canonical cracks down on crypto cons following Snap Store scam spree

In happier news, Ubuntu Pro extended support now goes up to 12 years

Liam Proven [Liam Proven](https://www.theregister.com/author/liam-proven)

Published thu 28 Mar 2024 // 11:45 UTC

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641  https://bsky.app/intent/compose?text=Canonical%20to%20manually%20approve%20new%20Snap%20package%20names%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641&title=Canonical%20to%20manually%20approve%20new%20Snap%20package%20names https://api.whatsapp.com/send?text=Canonical%20to%20manually%20approve%20new%20Snap%20package%20names%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641&text=Canonical%20to%20manually%20approve%20new%20Snap%20package%20names [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

After multiple waves of cryptocurrency credential-stealing apps were uploaded to the Snap store, Canonical is changing its policies.

In what's expected to be a temporary measure, an announcement on the Snapcraft Discourse says that the Ubuntu vendor will switch to [manual review of all new snap name registrations](https://forum.snapcraft.io/t/manual-review-of-all-new-snap-name-registrations/39440/1). The post from [Holly Hall](https://ubuntu.com/blog/author/hollyhall) says:

##

Over the recent weeks, the Snap Store has been under attack by various publishers attempting to upload malicious crypto-wallet snaps.

Former Ubuntu staffer Alan "Popey" Pope has detailed some of this wave of scammy apps. Firstly, a fake "Exodus" wallet app [caused one user to lose nine Bitcoins](https://popey.com/blog/2024/02/exodus-bitcoin-wallet-490k-swindle/) – currently worth roughly $490,000. This scam app from movementexod was masquerading as the real Exodus wallet from Exodus Movement – a company that already offers native Debian packages alongside its Windows and Mac versions.

REG AD

A few days later, Popey published a [follow-up post](https://popey.com/blog/2024/02/exodus-bitcoin-wallet-follow-up/) including the response of Canonical founder and "self-appointed benevolent dictator for life" Mark Shuttleworth. You can read the [discussions](https://forum.snapcraft.io/t/should-unverified-cryptocurrency-apps-be-banned/38919/1) of the [problem](https://forum.snapcraft.io/t/stronger-identity-verification-for-all-publishers/39061) on the Snapcraft forums. (If you are inclined to join in, note that the Snapcraft Discourse instance has its own membership system, separate from the [official Ubuntu instance](https://discourse.ubuntu.com/).)

REG AD

The saga did not end there. This month, he detailed [ten more fake wallets](https://popey.com/blog/2024/03/exodus-wallet-part-three/) that appeared on the Snap Store, from an account called digisafe00000. When these were removed, the next day, replacements reappeared from [a new account called codeshield0x0000](https://forum.snapcraft.io/t/stop-the-line/39357/1). Popey also did some digging into how they work. Worse still, these fake wallets aren't the only ones – he has a [Mastodon thread](https://mastodon.social/@popey/112118758628030881) detailing others.

Part of the problem is that these apps look legitimate to casual inspection because the Snap Store badges them as "safe." Unfortunately, that just means the apps are strictly confined in a sandbox, but for the scammers' purposes, that doesn't matter. The apps look like the real thing and use simple social engineering to extract users' credentials, which they then use to siphon off the victim's funds. But the apps can't touch your system, so by Snap confinement rules, they're "safe."

This sort of problem is nothing new, and was [satirized by Xkcd](https://xkcd.com/1200/) over a decade ago. The moves also parallel [recent changes at Flathub](https://docs.flathub.org/blog/improved-build-validation/). This isn't the first time that Canonical has faced problems with [cryptocurrency apps in the Snap Store](https://www.theregister.com/2018/05/14/ubuntu_crypto_mining_apps/) either.

## MORE CONTEXT

-  [### Good news: HMRC offers a Linux version of Basic PAYE Tools. Bad news: It broke](https://www.theregister.com/2024/03/26/hmrc_linux_paye_tools/)
-  [### FreeBSD Foundation hands out Beacon gongs for safer software](https://www.theregister.com/2024/03/26/beacon_awards_freebsd/)
-  [### First release candidate of Linux kernel 6.9 looks 'fairly normal,' says Torvalds](https://www.theregister.com/2024/03/25/kernel_6_9_rc1/)
-  [### Linux kernel 4.14 gets a life extension, thanks to OpenELA](https://www.theregister.com/2024/03/19/kernel_414_life_extension/)

The Reg FOSS desk takes an extremely jaundiced view of this. For us, all cryptocurrencies and associated apps of any form are scams, with no exceptions, as are all forms of non-fungible token trading and [absolutely anything to do with "Web 3."](https://www.theregister.com/2021/12/15/web3_apparently_the_next_generation/) The main practical use case for this pretend internet money is for criminals to extort ransomware victims and other hapless sorts. If you retain any doubt about this whatsoever, we recommend reading the excellent [Attack of the 50 Foot Blockchain](https://davidgerard.co.uk/blockchain/book/). If one maintains this attitude, it provides complete and reliable protection from all wallet scams.

## In other news

It's not all bad news in Ubuntopia, though. The next LTS release, 24.04 "Noble Numbat," has not quite reached beta stage yet, although you can [look at its wallpapers](https://ubuntu.com/blog/the-coronation-of-a-new-mascot-noble-numbat) if you like. The release of "Noble" will mark a full decade since the release of [Ubuntu 14.04 "Trusty Tahr."](https://www.theregister.com/2014/04/22/ubuntu_14_04_review/) That in turn meant that "Trusty" was about to hit the end of the [ten years of patches](https://www.theregister.com/2023/10/26/ubuntu_10y_kernel_life/) you get with Ubuntu Pro's [Expanded Security Maintenance](https://ubuntu.com/security/esm).

However, Canonical [just announced](https://ubuntu.com//blog/canonical-expands-long-term-support-to-12-years-starting-with-ubuntu-14-04-lts) an extension of that ten years to 12, meaning that Pro customers get another two years of critical security updates for "Trusty." As we [reported in late 2022](https://www.theregister.com/2022/10/06/ubuntu_pro_free/), Ubuntu Pro is free for up to five machines now – so you can keep a few essential "Trusty" instances going for a little longer, if you absolutely must. ®

[canonical](https://www.theregister.com/tag/canonical) [operating system](https://www.theregister.com/tag/operating%20system) [open source](https://www.theregister.com/tag/open%20source) [linux](https://www.theregister.com/tag/linux) [ubuntu](https://www.theregister.com/tag/ubuntu) [security](https://www.theregister.com/tag/security)

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641  https://bsky.app/intent/compose?text=Canonical%20to%20manually%20approve%20new%20Snap%20package%20names%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641&title=Canonical%20to%20manually%20approve%20new%20Snap%20package%20names https://api.whatsapp.com/send?text=Canonical%20to%20manually%20approve%20new%20Snap%20package%20names%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F03%2F28%2Fcanonical-to-manually-approve-new-snap-package-names%2F1127641&text=Canonical%20to%20manually%20approve%20new%20Snap%20package%20names [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

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
