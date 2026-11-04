---
url: "https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices"
title: "PumaBot: Novel Botnet Targeting IoT Surveillance Devices"
captured: "2026-11-02T18:52:35Z"
via: "live"
sha256: "cd4c68ff8f8797122bdc3c89f32417b8a780923560ca49ba5d43763d3f34eb7b"
wayback: "https://web.archive.org/web/20261001050445/https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# PumaBot: Novel Botnet Targeting IoT Surveillance Devices

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices>
> Snapshot: <https://web.archive.org/web/20261001050445/https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices>

[Blog](https://www.darktrace.com/blog)

/

[Network](https://www.darktrace.com/primary-topics/network)

/

May 28, 2025

# PumaBot: Novel Botnet Targeting IoT Surveillance Devices

Darktrace investigated “PumaBot,” a Go-based Linux botnet targeting IoT devices. It avoids internet-wide scanning, instead using a C2 server to get targets and brute-force SSH credentials. Once inside, it executes remote commands and ensures persistence.

[Written by

Tara Gould

Malware Research Lead](https://www.darktrace.com/people/tara-gould)

Inside the SOC

Darktrace cyber analysts are world-class experts in threat intelligence, threat hunting and incident response, and provide 24/7 SOC support to thousands of Darktrace customers around the globe. Inside the SOC is exclusively authored by these experts, providing analysis of cyber incidents and threat trends, based on real-world experience in the field.

Written by

Tara Gould

Malware Research Lead

Share

https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices#https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices#https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices#

28

May 2025

## Introduction: PumaBot attacking IoT devices

Darktrace researchers have identified a custom Go-based Linux botnet named “PumaBot” targeting embedded Linux Internet of Things (IoT) devices. Rather than scanning the Internet, the malware retrieves a list of targets from a command-and-control (C2) server and attempts to brute-force SSH credentials. Upon gaining access, it receives remote commands and establishes persistence using system service files. This blog post provides a breakdown of its key functionalities, and explores binaries related to the campaign.

## Technical Analysis

Filename: jierui

md5: cab6f908f4dedcdaedcdd07fdc0a8e38

The Go-based botnet gains initial access through brute-forcing SSH credentials across a list of harvested IP addresses. Once it identifies a valid credential pair, it logs in, deploys itself, and begins its replication process.

Figure 1: Overview of Jierui functions.

The domain associated with the C2 server did not resolve to an IP address at the time of analysis. The following details are a result of static analysis of the malware.

The malware begins by retrieving a list of IP addresses of likely devices with open SSH ports from the C2 server (ssh.ddos-cc[.]org) via the getIPs() function. It then performs brute-force login attempts on port 22 using credential pairs also obtained from the C2 through the readLinesFromURL(), brute(), and trySSHLogin() functions.

Within trySSHLogin(), the malware performs several environment fingerprinting checks. These are used to avoid honeypots and unsuitable execution environments, such as restricted shells. Notably, the malware checks for the presence of the string “Pumatronix”, a manufacturer of surveillance and traffic camera systems, suggesting potential IoT targeting or an effort to evade specific devices [1].

Figure 2: Fingerprinting of “Pumatronix”.

If the environment passes these checks, the malware executes uname -a to collect basic system information, including the OS name, kernel version, and architecture. This data, along with the victim's IP address, port, username, and password, is then reported back to the C2 in a JSON payload.

Of note, the bot uses X-API-KEY: jieruidashabi, within a custom header when it communicates with the C2 server over HTTP.

The malware writes itself to /lib/redis, attempting to disguise itself as a legitimate Redis system file. It then creates a persistent systemd service in /etc/systemd/system, named either redis.service or mysqI.service (note the spelling of mysql with a capital I) depending on what has been hardcoded into the malware. This allows the malware to persist across reboots while appearing benign.

[Unit]
Description=redis Server Service

[Service]
Type=simple
Restart=always
RestartSec=1
User=root
ExecStart=/lib/redis e

[Install]
WantedBy=multi-user.target

In addition to gaining persistence with a systemd service, the malware also adds its own SSH keys into the users’ authorized_keys file. This ensures that access can be maintained, even if the service is removed.

A function named cleankill() contains an infinite loop that repeatedly attempts to execute the commands “xmrig” and “networkxm”. These are launched without full paths, relying on the system's PATH variable suggesting that the binaries may be downloaded or unpacked elsewhere on the system. The use of “time.Sleep” between attempts indicates this loop is designed to ensure persistence and possibly restart mining components if they are killed or missing.

During analysis of the botnet, Darktrace discovered related binaries that appear to be part of a wider campaign targeting Linux systems.

Filename: ddaemon
Md5: 48ee40c40fa320d5d5f8fc0359aa96f3

Ddaemon is a Go-based backdoor. The malware begins by parsing command line arguments and if conditions are met, enters a loop where it periodically verifies the MD5 hash of the binary. If the check fails or an update is available, it downloads a new version from a C2 server (db.17kp[.]xyz/getDdaemonMd5), verifies it and replaces the existing binary with a file of the same name and similar functionality (8b37d3a479d1921580981f325f13780c).

The malware uses main_downloadNetwork() to retrieve the binary “networkxm” into /usr/src/bao/networkxm. Additionally, the bash script “installx.sh” is also retrieved from the C2 and executed. The binary ensures persistence by writing a custom systemd service unit that auto starts on boot and executes ddaemon.

Filename: networkxm
Md5: be83729e943d8d0a35665f55358bdf88

The networkxm binary functions as an SSH brute-force tool, similar to the botnet. First it checks its own integrity using MD5 hashes and contacts the C2 server (db.17kp[.]xyz) to compare its hash with the latest version. If an update is found, it downloads and replaces itself.

Figure 3: Part of networkxm checking MD5 hash.

Figure 4: MD5 hash

After verifying its validity, it enters an infinite loop where it fetches a password list from the C2 (/getPassword), then attempts SSH connections across a list of target IPs from the /getIP endpoint. As with the other observed binaries, a systemd service is created if it doesn’t already exist for persistence in /etc/systemd/system/networkxm.service.

Figure 5: Bash script installx.sh.

Installx.sh is a simple bash script used to retrieve the script “jc.sh” from 1.lusyn[.]xyz, set permissions, execute and clear bash history.

Figure 6: Snippet of bash script jc.sh.

The script jc.sh starts by detecting the operating system type Debian-based or Red Hat-based and determines the location of the pam_unix.so file. Linux Pluggable Authentication Modules (PAM) is a framework that allows for flexible and centralized user authentication on Linux systems. PAM allows system administrators to configure how users are authenticated for services like login, SSH, or sudo by plugging in various authentication modules.

Jc.sh then attempts to fetch the current version of PAM installed on the system and formats that version to construct a URL. Using either curl or wget, the script downloads a replacement pam_unix.so file from a remote server and replaces the existing one, after disabling file immutability and backing up the original.

The script also downloads and executes an additional binary named “1” from the same remote server. Security settings are modified including enabling PAM in the SSH configuration and disabling SELinux enforcement, before restarting the SSH service. Finally, the script removes itself from the system.

Filename: Pam_unix.so_v131
md5: 1bd6bcd480463b6137179bc703f49545

Based on the PAM version that is retrieved from the bash query, the new malicious PAM replaces the existing PAM file. In this instance, pam_unix.so_v131 was retrieved from the server based on version 1.3.1. The purpose of this binary is to act as a rootkit that steals credentials by intercepting successful logins. Login data can include all accounts authenticated by PAM, local and remote (SSH). The malware retrieves the logged in user, the password and verifies that the password is valid. The details are stored in a file “con.txt” in /usr/bin/.

Figure 7: Function storing logins to con.txt

Filename: 1

md5: cb4011921894195bcffcdf4edce97135

In addition to the malicious PAM file, a binary named “1” is also retrieved from the server http://dasfsdfsdfsdfasfgbczxxc[.]lusyn[.]xyz/jc/1. The binary “1” is used as a watcher for the malicious PAM file using inotify to monitor for “con.txt” being written or moved to /usr/bin/.

Following the daemonize() function, the binary is run daemonized ensuring it runs silently in the background. The function read_and_send_files() is called which reads the contents of “/usr/bin/con.txt”, queries the system IP with ifconfig.me, queries SSH ports and sends the data to the remote C2 (http://dasfsdfsdfsdfasfgbczxxc[.]lusyn[.]xyz/api/).

Figure 8: Command querying SSH ports.

For persistence, a systemd service (my_daemon.service) is created to autostart the binary and ensure it restarts if the service has been terminated. Finally, con.txt is deleted, presumably to remove traces of the malware.

## Conclusion

The botnet represents a persistent Go-based SSH threat that leverages automation, credential brute-forcing, and native Linux tools to gain and maintain control over compromised systems. By mimicking legitimate binaries (e.g., Redis), abusing systemd for persistence, and embedding fingerprinting logic to avoid detection in honeypots or restricted environments, it demonstrates an intent to evade defenses.

While it does not appear to propagate automatically like a traditional worm, it does maintain worm-like behavior by brute-forcing targets, suggesting a semi-automated botnet campaign focused on device compromise and long-term access.

[related-resource]

## Recommendations

- Monitor for anomalous SSH login activity, especially failed login attempts across a wide IP range, which may indicate brute-force attempts.
- Audit systemd services regularly. Look for suspicious entries in /etc/systemd/system/ (e.g., misspelled or duplicate services like mysqI.service) and binaries placed in non-standard locations such as /lib/redis.
- Inspect authorized_keys files across user accounts for unknown SSH keys that may enable unauthorized access.
- Filter or alert on outbound HTTP requests with non-standard headers, such as X-API-KEY: jieruidashabi, which may indicate botnet C2 communication.
- Apply strict firewall rules to limit SSH exposure rather than exposing port 22 to the internet.

## Appendices

### References

1. https://pumatronix.com/

### Indicators of Compromise (IoCs)

#### Hashes

cab6f908f4dedcdaedcdd07fdc0a8e38 - jierui

a9412371dc9247aa50ab3a9425b3e8ba - bao

0e455e06315b9184d2e64dd220491f7e - networkxm

cb4011921894195bcffcdf4edce97135 - 1
48ee40c40fa320d5d5f8fc0359aa96f3 - ddaemon
1bd6bcd480463b6137179bc703f49545 - pam_unix.so_v131

#### RSA Key

ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC0tH30Li6Gduh0Jq5A5dO5rkWTsQlFttoWzPFnGnuGmuF+fwIfYvQN1z+WymKQmX0ogZdy/CEkki3swrkq29K/xsyQQclNm8+xgI8BJdEgTVDHqcvDyJv5D97cU7Bg1OL5ZsGLBwPjTo9huPE8TAkxCwOGBvWIKUE3SLZW3ap4ciR9m4ueQc7EmijPHy5qds/Fls+XN8uZWuz1e7mzTs0Pv1x2CtjWMR/NF7lQhdi4ek4ZAzj9t/2aRvLuNFlH+BQx+1kw+xzf2q74oBlGEoWVZP55bBicQ8tbBKSN03CZ/QF+JU81Ifb9hy2irBxZOkyLN20oSmWaMJIpBIsh4Pe9 @root

#### Network

http://ssh[.]ddos-cc.org:55554

http://ssh[.]ddos-cc.org:55554/log_success

http://ssh[.]ddos-cc.org:55554/get_cmd

http://ssh[.]ddos-cc.org:55554/pwd.txt

https://dow[.]17kp.xyz/

https://input[.]17kp.xyz/

https://db[.]17kp[.]xyz/

http://1[.]lusyn[.]xyz

http://1[.]lusyn[.]xyz/jc/1

http://1[.]lusyn[.]xyz/jc/jc.sh

http://1[.]lusyn[.]xyz/jc/aa

http://1[.]lusyn[.]xyz/jc/cs

http://dasfsdfsdfsdfasfgbczxxc[.]lusyn[.]xyz/api

http://dasfsdfsdfsdfasfgbczxxc[.]lusyn[.]xyz/jc

### Detection Rule

rule Linux_PumaBot

{

meta:

description = "Rule to match on PumaBot samples"

author = "[[email protected]](https://www.darktrace.com/cdn-cgi/l/email-protection)"

strings:

$xapikey = "X-API-KEY" ascii

$get_ips = "?count=5000" ascii

‍

$exec_start = "ExecStart=/lib/redis" ascii

$svc_name1 = "redis.service" ascii

$svc_name2 = "mysqI.service" ascii

$uname = "uname -a" ascii

$pumatronix = "Pumatronix" ascii

condition:

uint32(0) == 0x464c457f and

all of (

$xapikey,

$uname,

$get_ips,

$exec_start

) and any of (

$svc_name1,

$svc_name2

) and $pumatronix

}

### Get the latest insights on emerging cyber threats

This report explores the latest trends shaping the cybersecurity landscape and what defenders need to know in 2025

[Download now](https://www.darktrace.com/resources/annual-threat-report-2024)

[Written by

Tara Gould

Malware Research Lead](https://www.darktrace.com/people/tara-gould)

Inside the SOC

Darktrace cyber analysts are world-class experts in threat intelligence, threat hunting and incident response, and provide 24/7 SOC support to thousands of Darktrace customers around the globe. Inside the SOC is exclusively authored by these experts, providing analysis of cyber incidents and threat trends, based on real-world experience in the field.

Written by

Tara Gould

Malware Research Lead

Share this post

https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices#https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices#https://www.darktrace.com/blog/pumabot-novel-botnet-targeting-iot-surveillance-devices#

Latest blogs

[### Beyond the Fridge: Exposed Industrial Control Systems

OT

•

October 21, 2026

Tara Gould

Malware Research Lead](https://www.darktrace.com/blog/beyond-the-fridge-exposed-industrial-control-systems)

[### PHP Vulnerabilities, ARTEX and the Growing Role of AI in Cyber Attacks

AI

•

October 22, 2026

Nathaniel Jones

SVP, Global Threat Intelligence](https://www.darktrace.com/blog/vulnerability-discovery-to-active-attacks-ai-in-cyber-operations)

[Watch the NIS2 Webinar](https://darktrace.com/resources/nis2-directive-implications-on-cyber-security-and-ai)

Newsletter

# Enjoying the blog?

Sign up to receive the latest news and insights from the Darktrace newsletter – delivered directly to your inbox

Thanks for signing up!

Look out for your first newsletter, coming soon.

Oops! Something went wrong while submitting the form.

Trending blogs

[1

Prompt Security in Enterprise AI: Strengths, Weaknesses, and Common Approaches

May 20, 2026](https://www.darktrace.com/blog/prompt-security-in-enterprise-ai-strengths-weaknesses-and-common-approaches)

[2

What the Darktrace Annual Threat Report 2026 Means for Security Leaders

Feb 26, 2026](https://www.darktrace.com/blog/what-the-darktrace-annual-threat-report-2026-means-for-security-leaders)

[3

Journey of a Threat: How Multi-Layered AI Works in Darktrace / EMAIL

May 26, 2026](https://www.darktrace.com/blog/how-multi-layered-ai-works-in-darktrace-email)

[4

How to Evaluate AI Vendors: 5 Key categories for AI Adoption

May 27, 2026](https://www.darktrace.com/blog/how-to-evaluate-ai-vendors-5-key-categories-for-ai-adoption)

[5

Darktrace Unites Human Behavior and Threat Detection Across Email, Slack, Teams, and Zoom

Mar 24, 2026](https://www.darktrace.com/blog/darktrace-unites-human-behavior-and-threat-detection-across-email-slack-teams-and-zoom)

## More in this series

No items found.

Continue reading

[Network](https://www.darktrace.com/primary-topics/network)

•

October 20, 2026

### Why Kubernetes Security Needs A Behavioral Approach

As Kubernetes adoption accelerates, especially for AI workloads, Darktrace extends behavioral security across runtime activity, delivering visibility, detection, response, forensics, and risk context to identify and contain threats.

Elise Longhurst

Senior Product Executive

[Read more](https://www.darktrace.com/blog/why-kubernetes-security-needs-a-behavioral-approach)

[Network](https://www.darktrace.com/primary-topics/network)

•

October 14, 2026

### CikaBika Bot: Inside a Discord RAT

Darktrace researchers investigate CikaBika Bot, a Rust-based remote access trojan delivered through a document-themed lure. The malware uses a Nim loader, PowerShell UAC bypass and DLL search-order hijacking before leveraging Discord for C2, reconnaissance, remote execution, and data exfiltration through ngrok in observed malicious activity.

Tara Gould

Malware Research Lead

[Read more](https://www.darktrace.com/blog/cikabika-bot-inside-a-discord-rat)

[Network](https://www.darktrace.com/primary-topics/network)

•

September 24, 2026

### A Chain Reaction: Blockchain-Hosted Infostealer Campaign Targets Windows and macOS

Darktrace detected a blockchain-hosted infostealer campaign targeting Windows and macOS devices, combining ClickFix social engineering with the abuse of legitimate services such as GitHub, blockchain infrastructure, and DGA C2 endpoints to evade disruption and support malware delivery.

Nahisha Nobregas

Associate Principal Cyber Analyst & Team Lead

[Read more](https://www.darktrace.com/blog/a-chain-reaction-blockchain-hosted-infostealer-campaign-targets-windows-and-macos)

## Blog

/

## AI

/

## November 2, 2026

## PHP Vulnerabilities, ARTEX and the Growing Role of AI in Cyber Attacks

The disclosure of CVE-2026-109618 and CVE-2026-109619 placed immediate pressure on organizations running PHP. Both vulnerabilities can allow unauthenticated remote code execution, including in products and appliances where PHP is embedded, and updates depend on third-party vendors [1].

Early guidance focused on patching internet-facing systems and monitoring for behavior associated with successful exploitation, including unexpected command execution, Webroot changes and outbound connections from PHP servers.

Darktrace observed widespread scanning across monitored environments involving hundreds of distinct source IP addresses that public reporting has associated with activity targeting the PHP vulnerabilities. The volume of scanning was significantly greater than the evidence available for successful exploitation. For analysts, that distinction is important. Scanning demonstrates interest in exposed infrastructure; establishing compromise requires evidence of subsequent behavior on the targeted system, including command execution, file modification or unexpected outbound communication.

PHP is not an isolated example. Imperva subsequently demonstrated remote code execution against a vulnerable Node.js build through malicious Brotli data, reproducing execution through compressed HTTP responses and configured request parsers. The same inputs were rejected by the patched runtime. Imperva linked its Node2Shell and PHP2Shell research to the increasing speed of vulnerability discovery and exploit development. [2]

While Darktrace has not observed direct evidence of CVE-2026-109618 or CVE-2026-109619 exploitation, on October 21st we observed a six-month high in detections consistent with zero-day exploitation activity, representing a 64% increase compared to historic data. This increase has not been confirmed as related to the PHP vulnerabilities. Darktrace’s behavioral approach is designed to detect anomalous activity without relying solely on rules written for a specific exploit. Comparable increases were observed around the Log4j disclosure and other globally significant vulnerability disclosures.

### AI-assisted vulnerability research

The PHP disclosure arrives as AI systems have become more capable at vulnerability research [3]. OpenAI launched Daybreak in June 2026, combining GPT-5.5-Cyber, Codex Security and partnerships with security researchers and software maintainers. OpenAI says its models have been applied to discovering and generating patches for vulnerabilities in browsers, network infrastructure, FreeBSD and the Linux kernel. Darktrace joined the OpenAI Daybreak Cyber Partner Program in June to explore how OpenAI's cyber capabilities could be integrated within Darktrace products and services [4][10].

OpenAI reported in August that Codex Security Cloud had analyzed more than 30 million commits across more than 30,000 codebases. According to OpenAI, Cloudflare uses models accessed through Daybreak to investigate customer-authorized code and combines findings with live traffic and security signals to assess whether affected routes are active or receiving suspicious traffic, with proposed mitigations and patches remaining subject to customer review [5][6].

Faster discovery still leaves defenders with the work of locating affected software, testing fixes and completing remediation. The PHP case shows that gap clearly, particularly when vulnerable runtimes are embedded inside other products.

### Enterprise agents as an attack surface

Darktrace Signal Labs has also examined how a trusted enterprise agent could be abused after an attacker has already obtained access to an environment. In a controlled simulation, researchers demonstrated that, in that post-compromise scenario, the Claude platform’s memory stores feature could be abused to establish a live C2 channel between an external operator and internal Claude Code agents. The external operator was able to dynamically assign network discovery, credential access, and data exfiltration tasks to the agents via this C2 channel. Detection of the agents’ activities relied on the correlation of process creation and network traffic activity from agent harness processes, alongside behavioral analysis to identify deviations from expected behavior. Darktrace has not claimed it observed this memory-store command-and-control technique in operational attacks [7].

### AI-assisted attacks in Japan

Evidence from Japan indicates AI being used for vulnerability scanning and attacks outside controlled research. On 21 October, JPCERT/CC published findings from an investigation into recent unauthorized access incidents affecting organizations in Japan. JPCERT/CC states that it confirmed attackers had used AI to scan for known vulnerabilities and conduct attacks through APIs. According to JPCERT/CC, its examination of attack infrastructure found evidence of Android application analysis, API discovery and testing for weaknesses including server-side request forgery, and the infrastructure also targeted services intended for users outside Japan [8].

JPCERT/CC does not identify ARTEX by name in that publication. The named ARTEX attribution comes from separate investigations, including Microsoft's analysis of Storm-0940.

### AI in offensive operations

Microsoft describes ARTEX as an open-source, multi-agent penetration-testing system capable of automating activity with limited human interaction. Microsoft reported that Storm-0940, which it assesses to be a China state-aligned actor active since 2021, used ARTEX against hundreds of organizations, and Microsoft attributed the ConnectWise supply-chain compromise [9].

Microsoft observed ARTEX accelerating scanning, reconnaissance, password spraying and OAuth application abuse. Several ARTEX-driven operations reached privileged cloud access less than three hours after initial targeting. Following the ConnectWise compromise, Storm-0940 attempted to authenticate to more than 2,250 tenants at rates of up to 2,900 attempts per minute [9].

Darktrace Threat Research found activity in its own telemetry involving infrastructure later associated with the campaign. On 13 October, Darktrace recorded failed authentication activity involving 144.172.64[.]128, one of the addresses Microsoft later identified as performing sign-ins and Microsoft Graph activity in ConnectWise customer environments. Darktrace analysis identified 65 failed authentication attempts in around 20 minutes.

Figure 1: Darktrace Advanced Search showing failed authentication events involving 144.172.64[.]128 on 13 October 2026, (customer-identifying information redacted).

The telemetry establishes repeated failed authentication from infrastructure later associated with the campaign. It does not establish ARTEX execution or successful compromise. At the time of observation, Microsoft's later attribution was unavailable. Analysts therefore had the behavior itself: repeated failed authentication from the same source over a short period, together with any related identity, application or network activity inside the environment.

Microsoft documented a wider password-spray campaign against 327 accounts across eight organizations between 10 and 14 October. It recorded more than 151,000 failed logons, including more than 111,000 on 11 October, but no successful authentication, token issuance, Graph call or directory change from that campaign. Microsoft also reported that most Storm-0940 targeting was unsuccessful [9].

Where the actor progressed beyond authentication, Microsoft observed service-principal sign-ins followed by Graph and directory operations. Activity included reconnaissance, application credential changes and attempts to modify privileges. Microsoft recommends correlating service-principal sign-ins with Graph activity, directory changes and resource access [9].

### Behavior before attribution

The PHP and ARTEX cases begin with different information. PHP defenders knew which software required urgent remediation but initially had limited information about exploitation. Process execution, file modification and outbound communication could still provide evidence of compromise.

The PHP and ARTEX observations also illustrate the difference between volume and impact. Darktrace observed broad scanning around the PHP disclosure, while the ARTEX-associated infrastructure produced concentrated authentication activity. Neither observation independently demonstrates compromise and in each case examining the behavior that follows the initial interaction is the investigative clue.

The Darktrace ARTEX-related observation began with authentication behavior. Microsoft publicly attributed the source infrastructure eight days later. That attribution added context, but it did not change what had originally been recorded in the environment.

AI is now present at several points in this cycle. Daybreak applies models to vulnerability discovery and remediation. Darktrace Signal Labs demonstrated how a trusted enterprise agent could be co-opted in a controlled environment. JPCERT/CC confirmed AI-assisted vulnerability scanning and API attacks in recent incidents. Microsoft documented ARTEX being used by Storm-0940 in active operations [4][7][8][9].

Microsoft's findings also show why scale and impact must be assessed separately. One ARTEX-associated campaign produced more than 151,000 failed logons without successful access, while other operations reached privileged cloud access in less than three hours. Security teams have to distinguish high-volume attempts from activity that progresses through authentication, privilege changes and resource access.

The evidence required to assess these cases remains familiar: authentication records, process execution, application and directory changes, file activity and network communication. AI can increase the rate at which those behaviors occur. Defenders still need to establish their sequence and impact.

Credit to Emma Foulger, Sam Lister, Dylan Evans, Moishe Tajtelbaum

### References

[1] NCSC-NL, php2shell vulnerabilities (CVE-2026-109618, CVE-2026-109619), 21 October 2026. https://github.com/NCSC-NL/php2shell

[2] Imperva, https://www.imperva.com/blog/node2shell-from-brotli-data-to-remote-code-execution/

[3] Cyber Security Agency of Singapore, https://www.csa.gov.sg/alerts-and-advisories/alerts/al-2026-151/

[4] OpenAI, Daybreak: Tools for securing every organization in the world, 22 June 2026. https://openai.com/index/daybreak-securing-the-world/

[5] OpenAI Developers, Scaling cyber defenders with Daybreak, 21 August 2026. https://developers.openai.com/blog/scaling-cyber-defenders-with-daybreak

[6] OpenAI, Cloudflare links vulnerability discovery to defense with OpenAI, 21 October 2026. https://openai.com/index/cloudflare-cyber/

[7] Darktrace Signal Labs, Shared Memories as a Covert C2 Channel, draft research supplied for review.

[8] JPCERT/CC, Recent AI-Assisted Cyber Attacks in Japan: Investigating Attack Infrastructure and Techniques, 21 October 2026. https://blogs.jpcert.or.jp/en/2026/10/web_api_attacks.html

[9] Microsoft Security Research and Microsoft Threat Intelligence, AI-driven, large-scale nation-state campaign using ARTEX enables supply chain compromise, 21 October 2026. https://www.microsoft.com/en-us/security/blog/2026/10/21/storm-0940-uses-artex-agentic-ai-connectwise-supply-chain-compromise/

[10] Darktrace, Advancing the Use of Frontier AI in Cybersecurity: Darktrace Joins the OpenAI Daybreak Cyber Partner Program to Explore Defensive AI Integrations, 22 June 2026. https://www.darktrace.com/blog/advancing-the-use-of-frontier-ai-in-cybersecurity-darktrace-joins-the-openai-daybreak-cyber-partner-program-to-explore-defensive-ai-integrations

[Continue reading](https://www.darktrace.com/blog/vulnerability-discovery-to-active-attacks-ai-in-cyber-operations)

About the author

Nathaniel Jones

SVP, Global Threat Intelligence

## Blog

/

## OT

/

## November 2, 2026

## Beyond the Fridge: Exposed Industrial Control Systems

## Introduction

In October 2026, hundreds of supermarkets across France were reportedly affected by a cyberattack targeting their refrigeration systems. According to public reports, the attackers gained access to internet-connected controllers and modified their settings, forcing refrigeration units into high and extremely low temperatures and causing significant disruption. Total losses have been reported to reach several million euros.

The incident drew attention to the security of industrial refrigeration systems, particularly older devices with exposed management interfaces and known vulnerabilities.

The exact details of how the attackers gained access have not been confirmed, however it is speculated that the public-facing web server could be accessed with the username “admin” and password being the current date. Researchers from Claroty’s Team82 identified a vulnerability in Danfoss AK-SM 8xxA Series in 2025, “CVE-2025-41450", allowing for administrative access using a time-based “code-of-the-day" for authentication [1].

Default credentials, outdated firmware and publicly accessible administrative interfaces have been an issue for years. And yet these systems continue to appear in internet scans and require little sophistication to gain admin access. A simple search of internet-connected devices shows a range of exposed devices from Programmable Logic Controllers (PLCs) and remote gateways, to, yes, more fridges.

Darktrace malware researchers identified hundreds of IoT and OT devices publicly exposed on the internet, many of which are configured with no authentication, weak credentials, or default credentials. As shown in Figure 1, an exposed Copeland XWEB web interface, used to manage Dixell equipment, including refrigerators, can provide the ability to remotely control connected devices.

‍

Figure 1: Copeland XWEB interface used to control systems including refrigerators and air conditioning.

However, it’s not only fridges; web management interfaces for industrial controls are also publicly accessible, often protected only by weak credentials. Figure 2 shows an example of a publicly exposed WAGO interface. The WAGO management interface is a configuration and administration panel for controllers. Through it, a logged-in user can view device details and diagnostics, manage user accounts and passwords, update firmware, back up or restore the full configuration, set the clock, and manage SD card storage. While it typically doesn’t manage switch outputs or read live process values, someone with access could still disrupt operation by changing network or Modbus settings, loading bad firmware or configurations, or by resetting passwords and enabling insecure services.

‍

Figure 2: WAGO Management interface.

‍

The issue extends beyond management interfaces. During our research, we identified publicly accessible web interfaces belonging to PLCs, including Rockwell Automation Allen-Bradley devices. In each case, the risk arises from how the devices have been deployed and configured, not from a flaw in the vendors’ products.

‍

Figure 3: Web interface for Allen-Bradley CompactLogix controller.

‍

Figure 3 shows the web interface of an Allen-Bradley CompactLogix controller. Unlike the previous examples, this interface provides a view into the controller's operation, including running tasks, network connections, memory usage and diagnostic information. Rockwell Automation's CompactLogix controllers are commonly used to automate industrial processes, from manufacturing and packaging lines to water treatment and material handling systems. Depending on their configuration, these controllers can operate motors, pumps, valves and other physical equipment.

Although in this example the interface shown does not provide direct control over the PLC's logic, it exposes information about the model, firmware, network layout, and connections, which is useful reconnaissance for a bigger attack.

The concern is not necessarily that every exposed interface allows an attacker to immediately manipulate industrial processes. However, these devices are accessible from the public internet in the first place, providing information about systems that may be responsible for controlling physical equipment.

The attacks on refrigeration systems in France highlight the consequences of poorly secured industrial devices. As demonstrated throughout this research, publicly accessible management interfaces are not limited to refrigeration controllers but extend to PLCs and other systems used to manage industrial equipment.

Finding these devices requires little effort. Internet scanners make it possible to identify publicly exposed interfaces quickly, and in some cases these interfaces rely on publicly documented default credentials or have no authentication at all. As attackers increasingly use AI tools, such attacks may be identified and carried out faster and at a greater scale.

The consequences of this access depend on the device and the functionality exposed. While some interfaces provide little more than diagnostic information, others allow users to modify configurations, disrupt communications, or change settings that affect physical equipment. In industrial environments, this can result in operational downtime, damaged equipment and financial losses.

Weak passwords, default credentials and exposed administrative interfaces are not new security issues. Yet their continued presence on internet-connected industrial systems demonstrates how easily preventable weaknesses can leave physical operations vulnerable to attack.

Exposed devices often come from third-party suppliers and contractors, who frequently install refrigeration controllers, gateways and PLCs without the organization ever being aware they are connected to the internet. Asset visibility remains an important foundation, but organizations should assume that some unknown assets and services will exist. Effective security therefore also depends on continuous monitoring, anomaly detection, network segmentation and rapid response, so that risky exposure and unusual activity can be identified and addressed before it becomes disruptive.

Darktrace helps security teams gain visibility across IT and OT and respond to risks: https://www.darktrace.com/products/operational-technology.

Copeland, Dixell, WAGO, Rockwell Automation, Allen-Bradley and CompactLogix are trademarks of their respective owners. Their use here is for identification only and does not imply any affiliation with or endorsement by those owners.

## References

1. https://claroty.com/team82/research/freeze-the-controller-defrost-the-food-uncovering-vulnerabilities-in-danfoss-refrigeration-controllers

‍

[Continue reading](https://www.darktrace.com/blog/beyond-the-fridge-exposed-industrial-control-systems)

About the author

Tara Gould

Malware Research Lead

Your data. Our AI.

Elevate your network security with Darktrace AI

[Get a demo](https://www.darktrace.com/demo)
