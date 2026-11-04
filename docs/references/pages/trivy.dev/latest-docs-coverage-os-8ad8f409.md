---
url: "https://trivy.dev/latest/docs/coverage/os/"
title: "Trivy - Overview"
captured: "2026-11-02T18:46:51Z"
via: "live"
source_url: "https://trivy.dev/docs/latest/coverage/os/"
sha256: "d3f6e17ecda1703660b346954b762e520d6258ec91a2e5d5c559ee912813573e"
wayback: "https://web.archive.org/web/20251105223336/https://trivy.dev/latest/docs/coverage/os/"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# Trivy - Overview

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://trivy.dev/latest/docs/coverage/os/>
> Snapshot: <https://web.archive.org/web/20251105223336/https://trivy.dev/latest/docs/coverage/os/>

https://github.com/aquasecurity/trivy/blob/main/docs/coverage/os/index.md

# OS[¶](https://trivy.dev/docs/latest/coverage/os/#os)

## Scanner[¶](https://trivy.dev/docs/latest/coverage/os/#scanner)

Trivy supports operating systems for

- [SBOM](https://trivy.dev/docs/latest/supply-chain/sbom/)
- [Vulnerabilities](https://trivy.dev/docs/latest/scanner/vulnerability/)
- [Licenses](https://trivy.dev/docs/latest/scanner/license/)

## Supported OS[¶](https://trivy.dev/docs/latest/coverage/os/#supported-os)

| OS  | Supported Versions  | Package Managers

| [Alpine Linux](https://trivy.dev/docs/latest/coverage/os/alpine/)  | 2.2 - 2.7, 3.0 - 3.22, edge  | apk

| [Wolfi Linux](https://trivy.dev/docs/latest/coverage/os/wolfi/)  | (n/a)  | apk

| [Chainguard](https://trivy.dev/docs/latest/coverage/os/chainguard/)  | (n/a)  | apk

| [MinimOS](https://trivy.dev/docs/latest/coverage/os/minimos/)  | (n/a)  | apk

| [Red Hat Enterprise Linux](https://trivy.dev/docs/latest/coverage/os/rhel/)  | 6, 7, 8, 9  | dnf/yum/rpm

| [Red Hat Enterprise Linux](https://trivy.dev/docs/latest/coverage/os/rhel/)  | 10 (SBOM only)  | dnf/yum/rpm

| [CentOS](https://trivy.dev/docs/latest/coverage/os/centos/)[1](https://trivy.dev/docs/latest/coverage/os/#fn:1)  | 6, 7, 8  | dnf/yum/rpm

| [AlmaLinux](https://trivy.dev/docs/latest/coverage/os/alma/)  | 8, 9, 10  | dnf/yum/rpm

| [Rocky Linux](https://trivy.dev/docs/latest/coverage/os/rocky/)  | 8, 9  | dnf/yum/rpm

| [Oracle Linux](https://trivy.dev/docs/latest/coverage/os/oracle/)  | 5, 6, 7, 8  | dnf/yum/rpm

| [Azure Linux (CBL-Mariner)](https://trivy.dev/docs/latest/coverage/os/azure/)  | 1.0, 2.0, 3.0  | tdnf/dnf/yum/rpm

| [Amazon Linux](https://trivy.dev/docs/latest/coverage/os/amazon/)  | 1, 2, 2023  | dnf/yum/rpm

| [openSUSE Leap](https://trivy.dev/docs/latest/coverage/os/suse/)  | 42, 15  | zypper/rpm

| [openSUSE Tumbleweed](https://trivy.dev/docs/latest/coverage/os/suse/)  | (n/a)  | zypper/rpm

| [SUSE Linux Enterprise](https://trivy.dev/docs/latest/coverage/os/suse/)  | 11, 12, 15  | zypper/rpm

| [SUSE Linux Enterprise Micro](https://trivy.dev/docs/latest/coverage/os/suse/)  | 5, 6  | zypper/rpm

| [Photon OS](https://trivy.dev/docs/latest/coverage/os/photon/)  | 1.0, 2.0, 3.0, 4.0  | tndf/yum/rpm

| [CoreOS](https://trivy.dev/docs/latest/coverage/os/coreos/)[3](https://trivy.dev/docs/latest/coverage/os/#fn:3)  | All versions (SBOM only)  | rpm

| [Echo](https://trivy.dev/docs/latest/coverage/os/echo/)  | (n/a)  | apt/dpkg

| [Debian GNU/Linux](https://trivy.dev/docs/latest/coverage/os/debian/)  | 7, 8, 9, 10, 11, 12  | apt/dpkg

| [Ubuntu](https://trivy.dev/docs/latest/coverage/os/ubuntu/)  | All versions supported by Canonical  | apt/dpkg

| [Bottlerocket](https://trivy.dev/docs/latest/coverage/os/bottlerocket/)  | 1.7.0 and upper  | bottlerocket

| [OSs with installed Conda](https://trivy.dev/docs/latest/coverage/others/conda/)  | -  | conda

## Supported container images[¶](https://trivy.dev/docs/latest/coverage/os/#supported-container-images)

| Container image  | Supported Versions  | Package Managers

| [Google Distroless](https://trivy.dev/docs/latest/coverage/os/google-distroless/)[2](https://trivy.dev/docs/latest/coverage/os/#fn:2)  | Any  | apt/dpkg

| [Bitnami](https://trivy.dev/docs/latest/coverage/others/bitnami/)  | Any  | -

Each page gives more details.

-

CentOS Stream is not supported [↩](https://trivy.dev/docs/latest/coverage/os/#fnref:1)

-

https://github.com/GoogleContainerTools/distroless [↩](https://trivy.dev/docs/latest/coverage/os/#fnref:2)

-

Fedora CoreOS and the deprecated CoreOS Container Linux [↩](https://trivy.dev/docs/latest/coverage/os/#fnref:3)
