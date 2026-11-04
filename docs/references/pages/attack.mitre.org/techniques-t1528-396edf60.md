---
url: "https://attack.mitre.org/techniques/T1528/"
title: "Steal Application Access Token, Technique T1528 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:45:26Z"
via: "live"
sha256: "cb574466edf93d32d5dfad8d7d763028e8c54c9e44a9a91ee2cb89133b0aef80"
wayback: "https://web.archive.org/web/20261023090825/https://attack.mitre.org/techniques/T1528/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Steal Application Access Token, Technique T1528 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1528/>
> Snapshot: <https://web.archive.org/web/20261023090825/https://attack.mitre.org/techniques/T1528/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Steal Application Access Token

#  Steal Application Access Token

Adversaries can steal application access tokens as a means of acquiring credentials to access remote systems and resources.

Application access tokens are used to make authorized API requests on behalf of a user or service and are commonly used as a way to access resources in cloud and container-based applications and software-as-a-service (SaaS).[[1]](https://auth0.com/blog/why-should-use-accesstokens-to-secure-an-api/) Adversaries who steal account API tokens in cloud and containerized environments may be able to access data and perform actions with the permissions of these accounts, which can lead to privilege escalation and further compromise of the environment.

For example, in Kubernetes environments, processes running inside a container may communicate with the Kubernetes API server using service account tokens. If a container is compromised, an adversary may be able to steal the container’s token and thereby gain access to Kubernetes API commands.[[2]](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/)

Similarly, instances within continuous-development / continuous-integration (CI/CD) pipelines will often use API tokens to authenticate to other services for testing and deployment.[[3]](https://web.archive.org/web/20220316130828/https://www.cidersecurity.io/top-10-cicd-security-risks/) If these pipelines are compromised, adversaries may be able to steal these tokens and leverage their privileges.

In Azure, an adversary who compromises a resource with an attached Managed Identity, such as an Azure VM, can request short-lived tokens through the Azure Instance Metadata Service (IMDS). These tokens can then facilitate unauthorized actions or further access to other Azure services, bypassing typical credential-based authentication.[[4]](https://learn.microsoft.com/en-us/entra/identity/managed-identities-azure-resources/how-to-use-vm-token)[[5]](https://posts.specterops.io/managed-identity-attack-paths-part-1-automation-accounts-82667d17187a?gi=6a9daedade1c)

Token theft can also occur through social engineering, in which case user action may be required to grant access. OAuth is one commonly implemented framework that issues tokens to users for access to systems. An application desiring access to cloud-based services or protected APIs can gain entry using OAuth 2.0 through a variety of authorization protocols. An example commonly-used sequence is Microsoft's Authorization Code Grant flow.[[6]](https://docs.microsoft.com/en-us/azure/active-directory/develop/active-directory-v2-protocols)[[7]](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow) An OAuth access token enables a third-party application to interact with resources containing user data in the ways requested by the application without obtaining user credentials.

Adversaries can leverage OAuth authorization by constructing a malicious application designed to be granted access to resources with the target user's OAuth token.[[8]](https://www.amnesty.org/en/latest/research/2019/08/evolving-phishing-attacks-targeting-journalists-and-human-rights-defenders-from-the-middle-east-and-north-africa/)[[9]](https://blog.trendmicro.com/trendlabs-security-intelligence/pawn-storm-abuses-open-authentication-advanced-social-engineering-attacks) The adversary will need to complete registration of their application with the authorization server, for example Microsoft Identity Platform using Azure Portal, the Visual Studio IDE, the command-line interface, PowerShell, or REST API calls.[[10]](https://docs.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app) Then, they can send a [Spearphishing Link](https://attack.mitre.org/techniques/T1566/002) to the target user to entice them to grant access to the application. Once the OAuth access token is granted, the application can gain potentially long-term access to features of the user account through [Application Access Token](https://attack.mitre.org/techniques/T1550/001).[[11]](https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens)

Application access tokens may function within a limited lifetime, limiting how long an adversary can utilize the stolen token. However, in some cases, adversaries can also steal application refresh tokens[[12]](https://auth0.com/learn/refresh-tokens), allowing them to obtain new access tokens without prompting the user.

ID: T1528

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Containers, IaaS, Identity Provider, Office Suite, SaaS

Contributors: Alon Klayman, Hunters Security; Arun Seelagan, CISA; Eliraz Levi, Hunters Security; Jack Burns, HubSpot; Jeff Sakowicz, Microsoft Identity Developer Platform Services (IDPM Services); Mark Wee; Ram Pliskin, Microsoft Azure Security Center; Saisha Agrawal, Microsoft Threat Intelligent Center (MSTIC); Shailesh Tiwary (Indian Army); Suzy Schapperle - Microsoft Azure Red Team

Version: 1.5

Created: 04 September 2019

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1528/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1528/)

## Procedure Examples

| ID  | Name  | Description

|  [S0677](https://attack.mitre.org/software/S0677)   |  [AADInternals](https://attack.mitre.org/software/S0677)   |

[AADInternals](https://attack.mitre.org/software/S0677) can steal users’ access tokens via phishing emails containing malicious links.[[13]](https://o365blog.com/aadinternals)

|  [G0007](https://attack.mitre.org/groups/G0007)   |  [APT28](https://attack.mitre.org/groups/G0007)   |

[APT28](https://attack.mitre.org/groups/G0007) has used several malicious applications to steal user OAuth access tokens including applications masquerading as "Google Defender" "Google Email Protection," and "Google Scanner" for Gmail users. They also targeted Yahoo users with applications masquerading as "Delivery Service" and "McAfee Email Protection".[[9]](https://blog.trendmicro.com/trendlabs-security-intelligence/pawn-storm-abuses-open-authentication-advanced-social-engineering-attacks)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) uses stolen tokens to access victim accounts, without needing a password.[[14]](https://www.ic3.gov/Media/News/2024/240226.pdf)

|  [S9045](https://attack.mitre.org/software/S9045)   |  [BACKUPOSINT v9.0 APEX PREDATOR](https://attack.mitre.org/software/S9045)   |

[BACKUPOSINT v9.0 APEX PREDATOR](https://attack.mitre.org/software/S9045) has the ability to steal application tokens for CI/CD services to include Bamboo, Github, and Jenkins.[[15]](https://5943619.hs-sites.com/hubfs/116-Whitepapers/dragos-2026-ai-mexico-water-attack-intel-brief.pdf)

|  [S9042](https://attack.mitre.org/software/S9042)   |  [CanisterWorm](https://attack.mitre.org/software/S9042)   |

[CanisterWorm](https://attack.mitre.org/software/S9042) has gathered cloud access tokens.[[16]](https://www.ic3.gov/CSA/2026/260702.pdf)

|  [S9044](https://attack.mitre.org/software/S9044)   |  [Kali365](https://attack.mitre.org/software/S9044)   |

[Kali365](https://attack.mitre.org/software/S9044) has obtained OAuth access and refresh tokens by deceiving victims into completing actor-initiated device authorization requests, and intercepting authentication sessions with legitimate identity providers through adversary-in-the-middle reverse proxy infrastructure. [[17]](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)[[18]](https://arcticwolf.com/resources/blog/kali365-expands-into-aws-microsoft-okta-xerox-max-messenger/)[[19]](https://www.ic3.gov/PSA/2026/PSA260521)[[20]](https://spycloud.com/blog/kali365-anatomy-of-a-microsoft365-phishing-as-a-service-kit/)

|  [C0049](https://attack.mitre.org/campaigns/C0049)   |  [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049)   |

[Leviathan](https://attack.mitre.org/groups/G0065) abused access to compromised appliances to collect JSON Web Tokens (JWTs), used for creating virtual desktop sessions, during [Leviathan Australian Intrusions](https://attack.mitre.org/campaigns/C0049).[[21]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)

|  [S9043](https://attack.mitre.org/software/S9043)   |  [Mini Shai-Hulud](https://attack.mitre.org/software/S9043)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has stolen application access tokens and other tokens to include those associated with CI/CD.[[22]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[23]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[24]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[25]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[26]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  [S0683](https://attack.mitre.org/software/S0683)   |  [Peirates](https://attack.mitre.org/software/S0683)   |

[Peirates](https://attack.mitre.org/software/S0683) gathers Kubernetes service account tokens using a variety of techniques.[[27]](https://github.com/inguardians/peirates)

|  [S9008](https://attack.mitre.org/software/S9008)   |  [Shai-Hulud](https://attack.mitre.org/software/S9008)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has stolen access tokens and API tokens from with CI/CD pipeline solutions and repositories.[[28]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[29]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[30]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[31]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  [G1057](https://attack.mitre.org/groups/G1057)   |  [ShinyHunters](https://attack.mitre.org/groups/G1057)   |

[ShinyHunters](https://attack.mitre.org/groups/G1057) has stolen valid OAuth credentials from DevOps personnel or a company GitHub repository.[[32]](https://www.intel471.com/blog/shinyhunters-data-breach-mitre-attack) Additionally, [ShinyHunters](https://attack.mitre.org/groups/G1057) has stolen application access tokens to access cloud services and to bypass authentication mechanisms.[[33]](https://socradar.io/dark-web-profile-shinyhunters/)

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has used malware to steal access tokens from targeted cloud and developer environments.[[34]](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)[[35]](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)[[36]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)[[23]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[16]](https://www.ic3.gov/CSA/2026/260702.pdf)

|  [S9041](https://attack.mitre.org/software/S9041)   |  [TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041)   |

[TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041) can read runner.worker process memory to extract plaintext tokens.[[36]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)[[37]](https://www.sysdig.com/blog/teampcp-expands-supply-chain-compromise-spreads-from-trivy-to-checkmarx-github-actions)[[38]](https://www.wiz.io/blog/teampcp-attack-kics-github-action)[[36]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)[[16]](https://www.ic3.gov/CSA/2026/260702.pdf)[[39]](https://cloud.google.com/blog/topics/threat-intelligence/ai-vulnerability-exploitation-initial-access)

|  [S9009](https://attack.mitre.org/software/S9009)   |  [TruffleHog](https://attack.mitre.org/software/S9009)   |

[TruffleHog](https://attack.mitre.org/software/S9009) has gathered access tokens and API tokens from CI/CD pipeline solutions and repositories.[[40]](https://www.blackhillsinfosec.com/rooting-for-secrets-with-trufflehog/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Administrators should audit all cloud and container accounts to ensure that they are necessary and that the permissions granted to them are appropriate. Additionally, administrators should perform an audit of all OAuth applications and the permissions they have been granted to access organizational data. This should be done extensively on all applications in order to establish a baseline, followed up on with periodic audits of new or updated applications. Suspicious applications should be investigated and removed.

|  [M1021](https://attack.mitre.org/mitigations/M1021)   |  [Restrict Web-Based Content](https://attack.mitre.org/mitigations/M1021)   |

Administrators can block end-user consent to OAuth applications, disabling users from authorizing third-party apps through OAuth 2.0 and forcing administrative consent for all requests. They can also block end-user registration of applications by their users, to reduce risk. A Cloud Access Security Broker can also be used to ban applications.

Azure offers a couple of enterprise policy settings in the Azure Management Portal that may help:

"Users -> User settings -> App registrations: Users can register applications" can be set to "no" to prevent users from registering new applications.
"Enterprise applications -> User settings -> Enterprise applications: Users can consent to apps accessing company data on their behalf" can be set to "no" to prevent users from consenting to allow third-party multi-tenant applications

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Enforce role-based access control to limit accounts to the least privileges they require. A Cloud Access Security Broker (CASB) can be used to set usage policies and manage user permissions on cloud applications to prevent access to application access tokens. In Kubernetes applications, set "automountServiceAccountToken: false" in the YAML specification of pods that do not require access to service account tokens.[[41]](https://media.defense.gov/2022/Aug/29/2003066362/-1/-1/0/CTR_KUBERNETES_HARDENING_GUIDANCE_1.2_20220829.PDF)

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Users need to be trained to not authorize third-party applications they don’t recognize. The user should pay particular attention to the redirect URL: if the URL is a misspelled or convoluted sequence of words related to an expected service or SaaS application, the website is likely trying to spoof a legitimate service. Users should also be cautious about the permissions they are granting to apps. For example, offline access and access to read emails should excite higher suspicions because adversaries can utilize SaaS APIs to discover credentials and other sensitive communications.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0515](https://attack.mitre.org/detectionstrategies/DET0515)   |  [Detection Strategy for T1528 - Steal Application Access Token](https://attack.mitre.org/detectionstrategies/DET0515)   |  [AN1423](https://attack.mitre.org/detectionstrategies/DET0515#AN1423)   |

Access and retrieval of container service account tokens followed by unauthorized API requests using those tokens to interact with the Kubernetes API server or internal services.

|  [AN1424](https://attack.mitre.org/detectionstrategies/DET0515#AN1424)   |

Token retrieval from instance metadata endpoints such as AWS IMDS or Azure IMDS, followed by API usage using the obtained token from non-standard applications.

|  [AN1425](https://attack.mitre.org/detectionstrategies/DET0515#AN1425)   |

Unusual OAuth app registration followed by user-granted OAuth tokens and subsequent high-privilege resource access via those tokens.

|  [AN1426](https://attack.mitre.org/detectionstrategies/DET0515#AN1426)   |

Use of OAuth tokens by third-party apps to access user mail, calendar, or SharePoint resources where the token was granted recently or via spearphishing.

|  [AN1427](https://attack.mitre.org/detectionstrategies/DET0515#AN1427)   |

Programmatic access to user content via stolen access tokens in platforms like Slack, GitHub, Google Workspace — especially from new IPs, apps, or excessive resource access.

## References

-    [Auth0. (n.d.). Why You Should Always Use Access Tokens to Secure APIs. Retrieved September 12, 2019.](https://auth0.com/blog/why-should-use-accesstokens-to-secure-an-api/)
-    [Kubernetes. (2022, February 26). Configure Service Accounts for Pods. Retrieved April 1, 2022.](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/)
-    [Daniel Krivelevich and Omer Gil. (n.d.). Top 10 CI/CD Security Risks. Retrieved November 17, 2024.](https://web.archive.org/web/20220316130828/https://www.cidersecurity.io/top-10-cicd-security-risks/)
-    [Microsoft Entra. (2025, February 27). How to use managed identities for Azure resources on an Azure VM to acquire an access token. Retrieved March 18, 2025.](https://learn.microsoft.com/en-us/entra/identity/managed-identities-azure-resources/how-to-use-vm-token)
-    [Andy Robbins. (2022, June 6). Managed Identity Attack Paths, Part 1: Automation Accounts. Retrieved March 18, 2025.](https://posts.specterops.io/managed-identity-attack-paths-part-1-automation-accounts-82667d17187a?gi=6a9daedade1c)
-    [Microsoft. (n.d.). Retrieved September 12, 2019.](https://docs.microsoft.com/en-us/azure/active-directory/develop/active-directory-v2-protocols)
-    [Microsoft. (n.d.). Microsoft identity platform and OAuth 2.0 authorization code flow. Retrieved September 12, 2019.](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
-    [Amnesty International. (2019, August 16). Evolving Phishing Attacks Targeting Journalists and Human Rights Defenders from the Middle-East and North Africa. Retrieved October 8, 2019.](https://www.amnesty.org/en/latest/research/2019/08/evolving-phishing-attacks-targeting-journalists-and-human-rights-defenders-from-the-middle-east-and-north-africa/)
-    [Hacquebord, F.. (2017, April 25). Pawn Storm Abuses Open Authentication in Advanced Social Engineering Attacks. Retrieved October 4, 2019.](https://blog.trendmicro.com/trendlabs-security-intelligence/pawn-storm-abuses-open-authentication-advanced-social-engineering-attacks)
-    [Microsoft. (2019, May 8). Quickstart: Register an application with the Microsoft identity platform. Retrieved September 12, 2019.](https://docs.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app)
-    [Microsoft. (2019, August 29). Microsoft identity platform access tokens. Retrieved September 12, 2019.](https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens)
-    [Auth0 Inc.. (n.d.). Understanding Refresh Tokens. Retrieved November 17, 2024.](https://auth0.com/learn/refresh-tokens)
-    [Dr. Nestori Syynimaa. (2018, October 25). AADInternals. Retrieved February 18, 2022.](https://o365blog.com/aadinternals)
-    [UK National Cyber Security Center et al. (2024, February). SVR cyber actors adapt tactics for initial cloud access. Retrieved March 1, 2024.](https://www.ic3.gov/Media/News/2024/240226.pdf)
-    [Jay Deen. (2026, May). AI-Assisted Compromise of Mexican Water Utility with OT Implications. Retrieved September 2, 2026.](https://5943619.hs-sites.com/hubfs/116-Whitepapers/dragos-2026-ai-mexico-water-attack-intel-brief.pdf)
-    [FBI. (2026, July 2). Cyber Criminal Group TeamPCP. Retrieved July 7, 2026.](https://www.ic3.gov/CSA/2026/260702.pdf)
-    [Artic Wolf Labs. (2026, April 24). Token Bingo: Don’t Let Your Code be the Winner. Retrieved July 30, 2026.](https://arcticwolf.com/resources/blog/token-bingo-dont-let-your-code-be-the-winner/)
-    [Artic Wolf Labs. (2026, June 2). Retrieved July 30, 2026.](https://arcticwolf.com/resources/blog/kali365-expands-into-aws-microsoft-okta-xerox-max-messenger/)
-    [Federal Bureau of Investigation. (2026, May 21). Alert Number: I-052126-PSA: Kali365 Phishing-as-a-Service Kit Hijacks Microsoft 365 Access Tokens. Retrieved July 30, 2026.](https://www.ic3.gov/PSA/2026/PSA260521)
-    [Trevor Hilligoss. (2026, June 11). Kali365: Anatomy of a Microsoft 365 Phishing-as-a-Service Kit – From Telegram Hype to FBI Takedown Theater. Retrieved July 30, 2026.](https://spycloud.com/blog/kali365-anatomy-of-a-microsoft365-phishing-as-a-service-kit/)
-    [CISA et al. (2024, July 8). People’s Republic of China (PRC) Ministry of State Security APT40 Tradecraft in Action. Retrieved February 3, 2025.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa24-190a)

-    [McCarthy, R., Cohen, A., and Read, B. (2026, May 12). Mini Shai-Hulud Strikes Again: TanStack + more npm Packages Compromised. Retrieved July 16, 2026.](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)
-    [Santos, J. and Navato, J.R. (2026, May 13). Analyzing TeamPCP’s Supply Chain Attacks: Checkmarx KICS and elementary-data in CI/CD Credential Theft. Retrieved July 16, 2026.](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)
-    [Hunt.io. (2026, May 14). How TeamPCP's Python Toolkit Survives a C2 Takedown: FIRESCALE, GitHub, and the Victim's Own Account. Retrieved July 16, 2026.](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)
-    [Webb, M. (2026, May 20). TeamPCP Wave Four: GitHub Breach via Poisoned VS Code Extension, durabletask PyPI Worm, and ~4,000 Internal Repositories Exfiltrated. Retrieved July 16, 2026.](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)
-    [Flashpoint. (2026, May 28). The Mini Shai-Hulud Worm and the New Era of CI/CD Exploitation. Retrieved July 16, 2026.](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)
-    [InGuardians. (2022, January 5). Peirates GitHub. Retrieved February 8, 2022.](https://github.com/inguardians/peirates)
-    [Gianpietro Cutolo. (2025, November 26). Shai-Hulud 2.0: Aggressive, Automated, and Fast Spreading. Retrieved April 9, 2026.](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)
-    [Justin Moore. (2025, November 25). "Shai-Hulud" Worm Compromises npm Ecosystem in Supply Chain Attack (Updated November 26). Retrieved April 9, 2026.](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)
-    [Merav Bar, Rami McCarthy, Barak Sharoni. (2025, September 16). Shai-Hulud: Ongoing Package Supply Chain Worm Delivering Data-Stealing Malware. Retrieved April 9, 2026.](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)
-    [Socket Research Team. (2025, November 24). Shai Hulud Strikes Again (v2). Retrieved April 9, 2026.](https://socket.dev/blog/shai-hulud-strikes-again-v2)
-    [Intel 471. (2021, August 23). Here’s how to guard your enterprise against ShinyHunters. Retrieved July 29, 2026.](https://www.intel471.com/blog/shinyhunters-data-breach-mitre-attack)
-    [SOCRadar. (2024, March 18). Dark Web Profile: ShinyHunters. Retrieved May 18, 2026.](https://socradar.io/dark-web-profile-shinyhunters/)
-    [McCarthy, R. (2026, March 20). Trivy Compromised: Everything You Need to Know about the Latest Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)
-    [Aqua Team. (2026, April 1). Update: Ongoing Investigation and Continued Remediation. Retrieved July 1, 2026.](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)
-    [Unit 42. (2026, March 31). Weaponizing the Protectors: TeamPCP’s Multi-Stage Supply Chain Attack on Security Infrastructure. Retrieved July 1, 2026.](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)
-    [Sysdig Threat Research Team. (2026, March 23). TeamPCP expands: Supply chain compromise spreads from Trivy to Checkmarx GitHub Actions. Retrieved July 1, 2026.](https://www.sysdig.com/blog/teampcp-expands-supply-chain-compromise-spreads-from-trivy-to-checkmarx-github-actions)
-    [McCarthy, R., Haughom, J., Read, B. (2026, March 23). KICS GitHub Action Compromised: TeamPCP Strikes Again in Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/teampcp-attack-kics-github-action)
-    [Google Threat Intelligence Group. (2026, May 11). GTIG AI Threat Tracker: Adversaries Leverage AI for Vulnerability Exploitation, Augmented Operations, and Initial Access. Retrieved July 7, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ai-vulnerability-exploitation-initial-access)
-    [Chris Traynor. (2024, January 18). Rooting For Secrets with TruffleHog. Retrieved April 15, 2026.](https://www.blackhillsinfosec.com/rooting-for-secrets-with-trufflehog/)
-    [National Security Agency, Cybersecurity and Infrastructure Security Agency. (2022, March). Kubernetes Hardening Guide. Retrieved April 1, 2022.](https://media.defense.gov/2022/Aug/29/2003066362/-1/-1/0/CTR_KUBERNETES_HARDENING_GUIDANCE_1.2_20220829.PDF)

×
