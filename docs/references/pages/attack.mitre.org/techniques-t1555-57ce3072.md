---
url: "https://attack.mitre.org/techniques/T1555/"
title: "Credentials from Password Stores, Technique T1555 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:53:36Z"
via: "live"
sha256: "a671816f6525f22bb6e5ae1628c368df12319b2715460cf98703ff76d6e028b7"
wayback: "https://web.archive.org/web/20261023090821/https://attack.mitre.org/techniques/T1555/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Credentials from Password Stores, Technique T1555 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1555/>
> Snapshot: <https://web.archive.org/web/20261023090821/https://attack.mitre.org/techniques/T1555/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- Credentials from Password Stores

#  Credentials from Password Stores

##### Sub-techniques (6)

| ID  | Name

|  [T1555.001](https://attack.mitre.org/techniques/T1555/001/)   |  [Keychain](https://attack.mitre.org/techniques/T1555/001/)

|  [T1555.002](https://attack.mitre.org/techniques/T1555/002/)   |  [Securityd Memory](https://attack.mitre.org/techniques/T1555/002/)

|  [T1555.003](https://attack.mitre.org/techniques/T1555/003/)   |  [Credentials from Web Browsers](https://attack.mitre.org/techniques/T1555/003/)

|  [T1555.004](https://attack.mitre.org/techniques/T1555/004/)   |  [Windows Credential Manager](https://attack.mitre.org/techniques/T1555/004/)

|  [T1555.005](https://attack.mitre.org/techniques/T1555/005/)   |  [Password Managers](https://attack.mitre.org/techniques/T1555/005/)

|  [T1555.006](https://attack.mitre.org/techniques/T1555/006/)   |  [Cloud Secrets Management Stores](https://attack.mitre.org/techniques/T1555/006/)

Adversaries may search for common password storage locations to obtain user credentials.[[1]](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf) Passwords are stored in several places on a system, depending on the operating system or application holding the credentials. There are also specific applications and services that store passwords to make them easier for users to manage and maintain, such as password managers and cloud secrets vaults. Once credentials are obtained, they can be used to perform lateral movement and access restricted information.

ID: T1555

Sub-techniques:  [T1555.001](https://attack.mitre.org/techniques/T1555/001), [T1555.002](https://attack.mitre.org/techniques/T1555/002), [T1555.003](https://attack.mitre.org/techniques/T1555/003), [T1555.004](https://attack.mitre.org/techniques/T1555/004), [T1555.005](https://attack.mitre.org/techniques/T1555/005), [T1555.006](https://attack.mitre.org/techniques/T1555/006)

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: IaaS, Linux, Windows, macOS

Version: 1.2

Created: 11 February 2020

Last Modified: 12 May 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1555/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1555/)

## Procedure Examples

| ID  | Name  | Description

|  [C0063](https://attack.mitre.org/campaigns/C0063)   |  [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063)   |

During the [2025 Poland Wiper Attacks](https://attack.mitre.org/campaigns/C0063), the adversaries configured a native CLI to gather a targeted elevated users password using grep.[[2]](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)

|  [S0331](https://attack.mitre.org/software/S0331)   |  [Agent Tesla](https://attack.mitre.org/software/S0331)   |

[Agent Tesla](https://attack.mitre.org/software/S0331) has the ability to steal credentials from FTP clients and wireless profiles.[[3]](https://blog.malwarebytes.com/threat-analysis/2020/04/new-agenttesla-variant-steals-wifi-credentials/)

|  [C0064](https://attack.mitre.org/campaigns/C0064)   |  [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064)   |

During the [AI-Assisted Breach of Mexico's Government Infrastructure](https://attack.mitre.org/campaigns/C0064), the adversary extracted credentials from a database and tested those passwords against victim environments and services.[[4]](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)

|  [G0064](https://attack.mitre.org/groups/G0064)   |  [APT33](https://attack.mitre.org/groups/G0064)   |

[APT33](https://attack.mitre.org/groups/G0064) has used a variety of publicly available tools like [LaZagne](https://attack.mitre.org/software/S0349) to gather credentials.[[5]](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)[[6]](https://www.fireeye.com/blog/threat-research/2018/12/overruled-containing-a-potentially-destructive-adversary.html)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has stolen credentials from the compromised device to include those used to access cryptocurrency wallets.[[7]](https://www.microsoft.com/en-us/security/blog/2024/11/22/microsoft-shares-latest-intelligence-on-north-korean-and-chinese-threat-actors-at-cyberwarcon/)[[8]](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)

|  [G0087](https://attack.mitre.org/groups/G0087)   |  [APT39](https://attack.mitre.org/groups/G0087)   |

[APT39](https://attack.mitre.org/groups/G0087) has used the Smartftp Password Decryptor tool to decrypt FTP passwords.[[9]](https://www.bitdefender.com/blog/labs/iranian-chafer-apt-targeted-air-transportation-and-government-in-kuwait-and-saudi-arabia/)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) has obtained information about accounts, lists of employees, and plaintext and hashed passwords from databases.[[10]](https://www.group-ib.com/blog/apt41-world-tour-2021/)

|  [S0373](https://attack.mitre.org/software/S0373)   |  [Astaroth](https://attack.mitre.org/software/S0373)   |

[Astaroth](https://attack.mitre.org/software/S0373) uses an external software known as NetPass to recover passwords. [[11]](https://www.cybereason.com/blog/information-stealing-malware-targeting-brazil-full-research)

|  [S1246](https://attack.mitre.org/software/S1246)   |  [BeaverTail](https://attack.mitre.org/software/S1246)   |

[BeaverTail](https://attack.mitre.org/software/S1246) has collected keys stored for Solana stored in .config/solana/id.json and other login details associated with macOS within /Library/Keychains/login.keychain or for Linux within /.local/share/keyrings.[[12]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  [S0484](https://attack.mitre.org/software/S0484)   |  [Carberp](https://attack.mitre.org/software/S0484)   |

[Carberp](https://attack.mitre.org/software/S0484)'s passw.plug plugin can gather account information from multiple instant messaging, email, and social media services, as well as FTP, VNC, and VPN clients.[[13]](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)

|  [S0050](https://attack.mitre.org/software/S0050)   |  [CosmicDuke](https://attack.mitre.org/software/S0050)   |

[CosmicDuke](https://attack.mitre.org/software/S0050) collects user credentials, including passwords, for various programs including popular instant messaging applications and email clients as well as WLAN keys.[[1]](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)

|  [S1111](https://attack.mitre.org/software/S1111)   |  [DarkGate](https://attack.mitre.org/software/S1111)   |

[DarkGate](https://attack.mitre.org/software/S1111) use Nirsoft Network Password Recovery or NetPass tools to steal stored RDP credentials in some malware versions.[[14]](https://www.trellix.com/blogs/research/the-continued-evolution-of-the-darkgate-malware-as-a-service/)

|  [G0120](https://attack.mitre.org/groups/G0120)   |  [Evilnum](https://attack.mitre.org/groups/G0120)   |

[Evilnum](https://attack.mitre.org/groups/G0120) can collect email credentials from victims.[[15]](https://www.welivesecurity.com/2020/07/09/more-evil-deep-look-evilnum-toolset/)

|  [G0037](https://attack.mitre.org/groups/G0037)   |  [FIN6](https://attack.mitre.org/groups/G0037)   |

[FIN6](https://attack.mitre.org/groups/G0037) has used the Stealer One credential stealer to target e-mail and file transfer utilities including FTP.[[16]](https://usa.visa.com/dam/VCOM/global/support-legal/documents/fin6-cybercrime-group-expands-threat-To-ecommerce-merchants.pdf)

|  [G1001](https://attack.mitre.org/groups/G1001)   |  [HEXANE](https://attack.mitre.org/groups/G1001)   |

[HEXANE](https://attack.mitre.org/groups/G1001) has run cmdkey on victim machines to identify stored credentials.[[17]](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)

|  [S0526](https://attack.mitre.org/software/S0526)   |  [KGH_SPY](https://attack.mitre.org/software/S0526)   |

[KGH_SPY](https://attack.mitre.org/software/S0526) can collect credentials from WINSCP.[[18]](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)

|  [S0349](https://attack.mitre.org/software/S0349)   |  [LaZagne](https://attack.mitre.org/software/S0349)   |

[LaZagne](https://attack.mitre.org/software/S0349) can obtain credentials from databases, mail, and WiFi across multiple platforms.[[19]](https://github.com/AlessandroZ/LaZagne)

|  [G0077](https://attack.mitre.org/groups/G0077)   |  [Leafminer](https://attack.mitre.org/groups/G0077)   |

[Leafminer](https://attack.mitre.org/groups/G0077) used several tools for retrieving login and password information, including LaZagne.[[20]](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)

|  [S0447](https://attack.mitre.org/software/S0447)   |  [Lokibot](https://attack.mitre.org/software/S0447)   |

[Lokibot](https://attack.mitre.org/software/S0447) has stolen credentials from multiple applications and data sources including Windows OS credentials, email clients, FTP, and SFTP clients.[[21]](https://insights.infoblox.com/threat-intelligence-reports/threat-intelligence--22)

|  [G1026](https://attack.mitre.org/groups/G1026)   |  [Malteiro](https://attack.mitre.org/groups/G1026)   |

[Malteiro](https://attack.mitre.org/groups/G1026) has obtained credentials from mail clients via NirSoft MailPassView.[[22]](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)

|  [S1156](https://attack.mitre.org/software/S1156)   |  [Manjusaka](https://attack.mitre.org/software/S1156)   |

[Manjusaka](https://attack.mitre.org/software/S1156) extracts credentials from the Windows Registry associated with Premiumsoft Navicat, a utility used to facilitate access to various database types.[[23]](https://blog.talosintelligence.com/manjusaka-offensive-framework/)

|  [S0167](https://attack.mitre.org/software/S0167)   |  [Matryoshka](https://attack.mitre.org/software/S0167)   |

[Matryoshka](https://attack.mitre.org/software/S0167) is capable of stealing Outlook passwords.[[24]](http://www.clearskysec.com/wp-content/uploads/2017/07/Operation_Wilted_Tulip.pdf)[[25]](https://cdn2.hubspot.net/hubfs/1903456/Whitepapers/CopyKittens.pdf)

|  [S1146](https://attack.mitre.org/software/S1146)   |  [MgBot](https://attack.mitre.org/software/S1146)   |

[MgBot](https://attack.mitre.org/software/S1146) includes modules for stealing stored credentials from Outlook and Foxmail email client software.[[26]](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)[[27]](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)

|  [S0002](https://attack.mitre.org/software/S0002)   |  [Mimikatz](https://attack.mitre.org/software/S0002)   |

[Mimikatz](https://attack.mitre.org/software/S0002) performs credential dumping to obtain account and password information useful in gaining access to additional systems and enterprise network resources. It contains functionality to acquire information about credentials in many ways, including from the credential vault and DPAPI.[[28]](https://github.com/gentilkiwi/mimikatz)[[29]](https://github.com/gentilkiwi/mimikatz/wiki/module-~-lsadump)[[30]](https://www.dsinternals.com/en/retrieving-dpapi-backup-keys-from-active-directory/)[[31]](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)[[32]](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)

|  [S9022](https://attack.mitre.org/software/S9022)   |  [MirrorStealer](https://attack.mitre.org/software/S9022)   |

[MirrorStealer](https://attack.mitre.org/software/S9022) has the ability to steal credentials from email clients.[[33]](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)[[34]](https://www.trendmicro.com/en_us/research/24/k/lodeinfo-campaign-of-earth-kasha.html)

|  [S1122](https://attack.mitre.org/software/S1122)   |  [Mispadu](https://attack.mitre.org/software/S1122)   |

[Mispadu](https://attack.mitre.org/software/S1122) has obtained credentials from mail clients via NirSoft MailPassView.[[22]](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)[[35]](https://seguranca-informatica.pt/threat-analysis-the-emergent-ursa-trojan-impacts-many-countries-using-a-sophisticated-loader/)[[36]](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)

|  [G0069](https://attack.mitre.org/groups/G0069)   |  [MuddyWater](https://attack.mitre.org/groups/G0069)   |

[MuddyWater](https://attack.mitre.org/groups/G0069) has performed credential dumping with [LaZagne](https://attack.mitre.org/software/S0349) and other tools, including by dumping passwords saved in victim email.[[37]](https://researchcenter.paloaltonetworks.com/2017/11/unit42-muddying-the-water-targeted-attacks-in-the-middle-east/)[[38]](https://www.symantec.com/blogs/threat-intelligence/seedworm-espionage-group)[[39]](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) can retrieve passwords from messaging and mail client applications.[[40]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has used credential dumping tools such as [LaZagne](https://attack.mitre.org/software/S0349) to steal credentials to accounts logged into the compromised system and to Outlook Web Access.[[41]](https://web.archive.org/web/20230610050742/https://pan-unit42.github.io/playbook_viewer/?pb=evasive-serpens)[[42]](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)[[43]](https://static.carahsoft.com/concrete/files/1015/2779/3571/M-Trends-2018-Report.pdf)[[44]](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)

|  [S0138](https://attack.mitre.org/software/S0138)   |  [OLDBAIT](https://attack.mitre.org/software/S0138)   |

[OLDBAIT](https://attack.mitre.org/software/S0138) collects credentials from several email clients.[[45]](https://web.archive.org/web/20151022204649/https://www.fireeye.com/content/dam/fireeye-www/global/en/current-threats/pdfs/rpt-apt28.pdf)

|  [S0048](https://attack.mitre.org/software/S0048)   |  [PinchDuke](https://attack.mitre.org/software/S0048)   |

[PinchDuke](https://attack.mitre.org/software/S0048) steals credentials from compromised hosts. [PinchDuke](https://attack.mitre.org/software/S0048)'s credential stealing functionality is believed to be based on the source code of the Pinch credential stealing malware (also known as LdPinch). Credentials targeted by [PinchDuke](https://attack.mitre.org/software/S0048) include ones associated with many sources such as The Bat!, Yahoo!, Mail.ru, Passport.Net, Google Talk, and Microsoft Outlook.[[1]](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)

|  [S0435](https://attack.mitre.org/software/S0435)   |  [PLEAD](https://attack.mitre.org/software/S0435)   |

[PLEAD](https://attack.mitre.org/software/S0435) has the ability to steal saved passwords from Microsoft Outlook.[[46]](https://www.welivesecurity.com/2018/07/09/certificates-stolen-taiwanese-tech-companies-plead-malware-campaign/)

|  [S0378](https://attack.mitre.org/software/S0378)   |  [PoshC2](https://attack.mitre.org/software/S0378)   |

[PoshC2](https://attack.mitre.org/software/S0378) can decrypt passwords stored in the RDCMan configuration file.[[47]](https://www.secureworks.com/blog/lyceum-takes-center-stage-in-middle-east-campaign)

|  [S0113](https://attack.mitre.org/software/S0113)   |  [Prikormka](https://attack.mitre.org/software/S0113)   |

A module in [Prikormka](https://attack.mitre.org/software/S0113) collects passwords stored in applications installed on the victim.[[48]](http://www.welivesecurity.com/wp-content/uploads/2016/05/Operation-Groundbait.pdf)

|  [S0192](https://attack.mitre.org/software/S0192)   |  [Pupy](https://attack.mitre.org/software/S0192)   |

[Pupy](https://attack.mitre.org/software/S0192) can use Lazagne for harvesting credentials.[[49]](https://github.com/n1nj4sec/pupy)

|  [S0262](https://attack.mitre.org/software/S0262)   |  [QuasarRAT](https://attack.mitre.org/software/S0262)   |

[QuasarRAT](https://attack.mitre.org/software/S0262) can obtain passwords from common FTP clients.[[50]](https://github.com/quasar/QuasarRAT)[[51]](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)

|  [S1240](https://attack.mitre.org/software/S1240)   |  [RedLine Stealer](https://attack.mitre.org/software/S1240)   |

[RedLine Stealer](https://attack.mitre.org/software/S1240) has obtained credentials from VPN services, FTP clients and Instant Messenger (IM)/Chat clients.[[52]](https://www.kroll.com/en/publications/cyber/redlinestealer-malware)[[53]](https://www.proofpoint.com/us/blog/threat-insight/new-redline-stealer-distributed-using-coronavirus-themed-email-campaign)[[54]](https://www.splunk.com/en_us/blog/security/do-not-cross-the-redline-stealer-detections-and-analysis.html)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) used account credentials they obtained to attempt access to Group Managed Service Account (gMSA) passwords.[[55]](https://www.microsoft.com/security/blog/2021/01/20/deep-dive-into-the-solorigate-second-stage-activation-from-sunburst-to-teardrop-and-raindrop/)

|  [G0038](https://attack.mitre.org/groups/G0038)   |  [Stealth Falcon](https://attack.mitre.org/groups/G0038)   |

[Stealth Falcon](https://attack.mitre.org/groups/G0038) malware gathers passwords from multiple sources, including Windows Credential Vault and Outlook.[[56]](https://citizenlab.org/2016/05/stealth-falcon/)

|  [S9041](https://attack.mitre.org/software/S9041)   |  [TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041)   |

[TeamPCP Cloud Stealer](https://attack.mitre.org/software/S9041) can harvest credentials from cryptocurrency wallets and keystores such as Ethereum keystores, Cardano keys, Solana validator keypairs, Ledger device files, and Anchor deploy keys.[[57]](https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23)[[58]](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)[[59]](https://www.ic3.gov/CSA/2026/260702.pdf)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has attempted to obtain credentials from OpenSSH, realvnc, and PuTTY.[[60]](https://media.defense.gov/2023/May/24/2003229517/-1/-1/0/CSA_Living_off_the_Land.PDF)

|  [S1207](https://attack.mitre.org/software/S1207)   |  [XLoader](https://attack.mitre.org/software/S1207)   |

[XLoader](https://attack.mitre.org/software/S1207) can collect credentials stored in email clients.[[61]](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)[[62]](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)

## Mitigations

| ID  | Mitigation  | Description

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

The password for the user's login keychain can be changed from the user's login password. This increases the complexity for an adversary because they need to know an additional password.

Organizations may consider weighing the risk of storing credentials in password stores and web browsers. If system, software, or web browser credential disclosure is a significant concern, technical controls, policy, and user training may be used to prevent storage of credentials in improper locations.

|  [M1026](https://attack.mitre.org/mitigations/M1026)   |  [Privileged Account Management](https://attack.mitre.org/mitigations/M1026)   |

Limit the number of accounts and services with permission to query information from password stores to only those required. Ensure that accounts and services with permissions to query password stores only have access to the secrets they require.

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Perform regular software updates to mitigate exploitation risk.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0430](https://attack.mitre.org/detectionstrategies/DET0430)   |  [Detect Credentials Access from Password Stores](https://attack.mitre.org/detectionstrategies/DET0430)   |  [AN1198](https://attack.mitre.org/detectionstrategies/DET0430#AN1198)   |

Monitors suspicious access to password stores such as LSASS, DPAPI, Windows Credential Manager, or browser credential databases. Detects anomalous process-to-process access (e.g., Mimikatz accessing LSASS) and correlation of credential store file reads with execution of non-standard processes.

|  [AN1199](https://attack.mitre.org/detectionstrategies/DET0430#AN1199)   |

Detects access to known password store files (e.g., /etc/shadow, GNOME Keyring, KWallet, browser credential databases). Monitors anomalous process read attempts and suspicious API calls that attempt to extract stored credentials.

|  [AN1200](https://attack.mitre.org/detectionstrategies/DET0430#AN1200)   |

Monitors Keychain database access and suspicious invocations of security and osascript utilities. Correlates process execution with attempts to dump or unlock Keychain data.

|  [AN1201](https://attack.mitre.org/detectionstrategies/DET0430#AN1201)   |

Detects attempts to access or enumerate cloud password/secrets storage services such as AWS Secrets Manager, Azure Key Vault, or GCP Secret Manager. Monitors API calls for abnormal enumeration or bulk retrieval of secrets.

## References

-    [F-Secure Labs. (2015, September 17). The Dukes: 7 years of Russian cyberespionage. Retrieved December 10, 2015.](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)
-    [CERT Polska. (2026, January 30). Energy Sector Incident Report – 29 December. Retrieved April 22, 2026.](https://cert.pl/uploads/docs/CERT_Polska_Energy_Sector_Incident_Report_2025.pdf)
-    [Jazi, H. (2020, April 16). New AgentTesla variant steals WiFi credentials. Retrieved May 19, 2020.](https://blog.malwarebytes.com/threat-analysis/2020/04/new-agenttesla-variant-steals-wifi-credentials/)
-    [Eyal Sela. (2026, April). The AI-Assisted Breach of Mexico's Government Infrastructure. Retrieved September 2, 2026.](https://cdn.prod.website-files.com/69944dd945f20ca4a27a7c47/69d8bb5aea59e31efb3b8a7f_Tech_Report_ai_breach_mex_gov.pdf)
-    [Security Response attack Investigation Team. (2019, March 27). Elfin: Relentless Espionage Group Targets Multiple Organizations in Saudi Arabia and U.S.. Retrieved April 10, 2019.](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)
-    [Ackerman, G., et al. (2018, December 21). OVERRULED: Containing a Potentially Destructive Adversary. Retrieved January 17, 2019.](https://www.fireeye.com/blog/threat-research/2018/12/overruled-containing-a-potentially-destructive-adversary.html)
-    [Microsoft Threat Intelligence. (2024, November 22). Microsoft shares latest intelligence on North Korean and Chinese threat actors at CYBERWARCON. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2024/11/22/microsoft-shares-latest-intelligence-on-north-korean-and-chinese-threat-actors-at-cyberwarcon/)
-    [Microsoft Threat Intelligence. (2026, April 16). Dissecting Sapphire Sleet’s macOS intrusion from lure to compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)
-    [Rusu, B. (2020, May 21). Iranian Chafer APT Targeted Air Transportation and Government in Kuwait and Saudi Arabia. Retrieved May 22, 2020.](https://www.bitdefender.com/blog/labs/iranian-chafer-apt-targeted-air-transportation-and-government-in-kuwait-and-saudi-arabia/)
-    [Nikita Rostovcev. (2022, August 18). APT41 World Tour 2021 on a tight schedule. Retrieved February 22, 2024.](https://www.group-ib.com/blog/apt41-world-tour-2021/)
-    [Salem, E. (2019, February 13). ASTAROTH MALWARE USES LEGITIMATE OS AND ANTIVIRUS PROCESSES TO STEAL PASSWORDS AND PERSONAL DATA. Retrieved April 17, 2019.](https://www.cybereason.com/blog/information-stealing-malware-targeting-brazil-full-research)
-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [Giuliani, M., Allievi, A. (2011, February 28). Carberp - a modular information stealing trojan. Retrieved September 12, 2024.](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)
-    [Ernesto Fernández Provecho, Pham Duy Phuc, Ciana Driscoll & Vinoo Thomas. (2023, November 21). The Continued Evolution of the DarkGate Malware-as-a-Service. Retrieved February 9, 2024.](https://www.trellix.com/blogs/research/the-continued-evolution-of-the-darkgate-malware-as-a-service/)
-    [Porolli, M. (2020, July 9). More evil: A deep look at Evilnum and its toolset. Retrieved January 22, 2021.](https://www.welivesecurity.com/2020/07/09/more-evil-deep-look-evilnum-toolset/)
-    [Visa Public. (2019, February). FIN6 Cybercrime Group Expands Threat to eCommerce Merchants. Retrieved September 16, 2019.](https://usa.visa.com/dam/VCOM/global/support-legal/documents/fin6-cybercrime-group-expands-threat-To-ecommerce-merchants.pdf)
-    [Kayal, A. et al. (2021, October). LYCEUM REBORN: COUNTERINTELLIGENCE IN THE MIDDLE EAST. Retrieved June 14, 2022.](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)
-    [Dahan, A. et al. (2020, November 2). Back to the Future: Inside the Kimsuky KGH Spyware Suite. Retrieved November 6, 2020.](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)
-    [Zanni, A. (n.d.). The LaZagne Project !!!. Retrieved December 14, 2018.](https://github.com/AlessandroZ/LaZagne)
-    [Symantec Security Response. (2018, July 25). Leafminer: New Espionage Campaigns Targeting Middle Eastern Regions. Retrieved August 28, 2018.](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)
-    [Hoang, M. (2019, January 31). Malicious Activity Report: Elements of Lokibot Infostealer. Retrieved May 15, 2020.](https://insights.infoblox.com/threat-intelligence-reports/threat-intelligence--22)
-    [SCILabs. (2021, December 23). Cyber Threat Profile Malteiro. Retrieved March 13, 2024.](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)
-    [Asheer Malhotra & Vitor Ventura. (2022, August 2). Manjusaka: A Chinese sibling of Sliver and Cobalt Strike. Retrieved September 4, 2024.](https://blog.talosintelligence.com/manjusaka-offensive-framework/)
-    [ClearSky Cyber Security and Trend Micro. (2017, July). Operation Wilted Tulip: Exposing a cyber espionage apparatus. Retrieved August 21, 2017.](http://www.clearskysec.com/wp-content/uploads/2017/07/Operation_Wilted_Tulip.pdf)
-    [Minerva Labs LTD and ClearSky Cyber Security. (2015, November 23). CopyKittens Attack Group. Retrieved November 17, 2024.](https://cdn2.hubspot.net/hubfs/1903456/Whitepapers/CopyKittens.pdf)
-    [Facundo Muñoz. (2023, April 26). Evasive Panda APT group delivers malware via updates for popular Chinese software. Retrieved July 25, 2024.](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)
-    [Threat Hunter Team. (2023, April 20). Daggerfly: APT Actor Targets Telecoms Company in Africa. Retrieved July 25, 2024.](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)
-    [Deply, B. (n.d.). Mimikatz. Retrieved September 29, 2015.](https://github.com/gentilkiwi/mimikatz)
-    [Deply, B., Le Toux, V. (2016, June 5). module ~ lsadump. Retrieved August 7, 2017.](https://github.com/gentilkiwi/mimikatz/wiki/module-~-lsadump)
-    [Grafnetter, M. (2015, October 26). Retrieving DPAPI Backup Keys from Active Directory. Retrieved December 19, 2017.](https://www.dsinternals.com/en/retrieving-dpapi-backup-keys-from-active-directory/)
-    [The Australian Cyber Security Centre (ACSC), the Canadian Centre for Cyber Security (CCCS), the New Zealand National Cyber Security Centre (NZ NCSC), CERT New Zealand, the UK National Cyber Security Centre (UK NCSC) and the US National Cybersecurity and Communications Integration Center (NCCIC). (2018, October 11). Joint report on publicly available hacking tools. Retrieved March 11, 2019.](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)

-    [Strategic Cyber LLC. (2020, November 5). Cobalt Strike: Advanced Threat Tactics for Penetration Testers. Retrieved April 13, 2021.](https://web.archive.org/web/20210708035426/https://www.cobaltstrike.com/downloads/csmanual43.pdf)
-    [Breitenbacher, D. (2022, December 14). Unmasking MirrorFace: Operation LiberalFace targeting Japanese political entities. Retrieved April 17, 2026.](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)
-    [Trend Micro. (2024, November 19). Spot the Difference: Earth Kasha's New LODEINFO Campaign And The Correlation Analysis With The APT10 Umbrella. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/24/k/lodeinfo-campaign-of-earth-kasha.html)
-    [Pedro Tavares (Segurança Informática). (2020, September 15). Threat analysis: The emergent URSA trojan impacts many countries using a sophisticated loader. Retrieved March 13, 2024.](https://seguranca-informatica.pt/threat-analysis-the-emergent-ursa-trojan-impacts-many-countries-using-a-sophisticated-loader/)
-    [ESET Security. (2019, November 19). Mispadu: Advertisement for a discounted Unhappy Meal. Retrieved March 13, 2024.](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)
-    [Lancaster, T.. (2017, November 14). Muddying the Water: Targeted Attacks in the Middle East. Retrieved March 15, 2018.](https://researchcenter.paloaltonetworks.com/2017/11/unit42-muddying-the-water-targeted-attacks-in-the-middle-east/)
-    [Symantec DeepSight Adversary Intelligence Team. (2018, December 10). Seedworm: Group Compromises Government Agencies, Oil & Gas, NGOs, Telecoms, and IT Firms. Retrieved December 14, 2018.](https://www.symantec.com/blogs/threat-intelligence/seedworm-espionage-group)
-    [Peretz, A. and Theck, E. (2021, March 5). Earth Vetala – MuddyWater Continues to Target Organizations in the Middle East. Retrieved March 18, 2021.](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)
-    [Lambert, T. (2020, January 29). Intro to Netwire. Retrieved January 7, 2021.](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)
-    [Unit42. (2016, May 1). Evasive Serpens Unit 42 Playbook Viewer. Retrieved February 6, 2023.](https://web.archive.org/web/20230610050742/https://pan-unit42.github.io/playbook_viewer/?pb=evasive-serpens)
-    [Davis, S. and Caban, D. (2017, December 19). APT34 - New Targeted Attack in the Middle East. Retrieved December 20, 2017.](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)
-    [Mandiant. (2018). Mandiant M-Trends 2018. Retrieved November 17, 2024.](https://static.carahsoft.com/concrete/files/1015/2779/3571/M-Trends-2018-Report.pdf)
-    [Bromiley, M., et al.. (2019, July 18). Hard Pass: Declining APT34’s Invite to Join Their Professional Network. Retrieved August 26, 2019.](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)
-    [FireEye. (2015). APT28: A WINDOW INTO RUSSIA’S CYBER ESPIONAGE OPERATIONS?. Retrieved August 19, 2015.](https://web.archive.org/web/20151022204649/https://www.fireeye.com/content/dam/fireeye-www/global/en/current-threats/pdfs/rpt-apt28.pdf)
-    [Cherepanov, A.. (2018, July 9). Certificates stolen from Taiwanese tech‑companies misused in Plead malware campaign. Retrieved May 6, 2020.](https://www.welivesecurity.com/2018/07/09/certificates-stolen-taiwanese-tech-companies-plead-malware-campaign/)
-    [SecureWorks 2019, August 27 LYCEUM Takes Center Stage in Middle East Campaign Retrieved. 2019/11/19](https://www.secureworks.com/blog/lyceum-takes-center-stage-in-middle-east-campaign)
-    [Cherepanov, A.. (2016, May 17). Operation Groundbait: Analysis of a surveillance toolkit. Retrieved May 18, 2016.](http://www.welivesecurity.com/wp-content/uploads/2016/05/Operation-Groundbait.pdf)
-    [Nicolas Verdier. (n.d.). Retrieved January 29, 2018.](https://github.com/n1nj4sec/pupy)
-    [MaxXor. (n.d.). QuasarRAT. Retrieved July 10, 2018.](https://github.com/quasar/QuasarRAT)
-    [Meltzer, M, et al. (2018, June 07). Patchwork APT Group Targets US Think Tanks. Retrieved July 16, 2018.](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)
-    [George Glass. (2024, August 14). REDLINESTEALER Malware Driving the Initial Access Broker Market. Retrieved September 17, 2025.](https://www.kroll.com/en/publications/cyber/redlinestealer-malware)
-    [Proofpoint Threat Insight Team, Jeremy H, Axel F. (2020, March 16). New Redline Password Stealer Malware. Retrieved September 17, 2025.](https://www.proofpoint.com/us/blog/threat-insight/new-redline-stealer-distributed-using-coronavirus-themed-email-campaign)
-    [Splunk Threat Research Team. (2023, June 1). Do Not Cross The 'RedLine' Stealer: Detections and Analysis. Retrieved September 17, 2025.](https://www.splunk.com/en_us/blog/security/do-not-cross-the-redline-stealer-detections-and-analysis.html)
-    [MSTIC, CDOC, 365 Defender Research Team. (2021, January 20). Deep dive into the Solorigate second-stage activation: From SUNBURST to TEARDROP and Raindrop . Retrieved January 22, 2021.](https://www.microsoft.com/security/blog/2021/01/20/deep-dive-into-the-solorigate-second-stage-activation-from-sunburst-to-teardrop-and-raindrop/)
-    [Marczak, B. and Scott-Railton, J.. (2016, May 29). Keep Calm and (Don’t) Enable Macros: A New Threat Actor Targets UAE Dissidents. Retrieved June 8, 2016.](https://citizenlab.org/2016/05/stealth-falcon/)
-    [Aqua Security . (2026, March 21). Trivy ecosystem supply chain temporarily compromised. Retrieved July 1, 2026.](https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23)
-    [Aqua Team. (2026, April 1). Update: Ongoing Investigation and Continued Remediation. Retrieved July 1, 2026.](https://www.aquasec.com/blog/trivy-supply-chain-attack-what-you-need-to-know)
-    [FBI. (2026, July 2). Cyber Criminal Group TeamPCP. Retrieved July 7, 2026.](https://www.ic3.gov/CSA/2026/260702.pdf)
-    [NSA et al. (2023, May 24). People's Republic of China State-Sponsored Cyber Actor Living off the Land to Evade Detection. Retrieved July 27, 2023.](https://media.defense.gov/2023/May/24/2003229517/-1/-1/0/CSA_Living_off_the_Land.PDF)
-    [Nart Villeneuve, Randi Eitzman, Sandor Nemes & Tyler Dean, Google Cloud. (2017, October 5). Significant FormBook Distribution Campaigns Impacting the U.S. and South Korea. Retrieved March 11, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)
-    [Gustavo Palazolo, Netskope. (2022, March 11). New Formbook Campaign Delivered Through Phishing Emails. Retrieved March 11, 2025.](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)

×
