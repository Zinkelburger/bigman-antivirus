---
url: "https://attack.mitre.org/techniques/T1059/004/"
title: "Command and Scripting Interpreter: Unix Shell, Sub-technique T1059.004 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:30:07Z"
via: "live"
sha256: "9c8633ef11fc4a7f3ab353aee8a2442144b648cc3f028d1eb0099f4bf3625bcf"
wayback: "https://web.archive.org/web/20261023110339/https://attack.mitre.org/techniques/T1059/004/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Command and Scripting Interpreter: Unix Shell, Sub-technique T1059.004 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1059/004/>
> Snapshot: <https://web.archive.org/web/20261023110339/https://attack.mitre.org/techniques/T1059/004/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059)
- Unix Shell

#  Command and Scripting Interpreter: Unix Shell

##### Other sub-techniques of Command and Scripting Interpreter (13)

| ID  | Name

|  [T1059.001](https://attack.mitre.org/techniques/T1059/001/)   |  [PowerShell](https://attack.mitre.org/techniques/T1059/001/)

|  [T1059.002](https://attack.mitre.org/techniques/T1059/002/)   |  [AppleScript](https://attack.mitre.org/techniques/T1059/002/)

|  [T1059.003](https://attack.mitre.org/techniques/T1059/003/)   |  [Windows Command Shell](https://attack.mitre.org/techniques/T1059/003/)

|  T1059.004   |  Unix Shell

|  [T1059.005](https://attack.mitre.org/techniques/T1059/005/)   |  [Visual Basic](https://attack.mitre.org/techniques/T1059/005/)

|  [T1059.006](https://attack.mitre.org/techniques/T1059/006/)   |  [Python](https://attack.mitre.org/techniques/T1059/006/)

|  [T1059.007](https://attack.mitre.org/techniques/T1059/007/)   |  [JavaScript](https://attack.mitre.org/techniques/T1059/007/)

|  [T1059.008](https://attack.mitre.org/techniques/T1059/008/)   |  [Network Device CLI](https://attack.mitre.org/techniques/T1059/008/)

|  [T1059.009](https://attack.mitre.org/techniques/T1059/009/)   |  [Cloud API](https://attack.mitre.org/techniques/T1059/009/)

|  [T1059.010](https://attack.mitre.org/techniques/T1059/010/)   |  [AutoHotKey & AutoIT](https://attack.mitre.org/techniques/T1059/010/)

|  [T1059.011](https://attack.mitre.org/techniques/T1059/011/)   |  [Lua](https://attack.mitre.org/techniques/T1059/011/)

|  [T1059.012](https://attack.mitre.org/techniques/T1059/012/)   |  [Hypervisor CLI](https://attack.mitre.org/techniques/T1059/012/)

|  [T1059.013](https://attack.mitre.org/techniques/T1059/013/)   |  [Container CLI/API](https://attack.mitre.org/techniques/T1059/013/)

Adversaries may abuse Unix shell commands and scripts for execution. Unix shells are the primary command prompt on Linux, macOS, and ESXi systems, though many variations of the Unix shell exist (e.g. sh, ash, bash, zsh, etc.) depending on the specific OS or distribution.[[1]](https://linux.die.net/man/1/bash)[[2]](https://support.apple.com/HT208050) Unix shells can control every aspect of a system, with certain commands requiring elevated privileges.

Unix shells also support scripts that enable sequential execution of commands as well as other typical programming operations such as conditionals and loops. Common uses of shell scripts include long or repetitive tasks, or the need to run the same set of commands on multiple systems.

Adversaries may abuse Unix shells to execute various commands or payloads. Interactive shells may be accessed through command and control channels or during lateral movement such as with [SSH](https://attack.mitre.org/techniques/T1021/004). Adversaries may also leverage shell scripts to deliver and execute multiple commands on victims or as part of payloads used for persistence.

Some systems, such as embedded devices, lightweight Linux distributions, and ESXi servers, may leverage stripped-down Unix shells via Busybox, a small executable that contains a variety of tools, including a simple shell.

ID: T1059.004

Sub-technique of:  [T1059](https://attack.mitre.org/techniques/T1059)

ⓘ

Tactic: [Execution](https://attack.mitre.org/tactics/TA0002)

ⓘ

Platforms: ESXi, Linux, Network Devices, macOS

Version: 1.4

Created: 09 March 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1059/004/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1059/004/)

## Procedure Examples

| ID  | Name  | Description

|  [C0063](https://attack.mitre.org/campaigns/C0063)   |  [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063)   |

During the [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063), the adversaries utilized the Linux dd command to overwrite portions of the disks with random data.[[3]](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary tasked Claude Code to execute various commands (e.g., ls -la, nc, proxychains4, etc.) and scripts (e.g., bash, etc.) using the Unix shell.[[4]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [S0504](https://attack.mitre.org/software/S0504)   |  [Anchor](https://attack.mitre.org/software/S0504)   |

[Anchor](https://attack.mitre.org/software/S0504) can execute payloads via shell scripting.[[5]](https://medium.com/stage-2-security/anchor-dns-malware-family-goes-cross-platform-d807ba13ca30)

|  [S0584](https://attack.mitre.org/software/S0584)   |  [AppleJeus](https://attack.mitre.org/software/S0584)   |

[AppleJeus](https://attack.mitre.org/software/S0584) has used shell scripts to execute commands after installation and set persistence mechanisms.[[6]](https://us-cert.cisa.gov/ncas/alerts/aa21-048a)[[7]](https://objective-see.org/blog/blog_0x49.html)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has run malicious payloads using Linux and Unix shells. [APT38](https://attack.mitre.org/groups/G0082) has also executed Unix commands sequentially using && and in the background with &.[[8]](https://www.microsoft.com/en-us/security/blog/2026/04/01/mitigating-the-axios-npm-supply-chain-compromise/) [[9]](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) used Linux shell commands for system survey and information gathering prior to exploitation of vulnerabilities such as CVE-2019-19871.[[10]](https://www.fireeye.com/blog/threat-research/2020/03/apt41-initiates-global-intrusion-campaign-using-multiple-exploits.html)

|  [G0143](https://attack.mitre.org/groups/G0143)   |  [Aquatic Panda](https://attack.mitre.org/groups/G0143)   |

[Aquatic Panda](https://attack.mitre.org/groups/G0143) used malicious shell scripts in Linux environments following access via SSH to install Linux versions of Winnti malware.[[11]](https://go.crowdstrike.com/rs/281-OBQ-266/images/2022OverWatchThreatHuntingReport.pdf)

|  [S1184](https://attack.mitre.org/software/S1184)   |  [BOLDMOVE](https://attack.mitre.org/software/S1184)   |

[BOLDMOVE](https://attack.mitre.org/software/S1184) is capable of spawning a remote command shell.[[12]](https://cloud.google.com/blog/topics/threat-intelligence/chinese-actors-exploit-fortios-flaw/)

|  [S1161](https://attack.mitre.org/software/S1161)   |  [BPFDoor](https://attack.mitre.org/software/S1161)   |

[BPFDoor](https://attack.mitre.org/software/S1161) can create a reverse shell and supports vt100 emulator formatting.[[13]](https://sandflysecurity.com/blog/bpfdoor-an-evasive-linux-backdoor-technical-analysis/)

|  [S9015](https://attack.mitre.org/software/S9015)   |  [BRICKSTORM](https://attack.mitre.org/software/S9015)   |

[BRICKSTORM](https://attack.mitre.org/software/S9015) has executed shell commands using /bin/sh.[[14]](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-post-exploitation-lateral-movement)

|  [S0482](https://attack.mitre.org/software/S0482)   |  [Bundlore](https://attack.mitre.org/software/S0482)   |

[Bundlore](https://attack.mitre.org/software/S0482) has leveraged /bin/sh and /bin/bash to execute commands on the victim machine.[[15]](https://mackeeper.com/blog/post/610-macos-bundlore-adware-analysis/)

|  [S0077](https://attack.mitre.org/software/S0077)   |  [CallMe](https://attack.mitre.org/software/S0077)   |

[CallMe](https://attack.mitre.org/software/S0077) has the capability to create a reverse shell on victims.[[16]](http://researchcenter.paloaltonetworks.com/2016/01/scarlet-mimic-years-long-espionage-targets-minority-activists/)

|  [S9042](https://attack.mitre.org/software/S9042)   |  [CanisterWorm](https://attack.mitre.org/software/S9042)   |

[CanisterWorm](https://attack.mitre.org/software/S9042) has used shell commands to enable and start the malicious systemd service for execution and persistence.[[17]](https://www.aikido.dev/blog/teampcp-stage-payload-canisterworm-iran)[[18]](https://www.aikido.dev/blog/teampcp-deploys-worm-npm-trivy-compromise)

|  [S1224](https://attack.mitre.org/software/S1224)   |  [CASTLETAP](https://attack.mitre.org/software/S1224)   |

[CASTLETAP](https://attack.mitre.org/software/S1224) has the ability to spawn BusyBox command shell in victim environments.[[19]](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)

|  [S0220](https://attack.mitre.org/software/S0220)   |  [Chaos](https://attack.mitre.org/software/S0220)   |

[Chaos](https://attack.mitre.org/software/S0220) provides a reverse shell connection on 8338/TCP, encrypted via AES.[[20]](http://gosecure.net/2018/02/14/chaos-stolen-backdoor-rising/)

|  [S1105](https://attack.mitre.org/software/S1105)   |  [COATHANGER](https://attack.mitre.org/software/S1105)   |

[COATHANGER](https://attack.mitre.org/software/S1105) provides a BusyBox reverse shell for command and control.[[21]](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)

|  [S0369](https://attack.mitre.org/software/S0369)   |  [CoinTicker](https://attack.mitre.org/software/S0369)   |

[CoinTicker](https://attack.mitre.org/software/S0369) executes a bash script to establish a reverse shell.[[22]](https://blog.malwarebytes.com/threat-analysis/2018/10/mac-cryptocurrency-ticker-app-installs-backdoors/)

|  [G1052](https://attack.mitre.org/groups/G1052)   |  [Contagious Interview](https://attack.mitre.org/groups/G1052)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has targeted macOS victim hosts using a bash downloader coremedia.sh and a bash script cloud.sh.[[23]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)

|  [S0492](https://attack.mitre.org/software/S0492)   |  [CookieMiner](https://attack.mitre.org/software/S0492)   |

[CookieMiner](https://attack.mitre.org/software/S0492) has used a Unix shell script to run a series of commands targeting macOS.[[24]](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)

|  [S1153](https://attack.mitre.org/software/S1153)   |  [Cuckoo Stealer](https://attack.mitre.org/software/S1153)   |

[Cuckoo Stealer](https://attack.mitre.org/software/S1153) can spawn a bash shell to enable execution on compromised hosts.[[25]](https://www.kandji.io/blog/malware-cuckoo-infostealer-spyware)

|  [S9063](https://attack.mitre.org/software/S9063)   |  [DEEPROOT](https://attack.mitre.org/software/S9063)   |

[DEEPROOT](https://attack.mitre.org/software/S9063) can execute shell commands.[[26]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [S0021](https://attack.mitre.org/software/S0021)   |  [Derusbi](https://attack.mitre.org/software/S0021)   |

[Derusbi](https://attack.mitre.org/software/S0021) is capable of creating a remote Bash shell and executing commands.[[27]](https://paper.seebug.org/papers/APT/APT_CyberCriminal_Campagin/2016/2016.02.29.Turbo_Campaign_Derusbi/TA_Fidelis_Turbo_1602_0.pdf)[[28]](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)

|  [S0600](https://attack.mitre.org/software/S0600)   |  [Doki](https://attack.mitre.org/software/S0600)   |

[Doki](https://attack.mitre.org/software/S0600) has executed shell scripts with /bin/sh.[[29]](https://www.intezer.com/blog/cloud-security/watch-your-containers-doki-infecting-docker-servers-in-the-cloud/)

|  [S0502](https://attack.mitre.org/software/S0502)   |  [Drovorub](https://attack.mitre.org/software/S0502)   |

[Drovorub](https://attack.mitre.org/software/S0502) can execute arbitrary commands as root on a compromised system.[[30]](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)

|  [S0377](https://attack.mitre.org/software/S0377)   |  [Ebury](https://attack.mitre.org/software/S0377)   |

[Ebury](https://attack.mitre.org/software/S0377) can use the commands Xcsh or Xcls to open a shell with [Ebury](https://attack.mitre.org/software/S0377) level permissions and Xxsh to open a shell with root level.[[31]](https://web-assets.esetstatic.com/wls/en/papers/white-papers/ebury-is-alive-but-unseen.pdf)

|  [S0401](https://attack.mitre.org/software/S0401)   |  [Exaramel for Linux](https://attack.mitre.org/software/S0401)   |

[Exaramel for Linux](https://attack.mitre.org/software/S0401) has a command to execute a shell command on the system.[[32]](https://www.welivesecurity.com/2018/10/11/new-telebots-backdoor-linking-industroyer-notpetya/)[[33]](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)

|  [C0053](https://attack.mitre.org/campaigns/C0053)   |  [FLORAHOX Activity](https://attack.mitre.org/campaigns/C0053)   |

[FLORAHOX Activity](https://attack.mitre.org/campaigns/C0053) has executed multiple Bash controller scripts to provide command line inputs for FLORAHOX traversal configurations.[[34]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-orb-networks)

|  [S0410](https://attack.mitre.org/software/S0410)   |  [Fysbis](https://attack.mitre.org/software/S0410)   |

[Fysbis](https://attack.mitre.org/software/S0410) has the ability to create and execute commands in a remote shell for CLI.[[35]](https://researchcenter.paloaltonetworks.com/2016/02/a-look-into-fysbis-sofacys-linux-backdoor/)

|  [S1198](https://attack.mitre.org/software/S1198)   |  [Gomir](https://attack.mitre.org/software/S1198)   |

[Gomir](https://attack.mitre.org/software/S1198) reads command line arguments and parses them for functionality when executed from a Linux shell, and can execute arbitrary strings passed to it as shell commands.[[36]](https://www.security.com/threat-intelligence/springtail-kimsuky-backdoor-espionage)

|  [S0690](https://attack.mitre.org/software/S0690)   |  [Green Lambert](https://attack.mitre.org/software/S0690)   |

[Green Lambert](https://attack.mitre.org/software/S0690) can use shell scripts for execution, such as /bin/sh -c.[[37]](https://objective-see.com/blog/blog_0x68.html)[[38]](https://web.archive.org/web/20211018145402/https://www.glitch-cat.com/blog/green-lambert-and-attack)

|  [S0601](https://attack.mitre.org/software/S0601)   |  [Hildegard](https://attack.mitre.org/software/S0601)   |

[Hildegard](https://attack.mitre.org/software/S0601) has used shell scripts for execution.[[39]](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)

|  [S1203](https://attack.mitre.org/software/S1203)   |  [J-magic](https://attack.mitre.org/software/S1203)   |

The [J-magic](https://attack.mitre.org/software/S1203) agent is executed through a command line argument which specifies an interface and listening port.[[40]](https://blog.lumen.com/the-j-magic-show-magic-packets-and-where-to-find-them/)

|  [S0265](https://attack.mitre.org/software/S0265)   |  [Kazuar](https://attack.mitre.org/software/S0265)   |

[Kazuar](https://attack.mitre.org/software/S0265) uses /bin/bash to execute commands on the victim’s machine.[[41]](https://researchcenter.paloaltonetworks.com/2017/05/unit42-kazuar-multiplatform-espionage-backdoor-api-access/)

|  [S0599](https://attack.mitre.org/software/S0599)   |  [Kinsing](https://attack.mitre.org/software/S0599)   |

[Kinsing](https://attack.mitre.org/software/S0599) has used Unix shell scripts to execute commands in the victim environment.[[42]](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)

|  [S0641](https://attack.mitre.org/software/S0641)   |  [Kobalos](https://attack.mitre.org/software/S0641)   |

[Kobalos](https://attack.mitre.org/software/S0641) can spawn a new pseudo-terminal and execute arbitrary commands at the command prompt.[[43]](https://www.welivesecurity.com/2021/02/02/kobalos-complex-linux-threat-high-performance-computing-infrastructure/)

|  [C0035](https://attack.mitre.org/campaigns/C0035)   |  [KV Botnet Activity](https://attack.mitre.org/campaigns/C0035)   |

[KV Botnet Activity](https://attack.mitre.org/campaigns/C0035) utilizes multiple Bash scripts during botnet installation stages, and the final botnet payload allows for running commands in the Bash shell.[[44]](https://blog.lumen.com/routers-roasting-on-an-open-firewall-the-kv-botnet-investigation/)

|  [S0451](https://attack.mitre.org/software/S0451)   |  [LoudMiner](https://attack.mitre.org/software/S0451)   |

[LoudMiner](https://attack.mitre.org/software/S0451) used shell scripts to launch various services and to start/stop the QEMU virtualization.[[45]](https://www.welivesecurity.com/2019/06/20/loudminer-mining-cracked-vst-software/)

|  [S1016](https://attack.mitre.org/software/S1016)   |  [MacMa](https://attack.mitre.org/software/S1016)   |

[MacMa](https://attack.mitre.org/software/S1016) can execute supplied shell commands and uses bash scripts to perform additional actions.[[46]](https://www.welivesecurity.com/2022/01/25/watering-hole-deploys-new-macos-malware-dazzlespy-asia/)[[47]](https://objective-see.org/blog/blog_0x69.html)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) has the ability to use /bin/bash and /bin/sh to execute commands.[[48]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)[[49]](https://www.proofpoint.com/us/blog/threat-insight/geofenced-netwire-campaigns)

|  [S1107](https://attack.mitre.org/software/S1107)   |  [NKAbuse](https://attack.mitre.org/software/S1107)   |

[NKAbuse](https://attack.mitre.org/software/S1107) is initially installed and executed through an initial shell script.[[50]](https://securelist.com/unveiling-nkabuse/111512/)

|  [C0048](https://attack.mitre.org/campaigns/C0048)   |  [Operation MidnightEclipse](https://attack.mitre.org/campaigns/C0048)   |

During [Operation MidnightEclipse](https://attack.mitre.org/campaigns/C0048), threat actors piped output from stdout to bash for execution.[[51]](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)[[52]](https://unit42.paloaltonetworks.com/cve-2024-3400/)

|  [S0402](https://attack.mitre.org/software/S0402)   |  [OSX/Shlayer](https://attack.mitre.org/software/S0402)   |

[OSX/Shlayer](https://attack.mitre.org/software/S0402) can use bash scripts to check the macOS version, download payloads, and extract bytes from files. [OSX/Shlayer](https://attack.mitre.org/software/S0402) uses the command sh -c tail -c +1381... to extract bytes at an offset from a specified file. [OSX/Shlayer](https://attack.mitre.org/software/S0402) uses the curl -fsL "$url" >$tmp_path command to download malicious payloads into a temporary directory.[[53]](https://blogs.vmware.com/security/2020/02/vmware-carbon-black-tau-threat-analysis-shlayer-macos.html)[[54]](https://www.sentinelone.com/blog/coming-out-of-your-shell-from-shlayer-to-zshlayer/)[[55]](https://labs.sentinelone.com/20-common-tools-techniques-used-by-macos-threat-actors-malware/)[[56]](https://objective-see.com/blog/blog_0x4E.html)

|  [S0352](https://attack.mitre.org/software/S0352)   |  [OSX_OCEANLOTUS.D](https://attack.mitre.org/software/S0352)   |

[OSX_OCEANLOTUS.D](https://attack.mitre.org/software/S0352) uses a shell script as the main executable inside an app bundle and drops an embedded base64-encoded payload to the /tmp folder.[[57]](https://www.trendmicro.com/en_us/research/20/k/new-macos-backdoor-connected-to-oceanlotus-surfaces.html)[[58]](https://www.sentinelone.com/labs/apt32-multi-stage-macos-trojan-innovates-on-crimeware-scripting-technique/)

|  [S1109](https://attack.mitre.org/software/S1109)   |  [PACEMAKER](https://attack.mitre.org/software/S1109)   |

[PACEMAKER](https://attack.mitre.org/software/S1109) can use a simple bash script for execution.[[59]](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)

|  [S0587](https://attack.mitre.org/software/S0587)   |  [Penquin](https://attack.mitre.org/software/S0587)   |

[Penquin](https://attack.mitre.org/software/S0587) can execute remote commands using bash scripts.[[60]](https://www.leonardo.com/documents/20142/10868623/Malware+Technical+Insight+_Turla+%E2%80%9CPenquin_x64%E2%80%9D.pdf)

|  [S1123](https://attack.mitre.org/software/S1123)   |  [PITSTOP](https://attack.mitre.org/software/S1123)   |

[PITSTOP](https://attack.mitre.org/software/S1123) has the ability to receive shell commands over a Unix domain socket.[[61]](https://www.mandiant.com/resources/blog/investigating-ivanti-exploitation-persistence)

|  [S0279](https://attack.mitre.org/software/S0279)   |  [Proton](https://attack.mitre.org/software/S0279)   |

[Proton](https://attack.mitre.org/software/S0279) uses macOS' .command file type to script actions.[[62]](https://objective-see.com/blog/blog_0x25.html)

|  [S1108](https://attack.mitre.org/software/S1108)   |  [PULSECHECK](https://attack.mitre.org/software/S1108)   |

[PULSECHECK](https://attack.mitre.org/software/S1108) can use Unix shell script for command execution.[[59]](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)

|  [C0055](https://attack.mitre.org/campaigns/C0055)   |  [Quad7 Activity](https://attack.mitre.org/campaigns/C0055)   |

[Quad7 Activity](https://attack.mitre.org/campaigns/C0055) has enabled the creation of an access-controlled command shell /bin/sh on compromised routers.[[63]](https://www.microsoft.com/en-us/security/blog/2024/10/31/chinese-threat-actor-storm-0940-uses-credentials-from-password-spray-attacks-from-a-covert-network/)[[64]](https://www.bitsight.com/blog/7777-botnet-insights-multi-target-botnet)

|  [C0056](https://attack.mitre.org/campaigns/C0056)   |  [RedPenguin](https://attack.mitre.org/campaigns/C0056)   |

During [RedPenguin](https://attack.mitre.org/campaigns/C0056), [UNC3886](https://attack.mitre.org/groups/G1048) used malware capable of launching an interactive shell.[[65]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)[[66]](https://supportportal.juniper.net/sfc/servlet.shepherd/document/download/069Dp00000FzdmIIAR?operationContext=S1)

|  [S1219](https://attack.mitre.org/software/S1219)   |  [REPTILE](https://attack.mitre.org/software/S1219)   |

[REPTILE](https://attack.mitre.org/software/S1219) can deploy components automatically with shell scripts.[[67]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  [S1222](https://attack.mitre.org/software/S1222)   |  [RIFLESPINE](https://attack.mitre.org/software/S1222)   |

[RIFLESPINE](https://attack.mitre.org/software/S1222) can execute commands with /bin/sh.[[67]](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)

|  [G0106](https://attack.mitre.org/groups/G0106)   |  [Rocke](https://attack.mitre.org/groups/G0106)   |

[Rocke](https://attack.mitre.org/groups/G0106) used shell scripts to run commands which would obtain persistence and execute the cryptocurrency mining malware.[[68]](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)

|  [G1045](https://attack.mitre.org/groups/G1045)   |  [Salt Typhoon](https://attack.mitre.org/groups/G1045)   |

[Salt Typhoon](https://attack.mitre.org/groups/G1045) has utilized a bash shell to execute Linux commands.[[69]](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)

|  [G1015](https://attack.mitre.org/groups/G1015)   |  [Scattered Spider](https://attack.mitre.org/groups/G1015)   |

[Scattered Spider](https://attack.mitre.org/groups/G1015) has used the command shell to upload and install the Teleport remote access tool to a compromised vCenter Server Appliance.[[70]](https://cloud.google.com/blog/topics/threat-intelligence/defending-vsphere-from-unc3944)

|  [G1041](https://attack.mitre.org/groups/G1041)   |  [Sea Turtle](https://attack.mitre.org/groups/G1041)   |

[Sea Turtle](https://attack.mitre.org/groups/G1041) used shell scripts for post-exploitation execution in victim environments.[[71]](https://www.pwc.com/gx/en/issues/cybersecurity/cyber-threat-intelligence/tortoise-and-malwahare.html)[[72]](https://www.huntandhackett.com/blog/turkish-espionage-campaigns)

|  [S9008](https://attack.mitre.org/software/S9008)   |  [Shai-Hulud](https://attack.mitre.org/software/S9008)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has utilized Linux shell commands to modify configuration files.[[73]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  [S0468](https://attack.mitre.org/software/S0468)   |  [Skidmap](https://attack.mitre.org/software/S0468)   |

[Skidmap](https://attack.mitre.org/software/S0468) has used pm.sh to download and install its main payload.[[74]](https://blog.trendmicro.com/trendlabs-security-intelligence/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload/)

|  [S1163](https://attack.mitre.org/software/S1163)   |  [SnappyTCP](https://attack.mitre.org/software/S1163)   |

[SnappyTCP](https://attack.mitre.org/software/S1163) creates the reverse shell using a pthread spawning a bash shell.[[71]](https://www.pwc.com/gx/en/issues/cybersecurity/cyber-threat-intelligence/tortoise-and-malwahare.html)

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has leveraged malware capable of execution via the Linux CLI.[[75]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  [S9041](https://attack.mitre.org/software/S9041)   |  [TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041)   |

[TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041) has abused the shell script files entrypoint.sh (in trivy-action) and setup.sh (in ast-github-action/2.3.28) for discovery and credential harvesting.[[76]](https://www.sysdig.com/blog/teampcp-expands-supply-chain-compromise-spreads-from-trivy-to-checkmarx-github-actions) [[77]](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)

|  [G0139](https://attack.mitre.org/groups/G0139)   |  [TeamTNT](https://attack.mitre.org/groups/G0139)   |

[TeamTNT](https://attack.mitre.org/groups/G0139) has used shell scripts for execution.[[78]](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)[[79]](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)

|  [S0647](https://attack.mitre.org/software/S0647)   |  [Turian](https://attack.mitre.org/software/S0647)   |

[Turian](https://attack.mitre.org/software/S0647) has the ability to use /bin/sh to execute commands.[[80]](https://www.welivesecurity.com/2021/06/10/backdoordiplomacy-upgrading-quarian-turian/)

|  [G1048](https://attack.mitre.org/groups/G1048)   |  [UNC3886](https://attack.mitre.org/groups/G1048)   |

[UNC3886](https://attack.mitre.org/groups/G1048) has used a bash script to install malicious vSphere Installation Bundles (VIBs).[[81]](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)

|  [G1047](https://attack.mitre.org/groups/G1047)   |  [Velvet Ant](https://attack.mitre.org/groups/G1047)   |

[Velvet Ant](https://attack.mitre.org/groups/G1047) used a custom tool, VELVETSTING, to parse encoded inbound commands to compromised F5 BIG-IP devices and then execute them via the Unix shell.[[82]](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/)

|  [S1217](https://attack.mitre.org/software/S1217)   |  [VIRTUALPITA](https://attack.mitre.org/software/S1217)   |

[VIRTUALPITA](https://attack.mitre.org/software/S1217) has the ability to spawn a bash shell for script execution.[[81]](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has used Brightmetricagent.exe which contains a command- line interface (CLI) library that can leverage command shells including Z Shell (zsh).[[83]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [S0466](https://attack.mitre.org/software/S0466)   |  [WindTail](https://attack.mitre.org/software/S0466)   |

[WindTail](https://attack.mitre.org/software/S0466) can use the open command to execute an application.[[84]](https://objective-see.com/blog/blog_0x3B.html)

|  [S0658](https://attack.mitre.org/software/S0658)   |  [XCSSET](https://attack.mitre.org/software/S0658)   |

[XCSSET](https://attack.mitre.org/software/S0658) uses a shell script to execute Mach-o files and osacompile commands such as, osacompile -x -o xcode.app main.applescript.[[85]](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)

|  [S1114](https://attack.mitre.org/software/S1114)   |  [ZIPLINE](https://attack.mitre.org/software/S1114)   |

[ZIPLINE](https://attack.mitre.org/software/S1114) can use /bin/sh to create a reverse shell and execute commands.[[86]](https://www.mandiant.com/resources/blog/suspected-apt-targets-ivanti-zero-day)

## Mitigations

| ID  | Mitigation  | Description

|  [M1038](https://attack.mitre.org/mitigations/M1038)   |  [Execution Prevention](https://attack.mitre.org/mitigations/M1038)   |

Use application control where appropriate. On ESXi hosts, the execInstalledOnly feature prevents binaries from being run unless they have been packaged and signed as part of a vSphere installation bundle (VIB).[[87]](https://cloud.google.com/blog/topics/threat-intelligence/vmware-detection-containment-hardening)

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0384](https://attack.mitre.org/detectionstrategies/DET0384)   |  [Behavioral Detection of Unix Shell Execution](https://attack.mitre.org/detectionstrategies/DET0384)   |  [AN1081](https://attack.mitre.org/detectionstrategies/DET0384#AN1081)   |

Detects bash, sh, zsh, or BusyBox shell execution initiated via remote sessions, unauthorized users, or embedded within secondary script interpreters. Focus is on chained behavior: shell > suspicious commands > network discovery or persistence indicators.

|  [AN1082](https://attack.mitre.org/detectionstrategies/DET0384#AN1082)   |

Identifies use of sh/bash/zsh in suspicious context, such as user scripts launched from non-standard apps (e.g., Preview.app), embedded in LaunchDaemons, or executed outside Terminal.app. Looks for misuse in Automator, LaunchAgents, or NSAppleScript-executed shell.

|  [AN1083](https://attack.mitre.org/detectionstrategies/DET0384#AN1083)   |

Detects BusyBox or Ash shell execution from unauthorized logins or remote connections. Focus is on rare shell invocations from DCUI, SSH sessions, or remote management paths. Also watches for payload droppers or persistence artifacts using shell.

|  [AN1084](https://attack.mitre.org/detectionstrategies/DET0384#AN1084)   |

Detects Unix shell usage on network appliances (e.g., routers, firewalls, embedded Linux) through rare console commands, CLI interfaces, or script injection via exposed APIs or SSH.

## References

-    [die.net. (n.d.). bash(1) - Linux man page. Retrieved June 12, 2020.](https://linux.die.net/man/1/bash)
-    [Apple. (2020, January 28). Use zsh as the default shell on your Mac. Retrieved June 12, 2020.](https://support.apple.com/HT208050)
-    [CERT Polska. (2026, January 30). Energy Sector Incident Report – 29 December. Retrieved April 22, 2026.](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)
-    [Grange, W. (2020, July 13). Anchor_dns malware goes cross platform. Retrieved September 10, 2020.](https://medium.com/stage-2-security/anchor-dns-malware-family-goes-cross-platform-d807ba13ca30)
-    [Cybersecurity and Infrastructure Security Agency. (2021, February 21). AppleJeus: Analysis of North Korea’s Cryptocurrency Malware. Retrieved March 1, 2021.](https://us-cert.cisa.gov/ncas/alerts/aa21-048a)
-    [Patrick Wardle. (2019, October 12). Pass the AppleJeus. Retrieved September 28, 2022.](https://objective-see.org/blog/blog_0x49.html)
-    [Microsoft Threat Intelligence. (2026, April 1). Mitigating the Axios npm supply chain compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/01/mitigating-the-axios-npm-supply-chain-compromise/)
-    [Microsoft Threat Intelligence. (2026, April 16). Dissecting Sapphire Sleet’s macOS intrusion from lure to compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)
-    [Glyer, C, et al. (2020, March). This Is Not a Test: APT41 Initiates Global Intrusion Campaign Using Multiple Exploits. Retrieved April 28, 2020.](https://www.fireeye.com/blog/threat-research/2020/03/apt41-initiates-global-intrusion-campaign-using-multiple-exploits.html)
-    [CrowdStrike. (2023). 2022 Falcon OverWatch Threat Hunting Report. Retrieved May 20, 2024.](https://go.crowdstrike.com/rs/281-OBQ-266/images/2022OverWatchThreatHuntingReport.pdf)
-    [Scott Henderson, Cristiana Kittner, Sarah Hawley & Mark Lechtik, Google Cloud. (2023, January 19). Suspected Chinese Threat Actors Exploiting FortiOS Vulnerability (CVE-2022-42475). Retrieved December 31, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/chinese-actors-exploit-fortios-flaw/)
-    [The Sandfly Security Team. (2022, May 11). BPFDoor - An Evasive Linux Backdoor Technical Analysis. Retrieved September 29, 2023.](https://sandflysecurity.com/blog/bpfdoor-an-evasive-linux-backdoor-technical-analysis/)
-    [Matt Lin, Austin Larsen, John Wolfram, Ashley Pearson, Josh Murchie, Lukasz Lamparski, Joseph Pisano, Ryan Hall, Ron Craft, Shawn Crew, Billy Wong, Tyler McLellan. (2024, April 4). Cutting Edge, Part 4: Ivanti Connect Secure VPN Post-Exploitation Lateral Movement Case Studies. Retrieved April 16, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/ivanti-post-exploitation-lateral-movement)
-    [Sushko, O. (2019, April 17). macOS Bundlore: Mac Virus Bypassing macOS Security Features. Retrieved June 30, 2020.](https://mackeeper.com/blog/post/610-macos-bundlore-adware-analysis/)
-    [Falcone, R. and Miller-Osborn, J.. (2016, January 24). Scarlet Mimic: Years-Long Espionage Campaign Targets Minority Activists. Retrieved February 10, 2016.](http://researchcenter.paloaltonetworks.com/2016/01/scarlet-mimic-years-long-espionage-targets-minority-activists/)
-    [Eriksen, C. (2026, March 22). CanisterWorm Gets Teeth: TeamPCP's Kubernetes Wiper Targets Iran. Retrieved July 27, 2026.](https://www.aikido.dev/blog/teampcp-stage-payload-canisterworm-iran)
-    [Eriksen, C. (2026, March 20). TeamPCP deploys CanisterWorm on NPM following Trivy compromise. Retrieved July 27, 2026.](https://www.aikido.dev/blog/teampcp-deploys-worm-npm-trivy-compromise)
-    [Marvi, A. et al.. (2023, March 16). Fortinet Zero-Day and Custom Malware Used by Suspected Chinese Actor in Espionage Operation. Retrieved March 22, 2023.](https://www.mandiant.com/resources/blog/fortinet-malware-ecosystem)
-    [Sebastian Feldmann. (2018, February 14). Chaos: a Stolen Backdoor Rising Again. Retrieved March 5, 2018.](http://gosecure.net/2018/02/14/chaos-stolen-backdoor-rising/)
-    [Dutch Military Intelligence and Security Service (MIVD) & Dutch General Intelligence and Security Service (AIVD). (2024, February 6). Ministry of Defense of the Netherlands uncovers COATHANGER, a stealthy Chinese FortiGate RAT. Retrieved February 7, 2024.](https://www.ncsc.nl/binaries/ncsc/documenten/publicaties/2024/februari/6/mivd-aivd-advisory-coathanger-tlp-clear/TLP-CLEAR+MIVD+AIVD+Advisory+COATHANGER.pdf)
-    [Thomas Reed. (2018, October 29). Mac cryptocurrency ticker app installs backdoors. Retrieved April 23, 2019.](https://blog.malwarebytes.com/threat-analysis/2018/10/mac-cryptocurrency-ticker-app-installs-backdoors/)
-    [Amaury G., Coline Chavane, Felix Aimé and Sekoia TDR. (2025, March 31). From Contagious to ClickFake Interview: Lazarus leveraging the ClickFix tactic. Retrieved April 1, 2025.](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)
-    [Chen, y., et al. (2019, January 31). Mac Malware Steals Cryptocurrency Exchanges’ Cookies. Retrieved July 22, 2020.](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)
-    [Kohler, A. and Lopez, C. (2024, April 30). Malware: Cuckoo Behaves Like Cross Between Infostealer and Spyware. Retrieved August 20, 2024.](https://www.kandji.io/blog/malware-cuckoo-infostealer-spyware)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [Fidelis Cybersecurity. (2016, February 29). The Turbo Campaign, Featuring Derusbi for 64-bit Linux. Retrieved March 2, 2016.](https://paper.seebug.org/papers/APT/APT_CyberCriminal_Campagin/2016/2016.02.29.Turbo_Campaign_Derusbi/TA_Fidelis_Turbo_1602_0.pdf)
-    [FireEye. (2018, March 16). Suspected Chinese Cyber Espionage Group (TEMP.Periscope) Targeting U.S. Engineering and Maritime Industries. Retrieved April 11, 2018.](https://www.fireeye.com/blog/threat-research/2018/03/suspected-chinese-espionage-group-targeting-maritime-and-engineering-industries.html)
-    [Fishbein, N., Kajiloti, M.. (2020, July 28). Watch Your Containers: Doki Infecting Docker Servers in the Cloud. Retrieved March 30, 2021.](https://www.intezer.com/blog/cloud-security/watch-your-containers-doki-infecting-docker-servers-in-the-cloud/)
-    [NSA/FBI. (2020, August). Russian GRU 85th GTsSS Deploys Previously Undisclosed Drovorub Malware. Retrieved August 25, 2020.](https://media.defense.gov/2020/Aug/13/2002476465/-1/-1/0/CSA_DROVORUB_RUSSIAN_GRU_MALWARE_AUG_2020.PDF)
-    [Marc-Etienne M.Léveillé. (2024, May 1). Ebury is alive but unseen. Retrieved May 21, 2024.](https://web-assets.esetstatic.com/wls/en/papers/white-papers/ebury-is-alive-but-unseen.pdf)
-    [Cherepanov, A., Lipovsky, R. (2018, October 11). New TeleBots backdoor: First evidence linking Industroyer to NotPetya. Retrieved November 27, 2018.](https://www.welivesecurity.com/2018/10/11/new-telebots-backdoor-linking-industroyer-notpetya/)
-    [ANSSI. (2021, January 27). SANDWORM INTRUSION SET CAMPAIGN TARGETING CENTREON SYSTEMS. Retrieved March 30, 2021.](https://www.cert.ssi.gouv.fr/uploads/CERTFR-2021-CTI-005.pdf)
-    [Raggi, Michael. (2024, May 22). IOC Extinction? China-Nexus Cyber Espionage Actors Use ORB Networks to Raise Cost on Defenders. Retrieved July 8, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-orb-networks)
-    [Bryan Lee and Rob Downs. (2016, February 12). A Look Into Fysbis: Sofacy’s Linux Backdoor. Retrieved September 10, 2017.](https://researchcenter.paloaltonetworks.com/2016/02/a-look-into-fysbis-sofacys-linux-backdoor/)
-    [Symantec Threat Hunter Team. (2024, May 16). Springtail: New Linux Backdoor Added to Toolkit. Retrieved January 17, 2025.](https://www.security.com/threat-intelligence/springtail-kimsuky-backdoor-espionage)
-    [Sandvik, Runa. (2021, October 1). Made In America: Green Lambert for OS X. Retrieved March 21, 2022.](https://objective-see.com/blog/blog_0x68.html)
-    [Sandvik, Runa. (2021, October 18). Green Lambert and ATT&CK. Retrieved November 17, 2024.](https://web.archive.org/web/20211018145402/https://www.glitch-cat.com/blog/green-lambert-and-attack)
-    [Chen, J. et al. (2021, February 3). Hildegard: New TeamTNT Cryptojacking Malware Targeting Kubernetes. Retrieved April 5, 2021.](https://unit42.paloaltonetworks.com/hildegard-malware-teamtnt/)
-    [Black Lotus Labs. (2025, January 23). The J-Magic Show: Magic Packets and Where to find them. Retrieved February 17, 2025.](https://blog.lumen.com/the-j-magic-show-magic-packets-and-where-to-find-them/)
-    [Levene, B, et al. (2017, May 03). Kazuar: Multiplatform Espionage Backdoor with API Access. Retrieved July 17, 2018.](https://researchcenter.paloaltonetworks.com/2017/05/unit42-kazuar-multiplatform-espionage-backdoor-api-access/)
-    [Singer, G. (2020, April 3). Threat Alert: Kinsing Malware Attacks Targeting Container Environments. Retrieved April 1, 2021.](https://blog.aquasec.com/threat-alert-kinsing-malware-container-vulnerability)
-    [M.Leveille, M., Sanmillan, I. (2021, February 2). Kobalos – A complex Linux threat to high performance computing infrastructure. Retrieved August 24, 2021.](https://www.welivesecurity.com/2021/02/02/kobalos-complex-linux-threat-high-performance-computing-infrastructure/)
-    [Black Lotus Labs. (2023, December 13). Routers Roasting On An Open Firewall: The KV-Botnet Investigation. Retrieved June 10, 2024.](https://blog.lumen.com/routers-roasting-on-an-open-firewall-the-kv-botnet-investigation/)

-    [Malik, M. (2019, June 20). LoudMiner: Cross-platform mining in cracked VST software. Retrieved May 18, 2020.](https://www.welivesecurity.com/2019/06/20/loudminer-mining-cracked-vst-software/)
-    [M.Léveillé, M., Cherepanov, A.. (2022, January 25). Watering hole deploys new macOS malware, DazzleSpy, in Asia. Retrieved May 6, 2022.](https://www.welivesecurity.com/2022/01/25/watering-hole-deploys-new-macos-malware-dazzlespy-asia/)
-    [Wardle, P. (2021, November 11). OSX.CDDS (OSX.MacMa). Retrieved June 30, 2022.](https://objective-see.org/blog/blog_0x69.html)
-    [Lambert, T. (2020, January 29). Intro to Netwire. Retrieved January 7, 2021.](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)
-    [Proofpoint. (2020, December 2). Geofenced NetWire Campaigns. Retrieved January 7, 2021.](https://www.proofpoint.com/us/blog/threat-insight/geofenced-netwire-campaigns)
-    [KASPERSKY GERT. (2023, December 14). Unveiling NKAbuse: a new multiplatform threat abusing the NKN protocol. Retrieved February 8, 2024.](https://securelist.com/unveiling-nkabuse/111512/)
-    [Volexity Threat Research. (2024, April 12). Zero-Day Exploitation of Unauthenticated Remote Code Execution Vulnerability in GlobalProtect (CVE-2024-3400). Retrieved November 20, 2024.](https://www.volexity.com/blog/2024/04/12/zero-day-exploitation-of-unauthenticated-remote-code-execution-vulnerability-in-globalprotect-cve-2024-3400/)
-    [Unit 42. (2024, April 12). Threat Brief: Operation MidnightEclipse, Post-Exploitation Activity Related to CVE-2024-3400 . Retrieved January 15, 2025.](https://unit42.paloaltonetworks.com/cve-2024-3400/)
-    [Carbon Black Threat Analysis Unit. (2019, February 12). New macOS Malware Variant of Shlayer (OSX) Discovered. Retrieved August 8, 2019.](https://blogs.vmware.com/security/2020/02/vmware-carbon-black-tau-threat-analysis-shlayer-macos.html)
-    [Phil Stokes. (2020, September 8). Coming Out of Your Shell: From Shlayer to ZShlayer. Retrieved September 13, 2021.](https://www.sentinelone.com/blog/coming-out-of-your-shell-from-shlayer-to-zshlayer/)
-    [Phil Stokes. (2021, February 16). 20 Common Tools & Techniques Used by macOS Threat Actors & Malware. Retrieved August 23, 2021.](https://labs.sentinelone.com/20-common-tools-techniques-used-by-macos-threat-actors-malware/)
-    [Patrick Wardle. (2020, August 30). Apple Approved Malware malicious code ...now notarized!? #2020. Retrieved September 13, 2021.](https://objective-see.com/blog/blog_0x4E.html)
-    [Magisa, L. (2020, November 27). New MacOS Backdoor Connected to OceanLotus Surfaces. Retrieved December 2, 2020.](https://www.trendmicro.com/en_us/research/20/k/new-macos-backdoor-connected-to-oceanlotus-surfaces.html)
-    [Phil Stokes. (2020, December 2). APT32 Multi-stage macOS Trojan Innovates on Crimeware Scripting Technique. Retrieved September 13, 2021.](https://www.sentinelone.com/labs/apt32-multi-stage-macos-trojan-innovates-on-crimeware-scripting-technique/)
-    [Perez, D. et al. (2021, April 20). Check Your Pulse: Suspected APT Actors Leverage Authentication Bypass Techniques and Pulse Secure Zero-Day. Retrieved February 5, 2024.](https://www.mandiant.com/resources/blog/suspected-apt-actors-leverage-bypass-techniques-pulse-secure-zero-day)
-    [Leonardo. (2020, May 29). MALWARE TECHNICAL INSIGHT TURLA “Penquin_x64”. Retrieved March 11, 2021.](https://www.leonardo.com/documents/20142/10868623/Malware+Technical+Insight+_Turla+%E2%80%9CPenquin_x64%E2%80%9D.pdf)
-    [Lin, M. et al. (2024, February 27). Cutting Edge, Part 3: Investigating Ivanti Connect Secure VPN Exploitation and Persistence Attempts. Retrieved March 1, 2024.](https://www.mandiant.com/resources/blog/investigating-ivanti-exploitation-persistence)
-    [Patrick Wardle. (n.d.). Mac Malware of 2017. Retrieved September 21, 2018.](https://objective-see.com/blog/blog_0x25.html)
-    [Microsoft Threat Intelligence. (2024, October 31). Chinese threat actor Storm-0940 uses credentials from password spray attacks from a covert network. Retrieved June 4, 2025.](https://www.microsoft.com/en-us/security/blog/2024/10/31/chinese-threat-actor-storm-0940-uses-credentials-from-password-spray-attacks-from-a-covert-network/)
-    [Batista, João. Gi7w0rm. (2024, August 27). Retrieved June 5, 2025.](https://www.bitsight.com/blog/7777-botnet-insights-multi-target-botnet)
-    [Lamparski, L. et al. (2025, March 11). Ghost in the Router: China-Nexus Espionage Actor UNC3886 Targets Juniper Routers. Retrieved June 24, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-espionage-targets-juniper-routers)
-    [Juniper Networks, Cybersecurity R&D. (2025, March 11). The RedPenguin Malware Incident. Retrieved June 24, 2025.](https://supportportal.juniper.net/sfc/servlet.shepherd/document/download/069Dp00000FzdmIIAR?operationContext=S1)
-    [Punsaen Boonyakarn, Shawn Chew, Logeswaran Nadarajan, Mathew Potaczek, Jakub Jozwiak, and Alex Marvi. (2024, June 18). Cloaked and Covert: Uncovering UNC3886 Espionage Operations. Retrieved September 24, 2024.](https://cloud.google.com/blog/topics/threat-intelligence/uncovering-unc3886-espionage-operations)
-    [Liebenberg, D.. (2018, August 30). Rocke: The Champion of Monero Miners. Retrieved May 26, 2020.](https://blog.talosintelligence.com/2018/08/rocke-champion-of-monero-miners.html)
-    [CISA. (2025, September 3). Countering Chinese State-Sponsored Actors Compromise of Networks Worldwide to Feed Global Espionage System. Retrieved September 22, 2026.](https://www.cisa.gov/news-events/cybersecurity-advisories/aa25-239a)
-    [Mandiant Incident Response. (2025, July 23). From Help Desk to Hypervisor: Defending Your VMware vSphere Estate from UNC3944. Retrieved October 13, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/defending-vsphere-from-unc3944)
-    [PwC Threat Intelligence. (2023, December 5). The Tortoise and The Malware. Retrieved November 20, 2024.](https://www.pwc.com/gx/en/issues/cybersecurity/cyber-threat-intelligence/tortoise-and-malwahare.html)
-    [Hunt & Hackett Research Team. (2024, January 5). Turkish espionage campaigns in the Netherlands. Retrieved November 20, 2024.](https://www.huntandhackett.com/blog/turkish-espionage-campaigns)
-    [Socket Research Team. (2025, November 24). Shai Hulud Strikes Again (v2). Retrieved April 9, 2026.](https://socket.dev/blog/shai-hulud-strikes-again-v2)
-    [Remillano, A., Urbanec, J. (2019, September 19). Skidmap Linux Malware Uses Rootkit Capabilities to Hide Cryptocurrency-Mining Payload. Retrieved June 4, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/skidmap-linux-malware-uses-rootkit-capabilities-to-hide-cryptocurrency-mining-payload/)
-    [Hunt.io. (2026, May 14). How TeamPCP's Python Toolkit Survives a C2 Takedown: FIRESCALE, GitHub, and the Victim's Own Account. Retrieved July 16, 2026.](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)
-    [Sysdig Threat Research Team. (2026, March 23). TeamPCP expands: Supply chain compromise spreads from Trivy to Checkmarx GitHub Actions. Retrieved July 1, 2026.](https://www.sysdig.com/blog/teampcp-expands-supply-chain-compromise-spreads-from-trivy-to-checkmarx-github-actions)
-    [Unit 42. (2026, March 31). Weaponizing the Protectors: TeamPCP’s Multi-Stage Supply Chain Attack on Security Infrastructure. Retrieved July 1, 2026.](https://unit42.paloaltonetworks.com/teampcp-supply-chain-attacks/)
-    [Fiser, D. Oliveira, A. (n.d.). Tracking the Activities of TeamTNT A Closer Look at a Cloud-Focused Malicious Actor Group. Retrieved September 22, 2021.](https://documents.trendmicro.com/assets/white_papers/wp-tracking-the-activities-of-teamTNT.pdf)
-    [Darin Smith. (2022, April 21). TeamTNT targeting AWS, Alibaba. Retrieved August 4, 2022.](https://blog.talosintelligence.com/teamtnt-targeting-aws-alibaba-2/)
-    [Adam Burgher. (2021, June 10). BackdoorDiplomacy: Upgrading from Quarian to Turian. Retrieved September 1, 2021](https://www.welivesecurity.com/2021/06/10/backdoordiplomacy-upgrading-quarian-turian/)
-    [Alexander Marvi, Jeremy Koppen, Tufail Ahmed, and Jonathan Lepore. (2022, September 29). Bad VIB(E)s Part One: Investigating Novel Malware Persistence Within ESXi Hypervisors. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/esxi-hypervisors-malware-persistence)
-    [Sygnia Team. (2024, June 3). China-Nexus Threat Group ‘Velvet Ant’ Abuses F5 Load Balancers for Persistence. Retrieved March 14, 2025.](https://www.sygnia.co/blog/china-nexus-threat-group-velvet-ant/)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Wardle, Patrick. (2018, December 20). Middle East Cyber-Espionage analyzing WindShift's implant: OSX.WindTail (part 1). Retrieved October 3, 2019.](https://objective-see.com/blog/blog_0x3B.html)
-    [Mac Threat Response, Mobile Research Team. (2020, August 13). The XCSSET Malware: Inserts Malicious Code Into Xcode Projects, Performs UXSS Backdoor Planting in Safari, and Leverages Two Zero-day Exploits. Retrieved October 5, 2021.](https://documents.trendmicro.com/assets/pdf/XCSSET_Technical_Brief.pdf)
-    [McLellan, T. et al. (2024, January 12). Cutting Edge: Suspected APT Targets Ivanti Connect Secure VPN in New Zero-Day Exploitation. Retrieved February 27, 2024.](https://www.mandiant.com/resources/blog/suspected-apt-targets-ivanti-zero-day)
-    [Alex Marvi, Greg Blaum, and Ron Craft. (2023, June 28). Detection, Containment, and Hardening Opportunities for Privileged Guest Operations, Anomalous Behavior, and VMCI Backdoors on Compromised VMware Hosts. Retrieved March 26, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/vmware-detection-containment-hardening)

×
