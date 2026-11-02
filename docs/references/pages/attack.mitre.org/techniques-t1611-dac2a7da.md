---
url: "https://attack.mitre.org/techniques/T1611/"
title: "Escape to Host, Technique T1611 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T19:03:36Z"
via: "live"
sha256: "9576d0db6bbe7568589759c446f1d8efe58c9a7d70cffb09144c75878d6b9859"
wayback: "https://web.archive.org/web/20261023005807/https://attack.mitre.org/techniques/T1611/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Escape to Host, Technique T1611 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1611/>
> Snapshot: <https://web.archive.org/web/20261023005807/https://attack.mitre.org/techniques/T1611/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Escape to Host

#  Escape to Host

Adversaries may break out of a container or virtualized environment to gain access to the underlying host. This can allow an adversary access to other containerized or virtualized resources from the host level or to the host itself. In principle, containerized / virtualized resources should provide a clear separation of application functionality and be isolated from the host environment.[[1]](https://docs.docker.com/get-started/overview/)

There are multiple ways an adversary may escape from a container to a host environment. Examples include creating a container configured to mount the host’s filesystem using the bind parameter, which allows the adversary to drop payloads and execute control utilities such as cron on the host; utilizing a privileged container to run commands or load a malicious kernel module on the underlying host; or abusing system calls such as unshare and keyctl to escalate privileges and steal secrets.[[2]](https://docs.docker.com/storage/bind-mounts/)[[3]](https://www.trendmicro.com/en_us/research/19/l/why-running-a-privileged-container-in-docker-is-a-bad-idea.html)[[4]](https://www.intezer.com/blog/cloud-security/watch-your-containers-doki-infecting-docker-servers-in-the-cloud/)[[5]](https://0xn3va.gitbook.io/cheat-sheets/container/escaping)[[6]](https://www.crowdstrike.com/blog/cve-2022-0185-kubernetes-container-escape-using-linux-kernel-exploit/)[[7]](https://www.antitree.com/2020/07/keyctl-unmask-going-florida-on-the-state-of-containerizing-linux-keyrings/)

Additionally, an adversary may be able to exploit a compromised container with a mounted container management socket, such as docker.sock, to break out of the container via a [Container Administration Command](https://attack.mitre.org/techniques/T1609).[[5]](https://0xn3va.gitbook.io/cheat-sheets/container/escaping) Adversaries may also escape via [Exploitation for Privilege Escalation](https://attack.mitre.org/techniques/T1068), such as exploiting vulnerabilities in global symbolic links in order to access the root directory of a host machine.[[8]](https://unit42.paloaltonetworks.com/windows-server-containers-vulnerabilities/)

In ESXi environments, an adversary may exploit a vulnerability in order to escape from a virtual machine into the hypervisor.[[9]](https://github.com/vmware/vcf-security-and-compliance-guidelines/tree/main/security-advisories/vmsa-2025-0004)

Gaining access to the host may provide the adversary with the opportunity to achieve follow-on objectives, such as establishing persistence, moving laterally within the environment, accessing other containers or virtual machines running on the host, or setting up a command and control channel on the host.

ID: T1611

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)

ⓘ

Platforms: Containers, ESXi, Linux, Windows

Contributors: Alfredo Oliveira, Trend Micro; Ariel Shuper, Cisco; CrowdStrike; Daniel Prizmant, Palo Alto Networks; David Fiser, @anu4is, Trend Micro; Eran Ayalon, Cybereason; Idan Frimark, Cisco; Ilan Sokol, Cybereason; Joas Antonio dos Santos, @C0d3Cr4zy; Magno Logan, @magnologan, Trend Micro; Oren Ofer, Cybereason; Vishwas Manral, McAfee; Yossi Weizman, Azure Defender Research Team; Yuval Avrahami, Palo Alto Networks

Version: 1.6

Created: 30 March 2021

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1611/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1611/)

## Procedure Examples

| ID  | Name  | Description

|  [S0600](https://attack.mitre.org/software/S0600)   |  [Doki](https://attack.mitre.org/software/S0600)   |

[Doki](https://attack.mitre.org/software/S0600)’s container was configured to bind the host root directory.[[4]](https://www.intezer.com/blog/cloud-security/watch-your-containers-doki-infecting-docker-servers-in-the-cloud/)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has used the BOtB tool that can break out of containers. [[10]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [S0683](https://attack.mitre.org/software/S0683)   |  [Peirates](https://attack.mitre.org/software/S0683)   |

[Peirates](https://attack.mitre.org/software/S0683) can gain a reverse shell on a host node by mounting the Kubernetes hostPath.[[11]](https://github.com/inguardians/peirates)

|  [S0623](https://attack.mitre.org/software/S0623)   |  [Siloscape](https://attack.mitre.org/software/S0623)   |

[Siloscape](https://attack.mitre.org/software/S0623) maps the host’s C drive to the container by creating a global symbolic link to the host through the calling of NtSetInformationSymbolicLink.[[12]](https://unit42.paloaltonetworks.com/siloscape/)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has deployed privileged containers that mount the filesystem of victim machine.[[13]](https://www.intezer.com/blog/cloud-security/attackers-abusing-legitimate-cloud-monitoring-tools-to-conduct-cyber-attacks/)[[14]](https://blog.aquasec.com/container-security-tnt-container-attack)

## Mitigations

| ID  | Mitigation  | Description

|  [M1048](https://attack.mitre.org/mitigations/M1048)   |  [Application Isolation and Sandboxing](https://attack.mitre.org/mitigations/M1048)   |

Consider utilizing seccomp, seccomp-bpf, or a similar solution that restricts certain system calls such as mount. In Kubernetes environments, consider defining Pod Security Standards that limit container access to host process namespaces, the host network, and the host file system.[[15]](https://media.defense.gov/2022/Aug/29/2003066362/-1/-1/0/CTR_KUBERNETES_HARDENING_GUIDANCE_1.2_20220829.PDF)

|  [M1042](https://attack.mitre.org/mitigations/M1042)   |  [Disable or Remove Feature or Program](https://attack.mitre.org/mitigations/M1042)   |

Remove unnecessary tools and software from containers.

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

Use read-only containers, read-only file systems, and minimal images when possible to prevent the running of commands.[[15]](https://media.defense.gov/2022/Aug/29/2003066362/-1/-1/0/CTR_KUBERNETES_HARDENING_GUIDANCE_1.2_20220829.PDF) Where possible, also consider using application control and software restriction tools (such as those provided by SELinux) to restrict access to files, processes, and system calls in containers.[[16]](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/)

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Ensure containers are not running as root by default and do not use unnecessary privileges or mounted components. In Kubernetes environments, consider defining Pod Security Standards that prevent pods from running privileged containers.[[15]](https://media.defense.gov/2022/Aug/29/2003066362/-1/-1/0/CTR_KUBERNETES_HARDENING_GUIDANCE_1.2_20220829.PDF)

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Ensure that hosts are kept up-to-date with security patches.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0219](https://attack.mitre.org/detectionstrategies/DET0219)   |  [Detection Strategy for Escape to Host](https://attack.mitre.org/detectionstrategies/DET0219)   |  [AN0612](https://attack.mitre.org/detectionstrategies/DET0219#AN0612)   |

Detection of container escape attempts via bind mounts, privileged containers, or abuse of docker.sock. Defenders may observe anomalous volume mount configurations (e.g., hostPath to / or /proc), unexpected privileged container launches, or use of container administration commands to access host resources. These events typically correlate with subsequent process execution on the host outside of normal container isolation.

|  [AN0613](https://attack.mitre.org/detectionstrategies/DET0219#AN0613)   |

Detection of Linux container escape attempts via syscalls (unshare, keyctl, mount) or process execution outside container namespaces. Defenders may correlate unusual system calls from containerized processes with subsequent process creation on the host or modification of host resources.

|  [AN0614](https://attack.mitre.org/detectionstrategies/DET0219#AN0614)   |

Detection of Windows container escape attempts by observing processes accessing host directories, symbolic link abuse, or privilege escalation attempts. Defenders may detect anomalous process execution with access to system-level directories outside of container boundaries.

|  [AN0615](https://attack.mitre.org/detectionstrategies/DET0219#AN0615)   |

Detection of ESXi escape attempts by monitoring for anomalies in hypervisor logs such as unexpected VM operations, privilege escalation events, or attempts to load malicious kernel modules within the hypervisor environment.

## References

-    [Docker. (n.d.). Docker Overview. Retrieved March 30, 2021.](https://docs.docker.com/get-started/overview/)
-    [Docker. (n.d.). Use Bind Mounts. Retrieved March 30, 2021.](https://docs.docker.com/storage/bind-mounts/)
-    [Fiser, D., Oliveira, A.. (2019, December 20). Why a Privileged Container in Docker is a Bad Idea. Retrieved March 30, 2021.](https://www.trendmicro.com/en_us/research/19/l/why-running-a-privileged-container-in-docker-is-a-bad-idea.html)
-    [Fishbein, N., Kajiloti, M.. (2020, July 28). Watch Your Containers: Doki Infecting Docker Servers in the Cloud. Retrieved March 30, 2021.](https://www.intezer.com/blog/cloud-security/watch-your-containers-doki-infecting-docker-servers-in-the-cloud/)
-    [0xn3va. (n.d.). Escaping. Retrieved May 27, 2022.](https://0xn3va.gitbook.io/cheat-sheets/container/escaping)
-    [Manoj Ahuje. (2022, January 31). CVE-2022-0185: Kubernetes Container Escape Using Linux Kernel Exploit. Retrieved July 6, 2022.](https://www.crowdstrike.com/blog/cve-2022-0185-kubernetes-container-escape-using-linux-kernel-exploit/)
-    [Mark Manning. (2020, July 23). Keyctl-unmask: "Going Florida" on The State Of Containerizing Linux Keyrings. Retrieved July 6, 2022.](https://www.antitree.com/2020/07/keyctl-unmask-going-florida-on-the-state-of-containerizing-linux-keyrings/)
-    [Daniel Prizmant. (2020, July 15). Windows Server Containers Are Open, and Here's How You Can Break Out. Retrieved October 1, 2021.](https://unit42.paloaltonetworks.com/windows-server-containers-vulnerabilities/)

-    [Broadcom. (2025, March 6). VMSA-2025-0004: Questions & Answers. Retrieved March 26, 2025.](https://github.com/vmware/vcf-security-and-compliance-guidelines/tree/main/security-advisories/vmsa-2025-0004)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [InGuardians. (2022, January 5). Peirates GitHub. Retrieved February 8, 2022.](https://github.com/inguardians/peirates)
-    [Prizmant, D. (2021, June 7). Siloscape: First Known Malware Targeting Windows Containers to Compromise Cloud Environments. Retrieved June 9, 2021.](https://unit42.paloaltonetworks.com/siloscape/)
-    [Fishbein, N. (2020, September 8). Attackers Abusing Legitimate Cloud Monitoring Tools to Conduct Cyber Attacks. Retrieved September 22, 2021.](https://www.intezer.com/blog/cloud-security/attackers-abusing-legitimate-cloud-monitoring-tools-to-conduct-cyber-attacks/)
-    [Kol, Roi. Morag, A. (2020, August 25). Deep Analysis of TeamTNT Techniques Using Container Images to Attack. Retrieved September 22, 2021.](https://blog.aquasec.com/container-security-tnt-container-attack)
-    [National Security Agency, Cybersecurity and Infrastructure Security Agency. (2022, March). Kubernetes Hardening Guide. Retrieved April 1, 2022.](https://media.defense.gov/2022/Aug/29/2003066362/-1/-1/0/CTR_KUBERNETES_HARDENING_GUIDANCE_1.2_20220829.PDF)
-    [Kubernetes. (n.d.). Configure a Security Context for a Pod or Container. Retrieved March 8, 2023.](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/)

×
