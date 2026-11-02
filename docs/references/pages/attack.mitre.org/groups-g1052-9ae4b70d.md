---
url: "https://attack.mitre.org/groups/G1052/"
title: "Contagious Interview, DeceptiveDevelopment, Gwisin Gang, Tenacious Pungsan, DEV#POPPER, PurpleBravo, TAG-121, Group G1052 | MITRE ATT&CK®"
captured: "2026-11-02T18:22:45Z"
via: "live"
sha256: "14c4a9bd8995cd1437fba8a8957f698ee99ec7102b97c0d83f9d23360798f354"
wayback: "https://web.archive.org/web/20261023114559/https://attack.mitre.org/groups/G1052/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Contagious Interview, DeceptiveDevelopment, Gwisin Gang, Tenacious Pungsan, DEV#POPPER, PurpleBravo, TAG-121, Group G1052 | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/groups/G1052/>
> Snapshot: <https://web.archive.org/web/20261023114559/https://attack.mitre.org/groups/G1052/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Groups](https://attack.mitre.org/groups/)
- Contagious Interview

#  Contagious Interview

[Contagious Interview](https://attack.mitre.org/groups/G1052) is a North Korea–aligned threat group active since 2023. The group conducts both cyberespionage and financially motivated operations, including the theft of cryptocurrency and user credentials. [Contagious Interview](https://attack.mitre.org/groups/G1052) targets Windows, Linux, and macOS systems, with a particular focus on individuals engaged in software development and cryptocurrency-related activities. [[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[3]](https://securitylabs.datadoghq.com/articles/tenacious-pungsan-dprk-threat-actor-contagious-interview/)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

ID: G1052

ⓘ

Associated Groups: DeceptiveDevelopment, Gwisin Gang, Tenacious Pungsan, DEV#POPPER, PurpleBravo, TAG-121

Version: 1.0

Created: 19 October 2025

Last Modified: 31 July 2026

[Version Permalink](https://attack.mitre.org/versions/v20/groups/G1052/)

[Live Version](https://attack.mitre.org/versions/v20/groups/G1052/)

## Associated Group Descriptions

| Name  | Description

|  DeceptiveDevelopment   |

[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  Gwisin Gang   |

[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)

|  Tenacious Pungsan   |

[[3]](https://securitylabs.datadoghq.com/articles/tenacious-pungsan-dprk-threat-actor-contagious-interview/)

|  DEV#POPPER   |

[[11]](https://www.securonix.com/blog/analysis-of-devpopper-new-attack-campaign-targeting-software-developers-likely-associated-with-north-korean-threat-actors/)

|  PurpleBravo   |

[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|  TAG-121   |

[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

###### Enterprise Layer

[download](https://attack.mitre.org/groups/G1052/G1052-enterprise-layer.json)  [view](https://attack.mitre.org/groups/G1052/#)

## Techniques Used

| Domain  | ID  | Name  | Use

|  Enterprise   |  [T1583](https://attack.mitre.org/techniques/T1583)   |  [Acquire Infrastructure](https://attack.mitre.org/techniques/T1583)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has used services such as Astrill VPN.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|   |   |  [.001](https://attack.mitre.org/techniques/T1583/001)   |  [Domains](https://attack.mitre.org/techniques/T1583/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has registered domains to leverage in their social engineering campaigns.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also registered domains to utilize for C2.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)[[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)

|   |   |  [.003](https://attack.mitre.org/techniques/T1583/003)   |  [Virtual Private Server](https://attack.mitre.org/techniques/T1583/003)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has acquired virtual private servers from services such as Stark Industries Solutions and RouterHosting.[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also utilized hosting providers to include Tier[.]Net, Majestic Hosting, Leaseweb Singapore, and Kaopu Cloud.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|   |   |  [.006](https://attack.mitre.org/techniques/T1583/006)   |  [Web Services](https://attack.mitre.org/techniques/T1583/006)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has used web services such as Dropbox to receive stolen data and Google Drive, Firebase, GitHub, and Telegram to disseminate files.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also used a cloud platform such as Vercel for C2 operations leveraging malicious web applications and static pages.[[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also used Slack to coordinate their activities.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)

|  Enterprise   |  [T1071](https://attack.mitre.org/techniques/T1071)   |  [.003](https://attack.mitre.org/techniques/T1071/003)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Mail Protocols](https://attack.mitre.org/techniques/T1071/003)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has utilized email notifications from malware distribution servers to track victim engagement.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)

|  Enterprise   |  [T1547](https://attack.mitre.org/techniques/T1547)   |  [.001](https://attack.mitre.org/techniques/T1547/001)   |  [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547): [Registry Run Keys / Startup Folder](https://attack.mitre.org/techniques/T1547/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has established persistence using [InvisibleFerret](https://attack.mitre.org/software/S1245) malware to place a .bat file in the Startup Folder.[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|   |   |  [.013](https://attack.mitre.org/techniques/T1547/013)   |  [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547): [XDG Autostart Entries](https://attack.mitre.org/techniques/T1547/013)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has established persistence using [InvisibleFerret](https://attack.mitre.org/software/S1245) malware to create a .desktop entry to run on startup on GNOME-based Linux devices.[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1059](https://attack.mitre.org/techniques/T1059)   |  [.003](https://attack.mitre.org/techniques/T1059/003)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Windows Command Shell](https://attack.mitre.org/techniques/T1059/003)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has utilized VBS scripts to open cmd.exe and run commands to include the go_batch.bat batch file.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|   |   |  [.004](https://attack.mitre.org/techniques/T1059/004)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Unix Shell](https://attack.mitre.org/techniques/T1059/004)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has targeted macOS victim hosts using a bash downloader coremedia.sh and a bash script cloud.sh.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|   |   |  [.005](https://attack.mitre.org/techniques/T1059/005)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Visual Basic](https://attack.mitre.org/techniques/T1059/005)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has utilized Visual Basic scripts in the execution of their downloader malware targeting Windows devices including as script called update.vbs.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|   |   |  [.006](https://attack.mitre.org/techniques/T1059/006)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Python](https://attack.mitre.org/techniques/T1059/006)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has used the Python-based malware such as [InvisibleFerret](https://attack.mitre.org/software/S1245) to install and execute Python Packages and Python modules.[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|   |   |  [.007](https://attack.mitre.org/techniques/T1059/007)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [JavaScript](https://attack.mitre.org/techniques/T1059/007)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has leveraged JavaScript in the execution of their downloader malware targeting Windows devices using a NodeJS script titled nvidia.js.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  Enterprise   |  [T1543](https://attack.mitre.org/techniques/T1543)   |  [.001](https://attack.mitre.org/techniques/T1543/001)   |  [Create or Modify System Process](https://attack.mitre.org/techniques/T1543): [Launch Agent](https://attack.mitre.org/techniques/T1543/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has established persistence using [InvisibleFerret](https://attack.mitre.org/software/S1245) malware to create file to run the script on Startup via LaunchAgents.[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also utilized a plist file located in /Library/LaunchAgents to enable a malicious bash script the ability to persist.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  Enterprise   |  [T1555](https://attack.mitre.org/techniques/T1555)   |  [.001](https://attack.mitre.org/techniques/T1555/001)   |  [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Keychain](https://attack.mitre.org/techniques/T1555/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has leveraged malware variants configured to dump credentials from the macOS keychain.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)

|  Enterprise   |  [T1587](https://attack.mitre.org/techniques/T1587)   |  [Develop Capabilities](https://attack.mitre.org/techniques/T1587)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) developed malicious NPM packages for delivery to or retrieval by victims.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|   |   |  [.001](https://attack.mitre.org/techniques/T1587/001)   |  [Malware](https://attack.mitre.org/techniques/T1587/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has developed malware that utilizes Qt cross-platform framework to include [BeaverTail](https://attack.mitre.org/software/S1246).[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|  Enterprise   |  [T1685](https://attack.mitre.org/techniques/T1685)   |  [Disable or Modify Tools](https://attack.mitre.org/techniques/T1685)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has convinced victims to disable Docker and other container environments and run code on their machine natively in attempts to bypass container isolation and ensure device infection.[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)

|  Enterprise   |  [T1573](https://attack.mitre.org/techniques/T1573)   |  [.001](https://attack.mitre.org/techniques/T1573/001)   |  [Encrypted Channel](https://attack.mitre.org/techniques/T1573): [Symmetric Cryptography](https://attack.mitre.org/techniques/T1573/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has encrypted C2 traffic using RC4.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  Enterprise   |  [T1585](https://attack.mitre.org/techniques/T1585)   |  [Establish Accounts](https://attack.mitre.org/techniques/T1585)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has created and maintained personas on code repositories to distribute malicious payloads.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|   |   |  [.001](https://attack.mitre.org/techniques/T1585/001)   |  [Social Media Accounts](https://attack.mitre.org/techniques/T1585/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has created fake social media accounts such as LinkedIn and Telegram accounts for their targeting efforts.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[17]](https://securityscorecard.com/blog/the-job-offer-that-wasnt-how-we-stopped-an-espionage-plot/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|   |   |  [.002](https://attack.mitre.org/techniques/T1585/002)   |  [Email Accounts](https://attack.mitre.org/techniques/T1585/002)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has created fake email accounts to correspond with social media accounts, fake LinkedIn personas, code repository accounts, and job announcements on development job board services.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also utilized fake email accounts with Threat Intelligence vendor services.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)

|  Enterprise   |  [T1546](https://attack.mitre.org/techniques/T1546)   |  [.004](https://attack.mitre.org/techniques/T1546/004)   |  [Event Triggered Execution](https://attack.mitre.org/techniques/T1546): [Unix Shell Configuration Modification](https://attack.mitre.org/techniques/T1546/004)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has targeted macOS victim hosts using a bash downloader coremedia.sh and a bash script cloud.sh.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  Enterprise   |  [T1480](https://attack.mitre.org/techniques/T1480)   |  [Execution Guardrails](https://attack.mitre.org/techniques/T1480)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has configured C2 endpoints to review IP geolocation, request headers, victim environment details and runtime conditions prior to delivering payloads.[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)

|  Enterprise   |  [T1048](https://attack.mitre.org/techniques/T1048)   |  [.003](https://attack.mitre.org/techniques/T1048/003)   |  [Exfiltration Over Alternative Protocol](https://attack.mitre.org/techniques/T1048): [Exfiltration Over Unencrypted Non-C2 Protocol](https://attack.mitre.org/techniques/T1048/003)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has exfiltrated victim information using FTP.[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|  Enterprise   |  [T1041](https://attack.mitre.org/techniques/T1041)   |  [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has exfiltrated data from a compromised host to actor-controlled C2 servers.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|  Enterprise   |  [T1567](https://attack.mitre.org/techniques/T1567)   |  [Exfiltration Over Web Service](https://attack.mitre.org/techniques/T1567)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has leveraged Telegram API to exfiltrate stolen data.[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|   |   |  [.002](https://attack.mitre.org/techniques/T1567/002)   |  [Exfiltration to Cloud Storage](https://attack.mitre.org/techniques/T1567/002)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has exfiltrated stolen passwords to Dropbox.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  Enterprise   |  [T1083](https://attack.mitre.org/techniques/T1083)   |  [File and Directory Discovery](https://attack.mitre.org/techniques/T1083)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has conducted key word searches within files and directories on a compromised hosts to identify files for exfiltration.[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1657](https://attack.mitre.org/techniques/T1657)   |  [Financial Theft](https://attack.mitre.org/techniques/T1657)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has stolen cryptocurrency wallet credentials and credit card information utilizing [BeaverTail](https://attack.mitre.org/software/S1246) and [InvisibleFerret](https://attack.mitre.org/software/S1245) malware.[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|  Enterprise   |  [T1589](https://attack.mitre.org/techniques/T1589)   |  [Gather Victim Identity Information](https://attack.mitre.org/techniques/T1589)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has researched specific professional groups such as software developers for targeting.[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[11]](https://www.securonix.com/blog/analysis-of-devpopper-new-attack-campaign-targeting-software-developers-likely-associated-with-north-korean-threat-actors/)[[17]](https://securityscorecard.com/blog/the-job-offer-that-wasnt-how-we-stopped-an-espionage-plot/)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also researched individuals who work in roles related to cryptocurrency and blockchain technologies.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  Enterprise   |  [T1683](https://attack.mitre.org/techniques/T1683)   |  [.001](https://attack.mitre.org/techniques/T1683/001)   |  [Generate Content](https://attack.mitre.org/techniques/T1683): [Written Content](https://attack.mitre.org/techniques/T1683/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has created fake social media accounts such as LinkedIn and Telegram accounts for their targeting efforts.[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|   |   |  [.002](https://attack.mitre.org/techniques/T1683/002)   |  [Generate Content](https://attack.mitre.org/techniques/T1683): [Audio-Visual Content](https://attack.mitre.org/techniques/T1683/002)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has used AI to clone video-conferencing applications to distribute their [BeaverTail](https://attack.mitre.org/software/S1246) malware. They have also used AI to create deepfake videos. [[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|  Enterprise   |  [T1070](https://attack.mitre.org/techniques/T1070)   |  [.004](https://attack.mitre.org/techniques/T1070/004)   |  [Indicator Removal](https://attack.mitre.org/techniques/T1070): [File Deletion](https://attack.mitre.org/techniques/T1070/004)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has configured malware to remove archives used in collection activities following successful exfiltration.[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)

|  Enterprise   |  [T1036](https://attack.mitre.org/techniques/T1036)   |  [Masquerading](https://attack.mitre.org/techniques/T1036)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has delivered [BeaverTail](https://attack.mitre.org/software/S1246) malware masquerading as legitimate software or applications.[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also delivered malicious payloads masquerading as legitimate software drivers.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  Enterprise   |  [T1571](https://attack.mitre.org/techniques/T1571)   |  [Non-Standard Port](https://attack.mitre.org/techniques/T1571)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has used TCP port 1224 for C2.[[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)

|  Enterprise   |  [T1027](https://attack.mitre.org/techniques/T1027)   |  [.010](https://attack.mitre.org/techniques/T1027/010)   |  [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Command Obfuscation](https://attack.mitre.org/techniques/T1027/010)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has obfuscated JavaScript code using Base64 and variable substitutions.[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[11]](https://www.securonix.com/blog/analysis-of-devpopper-new-attack-campaign-targeting-software-developers-likely-associated-with-north-korean-threat-actors/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|   |   |  [.013](https://attack.mitre.org/techniques/T1027/013)   |  [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Encrypted/Encoded File](https://attack.mitre.org/techniques/T1027/013)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has used hexadecimal string encoding to hide critical JavaScript module names, function names, and C2 URLs, which are decoded dynamically at runtime.[[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)

|  Enterprise   |  [T1588](https://attack.mitre.org/techniques/T1588)   |  [.002](https://attack.mitre.org/techniques/T1588/002)   |  [Obtain Capabilities](https://attack.mitre.org/techniques/T1588): [Tool](https://attack.mitre.org/techniques/T1588/002)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has used remote management and monitoring software such as "AnyDesk".[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|   |   |  [.007](https://attack.mitre.org/techniques/T1588/007)   |  [Obtain Capabilities](https://attack.mitre.org/techniques/T1588): [Artificial Intelligence](https://attack.mitre.org/techniques/T1588/007)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has appeared to have used AI to generate images and content to facilitate their campaigns.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|  Enterprise   |  [T1566](https://attack.mitre.org/techniques/T1566)   |  [.003](https://attack.mitre.org/techniques/T1566/003)   |  [Phishing](https://attack.mitre.org/techniques/T1566): [Spearphishing via Service](https://attack.mitre.org/techniques/T1566/003)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has used fake job advertisements and messages sent via social media to spearphish targets.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)[[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[17]](https://securityscorecard.com/blog/the-job-offer-that-wasnt-how-we-stopped-an-espionage-plot/) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also leveraged hiring websites to solicit victims.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|  Enterprise   |  [T1090](https://attack.mitre.org/techniques/T1090)   |  [Proxy](https://attack.mitre.org/techniques/T1090)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has leveraged Astrill VPN for C2.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|  Enterprise   |  [T1219](https://attack.mitre.org/techniques/T1219)   |  [.002](https://attack.mitre.org/techniques/T1219/002)   |  [Remote Access Tools](https://attack.mitre.org/techniques/T1219): [Remote Desktop Software](https://attack.mitre.org/techniques/T1219/002)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has downloaded remote management and monitoring software such as "AnyDesk" for post compromise activities.[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|  Enterprise   |  [T1593](https://attack.mitre.org/techniques/T1593)   |  [Search Open Websites/Domains](https://attack.mitre.org/techniques/T1593)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has utilized open-source indicator of compromise repositories to determine their exposure to include VirusTotal, and MalTrail.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)

|   |   |  [.001](https://attack.mitre.org/techniques/T1593/001)   |  [Social Media](https://attack.mitre.org/techniques/T1593/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) had identified and solicited victims through social media such as LinkedIn, X, and Telegram.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)[[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[17]](https://securityscorecard.com/blog/the-job-offer-that-wasnt-how-we-stopped-an-espionage-plot/)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|   |   |  [.003](https://attack.mitre.org/techniques/T1593/003)   |  [Code Repositories](https://attack.mitre.org/techniques/T1593/003)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) had identified and solicited victims through code repositories such as GitHub.[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1681](https://attack.mitre.org/techniques/T1681)   |  [Search Threat Vendor Data](https://attack.mitre.org/techniques/T1681)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has registered accounts with Threat Intelligence vendor services to check for reporting associated with their infrastructure and to evaluate new potential infrastructure.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)

|  Enterprise   |  [T1684](https://attack.mitre.org/techniques/T1684)   |  [.001](https://attack.mitre.org/techniques/T1684/001)   |  [Social Engineering](https://attack.mitre.org/techniques/T1684): [Impersonation](https://attack.mitre.org/techniques/T1684/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) had impersonated HR hiring personnel through social media, job board notifications, and conducted interviews with victims in order to entice them to download malware disguised as legitimate applications or malicious scripts from code repositories.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[11]](https://www.securonix.com/blog/analysis-of-devpopper-new-attack-campaign-targeting-software-developers-likely-associated-with-north-korean-threat-actors/)[[17]](https://securityscorecard.com/blog/the-job-offer-that-wasnt-how-we-stopped-an-espionage-plot/)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|  Enterprise   |  [T1608](https://attack.mitre.org/techniques/T1608)   |  [.001](https://attack.mitre.org/techniques/T1608/001)   |  [Stage Capabilities](https://attack.mitre.org/techniques/T1608): [Upload Malware](https://attack.mitre.org/techniques/T1608/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has hosted malicious payloads on code repositories used as lures for victims to download.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[11]](https://www.securonix.com/blog/analysis-of-devpopper-new-attack-campaign-targeting-software-developers-likely-associated-with-north-korean-threat-actors/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1082](https://attack.mitre.org/techniques/T1082)   |  [System Information Discovery](https://attack.mitre.org/techniques/T1082)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has configured malicious webpages to identify the victim’s operating system by reviewing the details of the victims User-Agent of their browser.[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  Enterprise   |  [T1204](https://attack.mitre.org/techniques/T1204)   |  [.001](https://attack.mitre.org/techniques/T1204/001)   |  [User Execution](https://attack.mitre.org/techniques/T1204): [Malicious Link](https://attack.mitre.org/techniques/T1204/001)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has lured victims to click on a malicious link that led to download of a malicious payload.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also leveraged links to malicious payloads on social media and code repositories.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|   |   |  [.002](https://attack.mitre.org/techniques/T1204/002)   |  [User Execution](https://attack.mitre.org/techniques/T1204): [Malicious File](https://attack.mitre.org/techniques/T1204/002)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has distributed malicious files requiring direct victim interaction to execute through the guise of a code test.[[16]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[17]](https://securityscorecard.com/blog/the-job-offer-that-wasnt-how-we-stopped-an-espionage-plot/)

|   |   |  [.004](https://attack.mitre.org/techniques/T1204/004)   |  [User Execution](https://attack.mitre.org/techniques/T1204): [Malicious Copy and Paste](https://attack.mitre.org/techniques/T1204/004)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has leveraged ClickFix type tactics enticing victims to copy and paste malicious code.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[12]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)[[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)

|   |   |  [.005](https://attack.mitre.org/techniques/T1204/005)   |  [User Execution](https://attack.mitre.org/techniques/T1204): [Malicious Library](https://attack.mitre.org/techniques/T1204/005)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has relied on users to install a malicious library from a code repository to infect the victim's device and has led to additional payload distribution and theft of sensitive data.[[9]](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)[[1]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[11]](https://www.securonix.com/blog/analysis-of-devpopper-new-attack-campaign-targeting-software-developers-likely-associated-with-north-korean-threat-actors/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1497](https://attack.mitre.org/techniques/T1497)   |  [Virtualization/Sandbox Evasion](https://attack.mitre.org/techniques/T1497)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has requested victims to disable Docker and other container environments in attempts to thwart container isolation and ensure device infection.[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)

## Software

| ID  | Name  | References  | Techniques

|  [S1246](https://attack.mitre.org/software/S1246)   |  [BeaverTail](https://attack.mitre.org/software/S1246)   |  [[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)[[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Web Protocols](https://attack.mitre.org/techniques/T1071/001), [Archive Collected Data](https://attack.mitre.org/techniques/T1560): [Archive via Utility](https://attack.mitre.org/techniques/T1560/001), [Browser Information Discovery](https://attack.mitre.org/techniques/T1217), [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [JavaScript](https://attack.mitre.org/techniques/T1059/007), [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Credentials from Web Browsers](https://attack.mitre.org/techniques/T1555/003), [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555), [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Keychain](https://attack.mitre.org/techniques/T1555/001), [Data from Local System](https://attack.mitre.org/techniques/T1005), [Data Obfuscation](https://attack.mitre.org/techniques/T1001): [Junk Data](https://attack.mitre.org/techniques/T1001/001), [Data Staged](https://attack.mitre.org/techniques/T1074): [Local Data Staging](https://attack.mitre.org/techniques/T1074/001), [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041), [File and Directory Discovery](https://attack.mitre.org/techniques/T1083), [Financial Theft](https://attack.mitre.org/techniques/T1657), [Indicator Removal](https://attack.mitre.org/techniques/T1070): [File Deletion](https://attack.mitre.org/techniques/T1070/004), [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105), [Log Enumeration](https://attack.mitre.org/techniques/T1654), [Masquerading](https://attack.mitre.org/techniques/T1036), [Non-Standard Port](https://attack.mitre.org/techniques/T1571), [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Encrypted/Encoded File](https://attack.mitre.org/techniques/T1027/013), [Supply Chain Compromise](https://attack.mitre.org/techniques/T1195): [Compromise Software Dependencies and Development Tools](https://attack.mitre.org/techniques/T1195/001), [System Information Discovery](https://attack.mitre.org/techniques/T1082), [System Time Discovery](https://attack.mitre.org/techniques/T1124), [User Execution](https://attack.mitre.org/techniques/T1204): [Malicious File](https://attack.mitre.org/techniques/T1204/002)

|  [S1249](https://attack.mitre.org/software/S1249)   |  [HexEval Loader](https://attack.mitre.org/software/S1249)   |  [[13]](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)[[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[15]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Web Protocols](https://attack.mitre.org/techniques/T1071/001), [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [JavaScript](https://attack.mitre.org/techniques/T1059/007), [Deobfuscate/Decode Files or Information](https://attack.mitre.org/techniques/T1140), [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041), [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105), [Input Capture](https://attack.mitre.org/techniques/T1056): [Keylogging](https://attack.mitre.org/techniques/T1056/001), [Masquerading](https://attack.mitre.org/techniques/T1036): [Match Legitimate Resource Name or Location](https://attack.mitre.org/techniques/T1036/005), [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Encrypted/Encoded File](https://attack.mitre.org/techniques/T1027/013), [System Information Discovery](https://attack.mitre.org/techniques/T1082), [System Location Discovery](https://attack.mitre.org/techniques/T1614), [System Network Configuration Discovery](https://attack.mitre.org/techniques/T1016), [System Owner/User Discovery](https://attack.mitre.org/techniques/T1033)

|  [S1245](https://attack.mitre.org/software/S1245)   |  [InvisibleFerret](https://attack.mitre.org/software/S1245)   |  [[2]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[5]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[6]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[7]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[8]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)   |  [Account Discovery](https://attack.mitre.org/techniques/T1087): [Local Account](https://attack.mitre.org/techniques/T1087/001), [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Web Protocols](https://attack.mitre.org/techniques/T1071/001), [Archive Collected Data](https://attack.mitre.org/techniques/T1560): [Archive via Utility](https://attack.mitre.org/techniques/T1560/001), [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547): [XDG Autostart Entries](https://attack.mitre.org/techniques/T1547/013), [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547): [Registry Run Keys / Startup Folder](https://attack.mitre.org/techniques/T1547/001), [Clipboard Data](https://attack.mitre.org/techniques/T1115), [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Python](https://attack.mitre.org/techniques/T1059/006), [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [PowerShell](https://attack.mitre.org/techniques/T1059/001), [Create or Modify System Process](https://attack.mitre.org/techniques/T1543): [Launch Agent](https://attack.mitre.org/techniques/T1543/001), [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Credentials from Web Browsers](https://attack.mitre.org/techniques/T1555/003), [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Password Managers](https://attack.mitre.org/techniques/T1555/005), [Data from Local System](https://attack.mitre.org/techniques/T1005), [Data Staged](https://attack.mitre.org/techniques/T1074): [Local Data Staging](https://attack.mitre.org/techniques/T1074/001), [Deobfuscate/Decode Files or Information](https://attack.mitre.org/techniques/T1140), [Exfiltration Over Alternative Protocol](https://attack.mitre.org/techniques/T1048): [Exfiltration Over Unencrypted Non-C2 Protocol](https://attack.mitre.org/techniques/T1048/003), [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041), [Exfiltration Over Web Service](https://attack.mitre.org/techniques/T1567), [File and Directory Discovery](https://attack.mitre.org/techniques/T1083), [Financial Theft](https://attack.mitre.org/techniques/T1657), [Hide Artifacts](https://attack.mitre.org/techniques/T1564): [Hidden Window](https://attack.mitre.org/techniques/T1564/003), [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105), [Input Capture](https://attack.mitre.org/techniques/T1056), [Input Capture](https://attack.mitre.org/techniques/T1056): [Keylogging](https://attack.mitre.org/techniques/T1056/001), [Non-Application Layer Protocol](https://attack.mitre.org/techniques/T1095), [Non-Standard Port](https://attack.mitre.org/techniques/T1571), [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Encrypted/Encoded File](https://attack.mitre.org/techniques/T1027/013), [Process Discovery](https://attack.mitre.org/techniques/T1057), [Remote Access Tools](https://attack.mitre.org/techniques/T1219), [Selective Exclusion](https://attack.mitre.org/techniques/T1679), [Service Stop](https://attack.mitre.org/techniques/T1489), [Software Discovery](https://attack.mitre.org/techniques/T1518), [System Information Discovery](https://attack.mitre.org/techniques/T1082), [System Location Discovery](https://attack.mitre.org/techniques/T1614), [System Network Configuration Discovery](https://attack.mitre.org/techniques/T1016), [System Owner/User Discovery](https://attack.mitre.org/techniques/T1033)

|  [S1248](https://attack.mitre.org/software/S1248)   |  [XORIndex Loader](https://attack.mitre.org/software/S1248)   |  [[14]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Web Protocols](https://attack.mitre.org/techniques/T1071/001), [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [JavaScript](https://attack.mitre.org/techniques/T1059/007), [Deobfuscate/Decode Files or Information](https://attack.mitre.org/techniques/T1140), [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041), [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105), [Masquerading](https://attack.mitre.org/techniques/T1036): [Match Legitimate Resource Name or Location](https://attack.mitre.org/techniques/T1036/005), [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Encrypted/Encoded File](https://attack.mitre.org/techniques/T1027/013), [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Command Obfuscation](https://attack.mitre.org/techniques/T1027/010), [System Information Discovery](https://attack.mitre.org/techniques/T1082), [System Location Discovery](https://attack.mitre.org/techniques/T1614), [System Network Configuration Discovery](https://attack.mitre.org/techniques/T1016), [System Owner/User Discovery](https://attack.mitre.org/techniques/T1033)

## References

-    [Efstratios Lontzetidis. (2025, January 16). Lazarus APT: Techniques for Hunting Contagious Interview. Retrieved October 20, 2025.](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)
-    [eSentire Threat Response Unit (TRU). (2024, November 14). Bored BeaverTail & InvisibleFerret Yacht Club – A Lazarus Lure Pt.2. Retrieved October 17, 2025.](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)
-    [Ian Kretz, Sebastian Obregoso, Datadog Security Research Team. (2024, October 24). Tenacious Pungsan: A DPRK threat actor linked to Contagious Interview. Retrieved October 20, 2025.](https://securitylabs.datadoghq.com/articles/tenacious-pungsan-dprk-threat-actor-contagious-interview/)
-    [Insikt Group. (2025, February 13). Inside the Scam: North Korea’s IT Worker Threat. Retrieved October 17, 2025.](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)
-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [Seongsu Park. (2024, November 4). From Pyongyang to Your Payroll: The Rise of North Korean Remote Workers in the West. Retrieved October 17, 2025.](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)
-    [Unit 42. (2023, November 21). Hacking Employers and Seeking Employment: Two Job-Related Campaigns Bear Hallmarks of North Korean Threat Actors. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)
-    [Unit42. (2024, October 9). Contagious Interview: DPRK Threat Actors Lure Tech Industry Job Seekers to Install New Variants of BeaverTail and InvisibleFerret Malware. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)
-    [Aleksandar Milenkoski, Sreekar Madabushi, Kenneth Kinion. (2025, September 4). Contagious Interview | North Korean Threat Actors Reveal Plans and Ops by Abusing Cyber Intel Platforms. Retrieved October 20, 2025.](https://www.sentinelone.com/labs/contagious-interview-threat-actors-scout-cyber-intel-platforms-reveal-plans-and-ops/)

-    [Michael “Barni” Barnhart, DTEX, and Anonymous SMEs. (2025, May 14). Exposing DPRK's Cyber Syndicate and Hidden IT Workforce. Retrieved September 3, 2025.](https://reports.dtexsystems.com/DTEX-Exposing+DPRK+Cyber+Syndicate+and+Hidden+IT+Workforce.pdf)
-    [Securonix Threat Research, D.Iuzvyk, T. Peck, O.Kolesnikov. (2024, April 24). Analysis of DEV#POPPER: New Attack Campaign Targeting Software Developers Likely Associated With North Korean Threat Actors. Retrieved October 20, 2025.](https://www.securonix.com/blog/analysis-of-devpopper-new-attack-campaign-targeting-software-developers-likely-associated-with-north-korean-threat-actors/)
-    [Amaury G., Coline Chavane, Felix Aimé and Sekoia TDR. (2025, March 31). From Contagious to ClickFake Interview: Lazarus leveraging the ClickFix tactic. Retrieved April 1, 2025.](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)
-    [Kirill Boychenko. (2025, April 4). Lazarus Expands Malicious npm Campaign: 11 New Packages Add Malware Loaders and Bitbucket Payloads. Retrieved October 20, 2025.](https://socket.dev/blog/lazarus-expands-malicious-npm-campaign-11-new-packages-add-malware-loaders-and-bitbucket)
-    [Kirill Boychenko. (2025, July 14). Contagious Interview Campaign Escalates With 67 Malicious npm Packages and New Malware Loader. Retrieved October 19, 2025.](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)
-    [Kirill Boychenko. (2025, June 25). Another Wave: North Korean Contagious Interview Campaign Drops 35 New Malicious npm Packages. Retrieved October 19, 2025.](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)
-    [Ryan Sherstobitoff. (2024, October 29). Inside a North Korean Phishing Operation Targeting DevOps Employees. Retrieved October 20, 2025.](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)
-    [Steve Cobb. (2024, October 29). The Job Offer That Wasn’t: How We Stopped an Espionage Plot. Retrieved October 20, 2025.](https://securityscorecard.com/blog/the-job-offer-that-wasnt-how-we-stopped-an-espionage-plot/)

×
