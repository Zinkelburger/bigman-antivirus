---
url: "https://attack.mitre.org/techniques/T1111/"
title: "Multi-Factor Authentication Interception, Technique T1111 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:34:44Z"
via: "live"
sha256: "fc52f147b21ad76fe40b51fcd16f440f7b5dce0519ec702a4a2a958a7aef7362"
wayback: "https://web.archive.org/web/20261023090824/https://attack.mitre.org/techniques/T1111/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Multi-Factor Authentication Interception, Technique T1111 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1111/>
> Snapshot: <https://web.archive.org/web/20261023090824/https://attack.mitre.org/techniques/T1111/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Multi-Factor Authentication Interception

#  Multi-Factor Authentication Interception

Adversaries may target multi-factor authentication (MFA) mechanisms, (i.e., smart cards, token generators, etc.) to gain access to credentials that can be used to access systems, services, and network resources. Use of MFA is recommended and provides a higher level of security than usernames and passwords alone, but organizations should be aware of techniques that could be used to intercept and bypass these security mechanisms.

If a smart card is used for multi-factor authentication, then a keylogger will need to be used to obtain the password associated with a smart card during normal use. With both an inserted card and access to the smart card password, an adversary can connect to a network resource using the infected system to proxy the authentication with the inserted hardware token. [[1]](https://dl.mandiant.com/EE/assets/PDF_MTrends_2011.pdf)

Adversaries may also employ a keylogger to similarly target other hardware tokens, such as RSA SecurID. Capturing token input (including a user's personal identification code) may provide temporary access (i.e. replay the one-time passcode until the next value rollover) as well as possibly enabling adversaries to reliably predict future authentication values (given access to both the algorithm and any seed values used to generate appended temporary codes). [[2]](https://www.route-fifty.com/cybersecurity/2011/06/rsa-confirms-its-tokens-used-in-lockheed-hack/282818/)

Other methods of MFA may be intercepted and used by an adversary to authenticate. It is common for one-time codes to be sent via out-of-band communications (email, SMS). If the device and/or service is not secured, then it may be vulnerable to interception. Service providers can also be targeted: for example, an adversary may compromise an SMS messaging service in order to steal MFA codes sent to users’ phones.[[3]](https://sec.okta.com/scatterswine)

ID: T1111

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: John Lambert, Microsoft Threat Intelligence Center

Version: 2.1

Created: 31 May 2017

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1111/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1111/)

## Procedure Examples

| ID  | Name  | Description

|  [G1044](https://attack.mitre.org/groups/G1044)   |  [APT42](https://attack.mitre.org/groups/G1044)   |

[APT42](https://attack.mitre.org/groups/G1044) has intercepted SMS-based one-time passwords and has set up two-factor authentication.[[4]](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf) Additionally, [APT42](https://attack.mitre.org/groups/G1044) has used cloned or fake websites to capture MFA tokens.[[5]](https://cloud.google.com/blog/topics/threat-intelligence/untangling-iran-apt42-operations)

|  [G0114](https://attack.mitre.org/groups/G0114)   |  [Chimera](https://attack.mitre.org/groups/G0114)   |

[Chimera](https://attack.mitre.org/groups/G0114) has registered alternate phone numbers for compromised users to intercept 2FA codes sent via SMS.[[6]](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)

|  [S9003](https://attack.mitre.org/software/S9003)   |  [evilginx2](https://attack.mitre.org/software/S9003)   |

[evilginx2](https://attack.mitre.org/software/S9003) can intercept authentication tokens to enable bypass of non-phishing resistant forms of MFA.[[7]](https://breakdev.org/evilginx-2-next-generation-of-phishing-2fa-tokens/)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has used a proprietary tool to intercept one time passwords required for two-factor authentication.[[8]](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)

|  [G1004](https://attack.mitre.org/groups/G1004)   |  [LAPSUS$](https://attack.mitre.org/groups/G1004)   |

[LAPSUS$](https://attack.mitre.org/groups/G1004) has replayed stolen session token and passwords to trigger simple-approval MFA prompts in hope of the legitimate user will grant necessary approval.[[9]](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)

|  [C0049](https://attack.mitre.org/campaigns/C0049)   |  [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049)   |

[Leviathan](https://attack.mitre.org/groups/G0065) abused compromised appliance access to collect multifactor authentication token values during [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049).[[10]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors used a custom collection method to intercept two-factor authentication soft tokens.[[11]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [S1104](https://attack.mitre.org/software/S1104)   |  [SLOWPULSE](https://attack.mitre.org/software/S1104)   |

[SLOWPULSE](https://attack.mitre.org/software/S1104) can log credentials on compromised Pulse Secure VPNs during the DSAuth::AceAuthServer::checkUsernamePasswordACE-2FA authentication procedure.[[12]](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)

|  [S0018](https://attack.mitre.org/software/S0018)   |  [Sykipot](https://attack.mitre.org/software/S0018)   |

[Sykipot](https://attack.mitre.org/software/S0018) is known to contain functionality that enables targeting of smart card technologies to proxy authentication for connections to restricted network resources using detected hardware tokens.[[13]](https://www.alienvault.com/open-threat-exchange/blog/sykipot-variant-hijacks-dod-and-windows-smart-cards)

## Mitigations

| ID  | Mitigation  | Description

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Remove smart cards when not in use.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0246](https://attack.mitre.org/detectionstrategies/DET0246)   |  [Detection Strategy for MFA Interception via Input Capture and Smart Card Proxying](https://attack.mitre.org/detectionstrategies/DET0246)   |  [AN0687](https://attack.mitre.org/detectionstrategies/DET0246#AN0687)   |

Behavior chain involving unexpected API calls to capture keyboard input, driver loads for keyloggers, or remote use of smart card authentication via logon sessions not initiated by local user interaction

|  [AN0688](https://attack.mitre.org/detectionstrategies/DET0246#AN0688)   |

Detection of unauthorized keylogger behavior through access to /dev/input, loading kernel modules (e.g., via insmod), or polling user input devices from non-user shells

|  [AN0689](https://attack.mitre.org/detectionstrategies/DET0246#AN0689)   |

Processes accessing TCC-protected input APIs or polling HID services without user interaction, or dynamically loaded keylogging frameworks using accessibility privileges

## References

-    [Mandiant. (2011, January 27). Mandiant M-Trends 2011. Retrieved January 10, 2016.](https://dl.mandiant.com/EE/assets/PDF_MTrends_2011.pdf)
-    [Jackson, William. (2011, June 7). RSA confirms its tokens used in Lockheed hack. Retrieved November 17, 2024.](https://www.route-fifty.com/cybersecurity/2011/06/rsa-confirms-its-tokens-used-in-lockheed-hack/282818/)
-    [Okta. (2022, August 25). Detecting Scatter Swine: Insights into a Relentless Phishing Campaign. Retrieved February 24, 2023.](https://sec.okta.com/scatterswine)
-    [Mandiant. (n.d.). APT42: Crooked Charms, Cons and Compromises. Retrieved October 9, 2024.](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf)
-    [Rozmann, O., et al. (2024, May 1). Uncharmed: Untangling Iran's APT42 Operations. Retrieved October 9, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/untangling-iran-apt42-operations)
-    [Jansen, W . (2021, January 12). Abusing cloud services to fly under the radar. Retrieved September 12, 2024.](https://web.archive.org/web/20230218064220/https://research.nccgroup.com/2021/01/12/abusing-cloud-services-to-fly-under-the-radar/)
-    [Gretzky, K.. (2018, July 26). Evilginx 2 - Next Generation of Phishing 2FA Tokens. Retrieved October 14, 2019.](https://breakdev.org/evilginx-2-next-generation-of-phishing-2fa-tokens/)

-    [KISA. (2021). Phishing Target Reconnaissance and Attack Resource Analysis Operation Muzabi. Retrieved March 8, 2024.](https://web.archive.org/web/20220328121326/https://boho.or.kr/filedownload.do?attach_file_seq=2695&attach_file_id=EpF2695.pdf)
-    [MSTIC, DART, M365 Defender. (2022, March 24). DEV-0537 Criminal Actor Targeting Organizations for Data Exfiltration and Destruction. Retrieved May 17, 2022.](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)
-    [CISA et al. (2024, July 8). People’s Republic of China (PRC) Ministry of State Security APT40 Tradecraft in Action. Retrieved February 3, 2025.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [Perez, D. et al. (2021, April 20). Check Your Pulse: Suspected APT Actors Leverage Authentication Bypass Techniques and Pulse Secure Zero-Day. Retrieved February 5, 2024.](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)
-    [Blasco, J. (2012, January 12). Sykipot variant hijacks DOD and Windows smart cards. Retrieved January 10, 2016.](https://www.alienvault.com/open-threat-exchange/blog/sykipot-variant-hijacks-dod-and-windows-smart-cards)

×
