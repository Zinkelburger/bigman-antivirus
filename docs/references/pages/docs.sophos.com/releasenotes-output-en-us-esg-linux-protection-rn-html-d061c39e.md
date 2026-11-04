---
url: "https://docs.sophos.com/releasenotes/output/en-us/esg/linux_protection_rn.html"
title: "Sophos Protection for Linux"
captured: "2026-11-02T18:29:59Z"
via: "live"
sha256: "22f2f9e87cf395309d9a49e47858723271aca5dbcb767a9476f7259a17d150dd"
wayback: "https://web.archive.org/web/20261102195424/https://docs.sophos.com/releasenotes/index.html?productGroupID=esg&productID=linux_protection&versionID=Agent"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Sophos Protection for Linux

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.sophos.com/releasenotes/output/en-us/esg/linux_protection_rn.html>
> Snapshot: <https://web.archive.org/web/20261102195424/https://docs.sophos.com/releasenotes/index.html?productGroupID=esg&productID=linux_protection&versionID=Agent>

# Sophos Protection for Linux

These are the release notes for Sophos Protection for Linux, managed by Sophos Central.

The features mentioned in these release notes are only available if you have the appropriate license and run a supported distribution and kernel. See [Sophos Protection for Linux distribution and kernel support](https://docs.sophos.com/esg/spl/en-us/index.html?contextId=protect-devices-server-Linux-distro-support).

You may find that you can't yet download and use the latest version. This is because Sophos releases the software over a number of days, but publishes the release notes on the first day.

View the product documentation at [Sophos Central](https://docs.sophos.com/central/Customer/help/en-us/index.html).

## Version 2026.3 New

### New features

#### Threat detection and protection

- Tamper Resiliency: Added automatic detection and recovery for unauthorized changes to SPL Agent files.

#### Improved usability

- Update Cache and Message Relay support: Linux servers with SPL Agent installed can now be configured as Update Caches and Message Relays in Sophos Central.
- Endpoint Self Help: Added a command-line tool that lets administrators view product health and collect diagnostic information from a Linux server.

#### Performance and resiliency improvements

- Extended kernel support: Added support for kernel 7.1.
- Runtime Detection resiliency: Improved handling of high-volume activity to make Runtime Detection more stable.

#### Data lake updates

- Expanded network visibility: Added DNS activity and Server Name Indication (SNI) hostname events to the Sophos Data Lake.
- Container attribution for detections: Added container information to supported detection events.

### Updated components

- Sophos Protection for Linux is updated to 2026.3.
- SPL-Base-Component is updated to 1.6.1.
- SPL-Endpoint-Detection-and-Response-Plugin is updated to 1.2.2.
- SPL-Live-Response-Plugin is updated to 1.7.10.
- SPL-Anti-Virus-Plugin is updated to 1.2.2.
- SPL-Event-Journaler-Plugin is updated to 1.1.5.
- SPL-Runtime-Detection-Plugin is updated to 5.20.0.
- SPL-Response-Actions-Plugin is updated to 1.1.5.
- SPL-Device-Isolation-Plugin is updated to 1.1.3.

## Components

Components and their version numbers by release. The second column contains the latest release.

| Sophos Protection for Linux  | 2026.3

September 2026  | 2026.2

May 2026  | 2026.1

March 2026  | 2025.2.2

November 2025  | 2025.2

October 2025  | 2025.1

March 2025  | 2024.3

November 2024  | 2024.2.1

September 2024  | 2024.2

June 2024  | 2024.1

February 2024  | 2023.4

November 2023  | 2023.3

September 2023  | 2023.2

June 2023  | 2023.1.3

May 2023  | 2023.1.2

April 2023  | 2023.1

January 2023  | 2022.4

October 2022  | 1.2.0

September 2022  | 1.1.10

June 2022  | 1.1.9.1

March 2022  | 1.1.9

February 2022  | 1.1.8

November 2021  | 1.1.7.1

August 2021

| 1.1.7

July 2021

| 1.1.6

June 2021

| 1.1.5

April 2021

| 1.1.4

December 2020

| SPL-Base-Component  | 1.6.1  | 1.6.0  | 1.6.0  | 1.5.0  | 1.5.0  | 1.4.0  | 1.3.0  | 1.2.9  | 1.2.8  | 1.2.7  | 1.2.6  | 1.2.5  | 1.2.4  | 1.2.3  | 1.2.3  | 1.2.2  | 1.2.1  | 1.2.0  | 1.1.10  | 1.1.9  | 1.1.9  | 1.1.8  | 1.1.7  | 1.1.7  | 1.1.6  | 1.1.5  | 1.1.4

| SPL-Endpoint-Detection-and-Response-Plugin  | 1.2.2  | 1.2.1  | 1.2.1  | 1.2.0  | 1.2.0  | 1.1.15  | 1.1.14  | 1.1.13  | 1.1.13  | 1.1.12  | 1.1.11  | 1.1.10  | 1.1.9  | 1.1.8  | 1.1.8  | 1.1.8  | 1.1.7  | 1.1.7  | 1.1.6  | 1.1.5  | 1.1.5  | 1.1.4  | 1.1.2  | 1.1.2  | 1.1.2  | 1.1.1  | 1.1.0

| SPL-Live-Response-Plugin  | 1.7.10  | 1.7.9  | 1.7.9  | 1.7.8  | 1.7.8  | 1.7.7  | 1.7.6  | 1.7.5  | 1.7.5  | 1.7.4  | 1.7.3  | 1.7.2  | 1.7.1  | 1.7.0  | 1.7.0  | 1.7.0  | 1.6.1  | 1.6.1  | 1.5.2  | 1.5.1  | 1.5.1  | 1.5.0  | 1.2.2  | 1.2.2  | 1.2.2  | 1.2.2  | 1.2.1

| SPL-Anti-Virus-Plugin  | 1.2.2  | 1.2.1  | 1.2.0  | 1.1.8  | 1.1.8  | 1.1.7  | 1.1.6  | 1.1.5  | 1.1.5  | 1.1.4  | 1.1.3  | 1.1.2  | 1.1.1  | 1.1.0  | 1.1.0  | 1.1.0  | 1.0.8  | 1.0.7  | 1.0.6  | 1.0.5  | 1.0.5  | 1.0.4  | 1.0.3  | 1.0.2  |   |   |

| SPL-Event-Journaler-Plugin  | 1.1.5  | 1.1.4  | 1.1.4  | 1.1.3  | 1.1.3  | 1.1.2  | 1.1.1  | 1.1.0  | 1.1.0  | 1.0.8  | 1.0.7  | 1.0.6  | 1.0.5  | 1.0.4  | 1.0.4  | 1.0.4  | 1.0.3  | 1.0.3  | 1.0.2  | 1.0.1  | 1.0.1  | 1.0.0  |   |   |   |   |

| SPL-Runtime-Detection-Plugin  | 5.20.0  | 5.17.3  | 5.16.3  | 5.15.1  | 5.14.0  | 5.10.0  | 5.9.3  | 5.9.1  | 5.9.1  | 5.8.0  | 5.7.0  | 5.6.0  | 5.3.0  | 5.0.99  | 4.0.99  | 5.4.0  | 5.1.0  | 5.1.0  | 5.0.0  | 4.10.1  | 4.10.0  | 4.9.0  |   |   |   |   |

| SPL-Response-Actions-Plugin  | 1.1.5  | 1.1.4  | 1.1.4  | 1.1.3  | 1.1.3  | 1.1.2  | 1.1.1  | 1.1.0  | 1.1.0  | 1.0.3  | 1.0.2  | 1.0.1  | 1.0.0  |   |   |   |   |   |   |   |   |   |   |   |   |   |

| SPL-Device-Isolation-Plugin  | 1.1.3  | 1.1.2  | 1.1.2  | 1.1.1  | 1.1.1  | 1.1.0  | 1.0.2  | 1.0.1  | 1.0.1  | 1.0.0  |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |

## Version 2026.2

### New features

#### Performance and resiliency improvements

- Extended kernel support: Added support for kernel 6.19 and 7.0.
- Performance and CPU improvements: Improved event processing for endpoints with heavy container activity.

#### Data lake hydration updates

- Data processing updates: Batching updates to increase alert fidelity.
- account_activity_linux updates: These updates include account creation, deletion, and modifications to group membership.

### Updated components

- Sophos Protection for Linux has been updated to 2026.2.
- SPL-Anti-Virus-Plugin has been updated to 1.2.1.
- SPL-Runtime-Detection-Plugin has been updated to 5.17.3.

## Version 2026.1

### New features

#### Extended attack surface coverage

- On-demand scanning improvements for non-root namespaces: The SPL Agent can now scan non-root namespaces, which are prevalent in containerized environments. This configuration is turned on by default.

#### Improved usability

- Expanded health monitoring accuracy and granularity: You can see more detailed health insights within Sophos Central.

#### Platform resiliency and stability

- This update includes performance optimizations and reduced third-party dependencies.
- Added support for kernel 6.18.

### Updated components

- Sophos Protection for Linux has been updated to 2026.1.
- SPL-Base-Component has been updated to 1.6.0.
- SPL-Endpoint-Detection-and-Response-Plugin has been updated to 1.2.1.
- SPL-Live-Response-Plugin has been updated to 1.7.9.
- SPL-Anti-Virus-Plugin has been updated to 1.2.0.
- SPL-Event-Journaler-Plugin has been updated to 1.1.4.
- SPL-Runtime-Detection-Plugin has been updated to 5.16.3.
- SPL-Response-Actions-Plugin has been updated to 1.1.4.
- SPL-Device-Isolation-Plugin has been updated to 1.1.2.

## Version 2025.2.2

### New features

There are no new features in this release.

### Updated components

- Sophos Protection for Linux has been updated to 2025.2.2.
- SPL-Runtime-Detection-Plugin has been updated to 5.15.1.

## Version 2025.2

### New features

#### Performance and resiliency improvements

- OSQuery optimizations: Updates to OSQuery that minimize memory footprint and reduce performance overhead.
- Plugin resource management: Implemented individual CPU and memory limits for plugins to improve system stability and minimize performance impacts across services.
- Service architecture improvements: Consolidated services to improve resource efficiency.
- Extended kernel support: Added support for kernel 6.16.

#### Data lake hydration updates

- Data processing updates: Batching, filtering, and aggregation updates to increase performance.
- Throughput updates: Improved event processing to handle higher volumes of data on active endpoints.

#### Configuration updates

- Linux Forensic SDU support: Incident Response teams can quickly gather information from the endpoint during an investigation.
- Expanding Linux update management: Added Linux support within Update Management policy for configuring content updates.

### Resolved issues

There are no resolved issues in this release.

### Updated components

- Sophos Protection for Linux has been updated to 2025.2.1.
- SPL-Base-Component has been updated to 1.5.0.
- SPL-Endpoint-Detection-and-Response-Plugin has been updated to 1.2.0.
- SPL-Live-Response-Plugin has been updated to 1.7.8.
- SPL-Anti-Virus-Plugin has been updated to 1.1.8.
- SPL-Event-Journaler-Plugin has been updated to 1.1.3.
- SPL-Runtime-Detection-Plugin has been updated to 5.14.0.
- SPL-Response-Actions-Plugin has been updated to 1.1.3.
- SPL-Device-Isolation-Plugin has been updated to 1.1.1.

## Version 2025.1

### New features

- Performance and resiliency improvements:

- Removed dependency on the audit netlink to improve performance. This also removes the requirement to turn off auditd when running the SPL Agent.
- Improvements to limit downtime for the Runtime Detection plugin in the event of content failure. When a rule fails, the Runtime Detection plugin continues to run and logs the failed rule.

- Configuration updates:

- Added the ability to configure event journal disk size for Linux devices in Sophos Central.

### Resolved issues

There are no resolved issues in this release.

### Updated components

- Sophos Protection for Linux has been updated to 2025.1.
- SPL-Base-Component has been updated to 1.4.0.
- SPL-Endpoint-Detection-and-Response-Plugin has been updated to 1.1.15.
- SPL-Live-Response-Plugin has been updated to 1.7.7.
- SPL-Anti-Virus-Plugin has been updated to 1.1.7.
- SPL-Event-Journaler-Plugin has been updated to 1.1.2.
- SPL-Runtime-Detection-Plugin has been updated to 5.10.0.
- SPL-Response-Actions-Plugin has been updated to 1.1.2.
- SPL-Device-Isolation-Plugin has been updated to 1.1.0.

## Version 2024.3

### New features

- On-access file scanning updates:

- The SPL Agent will attempt to end processes associated with an on-access threat detection.
- Added additional detection capabilities when accessing files in PrivateTmp.

- Performance and resiliency improvements:

- Added optimizations for memory and CPU resource limits to help minimize the SPL Agent's footprint on the server.

- Data lake hydration updates:

- Continued updates to reduce the time before an event is accessible within the Threat Analysis Center and Data Lake in Central.

- Installation updates:

- Updates to the thin installer allow you to override the device name for a host.

### Resolved issues

There are no resolved issues in this release.

### Updated components

- Sophos Protection for Linux has been updated to 2024.3.
- SPL-Base-Component has been updated to 1.3.0.
- SPL-Endpoint-Detection-and-Response-Plugin has been updated to 1.1.14.
- SPL-Live-Response-Plugin has been updated to 1.7.6.
- SPL-Anti-Virus-Plugin has been updated to 1.1.6.
- SPL-Event-Journaler-Plugin has been updated to 1.1.1.
- SPL-Runtime-Detection-Plugin has been updated to 5.9.3.
- SPL-Response-Actions-Plugin has been updated to 1.1.1.
- SPL-Device-Isolation-Plugin has been updated to 1.0.2.

## Version 2024.2.1

### New features

There are no new features for this release.

### Resolved issues

Resolved an issue affecting SPL package upgrades.

### Updated components

- Sophos Protection for Linux has been updated to 2024.2.1.
- SPL-Base-Component has been updated to 1.2.9

## Version 2024.2

### New features

- Data Lake hydration updates:

- Updates that reduce the amount of time between the SPL Agent recording an event on the Linux device and sending the event to the Data Lake.
- Data Lake query updates that include the addition of additional enrichment data.

- Performance and resiliency updates:

- Optimizations were made to the SPL Agent to reduce process memory consumption and disk space utilization.
- Improved updating stability when issues are encountered during the initial connection with Sophos Central.

### Updated components

- Sophos Protection for Linux has been updated to 2024.2.
- SPL-Base-Component has been updated to 1.2.8
- SPL-Endpoint-Detection-and-Response-Plugin has been updated to 1.1.13
- SPL-Live-Response-Plugin has been updated to 1.7.5
- SPL-Anti-Virus-Plugin has been updated to 1.1.5
- SPL-Event-Journaler-Plugin has been updated to 1.1.0
- SPL-Runtime-Detection-Plugin has been updated to 5.9.1
- SPL-Response-Actions-Plugin has been updated to 1.1.0
- SPL-Device-Isolation-Plugin has been updated to 1.0.1

## Version 2024.1

### New features

- Device Isolation:

- You can isolate Linux devices from the network.
- You can still manage the device from Sophos Central while in isolation.
- Uses the existing Sophos Central capabilities to isolate, remove from isolation, and apply exclusions.

- Security Health Status updates for Linux agent:

- Sophos Linux Runtime Detections service is now included as part of Security Health in Sophos Central.
- This may affect the security health state for Linux devices in Sophos Central. For more information, see [Sophos Protection for Linux: Runtime detection device health](https://support.sophos.com/support/s/article/KB-000045907).

- Allow list by path support for Linux:

- Provides users the option to allow applications by filename or path.
- You can configure it directly from the event details or at General Settings > Allowed Applications.

### Updated components

- Sophos Protection for Linux has been updated to 2024.1.
- SPL-Base-Component has been updated to 1.2.7
- SPL-Endpoint-Detection-and-Response-Plugin has been updated to 1.1.12
- SPL-Live-Response-Plugin has been updated to 1.7.4
- SPL-Anti-Virus-Plugin has been updated to 1.1.4
- SPL-Event-Journaler-Plugin has been updated to 1.0.8
- SPL-Runtime-Detection-Plugin has been updated to 5.8.0
- SPL-Response-Actions-Plugin has been updated to 1.0.3
- SPL-Device-Isolation-Plugin has been updated to 1.0.0

## Version 2023.4

### New features

- ARM64 Support:

- You can now install the SPL Agent on an ARM64-based Linux server. The system requirements have been updated to include ARM64 support.
- For more information, see [Linux Runtime Detections on ARM64 servers](https://support.sophos.com/support/s/article/KB-000045875).

- Installation updates:

- Updates to the installer to allow the designation of a custom install location for the SPL Agent.
- Options to run pre-installation checks without installing the SPL Agent. This can help verify the server and environment and help with troubleshooting.

### Updated components

- Sophos Protection for Linux has been updated to 2023.4.
- SPL-Base-Component has been updated to 1.2.6.
- SPL-Endpoint-Detection-and-Response-Plugin has been updated to 1.1.11.
- SPL-Live-Response-Plugin has been updated to 1.7.3.
- SPL-Anti-Virus-Plugin has been updated to 1.1.3.
- SPL-Event-Journaler-Plugin has been updated to 1.0.7.
- SPL-Runtime-Detection-Plugin has been updated to 5.7.0.
- SPL-Response-Actions-Plugin has been updated to 1.0.2.

## Version 2023.3

### New features

- Enterprise Software Management:

- This release introduces Enterprise Software Management support for the Sophos Protection for Linux Agent. This provides additional flexibility for customers to test and control updates across their estate.
- Enterprise Software Management is replacing Controlled Updates. You can still use Controlled Updates for Linux servers until they're turned off on January 31st, 2024.

- Installation updates:

- You can now configure a Message Relay and Update Cache during installation.
- Updates to capture additional logging details to help troubleshoot installation failures.

- On-access file scanning updates:

- You can now configure on-read or on-write scanning for the on-access scanner.

### Updated components

- Sophos Protection for Linux has been updated to 2023.3.
- Sophos Linux Base has been updated to 1.2.5.
- Sophos Live Discover Plugin has been updated to 1.1.10.
- Sophos Linux Live Response has been updated to 1.7.2.
- Server Protection has been updated to 1.1.2.
- Sophos Linux Event Journaler has been updated to 1.0.6.
- Sophos Linux Runtime Detection Plugin has been updated to 5.6.0.

## Version 2023.2

### New features

- PUA Detections: Provides capability for detection of Potentially Unwanted Applications (PUAs) for Linux. PUA detection requires no additional policy configuration and is automatically enabled as part of on-demand or on-access scanning on the Sophos Linux Agent.

### Updated components

- Sophos Linux Base has been updated to 1.2.4.
- Sophos Live Discover Plugin has been updated to 1.1.9.
- Sophos Linux Live Response has been updated to 1.7.1.
- Server Protection has been updated to 1.1.1.
- Sophos Linux Event Journaler has been updated to 1.0.5.
- Sophos Linux Runtime Detection Plugin has been updated to 5.3.0.

## Version 2023.1.3

### Updated components

- SPL-Runtime-Detection-Plugin has been updated to 5.0.99.

## Version 2023.1.2

### Updated components

- SPL-Base-Component has been updated to 1.2.3.
- SPL-Runtime-Detection-Plugin has been updated to 4.0.99.

## Version 2023.1

### New features

- On-Access File Scanning.

You can now configure on-access scanning for the Sophos Linux Agent on your Linux devices. You can turn on on-access scanning for the Sophos Linux Agent in your server threat protection policies in Sophos Central. On-access scanning is turned off by default. See [Server Threat Protection Policy](https://docs.sophos.com/central/customer/help/en-us/index.html?contextID=server-protection-group-threat).

- Malware Quarantine.

We've introduced quarantine for the Sophos Linux Agent. This applies to both on-access and on-demand scanning. This gives your Linux devices additional protection by automatically quarantining suspicious files. Quarantine is based on a signature match.

### Updated components

- SPL-Base-Component has been updated to 1.2.2.
- SPL-Anti-Virus-Plugin (Server Protection) has been updated to 1.1.0.
- SPL-Endpoint-Detection-and-Response-Plugin has been updated to 1.1.8.
- SPL-Live-Response-Plugin has been updated to 1.7.0.
- SPL-Event-Journaler-Plugin has been updated to 1.0.4.
- SPL-Runtime-Detection-Plugin has been updated to 5.4.0.

## Version 2022.4

### Updated components

- SPL-Base-Component (Sophos Linux Base) has been updated to 1.2.1.
- SPL-Anti-Virus-Plugin (Server Protection) has been updated to 1.0.8.

## Version 1.2.0

### Updated components

We've updated the names of our components.

Sophos Linux Base is now called SPL-Base-Component.

Server Protection is now called SPL-Anti-Virus-Plugin.

Sophos Live Discover plugin is now called SPL-Endpoint-Detection-and-Response-Plugin.

Sophos Linux Event Journaler is now called SPL-Event-Journaler-Plugin.

Sophos Linux Live Response is now called SPL-Live-Response-Plugin.

Sophos Linux Runtime Detection plugin is now called SPL-Runtime-Detection-Plugin.

- SPL-Base-Component (Sophos Linux Base) has been updated to 1.2.0.
- SPL-Endpoint-Detection-and-Response-Plugin (Sophos Live Discover plugin) has been updated to 1.1.7.
- SPL-Live-Response-Plugin (Sophos Linux Live Response) has been updated to 1.6.1.
- SPL-Anti-Virus-Plugin (Server Protection) has been updated to 1.0.7.
- SPL-Event-Journaler-Plugin (Sophos Linux Event Journaler) has been updated to 1.0.3.
- SPL-Runtime-Detection-Plugin (Sophos Linux Runtime Detection Plugin) has been updated to 5.1.0.

## Version 1.1.10

### New features

We now support Amazon Linux 2022, Ubuntu 22.04 (LTS), Oracle 8, Miracle Linux, Debian 10 and Debian 11. Earlier versions of Sophos Protection for Linux don't support these platforms.

### Updated components

- Sophos Linux Base has been updated to 1.1.10.
- Sophos Live Discover plugin has been updated to 1.1.6.
- Sophos Linux Live Response has been updated to 1.5.2.
- Server Protection has been updated to 1.0.6.
- Sophos Linux Event Journaler has been updated to 1.0.2.
- Sophos Linux Runtime Detection Plugin has been updated to 5.0.0.

## Version 1.1.9.1

### Updated components

- Sophos Linux Runtime Detection Plugin has been updated to 4.10.1.

## Version 1.1.9

### New features

- Event Journals now record runtime detections to make them available in Live Discover.
- Sophos Protection for Linux now reports endpoint health in Sophos Central.

### Updated components

- Sophos Linux Base has been updated to 1.1.9.
- Sophos Live Discover Plugin has been updated to 1.1.5.
- Sophos Linux Live Response has been updated to 1.5.1.
- Server Protection has been updated to 1.0.5.
- Sophos Linux Event Journaler has been updated to 1.0.1.
- Sophos Linux Runtime Detection Plugin has been updated to 4.10.0.

## Version 1.1.8

### New features

- We've added Event Journals to record AV detections to make them available in Live Discover and Sophos XDR.

### New components

- Sophos Linux Event Journaler is version 1.0.0.
- Sophos Linux Runtime Detection Plugin is version 4.9.0.

### Updated components

- Sophos Linux Base has been updated to 1.1.8.
- Sophos Live Discover Plugin has been updated to 1.1.4.
- Sophos Linux Live Response has been updated to 1.5.0.
- Server Protection has been updated to 1.0.4.

## Version 1.1.7.1

### Updated components

- Server Protection has been updated to 1.0.3.

## Version 1.1.7

### New features

We've added Server Protection, an on-demand antivirus scanner. This uses a threat detection engine, including machine learning models, alongside signature-based protection.

## Version 1.1.6

### New features

You can assign products using the command line at install time if you use thin installer version 1.1.1 and later versions.

## Version 1.1.5

### New features

This version of Sophos Protection for Linux supports the XDR Data Lake capabilities in Sophos Central. Additionally, this version supports installing directly to a Sophos Central Group if you use thin installer version 1.0.8 and later versions.

### Updated components

- Sophos Linux Base has been updated to 1.1.5.
- Sophos Live Discover plugin has been been updated to 1.1.1.
- Sophos Linux Live Response has been updated to 1.2.2.

## Version 1.1.4

### Updated components

- Sophos Linux Base has been updated to 1.1.4.
- Sophos Live Discover plugin has been been updated to 1.1.0.
- Sophos Linux Live Response has been updated to 1.2.1.

## Version 1.1.3

### New features

This version of Sophos Protection for Linux supports the EDR 3 capabilities in Sophos Central. Live Response allows admins to remotely connect to devices and get access to a command-line interface. This allows them to perform detailed investigations or to take prompt action to contain or remediate a threat.

## Version 1.1.1

### New features

This version of Sophos Protection for Linux supports the EDR 3 capabilities in Sophos Central. Live Discover allows admins to use osquery to search the device data across their estate to answer almost any question they can think of.

## Known issues

#### Host kernel panic with Linux kernel versions 5.10.133 - 5.10.142

Linux kernel versions 5.10.133 - 5.10.142 have an issue with ftrace that may cause the host system kernel to become unresponsive. This is a known issue with these kernel versions that affects our endpoint software because it utilizes ftrace functionality for various capabilities. When you install SLS on the affected kernel versions, RTD enters dormant mode. Additionally, other applications on the host that depend on ftrace functionality may be affected.

For more known issues, see [Sophos Linux Known Issues list](https://docs.sophos.com/support/kil/index.html?product=LINUX).

## System requirements

- Free disk space: 2.5 GB
- Free memory: 2 GB
- System type: x86_64 and ARM64
- Systemd supported and running
- Bash is installed
- glibc 2.17 or later
- ARM64:

- glibc 2.18 or later
- Minimum kernel of 5.3

## Note

We test the latest point release for platforms listed here.

## Supported platforms

### Tested platforms

Minimum kernel versions for x86_64 systems are noted where applicable.

- Amazon Linux 2
- Amazon Linux 2023
- CentOS Stream
- Debian 11
- Debian 12
- Debian 13
- Oracle 8
- Oracle 9
- Oracle 10
- RHEL 8
- RHEL 9
- RHEL 10
- SUSE Linux Enterprise Server 15
- Ubuntu 22.04 (LTS)
- Ubuntu 24.04 (LTS)
- Ubuntu 26.04

### Legacy platforms

The following legacy platforms include versions of platforms that have recently reached their end of mainstream support as published by the distribution vendor. For additional details on extended support for legacy versions, see [Sophos Central: Extended support](https://support.sophos.com/support/s/article/KBA-000007256). For additional details on support for legacy versions, see [Legacy versions](https://docs.sophos.com/esg/sls/help/en-us/index.html?contextID=KB0010065#legacy-versions).

Minimum kernel versions for x86_64 systems are noted where applicable.

- CentOS 7 (3.10.0-1062 or later)
- Debian 10
- Oracle 7 (UEK 4.14 or later, RHCK 3.10.0-1062 or later)
- RHEL 7 (3.10.0-1062 or later)
- SUSE Linux Enterprise Server 12
- Ubuntu 18.04 (LTS) (4.15 or later)
- Ubuntu 20.04 (LTS)

### Unlisted platforms

Sophos will investigate any issues by attempting to replicate them on a corresponding testing distribution. For more information related to platforms and versions not listed here, including platforms that have reached their end of mainstream support as published by the distribution vendor, such as downstream and upstream distributions and legacy platforms, see [Sophos Protection for Linux distribution and kernel support](https://docs.sophos.com/esg/spl/en-us/index.html?contextId=protect-devices-server-Linux-distro-support).

To gather more information on installation errors, see the following pages:

- [Installer command-line options for Linux](https://docs.sophos.com/central/customer/help/en-us/index.html?contextID=protect-devices-server-CLI-Linux)
- [Sophos Protection for Linux installation troubleshooting](https://docs.sophos.com/central/customer/help/en-us/index.html?contextID=protect-devices-server-Linux-agent-troubleshooting)

## Support

You can find technical support for Sophos products in any of these ways:

- Visit Sophos Community at [Sophos community](https://community.sophos.com) and search for other users who are experiencing the same problem.
- Visit Sophos support at [Sophos support](https://support.sophos.com/support/s/).
- Find how-to, configuration and troubleshooting videos at [Sophos Techvids video hub](https://techvids.sophos.com/).

## Legal notices

Copyright © 2023 Sophos Limited. All rights reserved. No part of this publication may be reproduced, stored in a retrieval system, or transmitted, in any form or by any means, electronic, mechanical, photocopying, recording or otherwise unless you are either a valid licensee where the documentation can be reproduced in accordance with the license terms or you otherwise have the prior permission in writing of the copyright owner.

Sophos and Sophos Anti-Virus are registered trademarks of Sophos Limited and Sophos Group. All other product and company names mentioned are trademarks or registered trademarks of their respective owners.
