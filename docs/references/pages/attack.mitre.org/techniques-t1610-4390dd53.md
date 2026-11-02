---
url: "https://attack.mitre.org/techniques/T1610/"
title: "Deploy Container, Technique T1610 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T19:03:31Z"
via: "live"
sha256: "08b1fe97524ea3dcd4f4e06f90497f7a529237e13fed480920838a91001863e3"
wayback: "https://web.archive.org/web/20261023090731/https://attack.mitre.org/techniques/T1610/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Deploy Container, Technique T1610 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1610/>
> Snapshot: <https://web.archive.org/web/20261023090731/https://attack.mitre.org/techniques/T1610/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Deploy Container

#  Deploy Container

Adversaries may deploy a container into an environment to facilitate execution or evade defenses. In some cases, adversaries may deploy a new container to execute processes associated with a particular image or deployment, such as processes that execute or download malware. In others, an adversary may deploy a new container configured without network rules, user limitations, etc. to bypass existing defenses within the environment. In Kubernetes environments, an adversary may attempt to deploy a privileged or vulnerable container into a specific node in order to [Escape to Host](https://attack.mitre.org/techniques/T1611) and access other containers running on the node. [[1]](https://blog.appsecco.com/kubernetes-namespace-breakout-using-insecure-host-path-volume-part-1-b382f2a6e216)

Containers can be deployed by various means, such as via Docker's create and start APIs or via a web application such as the Kubernetes dashboard or Kubeflow. [[2]](https://docs.docker.com/reference/cli/docker/container/create/)[[3]](https://kubernetes.io/docs/tasks/access-application-cluster/web-ui-dashboard/)[[4]](https://www.kubeflow.org/docs/components/pipelines/overview/pipelines-overview/) In Kubernetes environments, containers may be deployed through workloads such as ReplicaSets or DaemonSets, which can allow containers to be deployed across multiple nodes.[[5]](https://kubernetes.io/docs/concepts/workloads/controllers/) Adversaries may deploy containers based on retrieved or built malicious images or from benign images that download and execute malicious payloads at runtime.[[6]](https://blog.aquasec.com/malicious-container-image-docker-container-host)

ID: T1610

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Execution](https://attack.mitre.org/tactics/TA0002)

ⓘ

Platforms: Containers

Contributors: Alfredo Oliveira, Trend Micro; Ariel Shuper, Cisco; Center for Threat-Informed Defense (CTID); Idan Frimark, Cisco; Joas Antonio dos Santos, @C0d3Cr4zy; Magno Logan, @magnologan, Trend Micro; Pawan Kinger, @kingerpawan, Trend Micro; Vishwas Manral, McAfee; Yossi Weizman, Azure Defender Research Team

Version: 2.0

Created: 29 March 2021

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1610/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1610/)

## Procedure Examples

| ID  | Name  | Description

|  [S0600](https://attack.mitre.org/software/S0600)   |  [Doki](https://attack.mitre.org/software/S0600)   |

[Doki](https://attack.mitre.org/software/S0600) was run through a deployed container.[[7]](https://www.intezer.com/blog/cloud-security/watch-your-containers-doki-infecting-docker-servers-in-the-cloud/)

|  [S0599](https://attack.mitre.org/software/S0599)   |  [Kinsing](https://attack.mitre.org/software/S0599)   |

[Kinsing](https://attack.mitre.org/software/S0599) was run through a deployed Ubuntu container.[[8]](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)

|  [S0683](https://attack.mitre.org/software/S0683)   |  [Peirates](https://attack.mitre.org/software/S0683)   |

[Peirates](https://attack.mitre.org/software/S0683) can deploy a pod that mounts its node’s root file system, then execute a command to create a reverse shell on the node.[[9]](https://github.com/inguardians/peirates)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has enabled Linux containers in victim network device environments using guestshell enable.[[10]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has deployed privileged DaemonSets to delete files on targeted Kubernetes clusters.[[11]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)

|  [S9041](https://attack.mitre.org/software/S9041)   |  [TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041)   |

[TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041) can deploy a pod in Kubernetes containers to run a script to establish persistence.[[12]](https://www.wiz.io/blog/teampcp-attack-kics-github-action)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has deployed different types of containers into victim environments to facilitate execution.[[13]](https://www.intezer.com/blog/cloud-security/attackers-abusing-legitimate-cloud-monitoring-tools-to-conduct-cyber-attacks/)[[14]](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf) [TeamTNT](https://attack.mitre.org/groups/G0139) has also transferred cryptocurrency mining software to Kubernetes clusters discovered within local IP address ranges.[[15]](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)

## Mitigations

| ID  | Mitigation  | Description

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Scan images before deployment, and block those that are not in compliance with security policies. In Kubernetes environments, the admission controller can be used to validate images after a container deployment request is authenticated but before the container is deployed.[[16]](https://media.defense.gov/2022/Aug/29/2003066362/-1/-1/0/CTR_KUBERNETES_HARDENING_GUIDANCE_1.2_20220829.PDF)

|  [M1035](https://attack.mitre.org/mitigations/M1035)   |  [Limit Access to Resource Over Network](https://attack.mitre.org/mitigations/M1035)   |

Limit communications with the container service to managed and secured channels, such as local Unix sockets or remote access via SSH. Require secure port access to communicate with the APIs over TLS by disabling unauthenticated access to the Docker API, Kubernetes API Server, and container orchestration web applications.[[17]](https://docs.docker.com/engine/security/protect-access/)[[18]](https://kubernetes.io/docs/concepts/security/controlling-access/) In Kubernetes clusters deployed in cloud environments, use native cloud platform features to restrict the IP ranges that are permitted to access to API server.[[19]](https://kubernetes.io/docs/concepts/security/overview/) Where possible, consider enabling just-in-time (JIT) access to the Kubernetes API to place additional restrictions on access.[[20]](https://learn.microsoft.com/en-us/azure/aks/managed-aad)

|  [M1030](https://attack.mitre.org/mitigations/M1030)   |  [Network Segmentation](https://attack.mitre.org/mitigations/M1030)   |

Deny direct remote access to internal systems through the use of network proxies, gateways, and firewalls.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Enforce the principle of least privilege by limiting container dashboard access to only the necessary users. When using Kubernetes, avoid giving users wildcard permissions or adding users to the system:masters group, and use RoleBindings rather than ClusterRoleBindings to limit user privileges to specific namespaces.[[21]](https://kubernetes.io/docs/concepts/security/rbac-good-practices/)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0249](https://attack.mitre.org/detectionstrategies/DET0249)   |  [Behavior-chain detection for T1610 Deploy Container across Docker & Kubernetes control/node planes](https://attack.mitre.org/detectionstrategies/DET0249)   |  [AN0693](https://attack.mitre.org/detectionstrategies/DET0249#AN0693)   |

Remote/API driven creation and start of a container whose image is not on an allow‑list (or is tagged latest), executed by a non-admin principal, and/or started with risky runtime attributes (e.g., --privileged, host PID/NET namespaces, sensitive host path mounts, capability adds). Correlates create ➜ start ➜ first network/process actions from that container within a short time window.

## References

-    [Abhisek Datta. (2020, March 18). Kubernetes Namespace Breakout using Insecure Host Path Volume — Part 1. Retrieved January 16, 2024.](https://blog.appsecco.com/kubernetes-namespace-breakout-using-insecure-host-path-volume-part-1-b382f2a6e216)
-    [DockerDocs. (n.d.). Retrieved December 8, 2025.](https://docs.docker.com/reference/cli/docker/container/create/)
-    [The Kubernetes Authors. (n.d.). Kubernetes Web UI (Dashboard). Retrieved March 29, 2021.](https://kubernetes.io/docs/tasks/access-application-cluster/web-ui-dashboard/)
-    [The Kubeflow Authors. (n.d.). Overview of Kubeflow Pipelines. Retrieved March 29, 2021.](https://www.kubeflow.org/docs/components/pipelines/overview/pipelines-overview/)
-    [Kubernetes. (n.d.). Workload Management. Retrieved March 28, 2024.](https://kubernetes.io/docs/concepts/workloads/controllers/)
-    [Assaf Morag. (2020, July 15). Threat Alert: Attackers Building Malicious Images on Your Hosts. Retrieved March 29, 2021.](https://blog.aquasec.com/malicious-container-image-docker-container-host)
-    [Fishbein, N., Kajiloti, M.. (2020, July 28). Watch Your Containers: Doki Infecting Docker Servers in the Cloud. Retrieved March 30, 2021.](https://www.intezer.com/blog/cloud-security/watch-your-containers-doki-infecting-docker-servers-in-the-cloud/)
-    [Singer, G. (2020, April 3). Threat Alert: Kinsing Malware Attacks Targeting Container Environments. Retrieved April 1, 2021.](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)
-    [InGuardians. (2022, January 5). Peirates GitHub. Retrieved February 8, 2022.](https://github.com/inguardians/peirates)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Unit 42. (2026, March 31). Weaponizing the Protectors: TeamPCP’s Multi-Stage Supply Chain Attack on Security Infrastructure. Retrieved July 1, 2026.](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)

-    [McCarthy, R., Haughom, J., Read, B. (2026, March 23). KICS GitHub Action Compromised: TeamPCP Strikes Again in Supply Chain Attack. Retrieved July 1, 2026.](https://www.wiz.io/blog/teampcp-attack-kics-github-action)
-    [Fishbein, N. (2020, September 8). Attackers Abusing Legitimate Cloud Monitoring Tools to Conduct Cyber Attacks. Retrieved September 22, 2021.](https://www.intezer.com/blog/cloud-security/attackers-abusing-legitimate-cloud-monitoring-tools-to-conduct-cyber-attacks/)
-    [Fiser, D. Oliveira, A. (n.d.). Tracking the Activities of TeamTNT A Closer Look at a Cloud-Focused Malicious Actor Group. Retrieved September 22, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)
-    [Darin Smith. (2022, April 21). TeamTNT targeting AWS, Alibaba. Retrieved August 4, 2022.](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)
-    [National Security Agency, Cybersecurity and Infrastructure Security Agency. (2022, March). Kubernetes Hardening Guide. Retrieved April 1, 2022.](https://media.defense.gov/2022/Aug/29/2003066362/-1/-1/0/CTR_KUBERNETES_HARDENING_GUIDANCE_1.2_20220829.PDF)
-    [Docker. (n.d.). Protect the Docker Daemon Socket. Retrieved March 29, 2021.](https://docs.docker.com/engine/security/protect-access/)
-    [The Kubernetes Authors. (n.d.). Controlling Access to The Kubernetes API. Retrieved March 29, 2021.](https://kubernetes.io/docs/concepts/security/controlling-access/)
-    [Kubernetes. (n.d.). Overview of Cloud Native Security. Retrieved March 8, 2023.](https://kubernetes.io/docs/concepts/security/overview/)
-    [Microsoft. (2023, February 27). AKS-managed Azure Active Directory integration. Retrieved March 8, 2023.](https://learn.microsoft.com/en-us/azure/aks/managed-aad)
-    [Kubernetes. (n.d.). Role Based Access Control Good Practices. Retrieved March 8, 2023.](https://kubernetes.io/docs/concepts/security/rbac-good-practices/)

×
