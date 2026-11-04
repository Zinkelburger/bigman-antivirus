---
url: "https://attack.mitre.org/techniques/T1555/003/"
title: "Credentials from Password Stores: Credentials from Web Browsers, Sub-technique T1555.003 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:53:50Z"
via: "live"
sha256: "755ec7507296b82f8aa516da401a5686a37b6c6ea24f82cab955712d2eb5e417"
wayback: "https://web.archive.org/web/20261101080451/https://attack.mitre.org/techniques/T1555/003/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Credentials from Password Stores: Credentials from Web Browsers, Sub-technique T1555.003 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1555/003/>
> Snapshot: <https://web.archive.org/web/20261101080451/https://attack.mitre.org/techniques/T1555/003/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555)
- Credentials from Web Browsers

#  Credentials from Password Stores: Credentials from Web Browsers

##### Other sub-techniques of Credentials from Password Stores (6)

| ID  | Name

|  [T1555.001](https://attack.mitre.org/techniques/T1555/001/)   |  [Keychain](https://attack.mitre.org/techniques/T1555/001/)

|  [T1555.002](https://attack.mitre.org/techniques/T1555/002/)   |  [Securityd Memory](https://attack.mitre.org/techniques/T1555/002/)

|  T1555.003   |  Credentials from Web Browsers

|  [T1555.004](https://attack.mitre.org/techniques/T1555/004/)   |  [Windows Credential Manager](https://attack.mitre.org/techniques/T1555/004/)

|  [T1555.005](https://attack.mitre.org/techniques/T1555/005/)   |  [Password Managers](https://attack.mitre.org/techniques/T1555/005/)

|  [T1555.006](https://attack.mitre.org/techniques/T1555/006/)   |  [Cloud Secrets Management Stores](https://attack.mitre.org/techniques/T1555/006/)

Adversaries may acquire credentials from web browsers by reading files specific to the target browser.[[1]](https://blog.talosintelligence.com/2018/02/olympic-destroyer.html) Web browsers commonly save credentials such as website usernames and passwords so that they do not need to be entered manually in the future. Web browsers typically store the credentials in an encrypted format within a credential store; however, methods exist to extract plaintext credentials from web browsers.

For example, on Windows systems, encrypted credentials may be obtained from Google Chrome by reading a database file, AppData\Local\Google\Chrome\User Data\Default\Login Data and executing a SQL query: SELECT action_url, username_value, password_value FROM logins;. The plaintext password can then be obtained by passing the encrypted credentials to the Windows API function CryptUnprotectData, which uses the victim’s cached logon credentials as the decryption key.[[2]](https://docs.microsoft.com/en-us/windows/desktop/api/dpapi/nf-dpapi-cryptunprotectdata)

Adversaries have executed similar procedures for common web browsers such as FireFox, Safari, Edge, etc.[[3]](https://www.proofpoint.com/us/threat-insight/post/new-vega-stealer-shines-brightly-targeted-campaign)[[4]](https://www.fireeye.com/blog/threat-research/2017/07/hawkeye-malware-distributed-in-phishing-campaign.html) Windows stores Internet Explorer and Microsoft Edge credentials in Credential Lockers managed by the [Windows Credential Manager](https://attack.mitre.org/techniques/T1555/004).

Adversaries may also acquire credentials by searching web browser process memory for patterns that commonly match credentials.[[5]](https://github.com/putterpanda/mimikittenz)

After acquiring credentials from web browsers, adversaries may attempt to recycle the credentials across different systems and/or accounts in order to expand access. This can result in significantly furthering an adversary's objective in cases where credentials gained from web browsers overlap with privileged accounts (e.g. domain administrator).

ID: T1555.003

Sub-technique of:  [T1555](https://attack.mitre.org/techniques/T1555)

ⓘ

Tactic: [Credential Access](https://attack.mitre.org/tactics/TA0006)

ⓘ

Platforms: Linux, Windows, macOS

Contributors: Barry Shteiman, Exabeam; Don Le, Stifel Financial; RedHuntLabs, @redhuntlabs; Ryan Benson, Exabeam; Sylvain Gil, Exabeam

Version: 1.2

Created: 12 February 2020

Last Modified: 11 September 2026

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1555/003/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1555/003/)

## Procedure Examples

| ID  | Name  | Description

|  [S0331](https://attack.mitre.org/software/S0331)   |  [Agent Tesla](https://attack.mitre.org/software/S0331)   |

[Agent Tesla](https://attack.mitre.org/software/S0331) can gather credentials from a number of browsers.[[6]](https://labs.bitdefender.com/2020/04/oil-gas-spearphishing-campaigns-drop-agent-tesla-spyware-in-advance-of-historic-opec-deal/)

|  [G0130](https://attack.mitre.org/groups/G0130)   |  [Ajax Security Team](https://attack.mitre.org/groups/G0130)   |

[Ajax Security Team](https://attack.mitre.org/groups/G0130) has used FireMalv custom-developed malware, which collected passwords from the Firefox browser storage.[[7]](https://blog.checkpoint.com/wp-content/uploads/2015/11/rocket-kitten-report.pdf)

|  [G0022](https://attack.mitre.org/groups/G0022)   |  [APT3](https://attack.mitre.org/groups/G0022)   |

[APT3](https://attack.mitre.org/groups/G0022) has used tools to dump passwords from browsers.[[8]](https://web.archive.org/web/20160910124439/http://www.symantec.com/connect/blogs/buckeye-cyberespionage-group-shifts-gaze-us-hong-kong)

|  [G0064](https://attack.mitre.org/groups/G0064)   |  [APT33](https://attack.mitre.org/groups/G0064)   |

[APT33](https://attack.mitre.org/groups/G0064) has used a variety of publicly available tools like [LaZagne](https://attack.mitre.org/software/S0349) to gather credentials.[[9]](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)[[10]](https://www.fireeye.com/blog/threat-research/2018/12/overruled-containing-a-potentially-destructive-adversary.html)

|  [G0067](https://attack.mitre.org/groups/G0067)   |  [APT37](https://attack.mitre.org/groups/G0067)   |

[APT37](https://attack.mitre.org/groups/G0067) has used a credential stealer known as ZUMKONG that can harvest usernames and passwords stored in browsers.[[11]](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has gathered Web browser data to include saved credentials, cookies, bookmarks, autofill data, and extension-specific storage. [APT38](https://attack.mitre.org/groups/G0082) has also sought cryptocurrency wallet extensions.[[12]](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)[[13]](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)

|  [G0096](https://attack.mitre.org/groups/G0096)   |  [APT41](https://attack.mitre.org/groups/G0096)   |

[APT41](https://attack.mitre.org/groups/G0096) used BrowserGhost, a tool designed to obtain credentials from browsers, to retrieve information from password stores.[[14]](https://www.group-ib.com/blog/apt41-world-tour-2021/)

|  [G1044](https://attack.mitre.org/groups/G1044)   |  [APT42](https://attack.mitre.org/groups/G1044)   |

[APT42](https://attack.mitre.org/groups/G1044) has used custom malware to steal credentials.[[15]](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf)

|  [S0344](https://attack.mitre.org/software/S0344)   |  [Azorult](https://attack.mitre.org/software/S0344)   |

[Azorult](https://attack.mitre.org/software/S0344) can steal credentials from the victim's browser.[[16]](https://researchcenter.paloaltonetworks.com/2018/11/unit42-new-wine-old-bottle-new-azorult-variant-found-findmyname-campaign-using-fallout-exploit-kit/)

|  [S0093](https://attack.mitre.org/software/S0093)   |  [Backdoor.Oldrea](https://attack.mitre.org/software/S0093)   |

Some [Backdoor.Oldrea](https://attack.mitre.org/software/S0093) samples contain a publicly available Web browser password recovery tool.[[17]](https://community.broadcom.com/symantecenterprise/communities/community-home/librarydocuments/viewdocument?DocumentKey=7382dce7-0260-4782-84cc-890971ed3f17&CommunityKey=1ecf5f55-9545-44d6-b0f4-4e4a7f5f5e68&tab=librarydocuments)

|  [S1246](https://attack.mitre.org/software/S1246)   |  [BeaverTail](https://attack.mitre.org/software/S1246)   |

[BeaverTail](https://attack.mitre.org/software/S1246) has stolen passwords saved in web browsers.[[18]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[19]](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)[[20]](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)[[21]](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/) [BeaverTail](https://attack.mitre.org/software/S1246) has also been known to collect login data from Firefox within key3.db, key4.db and logins.json from /.mozilla/firefox/ for exfiltration.[[22]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)

|  [S0089](https://attack.mitre.org/software/S0089)   |  [BlackEnergy](https://attack.mitre.org/software/S0089)   |

[BlackEnergy](https://attack.mitre.org/software/S0089) has used a plug-in to gather credentials from web browsers including FireFox, Google Chrome, and Internet Explorer.[[23]](https://blog-assets.f-secure.com/wp-content/uploads/2019/10/15163408/BlackEnergy_Quedagh.pdf)[[24]](https://securelist.com/be2-custom-plugins-router-abuse-and-target-profiles/67353/)

|  [S0657](https://attack.mitre.org/software/S0657)   |  [BLUELIGHT](https://attack.mitre.org/software/S0657)   |

[BLUELIGHT](https://attack.mitre.org/software/S0657) can collect passwords stored in web browers, including Internet Explorer, Edge, Chrome, and Naver Whale.[[25]](https://www.volexity.com/blog/2021/08/17/north-korean-apt-inkysquid-infects-victims-using-browser-exploits/)

|  [S0484](https://attack.mitre.org/software/S0484)   |  [Carberp](https://attack.mitre.org/software/S0484)   |

[Carberp](https://attack.mitre.org/software/S0484)'s passw.plug plugin can gather passwords saved in Opera, Internet Explorer, Safari, Firefox, and Chrome.[[26]](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)

|  [S0631](https://attack.mitre.org/software/S0631)   |  [Chaes](https://attack.mitre.org/software/S0631)   |

[Chaes](https://attack.mitre.org/software/S0631) can steal login credentials and stored financial information from the browser.[[27]](https://www.cybereason.com/hubfs/dam/collateral/reports/11-2020-Chaes-e-commerce-malware-research.pdf)

|  [S0144](https://attack.mitre.org/software/S0144)   |  [ChChes](https://attack.mitre.org/software/S0144)   |

[ChChes](https://attack.mitre.org/software/S0144) steals credentials stored inside Internet Explorer.[[28]](https://www.pwc.co.uk/cyber-security/pdf/pwc-uk-operation-cloud-hopper-technical-annex-april-2017.pdf)

|  [S0492](https://attack.mitre.org/software/S0492)   |  [CookieMiner](https://attack.mitre.org/software/S0492)   |

[CookieMiner](https://attack.mitre.org/software/S0492) can steal saved usernames and passwords in Chrome as well as credit card credentials.[[29]](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)

|  [S0050](https://attack.mitre.org/software/S0050)   |  [CosmicDuke](https://attack.mitre.org/software/S0050)   |

[CosmicDuke](https://attack.mitre.org/software/S0050) collects user credentials, including passwords, for various programs including Web browsers.[[30]](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)

|  [S9057](https://attack.mitre.org/software/S9057)   |  [CRASHPAD](https://attack.mitre.org/software/S9057)   |

[CRASHPAD](https://attack.mitre.org/software/S9057) has extracted credentials saved within web browsers.[[31]](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)

|  [S0115](https://attack.mitre.org/software/S0115)   |  [Crimson](https://attack.mitre.org/software/S0115)   |

[Crimson](https://attack.mitre.org/software/S0115) contains a module to steal credentials from Web browsers on the victim machine.[[32]](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)[[33]](https://securelist.com/transparent-tribe-part-1/98127/)

|  [S0367](https://attack.mitre.org/software/S0367)   |  [Emotet](https://attack.mitre.org/software/S0367)   |

[Emotet](https://attack.mitre.org/software/S0367) has been observed dropping browser password grabber modules. [[34]](https://documents.trendmicro.com/assets/white_papers/ExploringEmotetsActivities_Final.pdf)[[35]](https://securityintelligence.com/new-banking-trojan-icedid-discovered-by-ibm-x-force-research/)

|  [S0363](https://attack.mitre.org/software/S0363)   |  [Empire](https://attack.mitre.org/software/S0363)   |

[Empire](https://attack.mitre.org/software/S0363) can use modules that extract passwords from common web browsers such as Firefox and Chrome.[[36]](https://github.com/PowerShellEmpire/Empire)

|  [G0037](https://attack.mitre.org/groups/G0037)   |  [FIN6](https://attack.mitre.org/groups/G0037)   |

[FIN6](https://attack.mitre.org/groups/G0037) has used the Stealer One credential stealer to target web browsers.[[37]](https://usa.visa.com/dam/VCOM/global/support-legal/documents/fin6-cybercrime-group-expands-threat-To-ecommerce-merchants.pdf)

|  [S9010](https://attack.mitre.org/software/S9010)   |  [GlassWorm](https://attack.mitre.org/software/S9010)   |

[GlassWorm](https://attack.mitre.org/software/S9010) has gathered credentials stored in Mozilla FireFox and Chromium-based Browsers.[[38]](https://www.koi.ai/blog/glassworm-goes-mac-fresh-infrastructure-new-tricks)[[39]](https://socket.dev/blog/glassworm-loader-hits-open-vsx-via-suspected-developer-account-compromise)

|  [S0531](https://attack.mitre.org/software/S0531)   |  [Grandoreiro](https://attack.mitre.org/software/S0531)   |

[Grandoreiro](https://attack.mitre.org/software/S0531) can steal cookie data and credentials from Google Chrome.[[40]](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)[[41]](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)

|  [S0132](https://attack.mitre.org/software/S0132)   |  [H1N1](https://attack.mitre.org/software/S0132)   |

[H1N1](https://attack.mitre.org/software/S0132) dumps usernames and passwords from Firefox, Internet Explorer, and Outlook.[[42]](https://web.archive.org/web/20231210122239/https://blogs.cisco.com/security/h1n1-technical-analysis-reveals-new-capabilities-part-2)

|  [G1001](https://attack.mitre.org/groups/G1001)   |  [HEXANE](https://attack.mitre.org/groups/G1001)   |

[HEXANE](https://attack.mitre.org/groups/G1001) has used a [Mimikatz](https://attack.mitre.org/software/S0002)-based tool and a PowerShell script to steal passwords from Google Chrome.[[43]](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)

|  [S0434](https://attack.mitre.org/software/S0434)   |  [Imminent Monitor](https://attack.mitre.org/software/S0434)   |

[Imminent Monitor](https://attack.mitre.org/software/S0434) has a PasswordRecoveryPacket module for recovering browser passwords.[[44]](https://web.archive.org/web/20190625182633if_/https://ti.360.net/blog/articles/apt-c-36-continuous-attacks-targeting-colombian-government-institutions-and-corporations-en/)

|  [G0100](https://attack.mitre.org/groups/G0100)   |  [Inception](https://attack.mitre.org/groups/G0100)   |

[Inception](https://attack.mitre.org/groups/G0100) used a browser plugin to steal passwords and sessions from Internet Explorer, Chrome, Opera, Firefox, Torch, and Yandex.[[45]](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/inception-framework-hiding-behind-proxies)

|  [S1245](https://attack.mitre.org/software/S1245)   |  [InvisibleFerret](https://attack.mitre.org/software/S1245)   |

[InvisibleFerret](https://attack.mitre.org/software/S1245) has stolen login data, autofill data, cryptocurrency wallets, and payment information saved in web browsers such as Chrome, Brave, Opera, Yandex and Edge, to include versions affiliated with major operating systems on Windows, Linux, and macOS.[[18]](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)[[22]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/) [InvisibleFerret](https://attack.mitre.org/software/S1245) has also leveraged the command ssh_zcp to copy browser data to include extensions and cryptocurrency wallet data.[[46]](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)

|  [S0528](https://attack.mitre.org/software/S0528)   |  [Javali](https://attack.mitre.org/software/S0528)   |

[Javali](https://attack.mitre.org/software/S0528) can capture login credentials from open browsers including Firefox, Chrome, Internet Explorer, and Edge.[[47]](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)

|  [S0283](https://attack.mitre.org/software/S0283)   |  [jRAT](https://attack.mitre.org/software/S0283)   |

[jRAT](https://attack.mitre.org/software/S0283) can capture passwords from common web browsers such as Internet Explorer, Google Chrome, and Firefox.[[48]](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)

|  [C0044](https://attack.mitre.org/campaigns/C0044)   |  [Juicy Mix](https://attack.mitre.org/campaigns/C0044)   |

During [Juicy Mix](https://attack.mitre.org/campaigns/C0044), [OilRig](https://attack.mitre.org/groups/G0049) used the CDumper (Chrome browser) and EDumper (Edge browser) to collect credentials.[[49]](https://www.welivesecurity.com/en/eset-research/oilrigs-outer-space-juicy-mix-same-ol-rig-new-drill-pipes/)

|  [S0387](https://attack.mitre.org/software/S0387)   |  [KeyBoy](https://attack.mitre.org/software/S0387)   |

[KeyBoy](https://attack.mitre.org/software/S0387) attempts to collect passwords from browsers.[[50]](https://blog.rapid7.com/2013/06/07/keyboy-targeted-attacks-against-vietnam-and-india/)

|  [S0526](https://attack.mitre.org/software/S0526)   |  [KGH_SPY](https://attack.mitre.org/software/S0526)   |

[KGH_SPY](https://attack.mitre.org/software/S0526) has the ability to steal data from the Chrome, Edge, Firefox, Thunderbird, and Opera browsers.[[51]](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)

|  [G0094](https://attack.mitre.org/groups/G0094)   |  [Kimsuky](https://attack.mitre.org/groups/G0094)   |

[Kimsuky](https://attack.mitre.org/groups/G0094) has used browser extensions including Google Chrome to steal passwords and cookies from browsers. [Kimsuky](https://attack.mitre.org/groups/G0094) has also used Nirsoft's WebBrowserPassView tool to dump the passwords obtained from victims.[[52]](https://www.zdnet.com/article/cyber-espionage-group-uses-chrome-extension-to-infect-victims/)[[53]](https://us-cert.cisa.gov/ncas/alerts/aa20-301a)[[54]](https://asert.arbornetworks.com/stolen-pencil-campaign-targets-academia/)[[55]](https://blog.talosintelligence.com/2021/11/kimsuky-abuses-blogs-delivers-malware.html)

|  [S0356](https://attack.mitre.org/software/S0356)   |  [KONNI](https://attack.mitre.org/software/S0356)   |

[KONNI](https://attack.mitre.org/software/S0356) can steal profiles (containing credential information) from Firefox, Chrome, and Opera.[[56]](https://blog.talosintelligence.com/2017/05/konni-malware-under-radar-for-years.html)

|  [G1004](https://attack.mitre.org/groups/G1004)   |  [LAPSUS$](https://attack.mitre.org/groups/G1004)   |

[LAPSUS$](https://attack.mitre.org/groups/G1004) has obtained passwords and session tokens with the use of the Redline password stealer.[[57]](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)

|  [S0349](https://attack.mitre.org/software/S0349)   |  [LaZagne](https://attack.mitre.org/software/S0349)   |

[LaZagne](https://attack.mitre.org/software/S0349) can obtain credentials from web browsers such as Google Chrome, Internet Explorer, and Firefox.[[58]](https://github.com/AlessandroZ/LaZagne)

|  [G0077](https://attack.mitre.org/groups/G0077)   |  [Leafminer](https://attack.mitre.org/groups/G0077)   |

[Leafminer](https://attack.mitre.org/groups/G0077) used several tools for retrieving login and password information, including LaZagne.[[59]](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)

|  [S0681](https://attack.mitre.org/software/S0681)   |  [Lizar](https://attack.mitre.org/software/S0681)   |

[Lizar](https://attack.mitre.org/software/S0681) has a module to collect usernames and passwords stored in browsers.[[60]](https://bi-zone.medium.com/from-pentest-to-apt-attack-cybercriminal-group-fin7-disguises-its-malware-as-an-ethical-hackers-c23c9a75e319)

|  [S0447](https://attack.mitre.org/software/S0447)   |  [Lokibot](https://attack.mitre.org/software/S0447)   |

[Lokibot](https://attack.mitre.org/software/S0447) has demonstrated the ability to steal credentials from multiple applications and data sources including Safari and the Chromium and Mozilla Firefox-based web browsers.[[61]](https://insights.infoblox.com/threat-intelligence-reports/threat-intelligence--22)

|  [S1213](https://attack.mitre.org/software/S1213)   |  [Lumma Stealer](https://attack.mitre.org/software/S1213)   |

[Lumma Stealer](https://attack.mitre.org/software/S1213) has gathered credential and other information from multiple browsers.[[62]](https://www.cybereason.com/blog/threat-analysis-rise-of-lummastealer)[[63]](https://www.fortinet.com/blog/threat-research/lumma-variant-on-youtube)[[64]](https://www.trendmicro.com/en_us/research/25/a/lumma-stealers-github-based-delivery-via-mdr.html)

|  [S0409](https://attack.mitre.org/software/S0409)   |  [Machete](https://attack.mitre.org/software/S0409)   |

[Machete](https://attack.mitre.org/software/S0409) collects stored credentials from several web browsers.[[65]](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)

|  [G1026](https://attack.mitre.org/groups/G1026)   |  [Malteiro](https://attack.mitre.org/groups/G1026)   |

[Malteiro](https://attack.mitre.org/groups/G1026) has stolen credentials stored in the victim’s browsers via software tool NirSoft WebBrowserPassView.[[66]](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)

|  [S1156](https://attack.mitre.org/software/S1156)   |  [Manjusaka](https://attack.mitre.org/software/S1156)   |

[Manjusaka](https://attack.mitre.org/software/S1156) gathers credentials from Chromium-based browsers.[[67]](https://blog.talosintelligence.com/manjusaka-offensive-framework/)

|  [S0530](https://attack.mitre.org/software/S0530)   |  [Melcoz](https://attack.mitre.org/software/S0530)   |

[Melcoz](https://attack.mitre.org/software/S0530) has the ability to steal credentials from web browsers.[[47]](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)

|  [S1146](https://attack.mitre.org/software/S1146)   |  [MgBot](https://attack.mitre.org/software/S1146)   |

[MgBot](https://attack.mitre.org/software/S1146) includes modules for stealing credentials from various browsers and applications, including Chrome, Opera, Firefox, Foxmail, QQBrowser, FileZilla, and WinSCP.[[68]](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)[[69]](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)

|  [S0002](https://attack.mitre.org/software/S0002)   |  [Mimikatz](https://attack.mitre.org/software/S0002)   |

[Mimikatz](https://attack.mitre.org/software/S0002) performs credential dumping to obtain account and password information useful in gaining access to additional systems and enterprise network resources. It contains functionality to acquire information about credentials in many ways, including from DPAPI.[[70]](https://github.com/gentilkiwi/mimikatz)[[71]](https://github.com/gentilkiwi/mimikatz/wiki/module-~-lsadump)[[72]](https://www.dsinternals.com/en/retrieving-dpapi-backup-keys-from-active-directory/)[[73]](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)

|  [S9062](https://attack.mitre.org/software/S9062)   |  [MiniBrowse](https://attack.mitre.org/software/S9062)   |

[MiniBrowse](https://attack.mitre.org/software/S9062) has stolen credentials from Chrome and Edge browsers.[[74]](https://research.checkpoint.com/2025/nimbus-manticore-deploys-new-malware-targeting-europe/)

|  [S9022](https://attack.mitre.org/software/S9022)   |  [MirrorStealer](https://attack.mitre.org/software/S9022)   |

[MirrorStealer](https://attack.mitre.org/software/S9022) can steal credentials stored in browsers.[[75]](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)[[76]](https://www.trendmicro.com/en_us/research/24/k/lodeinfo-campaign-of-earth-kasha.html)

|  [S1122](https://attack.mitre.org/software/S1122)   |  [Mispadu](https://attack.mitre.org/software/S1122)   |

[Mispadu](https://attack.mitre.org/software/S1122) can steal credentials from Google Chrome.[[66]](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)[[77]](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)[[78]](https://www.metabaseq.com/mispadu-banking-trojan/)

|  [G0021](https://attack.mitre.org/groups/G0021)   |  [Molerats](https://attack.mitre.org/groups/G0021)   |

[Molerats](https://attack.mitre.org/groups/G0021) used the public tool BrowserPasswordDump10 to dump passwords saved in browsers on victims.[[79]](https://www.clearskysec.com/wp-content/uploads/2016/01/Operation%20DustySky_TLP_WHITE.pdf)

|  [G0069](https://attack.mitre.org/groups/G0069)   |  [MuddyWater](https://attack.mitre.org/groups/G0069)   |

[MuddyWater](https://attack.mitre.org/groups/G0069) has run tools including Browser64 to steal passwords saved in victim web browsers.[[80]](https://www.symantec.com/blogs/threat-intelligence/seedworm-espionage-group)[[81]](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)

|  [S0198](https://attack.mitre.org/software/S0198)   |  [NETWIRE](https://attack.mitre.org/software/S0198)   |

[NETWIRE](https://attack.mitre.org/software/S0198) has the ability to steal credentials from web browsers including Internet Explorer, Opera, Yandex, and Chrome.[[82]](https://www.mandiant.com/resources/blog/dissecting-netwire-phishing-campaigns-usage-process-hollowing)[[83]](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)[[84]](https://www.proofpoint.com/us/blog/threat-insight/geofenced-netwire-campaigns)

|  [S0385](https://attack.mitre.org/software/S0385)   |  [njRAT](https://attack.mitre.org/software/S0385)   |

[njRAT](https://attack.mitre.org/software/S0385) has a module that steals passwords saved in victim web browsers.[[85]](https://www.threatminer.org/_reports/2013/fta-1009---njrat-uncovered-1.pdf)[[86]](https://blog.trendmicro.com/trendlabs-security-intelligence/autoit-compiled-worm-affecting-removable-media-delivers-fileless-version-of-bladabindi-njrat-backdoor/)[[87]](https://citizenlab.ca/2016/08/group5-syria/)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has used credential dumping tools such as [LaZagne](https://attack.mitre.org/software/S0349) to steal credentials to accounts logged into the compromised system and to Outlook Web Access.[[88]](https://web.archive.org/web/20230610050742/https://pan-unit42.github.io/playbook_viewer/?pb=evasive-serpens)[[89]](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)[[90]](https://static.carahsoft.com/concrete/files/1015/2779/3571/M-Trends-2018-Report.pdf)[[91]](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html) [OilRig](https://attack.mitre.org/groups/G0049) has also used tool named PICKPOCKET to dump passwords from web browsers.[[91]](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)

|  [S0138](https://attack.mitre.org/software/S0138)   |  [OLDBAIT](https://attack.mitre.org/software/S0138)   |

[OLDBAIT](https://attack.mitre.org/software/S0138) collects credentials from Internet Explorer, Mozilla Firefox, and Eudora.[[92]](https://web.archive.org/web/20151022204649/https://www.fireeye.com/content/dam/fireeye-www/global/en/current-threats/pdfs/rpt-apt28.pdf)

|  [S0365](https://attack.mitre.org/software/S0365)   |  [Olympic Destroyer](https://attack.mitre.org/software/S0365)   |

[Olympic Destroyer](https://attack.mitre.org/software/S0365) contains a module that tries to obtain stored credentials from web browsers.[[1]](https://blog.talosintelligence.com/2018/02/olympic-destroyer.html)

|  [G0040](https://attack.mitre.org/groups/G0040)   |  [Patchwork](https://attack.mitre.org/groups/G0040)   |

[Patchwork](https://attack.mitre.org/groups/G0040) dumped the login data database from \AppData\Local\Google\Chrome\User Data\Default\Login Data.[[93]](https://web.archive.org/web/20180825085952/https:/s3-us-west-2.amazonaws.com/cymmetria-blog/public/Unveiling_Patchwork.pdf)

|  [S0048](https://attack.mitre.org/software/S0048)   |  [PinchDuke](https://attack.mitre.org/software/S0048)   |

[PinchDuke](https://attack.mitre.org/software/S0048) steals credentials from compromised hosts. [PinchDuke](https://attack.mitre.org/software/S0048)'s credential stealing functionality is believed to be based on the source code of the Pinch credential stealing malware (also known as LdPinch). Credentials targeted by [PinchDuke](https://attack.mitre.org/software/S0048) include ones associated with many sources such as Netscape Navigator, Mozilla Firefox, Mozilla Thunderbird, and Internet Explorer. [[30]](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)

|  [S0435](https://attack.mitre.org/software/S0435)   |  [PLEAD](https://attack.mitre.org/software/S0435)   |

[PLEAD](https://attack.mitre.org/software/S0435) can harvest saved credentials from browsers such as Google Chrome, Microsoft Internet Explorer, and Mozilla Firefox.[[94]](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)[[95]](https://www.welivesecurity.com/2018/07/09/certificates-stolen-taiwanese-tech-companies-plead-malware-campaign/)

|  [S0428](https://attack.mitre.org/software/S0428)   |  [PoetRAT](https://attack.mitre.org/software/S0428)   |

[PoetRAT](https://attack.mitre.org/software/S0428) has used a Python tool named Browdec.exe to steal browser credentials.[[96]](https://blog.talosintelligence.com/2020/04/poetrat-covid-19-lures.html)

|  [S0113](https://attack.mitre.org/software/S0113)   |  [Prikormka](https://attack.mitre.org/software/S0113)   |

A module in [Prikormka](https://attack.mitre.org/software/S0113) gathers logins and passwords stored in applications on the victims, including Google Chrome, Mozilla Firefox, and several other browsers.[[97]](http://www.welivesecurity.com/wp-content/uploads/2016/05/Operation-Groundbait.pdf)

|  [S0279](https://attack.mitre.org/software/S0279)   |  [Proton](https://attack.mitre.org/software/S0279)   |

[Proton](https://attack.mitre.org/software/S0279) gathers credentials for Google Chrome.[[98]](https://objective-see.com/blog/blog_0x25.html)

|  [S0192](https://attack.mitre.org/software/S0192)   |  [Pupy](https://attack.mitre.org/software/S0192)   |

[Pupy](https://attack.mitre.org/software/S0192) can use Lazagne for harvesting credentials.[[99]](https://github.com/n1nj4sec/pupy)

|  [S0650](https://attack.mitre.org/software/S0650)   |  [QakBot](https://attack.mitre.org/software/S0650)   |

[QakBot](https://attack.mitre.org/software/S0650) has collected usernames and passwords from Firefox and Chrome.[[100]](https://securelist.com/qakbot-technical-analysis/103931/)

|  [S0262](https://attack.mitre.org/software/S0262)   |  [QuasarRAT](https://attack.mitre.org/software/S0262)   |

[QuasarRAT](https://attack.mitre.org/software/S0262) can obtain passwords from common web browsers.[[101]](https://github.com/quasar/QuasarRAT)[[102]](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)[[103]](https://securelist.com/blindeagle-apt/113414/)

|  [S1148](https://attack.mitre.org/software/S1148)   |  [Raccoon Stealer](https://attack.mitre.org/software/S1148)   |

[Raccoon Stealer](https://attack.mitre.org/software/S1148) collects passwords, cookies, and autocomplete information from various popular web browsers.[[104]](https://blog.sekoia.io/raccoon-stealer-v2-part-2-in-depth-analysis/)

|  [S0629](https://attack.mitre.org/software/S0629)   |  [RainyDay](https://attack.mitre.org/software/S0629)   |

[RainyDay](https://attack.mitre.org/software/S0629) can use tools to collect credentials from web browsers.[[105]](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)

|  [G1039](https://attack.mitre.org/groups/G1039)   |  [RedCurl](https://attack.mitre.org/groups/G1039)   |

[RedCurl](https://attack.mitre.org/groups/G1039) used [LaZagne](https://attack.mitre.org/software/S0349) to obtain passwords from web browsers.[[106]](https://www.group-ib.com/resources/research-hub/red-curl/)[[107]](https://www.group-ib.com/resources/research-hub/red-curl-2/)

|  [S0153](https://attack.mitre.org/software/S0153)   |  [RedLeaves](https://attack.mitre.org/software/S0153)   |

[RedLeaves](https://attack.mitre.org/software/S0153) can gather browser usernames and passwords.[[108]](http://web.archive.org/web/20220810112638/https:/www.accenture.com/t20180423T055005Z_w_/se-en/_acnmedia/PDF-76/Accenture-Hogfish-Threat-Analysis.pdf)

|  [S1240](https://attack.mitre.org/software/S1240)   |  [RedLine Stealer](https://attack.mitre.org/software/S1240)   |

[RedLine Stealer](https://attack.mitre.org/software/S1240) was designed to steal sensitive information from web browsers, including credit card details, saved credentials, and autocomplete data.[[109]](https://www.welivesecurity.com/en/eset-research/life-crooked-redline-analyzing-infamous-infostealers-backend/) [RedLine Stealer](https://attack.mitre.org/software/S1240) can also gather credentials from several browsers.[[110]](https://www.kroll.com/en/publications/cyber/redlinestealer-malware)[[111]](https://www.proofpoint.com/us/blog/threat-insight/new-redline-stealer-distributed-using-coronavirus-themed-email-campaign)[[112]](https://www.splunk.com/en_us/blog/security/do-not-cross-the-redline-stealer-detections-and-analysis.html)

|  [S0240](https://attack.mitre.org/software/S0240)   |  [ROKRAT](https://attack.mitre.org/software/S0240)   |

[ROKRAT](https://attack.mitre.org/software/S0240) can steal credentials stored in Web browsers by querying the sqlite database.[[113]](https://blog.talosintelligence.com/2018/01/korea-in-crosshairs.html)

|  [G0034](https://attack.mitre.org/groups/G0034)   |  [Sandworm Team](https://attack.mitre.org/groups/G0034)   |

[Sandworm Team](https://attack.mitre.org/groups/G0034)'s CredRaptor tool can collect saved passwords from various internet browsers.[[114]](https://www.welivesecurity.com/2016/12/13/rise-telebots-analyzing-disruptive-killdisk-attacks/)

|  [S0692](https://attack.mitre.org/software/S0692)   |  [SILENTTRINITY](https://attack.mitre.org/software/S0692)   |

[SILENTTRINITY](https://attack.mitre.org/software/S0692) can collect clear text web credentials for Internet Explorer/Edge.[[115]](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)

|  [S0226](https://attack.mitre.org/software/S0226)   |  [Smoke Loader](https://attack.mitre.org/software/S0226)   |

[Smoke Loader](https://attack.mitre.org/software/S0226) searches for credentials stored from web browsers.[[116]](https://blog.talosintelligence.com/2018/07/smoking-guns-smoke-loader-learned-new.html#more)

|  [C0024](https://attack.mitre.org/campaigns/C0024)   |  [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024)   |

During the [SolarWinds Compromise](https://attack.mitre.org/campaigns/C0024), [APT29](https://attack.mitre.org/groups/G0016) stole users' saved passwords from Chrome.[[117]](https://www.crowdstrike.com/blog/observations-from-the-stellarparticle-campaign/)

|  [G0038](https://attack.mitre.org/groups/G0038)   |  [Stealth Falcon](https://attack.mitre.org/groups/G0038)   |

[Stealth Falcon](https://attack.mitre.org/groups/G0038) malware gathers passwords from multiple sources, including Internet Explorer, Firefox, and Chrome.[[118]](https://citizenlab.org/2016/05/stealth-falcon/)

|  [S1042](https://attack.mitre.org/software/S1042)   |  [SUGARDUMP](https://attack.mitre.org/software/S1042)   |

[SUGARDUMP](https://attack.mitre.org/software/S1042) variants have harvested credentials from browsers such as Firefox, Chrome, Opera, and Edge.[[119]](https://www.mandiant.com/resources/blog/suspected-iranian-actor-targeting-israeli-shipping)

|  [G0092](https://attack.mitre.org/groups/G0092)   |  [TA505](https://attack.mitre.org/groups/G0092)   |

[TA505](https://attack.mitre.org/groups/G0092) has used malware to gather credentials from Internet Explorer.[[120]](https://www.proofpoint.com/us/threat-insight/post/threat-actor-profile-ta505-dridex-globeimposter)

|  [S1201](https://attack.mitre.org/software/S1201)   |  [TRANSLATEXT](https://attack.mitre.org/software/S1201)   |

[TRANSLATEXT](https://attack.mitre.org/software/S1201) has stolen credentials stored in Chrome.[[121]](https://www.zscaler.com/blogs/security-research/kimsuky-deploys-translatext-target-south-korean-academia#technical-analysis)

|  [S0266](https://attack.mitre.org/software/S0266)   |  [TrickBot](https://attack.mitre.org/software/S0266)   |

[TrickBot](https://attack.mitre.org/software/S0266) can obtain passwords stored in files from web browsers such as Chrome, Firefox, Internet Explorer, and Microsoft Edge, sometimes using [esentutl](https://attack.mitre.org/software/S0404).[[122]](https://blog.trendmicro.com/trendlabs-security-intelligence/trickbot-shows-off-new-trick-password-grabber-module/)[[123]](https://www.cybereason.com/blog/dropping-anchor-from-a-trickbot-infection-to-the-discovery-of-the-anchor-malware)[[124]](https://www.bitdefender.com/files/News/CaseStudies/study/399/Bitdefender-PR-Whitepaper-Trickbot-creat5515-en-EN.pdf)

|  [S0094](https://attack.mitre.org/software/S0094)   |  [Trojan.Karagany](https://attack.mitre.org/software/S0094)   |

[Trojan.Karagany](https://attack.mitre.org/software/S0094) can steal data and credentials from browsers.[[125]](https://www.secureworks.com/research/updated-karagany-malware-targets-energy-sector)

|  [S0436](https://attack.mitre.org/software/S0436)   |  [TSCookie](https://attack.mitre.org/software/S0436)   |

[TSCookie](https://attack.mitre.org/software/S0436) has the ability to steal saved passwords from the Internet Explorer, Edge, Firefox, and Chrome browsers.[[126]](https://blogs.jpcert.or.jp/en/2018/03/malware-tscooki-7aa0.html)

|  [S0130](https://attack.mitre.org/software/S0130)   |  [Unknown Logger](https://attack.mitre.org/software/S0130)   |

[Unknown Logger](https://attack.mitre.org/software/S0130) is capable of stealing usernames and passwords from browsers on the victim machine.[[127]](https://www.forcepoint.com/sites/default/files/resources/files/forcepoint-security-labs-monsoon-analysis-report.pdf)

|  [S9050](https://attack.mitre.org/software/S9050)   |  [Vidar](https://attack.mitre.org/software/S9050)   |

[Vidar](https://attack.mitre.org/software/S9050) has gathered credentials and data, including history, credit cards and auto-filled information, from decrypted browser files from Chrome, Firefox, Edge, Opera, Vivaldi, Waterfox and Palemoon.[[128]](https://www.intrinsec.com/wp-content/uploads/2026/04/TLP_CLEAR-20260424-New_Vidar.pdf)[[129]](https://www.levelblue.com/blogs/spiderlabs-blog/unmasking-a-multi-stage-loader-autoit-abuse-leading-to-vidar-stealer-command-and-control-communication)[[130]](https://unit42.paloaltonetworks.com/vidar-stealer-xmrig-miner-campaign-analysis/)[[131]](https://www.cyfirma.com/research/vidar-stealer-an-in-depth-analysis-of-an-information-stealing-malware/)[[132]](https://www.quorumcyber.com/wp-content/uploads/2023/01/Malware-Analysis-Vidar.pdf)

|  [G1017](https://attack.mitre.org/groups/G1017)   |  [Volt Typhoon](https://attack.mitre.org/groups/G1017)   |

[Volt Typhoon](https://attack.mitre.org/groups/G1017) has targeted network administrator browser data including browsing history and stored credentials.[[133]](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)

|  [S0670](https://attack.mitre.org/software/S0670)   |  [WarzoneRAT](https://attack.mitre.org/software/S0670)   |

[WarzoneRAT](https://attack.mitre.org/software/S0670) has the capability to grab passwords from numerous web browsers as well as from Outlook and Thunderbird email clients.[[134]](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)[[135]](https://www.uptycs.com/blog/warzone-rat-comes-with-uac-bypass-technique)

|  [S0161](https://attack.mitre.org/software/S0161)   |  [XAgentOSX](https://attack.mitre.org/software/S0161)   |

[XAgentOSX](https://attack.mitre.org/software/S0161) contains the getFirefoxPassword function to attempt to locate Firefox passwords.[[136]](https://researchcenter.paloaltonetworks.com/2017/02/unit42-xagentosx-sofacys-xagent-macos-tool/)

|  [S1207](https://attack.mitre.org/software/S1207)   |  [XLoader](https://attack.mitre.org/software/S1207)   |

[XLoader](https://attack.mitre.org/software/S1207) can gather credentials from several web browsers.[[137]](https://www.zscaler.com/blogs/security-research/technical-analysis-xloader-versions-6-and-7-part-1)[[138]](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)[[139]](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)

|  [S0251](https://attack.mitre.org/software/S0251)   |  [Zebrocy](https://attack.mitre.org/software/S0251)   |

[Zebrocy](https://attack.mitre.org/software/S0251) has the capability to upload dumper tools that extract credentials from web browsers and store them in database files.[[140]](https://www.welivesecurity.com/2019/05/22/journey-zebrocy-land/)

|  [G0128](https://attack.mitre.org/groups/G0128)   |  [ZIRCONIUM](https://attack.mitre.org/groups/G0128)   |

[ZIRCONIUM](https://attack.mitre.org/groups/G0128) has used a tool to steal credentials from installed web browsers including Microsoft Internet Explorer and Google Chrome.[[141]](https://www.zscaler.com/blogs/security-research/apt-31-leverages-covid-19-vaccine-theme-and-abuses-legitimate-online)

## Mitigations

| ID  | Mitigation  | Description

|  [M1027](https://attack.mitre.org/mitigations/M1027)   |  [Password Policies](https://attack.mitre.org/mitigations/M1027)   |

Organizations may consider weighing the risk of storing credentials in web browsers. If web browser credential disclosure is a significant concern, technical controls, policy, and user training may be used to prevent storage of credentials in web browsers.

|  [M1021](https://attack.mitre.org/mitigations/M1021)   |  [Restrict Web-Based Content](https://attack.mitre.org/mitigations/M1021)   |

Restrict or block web-based content that could be used to extract session cookies or credentials stored in browsers. Use browser security settings, such as disabling third-party cookies and restricting browser extensions, to limit the attack surface.

|  [M1051](https://attack.mitre.org/mitigations/M1051)   |  [Update Software](https://attack.mitre.org/mitigations/M1051)   |

Regularly update web browsers, password managers, and all related software to the latest versions. Keeping software up-to-date reduces the risk of vulnerabilities being exploited by attackers to extract stored credentials or session cookies.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Implement strict user account management policies to prevent unnecessary accounts from accessing sensitive systems. Regularly audit user accounts to identify and disable inactive accounts that may be targeted by attackers to extract credentials or gain unauthorized access.

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Provide user training on secure practices for managing credentials, including avoiding storing sensitive passwords in browsers and using password managers securely. Users should also be educated on identifying phishing attempts that could steal session cookies or credentials.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0037](https://attack.mitre.org/detectionstrategies/DET0037)   |  [Detect Suspicious Access to Browser Credential Stores](https://attack.mitre.org/detectionstrategies/DET0037)   |  [AN0105](https://attack.mitre.org/detectionstrategies/DET0037#AN0105)   |

Detects unauthorized access to web browser credential stores (e.g., Chrome Login Data, Edge Credential Locker) by processes other than the browser itself. Correlates file reads of credential databases with subsequent API calls to CryptUnprotectData or memory inspection attempts.

|  [AN0106](https://attack.mitre.org/detectionstrategies/DET0037#AN0106)   |

Detects attempts to access browser credential stores (e.g., Firefox logins.json, Chrome SQLite DB) or processes (e.g., gnome-keyring-daemon). Observes unauthorized file reads and memory inspection of browser processes using ptrace or gdb.

|  [AN0107](https://attack.mitre.org/detectionstrategies/DET0037#AN0107)   |

Detects abnormal access to Safari credential stores (Keychain-backed) or Chrome/Firefox login databases. Observes processes executing security dump-keychain or directly reading credential files in ~/Library/Application Support. Correlates file access with suspicious process ancestry or unsigned binaries.

## References

-    [Mercer, W. and Rascagneres, P. (2018, February 12). Olympic Destroyer Takes Aim At Winter Olympics. Retrieved March 14, 2019.](https://blog.talosintelligence.com/2018/02/olympic-destroyer.html)
-    [Microsoft. (2018, April 12). CryptUnprotectData function. Retrieved June 18, 2019.](https://docs.microsoft.com/en-us/windows/desktop/api/dpapi/nf-dpapi-cryptunprotectdata)
-    [Proofpoint. (2018, May 10). New Vega Stealer shines brightly in targeted campaign . Retrieved June 18, 2019.](https://www.proofpoint.com/us/threat-insight/post/new-vega-stealer-shines-brightly-targeted-campaign)
-    [Swapnil Patil, Yogesh Londhe. (2017, July 25). HawkEye Credential Theft Malware Distributed in Recent Phishing Campaign. Retrieved June 18, 2019.](https://www.fireeye.com/blog/threat-research/2017/07/hawkeye-malware-distributed-in-phishing-campaign.html)
-    [Jamieson O'Reilly (putterpanda). (2016, July 4). mimikittenz. Retrieved June 20, 2019.](https://github.com/putterpanda/mimikittenz)
-    [Arsene, L. (2020, April 21). Oil & Gas Spearphishing Campaigns Drop Agent Tesla Spyware in Advance of Historic OPEC+ Deal. Retrieved May 19, 2020.](https://labs.bitdefender.com/2020/04/oil-gas-spearphishing-campaigns-drop-agent-tesla-spyware-in-advance-of-historic-opec-deal/)
-    [Check Point Software Technologies. (2015). ROCKET KITTEN: A CAMPAIGN WITH 9 LIVES. Retrieved March 16, 2018.](https://blog.checkpoint.com/wp-content/uploads/2015/11/rocket-kitten-report.pdf)
-    [Symantec Security Response. (2016, September 6). Buckeye cyberespionage group shifts gaze from US to Hong Kong. Retrieved September 26, 2016.](https://web.archive.org/web/20160910124439/http://www.symantec.com/connect/blogs/buckeye-cyberespionage-group-shifts-gaze-us-hong-kong)
-    [Security Response attack Investigation Team. (2019, March 27). Elfin: Relentless Espionage Group Targets Multiple Organizations in Saudi Arabia and U.S.. Retrieved April 10, 2019.](https://www.symantec.com/blogs/threat-intelligence/elfin-apt33-espionage)
-    [Ackerman, G., et al. (2018, December 21). OVERRULED: Containing a Potentially Destructive Adversary. Retrieved January 17, 2019.](https://www.fireeye.com/blog/threat-research/2018/12/overruled-containing-a-potentially-destructive-adversary.html)
-    [FireEye. (2018, February 20). APT37 (Reaper): The Overlooked North Korean Actor. Retrieved November 17, 2024.](https://services.google.com/fh/files/misc/apt37-reaper-the-overlooked-north-korean-actor.pdf)
-    [Microsoft Threat Intelligence. (2026, April 16). Dissecting Sapphire Sleet’s macOS intrusion from lure to compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/16/dissecting-sapphire-sleets-macos-intrusion-from-lure-to-compromise/)
-    [Microsoft Threat Intelligence. (2026, June 17). From package to postinstall payload: Inside the Mastra npm supply chain compromise by Sapphire Sleet. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)
-    [Nikita Rostovcev. (2022, August 18). APT41 World Tour 2021 on a tight schedule. Retrieved February 22, 2024.](https://www.group-ib.com/blog/apt41-world-tour-2021/)
-    [Mandiant. (n.d.). APT42: Crooked Charms, Cons and Compromises. Retrieved October 9, 2024.](https://services.google.com/fh/files/misc/apt42-crooked-charms-cons-and-compromises.pdf)
-    [Yan, T., et al. (2018, November 21). New Wine in Old Bottle: New Azorult Variant Found in FindMyName Campaign using Fallout Exploit Kit. Retrieved November 29, 2018.](https://researchcenter.paloaltonetworks.com/2018/11/unit42-new-wine-old-bottle-new-azorult-variant-found-findmyname-campaign-using-fallout-exploit-kit/)
-    [Symantec Security Response. (2014, June 30). Dragonfly: Cyberespionage Attacks Against Energy Suppliers. Retrieved April 8, 2016.](https://community.broadcom.com/symantecenterprise/communities/community-home/librarydocuments/viewdocument?DocumentKey=7382dce7-0260-4782-84cc-890971ed3f17&CommunityKey=1ecf5f55-9545-44d6-b0f4-4e4a7f5f5e68&tab=librarydocuments)
-    [eSentire Threat Response Unit (TRU). (2024, November 14). Bored BeaverTail & InvisibleFerret Yacht Club – A Lazarus Lure Pt.2. Retrieved October 17, 2025.](https://www.esentire.com/blog/bored-beavertail-invisibleferret-yacht-club-a-lazarus-lure-pt-2)
-    [Kirill Boychenko. (2025, July 14). Contagious Interview Campaign Escalates With 67 Malicious npm Packages and New Malware Loader. Retrieved October 19, 2025.](https://socket.dev/blog/contagious-interview-campaign-escalates-67-malicious-npm-packages)
-    [Kirill Boychenko. (2025, June 25). Another Wave: North Korean Contagious Interview Campaign Drops 35 New Malicious npm Packages. Retrieved October 19, 2025.](https://socket.dev/blog/north-korean-contagious-interview-campaign-drops-35-new-malicious-npm-packages)
-    [Unit42. (2024, October 9). Contagious Interview: DPRK Threat Actors Lure Tech Industry Job Seekers to Install New Variants of BeaverTail and InvisibleFerret Malware. Retrieved October 17, 2025.](https://unit42.paloaltonetworks.com/north-korean-threat-actors-lure-tech-job-seekers-as-fake-recruiters/)
-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [F-Secure Labs. (2014). BlackEnergy & Quedagh: The convergence of crimeware and APT attacks. Retrieved March 24, 2016.](https://blog-assets.f-secure.com/wp-content/uploads/2019/10/15163408/BlackEnergy_Quedagh.pdf)
-    [Baumgartner, K. and Garnaeva, M.. (2014, November 3). BE2 custom plugins, router abuse, and target profiles. Retrieved March 24, 2016.](https://securelist.com/be2-custom-plugins-router-abuse-and-target-profiles/67353/)
-    [Cash, D., Grunzweig, J., Meltzer, M., Adair, S., Lancaster, T. (2021, August 17). North Korean APT InkySquid Infects Victims Using Browser Exploits. Retrieved September 30, 2021.](https://www.volexity.com/blog/2021/08/17/north-korean-apt-inkysquid-infects-victims-using-browser-exploits/)
-    [Giuliani, M., Allievi, A. (2011, February 28). Carberp - a modular information stealing trojan. Retrieved September 12, 2024.](https://web.archive.org/web/20231227000328/http://pxnow.prevx.com/content/blog/carberp-a_modular_information_stealing_trojan.pdf)
-    [Salem, E. (2020, November 17). CHAES: Novel Malware Targeting Latin American E-Commerce. Retrieved June 30, 2021.](https://www.cybereason.com/hubfs/dam/collateral/reports/11-2020-Chaes-e-commerce-malware-research.pdf)
-    [PwC and BAE Systems. (2017, April). Operation Cloud Hopper: Technical Annex. Retrieved April 13, 2017.](https://www.pwc.co.uk/cyber-security/pdf/pwc-uk-operation-cloud-hopper-technical-annex-april-2017.pdf)
-    [Chen, y., et al. (2019, January 31). Mac Malware Steals Cryptocurrency Exchanges’ Cookies. Retrieved July 22, 2020.](https://unit42.paloaltonetworks.com/mac-malware-steals-cryptocurrency-exchanges-cookies/)
-    [F-Secure Labs. (2015, September 17). The Dukes: 7 years of Russian cyberespionage. Retrieved December 10, 2015.](https://www.f-secure.com/documents/996508/1030745/dukes_whitepaper.pdf)
-    [El-Banna, M., et al. (2025, November 17). Frontline Intelligence: Analysis of UNC1549 TTPs, Custom Tools, and Malware Targeting the Aerospace and Defense Ecosystem. Retrieved September 28, 2026.](https://cloud.google.com/blog/topics/threat-intelligence/analysis-of-unc1549-ttps-targeting-aerospace-defense)
-    [Huss, D. (2016, March 1). Operation Transparent Tribe. Retrieved June 8, 2016.](https://www.proofpoint.com/sites/default/files/proofpoint-operation-transparent-tribe-threat-insight-en.pdf)
-    [Dedola, G. (2020, August 20). Transparent Tribe: Evolution analysis, part 1. Retrieved September 2, 2021.](https://securelist.com/transparent-tribe-part-1/98127/)
-    [Trend Micro. (2019, January 16). Exploring Emotet's Activities . Retrieved March 25, 2019.](https://documents.trendmicro.com/assets/white_papers/ExploringEmotetsActivities_Final.pdf)
-    [Kessem, L., et al. (2017, November 13). New Banking Trojan IcedID Discovered by IBM X-Force Research. Retrieved July 14, 2020.](https://securityintelligence.com/new-banking-trojan-icedid-discovered-by-ibm-x-force-research/)
-    [Schroeder, W., Warner, J., Nelson, M. (n.d.). Github PowerShellEmpire. Retrieved April 28, 2016.](https://github.com/PowerShellEmpire/Empire)
-    [Visa Public. (2019, February). FIN6 Cybercrime Group Expands Threat to eCommerce Merchants. Retrieved September 16, 2019.](https://usa.visa.com/dam/VCOM/global/support-legal/documents/fin6-cybercrime-group-expands-threat-To-ecommerce-merchants.pdf)
-    [Gal Hachamov. (2025, December 29). GlassWorm Goes Mac: Fresh Infrastructure, New Tricks. Retrieved April 10, 2026.](https://www.koi.ai/blog/glassworm-goes-mac-fresh-infrastructure-new-tricks)
-    [Kirill Boychenko. (2026, January 31). GlassWorm Loader Hits Open VSX via Developer Account Compromise. Retrieved April 10, 2026.](https://socket.dev/blog/glassworm-loader-hits-open-vsx-via-suspected-developer-account-compromise)
-    [Abramov, D. (2020, April 13). Grandoreiro Malware Now Targeting Banks in Spain. Retrieved November 12, 2020.](https://securityintelligence.com/posts/grandoreiro-malware-now-targeting-banks-in-spain/)
-    [ESET. (2020, April 28). Grandoreiro: How engorged can an EXE get?. Retrieved November 13, 2020.](https://www.welivesecurity.com/2020/04/28/grandoreiro-how-engorged-can-exe-get/)
-    [Reynolds, J.. (2016, September 14). H1N1: Technical analysis reveals new capabilities – part 2. Retrieved November 17, 2024.](https://web.archive.org/web/20231210122239/https://blogs.cisco.com/security/h1n1-technical-analysis-reveals-new-capabilities-part-2)
-    [Kayal, A. et al. (2021, October). LYCEUM REBORN: COUNTERINTELLIGENCE IN THE MIDDLE EAST. Retrieved June 14, 2022.](https://vblocalhost.com/uploads/VB2021-Kayal-etal.pdf)
-    [QiAnXin Threat Intelligence Center. (2019, February 18). APT-C-36: Continuous Attacks Targeting Colombian Government Institutions and Corporations. Retrieved May 5, 2020.](https://web.archive.org/web/20190625182633if_/https://ti.360.net/blog/articles/apt-c-36-continuous-attacks-targeting-colombian-government-institutions-and-corporations-en/)
-    [Symantec. (2018, March 14). Inception Framework: Alive and Well, and Hiding Behind Proxies. Retrieved May 8, 2020.](https://symantec-enterprise-blogs.security.com/blogs/threat-intelligence/inception-framework-hiding-behind-proxies)
-    [Seongsu Park. (2024, November 4). From Pyongyang to Your Payroll: The Rise of North Korean Remote Workers in the West. Retrieved October 17, 2025.](https://www.zscaler.com/blogs/security-research/pyongyang-your-payroll-rise-north-korean-remote-workers-west)
-    [GReAT. (2020, July 14). The Tetrade: Brazilian banking malware goes global. Retrieved November 9, 2020.](https://securelist.com/the-tetrade-brazilian-banking-malware/97779/)
-    [Kamluk, V. & Gostev, A. (2016, February). Adwind - A Cross-Platform RAT. Retrieved April 23, 2019.](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07195002/KL_AdwindPublicReport_2016.pdf)
-    [Hromcova, Z. and Burgher, A. (2023, September 21). OilRig’s Outer Space and Juicy Mix: Same ol’ rig, new drill pipes. Retrieved November 21, 2024.](https://www.welivesecurity.com/en/eset-research/oilrigs-outer-space-juicy-mix-same-ol-rig-new-drill-pipes/)
-    [Guarnieri, C., Schloesser M. (2013, June 7). KeyBoy, Targeted Attacks against Vietnam and India. Retrieved June 14, 2019.](https://blog.rapid7.com/2013/06/07/keyboy-targeted-attacks-against-vietnam-and-india/)
-    [Dahan, A. et al. (2020, November 2). Back to the Future: Inside the Kimsuky KGH Spyware Suite. Retrieved November 6, 2020.](https://www.cybereason.com/blog/back-to-the-future-inside-the-kimsuky-kgh-spyware-suite)
-    [Cimpanu, C.. (2018, December 5). Cyber-espionage group uses Chrome extension to infect victims. Retrieved August 26, 2019.](https://www.zdnet.com/article/cyber-espionage-group-uses-chrome-extension-to-infect-victims/)
-    [CISA, FBI, CNMF. (2020, October 27). https://us-cert.cisa.gov/ncas/alerts/aa20-301a. Retrieved November 4, 2020.](https://us-cert.cisa.gov/ncas/alerts/aa20-301a)
-    [ASERT team. (2018, December 5). STOLEN PENCIL Campaign Targets Academia. Retrieved February 5, 2019.](https://asert.arbornetworks.com/stolen-pencil-campaign-targets-academia/)
-    [An, J and Malhotra, A. (2021, November 10). North Korean attackers use malicious blogs to deliver malware to high-profile South Korean targets. Retrieved December 29, 2021.](https://blog.talosintelligence.com/2021/11/kimsuky-abuses-blogs-delivers-malware.html)
-    [Rascagneres, P. (2017, May 03). KONNI: A Malware Under The Radar For Years. Retrieved November 5, 2018.](https://blog.talosintelligence.com/2017/05/konni-malware-under-radar-for-years.html)
-    [MSTIC, DART, M365 Defender. (2022, March 24). DEV-0537 Criminal Actor Targeting Organizations for Data Exfiltration and Destruction. Retrieved May 17, 2022.](https://www.microsoft.com/security/blog/2022/03/22/dev-0537-criminal-actor-targeting-organizations-for-data-exfiltration-and-destruction/)
-    [Zanni, A. (n.d.). The LaZagne Project !!!. Retrieved December 14, 2018.](https://github.com/AlessandroZ/LaZagne)
-    [Symantec Security Response. (2018, July 25). Leafminer: New Espionage Campaigns Targeting Middle Eastern Regions. Retrieved August 28, 2018.](https://www.symantec.com/blogs/threat-intelligence/leafminer-espionage-middle-east)
-    [BI.ZONE Cyber Threats Research Team. (2021, May 13). From pentest to APT attack: cybercriminal group FIN7 disguises its malware as an ethical hacker’s toolkit. Retrieved February 2, 2022.](https://bi-zone.medium.com/from-pentest-to-apt-attack-cybercriminal-group-fin7-disguises-its-malware-as-an-ethical-hackers-c23c9a75e319)
-    [Hoang, M. (2019, January 31). Malicious Activity Report: Elements of Lokibot Infostealer. Retrieved May 15, 2020.](https://insights.infoblox.com/threat-intelligence-reports/threat-intelligence--22)
-    [Cybereaon Security Services Team. (n.d.). Your Data Is Under New Lummanagement: The Rise of LummaStealer. Retrieved March 22, 2025.](https://www.cybereason.com/blog/threat-analysis-rise-of-lummastealer)
-    [Cara Lin, Fortinet. (2024, January 8). Deceptive Cracked Software Spreads Lumma Variant on YouTube. Retrieved March 22, 2025.](https://www.fortinet.com/blog/threat-research/lumma-variant-on-youtube)
-    [Buddy Tancio, Fe Cureg, and Jovit Samaniego, Trend Micro. (2025, January 30). Lumma Stealer’s GitHub-Based Delivery Explored via Managed Detection and Response. Retrieved March 22, 2025.](https://www.trendmicro.com/en_us/research/25/a/lumma-stealers-github-based-delivery-via-mdr.html)
-    [ESET. (2019, July). MACHETE JUST GOT SHARPER Venezuelan government institutions under attack. Retrieved September 13, 2019.](https://www.welivesecurity.com/wp-content/uploads/2019/08/ESET_Machete.pdf)
-    [SCILabs. (2021, December 23). Cyber Threat Profile Malteiro. Retrieved March 13, 2024.](https://blog.scilabs.mx/en/cyber-threat-profile-malteiro/)
-    [Asheer Malhotra & Vitor Ventura. (2022, August 2). Manjusaka: A Chinese sibling of Sliver and Cobalt Strike. Retrieved September 4, 2024.](https://blog.talosintelligence.com/manjusaka-offensive-framework/)
-    [Facundo Muñoz. (2023, April 26). Evasive Panda APT group delivers malware via updates for popular Chinese software. Retrieved July 25, 2024.](https://www.welivesecurity.com/2023/04/26/evasive-panda-apt-group-malware-updates-popular-chinese-software/)
-    [Threat Hunter Team. (2023, April 20). Daggerfly: APT Actor Targets Telecoms Company in Africa. Retrieved July 25, 2024.](https://symantec-enterprise-blogs.security.com/threat-intelligence/apt-attacks-telecoms-africa-mgbot)
-    [Deply, B. (n.d.). Mimikatz. Retrieved September 29, 2015.](https://github.com/gentilkiwi/mimikatz)
-    [Deply, B., Le Toux, V. (2016, June 5). module ~ lsadump. Retrieved August 7, 2017.](https://github.com/gentilkiwi/mimikatz/wiki/module-~-lsadump)

-    [Grafnetter, M. (2015, October 26). Retrieving DPAPI Backup Keys from Active Directory. Retrieved December 19, 2017.](https://www.dsinternals.com/en/retrieving-dpapi-backup-keys-from-active-directory/)
-    [The Australian Cyber Security Centre (ACSC), the Canadian Centre for Cyber Security (CCCS), the New Zealand National Cyber Security Centre (NZ NCSC), CERT New Zealand, the UK National Cyber Security Centre (UK NCSC) and the US National Cybersecurity and Communications Integration Center (NCCIC). (2018, October 11). Joint report on publicly available hacking tools. Retrieved March 11, 2019.](https://www.ncsc.gov.uk/report/joint-report-on-publicly-available-hacking-tools)
-    [Check Point. (2025, September 22). Nimbus Manticore Deploys New Malware Targeting Europe. Retrieved September 24, 2026.](https://research.checkpoint.com/2025/nimbus-manticore-deploys-new-malware-targeting-europe/)
-    [Breitenbacher, D. (2022, December 14). Unmasking MirrorFace: Operation LiberalFace targeting Japanese political entities. Retrieved April 17, 2026.](https://www.welivesecurity.com/2022/12/14/unmasking-mirrorface-operation-liberalface-targeting-japanese-political-entities/)
-    [Trend Micro. (2024, November 19). Spot the Difference: Earth Kasha's New LODEINFO Campaign And The Correlation Analysis With The APT10 Umbrella. Retrieved April 17, 2026.](https://www.trendmicro.com/en_us/research/24/k/lodeinfo-campaign-of-earth-kasha.html)
-    [ESET Security. (2019, November 19). Mispadu: Advertisement for a discounted Unhappy Meal. Retrieved March 13, 2024.](https://www.welivesecurity.com/2019/11/19/mispadu-advertisement-discounted-unhappy-meal/)
-    [Garcia, F., Regalado, D. (2023, March 7). Inside Mispadu massive infection campaign in LATAM. Retrieved March 15, 2024.](https://www.metabaseq.com/mispadu-banking-trojan/)
-    [ClearSky. (2016, January 7). Operation DustySky. Retrieved January 8, 2016.](https://www.clearskysec.com/wp-content/uploads/2016/01/Operation%20DustySky_TLP_WHITE.pdf)
-    [Symantec DeepSight Adversary Intelligence Team. (2018, December 10). Seedworm: Group Compromises Government Agencies, Oil & Gas, NGOs, Telecoms, and IT Firms. Retrieved December 14, 2018.](https://www.symantec.com/blogs/threat-intelligence/seedworm-espionage-group)
-    [Peretz, A. and Theck, E. (2021, March 5). Earth Vetala – MuddyWater Continues to Target Organizations in the Middle East. Retrieved March 18, 2021.](https://www.trendmicro.com/en_us/research/21/c/earth-vetala---muddywater-continues-to-target-organizations-in-t.html)
-    [Maniath, S. and Kadam P. (2019, March 19). Dissecting a NETWIRE Phishing Campaign's Usage of Process Hollowing. Retrieved January 7, 2021.](https://www.mandiant.com/resources/blog/dissecting-netwire-phishing-campaigns-usage-process-hollowing)
-    [Lambert, T. (2020, January 29). Intro to Netwire. Retrieved January 7, 2021.](https://redcanary.com/blog/netwire-remote-access-trojan-on-linux/)
-    [Proofpoint. (2020, December 2). Geofenced NetWire Campaigns. Retrieved January 7, 2021.](https://www.proofpoint.com/us/blog/threat-insight/geofenced-netwire-campaigns)
-    [Fidelis Cybersecurity. (2013, June 28). Fidelis Threat Advisory #1009: "njRAT" Uncovered. Retrieved June 4, 2019.](https://www.threatminer.org/_reports/2013/fta-1009---njrat-uncovered-1.pdf)
-    [Pascual, C. (2018, November 27). AutoIt-Compiled Worm Affecting Removable Media Delivers Fileless Version of BLADABINDI/njRAT Backdoor. Retrieved June 4, 2019.](https://blog.trendmicro.com/trendlabs-security-intelligence/autoit-compiled-worm-affecting-removable-media-delivers-fileless-version-of-bladabindi-njrat-backdoor/)
-    [Scott-Railton, J., et al. (2016, August 2). Group5: Syria and the Iranian Connection. Retrieved September 26, 2016.](https://citizenlab.ca/2016/08/group5-syria/)
-    [Unit42. (2016, May 1). Evasive Serpens Unit 42 Playbook Viewer. Retrieved February 6, 2023.](https://web.archive.org/web/20230610050742/https://pan-unit42.github.io/playbook_viewer/?pb=evasive-serpens)
-    [Davis, S. and Caban, D. (2017, December 19). APT34 - New Targeted Attack in the Middle East. Retrieved December 20, 2017.](https://www.brighttalk.com/webcast/10703/296317/apt34-new-targeted-attack-in-the-middle-east)
-    [Mandiant. (2018). Mandiant M-Trends 2018. Retrieved November 17, 2024.](https://static.carahsoft.com/concrete/files/1015/2779/3571/M-Trends-2018-Report.pdf)
-    [Bromiley, M., et al.. (2019, July 18). Hard Pass: Declining APT34’s Invite to Join Their Professional Network. Retrieved August 26, 2019.](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)
-    [FireEye. (2015). APT28: A WINDOW INTO RUSSIA’S CYBER ESPIONAGE OPERATIONS?. Retrieved August 19, 2015.](https://web.archive.org/web/20151022204649/https://www.fireeye.com/content/dam/fireeye-www/global/en/current-threats/pdfs/rpt-apt28.pdf)
-    [Cymmetria. (2016). Unveiling Patchwork - The Copy-Paste APT. Retrieved November 17, 2024.](https://web.archive.org/web/20180825085952/https:/s3-us-west-2.amazonaws.com/cymmetria-blog/public/Unveiling_Patchwork.pdf)
-    [Bermejo, L., et al. (2017, June 22). Following the Trail of BlackTech’s Cyber Espionage Campaigns. Retrieved May 5, 2020.](https://blog.trendmicro.com/trendlabs-security-intelligence/following-trail-blacktech-cyber-espionage-campaigns/)
-    [Cherepanov, A.. (2018, July 9). Certificates stolen from Taiwanese tech‑companies misused in Plead malware campaign. Retrieved May 6, 2020.](https://www.welivesecurity.com/2018/07/09/certificates-stolen-taiwanese-tech-companies-plead-malware-campaign/)
-    [Mercer, W, et al. (2020, April 16). PoetRAT: Python RAT uses COVID-19 lures to target Azerbaijan public and private sectors. Retrieved April 27, 2020.](https://blog.talosintelligence.com/2020/04/poetrat-covid-19-lures.html)
-    [Cherepanov, A.. (2016, May 17). Operation Groundbait: Analysis of a surveillance toolkit. Retrieved May 18, 2016.](http://www.welivesecurity.com/wp-content/uploads/2016/05/Operation-Groundbait.pdf)
-    [Patrick Wardle. (n.d.). Mac Malware of 2017. Retrieved September 21, 2018.](https://objective-see.com/blog/blog_0x25.html)
-    [Nicolas Verdier. (n.d.). Retrieved January 29, 2018.](https://github.com/n1nj4sec/pupy)
-    [Kuzmenko, A. et al. (2021, September 2). QakBot technical analysis. Retrieved September 27, 2021.](https://securelist.com/qakbot-technical-analysis/103931/)
-    [MaxXor. (n.d.). QuasarRAT. Retrieved July 10, 2018.](https://github.com/quasar/QuasarRAT)
-    [Meltzer, M, et al. (2018, June 07). Patchwork APT Group Targets US Think Tanks. Retrieved July 16, 2018.](https://www.volexity.com/blog/2018/06/07/patchwork-apt-group-targets-us-think-tanks/)
-    [Global Research & Analysis Team, Kaspersky. (2024, August 19). BlindEagle flying high in Latin America. Retrieved April 16, 2026.](https://securelist.com/blindeagle-apt/113414/)
-    [Pierre Le Bourhis, Quentin Bourgue, & Sekoia TDR. (2022, June 29). Raccoon Stealer v2 - Part 2: In-depth analysis. Retrieved August 1, 2024.](https://blog.sekoia.io/raccoon-stealer-v2-part-2-in-depth-analysis/)
-    [Vrabie, V. (2021, April 23). NAIKON – Traces from a Military Cyber-Espionage Operation. Retrieved June 29, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/396/Bitdefender-PR-Whitepaper-NAIKON-creat5397-en-EN.pdf)
-    [Group-IB. (2020, August). RedCurl: The Pentest You Didn’t Know About. Retrieved August 9, 2024.](https://www.group-ib.com/resources/research-hub/red-curl/)
-    [Group-IB. (2021, November). RedCurl: The Awakening. Retrieved August 14, 2024.](https://www.group-ib.com/resources/research-hub/red-curl-2/)
-    [Accenture Security. (2018, April 23). Hogfish Redleaves Campaign. Retrieved July 2, 2018.](http://web.archive.org/web/20220810112638/https:/www.accenture.com/t20180423T055005Z_w_/se-en/_acnmedia/PDF-76/Accenture-Hogfish-Threat-Analysis.pdf)
-    [Alexandre Cote Cyr. (2024, November 8). Life on a crooked RedLine: Analyzing the infamous infostealer’s backend. Retrieved September 17, 2025.](https://www.welivesecurity.com/en/eset-research/life-crooked-redline-analyzing-infamous-infostealers-backend/)
-    [George Glass. (2024, August 14). REDLINESTEALER Malware Driving the Initial Access Broker Market. Retrieved September 17, 2025.](https://www.kroll.com/en/publications/cyber/redlinestealer-malware)
-    [Proofpoint Threat Insight Team, Jeremy H, Axel F. (2020, March 16). New Redline Password Stealer Malware. Retrieved September 17, 2025.](https://www.proofpoint.com/us/blog/threat-insight/new-redline-stealer-distributed-using-coronavirus-themed-email-campaign)
-    [Splunk Threat Research Team. (2023, June 1). Do Not Cross The 'RedLine' Stealer: Detections and Analysis. Retrieved September 17, 2025.](https://www.splunk.com/en_us/blog/security/do-not-cross-the-redline-stealer-detections-and-analysis.html)
-    [Mercer, W., Rascagneres, P. (2018, January 16). Korea In The Crosshairs. Retrieved May 21, 2018.](https://blog.talosintelligence.com/2018/01/korea-in-crosshairs.html)
-    [Cherepanov, A.. (2016, December 13). The rise of TeleBots: Analyzing disruptive KillDisk attacks. Retrieved June 10, 2020.](https://www.welivesecurity.com/2016/12/13/rise-telebots-analyzing-disruptive-killdisk-attacks/)
-    [Salvati, M. (2019, August 6). SILENTTRINITY Modules. Retrieved March 24, 2022.](https://github.com/byt3bl33d3r/SILENTTRINITY/tree/master/silenttrinity/core/teamserver/modules/boo)
-    [Baker, B., Unterbrink H. (2018, July 03). Smoking Guns - Smoke Loader learned new tricks. Retrieved July 5, 2018.](https://blog.talosintelligence.com/2018/07/smoking-guns-smoke-loader-learned-new.html#more)
-    [CrowdStrike. (2022, January 27). Early Bird Catches the Wormhole: Observations from the StellarParticle Campaign. Retrieved February 7, 2022.](https://www.crowdstrike.com/blog/observations-from-the-stellarparticle-campaign/)
-    [Marczak, B. and Scott-Railton, J.. (2016, May 29). Keep Calm and (Don’t) Enable Macros: A New Threat Actor Targets UAE Dissidents. Retrieved June 8, 2016.](https://citizenlab.org/2016/05/stealth-falcon/)
-    [Mandiant Israel Research Team. (2022, August 17). Suspected Iranian Actor Targeting Israeli Shipping, Healthcare, Government and Energy Sectors. Retrieved September 21, 2022.](https://www.mandiant.com/resources/blog/suspected-iranian-actor-targeting-israeli-shipping)
-    [Proofpoint Staff. (2017, September 27). Threat Actor Profile: TA505, From Dridex to GlobeImposter. Retrieved May 28, 2019.](https://www.proofpoint.com/us/threat-insight/post/threat-actor-profile-ta505-dridex-globeimposter)
-    [Park, S. (2024, June 27). Kimsuky deploys TRANSLATEXT to target South Korean academia. Retrieved October 14, 2024.](https://www.zscaler.com/blogs/security-research/kimsuky-deploys-translatext-target-south-korean-academia#technical-analysis)
-    [Anthony, N., Pascual, C.. (2018, November 1). Trickbot Shows Off New Trick: Password Grabber Module. Retrieved November 16, 2018.](https://blog.trendmicro.com/trendlabs-security-intelligence/trickbot-shows-off-new-trick-password-grabber-module/)
-    [Dahan, A. et al. (2019, December 11). DROPPING ANCHOR: FROM A TRICKBOT INFECTION TO THE DISCOVERY OF THE ANCHOR MALWARE. Retrieved September 10, 2020.](https://www.cybereason.com/blog/dropping-anchor-from-a-trickbot-infection-to-the-discovery-of-the-anchor-malware)
-    [Radu Tudorica. (2021, July 12). A Fresh Look at Trickbot’s Ever-Improving VNC Module. Retrieved September 28, 2021.](https://www.bitdefender.com/files/News/CaseStudies/study/399/Bitdefender-PR-Whitepaper-Trickbot-creat5515-en-EN.pdf)
-    [Secureworks. (2019, July 24). Updated Karagany Malware Targets Energy Sector. Retrieved August 12, 2020.](https://www.secureworks.com/research/updated-karagany-malware-targets-energy-sector)
-    [Tomonaga, S. (2018, March 6). Malware “TSCookie”. Retrieved May 6, 2020.](https://blogs.jpcert.or.jp/en/2018/03/malware-tscooki-7aa0.html)
-    [Settle, A., et al. (2016, August 8). MONSOON - Analysis Of An APT Campaign. Retrieved September 22, 2016.](https://www.forcepoint.com/sites/default/files/resources/files/forcepoint-security-labs-monsoon-analysis-report.pdf)
-    [Intrinsec. (2026, April). "Chaos is a ladder": Vidar's recent rise to the top. Retrieved April 27, 2026.](https://www.intrinsec.com/wp-content/uploads/2026/04/TLP_CLEAR-20260424-New_Vidar.pdf)
-    [Joshi, M. (2026, May 7). Unmasking a Multi-Stage Loader: AutoIt Abuse Leading to Vidar Stealer Command-and-Control Communication. Retrieved May 8, 2026.](https://www.levelblue.com/blogs/spiderlabs-blog/unmasking-a-multi-stage-loader-autoit-abuse-leading-to-vidar-stealer-command-and-control-communication)
-    [Nannaka, B., Chhaparwal, P. (2026, July 7). Vidar Stealer Unmasked: Code Signing Abuse, Go Loaders and File Inflation. Retrieved July 8, 2026.](https://unit42.paloaltonetworks.com/vidar-stealer-xmrig-miner-campaign-analysis/)
-    [CYFIRMA. (2024, June 3). Vidar Stealer: An In-depth Analysis of an Information-Stealing Malware. Retrieved August 13, 2026.](https://www.cyfirma.com/research/vidar-stealer-an-in-depth-analysis-of-an-information-stealing-malware/)
-    [Quorum Cyber. (n.d.). Malware Analysis Report Vidar - Stealerware. Retrieved August 28, 2026.](https://www.quorumcyber.com/wp-content/uploads/2023/01/Malware-Analysis-Vidar.pdf)
-    [CISA et al.. (2024, February 7). PRC State-Sponsored Actors Compromise and Maintain Persistent Access to U.S. Critical Infrastructure. Retrieved May 15, 2024.](https://www.cisa.gov/sites/default/files/2024-03/aa24-038a_csa_prc_state_sponsored_actors_compromise_us_critical_infrastructure_3.pdf)
-    [Harakhavik, Y. (2020, February 3). Warzone: Behind the enemy lines. Retrieved December 17, 2021.](https://research.checkpoint.com/2020/warzone-behind-the-enemy-lines/)
-    [Mohanta, A. (2020, November 25). Warzone RAT comes with UAC bypass technique. Retrieved April 7, 2022.](https://www.uptycs.com/blog/warzone-rat-comes-with-uac-bypass-technique)
-    [Robert Falcone. (2017, February 14). XAgentOSX: Sofacy's Xagent macOS Tool. Retrieved July 12, 2017.](https://researchcenter.paloaltonetworks.com/2017/02/unit42-xagentosx-sofacys-xagent-macos-tool/)
-    [Zscaler Threatlabz. (2025, January 27). Technical Analysis of Xloader Versions 6 and 7 | Part 1. Retrieved March 11, 2025.](https://www.zscaler.com/blogs/security-research/technical-analysis-xloader-versions-6-and-7-part-1)
-    [Nart Villeneuve, Randi Eitzman, Sandor Nemes & Tyler Dean, Google Cloud. (2017, October 5). Significant FormBook Distribution Campaigns Impacting the U.S. and South Korea. Retrieved March 11, 2025.](https://cloud.google.com/blog/topics/threat-intelligence/formbook-malware-distribution-campaigns/)
-    [Gustavo Palazolo, Netskope. (2022, March 11). New Formbook Campaign Delivered Through Phishing Emails. Retrieved March 11, 2025.](https://www.netskope.com/blog/new-formbook-campaign-delivered-through-phishing-emails)
-    [ESET Research. (2019, May 22). A journey to Zebrocy land. Retrieved June 20, 2019.](https://www.welivesecurity.com/2019/05/22/journey-zebrocy-land/)
-    [Singh, S. and Antil, S. (2020, October 27). APT-31 Leverages COVID-19 Vaccine Theme and Abuses Legitimate Online Services. Retrieved March 24, 2021.](https://www.zscaler.com/blogs/security-research/apt-31-leverages-covid-19-vaccine-theme-and-abuses-legitimate-online)

×
