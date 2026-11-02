---
url: "https://attack.mitre.org/techniques/T1110/003/"
title: "Brute Force: Password Spraying, Sub-technique T1110.003 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:32:55Z"
via: "live"
sha256: "67ddb65a1258ab3b40aecc1bf0a8c329edf108c86466b38123425c36ed8e538f"
wayback: "https://web.archive.org/web/20261023145831/https://attack.mitre.org/techniques/T1110/003/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Brute Force: Password Spraying, Sub-technique T1110.003 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1110/003/>
> Snapshot: <https://web.archive.org/web/20261023145831/https://attack.mitre.org/techniques/T1110/003/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Brute Force](https://attack.mitre.org/techniques/T1110)
- Password Spraying

#  Brute Force: Password Spraying

##### Other sub-techniques of Brute Force (4)

| ID  | Name

|  [T1110.001](https://attack.mitre.org/techniques/T1110/001/)   |  [Password Guessing](https://attack.mitre.org/techniques/T1110/001/)

|  [T1110.002](https://attack.mitre.org/techniques/T1110/002/)   |  [Password Cracking](https://attack.mitre.org/techniques/T1110/002/)

|  T1110.003   |  Password Spraying

|  [T1110.004](https://attack.mitre.org/techniques/T1110/004/)   |  [Credential Stuffing](https://attack.mitre.org/techniques/T1110/004/)

Adversaries may use a single or small list of commonly used passwords against many different accounts to attempt to acquire valid account credentials. Password spraying uses one password (e.g. 'Password01'), or a small list of commonly used passwords, that may match the complexity policy of the domain. Logins are attempted with that password against many different accounts on a network to avoid account lockouts that would normally occur when brute forcing a single account with many passwords. [[1]](http://www.blackhillsinfosec.com/?p=4645)

Typically, management services over commonly used ports are used when password spraying. Commonly targeted services include the following:

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

In addition to management services, adversaries may "target single sign-on (SSO) and cloud-based applications utilizing federated authentication protocols," as well as externally facing email applications, such as Office 365.[[2]](https://www.us-cert.gov/ncas/alerts/TA18-086A)

In order to avoid detection thresholds, adversaries may deliberately throttle password spraying attempts to avoid triggering security alerting. Additionally, adversaries may leverage LDAP and Kerberos authentication attempts, which are less likely to trigger high-visibility events such as Windows "logon failure" event ID 4625 that is commonly triggered by failed SMB connection attempts.[[3]](https://www.microsoft.com/en-us/security/blog/2024/10/31/chinese-threat-actor-storm-0940-uses-credentials-from-password-spray-attacks-from-a-covert-network/)

ID: T1110.003

Sub-technique of:  [T1110](https://attack.mitre.org/techniques/T1110)

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Containers, ESXi, IaaS, Identity Provider, Linux, Network Devices, Office Suite, SaaS, Windows, macOS

Contributors: John Strand; Microsoft Threat Intelligence Center (MSTIC)

Version: 1.8

Created: 11 February 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1110/003/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1110/003/)

## Procedure Examples

| ID  | Name  | Description

|  [G1030](https://attack.mitre.org/groups/G1030)   |  [Agrius](https://attack.mitre.org/groups/G1030)   |

[Agrius](https://attack.mitre.org/groups/G1030) engaged in password spraying via SMB in victim environments.[[4]](https://unit42.paloaltonetworks.com/agonizing-serpens-targets-israeli-tech-higher-ed-sectors/)

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary tasked Claude Code to attempt to password spray several database servers.[[5]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary has also instructed Claude Code to attempt to password spray a SCADA web interface.[[5]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has used a brute-force/password-spray tooling that operated in two modes: in password-spraying mode it conducted approximately four authentication attempts per hour per targeted account over the course of several days or weeks.[[6]](https://www.microsoft.com/security/blog/2020/09/10/strontium-detecting-new-patters-credential-harvesting/)[[7]](https://blogs.microsoft.com/on-the-issues/2020/09/10/cyberattacks-us-elections-trump-biden/) [APT28](https://attack.mitre.org/groups/G0007) has also used a Kubernetes cluster to conduct distributed, large-scale password spray attacks.[[8]](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)

|  [C0051](https://attack.mitre.org/campaigns/C0051)   |  [APT28 Nearest Neighbor Campaign](https://attack.mitre.org/campaigns/C0051)   |

During [APT28 Nearest Neighbor Campaign](https://attack.mitre.org/campaigns/C0051), [APT28](https://attack.mitre.org/groups/G0007) performed password-spray attacks against public facing services to validate credentials.[[9]](https://www.volexity.com/blog/2024/11/22/the-nearest-neighbor-attack-how-a-russian-apt-weaponized-nearby-wi-fi-networks-for-covert-access/)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has conducted brute force password spray attacks.[[10]](https://msrc-blog.microsoft.com/2021/06/25/new-nobelium-activity/)[[11]](https://www.microsoft.com/security/blog/2021/10/25/nobelium-targeting-delegated-administrative-privileges-to-facilitate-broader-attacks/)[[12]](https://www.ic3.gov/Media/News/2024/240226.pdf)

|  [G0064](https://attack.mitre.org/groups/G0064)   |  [APT33](https://attack.mitre.org/groups/G0064)   |

[APT33](https://attack.mitre.org/groups/G0064) has used password spraying to gain access to target systems.[[13]](https://www.fireeye.com/blog/threat-research/2018/12/overruled-containing-a-potentially-destructive-adversary.html)[[14]](https://www.microsoft.com/security/blog/2020/06/18/inside-microsoft-threat-protection-mapping-attack-chains-from-cloud-to-endpoint/)

|  [S0606](https://attack.mitre.org/software/S0606)   |  [Bad Rabbit](https://attack.mitre.org/software/S0606)   |

[Bad Rabbit](https://attack.mitre.org/software/S0606)’s infpub.dat file uses NTLM login credentials to brute force Windows machines.[[15]](https://securelist.com/bad-rabbit-ransomware/82851/)

|  [G0114](https://attack.mitre.org/groups/G0114)   |  [Chimera](https://attack.mitre.org/groups/G0114)   |

[Chimera](https://attack.mitre.org/groups/G0114) has used multiple password spraying attacks against victim's remote services to obtain valid user and administrator accounts.[[16]](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)

|  [S0488](https://attack.mitre.org/software/S0488)   |  [CrackMapExec](https://attack.mitre.org/software/S0488)   |

[CrackMapExec](https://attack.mitre.org/software/S0488) can brute force credential authentication by using a supplied list of usernames and a single password.[[17]](https://github.com/byt3bl33d3r/CrackMapExec/wiki/SMB-Command-Reference)

|  [G1003](https://attack.mitre.org/groups/G1003)   |  [Ember Bear](https://attack.mitre.org/groups/G1003)   |

[Ember Bear](https://attack.mitre.org/groups/G1003) has conducted password spraying against Outlook Web Access (OWA) infrastructure to identify valid user names and passwords.[[18]](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)

|  [G0125](https://attack.mitre.org/groups/G0125)   |  [HAFNIUM](https://attack.mitre.org/groups/G0125)   |

[HAFNIUM](https://attack.mitre.org/groups/G0125) has gained initial access through password spray attacks.[[19]](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)

|  [G1001](https://attack.mitre.org/groups/G1001)   |  [HEXANE](https://attack.mitre.org/groups/G1001)   |

[HEXANE](https://attack.mitre.org/groups/G1001) has used password spraying attacks to obtain valid credentials.[[20]](https://www.secureworks.com/blog/lyceum-takes-center-stage-in-middle-east-campaign)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) malware attempts to connect to Windows shares for lateral movement by using a generated list of usernames, which center around permutations of the username Administrator, and weak passwords.[[21]](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)[[22]](https://web.archive.org/web/20220608001455/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-RAT-and-Staging-Report.pdf)

|  [G0077](https://attack.mitre.org/groups/G0077)   |  [Leafminer](https://attack.mitre.org/groups/G0077)   |

[Leafminer](https://attack.mitre.org/groups/G0077) used a tool called Total SMB BruteForcer to perform internal password spraying.[[23]](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)

|  [S0362](https://attack.mitre.org/software/S0362)   |  [Linux Rabbit](https://attack.mitre.org/software/S0362)   |

[Linux Rabbit](https://attack.mitre.org/software/S0362) brute forces SSH passwords in order to attempt to gain access and install its malware onto the server. [[24]](https://www.anomali.com/blog/pulling-linux-rabbit-rabbot-malware-out-of-a-hat)

|  [S0413](https://attack.mitre.org/software/S0413)   |  [MailSniper](https://attack.mitre.org/software/S0413)   |

[MailSniper](https://attack.mitre.org/software/S0413) can be used for password spraying against Exchange and Office 365.[[25]](https://github.com/dafthack/MailSniper)

|  [G1059](https://attack.mitre.org/groups/G1059)   |  [NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059)   |

[NIMBUS MANTICORE](https://attack.mitre.org/groups/G1059) has conducted password spraying attacks against victim environments.[[26]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [C0055](https://attack.mitre.org/campaigns/C0055)   |  [Quad7 Activity](https://attack.mitre.org/campaigns/C0055)   |

[Quad7 Activity](https://attack.mitre.org/campaigns/C0055) has conducted a throttled variant of password spraying techniques that only utilized a single attempt to sign in within a 24-hour time period, eluding brute force detection thresholds.[[3]](https://www.microsoft.com/en-us/security/blog/2024/10/31/chinese-threat-actor-storm-0940-uses-credentials-from-password-spray-attacks-from-a-covert-network/)

|  [G0122](https://attack.mitre.org/groups/G0122)   |  [Silent Librarian](https://attack.mitre.org/groups/G0122)   |

[Silent Librarian](https://attack.mitre.org/groups/G0122) has used collected lists of names and e-mail accounts to use in password spraying attacks against private sector targets.[[27]](https://www.justice.gov/usao-sdny/press-release/file/1045781/download)

## Mitigations

| ID  | Mitigation  | Description

|  [M1036](https://attack.mitre.org/mitigations/M1036)   |  [Account Use Policies](https://attack.mitre.org/mitigations/M1036)   |

Set account lockout policies after a certain number of failed login attempts to prevent passwords from being guessed. Too strict a policy may create a denial of service condition and render environments un-usable, with all accounts used in the brute force being locked-out. Use conditional access policies to block logins from non-compliant devices or from outside defined organization IP ranges.[[28]](https://learn.microsoft.com/en-us/azure/active-directory/conditional-access/concept-conditional-access-policy-common) Consider blocking risky authentication requests, such as those originating from anonymizing services/proxies.[[29]](https://sec.okta.com/blockanonymizers)

|  [M1032](https://attack.mitre.org/mitigations/M1032)   |  [Multi-factor Authentication](https://attack.mitre.org/mitigations/M1032)   |

Use multi-factor authentication. Where possible, also enable multi-factor authentication on externally facing services.

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Refer to NIST guidelines when creating password policies. [[30]](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-63-4.pdf)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0487](https://attack.mitre.org/detectionstrategies/DET0487)   |  [Distributed Password Spraying via Authentication Failures Across Multiple Accounts](https://attack.mitre.org/detectionstrategies/DET0487)   |  [AN1336](https://attack.mitre.org/detectionstrategies/DET0487#AN1336)   |

A high volume of authentication failures using a single password (or small set) across many different user accounts within a defined time window

|  [AN1337](https://attack.mitre.org/detectionstrategies/DET0487#AN1337)   |

Authentication failures across different accounts using a repeated or similar password via SSH or PAM stack within a short window

|  [AN1338](https://attack.mitre.org/detectionstrategies/DET0487#AN1338)   |

Multiple failed login attempts across different users using common password patterns (e.g., 'Welcome2023')

|  [AN1339](https://attack.mitre.org/detectionstrategies/DET0487#AN1339)   |

Sign-in failures across enterprise SSO applications or SaaS platforms from same IP address using the same password against multiple user identities

|  [AN1340](https://attack.mitre.org/detectionstrategies/DET0487#AN1340)   |

Authentication failure logs on routers/switches showing repeated use of default or common passwords across multiple accounts

|  [AN1341](https://attack.mitre.org/detectionstrategies/DET0487#AN1341)   |

Repeated failed authentication attempts to container APIs, control planes, or login shells across many user names using same password

|  [AN1342](https://attack.mitre.org/detectionstrategies/DET0487#AN1342)   |

Failed authentication attempts across user mailboxes using identical or common passwords (e.g., OWA brute attempts)

|  [AN1343](https://attack.mitre.org/detectionstrategies/DET0487#AN1343)   |

SaaS applications receiving authentication failures for dozens of accounts using same password or login signature

## References

-    [Thyer, J. (2015, October 30). Password Spraying & Other Fun with RPCCLIENT. Retrieved April 25, 2017.](http://www.blackhillsinfosec.com/?p=4645)
-    [US-CERT. (2018, March 27). TA18-068A Brute Force Attacks Conducted by Cyber Actors. Retrieved October 2, 2019.](https://www.us-cert.gov/ncas/alerts/TA18-086A)
-    [Microsoft Threat Intelligence. (2024, October 31). Chinese threat actor Storm-0940 uses credentials from password spray attacks from a covert network. Retrieved June 4, 2025.](https://www.microsoft.com/en-us/security/blog/2024/10/31/chinese-threat-actor-storm-0940-uses-credentials-from-password-spray-attacks-from-a-covert-network/)
-    [Or Chechik, Tom Fakterman, Daniel Frank & Assaf Dahan. (2023, November 6). Agonizing Serpens (Aka Agrius) Targeting the Israeli Higher Education and Tech Sectors. Retrieved May 22, 2024.](https://unit42.paloaltonetworks.com/agonizing-serpens-targets-israeli-tech-higher-ed-sectors/)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)
-    [Microsoft Threat Intelligence Center (MSTIC). (2020, September 10). STRONTIUM: Detecting new patterns in credential harvesting. Retrieved September 11, 2020.](https://www.microsoft.com/security/blog/2020/09/10/strontium-detecting-new-patters-credential-harvesting/)
-    [Burt, T. (2020, September 10). New cyberattacks targeting U.S. elections. Retrieved March 24, 2021.](https://blogs.microsoft.com/on-the-issues/2020/09/10/cyberattacks-us-elections-trump-biden/)
-    [NSA, CISA, FBI, NCSC. (2021, July). Russian GRU Conducting Global Brute Force Campaign to Compromise Enterprise and Cloud Environments. Retrieved July 26, 2021.](https://media.defense.gov/2021/Jul/01/2002753896/-1/-1/1/CSA_GRU_GLOBAL_BRUTE_FORCE_CAMPAIGN_UOO158036-21.PDF)
-    [Koessel, Sean. Adair, Steven. Lancaster, Tom. (2024, November 22). The Nearest Neighbor Attack: How A Russian APT Weaponized Nearby Wi-Fi Networks for Covert Access. Retrieved February 25, 2025.](https://www.volexity.com/blog/2024/11/22/the-nearest-neighbor-attack-how-a-russian-apt-weaponized-nearby-wi-fi-networks-for-covert-access/)
-    [MSRC. (2021, June 25). New Nobelium activity. Retrieved August 4, 2021.](https://msrc-blog.microsoft.com/2021/06/25/new-nobelium-activity/)
-    [Microsoft Threat Intelligence Center. (2021, October 25). NOBELIUM targeting delegated administrative privileges to facilitate broader attacks. Retrieved March 25, 2022.](https://www.microsoft.com/security/blog/2021/10/25/nobelium-targeting-delegated-administrative-privileges-to-facilitate-broader-attacks/)
-    [UK National Cyber Security Center et al. (2024, February). SVR cyber actors adapt tactics for initial cloud access. Retrieved March 1, 2024.](https://www.ic3.gov/Media/News/2024/240226.pdf)
-    [Ackerman, G., et al. (2018, December 21). OVERRULED: Containing a Potentially Destructive Adversary. Retrieved January 17, 2019.](https://www.fireeye.com/blog/threat-research/2018/12/overruled-containing-a-potentially-destructive-adversary.html)
-    [Microsoft Threat Protection Intelligence Team. (2020, June 18). Inside Microsoft Threat Protection: Mapping attack chains from cloud to endpoint. Retrieved June 22, 2020.](https://www.microsoft.com/security/blog/2020/06/18/inside-microsoft-threat-protection-mapping-attack-chains-from-cloud-to-endpoint/)
-    [Mamedov, O. Sinitsyn, F. Ivanov, A.. (2017, October 24). Bad Rabbit ransomware. Retrieved January 28, 2021.](https://securelist.com/bad-rabbit-ransomware/82851/)

-    [Jansen, W . (2021, January 12). Abusing cloud services to fly under the radar. Retrieved September 12, 2024.](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)
-    [byt3bl33d3r. (2018, September 8). SMB: Command Reference. Retrieved July 17, 2020.](https://github.com/byt3bl33d3r/CrackMapExec/wiki/SMB-Command-Reference)
-    [US Cybersecurity & Infrastructure Security Agency et al. (2024, September 5). Russian Military Cyber Actors Target U.S. and Global Critical Infrastructure. Retrieved September 6, 2024.](https://www.cisa.gov/sites/default/files/2024-09/aa24-249a-russian-military-cyber-actors-target-us-and-global-critical-infrastructure.pdf)
-    [Microsoft Threat Intelligence . (2025, March 5). Silk Typhoon targeting IT supply chain. Retrieved March 20, 2025.](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)
-    [SecureWorks 2019, August 27 LYCEUM Takes Center Stage in Middle East Campaign Retrieved. 2019/11/19](https://www.secureworks.com/blog/lyceum-takes-center-stage-in-middle-east-campaign)
-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Unraveling the Long Thread of the Sony Attack. Retrieved February 25, 2016.](https://web.archive.org/web/20160226161828/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-Report.pdf)
-    [Novetta Threat Research Group. (2016, February 24). Operation Blockbuster: Remote Administration Tools & Content Staging Malware Report. Retrieved March 16, 2016.](https://web.archive.org/web/20220608001455/https://www.operationblockbuster.com/wp-content/uploads/2016/02/Operation-Blockbuster-RAT-and-Staging-Report.pdf)
-    [Symantec Security Response. (2018, July 25). Leafminer: New Espionage Campaigns Targeting Middle Eastern Regions. Retrieved August 28, 2018.](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)
-    [Anomali Labs. (2018, December 6). Pulling Linux Rabbit/Rabbot Malware Out of a Hat. Retrieved March 4, 2019.](https://www.anomali.com/blog/pulling-linux-rabbit-rabbot-malware-out-of-a-hat)
-    [Bullock, B., . (2018, November 20). MailSniper. Retrieved October 4, 2019.](https://github.com/dafthack/MailSniper)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [DOJ. (2018, March 23). U.S. v. Rafatnejad et al . Retrieved February 3, 2021.](https://www.justice.gov/usao-sdny/press-release/file/1045781/download)
-    [Microsoft. (2022, December 14). Conditional Access templates. Retrieved February 21, 2023.](https://learn.microsoft.com/en-us/azure/active-directory/conditional-access/concept-conditional-access-policy-common)
-    [Moussa Diallo and Brett Winterford. (2024, April 26). How to Block Anonymizing Services using Okta. Retrieved May 28, 2024.](https://sec.okta.com/blockanonymizers)
-    [Temoshok, D., et al. (July 2025). SP 800-63-4, Digital Identity Guidelines. Retrieved July 29, 2026.](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-63-4.pdf)

×
