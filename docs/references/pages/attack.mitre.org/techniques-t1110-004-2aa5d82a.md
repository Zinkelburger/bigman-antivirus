---
url: "https://attack.mitre.org/techniques/T1110/004/"
title: "Brute Force: Credential Stuffing, Sub-technique T1110.004 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:33:47Z"
via: "live"
sha256: "e2fd1f899cbabc39b4162ecc180017d1d6ddac52e1b848d000122f02b19ebfcd"
wayback: "https://web.archive.org/web/20261023090821/https://attack.mitre.org/techniques/T1110/004/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Brute Force: Credential Stuffing, Sub-technique T1110.004 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1110/004/>
> Snapshot: <https://web.archive.org/web/20261023090821/https://attack.mitre.org/techniques/T1110/004/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Brute Force](https://attack.mitre.org/techniques/T1110)
- Credential Stuffing

#  Brute Force: Credential Stuffing

##### Other sub-techniques of Brute Force (4)

| ID  | Name

|  [T1110.001](https://attack.mitre.org/techniques/T1110/001/)   |  [Password Guessing](https://attack.mitre.org/techniques/T1110/001/)

|  [T1110.002](https://attack.mitre.org/techniques/T1110/002/)   |  [Password Cracking](https://attack.mitre.org/techniques/T1110/002/)

|  [T1110.003](https://attack.mitre.org/techniques/T1110/003/)   |  [Password Spraying](https://attack.mitre.org/techniques/T1110/003/)

|  T1110.004   |  Credential Stuffing

Adversaries may use credentials obtained from breach dumps of unrelated accounts to gain access to target accounts through credential overlap. Occasionally, large numbers of username and password pairs are dumped online when a website or service is compromised and the user account credentials accessed. The information may be useful to an adversary attempting to compromise accounts by taking advantage of the tendency for users to use the same passwords across personal and business accounts.

Credential stuffing is a risky option because it could cause numerous authentication failures and account lockouts, depending on the organization's login failure policies.

Typically, management services over commonly used ports are used when stuffing credentials. Commonly targeted services include the following:

- SSH (22/TCP)
- Telnet (23/TCP)
- FTP (21/TCP)
- NetBIOS / SMB / Samba (139/TCP & 445/TCP)
- LDAP (389/TCP)
- Kerberos (88/TCP)
- RDP / Terminal Services (3389/TCP)
- HTTP/HTTP Management Services (80/TCP & 443/TCP)
- MSSQL (1433/TCP)
- Oracle (1521/TCP)
- MySQL (3306/TCP)
- VNC (5900/TCP)

In addition to management services, adversaries may "target single sign-on (SSO) and cloud-based applications utilizing federated authentication protocols," as well as externally facing email applications, such as Office 365.[[1]](https://www.us-cert.gov/ncas/alerts/TA18-086A)

ID: T1110.004

Sub-technique of:  [T1110](https://attack.mitre.org/techniques/T1110)

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Containers, ESXi, IaaS, Identity Provider, Linux, Network Devices, Office Suite, SaaS, Windows, macOS

Contributors: Anastasios Pingios; Diogo Fernandes

Version: 1.7

Created: 11 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1110/004/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1110/004/)

## Procedure Examples

| ID  | Name  | Description

|  [G0114](https://attack.mitre.org/groups/G0114)   |  [Chimera](https://attack.mitre.org/groups/G0114)   |

[Chimera](https://attack.mitre.org/groups/G0114) has used credential stuffing against victim's remote services to obtain valid accounts.[[2]](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)

|  [S0266](https://attack.mitre.org/software/S0266)   |  [TrickBot](https://attack.mitre.org/software/S0266)   |

[TrickBot](https://attack.mitre.org/software/S0266) uses brute-force attack against RDP with rdpscanDll module.[[3]](https://www.welivesecurity.com/2020/10/12/eset-takes-part-global-operation-disrupt-trickbot/)[[4]](https://www.bitdefender.com/files/News/CaseStudies/study/316/Bitdefender-Whitepaper-TrickBot-en-EN-interactive.pdf)

|  [G1055](https://attack.mitre.org/groups/G1055)   |  [VOID MANTICORE](https://attack.mitre.org/groups/G1055)   |

[VOID MANTICORE](https://attack.mitre.org/groups/G1055) has utilized credential stuffing attacks to obtain initial access to victim environments.[[5]](https://dti.domaintools.com/research/handala-mois-linked-cyber-influence-ecosystem-threat-intelligence-assessment)

## Mitigations

| ID  | Mitigation  | Description

|  [M1036](https://attack.mitre.org/mitigations/M1036)   |  [Account Use Policies](https://attack.mitre.org/mitigations/M1036)   |

Set account lockout policies after a certain number of failed login attempts to prevent passwords from being guessed. Too strict a policy may create a denial of service condition and render environments un-usable, with all accounts used in the brute force being locked-out. Use conditional access policies to block logins from non-compliant devices or from outside defined organization IP ranges.[[6]](https://learn.microsoft.com/en-us/azure/active-directory/conditional-access/concept-conditional-access-policy-common) Consider blocking risky authentication requests, such as those originating from anonymizing services/proxies.[[7]](https://sec.okta.com/blockanonymizers)

|  [M1032](https://attack.mitre.org/mitigations/M1032)   |  [Multi-factor Authentication](https://attack.mitre.org/mitigations/M1032)   |

Use multi-factor authentication. Where possible, also enable multi-factor authentication on externally facing services.

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Refer to NIST guidelines when creating password policies. [[8]](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-63-4.pdf)

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Proactively reset accounts that are known to be part of breached credentials either immediately, or after detecting bruteforce attempts.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0460](https://attack.mitre.org/detectionstrategies/DET0460)   |  [Credential Stuffing Detection via Reused Breached Credentials Across Services](https://attack.mitre.org/detectionstrategies/DET0460)   |  [AN1262](https://attack.mitre.org/detectionstrategies/DET0460#AN1262)   |

Multiple failed authentication attempts using distinct username/password pairs from a single IP address or session within a short time window, targeting common services like RDP or SMB

|  [AN1263](https://attack.mitre.org/detectionstrategies/DET0460#AN1263)   |

Rapid login failures across different users from a single IP address, targeting SSH or PAM login with distinct username-password pairs

|  [AN1264](https://attack.mitre.org/detectionstrategies/DET0460#AN1264)   |

Burst of failed authentications with rotating usernames against loginwindow or remote management service using reused breached credentials

|  [AN1265](https://attack.mitre.org/detectionstrategies/DET0460#AN1265)   |

Same source IP performing multiple authentication attempts using known breached username/password combinations across different identities in Azure AD, Okta, or Duo

|  [AN1266](https://attack.mitre.org/detectionstrategies/DET0460#AN1266)   |

Multiple sign-in failures against cloud-based applications using username/password combinations leaked from unrelated domains

|  [AN1267](https://attack.mitre.org/detectionstrategies/DET0460#AN1267)   |

Router/firewall/syslog logs showing authentication failures with unique usernames and reused credentials from same source IP

|  [AN1268](https://attack.mitre.org/detectionstrategies/DET0460#AN1268)   |

Credential stuffing attempts against Kubernetes API or containerized login shells using stolen or leaked user credentials

|  [AN1269](https://attack.mitre.org/detectionstrategies/DET0460#AN1269)   |

Use of leaked credential pairs against Outlook Web Access (OWA), Microsoft 365, or Exchange from a single client IP with multiple failures

|  [AN1270](https://attack.mitre.org/detectionstrategies/DET0460#AN1270)   |

Burst of failed login attempts across VM instances using leaked credential pairs from single IP in public cloud environments

## References

-    [US-CERT. (2018, March 27). TA18-068A Brute Force Attacks Conducted by Cyber Actors. Retrieved October 2, 2019.](https://www.us-cert.gov/ncas/alerts/TA18-086A)
-    [Jansen, W . (2021, January 12). Abusing cloud services to fly under the radar. Retrieved September 12, 2024.](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)
-    [Boutin, J. (2020, October 12). ESET takes part in global operation to disrupt Trickbot. Retrieved March 15, 2021.](https://www.welivesecurity.com/2020/10/12/eset-takes-part-global-operation-disrupt-trickbot/)
-    [Tudorica, R., Maximciuc, A., Vatamanu, C. (2020, March 18). New TrickBot Module Bruteforces RDP Connections, Targets Select Telecommunication Services in US and Hong Kong. Retrieved March 15, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/316/Bitdefender-Whitepaper-TrickBot-en-EN-interactive.pdf)

-    [DomainTools Investigations. (2026, April 6). Handala: MOIS Linked Cyber Influence Ecosystem Threat Intelligence Assessment. Retrieved April 20, 2026.](https://dti.domaintools.com/research/handala-mois-linked-cyber-influence-ecosystem-threat-intelligence-assessment)
-    [Microsoft. (2022, December 14). Conditional Access templates. Retrieved February 21, 2023.](https://learn.microsoft.com/en-us/azure/active-directory/conditional-access/concept-conditional-access-policy-common)
-    [Moussa Diallo and Brett Winterford. (2024, April 26). How to Block Anonymizing Services using Okta. Retrieved May 28, 2024.](https://sec.okta.com/blockanonymizers)
-    [Temoshok, D., et al. (July 2025). SP 800-63-4, Digital Identity Guidelines. Retrieved July 29, 2026.](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-63-4.pdf)

×
