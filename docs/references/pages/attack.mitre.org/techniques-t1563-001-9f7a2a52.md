---
url: "https://attack.mitre.org/techniques/T1563/001/"
title: "Remote Service Session Hijacking: SSH Hijacking, Sub-technique T1563.001 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:57:13Z"
via: "live"
sha256: "901cf13468e597d81c7259905c30655acfb52f7838467ade6cdc0006db78acaf"
wayback: "https://web.archive.org/web/20261023090827/https://attack.mitre.org/techniques/T1563/001/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Remote Service Session Hijacking: SSH Hijacking, Sub-technique T1563.001 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1563/001/>
> Snapshot: <https://web.archive.org/web/20261023090827/https://attack.mitre.org/techniques/T1563/001/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Remote Service Session Hijacking](https://attack.mitre.org/techniques/T1563)
- SSH Hijacking

#  Remote Service Session Hijacking: SSH Hijacking

##### Other sub-techniques of Remote Service Session Hijacking (2)

| ID  | Name

|  T1563.001   |  SSH Hijacking

|  [T1563.002](https://attack.mitre.org/techniques/T1563/002/)   |  [RDP Hijacking](https://attack.mitre.org/techniques/T1563/002/)

Adversaries may hijack a legitimate user's SSH session to move laterally within an environment. Secure Shell (SSH) is a standard means of remote access on Linux and macOS systems. It allows a user to connect to another system via an encrypted tunnel, commonly authenticating through a password, certificate or the use of an asymmetric encryption key pair.

In order to move laterally from a compromised host, adversaries may take advantage of trust relationships established with other systems via public key authentication in active SSH sessions by hijacking an existing connection to another system. This may occur through compromising the SSH agent itself or by having access to the agent's socket. If an adversary is able to obtain root access, then hijacking SSH sessions is likely trivial.[[1]](https://www.slideshare.net/morisson/mistrusting-and-abusing-ssh-13526219)[[2]](https://www.blackhat.com/presentations/bh-usa-05/bh-us-05-boileau.pdf)[[3]](https://web.archive.org/web/20210311184303/https://www.clockwork.com/news/2012/09/28/602/ssh_agent_hijacking/)[[4]](https://matrix.org/blog/2019/05/08/post-mortem-and-remediations-for-apr-11-security-incident/)

[SSH Hijacking](https://attack.mitre.org/techniques/T1563/001) differs from use of [SSH](https://attack.mitre.org/techniques/T1021/004) because it hijacks an existing SSH session rather than creating a new session using [Valid Accounts](https://attack.mitre.org/techniques/T1078).

ID: T1563.001

Sub-technique of:  [T1563](https://attack.mitre.org/techniques/T1563)

ⓘ

Tactic: [Lateral Movement](https://attack.mitre.org/tactics/TA0008)

ⓘ

Platforms: Linux, macOS

Contributors: Anastasios Pingios

Version: 1.1

Created: 25 February 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1563/001/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1563/001/)

## Procedure Examples

| ID  | Name  | Description

|  [S1220](https://attack.mitre.org/software/S1220)   |  [MEDUSA](https://attack.mitre.org/software/S1220)   |

[MEDUSA](https://attack.mitre.org/software/S1220) can be configured to capture SSH credentials via SSH hijacking.[[5]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

## Mitigations

| ID  | Mitigation  | Description

|  [M1042](https://attack.mitre.org/mitigations/M1042)   |  [Disable or Remove Feature or Program](https://attack.mitre.org/mitigations/M1042)   |

Ensure that agent forwarding is disabled on systems that do not explicitly require this feature to prevent misuse. [[6]](https://www.symantec.com/connect/articles/ssh-and-ssh-agent)

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Ensure SSH key pairs have strong passwords and refrain from using key-store technologies such as ssh-agent unless they are properly protected.

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Do not allow remote access via SSH as root or other privileged accounts.

|  [M1022](https://attack.mitre.org/mitigations/M1022)   |  [Restrict File and Directory Permissions](https://attack.mitre.org/mitigations/M1022)   |

Ensure proper file permissions are set and harden system to prevent root privilege escalation opportunities.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0256](https://attack.mitre.org/detectionstrategies/DET0256)   |  [Detection Strategy for SSH Session Hijacking](https://attack.mitre.org/detectionstrategies/DET0256)   |  [AN0710](https://attack.mitre.org/detectionstrategies/DET0256#AN0710)   |

Suspicious reuse of SSH agent sockets across multiple users or processes, anomalous access to ~/.ssh/ or /tmp/ssh-* sockets, and abnormal patterns of lateral movement via SSH without new authentication events. Defender view: detect when one process accesses another user's SSH agent or when an existing SSH connection is used to pivot unexpectedly.

|  [AN0711](https://attack.mitre.org/detectionstrategies/DET0256#AN0711)   |

Unusual access to SSH agent sockets in /tmp/ or /private/tmp, process access to another user’s $SSH_AUTH_SOCK, and lateral SSH activity without corresponding login events. Defender view: correlation of socket access with anomalous network flows to internal systems.

## References

-    [Duarte, H., Morrison, B. (2012). (Mis)trusting and (ab)using ssh. Retrieved January 8, 2018.](https://www.slideshare.net/morisson/mistrusting-and-abusing-ssh-13526219)
-    [Adam Boileau. (2005, August 5). Trust Transience: Post Intrusion SSH Hijacking. Retrieved December 19, 2017.](https://www.blackhat.com/presentations/bh-usa-05/bh-us-05-boileau.pdf)
-    [Beuchler, B. (2012, September 28). SSH Agent Hijacking. Retrieved November 17, 2024.](https://web.archive.org/web/20210311184303/https://www.clockwork.com/news/2012/09/28/602/ssh_agent_hijacking/)

-    [Hodgson, M. (2019, May 8). Post-mortem and remediations for Apr 11 security incident. Retrieved November 17, 2024.](https://matrix.org/blog/2019/05/08/post-mortem-and-remediations-for-apr-11-security-incident/)
-    [Punsaen Boonyakarn, Shawn Chew, Logeswaran Nadarajan, Mathew Potaczek, Jakub Jozwiak, and Alex Marvi. (2024, June 18). Cloaked and Covert: Uncovering UNC3886 Espionage Operations. Retrieved September 24, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)
-    [Hatch, B. (2004, November 22). SSH and ssh-agent. Retrieved January 8, 2018.](https://www.symantec.com/connect/articles/ssh-and-ssh-agent)

×
