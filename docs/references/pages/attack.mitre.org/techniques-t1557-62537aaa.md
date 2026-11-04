---
url: "https://attack.mitre.org/techniques/T1557/"
title: "Adversary-in-the-Middle, Technique T1557 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:55:54Z"
via: "live"
sha256: "e1efd5d845fa14004c001bf00c89770c7e13e0a80a49cfb53b4f9875d5da62fd"
wayback: "https://web.archive.org/web/20261023090821/https://attack.mitre.org/techniques/T1557/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Adversary-in-the-Middle, Technique T1557 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1557/>
> Snapshot: <https://web.archive.org/web/20261023090821/https://attack.mitre.org/techniques/T1557/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Adversary-in-the-Middle

#  Adversary-in-the-Middle

##### Sub-techniques (4)

| ID  | Name

|  [T1557.001](https://attack.mitre.org/techniques/T1557/001/)   |  [Name Resolution Poisoning and SMB Relay](https://attack.mitre.org/techniques/T1557/001/)

|  [T1557.002](https://attack.mitre.org/techniques/T1557/002/)   |  [ARP Cache Poisoning](https://attack.mitre.org/techniques/T1557/002/)

|  [T1557.003](https://attack.mitre.org/techniques/T1557/003/)   |  [DHCP Spoofing](https://attack.mitre.org/techniques/T1557/003/)

|  [T1557.004](https://attack.mitre.org/techniques/T1557/004/)   |  [Evil Twin](https://attack.mitre.org/techniques/T1557/004/)

Adversaries may attempt to position themselves between two or more networked devices using an adversary-in-the-middle (AiTM) technique to support follow-on behaviors such as [Network Sniffing](https://attack.mitre.org/techniques/T1040), [Transmitted Data Manipulation](https://attack.mitre.org/techniques/T1565/002), or replay attacks ([Exploitation for Credential Access](https://attack.mitre.org/techniques/T1212)). By abusing features of common networking protocols that can determine the flow of network traffic (e.g. ARP, DNS, LLMNR, etc.), adversaries may force a device to communicate through an adversary controlled system so they can collect information or perform additional actions.[[1]](https://www.rapid7.com/fundamentals/man-in-the-middle-attacks/)

For example, adversaries may manipulate victim DNS settings to enable other malicious activities such as preventing/redirecting users from accessing legitimate sites and/or pushing additional malware.[[2]](https://blog.netlab.360.com/ttint-an-iot-remote-control-trojan-spread-through-2-0-day-vulnerabilities/)[[3]](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/web-attack/125/how-dns-changer-trojans-direct-users-to-threats)[[4]](https://securelist.com/ad-blocker-with-miner-included/101105/) Adversaries may also manipulate DNS and leverage their position in order to intercept user credentials, including access tokens ([Steal Application Access Token](https://attack.mitre.org/techniques/T1528)) and session cookies ([Steal Web Session Cookie](https://attack.mitre.org/techniques/T1539)).[[5]](https://www.volexity.com/blog/2022/06/15/driftingcloud-zero-day-sophos-firewall-exploitation-and-an-insidious-breach/)[[6]](https://www.microsoft.com/en-us/security/blog/2022/11/16/token-tactics-how-to-prevent-detect-and-respond-to-cloud-token-theft/) [Downgrade Attack](https://attack.mitre.org/techniques/T1689)s can also be used to establish an AiTM position, such as by negotiating a less secure, deprecated, or weaker version of communication protocol (SSL/TLS) or encryption algorithm.[[7]](https://www.praetorian.com/blog/man-in-the-middle-tls-ssl-protocol-downgrade-attack/)[[8]](https://arxiv.org/abs/1809.05681)[[9]](https://tlseminar.github.io/downgrade-attacks/)

Adversaries may also leverage the AiTM position to attempt to monitor and/or modify traffic, such as in [Transmitted Data Manipulation](https://attack.mitre.org/techniques/T1565/002). Adversaries can setup a position similar to AiTM to prevent traffic from flowing to the appropriate destination, potentially to impair defenses and/or in support of a [Network Denial of Service](https://attack.mitre.org/techniques/T1498).

ID: T1557

Sub-techniques:  [T1557.001](https://attack.mitre.org/techniques/T1557/001), [T1557.002](https://attack.mitre.org/techniques/T1557/002), [T1557.003](https://attack.mitre.org/techniques/T1557/003), [T1557.004](https://attack.mitre.org/techniques/T1557/004)

ⓘ

Tactics: [Credential Access](https://attack.mitre.org/tactics/TA0006), [Collection](https://attack.mitre.org/tactics/TA0009)

ⓘ

Platforms: Linux, Network Devices, Windows, macOS

Contributors: Daniil Yugoslavskiy, @yugoslavskiy, Atomic Threat Coverage project; Mayuresh Dani, Qualys; NEC

Version: 2.5

Created: 11 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1557/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1557/)

## Procedure Examples

| ID  | Name  | Description

|  [C0046](https://attack.mitre.org/campaigns/C0046)   |  [ArcaneDoor](https://attack.mitre.org/campaigns/C0046)   |

[ArcaneDoor](https://attack.mitre.org/campaigns/C0046) included interception of HTTP traffic to victim devices to identify and parse command and control information sent to the device.[[10]](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)

|  [S0281](https://attack.mitre.org/software/S0281)   |  [Dok](https://attack.mitre.org/software/S0281)   |

[Dok](https://attack.mitre.org/software/S0281) proxies web traffic to potentially monitor and alter victim HTTP(S) traffic.[[11]](https://objective-see.com/blog/blog_0x25.html)[[12]](https://blog.checkpoint.com/2017/04/27/osx-malware-catching-wants-read-https-traffic/)

|  [S9003](https://attack.mitre.org/software/S9003)   |  [evilginx2](https://attack.mitre.org/software/S9003)   |

[evilginx2](https://attack.mitre.org/software/S9003) has the ability to act as an adversary-in-the-middle (AiTM) relay between a legitimate website and a phished user to capture all transmitted data including usernames, passwords, authentication tokens, and session cookies and tokens.[[13]](https://breakdev.org/evilginx-2-next-generation-of-phishing-2fa-tokens/)[[14]](https://breakdev.org/evilginx-3-0-evilginx-mastery/)[[15]](https://breakdev.org/evilginx-3-2/)[[16]](https://www.sophos.com/en-us/blog/stealing-user-credentials-with-evilginx)

|  [S9044](https://attack.mitre.org/software/S9044)   |  [Kali365](https://attack.mitre.org/software/S9044)   |

[Kali365](https://attack.mitre.org/software/S9044) has created obfuscated phishing landing pages that act as an adversary in the middle infrastructure that intercepts communications between the victim host and legitimate services to steal credentials and user sessions.[[17]](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)[[18]](https://arcticwolf.com/resources/blog/kali365-expands-into-aws-microsoft-okta-xerox-max-messenger/)[[19]](https://spycloud.com/blog/kali365-anatomy-of-a-microsoft365-phishing-as-a-service-kit/)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has used modified versions of PHProxy to examine web traffic between the victim and the accessed website.[[20]](https://us-cert.cisa.gov/ncas/alerts/aa20-301a)

|  [S1188](https://attack.mitre.org/software/S1188)   |  [Line Runner](https://attack.mitre.org/software/S1188)   |

[Line Runner](https://attack.mitre.org/software/S1188) intercepts HTTP requests to the victim Cisco ASA, looking for a request with a 32-character, victim dependent parameter. If that parameter matches a value in the malware, a contained payload is then written to a Lua script and executed.[[10]](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)

|  [G0129](https://attack.mitre.org/groups/G0129)   |  [Mustang Panda](https://attack.mitre.org/groups/G0129)   |

[Mustang Panda](https://attack.mitre.org/groups/G0129) leveraged a captive portal hijack that redirected the victim to a webpage that prompted the victim to download a malicious payload.[[21]](https://cloud.google.com/blog/topics/threat-intelligence/prc-nexus-espionage-targets-diplomats)

|  [S1131](https://attack.mitre.org/software/S1131)   |  [NPPSPY](https://attack.mitre.org/software/S1131)   |

[NPPSPY](https://attack.mitre.org/software/S1131) opens a new network listener for the mpnotify.exe process that is typically contacted by the Winlogon process in Windows. A new, alternative RPC channel is set up with a malicious DLL recording plaintext credentials entered into Winlogon, effectively intercepting and redirecting the logon information.[[22]](https://www.huntress.com/blog/cleartext-shenanigans-gifting-user-passwords-to-adversaries-with-nppspy)

|  [G1041](https://attack.mitre.org/groups/G1041)   |  [Sea Turtle](https://attack.mitre.org/groups/G1041)   |

[Sea Turtle](https://attack.mitre.org/groups/G1041) modified DNS records at service providers to redirect traffic from legitimate resources to [Sea Turtle](https://attack.mitre.org/groups/G1041)-controlled servers to enable adversary-in-the-middle attacks for credential capture.[[23]](https://blog.talosintelligence.com/seaturtle/)[[24]](https://blog.talosintelligence.com/sea-turtle-keeps-on-swimming/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1042](https://attack.mitre.org/mitigations/M1042)   |  [Disable or Remove Feature or Program](https://attack.mitre.org/mitigations/M1042)   |

Disable legacy network protocols that may be used to intercept network traffic if applicable, especially those that are not needed within an environment.

|  [M1041](https://attack.mitre.org/mitigations/M1041)   |  [Encrypt Sensitive Information](https://attack.mitre.org/mitigations/M1041)   |

Ensure that all wired and/or wireless traffic is encrypted appropriately. Use best practices for authentication protocols, such as Kerberos, and ensure web traffic that may contain credentials is protected by SSL/TLS.

|  [M1037](https://attack.mitre.org/mitigations/M1037)   |  [Filter Network Traffic](https://attack.mitre.org/mitigations/M1037)   |

Use network appliances and host-based security software to block network traffic that is not necessary within the environment, such as legacy protocols that may be leveraged for AiTM conditions.

|  [M1035](https://attack.mitre.org/mitigations/M1035)   |  [Limit Access to Resource Over Network](https://attack.mitre.org/mitigations/M1035)   |

Limit access to network infrastructure and resources that can be used to reshape traffic or otherwise produce AiTM conditions.

|  [M1031](https://attack.mitre.org/mitigations/M1031)   |  [Network Intrusion Prevention](https://attack.mitre.org/mitigations/M1031)   |

Network intrusion detection and prevention systems that can identify traffic patterns indicative of AiTM activity can be used to mitigate activity at the network level.

|  [M1030](https://attack.mitre.org/mitigations/M1030)   |  [Network Segmentation](https://attack.mitre.org/mitigations/M1030)   |

Network segmentation can be used to isolate infrastructure components that do not require broad network access. This may mitigate, or at least alleviate, the scope of AiTM activity.

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Train users to be suspicious about certificate errors. Adversaries may use their own certificates in an attempt to intercept HTTPS traffic. Certificate errors may arise when the application’s certificate does not match the one expected by the host.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0296](https://attack.mitre.org/detectionstrategies/DET0296)   |  [Detect Adversary-in-the-Middle via Network and Configuration Anomalies](https://attack.mitre.org/detectionstrategies/DET0296)   |  [AN0823](https://attack.mitre.org/detectionstrategies/DET0296#AN0823)   |

Detects suspicious DNS/ARP poisoning attempts, unauthorized modifications to registry/network configuration, or abnormal TLS downgrade activity. Correlates changes in system configuration with subsequent unusual network flows or authentication events.

|  [AN0824](https://attack.mitre.org/detectionstrategies/DET0296#AN0824)   |

Detects unauthorized edits to /etc/hosts, /etc/resolv.conf, or suspicious ARP broadcasts. Correlates file modifications with subsequent unexpected network sessions or service creation.

|  [AN0825](https://attack.mitre.org/detectionstrategies/DET0296#AN0825)   |

Detects unauthorized edits to system configuration profiles, unexpected certificate trust changes, or abnormal ARP/DNS patterns indicative of interception.

|  [AN0826](https://attack.mitre.org/detectionstrategies/DET0296#AN0826)   |

Detects unauthorized firmware or configuration changes enabling adversary-in-the-middle positioning (e.g., route injection, DNS spoofing, SSL downgrade). Behavioral analytics focus on sudden changes to routing tables or image file integrity failures.

## References

-    [Rapid7. (n.d.). Man-in-the-Middle (MITM) Attacks. Retrieved March 2, 2020.](https://www.rapid7.com/fundamentals/man-in-the-middle-attacks/)
-    [Tu, L. Ma, Y. Ye, G. (2020, October 1). Ttint: An IoT Remote Access Trojan spread through 2 0-day vulnerabilities. Retrieved October 28, 2021.](https://blog.netlab.360.com/ttint-an-iot-remote-control-trojan-spread-through-2-0-day-vulnerabilities/)
-    [Abendan, O. (2012, June 14). How DNS Changer Trojans Direct Users to Threats. Retrieved October 28, 2021.](https://www.trendmicro.com/vinfo/us/threat-encyclopedia/web-attack/125/how-dns-changer-trojans-direct-users-to-threats)
-    [Kuzmenko, A.. (2021, March 10). Ad blocker with miner included. Retrieved October 28, 2021.](https://securelist.com/ad-blocker-with-miner-included/101105/)
-    [Adair, S., Lancaster, T., Volexity Threat Research. (2022, June 15). DriftingCloud: Zero-Day Sophos Firewall Exploitation and an Insidious Breach. Retrieved July 1, 2022.](https://www.volexity.com/blog/2022/06/15/driftingcloud-zero-day-sophos-firewall-exploitation-and-an-insidious-breach/)
-    [Microsoft Incident Response. (2022, November 16). Token tactics: How to prevent, detect, and respond to cloud token theft. Retrieved December 26, 2023.](https://www.microsoft.com/en-us/security/blog/2022/11/16/token-tactics-how-to-prevent-detect-and-respond-to-cloud-token-theft/)
-    [praetorian Editorial Team. (2014, August 19). Man-in-the-Middle TLS Protocol Downgrade Attack. Retrieved December 8, 2021.](https://www.praetorian.com/blog/man-in-the-middle-tls-ssl-protocol-downgrade-attack/)
-    [Alashwali, E. S., Rasmussen, K. (2019, January 26). What's in a Downgrade? A Taxonomy of Downgrade Attacks in the TLS Protocol and Application Protocols Using TLS. Retrieved December 7, 2021.](https://arxiv.org/abs/1809.05681)
-    [Team Cinnamon. (2017, February 3). Downgrade Attacks. Retrieved December 9, 2021.](https://tlseminar.github.io/downgrade-attacks/)
-    [Cisco Talos. (2024, April 24). ArcaneDoor - New espionage-focused campaign found targeting perimeter network devices. Retrieved January 6, 2025.](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)
-    [Patrick Wardle. (n.d.). Mac Malware of 2017. Retrieved September 21, 2018.](https://objective-see.com/blog/blog_0x25.html)
-    [Ofer Caspi. (2017, May 4). OSX Malware is Catching Up, and it wants to Read Your HTTPS Traffic. Retrieved October 5, 2021.](https://blog.checkpoint.com/2017/04/27/osx-malware-catching-wants-read-https-traffic/)

-    [Gretzky, K.. (2018, July 26). Evilginx 2 - Next Generation of Phishing 2FA Tokens. Retrieved October 14, 2019.](https://breakdev.org/evilginx-2-next-generation-of-phishing-2fa-tokens/)
-    [Gretzky, K. (2023, May 10). Evilginx 3.0 + Evilginx Mastery. Retrieved January 27, 2026.](https://breakdev.org/evilginx-3-0-evilginx-mastery/)
-    [Gretzky, K. (2023, August 24). Evilginx 3.2 - Swimming With The Phishes. Retrieved January 27, 2026.](https://breakdev.org/evilginx-3-2/)
-    [Everts, M. (2025, March 28). Stealing user credentials with evilginx. Retrieved January 27, 2026.](https://www.sophos.com/en-us/blog/stealing-user-credentials-with-evilginx)
-    [Artic Wolf Labs. (2026, April 24). Token Bingo: Don’t Let Your Code be the Winner. Retrieved July 30, 2026.](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)
-    [Artic Wolf Labs. (2026, June 2). Retrieved July 30, 2026.](https://arcticwolf.com/resources/blog/kali365-expands-into-aws-microsoft-okta-xerox-max-messenger/)
-    [Trevor Hilligoss. (2026, June 11). Kali365: Anatomy of a Microsoft 365 Phishing-as-a-Service Kit – From Telegram Hype to FBI Takedown Theater. Retrieved July 30, 2026.](https://spycloud.com/blog/kali365-anatomy-of-a-microsoft365-phishing-as-a-service-kit/)
-    [CISA, FBI, CNMF. (2020, October 27). https://us-cert.cisa.gov/ncas/alerts/aa20-301a. Retrieved November 4, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-301a)
-    [Patrick Whitsell. (2025, August 25). Deception in Depth: PRC-Nexus Espionage Campaign Hijacks Web Traffic to Target Diplomats. Retrieved September 9, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/prc-nexus-espionage-targets-diplomats)
-    [Dray Agha. (2022, August 16). Cleartext Shenanigans: Gifting User Passwords to Adversaries With NPPSPY. Retrieved May 17, 2024.](https://www.huntress.com/blog/cleartext-shenanigans-gifting-user-passwords-to-adversaries-with-nppspy)
-    [Cisco Talos. (2019, April 17). Sea Turtle: DNS Hijacking Abuses Trust In Core Internet Service. Retrieved November 20, 2024.](https://blog.talosintelligence.com/seaturtle/)
-    [Paul Rascagneres. (2019, July 9). Sea Turtle keeps on swimming, finds new victims, DNS hijacking techniques. Retrieved November 20, 2024.](https://blog.talosintelligence.com/sea-turtle-keeps-on-swimming/)

×
