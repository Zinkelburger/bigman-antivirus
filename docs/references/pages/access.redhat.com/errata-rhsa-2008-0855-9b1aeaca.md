---
url: "https://access.redhat.com/errata/RHSA-2008:0855"
captured: "2026-11-02T18:21:38Z"
via: "live"
sha256: "f1e9eed757bd47546232d247df991ac91033d8b61fd88c5b721faafbac1087d6"
wayback: "https://web.archive.org/web/20260120033646/https://access.redhat.com/errata/RHSA-2008:0855"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# https://access.redhat.com/errata/RHSA-2008:0855

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://access.redhat.com/errata/RHSA-2008:0855>
> Snapshot: <https://web.archive.org/web/20260120033646/https://access.redhat.com/errata/RHSA-2008:0855>

[Red Hat Product Errata](https://access.redhat.com/errata) RHSA-2008:0855 - Security Advisory

Issued:

2008-08-22

Updated:

2008-08-22

# RHSA-2008:0855 - Security Advisory

- [Overview](https://access.redhat.com/errata/RHSA-2008:0855#overview)
- [Updated Packages](https://access.redhat.com/errata/RHSA-2008:0855#packages)

## Synopsis

Critical: openssh security update

## Type/Severity

Security Advisory: Critical

## Red Hat Lightspeed patch analysis

Identify and remediate systems affected by this advisory.

[View affected systems](https://console.redhat.com/insights/patch/advisories/RHSA-2008:0855)

## Topic

Updated openssh packages are now available for Red Hat Enterprise Linux 4,
Red Hat Enterprise Linux 5, and Red Hat Enterprise Linux 4.5 Extended
Update Support.

This update has been rated as having critical security impact by the Red
Hat Security Response Team.

## Description

OpenSSH is OpenBSD's SSH (Secure SHell) protocol implementation.

Last week Red Hat detected an intrusion on certain of its computer systems
and took immediate action. While the investigation into the intrusion is
on-going, our initial focus was to review and test the distribution
channel we use with our customers, Red Hat Network (RHN) and its associated
security measures. Based on these efforts, we remain highly confident that
our systems and processes prevented the intrusion from compromising RHN or
the content distributed via RHN and accordingly believe that customers who
keep their systems updated using Red Hat Network are not at risk. We are
issuing this alert primarily for those who may obtain Red Hat binary
packages via channels other than those of official Red Hat subscribers.

In connection with the incident, the intruder was able to sign a small
number of OpenSSH packages relating only to Red Hat Enterprise Linux 4
(i386 and x86_64 architectures only) and Red Hat Enterprise Linux 5 (x86_64
architecture only). As a precautionary measure, we are releasing an
updated version of these packages, and have published a list of the
tampered packages and how to detect them at
http://www.redhat.com/security/data/openssh-blacklist.html

To reiterate, our processes and efforts to date indicate that packages
obtained by Red Hat Enterprise Linux subscribers via Red Hat Network are
not at risk.

These packages also fix a low severity flaw in the way ssh handles X11
cookies when creating X11 forwarding connections. When ssh was unable to
create untrusted cookie, ssh used a trusted cookie instead, possibly
allowing the administrative user of a untrusted remote server, or untrusted
application run on the remote server, to gain unintended access to a users
local X server. (CVE-2007-4752)

## Solution

Before applying this update, make sure that all previously-released
errata relevant to your system have been applied.

This update is available via Red Hat Network. Details on how to use
the Red Hat Network to apply this update are available at
http://kbase.redhat.com/faq/FAQ_58_10188

## Affected Products

-  Red Hat Enterprise Linux Server 5 x86_64
-  Red Hat Enterprise Linux Server 5 ia64
-  Red Hat Enterprise Linux Server 5 i386
-  Red Hat Enterprise Linux Server 4 x86_64
-  Red Hat Enterprise Linux Server 4 ia64
-  Red Hat Enterprise Linux Server 4 i386
-  Red Hat Enterprise Linux for x86_64 - Extended Update Support 5.2 x86_64
-  Red Hat Enterprise Linux for x86_64 - Extended Update Support 5.2 ia64
-  Red Hat Enterprise Linux for x86_64 - Extended Update Support 5.2 i386
-  Red Hat Enterprise Linux for x86_64 - Extended Update Support 4.7 x86_64
-  Red Hat Enterprise Linux for x86_64 - Extended Update Support 4.7 ia64
-  Red Hat Enterprise Linux for x86_64 - Extended Update Support 4.7 i386
-  Red Hat Enterprise Linux for x86_64 - Extended Update Support 4.5 x86_64
-  Red Hat Enterprise Linux for x86_64 - Extended Update Support 4.5 ia64
-  Red Hat Enterprise Linux for x86_64 - Extended Update Support 4.5 i386
-  Red Hat Enterprise Linux Workstation 5 x86_64
-  Red Hat Enterprise Linux Workstation 5 i386
-  Red Hat Enterprise Linux Workstation 4 x86_64
-  Red Hat Enterprise Linux Workstation 4 ia64
-  Red Hat Enterprise Linux Workstation 4 i386
-  Red Hat Enterprise Linux Desktop 5 x86_64
-  Red Hat Enterprise Linux Desktop 5 i386
-  Red Hat Enterprise Linux Desktop 4 x86_64
-  Red Hat Enterprise Linux Desktop 4 i386
-  Red Hat Enterprise Linux for IBM z Systems 5 s390x
-  Red Hat Enterprise Linux for IBM z Systems 4 s390x
-  Red Hat Enterprise Linux for IBM z Systems 4 s390
-  Red Hat Enterprise Linux for IBM z Systems - Extended Update Support 5.2 s390x
-  Red Hat Enterprise Linux for IBM z Systems - Extended Update Support 4.7 s390x
-  Red Hat Enterprise Linux for IBM z Systems - Extended Update Support 4.7 s390
-  Red Hat Enterprise Linux for IBM z Systems - Extended Update Support 4.5 s390x
-  Red Hat Enterprise Linux for IBM z Systems - Extended Update Support 4.5 s390
-  Red Hat Enterprise Linux for Power, big endian 5 ppc
-  Red Hat Enterprise Linux for Power, big endian 4 ppc
-  Red Hat Enterprise Linux for Power, big endian - Extended Update Support 5.2 ppc
-  Red Hat Enterprise Linux for Power, big endian - Extended Update Support 4.7 ppc
-  Red Hat Enterprise Linux for Power, big endian - Extended Update Support 4.5 ppc
-  Red Hat Enterprise Linux Server from RHUI 5 x86_64
-  Red Hat Enterprise Linux Server from RHUI 5 i386

## Fixes

-  [BZ - 280361](https://bugzilla.redhat.com/bugzilla/show_bug.cgi?id=280361) - CVE-2007-4752 openssh falls back to the trusted x11 cookie if generation of an untrusted cookie fails

## CVEs

- [CVE-2007-4752](https://www.redhat.com/security/data/cve/CVE-2007-4752.html)
- [CVE-2008-3844](https://www.redhat.com/security/data/cve/CVE-2008-3844.html)

## References

- http://www.redhat.com/security/data/openssh-blacklist.html
- http://www.redhat.com/security/updates/classification/#critical

Note:  More recent versions of these packages may be available. Click a package name for more details.

## Red Hat Enterprise Linux Server 5

| SRPM

|  openssh-4.3p2-26.el5_2.1.src.rpm   | SHA-256: 6a9394285a823cb267126b4041d24884a71596647e03a901a9d4771204799e76

| x86_64

|  openssh-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 8fbedda1925480ff13666ff999533109d12ef26d2ebc677e1da31feeea830d71

|  openssh-askpass-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 8607e2c5d3bfb5da8be52b04e7b6277a4319cb401e747934ff0bdcea255f8a09

|  openssh-clients-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 4062cbc8c7006f66c82733ba714cdd1af7ef81ce14de7a515446e98135c09981

|  openssh-server-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: b11a3050f4bdac012c489795b8c05db257d1ab3c858716e1948b2feae32b46b4

| ia64

|  openssh-4.3p2-26.el5_2.1.ia64.rpm   | SHA-256: 564d8becf2b7eae4d8d92cfd8ca2a5c8757719ef96c666b1dfa87fe863ebfb05

|  openssh-askpass-4.3p2-26.el5_2.1.ia64.rpm   | SHA-256: 8a817ffe63d9439aed5d3701f9fb256bcb88252190758845f3dc01e4a445249a

|  openssh-clients-4.3p2-26.el5_2.1.ia64.rpm   | SHA-256: 565a98e09ceba2f9eef777ed40617e4021474f26f177cec1175d3a57c0c27119

|  openssh-server-4.3p2-26.el5_2.1.ia64.rpm   | SHA-256: 7510d00a30edfc7e41ec434dc0d03349f3e87e66e769e9638b935fb1520e3260

| i386

|  openssh-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: c44fd38b192662b54e8c34d00b7a5ff40f413012b99dfeaf2edb2144e6ea5b6d

|  openssh-askpass-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: e108aef71cfc7b40f5842a068a1eb95340d6e52c96dbcb0ab44402fe7b365948

|  openssh-clients-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: 59839abed8ced3a0f0dd6aa435f854005b602e1225cdc2646428d5d3f28f73ec

|  openssh-server-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: 9083281b23ac21a67dd6e5e70946ca432e5d77aaa27c5827b40f2cd371f41827

## Red Hat Enterprise Linux Server 4

| SRPM

|  openssh-3.9p1-11.el4_7.src.rpm   | SHA-256: 16171470c8f44b6bf2cf684eb4dd4fd2a6361ad74ac32d7d7db206732e0b31ac

| x86_64

|  openssh-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 0adce7940aa0524d97e512d651e0267eda565fcfdb1f8a8db2edf9481b471d6a

|  openssh-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 0adce7940aa0524d97e512d651e0267eda565fcfdb1f8a8db2edf9481b471d6a

|  openssh-askpass-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: e193e3e3b60aa405e1cb8dfec7149ed3fd73a44679d3697b9d2ea71a5ba17b2a

|  openssh-askpass-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: e193e3e3b60aa405e1cb8dfec7149ed3fd73a44679d3697b9d2ea71a5ba17b2a

|  openssh-askpass-gnome-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 54e736532404918fd822e901cb95ee33e75fabdaa467c89cd8a3a6841937e5d8

|  openssh-askpass-gnome-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 54e736532404918fd822e901cb95ee33e75fabdaa467c89cd8a3a6841937e5d8

|  openssh-clients-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 4c8f833314aea81b1c940e0e766ae659025944ea751aa8ae644eae3a3edd8e52

|  openssh-clients-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 4c8f833314aea81b1c940e0e766ae659025944ea751aa8ae644eae3a3edd8e52

|  openssh-server-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 22130a5ce7124210f6b0c5a733a3db7638320e191c4d81039c8b08559aa321a4

|  openssh-server-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 22130a5ce7124210f6b0c5a733a3db7638320e191c4d81039c8b08559aa321a4

| ia64

|  openssh-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 85d4bb3d016403700663062aeb6dee116779c2a45c068dabddd326204b280a49

|  openssh-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 85d4bb3d016403700663062aeb6dee116779c2a45c068dabddd326204b280a49

|  openssh-askpass-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 1352d23aef0f9e4e8a9ca890461f10b6f736d362ebbf2a30d4bc27025bbe4393

|  openssh-askpass-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 1352d23aef0f9e4e8a9ca890461f10b6f736d362ebbf2a30d4bc27025bbe4393

|  openssh-askpass-gnome-3.9p1-11.el4_7.ia64.rpm   | SHA-256: a8a04bb1f98ede9a50d5f5a6749fdb32087493cbc3c2dcc4473bdcb90b6e1749

|  openssh-askpass-gnome-3.9p1-11.el4_7.ia64.rpm   | SHA-256: a8a04bb1f98ede9a50d5f5a6749fdb32087493cbc3c2dcc4473bdcb90b6e1749

|  openssh-clients-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 0d1e8382b1f3ecc3fead4630a5e0f210e86b5d0c260ec7881fa6cda6880398ca

|  openssh-clients-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 0d1e8382b1f3ecc3fead4630a5e0f210e86b5d0c260ec7881fa6cda6880398ca

|  openssh-server-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 5d08a074c6e61b7542f2642afcdff4f4af1db0a1d2b9137fd0c7e33629f63c74

|  openssh-server-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 5d08a074c6e61b7542f2642afcdff4f4af1db0a1d2b9137fd0c7e33629f63c74

| i386

|  openssh-3.9p1-11.el4_7.i386.rpm   | SHA-256: d08a06fdad88755fa734aa66a8ba572c3e5ef3d199f05d34d4dbcc65d68aea5d

|  openssh-3.9p1-11.el4_7.i386.rpm   | SHA-256: d08a06fdad88755fa734aa66a8ba572c3e5ef3d199f05d34d4dbcc65d68aea5d

|  openssh-askpass-3.9p1-11.el4_7.i386.rpm   | SHA-256: d0deb9e7d7168e5bca6ff5c762c5ac4db81167eb7ad7d27ddb3df85b098e954d

|  openssh-askpass-3.9p1-11.el4_7.i386.rpm   | SHA-256: d0deb9e7d7168e5bca6ff5c762c5ac4db81167eb7ad7d27ddb3df85b098e954d

|  openssh-askpass-gnome-3.9p1-11.el4_7.i386.rpm   | SHA-256: eed68469a2b4d4e27cfe9affc96e9faf8d52e86c70051af49782000084741c07

|  openssh-askpass-gnome-3.9p1-11.el4_7.i386.rpm   | SHA-256: eed68469a2b4d4e27cfe9affc96e9faf8d52e86c70051af49782000084741c07

|  openssh-clients-3.9p1-11.el4_7.i386.rpm   | SHA-256: 8e0e5ea2f808a333dc2d4c67bd1f76d9a58908a14405d6b2a774f4f2bcade70a

|  openssh-clients-3.9p1-11.el4_7.i386.rpm   | SHA-256: 8e0e5ea2f808a333dc2d4c67bd1f76d9a58908a14405d6b2a774f4f2bcade70a

|  openssh-server-3.9p1-11.el4_7.i386.rpm   | SHA-256: 6427890aa94aebd41a518014bed5a36fa6086fb147609321a1c034516df4babb

|  openssh-server-3.9p1-11.el4_7.i386.rpm   | SHA-256: 6427890aa94aebd41a518014bed5a36fa6086fb147609321a1c034516df4babb

## Red Hat Enterprise Linux for x86_64 - Extended Update Support 5.2

| SRPM

| x86_64

| ia64

| i386

## Red Hat Enterprise Linux for x86_64 - Extended Update Support 4.7

| SRPM

|  openssh-3.9p1-11.el4_7.src.rpm   | SHA-256: 16171470c8f44b6bf2cf684eb4dd4fd2a6361ad74ac32d7d7db206732e0b31ac

| x86_64

|  openssh-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 0adce7940aa0524d97e512d651e0267eda565fcfdb1f8a8db2edf9481b471d6a

|  openssh-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 0adce7940aa0524d97e512d651e0267eda565fcfdb1f8a8db2edf9481b471d6a

|  openssh-askpass-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: e193e3e3b60aa405e1cb8dfec7149ed3fd73a44679d3697b9d2ea71a5ba17b2a

|  openssh-askpass-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: e193e3e3b60aa405e1cb8dfec7149ed3fd73a44679d3697b9d2ea71a5ba17b2a

|  openssh-askpass-gnome-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 54e736532404918fd822e901cb95ee33e75fabdaa467c89cd8a3a6841937e5d8

|  openssh-askpass-gnome-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 54e736532404918fd822e901cb95ee33e75fabdaa467c89cd8a3a6841937e5d8

|  openssh-clients-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 4c8f833314aea81b1c940e0e766ae659025944ea751aa8ae644eae3a3edd8e52

|  openssh-clients-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 4c8f833314aea81b1c940e0e766ae659025944ea751aa8ae644eae3a3edd8e52

|  openssh-server-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 22130a5ce7124210f6b0c5a733a3db7638320e191c4d81039c8b08559aa321a4

|  openssh-server-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 22130a5ce7124210f6b0c5a733a3db7638320e191c4d81039c8b08559aa321a4

| ia64

|  openssh-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 85d4bb3d016403700663062aeb6dee116779c2a45c068dabddd326204b280a49

|  openssh-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 85d4bb3d016403700663062aeb6dee116779c2a45c068dabddd326204b280a49

|  openssh-askpass-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 1352d23aef0f9e4e8a9ca890461f10b6f736d362ebbf2a30d4bc27025bbe4393

|  openssh-askpass-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 1352d23aef0f9e4e8a9ca890461f10b6f736d362ebbf2a30d4bc27025bbe4393

|  openssh-askpass-gnome-3.9p1-11.el4_7.ia64.rpm   | SHA-256: a8a04bb1f98ede9a50d5f5a6749fdb32087493cbc3c2dcc4473bdcb90b6e1749

|  openssh-askpass-gnome-3.9p1-11.el4_7.ia64.rpm   | SHA-256: a8a04bb1f98ede9a50d5f5a6749fdb32087493cbc3c2dcc4473bdcb90b6e1749

|  openssh-clients-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 0d1e8382b1f3ecc3fead4630a5e0f210e86b5d0c260ec7881fa6cda6880398ca

|  openssh-clients-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 0d1e8382b1f3ecc3fead4630a5e0f210e86b5d0c260ec7881fa6cda6880398ca

|  openssh-server-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 5d08a074c6e61b7542f2642afcdff4f4af1db0a1d2b9137fd0c7e33629f63c74

|  openssh-server-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 5d08a074c6e61b7542f2642afcdff4f4af1db0a1d2b9137fd0c7e33629f63c74

| i386

|  openssh-3.9p1-11.el4_7.i386.rpm   | SHA-256: d08a06fdad88755fa734aa66a8ba572c3e5ef3d199f05d34d4dbcc65d68aea5d

|  openssh-3.9p1-11.el4_7.i386.rpm   | SHA-256: d08a06fdad88755fa734aa66a8ba572c3e5ef3d199f05d34d4dbcc65d68aea5d

|  openssh-askpass-3.9p1-11.el4_7.i386.rpm   | SHA-256: d0deb9e7d7168e5bca6ff5c762c5ac4db81167eb7ad7d27ddb3df85b098e954d

|  openssh-askpass-3.9p1-11.el4_7.i386.rpm   | SHA-256: d0deb9e7d7168e5bca6ff5c762c5ac4db81167eb7ad7d27ddb3df85b098e954d

|  openssh-askpass-gnome-3.9p1-11.el4_7.i386.rpm   | SHA-256: eed68469a2b4d4e27cfe9affc96e9faf8d52e86c70051af49782000084741c07

|  openssh-askpass-gnome-3.9p1-11.el4_7.i386.rpm   | SHA-256: eed68469a2b4d4e27cfe9affc96e9faf8d52e86c70051af49782000084741c07

|  openssh-clients-3.9p1-11.el4_7.i386.rpm   | SHA-256: 8e0e5ea2f808a333dc2d4c67bd1f76d9a58908a14405d6b2a774f4f2bcade70a

|  openssh-clients-3.9p1-11.el4_7.i386.rpm   | SHA-256: 8e0e5ea2f808a333dc2d4c67bd1f76d9a58908a14405d6b2a774f4f2bcade70a

|  openssh-server-3.9p1-11.el4_7.i386.rpm   | SHA-256: 6427890aa94aebd41a518014bed5a36fa6086fb147609321a1c034516df4babb

|  openssh-server-3.9p1-11.el4_7.i386.rpm   | SHA-256: 6427890aa94aebd41a518014bed5a36fa6086fb147609321a1c034516df4babb

## Red Hat Enterprise Linux for x86_64 - Extended Update Support 4.5

| SRPM

|  openssh-3.9p1-10.RHEL4.20.src.rpm   | SHA-256: b65dce92ff1ae5207a5d86a14d3f34e014e7f217eb0bf3150113dd39f26a2c08

| x86_64

|  openssh-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: 4ee4cedb6c7c460fc73c7c7f456a5fbc0ab6d818e5af9a745762372a74db0114

|  openssh-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: 4ee4cedb6c7c460fc73c7c7f456a5fbc0ab6d818e5af9a745762372a74db0114

|  openssh-askpass-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: fcf2f0b0c05a40c2f1c7f18f5d198cfb7df4b3fb588e7e83bcd77dd4b0cd945e

|  openssh-askpass-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: fcf2f0b0c05a40c2f1c7f18f5d198cfb7df4b3fb588e7e83bcd77dd4b0cd945e

|  openssh-askpass-gnome-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: 303ab18c64ddd7e5e554b55166f165df846836a0e02401b5c61b3ca1df02e7ff

|  openssh-askpass-gnome-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: 303ab18c64ddd7e5e554b55166f165df846836a0e02401b5c61b3ca1df02e7ff

|  openssh-clients-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: 45accbcae0fb1bb73cd38c624515dda74a9842d061b5ee7a0b7f1f2169a52b3f

|  openssh-clients-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: 45accbcae0fb1bb73cd38c624515dda74a9842d061b5ee7a0b7f1f2169a52b3f

|  openssh-server-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: 46010e89550a7ef21dd949453074f57cb2be8958e083c0ee53822da3d086c1a4

|  openssh-server-3.9p1-10.RHEL4.20.x86_64.rpm   | SHA-256: 46010e89550a7ef21dd949453074f57cb2be8958e083c0ee53822da3d086c1a4

| ia64

|  openssh-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: f3e6ca78d6e7db573453530ff2eaf185700c47b6237ba77a1f8597c8e8f3c959

|  openssh-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: f3e6ca78d6e7db573453530ff2eaf185700c47b6237ba77a1f8597c8e8f3c959

|  openssh-askpass-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: 2a026747e76d30a3d6e8e1fccba4bc53c8ba1f7903beb21009dfd4ad8fc6902d

|  openssh-askpass-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: 2a026747e76d30a3d6e8e1fccba4bc53c8ba1f7903beb21009dfd4ad8fc6902d

|  openssh-askpass-gnome-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: de1054901d96832c0282452d71f198d6e434ae5c07b98031d56ca52c6aa78201

|  openssh-askpass-gnome-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: de1054901d96832c0282452d71f198d6e434ae5c07b98031d56ca52c6aa78201

|  openssh-clients-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: 9565e8760ea5a0dc05ec3d082c6e20a02c158dcd14c3fb4c9e96c08dc38ad8c9

|  openssh-clients-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: 9565e8760ea5a0dc05ec3d082c6e20a02c158dcd14c3fb4c9e96c08dc38ad8c9

|  openssh-server-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: b3d1bc0ddc492d2b0367bd89d789fc9ebc9b4b1497e90bcee25ea5548187bff0

|  openssh-server-3.9p1-10.RHEL4.20.ia64.rpm   | SHA-256: b3d1bc0ddc492d2b0367bd89d789fc9ebc9b4b1497e90bcee25ea5548187bff0

| i386

|  openssh-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: 31656caa7d00f65674decc4f98cc5de73987e56885d043386689e3264058d867

|  openssh-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: 31656caa7d00f65674decc4f98cc5de73987e56885d043386689e3264058d867

|  openssh-askpass-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: 56a9721503787fe6e5fa07ca89ce7b23031e58d26c55c5ec52e88d2af61275bd

|  openssh-askpass-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: 56a9721503787fe6e5fa07ca89ce7b23031e58d26c55c5ec52e88d2af61275bd

|  openssh-askpass-gnome-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: 47227fcf6baf00d4c0b48547b87dc468dc6a666238e1993f7713b581547e1d29

|  openssh-askpass-gnome-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: 47227fcf6baf00d4c0b48547b87dc468dc6a666238e1993f7713b581547e1d29

|  openssh-clients-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: efb76efd1acf5df5bc435d5568a5668310e386d0dbe3640a05a20a56c068f98a

|  openssh-clients-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: efb76efd1acf5df5bc435d5568a5668310e386d0dbe3640a05a20a56c068f98a

|  openssh-server-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: fae9bcedd71ec6fef55dc55cf3e675521e53cf06bf37ef270f0b418cbcdbd9d5

|  openssh-server-3.9p1-10.RHEL4.20.i386.rpm   | SHA-256: fae9bcedd71ec6fef55dc55cf3e675521e53cf06bf37ef270f0b418cbcdbd9d5

## Red Hat Enterprise Linux Workstation 5

| SRPM

|  openssh-4.3p2-26.el5_2.1.src.rpm   | SHA-256: 6a9394285a823cb267126b4041d24884a71596647e03a901a9d4771204799e76

| x86_64

|  openssh-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 8fbedda1925480ff13666ff999533109d12ef26d2ebc677e1da31feeea830d71

|  openssh-askpass-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 8607e2c5d3bfb5da8be52b04e7b6277a4319cb401e747934ff0bdcea255f8a09

|  openssh-clients-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 4062cbc8c7006f66c82733ba714cdd1af7ef81ce14de7a515446e98135c09981

|  openssh-server-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: b11a3050f4bdac012c489795b8c05db257d1ab3c858716e1948b2feae32b46b4

| i386

|  openssh-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: c44fd38b192662b54e8c34d00b7a5ff40f413012b99dfeaf2edb2144e6ea5b6d

|  openssh-askpass-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: e108aef71cfc7b40f5842a068a1eb95340d6e52c96dbcb0ab44402fe7b365948

|  openssh-clients-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: 59839abed8ced3a0f0dd6aa435f854005b602e1225cdc2646428d5d3f28f73ec

|  openssh-server-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: 9083281b23ac21a67dd6e5e70946ca432e5d77aaa27c5827b40f2cd371f41827

## Red Hat Enterprise Linux Workstation 4

| SRPM

|  openssh-3.9p1-11.el4_7.src.rpm   | SHA-256: 16171470c8f44b6bf2cf684eb4dd4fd2a6361ad74ac32d7d7db206732e0b31ac

| x86_64

|  openssh-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 0adce7940aa0524d97e512d651e0267eda565fcfdb1f8a8db2edf9481b471d6a

|  openssh-askpass-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: e193e3e3b60aa405e1cb8dfec7149ed3fd73a44679d3697b9d2ea71a5ba17b2a

|  openssh-askpass-gnome-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 54e736532404918fd822e901cb95ee33e75fabdaa467c89cd8a3a6841937e5d8

|  openssh-clients-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 4c8f833314aea81b1c940e0e766ae659025944ea751aa8ae644eae3a3edd8e52

|  openssh-server-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 22130a5ce7124210f6b0c5a733a3db7638320e191c4d81039c8b08559aa321a4

| ia64

|  openssh-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 85d4bb3d016403700663062aeb6dee116779c2a45c068dabddd326204b280a49

|  openssh-askpass-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 1352d23aef0f9e4e8a9ca890461f10b6f736d362ebbf2a30d4bc27025bbe4393

|  openssh-askpass-gnome-3.9p1-11.el4_7.ia64.rpm   | SHA-256: a8a04bb1f98ede9a50d5f5a6749fdb32087493cbc3c2dcc4473bdcb90b6e1749

|  openssh-clients-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 0d1e8382b1f3ecc3fead4630a5e0f210e86b5d0c260ec7881fa6cda6880398ca

|  openssh-server-3.9p1-11.el4_7.ia64.rpm   | SHA-256: 5d08a074c6e61b7542f2642afcdff4f4af1db0a1d2b9137fd0c7e33629f63c74

| i386

|  openssh-3.9p1-11.el4_7.i386.rpm   | SHA-256: d08a06fdad88755fa734aa66a8ba572c3e5ef3d199f05d34d4dbcc65d68aea5d

|  openssh-askpass-3.9p1-11.el4_7.i386.rpm   | SHA-256: d0deb9e7d7168e5bca6ff5c762c5ac4db81167eb7ad7d27ddb3df85b098e954d

|  openssh-askpass-gnome-3.9p1-11.el4_7.i386.rpm   | SHA-256: eed68469a2b4d4e27cfe9affc96e9faf8d52e86c70051af49782000084741c07

|  openssh-clients-3.9p1-11.el4_7.i386.rpm   | SHA-256: 8e0e5ea2f808a333dc2d4c67bd1f76d9a58908a14405d6b2a774f4f2bcade70a

|  openssh-server-3.9p1-11.el4_7.i386.rpm   | SHA-256: 6427890aa94aebd41a518014bed5a36fa6086fb147609321a1c034516df4babb

## Red Hat Enterprise Linux Desktop 5

| SRPM

|  openssh-4.3p2-26.el5_2.1.src.rpm   | SHA-256: 6a9394285a823cb267126b4041d24884a71596647e03a901a9d4771204799e76

| x86_64

|  openssh-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 8fbedda1925480ff13666ff999533109d12ef26d2ebc677e1da31feeea830d71

|  openssh-askpass-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 8607e2c5d3bfb5da8be52b04e7b6277a4319cb401e747934ff0bdcea255f8a09

|  openssh-clients-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 4062cbc8c7006f66c82733ba714cdd1af7ef81ce14de7a515446e98135c09981

|  openssh-server-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: b11a3050f4bdac012c489795b8c05db257d1ab3c858716e1948b2feae32b46b4

| i386

|  openssh-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: c44fd38b192662b54e8c34d00b7a5ff40f413012b99dfeaf2edb2144e6ea5b6d

|  openssh-askpass-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: e108aef71cfc7b40f5842a068a1eb95340d6e52c96dbcb0ab44402fe7b365948

|  openssh-clients-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: 59839abed8ced3a0f0dd6aa435f854005b602e1225cdc2646428d5d3f28f73ec

|  openssh-server-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: 9083281b23ac21a67dd6e5e70946ca432e5d77aaa27c5827b40f2cd371f41827

## Red Hat Enterprise Linux Desktop 4

| SRPM

|  openssh-3.9p1-11.el4_7.src.rpm   | SHA-256: 16171470c8f44b6bf2cf684eb4dd4fd2a6361ad74ac32d7d7db206732e0b31ac

| x86_64

|  openssh-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 0adce7940aa0524d97e512d651e0267eda565fcfdb1f8a8db2edf9481b471d6a

|  openssh-askpass-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: e193e3e3b60aa405e1cb8dfec7149ed3fd73a44679d3697b9d2ea71a5ba17b2a

|  openssh-askpass-gnome-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 54e736532404918fd822e901cb95ee33e75fabdaa467c89cd8a3a6841937e5d8

|  openssh-clients-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 4c8f833314aea81b1c940e0e766ae659025944ea751aa8ae644eae3a3edd8e52

|  openssh-server-3.9p1-11.el4_7.x86_64.rpm   | SHA-256: 22130a5ce7124210f6b0c5a733a3db7638320e191c4d81039c8b08559aa321a4

| i386

|  openssh-3.9p1-11.el4_7.i386.rpm   | SHA-256: d08a06fdad88755fa734aa66a8ba572c3e5ef3d199f05d34d4dbcc65d68aea5d

|  openssh-askpass-3.9p1-11.el4_7.i386.rpm   | SHA-256: d0deb9e7d7168e5bca6ff5c762c5ac4db81167eb7ad7d27ddb3df85b098e954d

|  openssh-askpass-gnome-3.9p1-11.el4_7.i386.rpm   | SHA-256: eed68469a2b4d4e27cfe9affc96e9faf8d52e86c70051af49782000084741c07

|  openssh-clients-3.9p1-11.el4_7.i386.rpm   | SHA-256: 8e0e5ea2f808a333dc2d4c67bd1f76d9a58908a14405d6b2a774f4f2bcade70a

|  openssh-server-3.9p1-11.el4_7.i386.rpm   | SHA-256: 6427890aa94aebd41a518014bed5a36fa6086fb147609321a1c034516df4babb

## Red Hat Enterprise Linux for IBM z Systems 5

| SRPM

|  openssh-4.3p2-26.el5_2.1.src.rpm   | SHA-256: 6a9394285a823cb267126b4041d24884a71596647e03a901a9d4771204799e76

| s390x

|  openssh-4.3p2-26.el5_2.1.s390x.rpm   | SHA-256: 01b0c2087f401cb033fe84edb0dc322176e0fc401c3528b12810c8d0543113ba

|  openssh-askpass-4.3p2-26.el5_2.1.s390x.rpm   | SHA-256: bc95f3280a512d61534059056e95e45e9700ed29ff39b9206e8dd57b3bffc074

|  openssh-clients-4.3p2-26.el5_2.1.s390x.rpm   | SHA-256: 09ca4fdd0a76d3c3f9719e0c48d4133bc676cd58aa9f5fc5c4a6e670c896b454

|  openssh-server-4.3p2-26.el5_2.1.s390x.rpm   | SHA-256: 478261f5476f59679d0b0da358af43fd2ad9fc25b3beabd23bf8cc62cea97929

## Red Hat Enterprise Linux for IBM z Systems 4

| SRPM

|  openssh-3.9p1-11.el4_7.src.rpm   | SHA-256: 16171470c8f44b6bf2cf684eb4dd4fd2a6361ad74ac32d7d7db206732e0b31ac

| s390x

|  openssh-3.9p1-11.el4_7.s390x.rpm   | SHA-256: 70b02d0fd6ae427b919956dad569e2bfb41adbb10734eb626de0dbfb71126199

|  openssh-askpass-3.9p1-11.el4_7.s390x.rpm   | SHA-256: f9873e4d85c83b0d29a5c10eb6436edf36c870e64cb0220084f31017cd8b07eb

|  openssh-askpass-gnome-3.9p1-11.el4_7.s390x.rpm   | SHA-256: 01ef2f2457cae95e2fdd78ac47004b150055eb97679a57a6420fa1a65b5c2ba7

|  openssh-clients-3.9p1-11.el4_7.s390x.rpm   | SHA-256: 972b60a95e71bf24f284a1d2e68a81aa7179df0e5c2abfa2c43b71465206637b

|  openssh-server-3.9p1-11.el4_7.s390x.rpm   | SHA-256: 5c8d2eff4345b83f967155a31cfaaa3abf43c3e9db74deca1b409532cc8275bb

| s390

|  openssh-3.9p1-11.el4_7.s390.rpm   | SHA-256: 5ee947d4c47b60158ce9cdc68dea6a41b59dfefa9b5e27b66f8bdd65f0e54180

|  openssh-askpass-3.9p1-11.el4_7.s390.rpm   | SHA-256: bf21536d2d1f2dfbffb0e8d3dccdaf7dacef2c15cb4970c7c937627da073a840

|  openssh-askpass-gnome-3.9p1-11.el4_7.s390.rpm   | SHA-256: 23f01da2441d49e85a6de25f0782ca451dc702f401d4b4302f511a7fda902a94

|  openssh-clients-3.9p1-11.el4_7.s390.rpm   | SHA-256: 5eeb51ff36b5f9f8138d894524cda978886588ce5a889bfbeb605acc162d0bb0

|  openssh-server-3.9p1-11.el4_7.s390.rpm   | SHA-256: 9d96e795e17e0e6793004fc8d3ea4f4fca77af28338b95af2712bc941d0e3044

## Red Hat Enterprise Linux for IBM z Systems - Extended Update Support 5.2

| SRPM

| s390x

## Red Hat Enterprise Linux for IBM z Systems - Extended Update Support 4.7

| SRPM

|  openssh-3.9p1-11.el4_7.src.rpm   | SHA-256: 16171470c8f44b6bf2cf684eb4dd4fd2a6361ad74ac32d7d7db206732e0b31ac

| s390x

|  openssh-3.9p1-11.el4_7.s390x.rpm   | SHA-256: 70b02d0fd6ae427b919956dad569e2bfb41adbb10734eb626de0dbfb71126199

|  openssh-askpass-3.9p1-11.el4_7.s390x.rpm   | SHA-256: f9873e4d85c83b0d29a5c10eb6436edf36c870e64cb0220084f31017cd8b07eb

|  openssh-askpass-gnome-3.9p1-11.el4_7.s390x.rpm   | SHA-256: 01ef2f2457cae95e2fdd78ac47004b150055eb97679a57a6420fa1a65b5c2ba7

|  openssh-clients-3.9p1-11.el4_7.s390x.rpm   | SHA-256: 972b60a95e71bf24f284a1d2e68a81aa7179df0e5c2abfa2c43b71465206637b

|  openssh-server-3.9p1-11.el4_7.s390x.rpm   | SHA-256: 5c8d2eff4345b83f967155a31cfaaa3abf43c3e9db74deca1b409532cc8275bb

| s390

|  openssh-3.9p1-11.el4_7.s390.rpm   | SHA-256: 5ee947d4c47b60158ce9cdc68dea6a41b59dfefa9b5e27b66f8bdd65f0e54180

|  openssh-askpass-3.9p1-11.el4_7.s390.rpm   | SHA-256: bf21536d2d1f2dfbffb0e8d3dccdaf7dacef2c15cb4970c7c937627da073a840

|  openssh-askpass-gnome-3.9p1-11.el4_7.s390.rpm   | SHA-256: 23f01da2441d49e85a6de25f0782ca451dc702f401d4b4302f511a7fda902a94

|  openssh-clients-3.9p1-11.el4_7.s390.rpm   | SHA-256: 5eeb51ff36b5f9f8138d894524cda978886588ce5a889bfbeb605acc162d0bb0

|  openssh-server-3.9p1-11.el4_7.s390.rpm   | SHA-256: 9d96e795e17e0e6793004fc8d3ea4f4fca77af28338b95af2712bc941d0e3044

## Red Hat Enterprise Linux for IBM z Systems - Extended Update Support 4.5

| SRPM

|  openssh-3.9p1-10.RHEL4.20.src.rpm   | SHA-256: b65dce92ff1ae5207a5d86a14d3f34e014e7f217eb0bf3150113dd39f26a2c08

| s390x

|  openssh-3.9p1-10.RHEL4.20.s390x.rpm   | SHA-256: 42b81a964f418d26c353b8bbf19074a0b604a696ee40edead2768b039734decf

|  openssh-askpass-3.9p1-10.RHEL4.20.s390x.rpm   | SHA-256: d112fe8338a00257fe22634e9a7f2667a4f7e9794923de566642dfa06ac1aec4

|  openssh-askpass-gnome-3.9p1-10.RHEL4.20.s390x.rpm   | SHA-256: 4d2c4644b38e832e03c37a092821787f515a1499c09736334557b6333ff76817

|  openssh-clients-3.9p1-10.RHEL4.20.s390x.rpm   | SHA-256: 829e151a3387c10256e21817901b46a8cd04d93dfd034edc4fe1484c88d16038

|  openssh-server-3.9p1-10.RHEL4.20.s390x.rpm   | SHA-256: 077a32b8453fdfb4b535c092845e1f3a3dc4f8c71d84c246c107e76461095ba0

| s390

|  openssh-3.9p1-10.RHEL4.20.s390.rpm   | SHA-256: 61140542204210d5774e81a95b6ec54e319a4cd7e335816a865c0e45a29ff564

|  openssh-askpass-3.9p1-10.RHEL4.20.s390.rpm   | SHA-256: a1e0f20ac63d50e93d362ce74a8970c562d4b712f575fad3e0f168f4e7ab248c

|  openssh-askpass-gnome-3.9p1-10.RHEL4.20.s390.rpm   | SHA-256: e72fd54fd38a58a0562e76d5ca64ae589b0ebb14a66741fe99582fedeb10cdee

|  openssh-clients-3.9p1-10.RHEL4.20.s390.rpm   | SHA-256: 0b7f53c60fc242fbe56ff95d329c4bf363129a5af2e3a8c88b67265fd39d26e0

|  openssh-server-3.9p1-10.RHEL4.20.s390.rpm   | SHA-256: 74ef946639bcb50cdb67a2eca77d153c76009a50e8c24e0d6adf8df60133f974

## Red Hat Enterprise Linux for Power, big endian 5

| SRPM

|  openssh-4.3p2-26.el5_2.1.src.rpm   | SHA-256: 6a9394285a823cb267126b4041d24884a71596647e03a901a9d4771204799e76

| ppc

|  openssh-4.3p2-26.el5_2.1.ppc.rpm   | SHA-256: 057687411d79a415bef056be25d35ae166ae32f9326c5efa527cfe755bbac42a

|  openssh-askpass-4.3p2-26.el5_2.1.ppc.rpm   | SHA-256: 5f5801b7c112dc7baf601ad496b164c468f4b895cc7035642dfeeb2b2c65d899

|  openssh-clients-4.3p2-26.el5_2.1.ppc.rpm   | SHA-256: f2ff2a46cb28ecdcf0548ac5f84198d627b3b98595214276946f486233eed8c5

|  openssh-server-4.3p2-26.el5_2.1.ppc.rpm   | SHA-256: d3e59c0946425d455778d45d3cc3a22a6dc5e2577e5fdfcc39e7e7c9b5dbad78

## Red Hat Enterprise Linux for Power, big endian 4

| SRPM

|  openssh-3.9p1-11.el4_7.src.rpm   | SHA-256: 16171470c8f44b6bf2cf684eb4dd4fd2a6361ad74ac32d7d7db206732e0b31ac

| ppc

|  openssh-3.9p1-11.el4_7.ppc.rpm   | SHA-256: 18ad9f691c3fc7a93e78010a9299a2d622ad4a7a0140f998921ed441ef18148b

|  openssh-askpass-3.9p1-11.el4_7.ppc.rpm   | SHA-256: f2eca521616053abf58f9f970c2eb8a9055e8d47b605f60bca5178af5ecdb667

|  openssh-askpass-gnome-3.9p1-11.el4_7.ppc.rpm   | SHA-256: d7b66a079e4bc660703ea2936909fe7c2ddeb96feec89ca0031b2a60bf6705e8

|  openssh-clients-3.9p1-11.el4_7.ppc.rpm   | SHA-256: 271162d7e7d474160f53c2ac3048155fff1fb6ef9bded97252e4be9640f2e225

|  openssh-server-3.9p1-11.el4_7.ppc.rpm   | SHA-256: 38ff69803b679c6553ad03757e79ea0c023e3233367a4093f2370af3fba32948

## Red Hat Enterprise Linux for Power, big endian - Extended Update Support 5.2

| SRPM

| ppc

## Red Hat Enterprise Linux for Power, big endian - Extended Update Support 4.7

| SRPM

|  openssh-3.9p1-11.el4_7.src.rpm   | SHA-256: 16171470c8f44b6bf2cf684eb4dd4fd2a6361ad74ac32d7d7db206732e0b31ac

| ppc

|  openssh-3.9p1-11.el4_7.ppc.rpm   | SHA-256: 18ad9f691c3fc7a93e78010a9299a2d622ad4a7a0140f998921ed441ef18148b

|  openssh-askpass-3.9p1-11.el4_7.ppc.rpm   | SHA-256: f2eca521616053abf58f9f970c2eb8a9055e8d47b605f60bca5178af5ecdb667

|  openssh-askpass-gnome-3.9p1-11.el4_7.ppc.rpm   | SHA-256: d7b66a079e4bc660703ea2936909fe7c2ddeb96feec89ca0031b2a60bf6705e8

|  openssh-clients-3.9p1-11.el4_7.ppc.rpm   | SHA-256: 271162d7e7d474160f53c2ac3048155fff1fb6ef9bded97252e4be9640f2e225

|  openssh-server-3.9p1-11.el4_7.ppc.rpm   | SHA-256: 38ff69803b679c6553ad03757e79ea0c023e3233367a4093f2370af3fba32948

## Red Hat Enterprise Linux for Power, big endian - Extended Update Support 4.5

| SRPM

|  openssh-3.9p1-10.RHEL4.20.src.rpm   | SHA-256: b65dce92ff1ae5207a5d86a14d3f34e014e7f217eb0bf3150113dd39f26a2c08

| ppc

|  openssh-3.9p1-10.RHEL4.20.ppc.rpm   | SHA-256: 370b96e537100396598476fbf7069b5269aff022ef40b2e5cd3611b8ffd173db

|  openssh-askpass-3.9p1-10.RHEL4.20.ppc.rpm   | SHA-256: 71d8739b5078e10a90544b480e32a715afe3e3e96ee38e880e60551f5f2dd02b

|  openssh-askpass-gnome-3.9p1-10.RHEL4.20.ppc.rpm   | SHA-256: 5fd0795c195a8c3f623552afaf616227c544f8a001a09f215918c60bef60a4ad

|  openssh-clients-3.9p1-10.RHEL4.20.ppc.rpm   | SHA-256: 333f65768a17cf17faaf2f7ccca4c8c5784f10b4a3dea78a7166b9d6f69c73b0

|  openssh-server-3.9p1-10.RHEL4.20.ppc.rpm   | SHA-256: 3cc36e0863c10080a07815e501fb168bba068d2e8c41cfb408e4af90523cbfea

## Red Hat Enterprise Linux Server from RHUI 5

| SRPM

|  openssh-4.3p2-26.el5_2.1.src.rpm   | SHA-256: 6a9394285a823cb267126b4041d24884a71596647e03a901a9d4771204799e76

| x86_64

|  openssh-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 8fbedda1925480ff13666ff999533109d12ef26d2ebc677e1da31feeea830d71

|  openssh-askpass-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 8607e2c5d3bfb5da8be52b04e7b6277a4319cb401e747934ff0bdcea255f8a09

|  openssh-clients-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: 4062cbc8c7006f66c82733ba714cdd1af7ef81ce14de7a515446e98135c09981

|  openssh-server-4.3p2-26.el5_2.1.x86_64.rpm   | SHA-256: b11a3050f4bdac012c489795b8c05db257d1ab3c858716e1948b2feae32b46b4

| i386

|  openssh-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: c44fd38b192662b54e8c34d00b7a5ff40f413012b99dfeaf2edb2144e6ea5b6d

|  openssh-askpass-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: e108aef71cfc7b40f5842a068a1eb95340d6e52c96dbcb0ab44402fe7b365948

|  openssh-clients-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: 59839abed8ced3a0f0dd6aa435f854005b602e1225cdc2646428d5d3f28f73ec

|  openssh-server-4.3p2-26.el5_2.1.i386.rpm   | SHA-256: 9083281b23ac21a67dd6e5e70946ca432e5d77aaa27c5827b40f2cd371f41827

The Red Hat security contact is secalert@redhat.com. More contact details at https://access.redhat.com/security/team/contact/.
