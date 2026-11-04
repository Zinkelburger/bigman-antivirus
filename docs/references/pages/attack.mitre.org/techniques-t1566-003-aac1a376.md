---
url: "https://attack.mitre.org/techniques/T1566/003/"
title: "Phishing: Spearphishing via Service, Sub-technique T1566.003 - Enterprise | MITRE ATT&CK®"
captured: "2026-11-02T18:59:38Z"
via: "live"
sha256: "f7ab18deee478d9cd3c117b0bea9b7dd736921bd80fbbdc8b0399aee2636cd3c"
wayback: "https://web.archive.org/web/20261023090728/https://attack.mitre.org/techniques/T1566/003/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Phishing: Spearphishing via Service, Sub-technique T1566.003 - Enterprise | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/techniques/T1566/003/>
> Snapshot: <https://web.archive.org/web/20261023090728/https://attack.mitre.org/techniques/T1566/003/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Techniques](https://attack.mitre.org/techniques/enterprise)
- [Enterprise](https://attack.mitre.org/techniques/enterprise)
- [Phishing](https://attack.mitre.org/techniques/T1566)
- Spearphishing via Service

#  Phishing: Spearphishing via Service

##### Other sub-techniques of Phishing (4)

| ID  | Name

|  [T1566.001](https://attack.mitre.org/techniques/T1566/001/)   |  [Spearphishing Attachment](https://attack.mitre.org/techniques/T1566/001/)

|  [T1566.002](https://attack.mitre.org/techniques/T1566/002/)   |  [Spearphishing Link](https://attack.mitre.org/techniques/T1566/002/)

|  T1566.003   |  Spearphishing via Service

|  [T1566.004](https://attack.mitre.org/techniques/T1566/004/)   |  [Spearphishing Voice](https://attack.mitre.org/techniques/T1566/004/)

Adversaries may send spearphishing messages via third-party services in an attempt to gain access to victim systems. Spearphishing via service is a specific variant of spearphishing. It is different from other forms of spearphishing in that it employs the use of third party services rather than directly via enterprise email channels.

All forms of spearphishing are electronically delivered social engineering targeted at a specific individual, company, or industry. In this scenario, adversaries send messages through various social media services, personal webmail, and other non-enterprise controlled services.[[1]](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf) These services are more likely to have a less-strict security policy than an enterprise. As with most kinds of spearphishing, the goal is to generate rapport with the target or get the target's interest in some way. Adversaries will create fake social media accounts and message employees for potential job opportunities. Doing so allows a plausible reason for asking about services, policies, and software that's running in an environment. The adversary can then send malicious links or attachments through these services.

A common example is to build rapport with a target via social media, then send content to a personal webmail service that the target uses on their work computer. This allows an adversary to bypass some email restrictions on the work account, and the target is more likely to open the file since it's something they were expecting. If the payload doesn't work as expected, the adversary can continue normal communications and troubleshoot with the target on how to get it working.

ID: T1566.003

Sub-technique of:  [T1566](https://attack.mitre.org/techniques/T1566)

ⓘ

Tactic: [Initial Access](https://attack.mitre.org/tactics/TA0001)

ⓘ

Platforms: Linux, Windows, macOS

Version: 2.0

Created: 02 March 2020

Last Modified: 24 October 2025

[Version Permalink](https://attack.mitre.org/versions/v20/techniques/T1566/003/)

[Live Version](https://attack.mitre.org/versions/v20/techniques/T1566/003/)

## Procedure Examples

| ID  | Name  | Description

|  [G0130](https://attack.mitre.org/groups/G0130)   |  [Ajax Security Team](https://attack.mitre.org/groups/G0130)   |

[Ajax Security Team](https://attack.mitre.org/groups/G0130) has used various social media channels to spearphish victims.[[2]](https://www.mandiant.com/sites/default/files/2021-09/rpt-operation-saffron-rose.pdf)

|  [G0016](https://attack.mitre.org/groups/G0016)   |  [APT29](https://attack.mitre.org/groups/G0016)   |

[APT29](https://attack.mitre.org/groups/G0016) has used the legitimate mailing service Constant Contact to send phishing e-mails.[[3]](https://www.microsoft.com/security/blog/2021/05/27/new-sophisticated-email-based-attack-from-nobelium/)

|  [G0082](https://attack.mitre.org/groups/G0082)   |  [APT38](https://attack.mitre.org/groups/G0082)   |

[APT38](https://attack.mitre.org/groups/G0082) has used social media sites such as LinkedIn to send malicious links to potential vicitims.[[4]](https://www.microsoft.com/en-us/security/blog/2026/04/01/mitigating-the-axios-npm-supply-chain-compromise/)[[5]](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)

|  [G1052](https://attack.mitre.org/groups/G1052)   |  [Contagious Interview](https://attack.mitre.org/groups/G1052)   |

[Contagious Interview](https://attack.mitre.org/groups/G1052) has used fake job advertisements and messages sent via social media to spearphish targets.[[6]](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)[[7]](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)[[8]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)[[9]](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)[[10]](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)[[11]](https://securityscorecard.com/blog/the-job-offer-that-wasnt-how-we-stopped-an-espionage-plot/) [Contagious Interview](https://attack.mitre.org/groups/G1052) has also leveraged hiring websites to solicit victims.[[8]](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)

|  [G1012](https://attack.mitre.org/groups/G1012)   |  [CURIUM](https://attack.mitre.org/groups/G1012)   |

[CURIUM](https://attack.mitre.org/groups/G1012) has used social media to deliver malicious files to victims.[[12]](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)

|  [G0070](https://attack.mitre.org/groups/G0070)   |  [Dark Caracal](https://attack.mitre.org/groups/G0070)   |

[Dark Caracal](https://attack.mitre.org/groups/G0070) spearphished victims via Facebook and Whatsapp.[[1]](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)

|  [G1011](https://attack.mitre.org/groups/G1011)   |  [EXOTIC LILY](https://attack.mitre.org/groups/G1011)   |

[EXOTIC LILY](https://attack.mitre.org/groups/G1011) has used the e-mail notification features of legitimate file sharing services for spearphishing.[[13]](https://blog.google/threat-analysis-group/exposing-initial-access-broker-ties-conti/)

|  [G0037](https://attack.mitre.org/groups/G0037)   |  [FIN6](https://attack.mitre.org/groups/G0037)   |

[FIN6](https://attack.mitre.org/groups/G0037) has used fake job advertisements sent via LinkedIn to spearphish targets.[[14]](https://securityintelligence.com/posts/more_eggs-anyone-threat-actor-itg08-strikes-again/)

|  [G0032](https://attack.mitre.org/groups/G0032)   |  [Lazarus Group](https://attack.mitre.org/groups/G0032)   |

[Lazarus Group](https://attack.mitre.org/groups/G0032) has used social media platforms, including LinkedIn and Twitter, to send spearphishing messages.[[15]](https://blog.google/threat-analysis-group/new-campaign-targeting-security-researchers/)

|  [G0059](https://attack.mitre.org/groups/G0059)   |  [Magic Hound](https://attack.mitre.org/groups/G0059)   |

[Magic Hound](https://attack.mitre.org/groups/G0059) used various social media channels (such as LinkedIn) as well as messaging services (such as WhatsApp) to spearphish victims.[[16]](https://www.secureworks.com/research/the-curious-case-of-mia-ash)[[17]](https://blogs.microsoft.com/on-the-issues/2019/03/27/new-steps-to-protect-customers-from-hacking/)[[18]](https://www.clearskysec.com/wp-content/uploads/2020/08/The-Kittens-are-Back-in-Town-3.pdf)

|  [G1036](https://attack.mitre.org/groups/G1036)   |  [Moonstone Sleet](https://attack.mitre.org/groups/G1036)   |

[Moonstone Sleet](https://attack.mitre.org/groups/G1036) has used social media services to spear phish victims to deliver trojainized software.[[19]](https://www.microsoft.com/en-us/security/blog/2024/05/28/moonstone-sleet-emerges-as-new-north-korean-threat-actor-with-new-bag-of-tricks/)

|  [S1100](https://attack.mitre.org/software/S1100)   |  [Ninja](https://attack.mitre.org/software/S1100)   |

[Ninja](https://attack.mitre.org/software/S1100) has been distributed to victims via the messaging app Telegram.[[20]](https://securelist.com/toddycat/106799/)

|  [G0049](https://attack.mitre.org/groups/G0049)   |  [OilRig](https://attack.mitre.org/groups/G0049)   |

[OilRig](https://attack.mitre.org/groups/G0049) has used LinkedIn to send spearphishing links.[[21]](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)

|  [C0022](https://attack.mitre.org/campaigns/C0022)   |  [Operation Dream Job](https://attack.mitre.org/campaigns/C0022)   |

During [Operation Dream Job](https://attack.mitre.org/campaigns/C0022), [Lazarus Group](https://attack.mitre.org/groups/G0032) sent victims spearphishing messages via LinkedIn concerning fictitious jobs.[[22]](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)[[23]](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_Operation_Interception.pdf)

|  [G1046](https://attack.mitre.org/groups/G1046)   |  [Storm-1811](https://attack.mitre.org/groups/G1046)   |

[Storm-1811](https://attack.mitre.org/groups/G1046) has used Microsoft Teams to send messages and initiate voice calls to victims posing as IT support personnel.[[24]](https://www.microsoft.com/en-us/security/blog/2024/05/15/threat-actors-misusing-quick-assist-in-social-engineering-attacks-leading-to-ransomware/)

|  [G1022](https://attack.mitre.org/groups/G1022)   |  [ToddyCat](https://attack.mitre.org/groups/G1022)   |

[ToddyCat](https://attack.mitre.org/groups/G1022) has sent loaders configured to run [Ninja](https://attack.mitre.org/software/S1100) as zip archives via Telegram.[[20]](https://securelist.com/toddycat/106799/)

|  [G0112](https://attack.mitre.org/groups/G0112)   |  [Windshift](https://attack.mitre.org/groups/G0112)   |

[Windshift](https://attack.mitre.org/groups/G0112) has used fake personas on social media to engage and target victims.[[25]](https://www.scribd.com/document/661837258/WINDSHIFT-summit-archive-1554718868)

## Mitigations

| ID  | Mitigation  | Description

|  [M1049](https://attack.mitre.org/mitigations/M1049)   |  [Antivirus/Antimalware](https://attack.mitre.org/mitigations/M1049)   |

Anti-virus can also automatically quarantine suspicious files.

|  [M1047](https://attack.mitre.org/mitigations/M1047)   |  [Audit](https://attack.mitre.org/mitigations/M1047)   |

Implement auditing and logging for interactions with third-party messaging services or collaboration platforms. Monitor user activity and review logs for signs of suspicious links, downloads, or file exchanges that could indicate spearphishing attempts. Effective auditing allows for the quick identification of malicious activity originating from compromised service accounts.

|  [M1021](https://attack.mitre.org/mitigations/M1021)   |  [Restrict Web-Based Content](https://attack.mitre.org/mitigations/M1021)   |

Determine if certain social media sites, personal webmail services, or other service that can be used for spearphishing is necessary for business operations and consider blocking access if activity cannot be monitored well or if it poses a significant risk.

|  [M1018](https://attack.mitre.org/mitigations/M1018)   |  [User Account Management](https://attack.mitre.org/mitigations/M1018)   |

Enforce strict user account management policies on third-party service accounts to control access and limit privileges. Configure accounts with the minimum permissions necessary to perform their roles and regularly review access levels. This minimizes the risk of adversaries exploiting service accounts to execute spearphishing attacks or gain unauthorized access to sensitive resources.

|  [M1017](https://attack.mitre.org/mitigations/M1017)   |  [User Training](https://attack.mitre.org/mitigations/M1017)   |

Users can be trained to identify social engineering techniques and spearphishing messages with malicious links.

## Detection Strategy

| ID  | Name  | Analytic ID  | Analytic Description

|  [DET0115](https://attack.mitre.org/detectionstrategies/DET0115)   |  [Detection Strategy for Spearphishing via a Service across OS Platforms](https://attack.mitre.org/detectionstrategies/DET0115)   |  [AN0320](https://attack.mitre.org/detectionstrategies/DET0115#AN0320)   |

Inbound spearphishing attempts delivered via third-party services (e.g., Gmail, LinkedIn messages) leading to malicious file downloads or browser-initiated script execution. Defender view includes correlation of external service logins, unexpected file write operations, and suspicious descendant processes spawned from productivity or browser applications.

|  [AN0321](https://attack.mitre.org/detectionstrategies/DET0115#AN0321)   |

Use of non-enterprise email or messaging services in Thunderbird, Evolution, or browsers leading to suspicious file downloads and subsequent execution. Defender view includes browser-initiated downloads of unexpected content and shell or interpreter processes launched post-download.

|  [AN0322](https://attack.mitre.org/detectionstrategies/DET0115#AN0322)   |

Phishing attempts via iCloud Mail, Gmail, or social media apps accessed on macOS systems. Defender view includes Mail.app or Safari downloads of files followed by osascript, Terminal, or abnormal child process execution.

## References

-    [Blaich, A., et al. (2018, January 18). Dark Caracal: Cyber-espionage at a Global Scale. Retrieved April 11, 2018.](https://info.lookout.com/rs/051-ESQ-475/images/Lookout_Dark-Caracal_srr_20180118_us_v.1.0.pdf)
-    [Villeneuve, N. et al.. (2013). OPERATION SAFFRON ROSE . Retrieved May 28, 2020.](https://www.mandiant.com/sites/default/files/2021-09/rpt-operation-saffron-rose.pdf)
-    [Microsoft Threat Intelligence Center (MSTIC). (2021, May 27). New sophisticated email-based attack from NOBELIUM. Retrieved May 28, 2021.](https://www.microsoft.com/security/blog/2021/05/27/new-sophisticated-email-based-attack-from-nobelium/)
-    [Microsoft Threat Intelligence. (2026, April 1). Mitigating the Axios npm supply chain compromise. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/04/01/mitigating-the-axios-npm-supply-chain-compromise/)
-    [Microsoft Threat Intelligence. (2026, June 17). From package to postinstall payload: Inside the Mastra npm supply chain compromise by Sapphire Sleet. Retrieved September 30, 2026.](https://www.microsoft.com/en-us/security/blog/2026/06/17/postinstall-payload-inside-mastra-npm-supply-chain-compromise/)
-    [Amaury G., Coline Chavane, Felix Aimé and Sekoia TDR. (2025, March 31). From Contagious to ClickFake Interview: Lazarus leveraging the ClickFix tactic. Retrieved April 1, 2025.](https://blog.sekoia.io/clickfake-interview-campaign-by-lazarus/)
-    [Efstratios Lontzetidis. (2025, January 16). Lazarus APT: Techniques for Hunting Contagious Interview. Retrieved October 20, 2025.](https://www.validin.com/blog/inoculating_contagious_interview_with_validin/)
-    [Insikt Group. (2025, February 13). Inside the Scam: North Korea’s IT Worker Threat. Retrieved October 17, 2025.](https://www.recordedfuture.com/research/inside-the-scam-north-koreas-it-worker-threat)
-    [Matej Havranek. (2025, February 20). DeceptiveDevelopment targets freelance developers. Retrieved October 17, 2025.](https://www.welivesecurity.com/en/eset-research/deceptivedevelopment-targets-freelance-developers/)
-    [Ryan Sherstobitoff. (2024, October 29). Inside a North Korean Phishing Operation Targeting DevOps Employees. Retrieved October 20, 2025.](https://securityscorecard.com/blog/inside-a-north-korean-phishing-operation-targeting-devops-employees/)
-    [Steve Cobb. (2024, October 29). The Job Offer That Wasn’t: How We Stopped an Espionage Plot. Retrieved October 20, 2025.](https://securityscorecard.com/blog/the-job-offer-that-wasnt-how-we-stopped-an-espionage-plot/)
-    [MSTIC. (2021, November 16). Evolving trends in Iranian threat actor activity – MSTIC presentation at CyberWarCon 2021. Retrieved January 12, 2023.](https://www.microsoft.com/en-us/security/blog/2021/11/16/evolving-trends-in-iranian-threat-actor-activity-mstic-presentation-at-cyberwarcon-2021)
-    [Stolyarov, V. (2022, March 17). Exposing initial access broker with ties to Conti. Retrieved August 18, 2022.](https://blog.google/threat-analysis-group/exposing-initial-access-broker-ties-conti/)

-    [Villadsen, O.. (2019, August 29). More_eggs, Anyone? Threat Actor ITG08 Strikes Again. Retrieved September 16, 2019.](https://securityintelligence.com/posts/more_eggs-anyone-threat-actor-itg08-strikes-again/)
-    [Weidemann, A. (2021, January 25). New campaign targeting security researchers. Retrieved December 20, 2021.](https://blog.google/threat-analysis-group/new-campaign-targeting-security-researchers/)
-    [Counter Threat Unit Research Team. (2017, July 27). The Curious Case of Mia Ash: Fake Persona Lures Middle Eastern Targets. Retrieved February 26, 2018.](https://www.secureworks.com/research/the-curious-case-of-mia-ash)
-    [Burt, T. (2019, March 27). New steps to protect customers from hacking. Retrieved May 27, 2020.](https://blogs.microsoft.com/on-the-issues/2019/03/27/new-steps-to-protect-customers-from-hacking/)
-    [ClearSky Research Team. (2020, August 1). The Kittens Are Back in Town 3 - Charming Kitten Campaign Evolved and Deploying Spear-Phishing link by WhatsApp. Retrieved April 21, 2021.](https://www.clearskysec.com/wp-content/uploads/2020/08/The-Kittens-are-Back-in-Town-3.pdf)
-    [Microsoft Threat Intelligence. (2024, May 28). Moonstone Sleet emerges as new North Korean threat actor with new bag of tricks. Retrieved August 26, 2024.](https://www.microsoft.com/en-us/security/blog/2024/05/28/moonstone-sleet-emerges-as-new-north-korean-threat-actor-with-new-bag-of-tricks/)
-    [Dedola, G. (2022, June 21). APT ToddyCat. Retrieved January 3, 2024.](https://securelist.com/toddycat/106799/)
-    [Bromiley, M., et al.. (2019, July 18). Hard Pass: Declining APT34’s Invite to Join Their Professional Network. Retrieved August 26, 2019.](https://www.fireeye.com/blog/threat-research/2019/07/hard-pass-declining-apt34-invite-to-join-their-professional-network.html)
-    [ClearSky Research Team. (2020, August 13). Operation 'Dream Job' Widespread North Korean Espionage Campaign. Retrieved December 20, 2021.](https://www.clearskysec.com/wp-content/uploads/2020/08/Dream-Job-Campaign.pdf)
-    [Breitenbacher, D and Osis, K. (2020, June 17). OPERATION IN(TER)CEPTION: Targeted Attacks Against European Aerospace and Military Companies. Retrieved December 20, 2021.](https://www.welivesecurity.com/wp-content/uploads/2020/06/ESET_Operation_Interception.pdf)
-    [Microsoft Threat Intelligence. (2024, May 15). Threat actors misusing Quick Assist in social engineering attacks leading to ransomware. Retrieved March 14, 2025.](https://www.microsoft.com/en-us/security/blog/2024/05/15/threat-actors-misusing-quick-assist-in-social-engineering-attacks-leading-to-ransomware/)
-    [Karim, T. (2018, August). TRAILS OF WINDSHIFT. Retrieved November 17, 2024.](https://www.scribd.com/document/661837258/WINDSHIFT-summit-archive-1554718868)

×
