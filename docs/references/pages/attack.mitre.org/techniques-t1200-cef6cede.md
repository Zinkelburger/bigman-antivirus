---
url: "https://attack.mitre.org/techniques/T1200/"
title: "Hardware Additions, Technique T1200 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:40:23Z"
via: "live"
sha256: "e7a6935d8e777f64a9fb001fd834bcf24343e5110705bcdb91ce48de6af2ad8c"
wayback: "https://web.archive.org/web/20261023090728/https://attack.mitre.org/techniques/T1200/"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Hardware Additions, Technique T1200 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1200/>
> Snapshot: <https://web.archive.org/web/20261023090728/https://attack.mitre.org/techniques/T1200/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Hardware Additions

#  Hardware Additions

Adversaries may physically introduce computer accessories, networking hardware, or other computing devices into a system or network that can be used as a vector to gain access. Rather than just connecting and distributing payloads via removable storage (i.e. [Replication Through Removable Media](https://attack.mitre.org/techniques/T1091)), more robust hardware additions can be used to introduce new functionalities and/or features into a system that can then be abused.

While public references of usage by threat actors are scarce, many red teams/penetration testers leverage hardware additions for initial access. Commercial and open source products can be leveraged with capabilities such as passive network tapping, network traffic modification (i.e. [Adversary-in-the-Middle](https://attack.mitre.org/techniques/T1557)), keystroke injection, kernel memory reading via DMA, addition of new wireless access points to an existing network, and others.[[1]](https://ossmann.blogspot.com/2011/02/throwing-star-lan-tap.html)[[2]](https://www.youtube.com/watch?v=lDvf4ScWbcQ)[[3]](https://www.youtube.com/watch?v=fXthwl6ShOg)[[4]](https://arstechnica.com/information-technology/2012/03/the-pwn-plug-is-a-little-white-box-that-can-hack-your-network/)

ID: T1200

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Initial Access](https://attack.mitre.org/tactics/TA0001)

ⓘ

Platforms: Linux, Windows, macOS

Version: 1.7

Created: 18 April 2018

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1200/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1200/)

## Procedure Examples

| ID  | Name  | Description

|  [G0105](https://attack.mitre.org/groups/G0105)   |  [DarkVishnya](https://attack.mitre.org/groups/G0105)   |

[DarkVishnya](https://attack.mitre.org/groups/G0105) physically connected Bash Bunny, Raspberry Pi, netbooks, and inexpensive laptops to the target organization's environment to access the company’s local network.[[5]](https://securelist.com/darkvishnya/89169/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1035](https://attack.mitre.org/mitigations/M1035)   |  [Limit Access to Resource Over Network](https://attack.mitre.org/mitigations/M1035)   |

Establish network access control policies, such as using device certificates and the 802.1x standard. [[6]](https://en.wikipedia.org/wiki/IEEE_802.1X) Restrict use of DHCP to registered devices to prevent unregistered devices from communicating with trusted systems.

|  [M1034](https://attack.mitre.org/mitigations/M1034)   |  [Limit Hardware Installation](https://attack.mitre.org/mitigations/M1034)   |

Block unknown devices and accessories by endpoint security configuration and monitoring agent.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0069](https://attack.mitre.org/detectionstrategies/DET0069)   |  [Detect unauthorized or suspicious Hardware Additions (USB/Thunderbolt/Network)](https://attack.mitre.org/detectionstrategies/DET0069)   |  [AN0185](https://attack.mitre.org/detectionstrategies/DET0069#AN0185)   |

Chain: (1) a new external device is recognized by Windows (USB/Thunderbolt/PCIe) or a new block device appears; (2) within a short window, the same user/session spawns processes or the OS mounts a new volume; (3) optional follow-on activity such as HID keystroke injection, DMA driver load, or new network interface MAC on DHCP. Correlate Security EID 6416 / Kernel-PnP with sysmon and DHCP/network metadata.

|  [AN0186](https://attack.mitre.org/detectionstrategies/DET0069#AN0186)   |

Chain: (1) udev / kernel logs show hot-plug (USB/Thunderbolt/PCIe); (2) block device created by udisks/diskarbitration; (3) optional: new network interface or DHCP lease observed. Correlate /var/log/messages|syslog, auditd SYSCALL open/creat on /dev, and DHCP/Zeek.

|  [AN0187](https://attack.mitre.org/detectionstrategies/DET0069#AN0187)   |

Chain: (1) unified logs report IOUSBHost/IOThunderbolt device arrival; (2) diskarbitrationd attaches a new volume; (3) optional: config profile manipulation or new network interface MAC obtains a lease. Correlate unifiedlogs (subsystems: IOUSBHost, IOKit, diskarbitrationd), FSEvents, and DHCP/Zeek.

## References

-    [Michael Ossmann. (2011, February 17). Throwing Star LAN Tap. Retrieved March 30, 2018.](https://ossmann.blogspot.com/2011/02/throwing-star-lan-tap.html)
-    [Nick Aleks. (2015, November 7). Weapons of a Pentester - Understanding the virtual & physical tools used by white/black hat hackers. Retrieved March 30, 2018.](https://www.youtube.com/watch?v=lDvf4ScWbcQ)
-    [Ulf Frisk. (2016, August 5). Direct Memory Attack the Kernel. Retrieved March 30, 2018.](https://www.youtube.com/watch?v=fXthwl6ShOg)

-    [Robert McMillan. (2012, March 3). The Pwn Plug is a little white box that can hack your network. Retrieved March 30, 2018.](https://arstechnica.com/information-technology/2012/03/the-pwn-plug-is-a-little-white-box-that-can-hack-your-network/)
-    [Golovanov, S. (2018, December 6). DarkVishnya: Banks attacked through direct connection to local network. Retrieved May 15, 2020.](https://securelist.com/darkvishnya/89169/)
-    [Wikipedia. (2018, March 30). IEEE 802.1X. Retrieved April 11, 2018.](https://en.wikipedia.org/wiki/IEEE_802.1X)

×
