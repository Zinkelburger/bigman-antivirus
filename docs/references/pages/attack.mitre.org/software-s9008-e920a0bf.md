---
url: "https://attack.mitre.org/software/S9008/"
title: "Shai-Hulud, Software S9008 | MITRE ATT&CK®"
captured: "2026-11-02T18:23:57Z"
via: "live"
sha256: "9fba5ca6e6825438996d982170288ee047c5f7c8e984506abae0a97d9af34f7b"
wayback: "https://web.archive.org/web/20261024162433/https://attack.mitre.org/software/S9008/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Shai-Hulud, Software S9008 | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/software/S9008/>
> Snapshot: <https://web.archive.org/web/20261024162433/https://attack.mitre.org/software/S9008/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Software](https://attack.mitre.org/software/)
- Shai-Hulud

#  Shai-Hulud

[Shai-Hulud](https://attack.mitre.org/software/S9008) is a supply chain worm, first reported in September 2025, that spreads through code repositories, including GitHub and NPM packages. It exploits CI/CD pipeline dependencies to propagate to victims and poisons the supply chain by publishing malicious packages. Once inside a victim environment, [Shai-Hulud](https://attack.mitre.org/software/S9008) steals credentials and access tokens from compromised repository accounts and exfiltrates them to attacker-controlled servers via encoded GitHub Actions workflows.[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)

ID: S9008

ⓘ

Type: MALWARE

ⓘ

Platforms: Linux, SaaS, Windows

Contributors: Caio Silva; Alberto Garcia; Pedro Rodriguez; Víctor Alba

Version: 1.0

Created: 09 April 2026

Last Modified: 24 April 2026

[Version Permalink](https://attack.mitre.org/versions/v20/software/S9008/)

[Live Version](https://attack.mitre.org/versions/v20/software/S9008/)

###### Enterprise Layer

[download](https://attack.mitre.org/software/S9008/S9008-enterprise-layer.json)  [view](https://attack.mitre.org/software/S9008/#)

## Techniques Used

| Domain  | ID  | Name  | Use

|  Enterprise   |  [T1548](https://attack.mitre.org/techniques/T1548)   |  [.003](https://attack.mitre.org/techniques/T1548/003)   |  [Abuse Elevation Control Mechanism](https://attack.mitre.org/techniques/T1548): [Sudo and Sudo Caching](https://attack.mitre.org/techniques/T1548/003)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has attempted to gain root access by leveraging sudo and /etc/sudoers.d.[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1098](https://attack.mitre.org/techniques/T1098)   |  [Account Manipulation](https://attack.mitre.org/techniques/T1098)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has modified GitHub account settings for private repositories and changed them to public.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)

|  Enterprise   |  [T1071](https://attack.mitre.org/techniques/T1071)   |  [.001](https://attack.mitre.org/techniques/T1071/001)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Web Protocols](https://attack.mitre.org/techniques/T1071/001)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has utilized curl to install Bun over HTTPS.[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)

|  Enterprise   |  [T1119](https://attack.mitre.org/techniques/T1119)   |  [Automated Collection](https://attack.mitre.org/techniques/T1119)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has the ability to automatically collect host data, secrets, system information, and endpoints.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)

|  Enterprise   |  [T1059](https://attack.mitre.org/techniques/T1059)   |  [.001](https://attack.mitre.org/techniques/T1059/001)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [PowerShell](https://attack.mitre.org/techniques/T1059/001)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has utilized PowerShell Invoke-WebRequest to download and install the malicious payload.[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)

|   |   |  [.004](https://attack.mitre.org/techniques/T1059/004)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Unix Shell](https://attack.mitre.org/techniques/T1059/004)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has utilized Linux shell commands to modify configuration files.[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|   |   |  [.007](https://attack.mitre.org/techniques/T1059/007)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [JavaScript](https://attack.mitre.org/techniques/T1059/007)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has used JavaScript to create JSON file output and run scripts using node.js.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)

|  Enterprise   |  [T1543](https://attack.mitre.org/techniques/T1543)   |  [.002](https://attack.mitre.org/techniques/T1543/002)   |  [Create or Modify System Process](https://attack.mitre.org/techniques/T1543): [Systemd Service](https://attack.mitre.org/techniques/T1543/002)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has stopped systemd-resolved in order to manipulate DNS and firewalls.[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1555](https://attack.mitre.org/techniques/T1555)   |  [.006](https://attack.mitre.org/techniques/T1555/006)   |  [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Cloud Secrets Management Stores](https://attack.mitre.org/techniques/T1555/006)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has gathered secrets from AWS Secrets and GCP Secret Manager.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also gathered data from Azure Key Vault.[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1485](https://attack.mitre.org/techniques/T1485)   |  [Data Destruction](https://attack.mitre.org/techniques/T1485)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has destroyed the victim’s home directory by overwriting and deleting every writable file within the user's home folder.[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also utilized the shred command on Linux devices.[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)

|  Enterprise   |  [T1213](https://attack.mitre.org/techniques/T1213)   |  [.003](https://attack.mitre.org/techniques/T1213/003)   |  [Data from Information Repositories](https://attack.mitre.org/techniques/T1213): [Code Repositories](https://attack.mitre.org/techniques/T1213/003)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has downloaded existing packages from code repositories and extracted data stored within them.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)

|  Enterprise   |  [T1678](https://attack.mitre.org/techniques/T1678)   |  [Delay Execution](https://attack.mitre.org/techniques/T1678)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has delayed execution of its larger payloads by forking itself into background process.[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)

|  Enterprise   |  [T1685](https://attack.mitre.org/techniques/T1685)   |  [Disable or Modify Tools](https://attack.mitre.org/techniques/T1685)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has replaced DNS configuration from /tmp/resolved.conf in order to gain control of network-level control within CI environments and has flushed iptables rules using sudo iptables -F OUTPUT and sudo iptables -F DOCKER-USER.[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1546](https://attack.mitre.org/techniques/T1546)   |  [.016](https://attack.mitre.org/techniques/T1546/016)   |  [Event Triggered Execution](https://attack.mitre.org/techniques/T1546): [Installer Packages](https://attack.mitre.org/techniques/T1546/016)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has inserted a new lifecycle hook to include postinstall.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also leveraged the NPM lifecycle hook preinstall.[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1041](https://attack.mitre.org/techniques/T1041)   |  [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has used POST to exfiltrate secrets from the victim environment to an attacker-controlled URL.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)

|  Enterprise   |  [T1567](https://attack.mitre.org/techniques/T1567)   |  [.001](https://attack.mitre.org/techniques/T1567/001)   |  [Exfiltration Over Web Service](https://attack.mitre.org/techniques/T1567): [Exfiltration to Code Repository](https://attack.mitre.org/techniques/T1567/001)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has created a repository named Shai-Hulud under the compromised account that commits a JSON dump that contains system information, environment variables and collected secrets.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also posted stolen credentials to public GitHub repositories.[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)

|   |   |  [.004](https://attack.mitre.org/techniques/T1567/004)   |  [Exfiltration Over Web Service](https://attack.mitre.org/techniques/T1567): [Exfiltration Over Webhook](https://attack.mitre.org/techniques/T1567/004)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has exfiltrated repository secrets to webhook[.]site.[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)

|  Enterprise   |  [T1564](https://attack.mitre.org/techniques/T1564)   |  [.011](https://attack.mitre.org/techniques/T1564/011)   |  [Hide Artifacts](https://attack.mitre.org/techniques/T1564): [Ignore Process Interrupts](https://attack.mitre.org/techniques/T1564/011)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has suppressed NPM warnings by silently exiting through the use of the NPM success code that has a setting that all errors exit with code 0.[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1105](https://attack.mitre.org/techniques/T1105)   |  [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has downloaded packages from code repositories.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also downloaded and executed the secrets-discovery tool [TruffleHog](https://attack.mitre.org/software/S9009) to gather sensitive data.[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)

|  Enterprise   |  [T1036](https://attack.mitre.org/techniques/T1036)   |  [.005](https://attack.mitre.org/techniques/T1036/005)   |  [Masquerading](https://attack.mitre.org/techniques/T1036): [Match Legitimate Resource Name or Location](https://attack.mitre.org/techniques/T1036/005)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has masqueraded as a legitimate Bun installer.[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|   |   |  [.009](https://attack.mitre.org/techniques/T1036/009)   |  [Masquerading](https://attack.mitre.org/techniques/T1036): [Break Process Trees](https://attack.mitre.org/techniques/T1036/009)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has augmented its installation process by having its original install process exit cleanly to provide the user with the illusion that the service is installed normally.[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1027](https://attack.mitre.org/techniques/T1027)   |  [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has utilized double-base64 encoding to store stolen secrets within the Github Action Logs within the victim account.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also leveraged three layers of base64 encoding of exfiltrated data for anti-forensic purposes.[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1677](https://attack.mitre.org/techniques/T1677)   |  [Poisoned Pipeline Execution](https://attack.mitre.org/techniques/T1677)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has also leveraged GitHub actions from stolen accounts in order to create a malicious Github workflow within .github/workflows/discussion.yaml.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1593](https://attack.mitre.org/techniques/T1593)   |  [.003](https://attack.mitre.org/techniques/T1593/003)   |  [Search Open Websites/Domains](https://attack.mitre.org/techniques/T1593): [Code Repositories](https://attack.mitre.org/techniques/T1593/003)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has the ability to search open sites and code repositories for compromised credentials.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/) [Shai-Hulud](https://attack.mitre.org/software/S9008) has discovered packages associated with compromised accounts.[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also searched code repositories for other compromised repositories that include predefined parameters or markers to include "Second Coming" combined with an 18-character alphanumeric string.[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)

|  Enterprise   |  [T1608](https://attack.mitre.org/techniques/T1608)   |  [.001](https://attack.mitre.org/techniques/T1608/001)   |  [Stage Capabilities](https://attack.mitre.org/techniques/T1608): [Upload Malware](https://attack.mitre.org/techniques/T1608/001)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has published malicious gzip-compressed tarball (.tgz) following modification of packages within compromised accounts.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also modified packages within compromised accounts.[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)

|  Enterprise   |  [T1528](https://attack.mitre.org/techniques/T1528)   |  [Steal Application Access Token](https://attack.mitre.org/techniques/T1528)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has stolen access tokens and API tokens from with CI/CD pipeline solutions and repositories.[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1553](https://attack.mitre.org/techniques/T1553)   |  [Subvert Trust Controls](https://attack.mitre.org/techniques/T1553)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has suppressed victim NPM warnings using process["exit’](0x0); which results in having all errors exit with code 0.[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1195](https://attack.mitre.org/techniques/T1195)   |  [.001](https://attack.mitre.org/techniques/T1195/001)   |  [Supply Chain Compromise](https://attack.mitre.org/techniques/T1195): [Compromise Software Dependencies and Development Tools](https://attack.mitre.org/techniques/T1195/001)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has published itself on compromised code repository maintainers within infected packages in attempts to propagate to other victims.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also modified versions of code packages.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1082](https://attack.mitre.org/techniques/T1082)   |  [System Information Discovery](https://attack.mitre.org/techniques/T1082)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has gathered victim system information.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)

|  Enterprise   |  [T1552](https://attack.mitre.org/techniques/T1552)   |  [.001](https://attack.mitre.org/techniques/T1552/001)   |  [Unsecured Credentials](https://attack.mitre.org/techniques/T1552): [Credentials In Files](https://attack.mitre.org/techniques/T1552/001)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has gathered sensitive data stored in the Node.JS file process.env to include credentials and API keys.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages) [Shai-Hulud](https://attack.mitre.org/software/S9008) has harvested credentials stored in config files and credential files in victim environments to include ~/.aws/credentials, application_default_credentials.json, and azureProfile.json.[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also targeted credentials and tokens stored in NPM files .npmrc and GitHub config files.[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)

|   |   |  [.005](https://attack.mitre.org/techniques/T1552/005)   |  [Unsecured Credentials](https://attack.mitre.org/techniques/T1552): [Cloud Instance Metadata API](https://attack.mitre.org/techniques/T1552/005)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has queried the AWS and GCP metadata endpoints for instances and service credentials.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)

|  Enterprise   |  [T1550](https://attack.mitre.org/techniques/T1550)   |  [.001](https://attack.mitre.org/techniques/T1550/001)   |  [Use Alternate Authentication Material](https://attack.mitre.org/techniques/T1550): [Application Access Token](https://attack.mitre.org/techniques/T1550/001)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has leveraged captured valid NPM tokens to enumerate and update packages on compromised accounts.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages) [Shai-Hulud](https://attack.mitre.org/software/S9008) has also utilized stolen GitHub access tokens to access compromised accounts.[[3]](https://socket.dev/blog/shai-hulud-strikes-again-v2)[[4]](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)

|  Enterprise   |  [T1078](https://attack.mitre.org/techniques/T1078)   |  [.004](https://attack.mitre.org/techniques/T1078/004)   |  [Valid Accounts](https://attack.mitre.org/techniques/T1078): [Cloud Accounts](https://attack.mitre.org/techniques/T1078/004)   |

[Shai-Hulud](https://attack.mitre.org/software/S9008) has leveraged compromised accounts to log into cloud services to access cloud hosted repositories.[[5]](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)[[6]](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)[[1]](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)[[7]](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)[[2]](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)

## References

-    [Justin Moore. (2025, November 25). "Shai-Hulud" Worm Compromises npm Ecosystem in Supply Chain Attack (Updated November 26). Retrieved April 9, 2026.](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)
-    [Microsoft Defender Security Team. (n.d.). Shai-Hulud 2.0: Guidance for detecting, investigating, and defending against the supply chain attack. Retrieved April 9, 2026.](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)
-    [Socket Research Team. (2025, November 24). Shai Hulud Strikes Again (v2). Retrieved April 9, 2026.](https://socket.dev/blog/shai-hulud-strikes-again-v2)
-    [Socket Research Team. (2025, September 15). Popular Tinycolor npm Package Compromised in Supply Chain Attack Affecting 40+ Packages. Retrieved April 9, 2026.](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)

-    [Charlie Eriksen. (2025, September 16). S1ngularity/nx attackers strike again. Retrieved April 9, 2026.](https://www.aikido.dev/blog/s1ngularity-nx-attackers-strike-again)
-    [Gianpietro Cutolo. (2025, November 26). Shai-Hulud 2.0: Aggressive, Automated, and Fast Spreading. Retrieved April 9, 2026.](https://www.netskope.com/blog/shai-hulud-2-0-aggressive-automated-one-of-fastest-spreading-npm-supply-chain-attacks-ever-observed)
-    [Merav Bar, Rami McCarthy, Barak Sharoni. (2025, September 16). Shai-Hulud: Ongoing Package Supply Chain Worm Delivering Data-Stealing Malware. Retrieved April 9, 2026.](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack)

×
