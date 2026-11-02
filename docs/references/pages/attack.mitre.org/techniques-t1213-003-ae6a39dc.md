---
url: "https://attack.mitre.org/techniques/T1213/003/"
title: "Data from Information Repositories: Code Repositories, Sub-technique T1213.003 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:41:51Z"
via: "live"
sha256: "a52bf24a2cc943bde7c6c7eaa958b7c25847b1e9dacfbc2617b56c576e9c6d76"
wayback: "https://web.archive.org/web/20261015013125/https://attack.mitre.org/techniques/T1213/003/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Data from Information Repositories: Code Repositories, Sub-technique T1213.003 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1213/003/>
> Snapshot: <https://web.archive.org/web/20261015013125/https://attack.mitre.org/techniques/T1213/003/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Data from Information Repositories](https://attack.mitre.org/techniques/T1213)
- Code Repositories

#  Data from Information Repositories: Code Repositories

##### Other sub-techniques of Data from Information Repositories (6)

| ID  | Name

|  [T1213.001](https://attack.mitre.org/techniques/T1213/001/)   |  [Confluence](https://attack.mitre.org/techniques/T1213/001/)

|  [T1213.002](https://attack.mitre.org/techniques/T1213/002/)   |  [Sharepoint](https://attack.mitre.org/techniques/T1213/002/)

|  T1213.003   |  Code Repositories

|  [T1213.004](https://attack.mitre.org/techniques/T1213/004/)   |  [Customer Relationship Management Software](https://attack.mitre.org/techniques/T1213/004/)

|  [T1213.005](https://attack.mitre.org/techniques/T1213/005/)   |  [Messaging Applications](https://attack.mitre.org/techniques/T1213/005/)

|  [T1213.006](https://attack.mitre.org/techniques/T1213/006/)   |  [Databases](https://attack.mitre.org/techniques/T1213/006/)

Adversaries may leverage code repositories to collect valuable information. Code repositories are tools/services that store source code and automate software builds. They may be hosted internally or privately on third party sites such as Github, GitLab, SourceForge, and BitBucket. Users typically interact with code repositories through a web application or command-line utilities such as git.

Once adversaries gain access to a victim network or a private code repository, they may collect sensitive information such as proprietary source code or [Unsecured Credentials](https://attack.mitre.org/techniques/T1552) contained within software's source code. Having access to software's source code may allow adversaries to develop [Exploits](https://attack.mitre.org/techniques/T1587/004), while credentials may provide access to additional resources using [Valid Accounts](https://attack.mitre.org/techniques/T1078).[[1]](https://www.wired.com/story/uber-paid-off-hackers-to-hide-a-57-million-user-data-breach/)[[2]](https://krebsonsecurity.com/2013/10/adobe-to-announce-source-code-customer-data-breach/)

Note: This is distinct from [Code Repositories](https://attack.mitre.org/techniques/T1593/003), which focuses on conducting [Reconnaissance](https://attack.mitre.org/tactics/TA0043) via public code repositories.

ID: T1213.003

Sub-technique of:  [T1213](https://attack.mitre.org/techniques/T1213)

ⓘ

Tactic: [Collection](https://attack.mitre.org/tactics/TA0009)

ⓘ

Platforms: SaaS

Contributors: Itamar Mizrahi, Cymptom; Josh Liburdi, @jshlbrd; Toby Kohlenberg

Version: 1.2

Created: 11 May 2021

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1213/003/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1213/003/)

## Procedure Examples

| ID  | Name  | Description

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) cloned victim user Git repositories during intrusions.[[3]](https://www.group-ib.com/blog/apt41-world-tour-2021/)

|  [S9010](https://attack.mitre.org/software/S9010)   |  [GlassWorm](https://attack.mitre.org/software/S9010)   |

[GlassWorm](https://attack.mitre.org/software/S9010) has gathered code repository authentication materials for NPM and GitHub.[[4]](https://www.koi.ai/blog/glassworm-goes-mac-fresh-infrastructure-new-tricks)[[5]](https://www.koi.ai/blog/glassworm-first-self-propagating-worm-using-invisible-code-hits-openvsx-marketplace)[[6]](https://socket.dev/blog/glassworm-loader-hits-open-vsx-via-suspected-developer-account-compromise) [GlassWorm](https://attack.mitre.org/software/S9010) has collected details pertaining to the npm configuration data for _authToken.[[5]](https://www.koi.ai/blog/glassworm-first-self-propagating-worm-using-invisible-code-hits-openvsx-marketplace)[[6]](https://socket.dev/blog/glassworm-loader-hits-open-vsx-via-suspected-developer-account-compromise)

|  [G1004](https://attack.mitre.org/groups/G1004)   |  [LAPSUS$](https://attack.mitre.org/groups/G1004)   |

[LAPSUS$](https://attack.mitre.org/groups/G1004) has searched a victim's network for code repositories like GitLab and GitHub to discover further high-privilege account credentials.[[7]](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)[[8]](https://www.nccgroup.com/research-blog/lapsus-recent-techniques-tactics-and-procedures/)

|  [S9043](https://attack.mitre.org/software/S9043)   |  [Mini Shai-Hulud](https://attack.mitre.org/software/S9043)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has gathered and downloaded data stored on both compromised and publicly accessible code repositories.[[9]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  [G1015](https://attack.mitre.org/groups/G1015)   |  [Scattered Spider](https://attack.mitre.org/groups/G1015)   |

[Scattered Spider](https://attack.mitre.org/groups/G1015) enumerates data stored within victim code repositories, such as internal GitHub repositories.[[10]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-320a)[[11]](https://www.microsoft.com/en-us/security/blog/2023/10/25/octo-tempest-crosses-boundaries-to-facilitate-extortion-encryption-and-destruction/)

|  [S9008](https://attack.mitre.org/software/S9008)   |  [Shai-Hulud](https://attack.mitre.org/software/S9008)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has downloaded existing packages from code repositories and extracted data stored within them.[[12]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)

|  [G1057](https://attack.mitre.org/groups/G1057)   |  [ShinyHunters](https://attack.mitre.org/groups/G1057)   |

[ShinyHunters](https://attack.mitre.org/groups/G1057) has gathered information from and has searched for vulnerabilities in the target company’s GitHub repository source code.[[13]](https://www.intel471.com/blog/shinyhunters-data-breach-mitre-attack)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) downloaded source code from code repositories.[[14]](https://msrc-blog.microsoft.com/2021/02/18/microsoft-internal-solorigate-investigation-final-update/)

|  [S9041](https://attack.mitre.org/software/S9041)   |  [TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041)   |

[TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041) can target sensitive file paths in Git repos to extract credentials.[[15]](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)[[16]](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)

|  [S9009](https://attack.mitre.org/software/S9009)   |  [TruffleHog](https://attack.mitre.org/software/S9009)   |

[TruffleHog](https://attack.mitre.org/software/S9009) has gathered data and credentials from code repositories.[[17]](https://github.com/trufflesecurity/trufflehog)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Consider periodic reviews of accounts and privileges for critical and sensitive code repositories. Scan code repositories for exposed credentials or other sensitive information.

|  [M1032](https://attack.mitre.org/mitigations/M1032)   |  [Multi-factor Authentication](https://attack.mitre.org/mitigations/M1032)   |

Use multi-factor authentication for logons to code repositories.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Enforce the principle of least-privilege. Consider implementing access control mechanisms that include both authentication and authorization for code repositories.

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Develop and publish policies that define acceptable information to be stored in code repositories.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0263](https://attack.mitre.org/detectionstrategies/DET0263)   |  [Detecting Bulk or Anomalous Access to Private Code Repositories via SaaS Platforms](https://attack.mitre.org/detectionstrategies/DET0263)   |  [AN0732](https://attack.mitre.org/detectionstrategies/DET0263#AN0732)   |

Anomalous or bulk download activity from private or restricted repositories by non-developer or privileged accounts, often preceded by unusual login behavior (e.g., unfamiliar geo, OAuth token use, elevated API rate).

## References

-    [Andy Greenberg. (2017, January 21). Hack Brief: Uber Paid Off Hackers to Hide a 57-Million User Data Breach. Retrieved May 14, 2021.](https://www.wired.com/story/uber-paid-off-hackers-to-hide-a-57-million-user-data-breach/)
-    [Brian Krebs. (2013, October 3). Adobe To Announce Source Code, Customer Data Breach. Retrieved May 17, 2021.](https://krebsonsecurity.com/2013/10/adobe-to-announce-source-code-customer-data-breach/)
-    [Nikita Rostovcev. (2022, August 18). APT41 World Tour 2021 on a tight schedule. Retrieved February 22, 2024.](https://www.group-ib.com/blog/apt41-world-tour-2021/)
-    [Gal Hachamov. (2025, December 29). GlassWorm Goes Mac: Fresh Infrastructure, New Tricks. Retrieved April 10, 2026.](https://www.koi.ai/blog/glassworm-goes-mac-fresh-infrastructure-new-tricks)
-    [Idan Dardikman. (2025, October 18). GlassWorm: First Self-Propagating Worm Using Invisible Code Hits OpenVSX Marketplace. Retrieved April 10, 2026.](https://www.koi.ai/blog/glassworm-first-self-propagating-worm-using-invisible-code-hits-openvsx-marketplace)
-    [Kirill Boychenko. (2026, January 31). GlassWorm Loader Hits Open VSX via Developer Account Compromise. Retrieved April 10, 2026.](https://socket.dev/blog/glassworm-loader-hits-open-vsx-via-suspected-developer-account-compromise)
-    [MSTIC, DART, M365 Defender. (2022, March 24). DEV-0537 Criminal Actor Targeting Organizations for Data Exfiltration and Destruction. Retrieved May 17, 2022.](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)
-    [Brown, D., et al. (2022, April 28). LAPSUS$: Recent techniques, tactics and procedures. Retrieved December 22, 2022.](https://www.nccgroup.com/research-blog/lapsus-recent-techniques-tactics-and-procedures/)
-    [Webb, M. (2026, May 20). TeamPCP Wave Four: GitHub Breach via Poisoned VS Code Extension, durabletask PyPI Worm, and ~4,000 Internal Repositories Exfiltrated. Retrieved July 16, 2026.](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

-    [CISA. (2023, November 16). Cybersecurity Advisory: Scattered Spider (AA23-320A). Retrieved March 18, 2024.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-320a)
-    [Microsoft. (2023, October 25). Octo Tempest crosses boundaries to facilitate extortion, encryption, and destruction. Retrieved March 18, 2024.](https://www.microsoft.com/en-us/security/blog/2023/10/25/octo-tempest-crosses-boundaries-to-facilitate-extortion-encryption-and-destruction/)
-    [Charlie Eriksen. (2025, September 16). S1ngularity/nx attackers strike again. Retrieved April 9, 2026.](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)
-    [Intel 471. (2021, August 23). Here’s how to guard your enterprise against ShinyHunters. Retrieved July 29, 2026.](https://www.intel471.com/blog/shinyhunters-data-breach-mitre-attack)
-    [MSRC Team. (2021, February 18). Microsoft Internal Solorigate Investigation – Final Update. Retrieved May 14, 2021.](https://msrc-blog.microsoft.com/2021/02/18/microsoft-internal-solorigate-investigation-final-update/)
-    [McCarthy, R. (2026, March 20). Trivy Compromised: Everything You Need to Know about the Latest Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)
-    [Aqua Team. (2026, April 1). Update: Ongoing Investigation and Continued Remediation. Retrieved July 1, 2026.](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)
-    [Trufflesecurity. (2026, April 8). TruffleHog Enterprise. Retrieved April 15, 2026.](https://github.com/trufflesecurity/trufflehog)

×
