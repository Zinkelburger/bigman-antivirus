---
url: "https://attack.mitre.org/software/S9043/"
title: "Mini Shai-Hulud, Software S9043 | MITRE ATT&CK®"
captured: "2026-11-02T18:23:58Z"
via: "live"
sha256: "7f74d254d68b8c01f89b7e08e6f3fd0b418427cfa98684c397eacd4d4783d85a"
wayback: "https://web.archive.org/web/20260925051238/https://attack.mitre.org/software/S9043/"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Mini Shai-Hulud, Software S9043 | MITRE ATT&CK®

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://attack.mitre.org/software/S9043/>
> Snapshot: <https://web.archive.org/web/20260925051238/https://attack.mitre.org/software/S9043/>

ATT&CK v20 has been released! Check out the [blog post](https://medium.com/mitre-attack/attack-v20-9701dd75ffb4) for more information. [ATT&CKcon 7.0](https://na.eventscloud.com/website/93925/) is October 27-28, 2026 in McLean, VA.

- [Home](https://attack.mitre.org/)
- [Software](https://attack.mitre.org/software/)
- Mini Shai-Hulud

#  Mini Shai-Hulud

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) is a credential stealer and self-replicating supply chain worm, derived from [Shai-Hulud](https://attack.mitre.org/software/S9008), that has been used by [TeamPCP](https://attack.mitre.org/groups/G1056) to target Continuous Integration and Continuous Delivery/Deployment (CI/CD) workflows since at least 2026. [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) can compromise credentials across multiple cloud, container, and AI configuration file paths and can use stolen npm and GitHub OIDC tokens to spread to other packages maintained by the compromised user. [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) also has a targeted wiper component and has used multiple C2 and data exfiltration mechanisms.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)[[6]](https://www.ic3.gov/CSA/2026/260702.pdf)

ID: S9043

ⓘ

Type: MALWARE

ⓘ

Platforms: Containers, IaaS, Linux, macOS, SaaS, Windows

Contributors: Rajkumar Barot

Version: 1.0

Created: 29 July 2026

Last Modified: 31 July 2026

[Version Permalink](https://attack.mitre.org/versions/v20/software/S9043/)

[Live Version](https://attack.mitre.org/versions/v20/software/S9043/)

###### Enterprise Layer

[download](https://attack.mitre.org/software/S9043/S9043-enterprise-layer.json)  [view](https://attack.mitre.org/software/S9043/#)

## Techniques Used

| Domain  | ID  | Name  | Use

|  Enterprise   |  [T1087](https://attack.mitre.org/techniques/T1087)   |  [.004](https://attack.mitre.org/techniques/T1087/004)   |  [Account Discovery](https://attack.mitre.org/techniques/T1087): [Cloud Account](https://attack.mitre.org/techniques/T1087/004)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has enumerated cloud accounts and subscriptions accessible to the targeted identity.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1071](https://attack.mitre.org/techniques/T1071)   |  [.001](https://attack.mitre.org/techniques/T1071/001)   |  [Application Layer Protocol](https://attack.mitre.org/techniques/T1071): [Web Protocols](https://attack.mitre.org/techniques/T1071/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has has exfiltrated data through the use of HTTPS POST requests to C2 domains.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1560](https://attack.mitre.org/techniques/T1560)   |  [Archive Collected Data](https://attack.mitre.org/techniques/T1560)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has compressed collected credentials to reduce transmission size and to make string content harder to detect in memory forensics captures.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|   |   |  [.001](https://attack.mitre.org/techniques/T1560/001)   |  [Archive via Utility](https://attack.mitre.org/techniques/T1560/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has compressed collected credentials and data within tar archive files prior to exfiltration.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)

|  Enterprise   |  [T1119](https://attack.mitre.org/techniques/T1119)   |  [Automated Collection](https://attack.mitre.org/techniques/T1119)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has the ability to automatically compile gathered credentials from configuration files and password vaults within an archive and exfiltrate stolen data leveraging both a primary and fallback C2.[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1059](https://attack.mitre.org/techniques/T1059)   |  [.006](https://attack.mitre.org/techniques/T1059/006)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Python](https://attack.mitre.org/techniques/T1059/006)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has utilized Python scripts to execute payloads.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|   |   |  [.007](https://attack.mitre.org/techniques/T1059/007)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [JavaScript](https://attack.mitre.org/techniques/T1059/007)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has leveraged JavaScript runtime to execute malicious scripts.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|   |   |  [.013](https://attack.mitre.org/techniques/T1059/013)   |  [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059): [Container CLI/API](https://attack.mitre.org/techniques/T1059/013)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has utilized the Docker command-line tool to gather details of the victim environment and collect credentials.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1554](https://attack.mitre.org/techniques/T1554)   |  [Compromise Host Software Binary](https://attack.mitre.org/techniques/T1554)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has established persistence through modifying software binaries to include AI coding agents’ configuration or setting files that act as hooks, tasks or execution triggers.[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1609](https://attack.mitre.org/techniques/T1609)   |  [Container Administration Command](https://attack.mitre.org/techniques/T1609)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has utilized container administration commands to gather details of compromised hosts and gather credentials to include Kubernetes command-line utilities kubectl get secrets.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1543](https://attack.mitre.org/techniques/T1543)   |  [.001](https://attack.mitre.org/techniques/T1543/001)   |  [Create or Modify System Process](https://attack.mitre.org/techniques/T1543): [Launch Agent](https://attack.mitre.org/techniques/T1543/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has established persistence on macOS hosts by installing a gh-token-monitor daemon through LaunchAgent that polls GitHub every 60 seconds.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)

|   |   |  [.002](https://attack.mitre.org/techniques/T1543/002)   |  [Create or Modify System Process](https://attack.mitre.org/techniques/T1543): [Systemd Service](https://attack.mitre.org/techniques/T1543/002)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has created .service files using Systemd on victim Linux hosts to establish persistence.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1555](https://attack.mitre.org/techniques/T1555)   |  [.005](https://attack.mitre.org/techniques/T1555/005)   |  [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Password Managers](https://attack.mitre.org/techniques/T1555/005)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has gathered credentials stored in password managers to include password vaults.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|   |   |  [.006](https://attack.mitre.org/techniques/T1555/006)   |  [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555): [Cloud Secrets Management Stores](https://attack.mitre.org/techniques/T1555/006)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has captured credentials stored in cloud secret stores.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1485](https://attack.mitre.org/techniques/T1485)   |  [Data Destruction](https://attack.mitre.org/techniques/T1485)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has wiped data on devices that fall within specified parameters to include those that resolve to specific geolocations including Iran and Israel.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/) [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has also implemented a dead-man’s switch that wipes the victims home directory if the operator revokes a GitHub token created by the adversary. [[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1132](https://attack.mitre.org/techniques/T1132)   |  [.001](https://attack.mitre.org/techniques/T1132/001)   |  [Data Encoding](https://attack.mitre.org/techniques/T1132): [Standard Encoding](https://attack.mitre.org/techniques/T1132/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has used base64 encoding to obfuscate URLs used for C2.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1213](https://attack.mitre.org/techniques/T1213)   |  [.003](https://attack.mitre.org/techniques/T1213/003)   |  [Data from Information Repositories](https://attack.mitre.org/techniques/T1213): [Code Repositories](https://attack.mitre.org/techniques/T1213/003)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has gathered and downloaded data stored on both compromised and publicly accessible code repositories.[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1140](https://attack.mitre.org/techniques/T1140)   |  [Deobfuscate/Decode Files or Information](https://attack.mitre.org/techniques/T1140)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has the ability to decrypt obfuscated payloads.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)

|  Enterprise   |  [T1546](https://attack.mitre.org/techniques/T1546)   |  [Event Triggered Execution](https://attack.mitre.org/techniques/T1546)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has modified settings and configuration files of AI coding agents and other coding applications in order to create event triggered executions through creating hooks and runOn conditions.[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|   |   |  [.018](https://attack.mitre.org/techniques/T1546/018)   |  [Python Startup Hooks](https://attack.mitre.org/techniques/T1546/018)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has utilized Python startup hooks to include the .pth import mechanism for execution.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)

|  Enterprise   |  [T1480](https://attack.mitre.org/techniques/T1480)   |  [Execution Guardrails](https://attack.mitre.org/techniques/T1480)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has utilized execution guardrails in order to prevent operating in restricted geolocations to include Russia by checking the devices language and terminating when a forbidden value is detected.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/) [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has also utilized designated instructions that execute when victim hosts match geolocations to include wiping victim devices when the device is determined to be located within Iran or Israel.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1041](https://attack.mitre.org/techniques/T1041)   |  [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has exfiltrated encrypted archives over C2 domains.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1567](https://attack.mitre.org/techniques/T1567)   |  [.001](https://attack.mitre.org/techniques/T1567/001)   |  [Exfiltration Over Web Service](https://attack.mitre.org/techniques/T1567): [Exfiltration to Code Repository](https://attack.mitre.org/techniques/T1567/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has exfiltrated data through the use of the victim’s own GitHub repository by creating a new public repository using a unique naming convention from a curated list of key words or themes.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1008](https://attack.mitre.org/techniques/T1008)   |  [Fallback Channels](https://attack.mitre.org/techniques/T1008)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has established Fallback Channels to exfiltrate data to Github when other configured infrastructure is found to be unreachable.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1083](https://attack.mitre.org/techniques/T1083)   |  [File and Directory Discovery](https://attack.mitre.org/techniques/T1083)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has enumerated home directories, file paths and files associated with storing or containing credentials and other secrets.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1564](https://attack.mitre.org/techniques/T1564)   |  [.011](https://attack.mitre.org/techniques/T1564/011)   |  [Hide Artifacts](https://attack.mitre.org/techniques/T1564): [Ignore Process Interrupts](https://attack.mitre.org/techniques/T1564/011)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has suppressed output so that nothing is printed to terminal and has utilized silent exiting when environmental variables match restricted values.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1070](https://attack.mitre.org/techniques/T1070)   |  [.004](https://attack.mitre.org/techniques/T1070/004)   |  [Indicator Removal](https://attack.mitre.org/techniques/T1070): [File Deletion](https://attack.mitre.org/techniques/T1070/004)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has deleted all artifacts to include gathered credential archives to reduce disk persistence and detection.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)

|  Enterprise   |  [T1105](https://attack.mitre.org/techniques/T1105)   |  [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has the ability to download additional payloads from adversary controlled or compromised infrastructure.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1559](https://attack.mitre.org/techniques/T1559)   |  [Inter-Process Communication](https://attack.mitre.org/techniques/T1559)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has executed via the use of subprocess.run and fed input through standard input stdin which acted as a pipe to send data from the parent process and the child process sys.executable within memory.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1036](https://attack.mitre.org/techniques/T1036)   |  [.005](https://attack.mitre.org/techniques/T1036/005)   |  [Masquerading](https://attack.mitre.org/techniques/T1036): [Match Legitimate Resource Name or Location](https://attack.mitre.org/techniques/T1036/005)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has leveraged a user-agent string that mimics a standard git client to avoid detection within network logs.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1027](https://attack.mitre.org/techniques/T1027)   |  [.013](https://attack.mitre.org/techniques/T1027/013)   |  [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027): [Encrypted/Encoded File](https://attack.mitre.org/techniques/T1027/013)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has used a hybrid AES-256-GCM and RSA OAEP-SHA256 encryption to archive gathered data. [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has also utilized custom MD5-keystream XOR cipher to encrypt data.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html) [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has also been deployed via an obfuscated script using Bun JavaScript runtime.[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1003](https://attack.mitre.org/techniques/T1003)   |  [.007](https://attack.mitre.org/techniques/T1003/007)   |  [OS Credential Dumping](https://attack.mitre.org/techniques/T1003): [Proc Filesystem](https://attack.mitre.org/techniques/T1003/007)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has scraped runner process memory to extract short-lived identity tokens, which it then exchanged for per-package npm trusted-publisher tokens.[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1677](https://attack.mitre.org/techniques/T1677)   |  [Poisoned Pipeline Execution](https://attack.mitre.org/techniques/T1677)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has utilized Github Actions to propagate through the use of triggered workflows.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1090](https://attack.mitre.org/techniques/T1090)   |  [.003](https://attack.mitre.org/techniques/T1090/003)   |  [Proxy](https://attack.mitre.org/techniques/T1090): [Multi-hop Proxy](https://attack.mitre.org/techniques/T1090/003)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has the ability to exfiltrate stolen credentials via the Session messenger network.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)

|  Enterprise   |  [T1021](https://attack.mitre.org/techniques/T1021)   |  [.007](https://attack.mitre.org/techniques/T1021/007)   |  [Remote Services](https://attack.mitre.org/techniques/T1021): [Cloud Services](https://attack.mitre.org/techniques/T1021/007)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has accessed and propagated to AWS EC2 instances via SSM Send-Command.[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1053](https://attack.mitre.org/techniques/T1053)   |  [.006](https://attack.mitre.org/techniques/T1053/006)   |  [Scheduled Task/Job](https://attack.mitre.org/techniques/T1053): [Systemd Timers](https://attack.mitre.org/techniques/T1053/006)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has obtained persistence on Linux devices by writing the gh-token-monitor daemon within ~/.config/systemd/user/gh-token-monitor.service that polls GitHub every 60 seconds.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised) [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has also leveraged a daemon called "kitty-monitor.service" to maintain persistence within Linux hosts.[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1528](https://attack.mitre.org/techniques/T1528)   |  [Steal Application Access Token](https://attack.mitre.org/techniques/T1528)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has stolen application access tokens and other tokens to include those associated with CI/CD.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1649](https://attack.mitre.org/techniques/T1649)   |  [Steal or Forge Authentication Certificates](https://attack.mitre.org/techniques/T1649)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has collected victim client certificates to assist in signed authentication assertion with Azure environments.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1195](https://attack.mitre.org/techniques/T1195)   |  [.001](https://attack.mitre.org/techniques/T1195/001)   |  [Supply Chain Compromise](https://attack.mitre.org/techniques/T1195): [Compromise Software Dependencies and Development Tools](https://attack.mitre.org/techniques/T1195/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has published itself on compromised victim code repositories to propagate malicious versions of packages to other victims.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1082](https://attack.mitre.org/techniques/T1082)   |  [System Information Discovery](https://attack.mitre.org/techniques/T1082)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has gathered system information of victim hosts through the use of common discovery commands to include hostname, uname-a and printenv.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html) [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has also conducted system checks of the victim device to include enumerating the system type and the number of CPUs operating on victim host.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1614](https://attack.mitre.org/techniques/T1614)   |  [System Location Discovery](https://attack.mitre.org/techniques/T1614)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has discovered the compromised systems location through a query of the system timezone configuration and the locale settings.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|   |   |  [.001](https://attack.mitre.org/techniques/T1614/001)   |  [System Language Discovery](https://attack.mitre.org/techniques/T1614/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has the ability to check system details for its language configuration and terminates actions when the system is configured for the Russian language.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1016](https://attack.mitre.org/techniques/T1016)   |  [System Network Configuration Discovery](https://attack.mitre.org/techniques/T1016)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has discovered network configuration through the use of system commands to include ip addr, and ip route.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)

|  Enterprise   |  [T1033](https://attack.mitre.org/techniques/T1033)   |  [System Owner/User Discovery](https://attack.mitre.org/techniques/T1033)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has leveraged commands such as whoami to identify the system owner.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)

|  Enterprise   |  [T1124](https://attack.mitre.org/techniques/T1124)   |  [System Time Discovery](https://attack.mitre.org/techniques/T1124)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has queried the system timezone configuration and timezone data files to include /etc/localtime, and locale settings to determine the geolocation of the compromised host.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1205](https://attack.mitre.org/techniques/T1205)   |  [Traffic Signaling](https://attack.mitre.org/techniques/T1205)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has examined commit messages for a keyword followed by base64 encoded segments to validate communications and to execute subsequent actions to include exfiltration.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1552](https://attack.mitre.org/techniques/T1552)   |  [.001](https://attack.mitre.org/techniques/T1552/001)   |  [Unsecured Credentials](https://attack.mitre.org/techniques/T1552): [Credentials In Files](https://attack.mitre.org/techniques/T1552/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has collected credentials stored within configuration files.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/) [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has also gathered credentials from files stored in common credential file paths to include targeting git-credentials, azureProfile.json, and application_default_credentials.json.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|   |   |  [.004](https://attack.mitre.org/techniques/T1552/004)   |  [Unsecured Credentials](https://attack.mitre.org/techniques/T1552): [Private Keys](https://attack.mitre.org/techniques/T1552/004)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has gathered unsecured credentials to include SSH private keys within .ssh.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|   |   |  [.005](https://attack.mitre.org/techniques/T1552/005)   |  [Unsecured Credentials](https://attack.mitre.org/techniques/T1552): [Cloud Instance Metadata API](https://attack.mitre.org/techniques/T1552/005)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has gathered credentials and secrets from AWS, Google Cloud Platform (GCP) and Azure metadata API.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|   |   |  [.007](https://attack.mitre.org/techniques/T1552/007)   |  [Unsecured Credentials](https://attack.mitre.org/techniques/T1552): [Container API](https://attack.mitre.org/techniques/T1552/007)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has gathered unsecured API keys stored in container orchestrators.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|  Enterprise   |  [T1550](https://attack.mitre.org/techniques/T1550)   |  [.001](https://attack.mitre.org/techniques/T1550/001)   |  [Use Alternate Authentication Material](https://attack.mitre.org/techniques/T1550): [Application Access Token](https://attack.mitre.org/techniques/T1550/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has the ability to authenticate using stolen application access tokens.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

|  Enterprise   |  [T1078](https://attack.mitre.org/techniques/T1078)   |  [.004](https://attack.mitre.org/techniques/T1078/004)   |  [Valid Accounts](https://attack.mitre.org/techniques/T1078): [Cloud Accounts](https://attack.mitre.org/techniques/T1078/004)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has used compromised accounts for Docker Hub and GitHub to publish malicious software packages.[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)

|  Enterprise   |  [T1497](https://attack.mitre.org/techniques/T1497)   |  [Virtualization/Sandbox Evasion](https://attack.mitre.org/techniques/T1497)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has evaded sandbox detection by applying a 1-in-6 probability gate that generates a random number which will only trigger the wiper functionality when the set number outcome is met even in environments that match parameters of a geopolitical target.[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

|   |   |  [.001](https://attack.mitre.org/techniques/T1497/001)   |  [System Checks](https://attack.mitre.org/techniques/T1497/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has evaded execution in virtual environments and sandboxes through checking system information to include the number of CPUs and exiting at times when there were less than four and other times when there were less than two CPUs.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)

|  Enterprise   |  [T1102](https://attack.mitre.org/techniques/T1102)   |  [.001](https://attack.mitre.org/techniques/T1102/001)   |  [Web Service](https://attack.mitre.org/techniques/T1102): [Dead Drop Resolver](https://attack.mitre.org/techniques/T1102/001)   |

[Mini Shai-Hulud](https://attack.mitre.org/software/S9043) has leveraged GitHub commit-search API to recover fallback C2 domains stored in auto-created public Github repositories.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)

## Groups That Use This Software

| ID  | Name  | References

|  [G1056](https://attack.mitre.org/groups/G1056)   |  [TeamPCP](https://attack.mitre.org/groups/G1056)   |

[TeamPCP](https://attack.mitre.org/groups/G1056) has used [Mini Shai-Hulud](https://attack.mitre.org/software/S9043) in supply chain campaigns targeting CI/CD workflows.[[1]](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)[[2]](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)[[3]](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)[[4]](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)[[5]](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)[[6]](https://www.ic3.gov/CSA/2026/260702.pdf)

## References

-    [McCarthy, R., Cohen, A., and Read, B. (2026, May 12). Mini Shai-Hulud Strikes Again: TanStack + more npm Packages Compromised. Retrieved July 16, 2026.](https://www.wiz.io/blog/mini-shai-hulud-strikes-again-tanstack-more-npm-packages-compromised)
-    [Santos, J. and Navato, J.R. (2026, May 13). Analyzing TeamPCP’s Supply Chain Attacks: Checkmarx KICS and elementary-data in CI/CD Credential Theft. Retrieved July 16, 2026.](https://www.trendmicro.com/en_us/research/26/e/analyzing-teampcp-supply-chain-attacks.html)
-    [Hunt.io. (2026, May 14). How TeamPCP's Python Toolkit Survives a C2 Takedown: FIRESCALE, GitHub, and the Victim's Own Account. Retrieved July 16, 2026.](https://hunt.io/blog/teampcp-python-toolkit-firescale-github-c2-takedown)

-    [Webb, M. (2026, May 20). TeamPCP Wave Four: GitHub Breach via Poisoned VS Code Extension, durabletask PyPI Worm, and ~4,000 Internal Repositories Exfiltrated. Retrieved July 16, 2026.](https://phoenix.security/teampcp-github-breach-durabletask-pypi-supply-chain-wave-four-2026/)
-    [Flashpoint. (2026, May 28). The Mini Shai-Hulud Worm and the New Era of CI/CD Exploitation. Retrieved July 16, 2026.](https://flashpoint.io/blog/mini-shai-hulud-worm-new-era-ci-cd-exploitation/)
-    [FBI. (2026, July 2). Cyber Criminal Group TeamPCP. Retrieved July 7, 2026.](https://www.ic3.gov/CSA/2026/260702.pdf)

×
