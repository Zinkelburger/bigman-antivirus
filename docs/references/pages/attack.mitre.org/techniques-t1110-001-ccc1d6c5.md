---
url: "https://attack.mitre.org/techniques/T1110/001/"
title: "Brute Force: Password Guessing, Sub-technique T1110.001 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:32:35Z"
via: "live"
sha256: "bb72305f0441ce64b3a12b050ce2b0a65fa7b5b50b62e4fc6c1a11e7c9ada919"
wayback: "https://web.archive.org/web/20261025164736/https://attack.mitre.org/techniques/T1110/001/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Brute Force: Password Guessing, Sub-technique T1110.001 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1110/001/>
> Snapshot: <https://web.archive.org/web/20261025164736/https://attack.mitre.org/techniques/T1110/001/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Brute Force](https://attack.mitre.org/techniques/T1110)
- Password Guessing

#  Brute Force: Password Guessing

##### Other sub-techniques of Brute Force (4)

| ID  | Name

|  T1110.001   |  Password Guessing

|  [T1110.002](https://attack.mitre.org/techniques/T1110/002/)   |  [Password Cracking](https://attack.mitre.org/techniques/T1110/002/)

|  [T1110.003](https://attack.mitre.org/techniques/T1110/003/)   |  [Password Spraying](https://attack.mitre.org/techniques/T1110/003/)

|  [T1110.004](https://attack.mitre.org/techniques/T1110/004/)   |  [Credential Stuffing](https://attack.mitre.org/techniques/T1110/004/)

Adversaries with no prior knowledge of legitimate credentials within the system or environment may guess passwords to attempt access to accounts. Without knowledge of the password for an account, an adversary may opt to systematically guess the password using a repetitive or iterative mechanism. An adversary may guess login credentials without prior knowledge of system or environment passwords during an operation by using a list of common passwords. Password guessing may or may not take into account the target's policies on password complexity or use policies that may lock accounts out after a number of failed attempts.

Guessing passwords can be a risky option because it could cause numerous authentication failures and account lockouts, depending on the organization's login failure policies. [[1]](https://web.archive.org/web/20200302085133/https://www.cylance.com/content/dam/cylance/pages/operation-cleaver/Cylance_Operation_Cleaver_Report.pdf)

Typically, management services over commonly used ports are used when guessing passwords. Commonly targeted services include the following:

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
- SNMP (161/UDP and 162/TCP/UDP)

In addition to management services, adversaries may "target single sign-on (SSO) and cloud-based applications utilizing federated authentication protocols," as well as externally facing email applications, such as Office 365.[[2]](https://www.us-cert.gov/ncas/alerts/TA18-086A). Further, adversaries may abuse network device interfaces (such as wlanAPI) to brute force accessible wifi-router(s) via wireless authentication protocols.[[3]](https://www.trendmicro.com/vinfo/us/security/news/cybercrime-and-digital-threats/emotet-now-spreads-via-wi-fi)

In default environments, LDAP and Kerberos connection attempts are less likely to trigger events over SMB, which creates Windows "logon failure" event ID 4625.

ID: T1110.001

Sub-technique of:  [T1110](https://attack.mitre.org/techniques/T1110)

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Containers, ESXi, IaaS, Identity Provider, Linux, Network Devices, Office Suite, SaaS, Windows, macOS

Contributors: Microsoft Threat Intelligence Center (MSTIC); Mohamed Kmal

Version: 1.7

Created: 11 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1110/001/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1110/001/)

## Procedure Examples

| ID  | Name  | Description

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has used a brute-force/password-spray tooling that operated in two modes: in brute-force mode it typically sent over 300 authentication attempts per hour per targeted account over the course of several hours or days.[[4]](https://www.microsoft.com/security/blog/2020/09/10/strontium-detecting-new-patters-credential-harvesting/) [APT28](https://attack.mitre.org/groups/G0007) has also used a Kubernetes cluster to conduct distributed, large-scale password guessing attacks.[[5]](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has successfully conducted password guessing attacks against a list of mailboxes.[[6]](https://www.mandiant.com/resources/blog/apt29-continues-targeting-microsoft)

|  [S0020](https://attack.mitre.org/software/S0020)   |  [China Chopper](https://attack.mitre.org/software/S0020)   |

[China Chopper](https://attack.mitre.org/software/S0020)'s server component can perform brute force password guessing against authentication portals.[[7]](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)

|  [S0488](https://attack.mitre.org/software/S0488)   |  [CrackMapExec](https://attack.mitre.org/software/S0488)   |

[CrackMapExec](https://attack.mitre.org/software/S0488) can brute force passwords for a specified user on a single target system or across an entire network.[[8]](https://github.com/byt3bl33d3r/CrackMapExec/wiki/SMB-Command-Reference)

|  [S0367](https://attack.mitre.org/software/S0367)   |  [Emotet](https://attack.mitre.org/software/S0367)   |

[Emotet](https://attack.mitre.org/software/S0367) has been observed using a hard coded list of passwords to brute force user accounts. [[9]](https://support.malwarebytes.com/docs/DOC-2295)[[10]](https://www.symantec.com/blogs/threat-intelligence/evolution-emotet-trojan-distributor)[[11]](https://www.us-cert.gov/ncas/alerts/TA18-201A)[[12]](https://www.secureworks.com/blog/lazy-passwords-become-rocket-fuel-for-emotet-smb-spreader)[[13]](https://www.cisecurity.org/white-papers/ms-isac-security-primer-emotet/)[[14]](https://www.binarydefense.com/resources/blog/emotet-evolves-with-new-wi-fi-spreader/)

|  [S0698](https://attack.mitre.org/software/S0698)   |  [HermeticWizard](https://attack.mitre.org/software/S0698)   |

[HermeticWizard](https://attack.mitre.org/software/S0698) can use a list of hardcoded credentials in attempt to authenticate to SMB shares.[[15]](https://www.welivesecurity.com/2022/03/01/isaacwiper-hermeticwizard-wiper-worm-targeting-ukraine)

|  [S0532](https://attack.mitre.org/software/S0532)   |  [Lucifer](https://attack.mitre.org/software/S0532)   |

[Lucifer](https://attack.mitre.org/software/S0532) has attempted to brute force TCP ports 135 (RPC) and 1433 (MSSQL) with the default username or list of usernames and passwords.[[16]](https://unit42.paloaltonetworks.com/lucifer-new-cryptojacking-and-ddos-hybrid-malware/)

|  [S0598](https://attack.mitre.org/software/S0598)   |  [P.A.S. Webshell](https://attack.mitre.org/software/S0598)   |

[P.A.S. Webshell](https://attack.mitre.org/software/S0598) can use predefined users and passwords to execute brute force attacks against SSH, FTP, POP3, MySQL, MSSQL, and PostgreSQL services.[[17]](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)

|  [S0453](https://attack.mitre.org/software/S0453)   |  [Pony](https://attack.mitre.org/software/S0453)   |

[Pony](https://attack.mitre.org/software/S0453) has used a small dictionary of common passwords against a collected list of local accounts.[[18]](https://blog.malwarebytes.com/threat-analysis/2015/11/no-money-but-pony-from-a-mail-to-a-trojan-horse/)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has used weak credentials to gain access to victim devices.[[19]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  [S0374](https://attack.mitre.org/software/S0374)   |  [SpeakUp](https://attack.mitre.org/software/S0374)   |

[SpeakUp](https://attack.mitre.org/software/S0374) can perform brute forcing using a pre-defined list of usernames and passwords in an attempt to log in to administrative panels. [[20]](https://research.checkpoint.com/speakup-a-new-undetected-backdoor-linux-trojan/)

|  [G1055](https://attack.mitre.org/groups/G1055)   |  [VOID MANTICORE](https://attack.mitre.org/groups/G1055)   |

[VOID MANTICORE](https://attack.mitre.org/groups/G1055) has conducted password guessing to gain initial access.[[21]](https://dti.domaintools.com/research/handala-mois-linked-cyber-influence-ecosystem-threat-intelligence-assessment)

|  [S0341](https://attack.mitre.org/software/S0341)   |  [Xbash](https://attack.mitre.org/software/S0341)   |

[Xbash](https://attack.mitre.org/software/S0341) can obtain a list of weak passwords from the C2 server to use for brute forcing as well as attempt to brute force services with open ports.[[22]](https://researchcenter.paloaltonetworks.com/2018/09/unit42-xbash-combines-botnet-ransomware-coinmining-worm-targets-linux-windows/)[[23]](https://www.trendmicro.com/vinfo/us/security/news/cybercrime-and-digital-threats/new-multi-platform-xbash-packs-obfuscation-ransomware-coinminer-worm-and-botnet)

## Mitigations

| ID  | Mitigation  | Description

|  [M1036](https://attack.mitre.org/mitigations/M1036)   |  [Account Use Policies](https://attack.mitre.org/mitigations/M1036)   |

Set account lockout policies after a certain number of failed login attempts to prevent passwords from being guessed. Too strict a policy may create a denial of service condition and render environments un-usable, with all accounts used in the brute force being locked-out. Use conditional access policies to block logins from non-compliant devices or from outside defined organization IP ranges.[[24]](https://learn.microsoft.com/en-us/azure/active-directory/conditional-access/concept-conditional-access-policy-common) Consider blocking risky authentication requests, such as those originating from anonymizing services/proxies.[[25]](https://sec.okta.com/blockanonymizers)

|  [M1032](https://attack.mitre.org/mitigations/M1032)   |  [Multi-factor Authentication](https://attack.mitre.org/mitigations/M1032)   |

Use multi-factor authentication. Where possible, also enable multi-factor authentication on externally facing services.

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Refer to NIST guidelines when creating password policies. [[26]](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-63-4.pdf)

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Upgrade management services to the latest supported and compatible version. Specifically, any version providing increased password complexity or policy enforcement preventing default or weak passwords.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0551](https://attack.mitre.org/detectionstrategies/DET0551)   |  [Password Guessing via Multi-Source Authentication Failure Correlation](https://attack.mitre.org/detectionstrategies/DET0551)   |  [AN1521](https://attack.mitre.org/detectionstrategies/DET0551#AN1521)   |

Series of authentication failures (Event ID 4625) targeting the same or similar user accounts over time from one or more remote IPs

|  [AN1522](https://attack.mitre.org/detectionstrategies/DET0551#AN1522)   |

Repeated failed SSH login attempts followed by a possible success from the same remote host

|  [AN1523](https://attack.mitre.org/detectionstrategies/DET0551#AN1523)   |

Series of failed logins from loginwindow or sshd with repeated usernames or password prompts

|  [AN1524](https://attack.mitre.org/detectionstrategies/DET0551#AN1524)   |

Multiple failed sign-in attempts from external sources across many users followed by success from the same IP

|  [AN1525](https://attack.mitre.org/detectionstrategies/DET0551#AN1525)   |

Login attempt failures over SNMP, Telnet, or SSH interface, often reflected in logs or syslog events

|  [AN1526](https://attack.mitre.org/detectionstrategies/DET0551#AN1526)   |

Password guessing attempts against web-based apps (e.g., Dropbox, Google Workspace) reflected in API or sign-in logs

## References

-    [Cylance. (2014, December). Operation Cleaver. Retrieved September 14, 2017.](https://web.archive.org/web/20200302085133/https://www.cylance.com/content/dam/cylance/pages/operation-cleaver/Cylance_Operation_Cleaver_Report.pdf)
-    [US-CERT. (2018, March 27). TA18-068A Brute Force Attacks Conducted by Cyber Actors. Retrieved October 2, 2019.](https://www.us-cert.gov/ncas/alerts/TA18-086A)
-    [Cybercrime & Digital Threat Team. (2020, February 13). Emotet Now Spreads via Wi-Fi. Retrieved February 16, 2022.](https://www.trendmicro.com/vinfo/us/security/news/cybercrime-and-digital-threats/emotet-now-spreads-via-wi-fi)
-    [Microsoft Threat Intelligence Center (MSTIC). (2020, September 10). STRONTIUM: Detecting new patterns in credential harvesting. Retrieved September 11, 2020.](https://www.microsoft.com/security/blog/2020/09/10/strontium-detecting-new-patters-credential-harvesting/)
-    [NSA, CISA, FBI, NCSC. (2021, July). Russian GRU Conducting Global Brute Force Campaign to Compromise Enterprise and Cloud Environments. Retrieved July 26, 2021.](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)
-    [Douglas Bienstock. (2022, August 18). You Can’t Audit Me: APT29 Continues Targeting Microsoft 365. Retrieved February 23, 2023.](https://www.mandiant.com/resources/blog/apt29-continues-targeting-microsoft)
-    [FireEye. (2018, March 16). Suspected Chinese Cyber Espionage Group (TEMP.Periscope) Targeting U.S. Engineering and Maritime Industries. Retrieved April 11, 2018.](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)
-    [byt3bl33d3r. (2018, September 8). SMB: Command Reference. Retrieved July 17, 2020.](https://github.com/byt3bl33d3r/CrackMapExec/wiki/SMB-Command-Reference)
-    [Smith, A.. (2017, December 22). Protect your network from Emotet Trojan with Malwarebytes Endpoint Security. Retrieved January 17, 2019.](https://support.malwarebytes.com/docs/DOC-2295)
-    [Symantec. (2018, July 18). The Evolution of Emotet: From Banking Trojan to Threat Distributor. Retrieved March 25, 2019.](https://www.symantec.com/blogs/threat-intelligence/evolution-emotet-trojan-distributor)
-    [US-CERT. (2018, July 20). Alert (TA18-201A) Emotet Malware. Retrieved March 25, 2019.](https://www.us-cert.gov/ncas/alerts/TA18-201A)
-    [Mclellan, M.. (2018, November 19). Lazy Passwords Become Rocket Fuel for Emotet SMB Spreader. Retrieved March 25, 2019.](https://www.secureworks.com/blog/lazy-passwords-become-rocket-fuel-for-emotet-smb-spreader)
-    [CIS. (2018, December 12). MS-ISAC Security Primer- Emotet. Retrieved March 25, 2019.](https://www.cisecurity.org/white-papers/ms-isac-security-primer-emotet/)

-    [Binary Defense. (n.d.). Emotet Evolves With new Wi-Fi Spreader. Retrieved September 8, 2023.](https://www.binarydefense.com/resources/blog/emotet-evolves-with-new-wi-fi-spreader/)
-    [ESET. (2022, March 1). IsaacWiper and HermeticWizard: New wiper and worm targetingUkraine. Retrieved April 10, 2022.](https://www.welivesecurity.com/2022/03/01/isaacwiper-hermeticwizard-wiper-worm-targeting-ukraine)
-    [Hsu, K. et al. (2020, June 24). Lucifer: New Cryptojacking and DDoS Hybrid Malware Exploiting High and Critical Vulnerabilities to Infect Windows Devices. Retrieved November 16, 2020.](https://unit42.paloaltonetworks.com/lucifer-new-cryptojacking-and-ddos-hybrid-malware/)
-    [ANSSI. (2021, January 27). SANDWORM INTRUSION SET CAMPAIGN TARGETING CENTREON SYSTEMS. Retrieved March 30, 2021.](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)
-    [hasherezade. (2016, April 11). No money, but Pony! From a mail to a trojan horse. Retrieved May 21, 2020.](https://blog.malwarebytes.com/threat-analysis/2015/11/no-money-but-pony-from-a-mail-to-a-trojan-horse/)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Check Point Research. (2019, February 4). SpeakUp: A New Undetected Backdoor Linux Trojan. Retrieved April 17, 2019.](https://research.checkpoint.com/speakup-a-new-undetected-backdoor-linux-trojan/)
-    [DomainTools Investigations. (2026, April 6). Handala: MOIS Linked Cyber Influence Ecosystem Threat Intelligence Assessment. Retrieved April 20, 2026.](https://dti.domaintools.com/research/handala-mois-linked-cyber-influence-ecosystem-threat-intelligence-assessment)
-    [Xiao, C. (2018, September 17). Xbash Combines Botnet, Ransomware, Coinmining in Worm that Targets Linux and Windows. Retrieved November 14, 2018.](https://researchcenter.paloaltonetworks.com/2018/09/unit42-xbash-combines-botnet-ransomware-coinmining-worm-targets-linux-windows/)
-    [Trend Micro. (2018, September 19). New Multi-Platform Xbash Packs Obfuscation, Ransomware, Coinminer, Worm and Botnet. Retrieved June 4, 2019.](https://www.trendmicro.com/vinfo/us/security/news/cybercrime-and-digital-threats/new-multi-platform-xbash-packs-obfuscation-ransomware-coinminer-worm-and-botnet)
-    [Microsoft. (2022, December 14). Conditional Access templates. Retrieved February 21, 2023.](https://learn.microsoft.com/en-us/azure/active-directory/conditional-access/concept-conditional-access-policy-common)
-    [Moussa Diallo and Brett Winterford. (2024, April 26). How to Block Anonymizing Services using Okta. Retrieved May 28, 2024.](https://sec.okta.com/blockanonymizers)
-    [Temoshok, D., et al. (July 2025). SP 800-63-4, Digital Identity Guidelines. Retrieved July 29, 2026.](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-63-4.pdf)

×
