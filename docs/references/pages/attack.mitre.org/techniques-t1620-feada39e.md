---
url: "https://attack.mitre.org/techniques/T1620/"
title: "Reflective Code Loading, Technique T1620 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T19:04:14Z"
via: "live"
sha256: "57292affbcb5c38985b3e2099ac929bc9bf30e091d2658fd281f9ce9f013ac98"
wayback: "https://web.archive.org/web/20261023025719/https://attack.mitre.org/techniques/T1620/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Reflective Code Loading, Technique T1620 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1620/>
> Snapshot: <https://web.archive.org/web/20261023025719/https://attack.mitre.org/techniques/T1620/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Reflective Code Loading

#  Reflective Code Loading

Adversaries may reflectively load code into a process in order to conceal the execution of malicious payloads. Reflective loading involves allocating then executing payloads directly within the memory of the process, vice creating a thread or process backed by a file path on disk (e.g., [Shared Modules](https://attack.mitre.org/techniques/T1129)).

Reflectively loaded payloads may be compiled binaries, anonymous files (only present in RAM), or just snubs of fileless executable code (ex: position-independent shellcode).[[1]](https://thewover.github.io/Introducing-Donut/)[[2]](https://www.sentinelone.com/blog/building-a-custom-tool-for-shellcode-analysis/)[[3]](https://magisterquis.github.io/2018/03/31/in-memory-only-elf-execution.html)[[4]](https://0x00sec.org/t/super-stealthy-droppers/3715)[[5]](https://www.mandiant.com/resources/bring-your-own-land-novel-red-teaming-technique) For example, the Assembly.Load() method executed by [PowerShell](https://attack.mitre.org/techniques/T1059/001) may be abused to load raw code into the running process.[[6]](https://learn.microsoft.com/dotnet/api/system.reflection.assembly.load)

Reflective code injection is very similar to [Process Injection](https://attack.mitre.org/techniques/T1055) except that the "injection" loads code into the processes’ own memory instead of that of a separate process. Reflective loading may evade process-based detections since the execution of the arbitrary code may be masked within a legitimate or otherwise benign process. Reflectively loading payloads directly into memory may also avoid creating files or other artifacts on disk, while also enabling malware to keep these payloads encrypted (or otherwise obfuscated) until execution.[[3]](https://magisterquis.github.io/2018/03/31/in-memory-only-elf-execution.html)[[4]](https://0x00sec.org/t/super-stealthy-droppers/3715)[[7]](https://intezer.com/acbackdoor-analysis-of-a-new-multiplatform-backdoor/)[[8]](https://www.sentinelone.com/blog/teaching-an-old-rat-new-tricks/)

ID: T1620

Sub-techniques:  No sub-techniques

ⓘ

Tactic: [Stealth](https://attack.mitre.org/tactics/TA0005)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Jiraput Thamsongkrah; Joas Antonio dos Santos, @C0d3Cr4zy, Inmetrics; João Paulo de A. Filho, @Hug1nN__; Lior Ribak, SentinelOne; Rex Guo, @Xiaofei_REX, Confluera; Shlomi Salem, SentinelOne

Version: 2.0

Created: 05 October 2021

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1620/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1620/)

## Procedure Examples

| ID  | Name  | Description

|  [C0057](https://attack.mitre.org/campaigns/C0057)   |  [3CX Supply Chain Attack](https://attack.mitre.org/campaigns/C0057)   |

During the [3CX Supply Chain Attack](https://attack.mitre.org/campaigns/C0057), [AppleJeus](https://attack.mitre.org/groups/G1049) leverages the publicly available open-source project DAVESHELL to convert PE-COFF files to position-independent code to reflectively load the payload into memory.[[9]](https://cloud.google.com/blog/topics/threat-intelligence/3cx-software-supply-chain-compromise/)[[10]](https://github.com/monoxgas/sRDI)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has configured malicious payloads to reflectively load using the macOS NSCreateObjectFileImageFromMemory API.[[11]](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)
[APT38](https://attack.mitre.org/groups/G0082) has also reflectively loaded .NET DLLs to bypass disk-based detection.[[12]](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)

|  [S1081](https://attack.mitre.org/software/S1081)   |  [BADHATCH](https://attack.mitre.org/software/S1081)   |

[BADHATCH](https://attack.mitre.org/software/S1081) can copy a large byte array of 64-bit shellcode into process memory and execute it with a call to CreateThread.[[13]](https://blog.gigamon.com/2019/07/23/abadbabe-8badf00d-discovering-badhatch-and-a-detailed-look-at-fin8s-tooling/)

|  [S9011](https://attack.mitre.org/software/S9011)   |  [BRUSHFIRE](https://attack.mitre.org/software/S9011)   |

[BRUSHFIRE](https://attack.mitre.org/software/S9011) has executed its commands within memory and is not saved on disk.[[14]](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-exploiting-critical-ivanti-vulnerability)[[15]](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)

|  [S1063](https://attack.mitre.org/software/S1063)   |  [Brute Ratel C4](https://attack.mitre.org/software/S1063)   |

[Brute Ratel C4](https://attack.mitre.org/software/S1063) has used reflective loading to execute malicious DLLs.[[16]](https://www.mdsec.co.uk/2022/08/part-3-how-i-met-your-beacon-brute-ratel/)

|  [S0154](https://attack.mitre.org/software/S0154)   |  [Cobalt Strike](https://attack.mitre.org/software/S0154)   |

[Cobalt Strike](https://attack.mitre.org/software/S0154)'s execute-assembly command can run a .NET executable within the memory of a sacrificial process by loading the CLR.[[17]](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)

|  [S0625](https://attack.mitre.org/software/S0625)   |  [Cuba](https://attack.mitre.org/software/S0625)   |

[Cuba](https://attack.mitre.org/software/S0625) loaded the payload into memory using PowerShell.[[18]](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-cuba-ransomware.pdf)

|  [S0695](https://attack.mitre.org/software/S0695)   |  [Donut](https://attack.mitre.org/software/S0695)   |

[Donut](https://attack.mitre.org/software/S0695) can generate code modules that enable in-memory execution of VBScript, JScript, EXE, DLL, and dotNET payloads.[[19]](https://github.com/TheWover/donut)

|  [S0367](https://attack.mitre.org/software/S0367)   |  [Emotet](https://attack.mitre.org/software/S0367)   |

[Emotet](https://attack.mitre.org/software/S0367) has reflectively loaded payloads into memory.[[20]](https://www.binarydefense.com/resources/blog/emotet-evolves-with-new-wi-fi-spreader/)

|  [G0046](https://attack.mitre.org/groups/G0046)   |  [FIN7](https://attack.mitre.org/groups/G0046)   |

[FIN7](https://attack.mitre.org/groups/G0046) has loaded a .NET assembly into the currect execution context via Reflection.Assembly::Load.[[21]](https://geminiadvisory.io/fin7-flash-drives-spread-remote-access-trojan/)

|  [S0661](https://attack.mitre.org/software/S0661)   |  [FoggyWeb](https://attack.mitre.org/software/S0661)   |

[FoggyWeb](https://attack.mitre.org/software/S0661)'s loader has reflectively loaded .NET-based assembly/payloads into memory.[[22]](https://www.microsoft.com/security/blog/2021/09/27/foggyweb-targeted-nobelium-malware-leads-to-persistent-backdoor/)

|  [S9033](https://attack.mitre.org/software/S9033)   |  [Fooder](https://attack.mitre.org/software/S9033)   |

[Fooder](https://attack.mitre.org/software/S9033) has reflectively loaded a payload into memory.[[23]](https://www.welivesecurity.com/en/eset-research/muddywater-snakes-riverbank/)

|  [G0047](https://attack.mitre.org/groups/G0047)   |  [Gamaredon Group](https://attack.mitre.org/groups/G0047)   |

[Gamaredon Group](https://attack.mitre.org/groups/G0047) has used an obfuscated PowerShell script that used System.Reflection.Assembly to gather and send victim information to the C2.[[24]](https://www.security.com/threat-intelligence/shuckworm-ukraine-gammasteel)

|  [S0666](https://attack.mitre.org/software/S0666)   |  [Gelsemium](https://attack.mitre.org/software/S0666)   |

[Gelsemium](https://attack.mitre.org/software/S0666) can use custom shellcode to map embedded DLLs into memory.[[25]](https://www.welivesecurity.com/wp-content/uploads/2021/06/eset_gelsemium.pdf)

|  [S1022](https://attack.mitre.org/software/S1022)   |  [IceApple](https://attack.mitre.org/software/S1022)   |

[IceApple](https://attack.mitre.org/software/S1022) can use reflective code loading to load .NET assemblies into MSExchangeOWAAppPool on targeted Exchange servers.[[26]](https://www.crowdstrike.com/wp-content/uploads/2022/05/crowdstrike-iceapple-a-novel-internet-information-services-post-exploitation-framework.pdf)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has used the Invoke-Mimikatz PowerShell script to reflectively load a Mimikatz credential stealing DLL into memory.[[27]](https://services.google.com/fh/files/misc/apt43-report-en.pdf) [Kimsuky](https://attack.mitre.org/groups/G0094) has also used reflective loading through .NET assembly using [System.Reflection.Assembly]::Load.[[28]](https://www.securonix.com/blog/analyzing-deepdrive-north-korean-threat-actors-observed-exploiting-trusted-platforms-for-targeted-attacks/)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) has changed memory protection permissions then overwritten in memory DLL function code with shellcode, which was later executed via [KernelCallbackTable](https://attack.mitre.org/techniques/T1574/013) hijacking. [Lazarus Group](https://attack.mitre.org/groups/G0032) has also used shellcode within macros to decrypt and manually map DLLs into memory at runtime.[[29]](https://blog.malwarebytes.com/threat-intelligence/2022/01/north-koreas-lazarus-apt-leverages-windows-update-client-github-in-latest-campaign/)[[30]](https://blog.qualys.com/vulnerabilities-threat-research/2022/02/08/lolzarus-lazarus-group-incorporating-lolbins-into-campaigns)

|  [S0681](https://attack.mitre.org/software/S0681)   |  [Lizar](https://attack.mitre.org/software/S0681)   |

[Lizar](https://attack.mitre.org/software/S0681) has used the Reflective DLL injection module from Github to inject itself into a process’s memory.[[31]](https://blog.sekoia.io/unveiling-the-intricacies-of-diceloader/)

|  [S0447](https://attack.mitre.org/software/S0447)   |  [Lokibot](https://attack.mitre.org/software/S0447)   |

[Lokibot](https://attack.mitre.org/software/S0447) has reflectively loaded the decoded DLL into memory.[[32]](https://blog.talosintelligence.com/2021/01/a-deep-dive-into-lokibot-infection-chain.html)

|  [S1213](https://attack.mitre.org/software/S1213)   |  [Lumma Stealer](https://attack.mitre.org/software/S1213)   |

[Lumma Stealer](https://attack.mitre.org/software/S1213) has used reflective loading techniques to load content into memory during execution.[[33]](https://www.netskope.com/blog/lumma-stealer-fake-captchas-new-techniques-to-evade-detection)[[34]](https://www.fortinet.com/blog/threat-research/lumma-variant-on-youtube)

|  [S1143](https://attack.mitre.org/software/S1143)   |  [LunarLoader](https://attack.mitre.org/software/S1143)   |

[LunarLoader](https://attack.mitre.org/software/S1143) can use reflective loading to decrypt and run malicious executables in a new thread.[[35]](https://www.welivesecurity.com/en/eset-research/moon-backdoors-lunar-landing-diplomatic-missions/)

|  [S1059](https://attack.mitre.org/software/S1059)   |  [metaMain](https://attack.mitre.org/software/S1059)   |

[metaMain](https://attack.mitre.org/software/S1059) has reflectively loaded a DLL to read, decrypt, and load an orchestrator file.[[36]](https://assets.sentinelone.com/sentinellabs22/metador#page=1)

|  [S9032](https://attack.mitre.org/software/S9032)   |  [MuddyViper](https://attack.mitre.org/software/S9032)   |

[MuddyViper](https://attack.mitre.org/software/S9032) has reflectively loaded the decrypted HackBrowserData tool in a new thread.[[23]](https://www.welivesecurity.com/en/eset-research/muddywater-snakes-riverbank/)

|  [S1145](https://attack.mitre.org/software/S1145)   |  [Pikabot](https://attack.mitre.org/software/S1145)   |

[Pikabot](https://attack.mitre.org/software/S1145) reflectively loads stored, previously encrypted components of the PE file into memory of the currently executing process to avoid writing content to disk on the executing machine.[[37]](https://www.elastic.co/security-labs/pikabot-i-choose-you)

|  [S0013](https://attack.mitre.org/software/S0013)   |  [PlugX](https://attack.mitre.org/software/S0013)   |

[PlugX](https://attack.mitre.org/software/S0013) has loaded its payload into memory.[[38]](https://www.welivesecurity.com/2022/03/23/mustang-panda-hodur-old-tricks-new-korplug-variant/)[[39]](https://blog.eclecticiq.com/mustang-panda-apt-group-uses-european-commission-themed-lure-to-deliver-plugx-malware)[[40]](https://cloud.google.com/blog/topics/threat-intelligence/prc-nexus-espionage-targets-diplomats)[[41]](https://www.secureworks.com/blog/bronze-president-targets-russian-speakers-with-updated-plugx)[[42]](https://www.secureworks.com/blog/bronze-president-targets-government-officials)

|  [S0194](https://attack.mitre.org/software/S0194)   |  [PowerSploit](https://attack.mitre.org/software/S0194)   |

[PowerSploit](https://attack.mitre.org/software/S0194) reflectively loads a Windows PE file into a process.[[43]](https://github.com/PowerShellMafia/PowerSploit)[[44]](http://powersploit.readthedocs.io)

|  [S1085](https://attack.mitre.org/software/S1085)   |  [Sardonic](https://attack.mitre.org/software/S1085)   |

[Sardonic](https://attack.mitre.org/software/S1085) has a plugin system that can load specially made DLLs into memory and execute their functions.[[45]](https://www.bitdefender.com/files/News/CaseStudies/study/401/Bitdefender-PR-Whitepaper-FIN8-creat5619-en-EN.pdf)[[46]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/syssphinx-fin8-backdoor)

|  [C0058](https://attack.mitre.org/campaigns/C0058)   |  [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058)   |

During [SharePoint ToolShell Exploitation](https://attack.mitre.org/campaigns/C0058), threat actors reflectively loaded payloads using System.Reflection.Assembly.Load.[[47]](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)[[48]](https://research.eye.security/sharepoint-under-siege/)[[49]](https://www.trendmicro.com/en_us/research/25/g/cve-2025-53770-and-cve-2025-53771-sharepoint-attacks.html)[[50]](https://www.sentinelone.com/blog/sharepoint-toolshell-zero-day-exploited-in-the-wild-targets-enterprise-servers/)[[51]](https://unit42.paloaltonetworks.com/microsoft-sharepoint-cve-2025-49704-cve-2025-49706-cve-2025-53770/)

|  [S0692](https://attack.mitre.org/software/S0692)   |  [SILENTTRINITY](https://attack.mitre.org/software/S0692)   |

[SILENTTRINITY](https://attack.mitre.org/software/S0692) can run a .NET executable within the memory of a sacrificial process by loading the CLR.[[52]](https://github.com/byt3bl33d3r/SILENTTRINITY)

|  [S9001](https://attack.mitre.org/software/S9001)   |  [SystemBC](https://attack.mitre.org/software/S9001)   |

[SystemBC](https://attack.mitre.org/software/S9001) has downloaded a text file into memory and set the area of memory via the VirtualProtect call. Then, [SystemBC](https://attack.mitre.org/software/S9001) has executed the file via the CreateThread call.[[53]](https://www.kroll.com/en/publications/cyber/inside-the-systembc-malware-server)

|  [S0595](https://attack.mitre.org/software/S0595)   |  [ThiefQuest](https://attack.mitre.org/software/S0595)   |

[ThiefQuest](https://attack.mitre.org/software/S0595) uses various API functions such as NSCreateObjectFileImageFromMemory to load and link in-memory payloads.[[54]](https://objective-see.com/blog/blog_0x60.html)

|  [S0022](https://attack.mitre.org/software/S0022)   |  [Uroburos](https://attack.mitre.org/software/S0022)   |

[Uroburos](https://attack.mitre.org/software/S0022) has the ability to load new modules directly into memory using its Load Modules Mem command.[[55]](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)

|  [S0689](https://attack.mitre.org/software/S0689)   |  [WhisperGate](https://attack.mitre.org/software/S0689)   |

[WhisperGate](https://attack.mitre.org/software/S0689)'s downloader can reverse its third stage file bytes and reflectively load the file as a .NET assembly.[[56]](https://www.recordedfuture.com/research/whispergate-malware-corrupts-computers-ukraine)

## Mitigations

This type of attack technique cannot be easily mitigated with preventive controls since it is based on the abuse of system features.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0300](https://attack.mitre.org/detectionstrategies/DET0300)   |  [Detection Strategy for Reflective Code Loading](https://attack.mitre.org/detectionstrategies/DET0300)   |  [AN0838](https://attack.mitre.org/detectionstrategies/DET0300#AN0838)   |

Detect anomalous chains of memory allocation and execution inside the same process (e.g., VirtualAlloc → memcpy → VirtualProtect → CreateThread). Unlike process injection, reflective code loading does not perform cross-process memory writes — the suspicious activity occurs entirely within the process’s own PID context.

|  [AN0839](https://attack.mitre.org/detectionstrategies/DET0300#AN0839)   |

Monitor for in-process mmap + mprotect + execve/execveat activity where memory permissions are changed from writable to executable inside the same process without a corresponding ELF on disk.

|  [AN0840](https://attack.mitre.org/detectionstrategies/DET0300#AN0840)   |

Suspicious calls to dlopen(), dlsym(), or mmap with RWX flags in processes that do not typically perform dynamic module loading. Monitor anonymous memory regions executed by user processes.

## References

-    [The Wover. (2019, May 9). Donut - Injecting .NET Assemblies as Shellcode. Retrieved October 4, 2021.](https://thewover.github.io/Introducing-Donut/)
-    [Bunce, D. (2019, October 31). Building A Custom Tool For Shellcode Analysis. Retrieved October 4, 2021.](https://www.sentinelone.com/blog/building-a-custom-tool-for-shellcode-analysis/)
-    [Stuart. (2018, March 31). In-Memory-Only ELF Execution (Without tmpfs). Retrieved October 4, 2021.](https://magisterquis.github.io/2018/03/31/in-memory-only-elf-execution.html)
-    [0x00pico. (2017, September 25). Super-Stealthy Droppers. Retrieved October 4, 2021.](https://0x00sec.org/t/super-stealthy-droppers/3715)
-    [Kirk, N. (2018, June 18). Bring Your Own Land (BYOL) – A Novel Red Teaming Technique. Retrieved October 4, 2021.](https://www.mandiant.com/resources/bring-your-own-land-novel-red-teaming-technique)
-    [Microsoft. (n.d.). Assembly.Load Method. Retrieved February 9, 2024.](https://learn.microsoft.com/dotnet/api/system.reflection.assembly.load)
-    [Sanmillan, I. (2019, November 18). ACBackdoor: Analysis of a New Multiplatform Backdoor. Retrieved October 4, 2021.](https://intezer.com/acbackdoor-analysis-of-a-new-multiplatform-backdoor/)
-    [Landry, J. (2016, April 21). Teaching an old RAT new tricks. Retrieved October 4, 2021.](https://www.sentinelone.com/blog/teaching-an-old-rat-new-tricks/)
-    [Jeff Johnson, Fred Plan, Adrian Sanchez, Renato Fontana, Jake Nicastro, Dimiter Andonov, Marius Fodoreanu, Daniel Scott. (2023, April 20). 3CX Software Supply Chain Compromise Initiated by a Prior Software Supply Chain Compromise; Suspected North Korean Actor Responsible. Retrieved August 25, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/3cx-software-supply-chain-compromise/)
-    [Nick Landers (monoxgas). (2022, June 18). GitHub monoxgas sRDI (DAVESHELL). Retrieved October 1, 2025.](https://github.com/monoxgas/sRDI)
-    [Microsoft Threat Intelligence. (2026, April 16). Dissecting Sapphire Sleet’s macOS intrusion from lure to compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)
-    [Microsoft Threat Intelligence. (2026, June 17). From package to postinstall payload: Inside the Mastra npm supply chain compromise by Sapphire Sleet. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)
-    [Savelesky, K., et al. (2019, July 23). ABADBABE 8BADFOOD: Discovering BADHATCH and a Detailed Look at FIN8's Tooling. Retrieved September 8, 2021.](https://blog.gigamon.com/2019/07/23/abadbabe-8badf00d-discovering-badhatch-and-a-detailed-look-at-fin8s-tooling/)
-    [John Wolfram, Michael Edie, Jacob Thompson, Matt Lin, Josh Murchie. (2025, April 3). Suspected China-Nexus Threat Actor Actively Exploiting Critical Ivanti Connect Secure Vulnerability (CVE-2025-22457). Retrieved April 13, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/china-nexus-exploiting-critical-ivanti-vulnerability)
-    [Sila Ozeren Hacioglu. (2025, May 5). UNC5221’s Latest Exploit: Weaponizing CVE-2025-22457 in Ivanti Connect Secure. Retrieved April 13, 2026.](https://www.picussecurity.com/resource/blog/unc5221-cve-2025-22457-ivanti-connect-secure)
-    [Chell, D. PART 3: How I Met Your Beacon – Brute Ratel. Retrieved February 6, 2023.](https://www.mdsec.co.uk/2022/08/part-3-how-i-met-your-beacon-brute-ratel/)
-    [Strategic Cyber LLC. (2020, November 5). Cobalt Strike: Advanced Threat Tactics for Penetration Testers. Retrieved April 13, 2021.](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)
-    [Roccio, T., et al. (2021, April). Technical Analysis of Cuba Ransomware. Retrieved June 18, 2021.](https://www.mcafee.com/enterprise/en-us/assets/reports/rp-cuba-ransomware.pdf)
-    [TheWover. (2019, May 9). donut. Retrieved March 25, 2022.](https://github.com/TheWover/donut)
-    [Binary Defense. (n.d.). Emotet Evolves With new Wi-Fi Spreader. Retrieved September 8, 2023.](https://www.binarydefense.com/resources/blog/emotet-evolves-with-new-wi-fi-spreader/)
-    [Gemini Advisory. (2022, January 13). FIN7 Uses Flash Drives to Spread Remote Access Trojan. Retrieved May 14, 2025.](https://geminiadvisory.io/fin7-flash-drives-spread-remote-access-trojan/)
-    [Ramin Nafisi. (2021, September 27). FoggyWeb: Targeted NOBELIUM malware leads to persistent backdoor. Retrieved October 4, 2021.](https://www.microsoft.com/security/blog/2021/09/27/foggyweb-targeted-nobelium-malware-leads-to-persistent-backdoor/)
-    [ESET Research. (2025, December 2). MuddyWater: Snakes by the riverbank. Retrieved February 17, 2026.](https://www.welivesecurity.com/en/eset-research/muddywater-snakes-riverbank/)
-    [Threat Hunter Team, Symantec and Carbon Black. (2025, April 10). Shuckworm Targets Foreign Military Mission Based in Ukraine. Retrieved July 23, 2025.](https://www.security.com/threat-intelligence/shuckworm-ukraine-gammasteel)
-    [Dupuy, T. and Faou, M. (2021, June). Gelsemium. Retrieved November 30, 2021.](https://www.welivesecurity.com/wp-content/uploads/2021/06/eset_gelsemium.pdf)
-    [CrowdStrike. (2022, May). ICEAPPLE: A NOVEL INTERNET INFORMATION SERVICES (IIS) POST-EXPLOITATION FRAMEWORK. Retrieved June 27, 2022.](https://www.crowdstrike.com/wp-content/uploads/2022/05/crowdstrike-iceapple-a-novel-internet-information-services-post-exploitation-framework.pdf)
-    [Mandiant. (2024, March 14). APT43: North Korean Group Uses Cybercrime to Fund Espionage Operations. Retrieved May 3, 2024.](https://services.google.com/fh/files/misc/apt43-report-en.pdf)
-    [Den Iuzvyk, Tim Peck. (2025, February 13). Analyzing DEEP#DRIVE: North Korean Threat Actors Observed Exploiting Trusted Platforms for Targeted Attacks. Retrieved August 19, 2025.](https://www.securonix.com/blog/analyzing-deepdrive-north-korean-threat-actors-observed-exploiting-trusted-platforms-for-targeted-attacks/)

-    [Saini, A. and Hossein, J. (2022, January 27). North Korea’s Lazarus APT leverages Windows Update client, GitHub in latest campaign. Retrieved January 27, 2022.](https://blog.malwarebytes.com/threat-intelligence/2022/01/north-koreas-lazarus-apt-leverages-windows-update-client-github-in-latest-campaign/)
-    [Pradhan, A. (2022, February 8). LolZarus: Lazarus Group Incorporating Lolbins into Campaigns. Retrieved March 22, 2022.](https://blog.qualys.com/vulnerabilities-threat-research/2022/02/08/lolzarus-lazarus-group-incorporating-lolbins-into-campaigns)
-    [Bourhis, P., Sekoia TDR. (2024, February 1). Unveiling the intricacies of DiceLoader. Retrieved May 14, 2025.](https://blog.sekoia.io/unveiling-the-intricacies-of-diceloader/)
-    [Muhammad, I., Unterbrink, H.. (2021, January 6). A Deep Dive into Lokibot Infection Chain. Retrieved August 31, 2021.](https://blog.talosintelligence.com/2021/01/a-deep-dive-into-lokibot-infection-chain.html)
-    [Leandro Fróes, Netskope. (2025, January 23). Lumma Stealer: Fake CAPTCHAs & New Techniques to Evade Detection. Retrieved March 22, 2025.](https://www.netskope.com/blog/lumma-stealer-fake-captchas-new-techniques-to-evade-detection)
-    [Cara Lin, Fortinet. (2024, January 8). Deceptive Cracked Software Spreads Lumma Variant on YouTube. Retrieved March 22, 2025.](https://www.fortinet.com/blog/threat-research/lumma-variant-on-youtube)
-    [Jurčacko, F. (2024, May 15). To the Moon and back(doors): Lunar landing in diplomatic missions. Retrieved June 26, 2024.](https://www.welivesecurity.com/en/eset-research/moon-backdoors-lunar-landing-diplomatic-missions/)
-    [Ehrlich, A., et al. (2022, September). THE MYSTERY OF METADOR | AN UNATTRIBUTED THREAT HIDING IN TELCOS, ISPS, AND UNIVERSITIES. Retrieved January 23, 2023.](https://assets.sentinelone.com/sentinellabs22/metador#page=1)
-    [Daniel Stepanic & Salim Bitam. (2024, February 23). PIKABOT, I choose you!. Retrieved July 12, 2024.](https://www.elastic.co/security-labs/pikabot-i-choose-you)
-    [Alexandre Cote Cyr. (2022, March 23). Mustang Panda’s Hodur: Old tricks, new Korplug variant. Retrieved September 9, 2025.](https://www.welivesecurity.com/2022/03/23/mustang-panda-hodur-old-tricks-new-korplug-variant/)
-    [EclecticIQ Threat Research Team. (2023, February 2). Mustang Panda APT Group Uses European Commission-Themed Lure to Deliver PlugX Malware. Retrieved September 9, 2025.](https://blog.eclecticiq.com/mustang-panda-apt-group-uses-european-commission-themed-lure-to-deliver-plugx-malware)
-    [Patrick Whitsell. (2025, August 25). Deception in Depth: PRC-Nexus Espionage Campaign Hijacks Web Traffic to Target Diplomats. Retrieved September 9, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/prc-nexus-espionage-targets-diplomats)
-    [Secureworks Counter Threat Unit Research Team. (2022, April 27). BRONZE PRESIDENT Targets Russian Speakers with Updated PlugX. Retrieved September 9, 2025.](https://www.secureworks.com/blog/bronze-president-targets-russian-speakers-with-updated-plugx)
-    [Secureworks Counter Threat Unit Research Team. (2022, September 8). BRONZE PRESIDENT Targets Government Officials. Retrieved September 9, 2025.](https://www.secureworks.com/blog/bronze-president-targets-government-officials)
-    [PowerShellMafia. (2012, May 26). PowerSploit - A PowerShell Post-Exploitation Framework. Retrieved February 6, 2018.](https://github.com/PowerShellMafia/PowerSploit)
-    [PowerSploit. (n.d.). PowerSploit. Retrieved February 6, 2018.](http://powersploit.readthedocs.io)
-    [Budaca, E., et al. (2021, August 25). FIN8 Threat Actor Goes Agile with New Sardonic Backdoor. Retrieved August 9, 2023.](https://www.bitdefender.com/files/News/CaseStudies/study/401/Bitdefender-PR-Whitepaper-FIN8-creat5619-en-EN.pdf)
-    [Symantec Threat Hunter Team. (2023, July 18). FIN8 Uses Revamped Sardonic Backdoor to Deliver Noberus Ransomware. Retrieved August 9, 2023.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/syssphinx-fin8-backdoor)
-    [Microsoft Threat Intelligence. (2025, July 22). Disrupting active exploitation of on-premises SharePoint vulnerabilities. Retrieved October 15, 2025.](https://www.microsoft.com/en-us/security/blog/2025/07/22/disrupting-active-exploitation-of-on-premises-sharepoint-vulnerabilities/#storm-2603)
-    [Eye Security. (2025, July 19). SharePoint Under Siege: ToolShell Exploit (CVE-2025-49706 & CVE-2025-49704). Retrieved October 15, 2025.](https://research.eye.security/sharepoint-under-siege/)
-    [Trend Micro Research. (2022, July 22). Proactive Security Insights for SharePoint Attacks (CVE-2025-53770 and CVE-2025-53771). Retrieved October 15, 2025.](https://www.trendmicro.com/en_us/research/25/g/cve-2025-53770-and-cve-2025-53771-sharepoint-attacks.html)
-    [Kenin, S. et al. (2025, July 21). SharePoint ToolShell | Zero-Day Exploited in-the-Wild Targets Enterprise Servers. Retrieved October 15, 2025.](https://www.sentinelone.com/blog/sharepoint-toolshell-zero-day-exploited-in-the-wild-targets-enterprise-servers/)
-    [Unit 42. (2025, July 31). Active Exploitation of Microsoft SharePoint Vulnerabilities: Threat Brief (Updated). Retrieved October 15, 2025.](https://unit42.paloaltonetworks.com/microsoft-sharepoint-cve-2025-49704-cve-2025-49706-cve-2025-53770/)
-    [byt3bl33d3r. (n.d.). SILENTTRINITY. Retrieved September 12, 2024.](https://github.com/byt3bl33d3r/SILENTTRINITY)
-    [Truman, D. (2024, January 19). Inside the SYSTEMBC Command-and-Control Server. Retrieved June 18, 2025.](https://www.kroll.com/en/publications/cyber/inside-the-systembc-malware-server)
-    [Patrick Wardle. (2020, July 3). OSX.EvilQuest Uncovered part ii: insidious capabilities. Retrieved March 21, 2021.](https://objective-see.com/blog/blog_0x60.html)
-    [FBI et al. (2023, May 9). Hunting Russian Intelligence “Snake” Malware. Retrieved June 8, 2023.](https://www.cisa.gov/sites/default/files/2023-05/aa23-129a_snake_malware_2.pdf)
-    [Insikt Group. (2020, January 28). WhisperGate Malware Corrupts Computers in Ukraine. Retrieved September 16, 2024.](https://www.recordedfuture.com/research/whispergate-malware-corrupts-computers-ukraine)

×
