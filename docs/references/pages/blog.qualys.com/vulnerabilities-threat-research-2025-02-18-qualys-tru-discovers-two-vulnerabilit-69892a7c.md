---
url: "https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466"
title: "Qualys TRU Uncovers OpenSSH Vulnerabilities CVE‑2025‑26465 & CVE‑2025‑26466 | Qualys"
site: "Qualys"
published: "2025-02-18T09:04:45+00:00"
captured: "2026-11-02T18:23:03Z"
via: "live"
sha256: "ac8f786e02416f7b18a593503e5fda75af74481b5a8f7ec0ef0c73136f436c6e"
wayback: "https://web.archive.org/web/20260823071618/https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Qualys TRU Uncovers OpenSSH Vulnerabilities CVE‑2025‑26465 & CVE‑2025‑26466 | Qualys

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466>
> Snapshot: <https://web.archive.org/web/20260823071618/https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466>

# Qualys TRU Discovers Two Vulnerabilities in OpenSSH: CVE-2025-26465 & CVE-2025-26466

[Saeed Abbasi](https://blog.qualys.com/author/sabbasi), Head of Threat Research Unit (TRU), Director of Product, Qualys

February 24, 2026 - 7 min read

- https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fblog.qualys.com%2Fvulnerabilities-threat-research%2F2025%2F02%2F18%2Fqualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466
- https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fblog.qualys.com%2Fvulnerabilities-threat-research%2F2025%2F02%2F18%2Fqualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466
- https://twitter.com/share?url=https%3A%2F%2Fblog.qualys.com%2Fvulnerabilities-threat-research%2F2025%2F02%2F18%2Fqualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466&text=Qualys%20TRU%20Discovers%20Two%20Vulnerabilities%20in%20OpenSSH%3A%20CVE-2025-26465%20%26%20CVE-2025-26466&via=qualys
-

#### Table of Contents

- [About OpenSSH: Securing Enterprise Communications and Infrastructure](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#about-openssh-securing-enterprise-communications-and-infrastructure)
- [Affected OpenSSH versions:](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#affected-openssh-versions)
- [Potential Impact](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#potential-impact)
- [Technical Details](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#technical-details)
- [Qualys QID Coverage](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#qualys-qid-coverage)
- [Discover Vulnerable Assets Using Qualys CyberSecurity Asset Management (CSAM)](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#discover-vulnerable-assets-using-qualys-cybersecurity-asset-management-csam)
- [Enhance Your Security Posture with Qualys Vulnerability Management, Detection, and Response (VMDR)](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#enhance-your-security-posture-with-qualys-vulnerability-management-detection-and-response-vmdr)
- [Automatically Patch these vulnerabilities With Qualys Patch Management](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#automatically-patch-these-vulnerabilities-with-qualys-patch-management)
- [Detect and remediate CVE-2025-26466 and CVE-2025-26465 with Qualys TotalCloud Container Security](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#detect-and-remediate-cve-2025-26466-and-cve-2025-26465-with-qualys-totalcloud-container-security)
- [Conclusion](https://blog.qualys.com/vulnerabilities-threat-research/2025/02/18/qualys-tru-discovers-two-vulnerabilities-in-openssh-cve-2025-26465-cve-2025-26466#conclusion)

The [Qualys Threat Research Unit (TRU)](https://www.qualys.com/tru/) has identified two vulnerabilities in OpenSSH. The first, tracked as CVE-2025-26465, allows an active machine-in-the-middle attack on the OpenSSH client when the VerifyHostKeyDNS option is enabled. The second, CVE-2025-26466, affects both the OpenSSH client and server, enabling a pre-authentication denial-of-service attack.

The attack against the OpenSSH client (CVE-2025-26465) succeeds regardless of whether the VerifyHostKeyDNS option is set to “yes” or “ask” (its default is “no”), requires no user interaction, and does not depend on the existence of an SSHFP resource record (an SSH fingerprint) in DNS. VerifyHostKeyDNS is an OpenSSH client configuration option that lets the SSH client look up and verify a server’s host key using DNS records (specifically, SSHFP records). The vulnerability was introduced in December 2014, just before the release of OpenSSH 6.8p1. Although VerifyHostKeyDNS is disabled by default, it was enabled by default on FreeBSD from September 2013 until March 2023.

The OpenSSH client and server are vulnerable (CVE-2025-26466) to a pre-authentication denial-of-service attack–an asymmetric resource consumption of both memory and CPU–that was introduced in August 2023 (shortly before the release of OpenSSH 9.5p1). On the server side, this attack can be mitigated by leveraging existing mechanisms in OpenSSH, such as LoginGraceTime, MaxStartups, and the more recent PerSourcePenalties.

Recommended Action: OpenSSH 9.9p2 addresses these vulnerabilities mentioned above. To ensure continued security, we strongly advise upgrading affected systems to 9.9p2 as soon as possible.

## About OpenSSH: Securing Enterprise Communications and Infrastructure

OpenSSH is a free, open-source implementation of the Secure Shell (SSH) protocol that enables encrypted communications over insecure networks. Widely adopted across Unix-like systems (including Linux and macOS) and many modern operating systems, it replaces clear-text protocols such as Telnet and FTP by providing secure remote login, file transfers, port forwarding, and tunneling.

With robust encryption, privilege separation, sandboxing, and modern memory allocators, OpenSSH minimizes the risk of memory-related vulnerabilities and unauthorized access. Its enterprise-grade scalability supports automated processes, data backups, and complex DevOps workflows—all while enforcing strong access controls. Despite these two vulnerabilities, OpenSSH’s overall track record in maintaining confidentiality and integrity has made it a benchmark in software security, ensuring secure communications for organizations worldwide.

## Affected OpenSSH versions:

- OpenSSH versions from 6.8p1 through 9.9p1 are vulnerable to CVE-2025-26465, the flaw introduced in December 2014.
- OpenSSH versions 9.5p1 through 9.9p1 are vulnerable to CVE-2025-26466, the flaw introduced in August 2023.

OpenSSH 9.9p2 addresses the vulnerabilities mentioned above. Upgrade promptly to maintain security.

## Potential Impact

If an attacker can perform a man-in-the-middle attack via CVE-2025-26465, the client may accept the attacker’s key instead of the legitimate server’s key. This would break the integrity of the SSH connection, enabling potential interception or tampering with the session before the user even realizes it. SSH sessions can be a prime target for attackers aiming to intercept credentials or hijack sessions. If compromised, hackers could view or manipulate sensitive data, move across multiple critical servers laterally, and exfiltrate valuable information such as database credentials. Such breaches can lead to reputational damage, violate compliance mandates (e.g., GDPR, HIPAA, PCI-DSS), and potentially disrupt critical operations by forcing system downtime to contain the threat.

SSH is a critical service for remote system administration. If attackers can repeatedly exploit the flaw CVE-2025-26466, they may cause prolonged outages or prevent administrators from managing servers, effectively locking legitimate users out. An enterprise facing this vulnerability could see critical servers become unreachable, interrupting routine operations and stalling essential maintenance tasks.

When the Qualys research team confirmed the vulnerability, Qualys initiated a responsible disclosure process and worked with OpenSSH to coordinate its announcement.

## Technical Details

You can find the technical details of this vulnerability at:

https://www.qualys.com/2025/02/18/openssh-mitm-dos.txt

## Qualys QID Coverage

Qualys is releasing the QIDs in the table below as they become available. Please refer to the Qualys Vulnerability Knowledgebase for a complete overview of these vulnerabilities and their coverage.

| QID | Title

| 38968 |  OpenSSH Security Update (CVE-2025-26465)

| 517056 | Alpine Linux 3.21 Security Update for openssh

| 517055 | Alpine Linux 3.19 Security Update for openssh

| 517054 | Alpine Linux 3.18 Security Update for openssh

| 711065 | Gentoo Linux OpenSSH Multiple Vulnerabilities (GLSA 202502-01)

| 6021670 | Ubuntu Security Notification for OpenSSH Vulnerability (USN-7270-2)

| 6021667 | Ubuntu Security Notification for OpenSSH Vulnerabilities (USN-7270-1)

| 758129 | SUSE Enterprise Linux Security Update for openssh (SUSE-SU-2025:0585-1)

| 258351 | CentOS Stream Security update for openssh

## Discover Vulnerable Assets Using Qualys CyberSecurity Asset Management (CSAM)

The initial and crucial step in managing this critical vulnerability and mitigating associated risks involves pinpointing all assets susceptible to this specific issue. Use [CSAM 3.0 with External Attack Surface Management](https://www.qualys.com/apps/cybersecurity-asset-management/) to identify your organization’s internet-facing instances that have vulnerable versions of OpenSSH or are at their End of Life (EOL) or End of Support (EOS).

#### Free Trial

### Identify internet-facing instances with vulnerable versions of OpenSSH

[Click Here to Start](https://www.qualys.com/apps/external-attack-surface-management/)

In the following example, we aim to identify all assets running the OpenSSH for CVE-2025-26465 with the affected range: 6.8p1 through 9.9p1:

```
software:(name:"openssh" AND version>=6.8 AND version<=9.9)
```

In the following example, we aim to identify all assets running the OpenSSH for CVE-2025-26466 with the affected range: 9.5p1 through 9.9p1:

```
software:(name:"openssh" AND version>=9.5 AND version<=9.9)
```

## Enhance Your Security Posture with Qualys Vulnerability Management, Detection, and Response (VMDR)

[Qualys VMDR](https://www.qualys.com/apps/vulnerability-management-detection-response/) offers comprehensive coverage and visibility into vulnerabilities, empowering organizations to rapidly respond to, prioritize, and mitigate the associated risks. Additionally, Qualys customers can leverage [Qualys Patch Management](https://www.qualys.com/apps/patch-management/) to remediate these vulnerabilities effectively.

Leverage the power of Qualys VMDR alongside TruRisk and the Qualys Query Language (QQL) to efficiently identify and prioritize vulnerable assets, effectively addressing the vulnerabilities highlighted above.

#### Free Trial

### Try Qualys VMDR at no cost for 30 days

[Get the Free Trial](https://www.qualys.com/forms/vmdr/)

Use this QQL statement:

```
vulnerabilities.vulnerability.cveIds:CVE-2025-26465 or CVE-2025-26466
```

## Automatically Patch these vulnerabilities With Qualys Patch Management

We expect vendors to release patches for this vulnerability shortly. Qualys Patch Management can automatically deploy those patches to vulnerable assets, when available.

Customers can use the “patch now” button found to the right of the vulnerability to add these vulnerabilities to a patch job. Once patches are released, Qualys will find the relevant patches for these vulnerabilities and automatically add those patches to a patch job. This will allow customers to deploy those patches to vulnerable devices, all from the Qualys Cloud Platform.

#### Free Trial

### Qualys Patch Management No-Cost 45-Day Trial

[Get the Free Trial](https://www.qualys.com/forms/patch-management/)

## Detect and remediate CVE-2025-26466 and CVE-2025-26465 with Qualys TotalCloud Container Security

Qualys TotalCloud Container Security offers comprehensive coverage and visibility into vulnerabilities across all your container environments, including managed Kubernetes and on-premises Kubernetes. This empowers organizations to rapidly respond to, prioritize, and mitigate associated risks effectively.

Leverage the power of Qualys TotalCloud Container Security and the Qualys Query Language (QQL) to efficiently identify and prioritize vulnerable assets, ensuring prompt and effective remediation of the vulnerabilities highlighted by CVE-2025-26466 and CVE-2025-26465.

Use this QQL statement:

```
state:`RUNNING` and vulnerabilities.cveids:CVE-2025-26466 or CVE-2025-26465
```

## Conclusion

At Qualys, we strongly recommend that all customers and users upgrade to the latest version of OpenSSH to address potential security vulnerabilities. The OpenSSH project has a longstanding reputation for delivering secure remote access, and this new release underscores its commitment to protecting the user community. By upgrading, users gain access to critical security improvements and the most advanced features offered by the OpenSSH project. We want to thank the OpenSSH developers for their work on this release and their continued dedication to the OpenSSH project.

To stay up to date with our TRU team’s latest discoveries, [subscribe to our blog](https://blog.qualys.com/).

### Related
