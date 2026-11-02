---
url: "https://attack.mitre.org/techniques/T1552/004/"
title: "Unsecured Credentials: Private Keys, Sub-technique T1552.004 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:52:48Z"
via: "live"
sha256: "d7f7f511cd4ee84c81e79fbff52fabc1b22f09c042ed806f80e9343723f49c6d"
wayback: "https://web.archive.org/web/20261023090826/https://attack.mitre.org/techniques/T1552/004/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Unsecured Credentials: Private Keys, Sub-technique T1552.004 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1552/004/>
> Snapshot: <https://web.archive.org/web/20261023090826/https://attack.mitre.org/techniques/T1552/004/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Unsecured Credentials](https://attack.mitre.org/techniques/T1552)
- Private Keys

#  Unsecured Credentials: Private Keys

##### Other sub-techniques of Unsecured Credentials (8)

| ID  | Name

|  [T1552.001](https://attack.mitre.org/techniques/T1552/001/)   |  [Credentials In Files](https://attack.mitre.org/techniques/T1552/001/)

|  [T1552.002](https://attack.mitre.org/techniques/T1552/002/)   |  [Credentials in Registry](https://attack.mitre.org/techniques/T1552/002/)

|  [T1552.003](https://attack.mitre.org/techniques/T1552/003/)   |  [Shell History](https://attack.mitre.org/techniques/T1552/003/)

|  T1552.004   |  Private Keys

|  [T1552.005](https://attack.mitre.org/techniques/T1552/005/)   |  [Cloud Instance Metadata API](https://attack.mitre.org/techniques/T1552/005/)

|  [T1552.006](https://attack.mitre.org/techniques/T1552/006/)   |  [Group Policy Preferences](https://attack.mitre.org/techniques/T1552/006/)

|  [T1552.007](https://attack.mitre.org/techniques/T1552/007/)   |  [Container API](https://attack.mitre.org/techniques/T1552/007/)

|  [T1552.008](https://attack.mitre.org/techniques/T1552/008/)   |  [Chat Messages](https://attack.mitre.org/techniques/T1552/008/)

Adversaries may search for private key certificate files on compromised systems for insecurely stored credentials. Private cryptographic keys and certificates are used for authentication, encryption/decryption, and digital signatures.[[1]](https://en.wikipedia.org/wiki/Public-key_cryptography) Common key and certificate file extensions include: .key, .pgp, .gpg, .ppk., .p12, .pem, .pfx, .cer, .p7b, .asc.

Adversaries may also look in common key directories, such as ~/.ssh for SSH keys on * nix-based systems or C:\Users\(username)\.ssh\ on Windows. Adversary tools may also search compromised systems for file extensions relating to cryptographic keys and certificates.[[2]](https://web.archive.org/web/20141031134104/http://kasperskycontenthub.com/wp-content/uploads/sites/43/vlpdfs/unveilingthemask_v1.0.pdf)[[3]](https://researchcenter.paloaltonetworks.com/2016/06/unit42-prince-of-persia-game-over/)

When a device is registered to Entra ID, a device key and a transport key are generated and used to verify the device’s identity.[[4]](https://learn.microsoft.com/en-us/azure/active-directory/devices/concept-primary-refresh-token) An adversary with access to the device may be able to export the keys in order to impersonate the device.[[5]](https://aadinternals.com/post/deviceidentity/)

On network devices, private keys may be exported via [Network Device CLI](https://attack.mitre.org/techniques/T1059/008) commands such as crypto pki export.[[6]](https://www.cisco.com/c/en/us/td/docs/ios-xml/ios/sec_conn_pki/configuration/xe-17/sec-pki-xe-17-book/sec-deploy-rsa-pki.html#GUID-1CB802D8-9DE3-447F-BECE-CF22F5E11436)

Some private keys require a password or passphrase for operation, so an adversary may also use [Input Capture](https://attack.mitre.org/techniques/T1056) for keylogging or attempt to [Brute Force](https://attack.mitre.org/techniques/T1110) the passphrase off-line. These private keys can be used to authenticate to [Remote Services](https://attack.mitre.org/techniques/T1021) like SSH or for use in decrypting other collected files such as email.

ID: T1552.004

Sub-technique of:  [T1552](https://attack.mitre.org/techniques/T1552)

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Linux, Network Devices, Windows, macOS

Contributors: Austin Clark, @c2defense; Itzik Kotler, SafeBreach

Version: 1.3

Created: 04 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1552/004/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1552/004/)

## Procedure Examples

| ID  | Name  | Description

|  [S0677](https://attack.mitre.org/software/S0677)   |  [AADInternals](https://attack.mitre.org/software/S0677)   |

[AADInternals](https://attack.mitre.org/software/S0677) can gather encryption keys from Azure AD services such as ADSync and Active Directory Federated Services servers.[[7]](https://o365blog.com/aadinternals)

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary tasked Claude Code to harvest credentials and SSH keys from victim systems.[[8]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has gathered cryptographic key data related to cryptocurrency wallets, SSH keys, and messaging applications to include Telegram.[[9]](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)

|  [S9042](https://attack.mitre.org/software/S9042)   |  [CanisterWorm](https://attack.mitre.org/software/S9042)   |

[CanisterWorm](https://attack.mitre.org/software/S9042) has gathered SSH private keys from the .ssh file.[[10]](https://www.aikido.dev/blog/teampcp-stage-payload-canisterworm-iran)

|  [S0377](https://attack.mitre.org/software/S0377)   |  [Ebury](https://attack.mitre.org/software/S0377)   |

[Ebury](https://attack.mitre.org/software/S0377) has intercepted unencrypted private keys as well as private key pass-phrases.[[11]](https://www.welivesecurity.com/2014/02/21/an-in-depth-analysis-of-linuxebury/)

|  [S0363](https://attack.mitre.org/software/S0363)   |  [Empire](https://attack.mitre.org/software/S0363)   |

[Empire](https://attack.mitre.org/software/S0363) can use modules like Invoke-SessionGopher to extract private key and session information.[[12]](https://github.com/PowerShellEmpire/Empire)

|  [S0661](https://attack.mitre.org/software/S0661)   |  [FoggyWeb](https://attack.mitre.org/software/S0661)   |

[FoggyWeb](https://attack.mitre.org/software/S0661) can retrieve token signing certificates and token decryption certificates from a compromised AD FS server.[[13]](https://www.microsoft.com/security/blog/2021/09/27/foggyweb-targeted-nobelium-malware-leads-to-persistent-backdoor/)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has searched for private keys in .ssh.[[14]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [S0283](https://attack.mitre.org/software/S0283)   |  [jRAT](https://attack.mitre.org/software/S0283)   |

[jRAT](https://attack.mitre.org/software/S0283) can steal keys for VPNs and cryptocurrency wallets.[[15]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has accessed a Local State files associated with Chromium-based browsers that contain the AES key used to encrypt passwords stored in the browser to include app_bound_encrypted_key.[[16]](https://www.aryaka.com/docs/reports/aryaka-kimsuky-apt-operational-blueprint.pdf)

|  [S0599](https://attack.mitre.org/software/S0599)   |  [Kinsing](https://attack.mitre.org/software/S0599)   |

[Kinsing](https://attack.mitre.org/software/S0599) has searched for private keys.[[17]](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)

|  [S0409](https://attack.mitre.org/software/S0409)   |  [Machete](https://attack.mitre.org/software/S0409)   |

[Machete](https://attack.mitre.org/software/S0409) has scanned and looked for cryptographic keys and certificate file extensions.[[18]](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)

|  [S1060](https://attack.mitre.org/software/S1060)   |  [Mafalda](https://attack.mitre.org/software/S1060)   |

[Mafalda](https://attack.mitre.org/software/S1060) can collect a Chrome encryption key used to protect browser cookies.[[19]](https://assets.sentinelone.com/sentinellabs22/metador#page=1)

|  [S0002](https://attack.mitre.org/software/S0002)   |  [Mimikatz](https://attack.mitre.org/software/S0002)   |

[Mimikatz](https://attack.mitre.org/software/S0002)'s CRYPTO::Extract module can extract keys by interacting with Windows cryptographic application programming interface (API) functions.[[20]](https://adsecurity.org/?page_id=1821)

|  [S9043](https://attack.mitre.org/software/S9043)   |  [Mini Shai-Hulud](https://attack.mitre.org/software/S9043)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has gathered unsecured credentials to include SSH private keys within .ssh.[[21]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[22]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[23]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors used [Mimikatz](https://attack.mitre.org/software/S0002) to dump certificates and private keys from the Windows certificate store.[[24]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106) has used SSH private keys on the infected machine to spread its coinminer throughout a network.[[25]](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)

|  [G1015](https://attack.mitre.org/groups/G1015)   |  [Scattered Spider](https://attack.mitre.org/groups/G1015)   |

[Scattered Spider](https://attack.mitre.org/groups/G1015) enumerate and exfiltrate code-signing certificates from a compromised host.[[26]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-320a)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) obtained PKI keys, certificate files, and the private encryption key from an Active Directory Federation Services (AD FS) container to decrypt corresponding SAML signing certificates.[[27]](https://www.microsoft.com/security/blog/2020/12/28/using-microsoft-365-defender-to-coordinate-protection-against-solorigate/)[[28]](https://www.ncsc.gov.uk/files/Advisory-further-TTPs-associated-with-SVR-cyber-actors.pdf)

|  [G1053](https://attack.mitre.org/groups/G1053)   |  [Storm-0501](https://attack.mitre.org/groups/G1053)   |

[Storm-0501](https://attack.mitre.org/groups/G1053) has leveraged the Azure Owner role to access and steal the Storage Account Access keys using the Microsoft.Storage/storageAccounts/listkeys/action operation.[[29]](https://www.microsoft.com/en-us/security/blog/2025/08/27/storm-0501s-evolving-techniques-lead-to-cloud-based-ransomware/)

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has used malware to extract SSH and GPG keys from victim environments.[[30]](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)[[31]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)[[32]](https://www.ic3.gov/CSA/2026/260702.pdf)

|  [S9041](https://attack.mitre.org/software/S9041)   |  [TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041)   |

[TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041) has searched victim hosts for TLS and SSH keys.[[30]](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)[[33]](https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23)[[34]](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)[[35]](https://www.wiz.io/blog/teampcp-attack-kics-github-action)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has searched for unsecured SSH keys.[[36]](https://www.cadosecurity.com/team-tnt-the-first-crypto-mining-worm-to-steal-aws-credentials/)[[37]](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)

|  [S1196](https://attack.mitre.org/software/S1196)   |  [Troll Stealer](https://attack.mitre.org/software/S1196)   |

[Troll Stealer](https://attack.mitre.org/software/S1196) collects all data in victim .ssh folders by creating a compressed copy that is subsequently exfiltrated to command and control infrastructure. [Troll Stealer](https://attack.mitre.org/software/S1196) also collects key information associated with the Government Public Key Infrastructure (GPKI) service for South Korean government information systems.[[38]](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)[[39]](https://www.security.com/threat-intelligence/springtail-kimsuky-backdoor-espionage)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has accessed a Local State file that contains the AES key used to encrypt passwords stored in the Chrome browser.[[40]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Ensure only authorized keys are allowed access to critical resources and audit access lists regularly.

|  [M1041](https://attack.mitre.org/mitigations/M1041)   |  [Encrypt Sensitive Information](https://attack.mitre.org/mitigations/M1041)   |

When possible, store keys on separate cryptographic hardware instead of on the local system. For example, on Windows systems use a TPM to secure keys and other sensitive credential material.[[4]](https://learn.microsoft.com/en-us/azure/active-directory/devices/concept-primary-refresh-token)

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Use strong passphrases for private keys to make cracking difficult.

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Ensure permissions are properly set on folders containing sensitive private keys to prevent unintended access. Additionally, on Cisco devices, set the nonexportable flag during RSA key pair generation.[[6]](https://www.cisco.com/c/en/us/td/docs/ios-xml/ios/sec_conn_pki/configuration/xe-17/sec-pki-xe-17-book/sec-deploy-rsa-pki.html#GUID-1CB802D8-9DE3-447F-BECE-CF22F5E11436)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0549](https://attack.mitre.org/detectionstrategies/DET0549)   |  [Detect Suspicious Access to Private Key Files and Export Attempts Across Platforms](https://attack.mitre.org/detectionstrategies/DET0549)   |  [AN1516](https://attack.mitre.org/detectionstrategies/DET0549#AN1516)   |

A process (non-system or user-initiated) accesses private key files in user profile paths or system certificate stores followed by potential network connections or compression activity.

|  [AN1517](https://attack.mitre.org/detectionstrategies/DET0549#AN1517)   |

User or script-based access to ~/.ssh or other directories containing private keys followed by unusual shell activity or network connections.

|  [AN1518](https://attack.mitre.org/detectionstrategies/DET0549#AN1518)   |

Access to user private key directories (e.g., /Users/*/.ssh) via Terminal, scripting engines, or non-default processes.

|  [AN1519](https://attack.mitre.org/detectionstrategies/DET0549#AN1519)   |

CLI-based export of private key material (e.g., 'crypto pki export') with anomalous user session or AAA role escalation.

## References

-    [Wikipedia. (2017, June 29). Public-key cryptography. Retrieved July 5, 2017.](https://en.wikipedia.org/wiki/Public-key_cryptography)
-    [Kaspersky Labs. (2014, February 11). Unveiling “Careto” - The Masked APT. Retrieved July 5, 2017.](https://web.archive.org/web/20141031134104/http://kasperskycontenthub.com/wp-content/uploads/sites/43/vlpdfs/unveilingthemask_v1.0.pdf)
-    [Bar, T., Conant, S., Efraim, L. (2016, June 28). Prince of Persia – Game Over. Retrieved July 5, 2017.](https://researchcenter.paloaltonetworks.com/2016/06/unit42-prince-of-persia-game-over/)
-    [Microsoft. (2022, September 9). What is a Primary Refresh Token?. Retrieved February 21, 2023.](https://learn.microsoft.com/en-us/azure/active-directory/devices/concept-primary-refresh-token)
-    [Dr. Nestori Syynimaa. (2022, February 15). Stealing and faking Azure AD device identities. Retrieved February 21, 2023.](https://aadinternals.com/post/deviceidentity/)
-    [Cisco. (2023, February 17). Chapter: Deploying RSA Keys Within a PKI . Retrieved March 27, 2023.](https://www.cisco.com/c/en/us/td/docs/ios-xml/ios/sec_conn_pki/configuration/xe-17/sec-pki-xe-17-book/sec-deploy-rsa-pki.html#GUID-1CB802D8-9DE3-447F-BECE-CF22F5E11436)
-    [Dr. Nestori Syynimaa. (2018, October 25). AADInternals. Retrieved February 18, 2022.](https://o365blog.com/aadinternals)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)
-    [Microsoft Threat Intelligence. (2026, April 16). Dissecting Sapphire Sleet’s macOS intrusion from lure to compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)
-    [Eriksen, C. (2026, March 22). CanisterWorm Gets Teeth: TeamPCP's Kubernetes Wiper Targets Iran. Retrieved July 27, 2026.](https://www.aikido.dev/blog/teampcp-stage-payload-canisterworm-iran)
-    [M.Léveillé, M.. (2014, February 21). An In-depth Analysis of Linux/Ebury. Retrieved April 19, 2019.](https://www.welivesecurity.com/2014/02/21/an-in-depth-analysis-of-linuxebury/)
-    [Schroeder, W., Warner, J., Nelson, M. (n.d.). Github PowerShellEmpire. Retrieved April 28, 2016.](https://github.com/PowerShellEmpire/Empire)
-    [Ramin Nafisi. (2021, September 27). FoggyWeb: Targeted NOBELIUM malware leads to persistent backdoor. Retrieved October 4, 2021.](https://www.microsoft.com/security/blog/2021/09/27/foggyweb-targeted-nobelium-malware-leads-to-persistent-backdoor/)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [Kamluk, V. & Gostev, A. (2016, February). Adwind - A Cross-Platform RAT. Retrieved April 23, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)
-    [Varadharajan Krishnasamy, Aditya K Sood. (2025, July 29). From Reconnaissance to Control: The Operational Blueprint of Kimsuky APT for Cyber Espionage. Retrieved April 18, 2026.](https://www.aryaka.com/docs/reports/aryaka-kimsuky-apt-operational-blueprint.pdf)
-    [Singer, G. (2020, April 3). Threat Alert: Kinsing Malware Attacks Targeting Container Environments. Retrieved April 1, 2021.](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)
-    [ESET. (2019, July). MACHETE JUST GOT SHARPER Venezuelan government institutions under attack. Retrieved September 13, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)
-    [Ehrlich, A., et al. (2022, September). THE MYSTERY OF METADOR | AN UNATTRIBUTED THREAT HIDING IN TELCOS, ISPS, AND UNIVERSITIES. Retrieved January 23, 2023.](https://assets.sentinelone.com/sentinellabs22/metador#page=1)
-    [Metcalf, S. (2015, November 13). Unofficial Guide to Mimikatz & Command Reference. Retrieved December 23, 2015.](https://adsecurity.org/?page_id=1821)

-    [Santos, J. and Navato, J.R. (2026, May 13). Analyzing TeamPCP’s Supply Chain Attacks: Checkmarx KICS and elementary-data in CI/CD Credential Theft. Retrieved July 16, 2026.](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)
-    [Hunt.io. (2026, May 14). How TeamPCP's Python Toolkit Survives a C2 Takedown: FIRESCALE, GitHub, and the Victim's Own Account. Retrieved July 16, 2026.](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)
-    [Flashpoint. (2026, May 28). The Mini Shai-Hulud Worm and the New Era of CI/CD Exploitation. Retrieved July 16, 2026.](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [Anomali Labs. (2019, March 15). Rocke Evolves Its Arsenal With a New Malware Family Written in Golang. Retrieved April 24, 2019.](https://www.anomali.com/blog/rocke-evolves-its-arsenal-with-a-new-malware-family-written-in-golang)
-    [CISA. (2023, November 16). Cybersecurity Advisory: Scattered Spider (AA23-320A). Retrieved March 18, 2024.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-320a)
-    [Microsoft 365 Defender Team. (2020, December 28). Using Microsoft 365 Defender to protect against Solorigate. Retrieved January 7, 2021.](https://www.microsoft.com/security/blog/2020/12/28/using-microsoft-365-defender-to-coordinate-protection-against-solorigate/)
-    [NCSC, CISA, FBI, NSA. (2021, May 7). Further TTPs associated with SVR cyber actors. Retrieved July 29, 2021.](https://www.ncsc.gov.uk/files/Advisory-further-TTPs-associated-with-SVR-cyber-actors.pdf)
-    [Microsoft Threat Intelligence. (2025, August 27). Storm-0501’s evolving techniques lead to cloud-based ransomware. Retrieved October 19, 2025.](https://www.microsoft.com/en-us/security/blog/2025/08/27/storm-0501s-evolving-techniques-lead-to-cloud-based-ransomware/)
-    [McCarthy, R. (2026, March 20). Trivy Compromised: Everything You Need to Know about the Latest Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack)
-    [Unit 42. (2026, March 31). Weaponizing the Protectors: TeamPCP’s Multi-Stage Supply Chain Attack on Security Infrastructure. Retrieved July 1, 2026.](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)
-    [FBI. (2026, July 2). Cyber Criminal Group TeamPCP. Retrieved July 7, 2026.](https://www.ic3.gov/CSA/2026/260702.pdf)
-    [Aqua Security . (2026, March 21). Trivy ecosystem supply chain temporarily compromised. Retrieved July 1, 2026.](https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23)
-    [Aqua Team. (2026, April 1). Update: Ongoing Investigation and Continued Remediation. Retrieved July 1, 2026.](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)
-    [McCarthy, R., Haughom, J., Read, B. (2026, March 23). KICS GitHub Action Compromised: TeamPCP Strikes Again in Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/teampcp-attack-kics-github-action)
-    [Cado Security. (2020, August 16). Team TNT – The First Crypto-Mining Worm to Steal AWS Credentials. Retrieved September 22, 2021.](https://www.cadosecurity.com/team-tnt-the-first-crypto-mining-worm-to-steal-aws-credentials/)
-    [Fiser, D. Oliveira, A. (n.d.). Tracking the Activities of TeamTNT A Closer Look at a Cloud-Focused Malicious Actor Group. Retrieved September 22, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)
-    [Jiho Kim & Sebin Lee, S2W. (2024, February 7). Kimsuky disguised as a Korean company signed with a valid certificate to distribute Troll Stealer (English ver.). Retrieved January 17, 2025.](https://medium.com/s2wblog/kimsuky-disguised-as-a-korean-company-signed-with-a-valid-certificate-to-distribute-troll-stealer-cfa5d54314e2)
-    [Symantec Threat Hunter Team. (2024, May 16). Springtail: New Linux Backdoor Added to Toolkit. Retrieved January 17, 2025.](https://www.security.com/threat-intelligence/springtail-kimsuky-backdoor-espionage)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

×
