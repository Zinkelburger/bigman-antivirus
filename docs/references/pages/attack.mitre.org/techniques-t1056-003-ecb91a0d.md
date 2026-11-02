---
url: "https://attack.mitre.org/techniques/T1056/003/"
title: "Input Capture: Web Portal Capture, Sub-technique T1056.003 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:30:02Z"
via: "live"
sha256: "2cd0780795b147f9e5bcf7bf046cdb73502f71d516c57db16fcc0982e812683f"
wayback: "https://web.archive.org/web/20261023090824/https://attack.mitre.org/techniques/T1056/003/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Input Capture: Web Portal Capture, Sub-technique T1056.003 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1056/003/>
> Snapshot: <https://web.archive.org/web/20261023090824/https://attack.mitre.org/techniques/T1056/003/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Input Capture](https://attack.mitre.org/techniques/T1056)
- Web Portal Capture

#  Input Capture: Web Portal Capture

##### Other sub-techniques of Input Capture (4)

| ID  | Name

|  [T1056.001](https://attack.mitre.org/techniques/T1056/001/)   |  [Keylogging](https://attack.mitre.org/techniques/T1056/001/)

|  [T1056.002](https://attack.mitre.org/techniques/T1056/002/)   |  [GUI Input Capture](https://attack.mitre.org/techniques/T1056/002/)

|  T1056.003   |  Web Portal Capture

|  [T1056.004](https://attack.mitre.org/techniques/T1056/004/)   |  [Credential API Hooking](https://attack.mitre.org/techniques/T1056/004/)

Adversaries may install code on externally facing portals, such as a VPN login page, to capture and transmit credentials of users who attempt to log into the service. For example, a compromised login page may log provided user credentials before logging the user in to the service.

This variation on input capture may be conducted post-compromise using legitimate administrative access as a backup measure to maintain network access through [External Remote Services](https://attack.mitre.org/techniques/T1133) and [Valid Accounts](https://attack.mitre.org/techniques/T1078) or as part of the initial compromise by exploitation of the externally facing web service.[[1]](https://www.volexity.com/blog/2015/10/07/virtual-private-keylogging-cisco-web-vpns-leveraged-for-access-and-persistence/)

ID: T1056.003

Sub-technique of:  [T1056](https://attack.mitre.org/techniques/T1056)

ⓘ

Tactics: [Collection](https://attack.mitre.org/tactics/TA0009), [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Linux, Windows, macOS

Version: 1.1

Created: 11 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1056/003/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1056/003/)

## Procedure Examples

| ID  | Name  | Description

|  [C0029](https://attack.mitre.org/campaigns/C0029)   |  [Cutting Edge](https://attack.mitre.org/campaigns/C0029)   |

During [Cutting Edge](https://attack.mitre.org/campaigns/C0029), threat actors modified the JavaScript loaded by the Ivanti Connect Secure login page to capture credentials entered.[[2]](https://www.volexity.com/blog/2024/01/10/active-exploitation-of-two-zero-day-vulnerabilities-in-ivanti-connect-secure-vpn/)

|  [S1022](https://attack.mitre.org/software/S1022)   |  [IceApple](https://attack.mitre.org/software/S1022)   |

The [IceApple](https://attack.mitre.org/software/S1022) OWA credential logger can monitor for OWA authentication requests and log the credentials.[[3]](https://www.crowdstrike.com/wp-content/uploads/2022/05/crowdstrike-iceapple-a-novel-internet-information-services-post-exploitation-framework.pdf)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has collected credentials from a fake Google account login page.[[4]](https://www.ic3.gov/CSA/2026/260108.pdf)

|  [C0030](https://attack.mitre.org/campaigns/C0030)   |  [Triton Safety Instrumented System Attack](https://attack.mitre.org/campaigns/C0030)   |

In the [Triton Safety Instrumented System Attack](https://attack.mitre.org/campaigns/C0030), [TEMP.Veles](https://attack.mitre.org/groups/G0088) captured credentials as they were being changed by redirecting text-based login codes to websites they controlled.[[5]](https://www.eenews.net/articles/the-inside-story-of-the-worlds-most-dangerous-malware/)

|  [S1116](https://attack.mitre.org/software/S1116)   |  [WARPWIRE](https://attack.mitre.org/software/S1116)   |

[WARPWIRE](https://attack.mitre.org/software/S1116) can capture credentials submitted during the web logon process in order to access layer seven applications such as RDP.[[6]](https://www.mandiant.com/resources/blog/suspected-apt-targets-ivanti-zero-day)

|  [G1035](https://attack.mitre.org/groups/G1035)   |  [Winter Vivern](https://attack.mitre.org/groups/G1035)   |

[Winter Vivern](https://attack.mitre.org/groups/G1035) registered and hosted domains to allow for creation of web pages mimicking legitimate government email logon sites to collect logon information.[[7]](https://www.sentinelone.com/labs/winter-vivern-uncovering-a-wave-of-global-espionage/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Do not allow administrator accounts that have permissions to modify the Web content of organization login portals to be used for day-to-day operations that may expose them to potential adversaries on unprivileged systems.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0480](https://attack.mitre.org/detectionstrategies/DET0480)   |  [Detection of Credential Harvesting via Web Portal Modification](https://attack.mitre.org/detectionstrategies/DET0480)   |  [AN1320](https://attack.mitre.org/detectionstrategies/DET0480#AN1320)   |

Detects unauthorized modifications to login-facing web server files (e.g., index.php, login.js) typically tied to VPN, SSO, or intranet portals. Correlates suspicious file changes with remote access artifacts or web shell behavior.

|  [AN1321](https://attack.mitre.org/detectionstrategies/DET0480#AN1321)   |

Detects tampering of IIS-based login pages (e.g., default.aspx, login.aspx) tied to VPN, OWA, or SharePoint via script injection or unexpected editor processes modifying web roots.

|  [AN1322](https://attack.mitre.org/detectionstrategies/DET0480#AN1322)   |

Detects unauthorized changes to locally hosted login pages on macOS (common in developer VPN environments) and links file edits to cron jobs, background scripts, or SUID binaries.

## References

-    [Adair, S. (2015, October 7). Virtual Private Keylogging: Cisco Web VPNs Leveraged for Access and Persistence. Retrieved March 20, 2017.](https://www.volexity.com/blog/2015/10/07/virtual-private-keylogging-cisco-web-vpns-leveraged-for-access-and-persistence/)
-    [Meltzer, M. et al. (2024, January 10). Active Exploitation of Two Zero-Day Vulnerabilities in Ivanti Connect Secure VPN. Retrieved February 27, 2024.](https://www.volexity.com/blog/2024/01/10/active-exploitation-of-two-zero-day-vulnerabilities-in-ivanti-connect-secure-vpn/)
-    [CrowdStrike. (2022, May). ICEAPPLE: A NOVEL INTERNET INFORMATION SERVICES (IIS) POST-EXPLOITATION FRAMEWORK. Retrieved June 27, 2022.](https://www.crowdstrike.com/wp-content/uploads/2022/05/crowdstrike-iceapple-a-novel-internet-information-services-post-exploitation-framework.pdf)
-    [FBI. (2026, January 8). FBI Flash AC-000001-MW North Korean Kimsuky Actors Leverage Malicious QR Codes in Spearphishing Campaigns Targeting U.S. Entities. Retrieved April 18, 2026.](https://www.ic3.gov/CSA/2026/260108.pdf)

-    [Blake Sobczak. (2019, March 7). The inside story of the world’s most dangerous malware. Retrieved March 25, 2024.](https://www.eenews.net/articles/the-inside-story-of-the-worlds-most-dangerous-malware/)
-    [McLellan, T. et al. (2024, January 12). Cutting Edge: Suspected APT Targets Ivanti Connect Secure VPN in New Zero-Day Exploitation. Retrieved February 27, 2024.](https://www.mandiant.com/resources/blog/suspected-apt-targets-ivanti-zero-day)
-    [Tom Hegel. (2023, March 16). Winter Vivern | Uncovering a Wave of Global Espionage. Retrieved July 29, 2024.](https://www.sentinelone.com/labs/winter-vivern-uncovering-a-wave-of-global-espionage/)

×
