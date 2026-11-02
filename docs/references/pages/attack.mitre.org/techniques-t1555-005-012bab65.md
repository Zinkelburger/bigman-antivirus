---
url: "https://attack.mitre.org/techniques/T1555/005/"
title: "Credentials from Password Stores: Password Managers, Sub-technique T1555.005 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:54:48Z"
via: "live"
sha256: "15203e4bd85deb9dee7de2b5a37daa93594945bd4e89a55f0804ba82f798d034"
wayback: "https://web.archive.org/web/20261102192643/https://attack.mitre.org/techniques/T1555/005/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Credentials from Password Stores: Password Managers, Sub-technique T1555.005 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1555/005/>
> Snapshot: <https://web.archive.org/web/20261102192643/https://attack.mitre.org/techniques/T1555/005/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555)
- Password Managers

#  Credentials from Password Stores: Password Managers

##### Other sub-techniques of Credentials from Password Stores (6)

| ID  | Name

|  [T1555.001](https://attack.mitre.org/techniques/T1555/001/)   |  [Keychain](https://attack.mitre.org/techniques/T1555/001/)

|  [T1555.002](https://attack.mitre.org/techniques/T1555/002/)   |  [Securityd Memory](https://attack.mitre.org/techniques/T1555/002/)

|  [T1555.003](https://attack.mitre.org/techniques/T1555/003/)   |  [Credentials from Web Browsers](https://attack.mitre.org/techniques/T1555/003/)

|  [T1555.004](https://attack.mitre.org/techniques/T1555/004/)   |  [Windows Credential Manager](https://attack.mitre.org/techniques/T1555/004/)

|  T1555.005   |  Password Managers

|  [T1555.006](https://attack.mitre.org/techniques/T1555/006/)   |  [Cloud Secrets Management Stores](https://attack.mitre.org/techniques/T1555/006/)

Adversaries may acquire user credentials from third-party password managers.[[1]](https://www.ise.io/casestudies/password-manager-hacking/) Password managers are applications designed to store user credentials, normally in an encrypted database. Credentials are typically accessible after a user provides a master password that unlocks the database. After the database is unlocked, these credentials may be copied to memory. These databases can be stored as files on disk.[[1]](https://www.ise.io/casestudies/password-manager-hacking/)

Adversaries may acquire user credentials from password managers by extracting the master password and/or plain-text credentials from memory.[[2]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)[[3]](https://github.com/GhostPack/KeeThief) Adversaries may extract credentials from memory via [Exploitation for Credential Access](https://attack.mitre.org/techniques/T1212).[[4]](https://nvd.nist.gov/vuln/detail/CVE-2019-3610)
Adversaries may also try brute forcing via [Password Guessing](https://attack.mitre.org/techniques/T1110/001) to obtain the master password of a password manager.[[5]](https://www.cybereason.com/blog/dropping-anchor-from-a-trickbot-infection-to-the-discovery-of-the-anchor-malware)

ID: T1555.005

Sub-technique of:  [T1555](https://attack.mitre.org/techniques/T1555)

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Don Le, Stifel Financial; Matt Burrough, @mattburrough, Microsoft

Version: 1.1

Created: 22 January 2021

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1555/005/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1555/005/)

## Procedure Examples

| ID  | Name  | Description

|  [G0117](https://attack.mitre.org/groups/G0117)   |  [Fox Kitten](https://attack.mitre.org/groups/G0117)   |

[Fox Kitten](https://attack.mitre.org/groups/G0117) has used scripts to access credential information from the KeePass database.[[6]](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)

|  [G0119](https://attack.mitre.org/groups/G0119)   |  [Indrik Spider](https://attack.mitre.org/groups/G0119)   |

[Indrik Spider](https://attack.mitre.org/groups/G0119) has accessed and exported passwords from password managers.[[7]](https://cloud.google.com/blog/topics/threat-intelligence/unc2165-shifts-to-evade-sanctions/)

|  [S1245](https://attack.mitre.org/software/S1245)   |  [InvisibleFerret](https://attack.mitre.org/software/S1245)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has utilized the command ssh_zcp to exfiltrate data from browser extensions and password managers via Telegram and FTP.[[8]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[9]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  [G1004](https://attack.mitre.org/groups/G1004)   |  [LAPSUS$](https://attack.mitre.org/groups/G1004)   |

[LAPSUS$](https://attack.mitre.org/groups/G1004) has accessed local password managers and databases to obtain further credentials from a compromised network.[[10]](https://www.nccgroup.com/research-blog/lapsus-recent-techniques-tactics-and-procedures/)

|  [S0652](https://attack.mitre.org/software/S0652)   |  [MarkiRAT](https://attack.mitre.org/software/S0652)   |

[MarkiRAT](https://attack.mitre.org/software/S0652) can gather information from the Keepass password manager.[[11]](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)

|  [S9043](https://attack.mitre.org/software/S9043)   |  [Mini Shai-Hulud](https://attack.mitre.org/software/S9043)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has gathered credentials stored in password managers to include password vaults.[[12]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[13]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[14]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  [C0014](https://attack.mitre.org/campaigns/C0014)   |  [Operation Wocao](https://attack.mitre.org/campaigns/C0014)   |

During [Operation Wocao](https://attack.mitre.org/campaigns/C0014), threat actors accessed and collected credentials from password managers.[[2]](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)

|  [S0279](https://attack.mitre.org/software/S0279)   |  [Proton](https://attack.mitre.org/software/S0279)   |

[Proton](https://attack.mitre.org/software/S0279) gathers credentials in files for 1password.[[15]](https://objective-see.com/blog/blog_0x25.html)

|  [G1015](https://attack.mitre.org/groups/G1015)   |  [Scattered Spider](https://attack.mitre.org/groups/G1015)   |

[Scattered Spider](https://attack.mitre.org/groups/G1015) has searched for credentials in password vaults and Privileged Access Management (PAM) solutions including HashiCorp Vault.[[16]](https://cloud.google.com/blog/topics/threat-intelligence/unc3944-proactive-hardening-recommendations)[[17]](https://cloud.google.com/blog/topics/threat-intelligence/defending-vsphere-from-unc3944)

|  [G1053](https://attack.mitre.org/groups/G1053)   |  [Storm-0501](https://attack.mitre.org/groups/G1053)   |

[Storm-0501](https://attack.mitre.org/groups/G1053) has stolen credentials contained in the password manager Keepass by utilizing Find-KeePassConfig.ps1.[[18]](https://www.microsoft.com/en-us/security/blog/2024/09/26/storm-0501-ransomware-attacks-expanding-to-hybrid-cloud-environments/)

|  [G0027](https://attack.mitre.org/groups/G0027)   |  [Threat Group-3390](https://attack.mitre.org/groups/G0027)   |

[Threat Group-3390](https://attack.mitre.org/groups/G0027) obtained a KeePass database from a compromised host.[[19]](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)

|  [S0266](https://attack.mitre.org/software/S0266)   |  [TrickBot](https://attack.mitre.org/software/S0266)   |

[TrickBot](https://attack.mitre.org/software/S0266) can steal passwords from the KeePass open source password manager.[[5]](https://www.cybereason.com/blog/dropping-anchor-from-a-trickbot-infection-to-the-discovery-of-the-anchor-malware)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has targeted KeyPass password database files for credential access.[[20]](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)

## Mitigations

| ID  | Mitigation  | Description

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Refer to NIST guidelines when creating password policies for master passwords.[[21]](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-63-4.pdf)

|  [M1054](https://attack.mitre.org/mitigations/M1054)   |  [Software Configuration](https://attack.mitre.org/mitigations/M1054)   |

Consider re-locking password managers after a short timeout to limit the time plaintext credentials live in memory from decrypted databases.

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Regularly update web browsers, password managers, and all related software to the latest versions. Keeping software up-to-date reduces the risk of vulnerabilities being exploited by attackers to extract stored credentials or session cookies.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Implement strict user account management policies to prevent unnecessary accounts from accessing sensitive systems. Regularly audit user accounts to identify and disable inactive accounts that may be targeted by attackers to extract credentials or gain unauthorized access.

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Provide user training on secure practices for managing credentials, including avoiding storing sensitive passwords in browsers and using password managers securely. Users should also be educated on identifying phishing attempts that could steal session cookies or credentials.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0597](https://attack.mitre.org/detectionstrategies/DET0597)   |  [Detect Unauthorized Access to Password Managers](https://attack.mitre.org/detectionstrategies/DET0597)   |  [AN1641](https://attack.mitre.org/detectionstrategies/DET0597#AN1641)   |

Detection of suspicious access to password manager processes (KeePass, 1Password, LastPass, Bitwarden) through abnormal process injection, memory reads, or command-line usage of vault-related DLLs. Correlates process creation with OS API calls and file access to vault databases (.kdbx, .opvault, .ldb).

|  [AN1642](https://attack.mitre.org/detectionstrategies/DET0597#AN1642)   |

Suspicious access to password manager vaults (KeePassXC, gnome-keyring, pass) via memory scraping or unauthorized file reads. Detects unusual command execution involving gdb/strace attached to password manager processes.

|  [AN1643](https://attack.mitre.org/detectionstrategies/DET0597#AN1643)   |

Detection of password manager database access (1Password .opvault, LastPass caches, KeePass .kdbx) outside expected parent processes. Identifies memory scraping attempts via suspicious API calls or tools attaching to password manager processes.

## References

-    [ise. (2019, February 19). Password Managers: Under the Hood of Secrets Management. Retrieved January 22, 2021.](https://www.ise.io/casestudies/password-manager-hacking/)
-    [Dantzig, M. v., Schamper, E. (2019, December 19). Operation Wocao: Shining a light on one of China’s hidden hacking groups. Retrieved October 8, 2020.](https://www.fox-it.com/media/kadlze5c/201912_report_operation_wocao.pdf)
-    [Lee, C., Schoreder, W. (n.d.). KeeThief. Retrieved February 8, 2021.](https://github.com/GhostPack/KeeThief)
-    [National Vulnerability Database. (2019, October 9). CVE-2019-3610 Detail. Retrieved April 14, 2021.](https://nvd.nist.gov/vuln/detail/CVE-2019-3610)
-    [Dahan, A. et al. (2019, December 11). DROPPING ANCHOR: FROM A TRICKBOT INFECTION TO THE DISCOVERY OF THE ANCHOR MALWARE. Retrieved September 10, 2020.](https://www.cybereason.com/blog/dropping-anchor-from-a-trickbot-infection-to-the-discovery-of-the-anchor-malware)
-    [CISA. (2020, September 15). Iran-Based Threat Actor Exploits VPN Vulnerabilities. Retrieved December 21, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-259a)
-    [Mandiant Intelligence. (2022, June 2). To HADES and Back: UNC2165 Shifts to LOCKBIT to Evade Sanctions. Retrieved July 29, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/unc2165-shifts-to-evade-sanctions/)
-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [Seongsu Park. (2024, November 4). From Pyongyang to Your Payroll: The Rise of North Korean Remote Workers in the West. Retrieved October 17, 2025.](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)
-    [Brown, D., et al. (2022, April 28). LAPSUS$: Recent techniques, tactics and procedures. Retrieved December 22, 2022.](https://www.nccgroup.com/research-blog/lapsus-recent-techniques-tactics-and-procedures/)
-    [GReAT. (2021, June 16). Ferocious Kitten: 6 Years of Covert Surveillance in Iran. Retrieved September 22, 2021.](https://securelist.com/ferocious-kitten-6-years-of-covert-surveillance-in-iran/102806/)

-    [McCarthy, R., Cohen, A., and Read, B. (2026, May 12). Mini Shai-Hulud Strikes Again: TanStack + more npm Packages Compromised. Retrieved July 16, 2026.](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)
-    [Hunt.io. (2026, May 14). How TeamPCP's Python Toolkit Survives a C2 Takedown: FIRESCALE, GitHub, and the Victim's Own Account. Retrieved July 16, 2026.](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)
-    [Webb, M. (2026, May 20). TeamPCP Wave Four: GitHub Breach via Poisoned VS Code Extension, durabletask PyPI Worm, and ~4,000 Internal Repositories Exfiltrated. Retrieved July 16, 2026.](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)
-    [Patrick Wardle. (n.d.). Mac Malware of 2017. Retrieved September 21, 2018.](https://objective-see.com/blog/blog_0x25.html)
-    [Mandiant Incident Response. (2025, May 6). Defending Against UNC3944: Cybercrime Hardening Guidance from the Frontlines. Retrieved October 13, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/unc3944-proactive-hardening-recommendations)
-    [Mandiant Incident Response. (2025, July 23). From Help Desk to Hypervisor: Defending Your VMware vSphere Estate from UNC3944. Retrieved October 13, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/defending-vsphere-from-unc3944)
-    [Microsoft Threat Intelligence. (2024, September 26). Storm-0501: Ransomware attacks expanding to hybrid cloud environments. Retrieved October 19, 2025.](https://www.microsoft.com/en-us/security/blog/2024/09/26/storm-0501-ransomware-attacks-expanding-to-hybrid-cloud-environments/)
-    [Lunghi, D. et al. (2020, February). Uncovering DRBControl. Retrieved November 12, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-uncovering-DRBcontrol.pdf)
-    [Alexander Marvi, Jeremy Koppen, Tufail Ahmed, and Jonathan Lepore. (2022, September 29). Bad VIB(E)s Part One: Investigating Novel Malware Persistence Within ESXi Hypervisors. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)
-    [Temoshok, D., et al. (July 2025). SP 800-63-4, Digital Identity Guidelines. Retrieved July 29, 2026.](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-63-4.pdf)

×
