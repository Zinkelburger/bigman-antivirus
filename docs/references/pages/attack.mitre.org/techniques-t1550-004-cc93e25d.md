---
url: "https://attack.mitre.org/techniques/T1550/004/"
title: "Use Alternate Authentication Material: Web Session Cookie, Sub-technique T1550.004 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:51:27Z"
via: "live"
sha256: "65afd9073f7971f5cb209532dd969011727c8ac6065d48ef9e8b25f8cd79731d"
wayback: "https://web.archive.org/web/20261023090829/https://attack.mitre.org/techniques/T1550/004/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Use Alternate Authentication Material: Web Session Cookie, Sub-technique T1550.004 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1550/004/>
> Snapshot: <https://web.archive.org/web/20261023090829/https://attack.mitre.org/techniques/T1550/004/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Use Alternate Authentication Material](https://attack.mitre.org/techniques/T1550)
- Web Session Cookie

#  Use Alternate Authentication Material: Web Session Cookie

##### Other sub-techniques of Use Alternate Authentication Material (4)

| ID  | Name

|  [T1550.001](https://attack.mitre.org/techniques/T1550/001/)   |  [Application Access Token](https://attack.mitre.org/techniques/T1550/001/)

|  [T1550.002](https://attack.mitre.org/techniques/T1550/002/)   |  [Pass the Hash](https://attack.mitre.org/techniques/T1550/002/)

|  [T1550.003](https://attack.mitre.org/techniques/T1550/003/)   |  [Pass the Ticket](https://attack.mitre.org/techniques/T1550/003/)

|  T1550.004   |  Web Session Cookie

Adversaries can use stolen session cookies to authenticate to web applications and services. This technique bypasses some multi-factor authentication protocols since the session is already authenticated.[[1]](https://wunderwuzzi23.github.io/blog/passthecookie.html)

Authentication cookies are commonly used in web applications, including cloud-based services, after a user has authenticated to the service so credentials are not passed and re-authentication does not need to occur as frequently. Cookies are often valid for an extended period of time, even if the web application is not actively used. After the cookie is obtained through [Steal Web Session Cookie](https://attack.mitre.org/techniques/T1539) or [Web Cookies](https://attack.mitre.org/techniques/T1606/001), the adversary may then import the cookie into a browser they control and is then able to use the site or application as the user for as long as the session cookie is active. Once logged into the site, an adversary can access sensitive information, read email, or perform actions that the victim account has permissions to perform.

There have been examples of malware targeting session cookies to bypass multi-factor authentication systems.[[2]](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)

ID: T1550.004

Sub-technique of:  [T1550](https://attack.mitre.org/techniques/T1550)

ⓘ

Tactic: [Lateral Movement](https://attack.mitre.org/tactics/TA0008)

ⓘ

Platforms: IaaS, Office Suite, SaaS

Contributors: Jack Burns, HubSpot; Johann Rehberger

Version: 2.0

Created: 30 January 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1550/004/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1550/004/)

## Procedure Examples

| ID  | Name  | Description

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) used stolen cookies to access cloud resources and a forged duo-sid cookie to bypass MFA set on an email account.[[3]](https://www.volexity.com/blog/2020/12/14/dark-halo-leverages-solarwinds-compromise-to-breach-organizations/)[[4]](https://www.crowdstrike.com/blog/observations-from-the-stellarparticle-campaign/)

|  [G1033](https://attack.mitre.org/groups/G1033)   |  [Star Blizzard](https://attack.mitre.org/groups/G1033)   |

[Star Blizzard](https://attack.mitre.org/groups/G1033) has bypassed multi-factor authentication on victim email accounts by using session cookies stolen using EvilGinx.[[5]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-341a)

## Mitigations

| ID  | Mitigation  | Description

|  [M1054](https://attack.mitre.org/mitigations/M1054)   |  [Software Configuration](https://attack.mitre.org/mitigations/M1054)   |

Configure browsers or tasks to regularly delete persistent cookies.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0074](https://attack.mitre.org/detectionstrategies/DET0074)   |  [Detect Use of Stolen Web Session Cookies Across Platforms](https://attack.mitre.org/detectionstrategies/DET0074)   |  [AN0201](https://attack.mitre.org/detectionstrategies/DET0074#AN0201)   |

Anomalous access to cloud web applications using session tokens without corresponding MFA/credential validation, often from unusual locations or device fingerprints.

|  [AN0202](https://attack.mitre.org/detectionstrategies/DET0074#AN0202)   |

Session cookie reuse on unmanaged browsers, devices, or client types deviating from user baseline (e.g., switching from Chrome to curl).

|  [AN0203](https://attack.mitre.org/detectionstrategies/DET0074#AN0203)   |

Web session tokens reused in native Office apps (e.g., Outlook, Teams) without associated token refresh or login behavior on the endpoint.

## References

-    [Rehberger, J. (2018, December). Pivot to the Cloud using Pass the Cookie. Retrieved April 5, 2019.](https://wunderwuzzi23.github.io/blog/passthecookie.html)
-    [Chen, Y., Hu, W., Xu, Z., et. al. (2019, January 31). Mac Malware Steals Cryptocurrency Exchanges’ Cookies. Retrieved October 14, 2019.](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)
-    [Cash, D. et al. (2020, December 14). Dark Halo Leverages SolarWinds Compromise to Breach Organizations. Retrieved December 29, 2020.](https://www.volexity.com/blog/2020/12/14/dark-halo-leverages-solarwinds-compromise-to-breach-organizations/)

-    [CrowdStrike. (2022, January 27). Early Bird Catches the Wormhole: Observations from the StellarParticle Campaign. Retrieved February 7, 2022.](https://www.crowdstrike.com/blog/observations-from-the-stellarparticle-campaign/)
-    [CISA, et al. (2023, December 7). Russian FSB Cyber Actor Star Blizzard Continues Worldwide Spear-phishing Campaigns. Retrieved June 13, 2024.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-341a)

×
