---
url: "https://github.com/thinkst/opencanary"
title: "GitHub - thinkst/opencanary: Modular and decentralised honeypot"
site: "GitHub"
captured: "2026-11-02T19:08:38Z"
via: "live"
sha256: "1f62b2d43eee02e98cfa56b01e88f187e4285775f2d23776c43ed90d201fd45c"
wayback: "https://web.archive.org/web/20260901194516/https://github.com/thinkst/opencanary"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# GitHub - thinkst/opencanary: Modular and decentralised honeypot

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/thinkst/opencanary>
> Snapshot: <https://web.archive.org/web/20260901194516/https://github.com/thinkst/opencanary>

# OpenCanary by Thinkst Canary

https://github.com/thinkst/opencanary#opencanary-by-thinkst-canary

https://raw.githubusercontent.com/thinkst/opencanary/master/docs/logo.png OpenCanary is a multi-protocol network honeypot. It's primary use-case is to catch hackers after they've breached non-public networks. It has extremely low resource requirements and can be tweaked, modified, and extended.

https://github.com/thinkst/opencanary/actions/workflows/opencanary_tests.yml https://github.com/thinkst/opencanary/actions/workflows/docker-build.yml https://github.com/thinkst/opencanary/actions/workflows/publish.yml

## Overview

https://github.com/thinkst/opencanary#overview

OpenCanary runs as a daemon and implements multiple common network protocols. When attackers breach networks and interact with the honeypot, OpenCanary will send you alerts via a variety of mechanisms.

OpenCanary is implemented in Python, so the core honeypot is cross-platform; however, certain features require specific OSes. Running on Linux will give you the most options. It has extremely low resource requirements; for example, it can be deployed happily on a Raspberry Pi or a VM with minimal resources.

This README describes how to install and configure OpenCanary on Ubuntu Linux and MacOS.

OpenCanary is the Open Source version of our commercial [Thinkst Canary](https://canary.tools) honeypot.

## Table of Contents

https://github.com/thinkst/opencanary#table-of-contents

- [Prerequisites](https://github.com/thinkst/opencanary#prerequisites)
- [Features](https://github.com/thinkst/opencanary#features)
- [Installation](https://github.com/thinkst/opencanary#installation)

- [Installation on Ubuntu](https://github.com/thinkst/opencanary#installation-on-ubuntu)
- [Installation on macOS](https://github.com/thinkst/opencanary#installation-on-macos)
- [Installation via Git](https://github.com/thinkst/opencanary#installation-via-git)
- [Installation for Docker](https://github.com/thinkst/opencanary#installation-for-docker)

- [Configuring OpenCanary](https://github.com/thinkst/opencanary#configuring-opencanary)

- [Creating the initial configuration](https://github.com/thinkst/opencanary#creating-the-initial-configuration)
- [Enabling protocol modules and alerting](https://github.com/thinkst/opencanary#enabling-protocol-modules-and-alerting)
- [Optional modules](https://github.com/thinkst/opencanary#optional-modules)

- [SNMP](https://github.com/thinkst/opencanary#snmp)
- [Portscan](https://github.com/thinkst/opencanary#portscan)
- [Samba Setup](https://github.com/thinkst/opencanary#samba-setup)

- [Running OpenCanary](https://github.com/thinkst/opencanary#running-opencanary)

- [Directly on Linux or macOS](https://github.com/thinkst/opencanary#directly-on-linux-or-macos)
- [With docker compose](https://github.com/thinkst/opencanary#with-docker-compose)
- [With Docker](https://github.com/thinkst/opencanary#with-docker)

- [Documentation](https://github.com/thinkst/opencanary#documentation)
- [Project Participation](https://github.com/thinkst/opencanary#project-participation)

- [Contributing](https://github.com/thinkst/opencanary#contributing)
- [Security Vulnerability Reports](https://github.com/thinkst/opencanary#security-vulnerability-reports)
- [Bug reports](https://github.com/thinkst/opencanary#bug-reports)
- [Feature Requests](https://github.com/thinkst/opencanary#feature-requests)
- [Code of Conduct](https://github.com/thinkst/opencanary#code-of-conduct)

## Prerequisites

https://github.com/thinkst/opencanary#prerequisites

- AMD64: Python 3.10+
- ARM64: Python 3.10+
- Optional SNMP requires the Python library Scapy
- Optional Samba module needs a working installation of Samba
- Optional Portscan uses iptables (not nftables) and is only supported on Linux-based operating systems

## Features

https://github.com/thinkst/opencanary#features

- Mimic an array of network-accessible services for attackers to interact with.
- Receive various alerts as soon as potential threats are detected, highlighting the threat source IP address and where the breach may have occurred.

## Installation

https://github.com/thinkst/opencanary#installation

The OpenCanary installation essentially involves ensuring the Python environment is ready, then installing the OpenCanary Python package (plus optional extras).

If uv is installed, you can use it for virtual environment creation and package installation. If it is not installed, the standard python/pip flow below continues to work.

### Installation on Ubuntu

https://github.com/thinkst/opencanary#installation-on-ubuntu

Installation on Ubuntu 22.04 LTS or 24.04 LTS:

```
$ sudo apt-get install python3-dev python3-pip python3-virtualenv python3-venv python3-scapy libssl-dev libpcap-dev
$ virtualenv env/
$ . env/bin/activate
$ pip install opencanary

```

Optional uv equivalent:

```
$ uv venv env
$ . env/bin/activate
$ uv pip install opencanary

```

Optional extras (if you wish to use the Windows File Share module, and the SNMP module):

```
$ sudo apt install samba # if you plan to use the Windows File Share module
$ pip install scapy pcapy-ng # if you plan to use the SNMP module

```

### Installation on macOS

https://github.com/thinkst/opencanary#installation-on-macos

First, create and activate a new Python virtual environment:

```
$ virtualenv env/
$ . env/bin/activate

```

Optional uv equivalent:

```
$ uv venv env
$ . env/bin/activate

```

Macports users should then run:

```
$ sudo port install openssl
$ env ARCHFLAGS="-arch x86_64" LDFLAGS="-L/opt/local/lib" CFLAGS="-I/opt/local/include" pip install cryptography

```

Alternatively, Homebrew x86 users run:

```
$ brew install openssl
$ env ARCHFLAGS="-arch x86_64" LDFLAGS="-L/usr/local/opt/openssl/lib" CFLAGS="-I/usr/local/opt/openssl/include" pip install cryptography

```

Homebrew M1 users run:

```
$ brew install openssl
$ env ARCHFLAGS="-arch arm64" LDFLAGS="-L/opt/homebrew/opt/openssl@1.1/lib" CFLAGS="-I/opt/homebrew/opt/openssl@1.1/include" pip install cryptography

```

(The compilation step above is necessary as multiple OpenSSL versions may exist, which can confound the Python libraries.)

Now the installation can run as usual:

```
$ pip install opencanary
$ pip install scapy pcapy-ng # optional

```

With uv installed, the equivalent commands are:

```
$ uv pip install opencanary
$ uv pip install scapy pcapy-ng # optional

```

The Windows File Share (smb) module is not available on macOS.

### Installation via Git

https://github.com/thinkst/opencanary#installation-via-git

To install from source, instead of running pip do the following:

```
$ git clone https://github.com/thinkst/opencanary
$ cd opencanary
$ python setup.py sdist
$ cd dist
$ pip install opencanary-<version>.tar.gz

```

With uv installed, you can replace the final install step with:

```
$ uv pip install opencanary-<version>.tar.gz

```

### Use via pkgx

https://github.com/thinkst/opencanary#use-via-pkgx

OpenCanary is packaged via [pkgx](https://pkgx.sh/), so no installation is needed if pkgx is installed, simply preface the opencanaryd command with pkgx. Due to environment variable protections in modern sudo implementations, the entire command must be run as root, or via sudo -E.

```
$ pkgx opencanaryd --version

```

### Installation for Docker

https://github.com/thinkst/opencanary#installation-for-docker

OpenCanary Docker images are hosted on Docker Hub. These are only useful on Linux Docker hosts, as the host network engine is required for accurate network information.

## Configuring OpenCanary

https://github.com/thinkst/opencanary#configuring-opencanary

### Creating the initial configuration

https://github.com/thinkst/opencanary#creating-the-initial-configuration

When OpenCanary starts it looks for config files in the following locations and will stop when the first configuration is found:

- /etc/opencanaryd/opencanary.conf
- ~/.opencanary.conf (i.e. the home directory of the user, usually this will be root so /root/.opencanary.conf)
- ./opencanary.conf (i.e. the directory where OpenCanary is installed)

To create an initial configuration, run as root (you may be prompted for a sudo password):

```
$ opencanaryd --copyconfig
[*] A sample config file is ready /etc/opencanaryd/opencanary.conf

[*] Edit your configuration, then launch with "opencanaryd --start --uid=nobody --gid=nogroup"

```

This creates the path and file /etc/opencanaryd/opencanary.conf. You must now edit the config file to determine which services and logging options you want to enable.

Warning

The config file includes complex data, include Python objects for the logging configuration. The configuration is read while the process is running with root privileges. We recommend making the file root-owned and only writable by root. If regular users can write the file, it is possible to escalate privileges to root.

### Enabling protocol modules and alerting

https://github.com/thinkst/opencanary#enabling-protocol-modules-and-alerting

Configuration is performed via the JSON config file. Edit the file, and when happy save and exit.

### Optional modules

https://github.com/thinkst/opencanary#optional-modules

#### SNMP

https://github.com/thinkst/opencanary#snmp

The snmp module is only available when Scapy is present. See the installation steps for SNMP above.

#### Portscan

https://github.com/thinkst/opencanary#portscan

The portscan module is only available on Linux hosts, as it modifies iptables rules.

Please note that for the Portscan service, we have added a portscan.ignore_localhost setting, which means the OpenCanary portscan service will ignore (not alert on) port scans originating for the localhost IP (127.0.0.1). This setting is false by default.

#### Samba Setup

https://github.com/thinkst/opencanary#samba-setup

The Windows File Share module (smb) requires a Samba installation. See a step-by-step guide on [the Wiki](https://github.com/thinkst/opencanary/wiki/Opencanary-and-Samba).

## Running OpenCanary

https://github.com/thinkst/opencanary#running-opencanary

OpenCanary is either run directly on a Linux or macOS host, or via a Docker container.

### Directly on Linux or macOS

https://github.com/thinkst/opencanary#directly-on-linux-or-macos

Start OpenCanary by running:

```
$ . env/bin/activate
$ opencanaryd --start --uid=nobody --gid=nogroup

```

With the uid and gid flags, OpenCanary drops root privileges after binding to its ports. This can be changed to other low-privileged user/group or omitted to keep running with root privileges.

### With pkgx

https://github.com/thinkst/opencanary#with-pkgx

Start OpenCanary by running:

```
$ sudo -E pkgx opencanaryd --start --uid=nobody --gid=nogroup

```

With the uid and gid flags, OpenCanary drops root privileges after binding to its ports. This can be changed to other low-privileged user/group or omitted to keep running with root privileges.

### With docker compose

https://github.com/thinkst/opencanary#with-docker-compose

The route requires [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/) to be installed.

Note The portscan module is automatically disabled for Dockerised OpenCanary.

- Edit the data/.opencanary.conf file to enable, disable or customize the services that will run.
- Edit the ports section of the docker-compose.yml file to enable/disable the desired ports based on the services you have enabled in the config file.
- Run the container.

```
docker compose up latest
```

To view the logs run docker compose logs latest.

To stop the container run docker compose down.

To build your own Docker OpenCanary using docker compose, head over to our [wiki](https://github.com/thinkst/opencanary/wiki/Using-Dockerised-OpenCanary#building-and-running-your-own-docker-opencanary-image-with-docker-compose)

### With Docker

https://github.com/thinkst/opencanary#with-docker

Please head over our dedicated Docker [wiki](https://github.com/thinkst/opencanary/wiki/Using-Dockerised-OpenCanary#building-and-running-your-own-docker-opencanary-image-with-docker) for everything Dockerised OpenCanary.

### With Ansible

https://github.com/thinkst/opencanary#with-ansible

Please head over to our forked repository for an Ansible OpenCanary role over [here](https://github.com/thinkst/ansible-role-opencanary).

## Documentation

https://github.com/thinkst/opencanary#documentation

- The [Wiki](https://github.com/thinkst/opencanary/wiki) contains our FAQ.
- Additional documentation is available on our [main site](https://opencanary.org).

## Project Participation

https://github.com/thinkst/opencanary#project-participation

### Contributing

https://github.com/thinkst/opencanary#contributing

We welcome PRs to this project. Please read our [Code of Conduct](https://github.com/thinkst/.github/blob/master/CODE_OF_CONDUCT.md) and [Contributing](https://github.com/thinkst/.github/blob/master/CONTRIBUTING.md) documents before submitting a pull request.

At a minimum you should run pre-commit before submitting the PR. Install and run it in the same Python environment that OpenCanary is installed into:

```
$ pip install pre-commit
$ pre-commit install
# Do work
$ git add file
$ git commit
# If pre-commit generated errors, then run "git add ... && git commit" again

```

### Security Vulnerability Reports

https://github.com/thinkst/opencanary#security-vulnerability-reports

See our [Security Policy](https://github.com/thinkst/opencanary/security/policy) for details on how to report security vulnerabilities.

### Bug reports

https://github.com/thinkst/opencanary#bug-reports

Please file bug reports on [Github](https://github.com/thinkst/opencanary/issues) using the template we provide.

### Feature Requests

https://github.com/thinkst/opencanary#feature-requests

Feature requests are tracked [here](https://github.com/thinkst/opencanary/discussions/categories/feature-requests).

### Code of Conduct

https://github.com/thinkst/opencanary#code-of-conduct

This project and everyone participating in it is governed by the [Code of Conduct](https://github.com/thinkst/.github/blob/master/CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to github@thinkst.com.
