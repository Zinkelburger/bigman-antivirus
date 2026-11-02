---
url: "https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions"
title: "Qualys TRU Discovers Three Bypasses of Ubuntu Unprivileged User Namespace Restrictions | Qualys"
site: "Qualys"
published: "2025-03-27T17:10:29+00:00"
captured: "2026-11-02T18:24:03Z"
via: "live"
sha256: "b487feeb75a4ee064d6044ee5dbfaf2f90f36019a29e6459e5ec5368f0d1cfce"
wayback: "https://web.archive.org/web/20261102193132/https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Qualys TRU Discovers Three Bypasses of Ubuntu Unprivileged User Namespace Restrictions | Qualys

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions>
> Snapshot: <https://web.archive.org/web/20261102193132/https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions>

# Qualys TRU Discovers Three Bypasses of Ubuntu Unprivileged User Namespace Restrictions

[Saeed Abbasi](https://blog.qualys.com/author/sabbasi), Head of Threat Research Unit (TRU), Director of Product, Qualys

March 27, 2025 - 4 min read

- https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fblog.qualys.com%2Fvulnerabilities-threat-research%2F2025%2F03%2F27%2Fqualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions
- https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fblog.qualys.com%2Fvulnerabilities-threat-research%2F2025%2F03%2F27%2Fqualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions
- https://twitter.com/share?url=https%3A%2F%2Fblog.qualys.com%2Fvulnerabilities-threat-research%2F2025%2F03%2F27%2Fqualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions&text=Qualys%20TRU%20Discovers%20Three%20Bypasses%20of%20Ubuntu%20Unprivileged%20User%20Namespace%20Restrictions&via=qualys
-

#### Table of Contents

- [What are the User Namespace in Linux and Why are they important?](https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions#what-are-the-user-namespace-in-linux-and-why-are-they-important)
- [Affected Ubuntu versions:](https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions#affected-ubuntu-versions)
- [Technical Details](https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions#technical-details)
- [Leverage Qualys TruRisk Eliminate to Mitigate These Risks](https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions#leverage-qualys-trurisk-eliminate-to-mitigate-these-risks)
- [Conclusion](https://blog.qualys.com/vulnerabilities-threat-research/2025/03/27/qualys-tru-discovers-three-bypasses-of-ubuntu-unprivileged-user-namespace-restrictions#conclusion)

The [Qualys Threat Research Unit (TRU)](https://www.qualys.com/tru/) recently disclosed three security bypasses in Ubuntu’s unprivileged user namespace restrictions. Qualys responsibly disclosed these vulnerabilities to the Ubuntu Security Team on January 15, 2025, and has been working with Ubuntu since then.

Qualys TRU uncovered three distinct bypasses of these namespace restrictions, each enabling local attackers to create user namespaces with full administrative capabilities. These bypasses facilitate exploiting vulnerabilities in kernel components requiring powerful administrative privileges within a confined environment. The restrictions on unprivileged user namespaces were initially introduced in Ubuntu 23.10 and enabled by default in Ubuntu 24.04. It is important to note that these bypasses alone do not enable complete system takeover; however, they become dangerous when combined with other vulnerabilities, typically kernel-related.

## What are the User Namespace in Linux and Why are they important?

Most major Linux distributions permit unprivileged users to create namespaces in which they effectively gain full administrative rights. While beneficial for creating containers and sandboxes, this significantly expands the kernel’s attack surface. When new kernel vulnerabilities emerge, namespaces often make these exploitable by unprivileged users. Ubuntu is notably proactive in mitigating such risks by preventing unprivileged users from acquiring capabilities within namespaces.

## Affected Ubuntu versions:

The security bypasses affect Ubuntu version 24.04 and later. Ubuntu 23.10 introduces built-in unprivileged user namespace restrictions, though they are not enabled by default. These protections were introduced in this release, and users who have previously enabled and relied on them are affected.

## Technical Details

You can find the technical details of these security bypasses at:

https://www.qualys.com/2025/three-bypasses-of-Ubuntu-unprivileged-user-namespace-restrictions.txt

## Leverage Qualys TruRisk Eliminate to Mitigate These Risks

To help organizations address these risks quickly, customers leveraging the Qualys agent can use the TruRisk Eliminate module. This module is fully integrated with the VM module to efficiently assign all of the IG QIDs to the team responsible for those Ubuntu servers. It allows the team to test and deploy the mitigation directly from the Qualys console, leveraging the Qualys agent. There is nothing new to install.

The same researchers who uncovered these Ubuntu bypasses have proactively developed and thoroughly tested those mitigation scripts, ensuring organizations can rapidly and effectively neutralize this new threat.

If you are not subscribed to the TruRisk Eliminate module, you can visit [this page](https://www.qualys.com/forms/trurisk-eliminate/) to start a trial or ask your TAM to enable a trial for you.

Once the TruRisk Eliminate module is enabled, you can address this risk by visit VMDR and Vulnerabilities tab, and select all the vulns on the assets you would like to mitigate this vulnerability on and use Actions-> View Risk Eliminate:

Use the Mitigate Now button or multi-select vulnerability and select Actions -> Create Mitigation Job to start a mitigation job that will apply the mitigation to your assets.

Note: While this feature can rapidly reduce bypass risk, using it broadly may introduce operational risks or lead to undesired application behavior. We recommend thorough testing in a controlled environment to confirm compatibility and maintain system stability.

With [Qualys TruRisk Eliminate](https://www.qualys.com/apps/trurisk-eliminate/), security teams have an immediate, actionable path to risk reduction.

This scenario underscores why mitigation should be an essential component of any comprehensive cybersecurity strategy, as it provides a critical layer of defense precisely when patches are absent.

For more information and details on minimizing the attack surface for these flaws, please visit the Ubuntu’s post at: https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction

## Conclusion

Cybersecurity today requires more than reactive patching—it demands agility and the ability to quickly mitigate vulnerabilities, especially when traditional patching isn’t feasible. Organizations need proactive methods that allow them to neutralize threats rapidly and effectively, minimizing risk exposure even in complex scenarios. Approaches like virtual patching and real-time mitigations are becoming essential components of a robust cybersecurity strategy.

Explore how solutions such as [Qualys TruRisk Eliminate](https://www.qualys.com/apps/trurisk-eliminate/) can help organizations stay ahead of vulnerabilities, keeping critical infrastructure protected even when conventional patches aren’t available.

### Related
