---
url: "https://people.redhat.com/sgrubb/audit/"
title: "Audit"
captured: "2026-11-02T18:39:32Z"
via: "live"
sha256: "5f7a5a4c12b1eceef0cad6e7fe94bb4406379bb3453cb75dcb25f7414ae0e81c"
wayback: "https://web.archive.org/web/20260827123805/https://people.redhat.com/sgrubb/audit/"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Audit

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://people.redhat.com/sgrubb/audit/>
> Snapshot: <https://web.archive.org/web/20260827123805/https://people.redhat.com/sgrubb/audit/>

|    |

## Linux Audit

|    |

[Up](https://people.redhat.com/sgrubb/index.html)

Download:

The audit package is no longer being distributed from this page. Get the latest release directly from github. The historical releases will still be here for the time being. The last one distributed from this page was released on Aug 08, 2024.
[ChangeLog](https://people.redhat.com/sgrubb/audit/ChangeLog)

[audit-4.0.2.tar.gz](https://people.redhat.com/sgrubb/audit/audit-4.0.2.tar.gz)

[audit-4.0.1.tar.gz](https://people.redhat.com/sgrubb/audit/audit-4.0.1.tar.gz)

RHEL-9

The latest maintenance release is 3.1.5, released July 29, 2024.

[audit-3.1.5.tar.gz](https://people.redhat.com/sgrubb/audit/audit-3.1.5.tar.gz)

[audit-3.1.4.tar.gz](https://people.redhat.com/sgrubb/audit/audit-3.1.4.tar.gz)

RHEL-7

[audit-2.8.5.tar.gz](https://people.redhat.com/sgrubb/audit/audit-2.8.5.tar.gz)

[audit-1.7.18.tar.gz](https://people.redhat.com/sgrubb/audit/audit-1.7.18.tar.gz)

|    |

### Future Direction

4.1.X - IDS / IPS
4.2.X - TLS for remote logging, (maybe container support)

### Technical Resources

github
git clone https://github.com/linux-audit/audit-userspace.git or [browse audit code](https://github.com/linux-audit/audit-userspace)

Mail List
The mail list was migrated to https://lists.linux-audit.osci.io/archives/list/linux-audit@lists.linux-audit.osci.iohttps://lists.linux-audit.osci.io (as of Nov 6, 2023).
Besides the archives at the new website, archives can also be found here:
https://marc.info/?l=linux-audit&r=1&w=2
https://lore.kernel.org/linux-audit

IRC
We have #audit on libera.chat

Blog
[Security + Data Science](http://security-plus-data-science.blogspot.com/)

Specs
The specifications have [moved to github.](https://github.com/linux-audit/audit-documentation/wiki)

Log Visualization Tool
[audit-explorer](http://github.com/stevegrubb/audit-explorer/)

Helpful Diagrams
A diagram showing [Audit System State](https://people.redhat.com/sgrubb/audit/audit-state-diagram.png)
A diagram of fields in common events [Event Fields](https://people.redhat.com/sgrubb/audit/record-fields.html)

Presentations:
[Presentation from 2018 devconf](https://people.redhat.com/sgrubb/audit/audit-normalizer.pdf) about the audit log normalizer
[Updated version of the 2007 Red Hat Summit slides](https://people.redhat.com/sgrubb/audit/audit_ids_2011.pdf) about audit system and layering an IDS/IPS on it
[Presentation given at Red Hat Summit 2008](https://people.redhat.com/sgrubb/audit/audit-ids.pdf) about audit system and the prelude plugin
[Presentation given at Red Hat Summit 2007](https://people.redhat.com/sgrubb/audit/summit07_audit_ids.pdf) about audit system and layering an IDS/IPS on it
[Slides from audit BoFs at SE Linux Symposium 2007](https://people.redhat.com/sgrubb/audit/audit-bofs-selinux-2007.odp)
[Slides from audit BoFs at SE Linux Symposium 2006](https://people.redhat.com/sgrubb/audit/audit-bofs-selinux-2006.odp)

FAQ
[Audit System FAQ](https://people.redhat.com/sgrubb/audit/audit-faq.txt) (old)

Test Suites
[ausearch-test-0.6](https://people.redhat.com/sgrubb/audit/ausearch-test-0.6.tar.gz)
[audit-validation-0.1](https://people.redhat.com/sgrubb/audit/audit-validation-0.1.tar.gz)

|
