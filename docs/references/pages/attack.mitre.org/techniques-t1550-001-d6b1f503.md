---
url: "https://attack.mitre.org/techniques/T1550/001/"
title: "Use Alternate Authentication Material: Application Access Token, Sub-technique T1550.001 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:50:34Z"
via: "live"
sha256: "3d2bd2b4eb1e722688b5d7fe7c120e12a9217e772b391d0276a310a411db76aa"
wayback: "https://web.archive.org/web/20261023090829/https://attack.mitre.org/techniques/T1550/001/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Use Alternate Authentication Material: Application Access Token, Sub-technique T1550.001 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1550/001/>
> Snapshot: <https://web.archive.org/web/20261023090829/https://attack.mitre.org/techniques/T1550/001/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Use Alternate Authentication Material](https://attack.mitre.org/techniques/T1550)
- Application Access Token

#  Use Alternate Authentication Material: Application Access Token

##### Other sub-techniques of Use Alternate Authentication Material (4)

| ID  | Name

|  T1550.001   |  Application Access Token

|  [T1550.002](https://attack.mitre.org/techniques/T1550/002/)   |  [Pass the Hash](https://attack.mitre.org/techniques/T1550/002/)

|  [T1550.003](https://attack.mitre.org/techniques/T1550/003/)   |  [Pass the Ticket](https://attack.mitre.org/techniques/T1550/003/)

|  [T1550.004](https://attack.mitre.org/techniques/T1550/004/)   |  [Web Session Cookie](https://attack.mitre.org/techniques/T1550/004/)

Adversaries may use stolen application access tokens to bypass the typical authentication process and access restricted accounts, information, or services on remote systems. These tokens are typically stolen from users or services and used in lieu of login credentials.

Application access tokens are used to make authorized API requests on behalf of a user or service and are commonly used to access resources in cloud, container-based applications, and software-as-a-service (SaaS).[[1]](https://auth0.com/blog/why-should-use-accesstokens-to-secure-an-api/)

OAuth is one commonly implemented framework that issues tokens to users for access to systems. These frameworks are used collaboratively to verify the user and determine what actions the user is allowed to perform. Once identity is established, the token allows actions to be authorized, without passing the actual credentials of the user. Therefore, compromise of the token can grant the adversary access to resources of other sites through a malicious application.[[2]](https://developer.okta.com/blog/2018/06/20/what-happens-if-your-jwt-is-stolen)

For example, with a cloud-based email service, once an OAuth access token is granted to a malicious application, it can potentially gain long-term access to features of the user account if a "refresh" token enabling background access is awarded.[[3]](https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens) With an OAuth access token an adversary can use the user-granted REST API to perform functions such as email searching and contact enumeration.[[4]](https://staaldraad.github.io/2017/08/02/o356-phishing-with-oauth/)

Compromised access tokens may be used as an initial step in compromising other services. For example, if a token grants access to a victim’s primary email, the adversary may be able to extend access to all other services which the target subscribes by triggering forgotten password routines. In AWS and GCP environments, adversaries can trigger a request for a short-lived access token with the privileges of another user account.[[5]](https://cloud.google.com/iam/docs/creating-short-lived-service-account-credentials)[[6]](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_request.html) The adversary can then use this token to request data or perform actions the original account could not. If permissions for this feature are misconfigured – for example, by allowing all users to request a token for a particular account - an adversary may be able to gain initial access to a Cloud Account or escalate their privileges.[[7]](https://rhinosecuritylabs.com/aws/assume-worst-aws-assume-role-enumeration)

Direct API access through a token negates the effectiveness of a second authentication factor and may be immune to intuitive countermeasures like changing passwords. For example, in AWS environments, an adversary who compromises a user’s AWS API credentials may be able to use the sts:GetFederationToken API call to create a federated user session, which will have the same permissions as the original user but may persist even if the original user credentials are deactivated.[[8]](https://www.crowdstrike.com/blog/how-adversaries-persist-with-aws-user-federation/) Additionally, access abuse over an API channel can be difficult to detect even from the service provider end, as the access can still align well with a legitimate workflow.

ID: T1550.001

Sub-technique of:  [T1550](https://attack.mitre.org/techniques/T1550)

ⓘ

Tactic: [Lateral Movement](https://attack.mitre.org/tactics/TA0008)

ⓘ

Platforms: Containers, IaaS, Identity Provider, Office Suite, SaaS

Contributors: Blake Strom, Microsoft Threat Intelligence; Dylan Silva, AWS Security; Ian Davila, Tidal Cyber; Jack Burns, HubSpot; Jeff Sakowicz, Microsoft Identity Developer Platform Services (IDPM Services); Mark Wee; Pawel Partyka, Microsoft Threat Intelligence; Saisha Agrawal, Microsoft Threat Intelligent Center (MSTIC); Shailesh Tiwary (Indian Army)

Version: 2.0

Created: 30 January 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1550/001/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1550/001/)

## Procedure Examples

| ID  | Name  | Description

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has used several malicious applications that abused OAuth access tokens to gain access to target email accounts, including Gmail and Yahoo Mail.[[9]](https://blog.trendmicro.com/trendlabs-security-intelligence/pawn-storm-abuses-open-authentication-advanced-social-engineering-attacks)

|  [S9042](https://attack.mitre.org/software/S9042)   |  [CanisterWorm](https://attack.mitre.org/software/S9042)   |

[CanisterWorm](https://attack.mitre.org/software/S9042) has leveraged stolen npm tokens to automate compromise by enumerating all publishable packages in a namespace, bumping versions, and publishing itself across the entire scope.[[10]](https://www.aikido.dev/blog/telnyx-pypi-compromised-teampcp-canisterworm)[[11]](https://www.aikido.dev/blog/teampcp-deploys-worm-npm-trivy-compromise)

|  [S1023](https://attack.mitre.org/software/S1023)   |  [CreepyDrive](https://attack.mitre.org/software/S1023)   |

[CreepyDrive](https://attack.mitre.org/software/S1023) can use legitimate OAuth refresh tokens to authenticate with OneDrive.[[12]](https://www.microsoft.com/security/blog/2022/06/02/exposing-polonium-activity-and-infrastructure-targeting-israeli-organizations/)

|  [G0125](https://attack.mitre.org/groups/G0125)   |  [HAFNIUM](https://attack.mitre.org/groups/G0125)   |

[HAFNIUM](https://attack.mitre.org/groups/G0125) has abused service principals with administrative permissions for data exfiltration.[[13]](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)

|  [S9044](https://attack.mitre.org/software/S9044)   |  [Kali365](https://attack.mitre.org/software/S9044)   |

[Kali365](https://attack.mitre.org/software/S9044) has utilized an Exchange Admin module that captured admin tokens to create rogue mailbox connectors and change mail-flow rules.[[14]](https://www.huntress.com/blog/kali365-device-code-phishing-kit)

|  [S9043](https://attack.mitre.org/software/S9043)   |  [Mini Shai-Hulud](https://attack.mitre.org/software/S9043)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has the ability to authenticate using stolen application access tokens.[[15]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[16]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[17]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  [S0683](https://attack.mitre.org/software/S0683)   |  [Peirates](https://attack.mitre.org/software/S0683)   |

[Peirates](https://attack.mitre.org/software/S0683) can use stolen service account tokens to perform its operations. It also enables adversaries to switch between valid service accounts.[[18]](https://github.com/inguardians/peirates)

|  [S9008](https://attack.mitre.org/software/S9008)   |  [Shai-Hulud](https://attack.mitre.org/software/S9008)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has leveraged captured valid NPM tokens to enumerate and update packages on compromised accounts.[[19]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[20]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[21]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also utilized stolen GitHub access tokens to access compromised accounts.[[20]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[21]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)

|  [G1057](https://attack.mitre.org/groups/G1057)   |  [ShinyHunters](https://attack.mitre.org/groups/G1057)   |

[ShinyHunters](https://attack.mitre.org/groups/G1057) has used stolen OAuth keys to access cloud infrastructure and to bypass two-factor authentication.[[22]](https://www.intel471.com/blog/shinyhunters-data-breach-mitre-attack)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) used compromised service principals to make changes to the Office 365 environment.[[23]](https://www.crowdstrike.com/blog/observations-from-the-stellarparticle-campaign/)

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has used stolen access tokens to inject malicious code into CI/CD workflows and to exfiltrate sensitive data from cloud, developer, and container environments.[[24]](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)[[10]](https://www.aikido.dev/blog/telnyx-pypi-compromised-teampcp-canisterworm)[[25]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1036](https://attack.mitre.org/mitigations/M1036)   |  [Account Use Policies](https://attack.mitre.org/mitigations/M1036)   |

Where possible, consider restricting the use of access tokens outside of expected contexts. For example, in AWS environments, consider using data perimeters to prevent credential use outside of an expected network.[[26]](https://aws.amazon.com/identity/data-perimeters-on-aws/)

|  [M1013](https://attack.mitre.org/mitigations/M1013)   |  [Application Developer Guidance](https://attack.mitre.org/mitigations/M1013)   |

Consider implementing token binding strategies, such as Azure AD token protection or OAuth Proof of Possession, that cryptographically bind a token to a secret. This may prevent the token from being used without knowledge of the secret or possession of the device the token is tied to.[[27]](https://learn.microsoft.com/en-us/entra/identity/conditional-access/concept-token-protection)[[28]](https://www.okta.com/blog/2023/06/a-leap-forward-in-token-security-okta-adds-support-for-dpop/)

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Administrators should audit all cloud and container accounts to ensure that they are necessary and that the permissions granted to them are appropriate. Where possible, the ability to request temporary account tokens on behalf of another accounts should be disabled. Additionally, administrators can leverage audit tools to monitor actions that can be conducted as a result of OAuth 2.0 access. For instance, audit reports enable admins to identify privilege escalation actions such as role creations or policy modifications, which could be actions performed after initial access.

|  [M1041](https://attack.mitre.org/mitigations/M1041)   |  [Encrypt Sensitive Information](https://attack.mitre.org/mitigations/M1041)   |

File encryption should be enforced across email communications containing sensitive information that may be obtained through access to email services.

|  [M1021](https://attack.mitre.org/mitigations/M1021)   |  [Restrict Web-Based Content](https://attack.mitre.org/mitigations/M1021)   |

Update corporate policies to restrict what types of third-party applications may be added to any online service or tool that is linked to the company's information, accounts or network (e.g., Google, Microsoft, Dropbox, Basecamp, GitHub). However, rather than providing high-level guidance on this, be extremely specific—include a list of per-approved applications and deny all others not on the list. Administrators may also block end-user consent through administrative portals, such as the Azure Portal, disabling users from authorizing third-party apps through OAuth and forcing administrative consent.[[29]](https://docs.microsoft.com/en-us/azure/security/fundamentals/steps-secure-identity#block-end-user-consent)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0185](https://attack.mitre.org/detectionstrategies/DET0185)   |  [Behavioral Detection Strategy for Use Alternate Authentication Material: Application Access Token (T1550.001)](https://attack.mitre.org/detectionstrategies/DET0185)   |  [AN0526](https://attack.mitre.org/detectionstrategies/DET0185#AN0526)   |

Use of AWS STS or GCP IAM APIs to request temporary tokens or federation sessions inconsistent with normal account activity, including from unexpected principals or regions.

|  [AN0527](https://attack.mitre.org/detectionstrategies/DET0185#AN0527)   |

OAuth or SAML access tokens reused across multiple sessions or clients without corresponding MFA or login activity.

|  [AN0528](https://attack.mitre.org/detectionstrategies/DET0185#AN0528)   |

Application access tokens used to call APIs (e.g., Google Workspace, Salesforce) without interactive logins, often with unusual scopes or elevated permissions.

|  [AN0529](https://attack.mitre.org/detectionstrategies/DET0185#AN0529)   |

OAuth token usage for Exchange Online or SharePoint API access without preceding login or from unauthorized clients.

|  [AN0530](https://attack.mitre.org/detectionstrategies/DET0185#AN0530)   |

Compromised service account tokens mounted inside containers and reused for external API calls or lateral movement across services.

## References

-    [Auth0. (n.d.). Why You Should Always Use Access Tokens to Secure APIs. Retrieved September 12, 2019.](https://auth0.com/blog/why-should-use-accesstokens-to-secure-an-api/)
-    [okta. (n.d.). What Happens If Your JWT Is Stolen?. Retrieved September 12, 2019.](https://developer.okta.com/blog/2018/06/20/what-happens-if-your-jwt-is-stolen)
-    [Cai, S., Flores, J., de Guzman, C., et. al.. (2019, August 27). Microsoft identity platform access tokens. Retrieved October 4, 2019.](https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens)
-    [Stalmans, E.. (2017, August 2). Phishing with OAuth and o365/Azure. Retrieved October 4, 2019.](https://staaldraad.github.io/2017/08/02/o356-phishing-with-oauth/)
-    [Google Cloud. (2022, March 31). Creating short-lived service account credentials. Retrieved April 1, 2022.](https://cloud.google.com/iam/docs/creating-short-lived-service-account-credentials)
-    [AWS. (n.d.). Requesting temporary security credentials. Retrieved April 1, 2022.](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_request.html)
-    [Spencer Gietzen. (2018, August 8). Assume the Worst: Enumerating AWS Roles through ‘AssumeRole’. Retrieved April 1, 2022.](https://rhinosecuritylabs.com/aws/assume-worst-aws-assume-role-enumeration)
-    [Vaishnav Murthy and Joel Eng. (2023, January 30). How Adversaries Can Persist with AWS User Federation. Retrieved March 10, 2023.](https://www.crowdstrike.com/blog/how-adversaries-persist-with-aws-user-federation/)
-    [Hacquebord, F.. (2017, April 25). Pawn Storm Abuses Open Authentication in Advanced Social Engineering Attacks. Retrieved October 4, 2019.](https://blog.trendmicro.com/trendlabs-security-intelligence/pawn-storm-abuses-open-authentication-advanced-social-engineering-attacks)
-    [Eriksen, C. (2026, March 27). Popular telnyx package compromised on PyPI by TeamPCP. Retrieved July 16, 2026.](https://www.aikido.dev/blog/telnyx-pypi-compromised-teampcp-canisterworm)
-    [Eriksen, C. (2026, March 20). TeamPCP deploys CanisterWorm on NPM following Trivy compromise. Retrieved July 27, 2026.](https://www.aikido.dev/blog/teampcp-deploys-worm-npm-trivy-compromise)
-    [Microsoft. (2022, June 2). Exposing POLONIUM activity and infrastructure targeting Israeli organizations. Retrieved July 1, 2022.](https://www.microsoft.com/security/blog/2022/06/02/exposing-polonium-activity-and-infrastructure-targeting-israeli-organizations/)
-    [Microsoft Threat Intelligence . (2025, March 5). Silk Typhoon targeting IT supply chain. Retrieved March 20, 2025.](https://www.microsoft.com/en-us/security/blog/2025/03/05/silk-typhoon-targeting-it-supply-chain/)
-    [Tanner Flip. (2026, June 11). Hook, Line, and Token: Anatomy of the Kali365 / Octopi365 Phishing-as-a-Service Kit. Retrieved July 30, 2026.](https://www.huntress.com/blog/kali365-device-code-phishing-kit)
-    [McCarthy, R., Cohen, A., and Read, B. (2026, May 12). Mini Shai-Hulud Strikes Again: TanStack + more npm Packages Compromised. Retrieved July 16, 2026.](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)

-    [Webb, M. (2026, May 20). TeamPCP Wave Four: GitHub Breach via Poisoned VS Code Extension, durabletask PyPI Worm, and ~4,000 Internal Repositories Exfiltrated. Retrieved July 16, 2026.](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)
-    [Flashpoint. (2026, May 28). The Mini Shai-Hulud Worm and the New Era of CI/CD Exploitation. Retrieved July 16, 2026.](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)
-    [InGuardians. (2022, January 5). Peirates GitHub. Retrieved February 8, 2022.](https://github.com/inguardians/peirates)
-    [Charlie Eriksen. (2025, September 16). S1ngularity/nx attackers strike again. Retrieved April 9, 2026.](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)
-    [Socket Research Team. (2025, November 24). Shai Hulud Strikes Again (v2). Retrieved April 9, 2026.](https://socket.dev/blog/shai-hulud-strikes-again-v2)
-    [Socket Research Team. (2025, September 15). Popular Tinycolor npm Package Compromised in Supply Chain Attack Affecting 40+ Packages. Retrieved April 9, 2026.](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)
-    [Intel 471. (2021, August 23). Here’s how to guard your enterprise against ShinyHunters. Retrieved July 29, 2026.](https://www.intel471.com/blog/shinyhunters-data-breach-mitre-attack)
-    [CrowdStrike. (2022, January 27). Early Bird Catches the Wormhole: Observations from the StellarParticle Campaign. Retrieved February 7, 2022.](https://www.crowdstrike.com/blog/observations-from-the-stellarparticle-campaign/)
-    [McCarthy, R. (2026, March 20). Trivy Compromised: Everything You Need to Know about the Latest Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)
-    [Unit 42. (2026, March 31). Weaponizing the Protectors: TeamPCP’s Multi-Stage Supply Chain Attack on Security Infrastructure. Retrieved July 1, 2026.](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)
-    [AWS. (n.d.). Data perimeters on AWS. Retrieved October 16, 2024.](https://aws.amazon.com/identity/data-perimeters-on-aws/)
-    [Microsoft. (2023, October 23). Conditional Access: Token protection (preview). Retrieved January 2, 2024.](https://learn.microsoft.com/en-us/entra/identity/conditional-access/concept-token-protection)
-    [Venkat Viswanathan. (2023, June 13). A leap forward in token security: Okta adds support for DPoP. Retrieved January 2, 2024.](https://www.okta.com/blog/2023/06/a-leap-forward-in-token-security-okta-adds-support-for-dpop/)
-    [Baldwin, M., Flores, J., Kess, B.. (2018, June 17). Five steps to securing your identity infrastructure. Retrieved October 4, 2019.](https://docs.microsoft.com/en-us/azure/security/fundamentals/steps-secure-identity#block-end-user-consent)

×
