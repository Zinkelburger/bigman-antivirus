---
url: "https://attack.mitre.org/techniques/T1556/"
title: "Modify Authentication Process, Technique T1556 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:54:57Z"
via: "live"
sha256: "f0fa8500524ec5303ee8b38aa9e715563f4ebfdefb2e69538e73d6c69ff5ac1e"
wayback: "https://web.archive.org/web/20261019211110/https://attack.mitre.org/techniques/T1556/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Modify Authentication Process, Technique T1556 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1556/>
> Snapshot: <https://web.archive.org/web/20261019211110/https://attack.mitre.org/techniques/T1556/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Modify Authentication Process

#  Modify Authentication Process

##### Sub-techniques (9)

| ID  | Name

|  [T1556.001](https://attack.mitre.org/techniques/T1556/001/)   |  [Domain Controller Authentication](https://attack.mitre.org/techniques/T1556/001/)

|  [T1556.002](https://attack.mitre.org/techniques/T1556/002/)   |  [Password Filter DLL](https://attack.mitre.org/techniques/T1556/002/)

|  [T1556.003](https://attack.mitre.org/techniques/T1556/003/)   |  [Pluggable Authentication Modules](https://attack.mitre.org/techniques/T1556/003/)

|  [T1556.004](https://attack.mitre.org/techniques/T1556/004/)   |  [Network Device Authentication](https://attack.mitre.org/techniques/T1556/004/)

|  [T1556.005](https://attack.mitre.org/techniques/T1556/005/)   |  [Reversible Encryption](https://attack.mitre.org/techniques/T1556/005/)

|  [T1556.006](https://attack.mitre.org/techniques/T1556/006/)   |  [Multi-Factor Authentication](https://attack.mitre.org/techniques/T1556/006/)

|  [T1556.007](https://attack.mitre.org/techniques/T1556/007/)   |  [Hybrid Identity](https://attack.mitre.org/techniques/T1556/007/)

|  [T1556.008](https://attack.mitre.org/techniques/T1556/008/)   |  [Network Provider DLL](https://attack.mitre.org/techniques/T1556/008/)

|  [T1556.009](https://attack.mitre.org/techniques/T1556/009/)   |  [Conditional Access Policies](https://attack.mitre.org/techniques/T1556/009/)

Adversaries may modify authentication mechanisms and processes to access user credentials or enable otherwise unwarranted access to accounts. The authentication process is handled by mechanisms, such as the Local Security Authentication Server (LSASS) process and the Security Accounts Manager (SAM) on Windows, pluggable authentication modules (PAM) on Unix-based systems, and authorization plugins on MacOS systems, responsible for gathering, storing, and validating credentials. By modifying an authentication process, an adversary may be able to authenticate to a service or system without using [Valid Accounts](https://attack.mitre.org/techniques/T1078).

Adversaries may maliciously modify a part of this process to either reveal credentials or bypass authentication mechanisms. Compromised credentials or access may be used to bypass access controls placed on various resources on systems within the network and may even be used for persistent access to remote systems and externally available services, such as VPNs, Outlook Web Access and remote desktop.

ID: T1556

Sub-techniques:  [T1556.001](https://attack.mitre.org/techniques/T1556/001), [T1556.002](https://attack.mitre.org/techniques/T1556/002), [T1556.003](https://attack.mitre.org/techniques/T1556/003), [T1556.004](https://attack.mitre.org/techniques/T1556/004), [T1556.005](https://attack.mitre.org/techniques/T1556/005), [T1556.006](https://attack.mitre.org/techniques/T1556/006), [T1556.007](https://attack.mitre.org/techniques/T1556/007), [T1556.008](https://attack.mitre.org/techniques/T1556/008), [T1556.009](https://attack.mitre.org/techniques/T1556/009)

ⓘ

Tactics: [Defense Impairment](https://attack.mitre.org/tactics/TA0112), [Persistence](https://attack.mitre.org/tactics/TA0003), [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: IaaS, Identity Provider, Linux, Network Devices, Office Suite, SaaS, Windows, macOS

Contributors: Chris Ross @xorrior

Version: 3.0

Created: 11 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1556/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1556/)

## Procedure Examples

| ID  | Name  | Description

|  [C0046](https://attack.mitre.org/campaigns/C0046)   |  [ArcaneDoor](https://attack.mitre.org/campaigns/C0046)   |

[ArcaneDoor](https://attack.mitre.org/campaigns/C0046) included modification of the AAA process to bypass authentication mechanisms.[[1]](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)

|  [S9013](https://attack.mitre.org/software/S9013)   |  [DRYHOOK](https://attack.mitre.org/software/S9013)   |

[DRYHOOK](https://attack.mitre.org/software/S9013) has intercepted and logged user credentials by modifying the Perl module in Ivanti Connect Secure VPN edge-devices located within /home/perl/DSAuth.pm.[[2]](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)[[3]](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)

|  [S0377](https://attack.mitre.org/software/S0377)   |  [Ebury](https://attack.mitre.org/software/S0377)   |

[Ebury](https://attack.mitre.org/software/S0377) can intercept private keys using a trojanized ssh-add function.[[4]](https://www.welivesecurity.com/2014/02/21/an-in-depth-analysis-of-linuxebury/)

|  [G1016](https://attack.mitre.org/groups/G1016)   |  [FIN13](https://attack.mitre.org/groups/G1016)   |

[FIN13](https://attack.mitre.org/groups/G1016) has replaced legitimate KeePass binaries with trojanized versions to collect passwords from numerous applications.[[5]](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)

|  [S0487](https://attack.mitre.org/software/S0487)   |  [Kessel](https://attack.mitre.org/software/S0487)   |

[Kessel](https://attack.mitre.org/software/S0487) has trojanized the ssh_login and user-auth_pubkey functions to steal plaintext credentials.[[6]](https://www.welivesecurity.com/wp-content/uploads/2018/12/ESET-The_Dark_Side_of_the_ForSSHe.pdf)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has modified the Authentication, Authorization, and Accounting (AAA) configurations forcing devices to use less secure authentication methods and to send information to their infrastructure.[[7]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  [S0692](https://attack.mitre.org/software/S0692)   |  [SILENTTRINITY](https://attack.mitre.org/software/S0692)   |

[SILENTTRINITY](https://attack.mitre.org/software/S0692) can create a backdoor in KeePass using a malicious config file and in TortoiseSVN using a registry hook.[[8]](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Review authentication logs to ensure that mechanisms such as enforcement of MFA are functioning as intended.

Periodically review the hybrid identity solution in use for any discrepancies. For example, review all Pass Through Authentication (PTA) agents in the Azure Management Portal to identify any unwanted or unapproved ones.[[9]](https://www.mandiant.com/resources/detecting-microsoft-365-azure-active-directory-backdoors) If ADFS is in use, review DLLs and executable files in the AD FS and Global Assembly Cache directories to ensure that they are signed by Microsoft. Note that in some cases binaries may be catalog-signed, which may cause the file to appear unsigned when viewing file properties.[[10]](https://www.microsoft.com/security/blog/2022/08/24/magicweb-nobeliums-post-compromise-trick-to-authenticate-as-anyone/)

Periodically review for new and unknown network provider DLLs within the Registry (HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\<NetworkProviderName>\NetworkProvider\ProviderPath). Ensure only valid network provider DLLs are registered. The name of these can be found in the Registry key at HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\NetworkProvider\Order, and have corresponding service subkey pointing to a DLL at HKEY_LOCAL_MACHINE\SYSTEM\CurrentC ontrolSet\Services\<NetworkProviderName>\NetworkProvider.

|  [M1032](https://attack.mitre.org/mitigations/M1032)   |  [Multi-factor Authentication](https://attack.mitre.org/mitigations/M1032)   |

Integrating multi-factor authentication (MFA) as part of organizational policy can greatly reduce the risk of an adversary gaining control of valid credentials that may be used for additional tactics such as initial access, lateral movement, and collecting information. MFA can also be used to restrict access to cloud resources and APIs.

|  [M1028](https://attack.mitre.org/mitigations/M1028)   |  [Operating System Configuration](https://attack.mitre.org/mitigations/M1028)   |

Ensure only valid password filters are registered. Filter DLLs must be present in Windows installation directory (C:\Windows\System32\ by default) of a domain controller and/or local computer with a corresponding entry in HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Lsa\Notification Packages.

Starting in Windows 11 22H2, the EnableMPRNotifications policy can be disabled through Group Policy or through a configuration service provider to prevent Winlogon from sending credentials to network providers.[[11]](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-windowslogon)

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Ensure that AllowReversiblePasswordEncryption property is set to disabled unless there are application requirements.[[12]](https://docs.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/store-passwords-using-reversible-encryption)

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Audit domain and local accounts as well as their permission levels routinely to look for situations that could allow an adversary to gain wide access by obtaining credentials of a privileged account. [[13]](https://technet.microsoft.com/en-us/library/dn535501.aspx) [[14]](https://technet.microsoft.com/en-us/library/dn487450.aspx) These audits should also include if default accounts have been enabled, or if new local accounts are created that have not be authorized. Follow best practices for design and administration of an enterprise network to limit privileged account use across administrative tiers. [[15]](https://docs.microsoft.com/en-us/windows-server/identity/securing-privileged-access/securing-privileged-access-reference-material#a-nameesaebmaesae-administrative-forest-design-approach)

Limit access to the root account and prevent users from modifying protected components through proper privilege separation (ex SELinux, grsecurity, AppArmor, etc.) and limiting Privilege Escalation opportunities.

Limit on-premises accounts with access to the hybrid identity solution in place. For example, limit Azure AD Global Administrator accounts to only those required, and ensure that these are dedicated cloud-only accounts rather than hybrid ones.[[10]](https://www.microsoft.com/security/blog/2022/08/24/magicweb-nobeliums-post-compromise-trick-to-authenticate-as-anyone/)

|  [M1025](https://attack.mitre.org/mitigations/M1025)   |  [Privileged Process Integrity](https://attack.mitre.org/mitigations/M1025)   |

Enabled features, such as Protected Process Light (PPL), for LSA.[[16]](https://technet.microsoft.com/en-us/library/dn408187.aspx)

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Restrict write access to the /Library/Security/SecurityAgentPlugins directory.

|  [M1024](https://attack.mitre.org/mitigations/M1024)   |  [Restrict Registry Permissions](https://attack.mitre.org/mitigations/M1024)   |

Restrict Registry permissions to disallow the modification of sensitive Registry keys such as HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\NetworkProvider\Order.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Ensure that proper policies are implemented to dictate the the secure enrollment and deactivation of authentication mechanisms, such as MFA, for user accounts.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0104](https://attack.mitre.org/detectionstrategies/DET0104)   |  [Detect Modification of Authentication Processes Across Platforms](https://attack.mitre.org/detectionstrategies/DET0104)   |  [AN0287](https://attack.mitre.org/detectionstrategies/DET0104#AN0287)   |

Detects modification of LSASS and authentication DLLs, suspicious registry changes to password filter packages, and abnormal process access to lsass.exe. Correlates registry modifications, DLL loads, and process handle access events.

|  [AN0288](https://attack.mitre.org/detectionstrategies/DET0104#AN0288)   |

Detects modification of PAM configuration files, unauthorized new PAM modules, and suspicious process execution accessing PAM-related binaries. Correlates file modification events in /etc/pam.d/ with process execution of unauthorized binaries.

|  [AN0289](https://attack.mitre.org/detectionstrategies/DET0104#AN0289)   |

Detects unauthorized additions or changes to /Library/Security/SecurityAgentPlugins and suspicious process activity attempting to hook authentication APIs. Correlates file modifications with abnormal plugin loads in authentication flows.

|  [AN0290](https://attack.mitre.org/detectionstrategies/DET0104#AN0290)   |

Detects suspicious configuration changes in IdP authentication flows such as enabling reversible password encryption, MFA bypass, or policy weakening. Correlates policy modification events with unusual administrative activity.

|  [AN0291](https://attack.mitre.org/detectionstrategies/DET0104#AN0291)   |

Detects unauthorized changes to IAM authentication configurations such as disabling MFA, creating backdoor access keys, or altering trust policies. Correlates identity policy updates with unusual login behavior.

## References

-    [Cisco Talos. (2024, April 24). ArcaneDoor - New espionage-focused campaign found targeting perimeter network devices. Retrieved January 6, 2025.](https://blog.talosintelligence.com/arcanedoor-new-espionage-focused-campaign-found-targeting-perimeter-network-devices/)
-    [John Wolfram, Josh Murchie, Matt Lin, Daniel Ainsworth, Robert Wallace, Dimiter Andonov, Dhanesh Kizhakkinan, Jacob Thompson. (2025, January 8). Ivanti Connect Secure VPN Targeted in New Zero-Day Exploitation. Retrieved April 14, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-connect-secure-vpn-zero-day)
-    [Sila Ozeren Hacioglu. (2025, May 5). UNC5221’s Latest Exploit: Weaponizing CVE-2025-22457 in Ivanti Connect Secure. Retrieved April 13, 2026.](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)
-    [M.Léveillé, M.. (2014, February 21). An In-depth Analysis of Linux/Ebury. Retrieved April 19, 2019.](https://www.welivesecurity.com/2014/02/21/an-in-depth-analysis-of-linuxebury/)
-    [Ta, V., et al. (2022, August 8). FIN13: A Cybercriminal Threat Actor Focused on Mexico. Retrieved February 9, 2023.](https://www.mandiant.com/resources/blog/fin13-cybercriminal-mexico)
-    [Dumont, R., M.Léveillé, M., Porcher, H. (2018, December 1). THE DARK SIDE OF THE FORSSHE A landscape of OpenSSH backdoors. Retrieved July 16, 2020.](https://www.welivesecurity.com/wp-content/uploads/2018/12/ESET-The_Dark_Side_of_the_ForSSHe.pdf)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Salvati, M. (2019, August 6). SILENTTRINITY Modules. Retrieved March 24, 2022.](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)

-    [Mike Burns. (2020, September 30). Detecting Microsoft 365 and Azure Active Directory Backdoors. Retrieved September 28, 2022.](https://www.mandiant.com/resources/detecting-microsoft-365-azure-active-directory-backdoors)
-    [Microsoft Threat Intelligence Center, Microsoft Detection and Response Team, Microsoft 365 Defender Research Team . (2022, August 24). MagicWeb: NOBELIUM’s post-compromise trick to authenticate as anyone. Retrieved September 28, 2022.](https://www.microsoft.com/security/blog/2022/08/24/magicweb-nobeliums-post-compromise-trick-to-authenticate-as-anyone/)
-    [Microsoft. (2023, January 26). Policy CSP - WindowsLogon. Retrieved March 30, 2023.](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-windowslogon)
-    [Microsoft. (2021, October 28). Store passwords using reversible encryption. Retrieved January 3, 2022.](https://docs.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/store-passwords-using-reversible-encryption)
-    [Microsoft. (2016, April 15). Attractive Accounts for Credential Theft. Retrieved June 3, 2016.](https://technet.microsoft.com/en-us/library/dn535501.aspx)
-    [Microsoft. (2016, April 16). Implementing Least-Privilege Administrative Models. Retrieved June 3, 2016.](https://technet.microsoft.com/en-us/library/dn487450.aspx)
-    [Plett, C., Poggemeyer, L. (12, October 26). Securing Privileged Access Reference Material. Retrieved April 25, 2017.](https://docs.microsoft.com/en-us/windows-server/identity/securing-privileged-access/securing-privileged-access-reference-material#a-nameesaebmaesae-administrative-forest-design-approach)
-    [Microsoft. (2013, July 31). Configuring Additional LSA Protection. Retrieved February 13, 2015.](https://technet.microsoft.com/en-us/library/dn408187.aspx)

×
