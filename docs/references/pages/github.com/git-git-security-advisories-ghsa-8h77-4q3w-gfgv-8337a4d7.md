---
url: "https://github.com/git/git/security/advisories/GHSA-8h77-4q3w-gfgv"
title: "Recursive clones on case-insensitive filesystems that support symlinks are susceptible to Remote Code Execution"
site: "GitHub"
captured: "2026-11-02T19:00:34Z"
via: "live"
sha256: "5449fb157b6dbdcdca47d3914987e57b697949146db59909baae0180f76ba8bd"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Recursive clones on case-insensitive filesystems that support symlinks are susceptible to Remote Code Execution

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/git/git/security/advisories/GHSA-8h77-4q3w-gfgv>

## Repository navigation

#  Recursive clones on case-insensitive filesystems that support symlinks are susceptible to Remote Code Execution

Critical

[dscho](https://github.com/dscho) published GHSA-8h77-4q3w-gfgv May 14, 2024

## Package

git

## Affected versions

v2.45.0 v2.44.0 <=v2.43.3 <=v2.42.1 v2.41.0 <=v2.40.1 <=v2.39.3

## Patched versions

v2.45.1 v2.44.1 v2.43.4 v2.42.2 v2.41.1 v2.40.2 v2.39.4

## Description

### Impact

Repositories with submodules can be crafted in a way that exploits a bug in Git whereby it can be fooled into writing files not into the submodule's worktree but into a .git/ directory. This allows writing a hook that will be executed while the clone operation is still running, giving the user no opportunity to inspect the code that is being executed.

### Patches

The problem has been patched in the versions published on Tuesday, May 14th, 2024.

### Workarounds

If symbolic link support is disabled in Git (e.g. via git config --global core.symlinks false), the described attack won't work.

As always, it is best to avoid cloning repositories from untrusted sources.

### References

- [git clone --recurse-submodules](https://git-scm.com/docs/git-clone#Documentation/git-clone.txt---recurse-submodulesltpathspecgt)
- [core.symlinks](https://git-scm.com/docs/git-config#Documentation/git-config.txt-coresymlinks)

### Severity

Critical

/ 10

#### CVSS v3 base metrics

Attack vector

Network

Attack complexity

High

Privileges required

None

User interaction

None

Scope

Changed

Confidentiality

High

Integrity

High

Availability

High

CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:C/C:H/I:H/A:H

### CVE ID

CVE-2024-32002

### Weaknesses

Weakness CWE-22

####   [Improper Limitation of a Pathname to a Restricted Directory ('Path Traversal')](https://github.com/advisories?query=cwe%3A22)

The product uses external input to construct a pathname that is intended to identify a file or directory that is located underneath a restricted parent directory, but the product does not properly neutralize special elements within the pathname that can cause the pathname to resolve to a location that is outside of the restricted directory.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/22.html)

Weakness CWE-434

####   [Unrestricted Upload of File with Dangerous Type](https://github.com/advisories?query=cwe%3A434)

The product allows the upload or transfer of dangerous file types that are automatically processed within its environment.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/434.html)

### Credits

-   https://github.com/filip-hejsek [filip-hejsek](https://github.com/filip-hejsek)   Reporter
-   https://github.com/dscho [dscho](https://github.com/dscho)   Coordinator
-   https://github.com/pks-t [pks-t](https://github.com/pks-t)   Remediation reviewer
