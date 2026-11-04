---
url: "https://www.theregister.com/2024/07/21/crowdstrike_linux_crashes_restoration_tools/"
title: "CrowdStrike’s Falcon Sensor linked to Linux crashes, too"
site: "theregister"
published: "2024-07-21T23:51:18.000Z"
captured: "2026-11-02T19:03:30Z"
via: "live"
source_url: "https://www.theregister.com/security/2024/07/22/crowdstrikes-falcon-sensor-linked-to-linux-crashes-too/677830"
sha256: "8ba53af9a2864190255b759b62d39f6fc101689ab3a37ad0dd5171b83e4b9172"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# CrowdStrike’s Falcon Sensor linked to Linux crashes, too

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.theregister.com/2024/07/21/crowdstrike_linux_crashes_restoration_tools/>

[Security](https://www.theregister.com/tag/security)

#  CrowdStrike's Falcon Sensor also linked to Linux kernel panics and crashes

Rapid restore tool being tested as Microsoft estimates 8.5M machines went down

Simon Sharwood [Simon Sharwood](https://www.theregister.com/author/simon-sharwood)

Published mon 22 Jul 2024 // 00:51 UTC

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830  https://bsky.app/intent/compose?text=CrowdStrike%E2%80%99s%20Falcon%20Sensor%20linked%20to%20Linux%20crashes%2C%20too%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830&title=CrowdStrike%E2%80%99s%20Falcon%20Sensor%20linked%20to%20Linux%20crashes%2C%20too https://api.whatsapp.com/send?text=CrowdStrike%E2%80%99s%20Falcon%20Sensor%20linked%20to%20Linux%20crashes%2C%20too%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830&text=CrowdStrike%E2%80%99s%20Falcon%20Sensor%20linked%20to%20Linux%20crashes%2C%20too [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

UPDATED CrowdStrike's now-infamous Falcon Sensor software, which last week led to widespread outages of Windows-powered computers, has also been linked to crashes of Linux machines.

Red Hat in June [warned](https://access.redhat.com/solutions/7068083) its customers of a problem it described as a "kernel panic observed after booting 5.14.0-427.13.1.el9_4.x86_64 by falcon-sensor process" that impacted some users of Red Hat Enterprise Linux 9.4 after (as the warning suggests) booting on kernel version 5.14.0-427.13.1.el9_4.x86_64.

A [second issue](https://access.redhat.com/solutions/7068083) titled "system crashed at cshook_network_ops_inet6_sockraw_release+0x171a9" offered users "assistance with troubleshooting potential issues with the falcon_lsm_serviceable kernel module provided from the CrowdStrike Falcon Sensor/Agent security software suite."

REG AD

Red Hat also advised that "disabling the CrowdStrike Falcon Sensor/Agent software suite … will mitigate the crashes and provide temporary stability to the system in question while the issue is investigated." The issue was "observed but not limited to release 6 and 7."

REG AD

We've also spotted reports of CrowdStrike being suspected of [causing problems](https://news.ycombinator.com/item?id=41005936) in [Debian](https://lists.debian.org/debian-kernel/2024/04/msg00202.html) and [Rocky Linux](https://forums.rockylinux.org/t/crowdstrike-freezing-rockylinux-after-9-4-upgrade/14041).

Linux kernel panics and Windows Blue Screens of Death are broadly comparable. The occurrence of kernel panics mere weeks before CrowdStrike broke many Windows implementations therefore hints at wider issues at the security vendor.

The Register has asked CrowdStrike to comment on the issues identified by Red Hat, and will update this story if we receive substantial information.

##

CrowdStrike's CEO oversaw very similar McAfee meltdown

In 2010, PCs around the world [crashed](https://www.theregister.com/2010/04/21/mcafee_false_positive/) after antivirus vendor McAfee pushed a bad update that left PCs in an endless reboot cycle.

At the time, McAfee's chief technology officer was George Kurtz – who now serves as CEO of CrowdStrike.

Kurtz therefore has the possibly unique and almost-certainly-unwanted distinction of having presided over two major global outage events caused by bad software updates.

## Rapid restore tool on the way

CrowdStrike on Sunday [teased](https://www.linkedin.com/posts/crowdstrike_falcon-content-update-remediation-and-guidance-activity-7220896892623220737-lq8Y/) a rapid recovery tool for the mess it made.

"Together with customers, we tested a new technique to accelerate impacted system remediation," the security vendor stated on LinkedIn, adding "We're in the process of operationalizing an opt-in to this technique. We're making progress by the minute."

That progress will likely be of great interest, as Microsoft veep for enterprise and OS security David Weston on Saturday [estimated](https://blogs.microsoft.com/blog/2024/07/20/helping-our-customers-through-the-crowdstrike-outage/) that 8.5 million Windows machines had been laid low by the problem. That's less than one percent of all Windows devices in operation, though a lot of the ones affected obviously were in critical environments.

Microsoft also created a repair tool that runs from a bootable USB storage device and can be found [here](https://techcommunity.microsoft.com/t5/intune-customer-success/new-recovery-tool-to-help-with-crowdstrike-issue-impacting/ba-p/4196959), along with instructions for use. Those instructions were modified on Sunday to require a full wipe of the USB device "so it doesn't error out when used in the recovery process."

REG AD

CrowdStrike [published](https://www.crowdstrike.com/blog/falcon-update-for-windows-hosts-technical-details/) some technical details of the incident. It has also offered [guidance](https://www.crowdstrike.com/falcon-content-update-remediation-and-guidance-hub/) on how to recover Windows machines encrypted with BitLocker.

Former Microsoft operating system developer David Plummer has shared his dissection of the flawed CrowdStrike update [here](https://www.youtube.com/watch?v=wAzEJxOo1ts).

## Up in the air

## MORE CONTEXT

-  [### CrowdStrike shares sink as global IT outage savages systems worldwide](https://www.theregister.com/2024/07/19/crowdstrike_shares_sink_as_global/)
-  [### Cybercriminals quickly exploit CrowdStrike chaos](https://www.theregister.com/2024/07/19/cyber_criminals_quickly_exploit_crowdstrike/)
-  [### Angry admins share the CrowdStrike outage experience](https://www.theregister.com/2024/07/19/admin_crowdstrike_update_mess/)
-  [### Life, interrupted: How CrowdStrike's patch failure is messing up the world](https://www.theregister.com/2024/07/19/life_interrupted_how_crowdstrikes_patch/)

The extent of disruption caused by CrowdStrike remains uncertain, but we've read accounts of [over 6,800](https://simpleflying.com/6855-flights-canceled-it-outage/) flights cancelled last Friday alone, and of some airlines only restoring systems [on Sunday evening](https://www.jetstar.com/au/en/travel-alerts#Update%20on%20global%20software%20issue).

The British Medical Association has [warned](https://www.bma.org.uk/bma-media-centre/gps-will-take-time-to-recover-from-crowdstrike-outage-says-bma) that "normal service cannot be resumed immediately" by UK doctors, at least, due to the backlog caused by the outage.

Australia's home affairs minister Claire O'Neill has warned that remediation could take weeks.

This remains a developing story: The Register will update this item, or write others, as further info emerges. ®

## Updated to add on July 24

REG AD

We understand now that CrowdStrike's software on Linux crashed due to [a kernel bug](https://news.ycombinator.com/item?id=41040278) involving BPF, which will need to be patched as per advisories from distro makers. Falcon Sensor code running at the kernel level was not affected; code at the user level using BPF to do its work was affected.

[crowdstrike](https://www.theregister.com/tag/crowdstrike) [linux](https://www.theregister.com/tag/linux) [outage](https://www.theregister.com/tag/outage) [security](https://www.theregister.com/tag/security)

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830  https://bsky.app/intent/compose?text=CrowdStrike%E2%80%99s%20Falcon%20Sensor%20linked%20to%20Linux%20crashes%2C%20too%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830&title=CrowdStrike%E2%80%99s%20Falcon%20Sensor%20linked%20to%20Linux%20crashes%2C%20too https://api.whatsapp.com/send?text=CrowdStrike%E2%80%99s%20Falcon%20Sensor%20linked%20to%20Linux%20crashes%2C%20too%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2024%2F07%2F22%2Fcrowdstrikes-falcon-sensor-linked-to-linux-crashes-too%2F677830&text=CrowdStrike%E2%80%99s%20Falcon%20Sensor%20linked%20to%20Linux%20crashes%2C%20too [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

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
