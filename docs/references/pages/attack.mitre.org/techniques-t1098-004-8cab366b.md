---
url: "https://attack.mitre.org/techniques/T1098/004/"
title: "Account Manipulation: SSH Authorized Keys, Sub-technique T1098.004 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:32:27Z"
via: "live"
sha256: "6fcfdd680a7a8da98464a4c000934177ea1045fdac440676a8b7fffb74ff60e8"
wayback: "https://web.archive.org/web/20261023090736/https://attack.mitre.org/techniques/T1098/004/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Account Manipulation: SSH Authorized Keys, Sub-technique T1098.004 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1098/004/>
> Snapshot: <https://web.archive.org/web/20261023090736/https://attack.mitre.org/techniques/T1098/004/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Account Manipulation](https://attack.mitre.org/techniques/T1098)
- SSH Authorized Keys

#  Account Manipulation: SSH Authorized Keys

##### Other sub-techniques of Account Manipulation (7)

| ID  | Name

|  [T1098.001](https://attack.mitre.org/techniques/T1098/001/)   |  [Additional Cloud Credentials](https://attack.mitre.org/techniques/T1098/001/)

|  [T1098.002](https://attack.mitre.org/techniques/T1098/002/)   |  [Additional Email Delegate Permissions](https://attack.mitre.org/techniques/T1098/002/)

|  [T1098.003](https://attack.mitre.org/techniques/T1098/003/)   |  [Additional Cloud Roles](https://attack.mitre.org/techniques/T1098/003/)

|  T1098.004   |  SSH Authorized Keys

|  [T1098.005](https://attack.mitre.org/techniques/T1098/005/)   |  [Device Registration](https://attack.mitre.org/techniques/T1098/005/)

|  [T1098.006](https://attack.mitre.org/techniques/T1098/006/)   |  [Additional Container Cluster Roles](https://attack.mitre.org/techniques/T1098/006/)

|  [T1098.007](https://attack.mitre.org/techniques/T1098/007/)   |  [Additional Local or Domain Groups](https://attack.mitre.org/techniques/T1098/007/)

Adversaries may modify the SSH authorized_keys file to maintain persistence on a victim host. Linux distributions, macOS, and ESXi hypervisors commonly use key-based authentication to secure the authentication process of SSH sessions for remote management. The authorized_keys file in SSH specifies the SSH keys that can be used for logging into the user account for which the file is configured. This file is usually found in the user's home directory under <user-home>/.ssh/authorized_keys (or, on ESXi, /etc/ssh/keys-<username>/authorized_keys).[[1]](https://www.ssh.com/ssh/authorized_keys/) Users may edit the system’s SSH config file to modify the directives PubkeyAuthentication and RSAAuthentication to the value yes to ensure public key and RSA authentication are enabled, as well as modify the directive PermitRootLogin to the value yes to enable root authentication via SSH.[[2]](https://knowledge.broadcom.com/external/article/313767/allowing-ssh-access-to-vmware-vsphere-es.html) The SSH config file is usually located under /etc/ssh/sshd_config.

Adversaries may modify SSH authorized_keys files directly with scripts or shell commands to add their own adversary-supplied public keys. In cloud environments, adversaries may be able to modify the SSH authorized_keys file of a particular virtual machine via the command line interface or rest API. For example, by using the Google Cloud CLI’s "add-metadata" command an adversary may add SSH keys to a user account.[[3]](https://cloud.google.com/sdk/gcloud/reference/compute/instances/add-metadata)[[4]](https://about.gitlab.com/blog/2020/02/12/plundering-gcp-escalating-privileges-in-google-cloud-platform/) Similarly, in Azure, an adversary may update the authorized_keys file of a virtual machine via a PATCH request to the API.[[5]](https://docs.microsoft.com/en-us/rest/api/compute/virtual-machines/update) This ensures that an adversary possessing the corresponding private key may log in as an existing user via SSH.[[6]](https://www.venafi.com/blog/growing-abuse-ssh-keys-commodity-malware-campaigns-now-equipped-ssh-capabilities)[[7]](https://www.cybereason.com/blog/new-pervasive-worm-exploiting-linux-exim-server-vulnerability) It may also lead to privilege escalation where the virtual machine or instance has distinct permissions from the requesting user.

Where authorized_keys files are modified via cloud APIs or command line interfaces, an adversary may achieve privilege escalation on the target virtual machine if they add a key to a higher-privileged user.

SSH keys can also be added to accounts on network devices, such as with the ip ssh pubkey-chain [Network Device CLI](https://attack.mitre.org/techniques/T1059/008) command.[[8]](https://www.cisco.com/c/en/us/td/docs/ios-xml/ios/security/d1/sec-d1-cr-book/sec-cr-i3.html#wp1254331478)

ID: T1098.004

Sub-technique of:  [T1098](https://attack.mitre.org/techniques/T1098)

ⓘ

Tactics: [Persistence](https://attack.mitre.org/tactics/TA0003), [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)

ⓘ

Platforms: ESXi, IaaS, Linux, Network Devices, macOS

Contributors: Arad Inbar, Fidelis Security; Austin Clark, @c2defense; Dror Alon, Palo Alto Networks; Or Kliger, Palo Alto Networks; Tony Lambert, Red Canary

Version: 1.4

Created: 24 June 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1098/004/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1098/004/)

## Procedure Examples

| ID  | Name  | Description

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary tasked Claude Code to establish persistence on the system by injecting an SSH key located at /root/.ssh/authorized_keys file via a crontab script.[[9]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [S0482](https://attack.mitre.org/software/S0482)   |  [Bundlore](https://attack.mitre.org/software/S0482)   |

[Bundlore](https://attack.mitre.org/software/S0482) creates a new key pair with ssh-keygen and drops the newly created user key in authorized_keys to enable remote login.[[10]](https://mackeeper.com/blog/post/610-macos-bundlore-adware-analysis/)

|  [G1006](https://attack.mitre.org/groups/G1006)   |  [Earth Lusca](https://attack.mitre.org/groups/G1006)   |

[Earth Lusca](https://attack.mitre.org/groups/G1006) has dropped an SSH-authorized key in the /root/.ssh folder in order to access a compromised server with SSH.[[11]](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)

|  [C0061](https://attack.mitre.org/campaigns/C0061)   |  [Operation Digital Eye](https://attack.mitre.org/campaigns/C0061)   |

During [Operation Digital Eye](https://attack.mitre.org/campaigns/C0061), threat actors used SSH access enabled by authorized_keys files for remote execution.[[12]](https://www.sentinelone.com/labs/operation-digital-eye-chinese-apt-compromises-critical-digital-infrastructure-via-visual-studio-code-tunnels/)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has added SSH authorized_keys under root or other users at the Linux level on compromised network devices.[[13]](https://blog.talosintelligence.com/salt-typhoon-analysis/)[[14]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  [S0468](https://attack.mitre.org/software/S0468)   |  [Skidmap](https://attack.mitre.org/software/S0468)   |

[Skidmap](https://attack.mitre.org/software/S0468) has the ability to add the public key of its handlers to the authorized_keys file to maintain persistence on an infected host.[[15]](https://blog.trendmicro.com/trendlabs-security-intelligence/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload/)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has added RSA keys in authorized_keys.[[16]](https://blog.aquasec.com/container-security-tnt-container-attack)[[17]](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

[XCSSET](https://attack.mitre.org/software/S0658) will create an ssh key if necessary with the ssh-keygen -t rsa -f $HOME/.ssh/id_rsa -P command. [XCSSET](https://attack.mitre.org/software/S0658) will upload a private key file to the server to remotely access the host without a password.[[18]](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)

## Mitigations

| ID  | Mitigation  | Description

|  [M1042](https://attack.mitre.org/mitigations/M1042)   |  [Disable or Remove Feature or Program](https://attack.mitre.org/mitigations/M1042)   |

Disable SSH if it is not necessary on a host or restrict SSH access for specific users/groups using /etc/ssh/sshd_config. Setting the PermitRootLogin directive to no will prevent the root user from logging in via SSH.[[2]](https://knowledge.broadcom.com/external/article/313767/allowing-ssh-access-to-vmware-vsphere-es.html)

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Restrict access to the authorized_keys file.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

In cloud environments, ensure that only users who explicitly require the permissions to update instance metadata or configurations can do so.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0126](https://attack.mitre.org/detectionstrategies/DET0126)   |  [Detection Strategy for SSH Key Injection in Authorized Keys](https://attack.mitre.org/detectionstrategies/DET0126)   |  [AN0350](https://attack.mitre.org/detectionstrategies/DET0126#AN0350)   |

Adversary attempts to gain persistence by modifying ~/.ssh/authorized_keys via shell, text editor, echo or redirected output.

|  [AN0351](https://attack.mitre.org/detectionstrategies/DET0126#AN0351)   |

Insertion of public keys into authorized_keys using bash/zsh or editor tools, correlated with suspicious process ancestry.

|  [AN0352](https://attack.mitre.org/detectionstrategies/DET0126#AN0352)   |

Abuse of cloud metadata APIs or CLI to push SSH public keys to authorized_keys of virtual machines.

|  [AN0353](https://attack.mitre.org/detectionstrategies/DET0126#AN0353)   |

Direct modification of /etc/ssh/keys-/authorized_keys or enabling SSH in sshd_config to support public key auth.

|  [AN0354](https://attack.mitre.org/detectionstrategies/DET0126#AN0354)   |

Use of command-line like ip ssh pubkey-chain to bind SSH keys to privileged accounts on routers or switches.

## References

-    [ssh.com. (n.d.). Authorized_keys File in SSH. Retrieved June 24, 2020.](https://www.ssh.com/ssh/authorized_keys/)
-    [Broadcom. (2024, December 12). Allowing SSH access to VMware vSphere ESXi/ESX hosts with public/private key authentication. Retrieved March 26, 2025.](https://knowledge.broadcom.com/external/article/313767/allowing-ssh-access-to-vmware-vsphere-es.html)
-    [Google Cloud. (2022, March 31). gcloud compute instances add-metadata. Retrieved April 1, 2022.](https://cloud.google.com/sdk/gcloud/reference/compute/instances/add-metadata)
-    [Chris Moberly. (2020, February 12). Tutorial on privilege escalation and post exploitation tactics in Google Cloud Platform environments. Retrieved April 1, 2022.](https://about.gitlab.com/blog/2020/02/12/plundering-gcp-escalating-privileges-in-google-cloud-platform/)
-    [Microsoft. (n.d.). Virtual Machines - Update. Retrieved April 1, 2022.](https://docs.microsoft.com/en-us/rest/api/compute/virtual-machines/update)
-    [Blachman, Y. (2020, April 22). Growing Abuse of SSH Keys: Commodity Malware Campaigns Now Equipped with SSH Capabilities. Retrieved June 24, 2020.](https://www.venafi.com/blog/growing-abuse-ssh-keys-commodity-malware-campaigns-now-equipped-ssh-capabilities)
-    [Cybereason Nocturnus. (2019, June 13). New Pervasive Worm Exploiting Linux Exim Server Vulnerability. Retrieved June 24, 2020.](https://www.cybereason.com/blog/new-pervasive-worm-exploiting-linux-exim-server-vulnerability)
-    [Cisco. (2021, August 23). ip ssh pubkey-chain. Retrieved July 13, 2022.](https://www.cisco.com/c/en/us/td/docs/ios-xml/ios/security/d1/sec-d1-cr-book/sec-cr-i3.html#wp1254331478)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

-    [Sushko, O. (2019, April 17). macOS Bundlore: Mac Virus Bypassing macOS Security Features. Retrieved June 30, 2020.](https://mackeeper.com/blog/post/610-macos-bundlore-adware-analysis/)
-    [Chen, J., et al. (2022). Delving Deep: An Analysis of Earth Lusca’s Operations. Retrieved July 1, 2022.](https://www.trendmicro.com/content/dam/trendmicro/global/en/research/22/a/earth-lusca-employs-sophisticated-infrastructure-varied-tools-and-techniques/technical-brief-delving-deep-an-analysis-of-earth-lusca-operations.pdf)
-    [Aleksandar Milenkoski, Luigi Martire. (2024, December 10). Operation Digital Eye | Chinese APT Compromises Critical Digital Infrastructure via Visual Studio Code Tunnels. Retrieved February 27, 2025.](https://www.sentinelone.com/labs/operation-digital-eye-chinese-apt-compromises-critical-digital-infrastructure-via-visual-studio-code-tunnels/)
-    [Cisco Talos. (2025, February 20). Weathering the storm: In the midst of a Typhoon. Retrieved February 24, 2025.](https://blog.talosintelligence.com/salt-typhoon-analysis/)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Remillano, A., Urbanec, J. (2019, September 19). Skidmap Linux Malware Uses Rootkit Capabilities to Hide Cryptocurrency-Mining Payload. Retrieved June 4, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload/)
-    [Kol, Roi. Morag, A. (2020, August 25). Deep Analysis of TeamTNT Techniques Using Container Images to Attack. Retrieved September 22, 2021.](https://blog.aquasec.com/container-security-tnt-container-attack)
-    [Darin Smith. (2022, April 21). TeamTNT targeting AWS, Alibaba. Retrieved August 4, 2022.](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)
-    [Mac Threat Response, Mobile Research Team. (2020, August 13). The XCSSET Malware: Inserts Malicious Code Into Xcode Projects, Performs UXSS Backdoor Planting in Safari, and Leverages Two Zero-day Exploits. Retrieved October 5, 2021.](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)

×
