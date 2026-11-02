---
url: "https://cisofy.com/lynis/"
title: "Lynis - Security auditing and hardening tool for Linux/Unix"
captured: "2026-11-02T18:25:17Z"
via: "live"
sha256: "0439f9d3d3a33123a6206cc79fc44a7e7574acc0c6f16f856967c637872396a1"
wayback: "https://web.archive.org/web/20261021103451/https://cisofy.com/lynis/"
cited_in:
  - "REVIEW.md"
  - "TODO.md"
  - "docs/research/linux-av-landscape-and-papers.md"
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# Lynis - Security auditing and hardening tool for Linux/Unix

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://cisofy.com/lynis/>
> Snapshot: <https://web.archive.org/web/20261021103451/https://cisofy.com/lynis/>

# Lynis

## Lynis, an introduction

### Auditing, system hardening, compliance testing

Lynis is a battle-tested security tool for systems running Linux, macOS, or Unix-based operating system. It performs an extensive health scan of your systems to support system hardening and compliance testing. The project is open source software with the GPL license and available since 2007.

Security scan with Lynis

### Project goals

Since Lynis is flexible, it is used for several different purposes. Typical use cases for Lynis include:

- Security auditing
- Compliance testing (e.g. PCI, HIPAA, SOx)
- Penetration testing
- Vulnerability detection
- System hardening

### Audience and use cases

- Developers: Test that Docker image, or improve the hardening of your deployed web application.
- System administrators: Run daily health scans to discover new weaknesses.
- IT auditors: Show colleagues or clients what can be done to improve security.
- Penetration testers: Discover security weaknesses on systems of your clients, that may eventually result in system compromise.

### Supported operating systems

Lynis runs on almost all UNIX-based systems and versions, including:

- AIX
- FreeBSD
- HP-UX
- Linux
- macOS
- NetBSD
- NixOS
- OpenBSD
- Solaris
- and others

It even runs on systems like the Raspberry Pi, IoT devices, and QNAP storage devices.

## How it works

Lynis scanning is modular and opportunistic. This means it will only use and test the components that it can find, such as the available system tools and its libraries. The benefit is that no installation of other tools is needed, so you can keep your systems clean.

By using this scanning method, the tool can run with almost no dependencies. Also, the more components it discovers, the more extensive the audit will be. In other words: Lynis will always perform scans that are tailored to your system. No audit will be the same!

Example: When Lynis detects that you are running Apache, it will perform an initial round of Apache related tests. Then when it performs the specific Apache tests, it may also discover a SSL/TLS configuration. It then performs additional auditing steps based on that. A good example is collecting any discovered certificates, so that they can be scanned later as well.

### Audit steps

This is what happens during a typical scan with Lynis:

- Initialization
- Perform basic checks, such as file ownership
- Determine operating system and tools
- Search for available software components
- Check latest Lynis version
- Run enabled plugins
- Run security tests per category
- Perform execution of your custom tests (optional)
- Report status of security scan

Besides the report and information displayed on screen, all technical details about the scan are stored in a log file (lynis.log). Findings like warnings and suggestions are stored in a separate report file (lynis-report.dat).

### Lynis tests (controls)

Lynis performs hundreds of individual tests. Each test will help to determine the security state of the system. Most tests are written in shell script and have a unique identifier (e.g. KRNL-6000).

Interested in learning more about the tests? Have a look at the [Lynis controls](https://cisofy.com/lynis/controls/) and individual tests.

#### Flexibility

With the unique identifiers it is possible to tune a security scan. For example, if a test is too strict for your scanning appetite, simply disable it. This way you get an optimal system audit for your environment.

Lynis is modular and allows to run your self-created tests. You can even create them in other scripting or programming languages.

### Lynis Plugins

Plugins are modular extensions to Lynis. With the help of the plugins, Lynis will perform additional tests and collect more system information.

Each plugin has the objective to collect specific data. This data is stored in the Lynis report file (lynis-report.dat). Depending on your usage of Lynis, the collected data might provide valuable insights between systems or between individual scans.

The plugins provide the most value in environments with more than 10 systems. Some plugins are available in the downloads section.

#### Extra plugins

As part of our [Lynis Enterprise](https://cisofy.com/lynis-enterprise/) offering, the core developers maintain a set of plugins for our customers. The data that is collected centrally (SaaS or self-hosted), provide additional insights, such as available users, processes, and network details. Another important area is compliance testing, where the data points help to test against common standards and hardening guides.

[Lynis plugins overview](https://cisofy.com/lynis/plugins/)

### Supported standards

Other tools typically use the same data files to perform tests. Lynis is not limited to a specific Linux distribution, therefore it uses the knowledge of 10+ years from a wide range of sources. It may help you to automate or test against security best practices from sources like:

- CIS benchmarks
- NIST
- NSA
- OpenSCAP data
- Vendor guides and recommendations (e.g. Debian Gentoo, Red Hat)

## Demo (in 30 seconds!)

Time is precious. So look how quick you can install Lynis and have it perform a security scan. That is hard to beat, right?

## Comparison with other tools

Lynis has a different way of doing things, so you gain more flexibility. After all, you should be the one deciding what security controls make sense for your environment. Here are some comparisons with some other well-known tools.

### Bastille Linux

Bastille was for a long time the best known utility for hardening Linux systems. It focuses mainly on automatically hardening the system.

#### Differences with Bastille

Automated hardening tools are helpful, but at the same time might give a false sense of security. Instead of just turning on some settings, Lynis perform an in-depth security scan. You are the one to decide what level of security is appropriate for your environment. After all, not all systems have to be like [Fort Knox](http://en.wikipedia.org/wiki/Fort_Knox), unless you want it to be.

#### Benefits of Lynis

- Supports more operating systems
- Won't break your system
- More in-depth audit

### OpenVAS and Nessus

These products focus primarily on vulnerability scanning. They do this via the network by searching for discoverable services. Optionally, they will log in to a system and gather data.

#### Differences with Nessus or OpenVAS

Lynis runs on the host itself. Therefore it can perform a deeper analysis compared with network-based scans. This means less risk to impact your business processes and log files remain clean from connection attempts and incorrect requests.

Although Lynis is an auditing tool, it will discover vulnerabilities as well. It does so by using existing tools and analyzing configuration files.

Lynis and OpenVAS are both open source and free to use. Nessus is proprietary software and only available as part of a commercial offering.

#### Benefits of Lynis

- Much faster
- No pollution of log files
- Much lower risk of disruption to business services
- Host-based scans provide a more in-depth audit

### Tiger

[Tiger](https://www.nongnu.org/tiger/) was one of the first tools for testing the security of Linux systems. It was created by CIS Network group of the A&M campus of the Texas University.

Lynis and Tiger are similar, with one big difference: Lynis is still maintained, Tiger is not.

#### Benefits of Lynis

- Maintained
- Supports newer technologies

## Installation

Lynis is light-weight and easy to use. Although most users will install Lynis via a package, installation is not required! Just extract the archive (tarball) and run ./lynis audit system

### Installation options

- Clone via GitHub
- Software package
- Tarball
- Homebrew
- Ports (BSD)

First time user to Lynis? We suggest to follow the [Get Started](https://cisofy.com/documentation/lynis/get-started/) guide.

## Download

### Download tarball

If you prefer to use a tarball to test and deploy, see details on the [download page](https://cisofy.com/downloads/lynis/)

### GitHub

The Lynis project can be easily cloned via [GitHub](https://github.com/CISOfy/Lynis)

### Install as package

Rather use a package to install? Most distributions already have a package version available. They might be outdated, so we provide the latest software packages via our [software repository](https://packages.cisofy.com).

## Background information

- » Open Source ([GPL](http://www.gnu.org/copyleft/gpl.html))
- » Free to use
- » Well-known and mature software
- » Big community of users
- » Packages and ports available
- » Used by individuals, businesses, government departments, and multinationals

## Upgrade to Lynis Enterprise

The Lynis Enterprise Suite uses Lynis as a core component. Lynis can run as a standalone security tool. When used together with the suite, it become a data collection client.

### Continuous auditing

Security is not a one-time event. For companies who want to do continuous auditing, we provide Lynis Enterprise. This security suite provides central management, plugins, reporting, hardening snippets, and more.

Read more about [Lynis Enterprise](https://cisofy.com/lynis-enterprise/) and discover all benefits.
