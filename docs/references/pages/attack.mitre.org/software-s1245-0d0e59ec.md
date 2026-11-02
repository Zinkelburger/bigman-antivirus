---
url: "https://attack.mitre.org/software/S1245/"
title: "InvisibleFerret, Software S1245 | MITRE ATT&CK®"
captured: "2026-11-02T18:23:56Z"
via: "live"
sha256: "29282df6f930be748475d8a5f67b4ad76f377fc9eac1711828482a16433b0da6"
wayback: "https://web.archive.org/web/20261023114623/https://attack.mitre.org/software/S1245/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# InvisibleFerret, Software S1245 | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/software/S1245/>
> Snapshot: <https://web.archive.org/web/20261023114623/https://attack.mitre.org/software/S1245/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Software](https://attack.mitre.org/software/)
- InvisibleFerret

#  InvisibleFerret

[InvisibleFerret](https://attack.mitre.org/software/S1245) is a modular python malware that is leveraged for data exfiltration and remote access capabilities.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) consists of four modules: main, payload, browser, and AnyDesk.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/) [InvisibleFerret](https://attack.mitre.org/software/S1245) malware has been leveraged by North Korea-affiliated threat actors identified as DeceptiveDevelopment or [Contagious Interview](https://attack.mitre.org/groups/G1052) since 2023.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[5]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has historically been introduced to the victim environment through the use of the [BeaverTail](https://attack.mitre.org/software/S1246) malware.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[5]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

ID: S1245

ⓘ

Type: MALWARE

ⓘ

Platforms: Linux, macOS, Windows

Version: 1.0

Created: 17 October 2025

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/software/S1245/)

[Live Version](https://attack.mitre.org/versions/v20/software/S1245/)

###### Enterprise Layer

[download](https://attack.mitre.org/software/S1245/S1245-enterprise-layer.json)  [view](https://attack.mitre.org/software/S1245/#)

## Techniques Used

| Domain  | ID  | Name  | Use

|  Enterprise   |  [T1087](https://attack.mitre.org/techniques/T1087)   |  [.001](https://attack.mitre.org/techniques/T1087/001)   |  [Account Discovery](https://attack.mitre.org/techniques/T1087): [Local Account](https://attack.mitre.org/techniques/T1087/001)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has queried the victim device using Python scripts to obtain the User and Hostname.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1071](https://attack.mitre.org/techniques/T1071)   |  [.001](https://attack.mitre.org/techniques/T1071/001)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Web Protocols](https://attack.mitre.org/techniques/T1071/001)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has used HTTP for C2 communications.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1560](https://attack.mitre.org/techniques/T1560)   |  [.001](https://attack.mitre.org/techniques/T1560/001)   |  [Archive Collected Data](https://attack.mitre.org/techniques/T1560): [Archive via Utility](https://attack.mitre.org/techniques/T1560/001)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has used 7zip, RAR and zip files to archive collected data for exfiltration.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1547](https://attack.mitre.org/techniques/T1547)   |  [.001](https://attack.mitre.org/techniques/T1547/001)   |  [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547): [Registry Run Keys / Startup Folder](https://attack.mitre.org/techniques/T1547/001)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has established persistence within Windows devices by creating a .bat file "queue.bat" within the Startup folder to run a Python script.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|   |   |  [.013](https://attack.mitre.org/techniques/T1547/013)   |  [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547): [XDG Autostart Entries](https://attack.mitre.org/techniques/T1547/013)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has established persistence within GNOME-based Linux environments by placing entries within .desktop that run on Startup.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1115](https://attack.mitre.org/techniques/T1115)   |  [Clipboard Data](https://attack.mitre.org/techniques/T1115)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has stolen data from the clipboard using the Python project "pyperclip".[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also captured clipboard contents during copy and paste operations.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1059](https://attack.mitre.org/techniques/T1059)   |  [.001](https://attack.mitre.org/techniques/T1059/001)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [PowerShell](https://attack.mitre.org/techniques/T1059/001)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has utilized a PowerShell script created in the victim’s home directory named "conf.ps1" that is used to modify configuration files for AnyDesk remote services.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|   |   |  [.006](https://attack.mitre.org/techniques/T1059/006)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Python](https://attack.mitre.org/techniques/T1059/006)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) is written in Python and has used Python scripts for execution.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1543](https://attack.mitre.org/techniques/T1543)   |  [.001](https://attack.mitre.org/techniques/T1543/001)   |  [Create or Modify System Process](https://attack.mitre.org/techniques/T1543): [Launch Agent](https://attack.mitre.org/techniques/T1543/001)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has established persistence using LaunchAgents on macOS that run on Startup using a file named "com.avatar.update.wake.plist".[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1555](https://attack.mitre.org/techniques/T1555)   |  [.003](https://attack.mitre.org/techniques/T1555/003)   |  [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Credentials from Web Browsers](https://attack.mitre.org/techniques/T1555/003)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has stolen login data, autofill data, cryptocurrency wallets, and payment information saved in web browsers such as Chrome, Brave, Opera, Yandex and Edge, to include versions affiliated with major operating systems on Windows, Linux, and macOS.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also leveraged the command ssh_zcp to copy browser data to include extensions and cryptocurrency wallet data.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|   |   |  [.005](https://attack.mitre.org/techniques/T1555/005)   |  [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Password Managers](https://attack.mitre.org/techniques/T1555/005)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has utilized the command ssh_zcp to exfiltrate data from browser extensions and password managers via Telegram and FTP.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1005](https://attack.mitre.org/techniques/T1005)   |  [Data from Local System](https://attack.mitre.org/techniques/T1005)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has collected data utilizing a script that contained a list of excluded files and directory names and naming patterns of interest such as environment and configuration files, documents, spreadsheets, and other files that contained the words secret, wallet, private, and password.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  Enterprise   |  [T1074](https://attack.mitre.org/techniques/T1074)   |  [.001](https://attack.mitre.org/techniques/T1074/001)   |  [Data Staged](https://attack.mitre.org/techniques/T1074): [Local Data Staging](https://attack.mitre.org/techniques/T1074/001)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has staged data in consolidated folders prior to exfiltration.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  Enterprise   |  [T1140](https://attack.mitre.org/techniques/T1140)   |  [Deobfuscate/Decode Files or Information](https://attack.mitre.org/techniques/T1140)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has decoded XOR-encrypted and Base-64-encoded payloads prior to execution.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  Enterprise   |  [T1048](https://attack.mitre.org/techniques/T1048)   |  [.003](https://attack.mitre.org/techniques/T1048/003)   |  [Exfiltration Over Alternative Protocol](https://attack.mitre.org/techniques/T1048): [Exfiltration Over Unencrypted Non-C2 Protocol](https://attack.mitre.org/techniques/T1048/003)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has used FTP to exfiltrate files and directories using the command ssh_upload which contains with six subcommands of .sdira, sdir, sfile, sfinda, sfindr and sfind that had varying functions.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west) [InvisibleFerret](https://attack.mitre.org/software/S1245) has exfiltrated stolen files and data to the C2 servers over ports 1224, 2245 and 8637.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)

|  Enterprise   |  [T1041](https://attack.mitre.org/techniques/T1041)   |  [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has used HTTP communications to the "/Uploads" URI for file exfiltration.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1567](https://attack.mitre.org/techniques/T1567)   |  [Exfiltration Over Web Service](https://attack.mitre.org/techniques/T1567)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has leveraged Telegram chat to upload stolen data using the Telegram API with a bot token.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1083](https://attack.mitre.org/techniques/T1083)   |  [File and Directory Discovery](https://attack.mitre.org/techniques/T1083)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has identified specific directories and files for exfiltration using the ssh_upload command which contains subcommands of .sdira, sdir, sfile, sfinda, sfindr, sfind.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west) [InvisibleFerret](https://attack.mitre.org/software/S1245) also has the capability to scan and upload files of interest from multiple OS systems through the use of scripts that check file names, file extensions, and avoids certain path names.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has utilized the findstr on Windows or the macOS find commands to search for files of interest.[[5]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)

|  Enterprise   |  [T1657](https://attack.mitre.org/techniques/T1657)   |  [Financial Theft](https://attack.mitre.org/techniques/T1657)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has searched the victim device credentials and files commonly associated with cryptocurrency wallets.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1564](https://attack.mitre.org/techniques/T1564)   |  [.003](https://attack.mitre.org/techniques/T1564/003)   |  [Hide Artifacts](https://attack.mitre.org/techniques/T1564): [Hidden Window](https://attack.mitre.org/techniques/T1564/003)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has executed Python instances of the browser module ".n2/bow" utilizing the CREATE_NO_WINDOW process creation flag.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  Enterprise   |  [T1105](https://attack.mitre.org/techniques/T1105)   |  [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has downloaded "AnyDesk.exe" into the user’s home directory from the C2 server when checks for the service fail to identify its presence in the victim environment.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also been configured to download additional payloads using a command which calls to the /bow URI.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1056](https://attack.mitre.org/techniques/T1056)   |  [Input Capture](https://attack.mitre.org/techniques/T1056)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has collected mouse and keyboard events using "pyWinhook".[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|   |   |  [.001](https://attack.mitre.org/techniques/T1056/001)   |  [Keylogging](https://attack.mitre.org/techniques/T1056/001)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has conducted keylogging using the Python project "pyWinHook" and "Pyhook".[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also captured keylogging thread checks for changes in an active window and key presses.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1095](https://attack.mitre.org/techniques/T1095)   |  [Non-Application Layer Protocol](https://attack.mitre.org/techniques/T1095)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has established a connection with the C2 server over TCP traffic.[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also created a TCP reverse shell communicating via a socket connection over ports 1245, 80, 2245, 3001, and 5000.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  Enterprise   |  [T1571](https://attack.mitre.org/techniques/T1571)   |  [Non-Standard Port](https://attack.mitre.org/techniques/T1571)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has been observed utilizing HTTP communications to the C2 server over ports 1224, 2245 and 8637.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)

|  Enterprise   |  [T1027](https://attack.mitre.org/techniques/T1027)   |  [.013](https://attack.mitre.org/techniques/T1027/013)   |  [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Encrypted/Encoded File](https://attack.mitre.org/techniques/T1027/013)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has utilized the XOR and Base64 encoding for each of its modules.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also obfuscated files with a combination of zlib, Base64 and reverse string order.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also utilized the XOR and Base64 encoding some of its Python scripts.[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1057](https://attack.mitre.org/techniques/T1057)   |  [Process Discovery](https://attack.mitre.org/techniques/T1057)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has the capability to query installed programs and running processes.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also identified running processes using the Python project "psutil".[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1219](https://attack.mitre.org/techniques/T1219)   |  [Remote Access Tools](https://attack.mitre.org/techniques/T1219)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has utilized remote access software including AnyDesk client through the "adc" module.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also downloaded the AnyDesk client should it not already exist on the compromised host by searching for C:/Program Files(x86)/AnyDesk/AnyDesk.exe.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1679](https://attack.mitre.org/techniques/T1679)   |  [Selective Exclusion](https://attack.mitre.org/techniques/T1679)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has the capability to scan for file names, file extensions, and avoids pre-designated path names and file types.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1489](https://attack.mitre.org/techniques/T1489)   |  [Service Stop](https://attack.mitre.org/techniques/T1489)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has terminated Chrome and Brave browsers using the taskkill command on Windows and the killall command on other systems such as Linux and macOS.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also utilized it’s ssh_kill command to terminate Chrome and Brave browser processes.[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1518](https://attack.mitre.org/techniques/T1518)   |  [Software Discovery](https://attack.mitre.org/techniques/T1518)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has gathered installed programs and running processes.[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  Enterprise   |  [T1082](https://attack.mitre.org/techniques/T1082)   |  [System Information Discovery](https://attack.mitre.org/techniques/T1082)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has collected OS type, hostname and system version through the "pay" module.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also queried the victim device using Python scripts to obtain the User and Hostname.[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1614](https://attack.mitre.org/techniques/T1614)   |  [System Location Discovery](https://attack.mitre.org/techniques/T1614)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has collected the internal IP address, IP geolocation information of the infected host and sends the data to a C2 server.[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also leveraged the "pay" module to obtain region name, country, city, zip code, ISP, latitude and longitude using "http://ip-api.com/json".[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  Enterprise   |  [T1016](https://attack.mitre.org/techniques/T1016)   |  [System Network Configuration Discovery](https://attack.mitre.org/techniques/T1016)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has collected the local IP address, and external IP.[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

|  Enterprise   |  [T1033](https://attack.mitre.org/techniques/T1033)   |  [System Owner/User Discovery](https://attack.mitre.org/techniques/T1033)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has identified the user’s UUID and username through the "pay" module.[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

## Groups That Use This Software

| ID  | Name  | References

|  [G1052](https://attack.mitre.org/groups/G1052)   |  [Contagious Interview](https://attack.mitre.org/groups/G1052)   |

[[6]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[1]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[2]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)[[3]](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)[[5]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)[[4]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

## References

-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [Seongsu Park. (2024, November 4). From Pyongyang to Your Payroll: The Rise of North Korean Remote Workers in the West. Retrieved October 17, 2025.](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)
-    [Unit 42. (2023, November 21). Hacking Employers and Seeking Employment: Two Job-Related Campaigns Bear Hallmarks of North Korean Threat Actors. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/two-campaigns-by-north-korea-bad-actors-target-job-hunters/)

-    [Insikt Group. (2025, February 13). Inside the Scam: North Korea’s IT Worker Threat. Retrieved October 17, 2025.](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)
-    [Unit42. (2024, October 9). Contagious Interview: DPRK Threat Actors Lure Tech Industry Job Seekers to Install New Variants of BeaverTail and InvisibleFerret Malware. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)
-    [eSentire Threat Response Unit (TRU). (2024, November 14). Bored BeaverTail & InvisibleFerret Yacht Club – A Lazarus Lure Pt.2. Retrieved October 17, 2025.](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)

×
