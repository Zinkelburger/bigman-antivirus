---
url: "https://attack.mitre.org/techniques/T1498/002/"
title: "Network Denial of Service: Reflection Amplification, Sub-technique T1498.002 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:45:20Z"
via: "live"
sha256: "83fad03310f7ecfd2a1489eebad05f93a14e0f9770ed13fb286a0cd5ca6f2da0"
wayback: "https://web.archive.org/web/20261102192038/https://attack.mitre.org/techniques/T1498/002/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Network Denial of Service: Reflection Amplification, Sub-technique T1498.002 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1498/002/>
> Snapshot: <https://web.archive.org/web/20261102192038/https://attack.mitre.org/techniques/T1498/002/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Network Denial of Service](https://attack.mitre.org/techniques/T1498)
- Reflection Amplification

#  Network Denial of Service: Reflection Amplification

##### Other sub-techniques of Network Denial of Service (2)

| ID  | Name

|  [T1498.001](https://attack.mitre.org/techniques/T1498/001/)   |  [Direct Network Flood](https://attack.mitre.org/techniques/T1498/001/)

|  T1498.002   |  Reflection Amplification

Adversaries may attempt to cause a denial of service (DoS) by reflecting a high-volume of network traffic to a target. This type of Network DoS takes advantage of a third-party server intermediary that hosts and will respond to a given spoofed source IP address. This third-party server is commonly termed a reflector. An adversary accomplishes a reflection attack by sending packets to reflectors with the spoofed address of the victim. Similar to Direct Network Floods, more than one system may be used to conduct the attack, or a botnet may be used. Likewise, one or more reflectors may be used to focus traffic on the target.[[1]](https://blog.cloudflare.com/reflections-on-reflections/) This Network DoS attack may also reduce the availability and functionality of the targeted system(s) and network.

Reflection attacks often take advantage of protocols with larger responses than requests in order to amplify their traffic, commonly known as a Reflection Amplification attack. Adversaries may be able to generate an increase in volume of attack traffic that is several orders of magnitude greater than the requests sent to the amplifiers. The extent of this increase will depending upon many variables, such as the protocol in question, the technique used, and the amplifying servers that actually produce the amplification in attack volume. Two prominent protocols that have enabled Reflection Amplification Floods are DNS[[2]](https://www.cloudflare.com/learning/ddos/dns-amplification-ddos-attack/) and NTP[[3]](https://www.cloudflare.com/learning/ddos/ntp-amplification-ddos-attack/), though the use of several others in the wild have been documented.[[4]](https://pages.arbornetworks.com/rs/082-KNA-087/images/13th_Worldwide_Infrastructure_Security_Report.pdf) In particular, the memcache protocol showed itself to be a powerful protocol, with amplification sizes up to 51,200 times the requesting packet.[[5]](https://blog.cloudflare.com/memcrashed-major-amplification-attacks-from-port-11211/)

ID: T1498.002

Sub-technique of:  [T1498](https://attack.mitre.org/techniques/T1498)

ⓘ

Tactic: [Impact](https://attack.mitre.org/tactics/TA0040)

ⓘ

Platforms: IaaS, Linux, Windows, macOS

ⓘ

Impact Type: Availability

Version: 1.4

Created: 02 March 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1498/002/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1498/002/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1037](https://attack.mitre.org/mitigations/M1037)   |  [Filter Network Traffic](https://attack.mitre.org/mitigations/M1037)   |

When flood volumes exceed the capacity of the network connection being targeted, it is typically necessary to intercept the incoming traffic upstream to filter out the attack traffic from the legitimate traffic. Such defenses can be provided by the hosting Internet Service Provider (ISP) or by a 3rd party such as a Content Delivery Network (CDN) or providers specializing in DoS mitigations.[[6]](http://cert.europa.eu/static/WhitePapers/CERT-EU_Security_Whitepaper_DDoS_17-003.pdf)

Depending on flood volume, on-premises filtering may be possible by blocking source addresses sourcing the attack, blocking ports that are being targeted, or blocking protocols being used for transport.[[6]](http://cert.europa.eu/static/WhitePapers/CERT-EU_Security_Whitepaper_DDoS_17-003.pdf)

As immediate response may require rapid engagement of 3rd parties, analyze the risk associated to critical resources being affected by Network DoS attacks and create a disaster recovery plan/business continuity plan to respond to incidents.[[6]](http://cert.europa.eu/static/WhitePapers/CERT-EU_Security_Whitepaper_DDoS_17-003.pdf)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0408](https://attack.mitre.org/detectionstrategies/DET0408)   |  [Detection Strategy for Reflection Amplification DoS (T1498.002)](https://attack.mitre.org/detectionstrategies/DET0408)   |  [AN1140](https://attack.mitre.org/detectionstrategies/DET0408#AN1140)   |

Outbound spoofed traffic to known amplification protocols (e.g., DNS, NTP, Memcached) combined with abnormal network traffic volume targeting remote reflectors, resulting in disproportionate traffic returned to a victim

|  [AN1141](https://attack.mitre.org/detectionstrategies/DET0408#AN1141)   |

Spoofed outbound packets sent to amplification services from command-line tools or scripts, combined with abnormal outbound packet volume on known reflector ports

|  [AN1142](https://attack.mitre.org/detectionstrategies/DET0408#AN1142)   |

Command-line initiated UDP traffic bursts to external reflection amplification ports using built-in scripting or binaries with network anomalies

|  [AN1143](https://attack.mitre.org/detectionstrategies/DET0408#AN1143)   |

Cloud-hosted VM or container generates spoofed UDP requests to third-party services on known amplifier ports, with high outbound-to-inbound traffic ratios in VPC Flow Logs

## References

-    [Marek Majkowsk, Cloudflare. (2017, May 24). Reflections on reflection (attacks). Retrieved April 23, 2019.](https://blog.cloudflare.com/reflections-on-reflections/)
-    [Cloudflare. (n.d.). What is a DNS amplification attack?. Retrieved April 23, 2019.](https://www.cloudflare.com/learning/ddos/dns-amplification-ddos-attack/)
-    [Cloudflare. (n.d.). What is a NTP amplificaiton attack?. Retrieved April 23, 2019.](https://www.cloudflare.com/learning/ddos/ntp-amplification-ddos-attack/)

-    [Philippe Alcoy, Steinthor Bjarnason, Paul Bowen, C.F. Chui, Kirill Kasavchnko, and Gary Sockrider of Netscout Arbor. (2018, January). Insight into the Global Threat Landscape - Netscout Arbor's 13th Annual Worldwide Infrastructure Security Report. Retrieved April 22, 2019.](https://pages.arbornetworks.com/rs/082-KNA-087/images/13th_Worldwide_Infrastructure_Security_Report.pdf)
-    [Marek Majkowski of Cloudflare. (2018, February 27). Memcrashed - Major amplification attacks from UDP port 11211. Retrieved April 18, 2019.](https://blog.cloudflare.com/memcrashed-major-amplification-attacks-from-port-11211/)
-    [Meintanis, S., Revuelto, V., Socha, K.. (2017, March 10). DDoS Overview and Response Guide. Retrieved April 24, 2019.](http://cert.europa.eu/static/WhitePapers/CERT-EU_Security_Whitepaper_DDoS_17-003.pdf)

×
