---
url: "https://access.redhat.com/security/updates/backporting"
title: "Security Backporting Practice - Red Hat Customer Portal"
site: "Red Hat Customer Portal"
captured: "2026-11-02T18:23:01Z"
via: "live"
sha256: "a540920b00d8e12cce0ee5ff750479c7637a854ae6931bd5ec6f6dee926c3c60"
wayback: "https://web.archive.org/web/20261023025258/https://access.redhat.com/security/updates/backporting"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# Security Backporting Practice - Red Hat Customer Portal

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://access.redhat.com/security/updates/backporting>
> Snapshot: <https://web.archive.org/web/20261023025258/https://access.redhat.com/security/updates/backporting>

# Backporting Security Fixes

We use the term backporting to describe the action of taking a fix for a security flaw out of the most recent version of an upstream software package and applying that fix to an older version of the package we distribute.

Backporting is common among vendors like Red Hat and is essential to ensuring we can deploy automated updates to customers with minimal risk. Backporting might be a new concept for those more familiar with proprietary software updates.

Here is an example of why we backport security fixes:

Red Hat provides version 5.3 of PHP in Red Hat Enterprise Linux 6. The upstream version of PHP 5.3 has reached the end of life on August 14, 2014, meaning that no additional fixes or enhancements are provided for this version by upstream. However, on October 14, 2014, a buffer overflow flaw [CVE-2014-3670](https://access.redhat.com/security/cve/CVE-2014-3670), rated as [Important](https://access.redhat.com/security/updates/classification/#important), has been discovered in all versions of PHP that could allow a remote attacker to crash a PHP application or, possibly, execute arbitrary code with the privileges of the user running that PHP application.

Because version 5.3 of PHP has been retired upstream, the fix for this issue was not provided in an upstream release of PHP 5.3. The only way to mitigate the issue would be to upgrade to PHP 5.4, which did provide a fix for CVE-2014-3670. However, Red Hat customers using PHP 5.3 may not be able to migrate to PHP 5.4 due to possible backward compatibility problems between versions 5.3 and 5.4. The migration process would require manual effort by system administrators or developers. For this reason, Red Hat provided (backported) the fix for this issue to the PHP 5.3 packages shipped with Red Hat Enterprise Linux 6 so that customers could keep using PHP 5.3 and would mitigate CVE-2014-3670 at the same time.

When we backport security fixes, we:

- identify the fixes and isolate them from any other changes
- make sure the fixes do not introduce unwanted side effects
- apply the fixes to our previously released versions

For most products, our default practice is to backport security fixes, but we do sometimes provide version updates for some packages after careful testing and analysis. These are likely to be packages that have no interaction with others, or those used by an end-user, such as web browsers and instant messaging clients.

## Explaining Common Release-Numbering Confusion

Backporting has a number of advantages for customers, but it can create confusion when it is not understood. Customers need to be aware that just looking at the version number of a package will not tell them if they are vulnerable or not. For example, stories in the press may include phrases such as "upgrade to Apache httpd 2.0.43 to fix the issue," which only takes into account the upstream version number. This can cause confusion as even after installing updated packages from a vendor, it is not likely customers will have the latest upstream version. They will instead have an older upstream version with backported patches applied.

Also, some security scanning and auditing tools make decisions about vulnerabilities based solely on the version number of components they find. This results in false positives as the tools do not take into account backported security fixes.

Since the introduction of Red Hat Enterprise Linux, we have been careful to explain in our security advisories how we fixed an issue, whether by moving to a new upstream version or by backporting patches to the existing version. We have attached [CVE names](http://www.redhat.com/security/transparent/cve/) to all our advisories since January 2000, allowing customers to easily cross-reference vulnerabilities and find out how and when we fixed them, independent of version numbers.

We also supply [OVAL definitions](https://access.redhat.com/articles/221883) (machine-readable versions of our advisories) that third-party vulnerability tools can use to determine the status of vulnerabilities, even when security fixes have been backported. In doing this, we hope to remove some of the confusion surrounding backporting and make it easier for customers to always keep up to date with the latest security fixes.
