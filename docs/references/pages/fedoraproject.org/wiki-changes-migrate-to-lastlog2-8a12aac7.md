---
url: "https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2"
title: "Changes/Migrate to lastlog2 - Fedora Project Wiki"
captured: "2026-11-03T02:57:15Z"
via: "live"
sha256: "d52aa262840a995309aa29a4de85e715b48f164222505bd698a5fcbd945a9a13"
wayback: "https://web.archive.org/web/20260722102016/https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2"
cited_in:
  - "docs/research/hardening.md"
---

# Changes/Migrate to lastlog2 - Fedora Project Wiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2>
> Snapshot: <https://web.archive.org/web/20260722102016/https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2>

https://fedoraproject.org/wiki/Fedora_Project_Wiki

- [Links](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#)

- [Get Fedora](https://getfedora.org/)
- [Fedora Docs](https://docs.fedoraproject.org/)
- [Fedora Magazine](https://fedoramagazine.org/)
- [What Can I Do?](https://whatcanidoforfedora.org/)

- [CodeOfConduct](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#)

- [Code of Conduct](https://docs.fedoraproject.org/en-US/project/code-of-conduct/)

- [Subprojects](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#)

- [Ambassadors](https://fedoraproject.org/wiki/Ambassadors)
- [Community Operations](https://fedoraproject.org/wiki/CommOps)
- [Design](https://fedoraproject.org/wiki/Design)
- [Documentation](https://fedoraproject.org/wiki/DocsProject)
- [EPEL](https://fedoraproject.org/wiki/EPEL)
- [Infrastructure](https://fedoraproject.org/wiki/Infrastructure)
- [Internationalization](https://fedoraproject.org/wiki/I18N)
- [Localization](https://fedoraproject.org/wiki/L10N)
- [Marketing](https://fedoraproject.org/wiki/Marketing)
- [Magazine](https://fedoraproject.org/wiki/Magazine)
- [Package Maintainers](https://fedoraproject.org/wiki/PackageMaintainers)
- [Quality Assurance](https://fedoraproject.org/wiki/QA)
- [Websites](https://fedoraproject.org/wiki/Websites)
- [All projects](https://fedoraproject.org/wiki/Projects)

- [Navigation](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#)

- [Special pages](https://fedoraproject.org/wiki/Special:SpecialPages)

[Log In](https://fedoraproject.org/w/index.php?title=Special:UserLogin&returnto=Changes%2FMigrate+to+lastlog2)

# Changes/Migrate to lastlog2

- [Page](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2)
- [Discussion](https://fedoraproject.org/w/index.php?title=Talk:Changes/Migrate_to_lastlog2&action=edit&redlink=1)

- [Read](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2)
- [View source](https://fedoraproject.org/w/index.php?title=Changes/Migrate_to_lastlog2&action=edit)
- [View history](https://fedoraproject.org/w/index.php?title=Changes/Migrate_to_lastlog2&action=history)

From Fedora Project Wiki

< [Changes](https://fedoraproject.org/wiki/Changes)

## Contents

- [1 Migrate to lastlog2](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Migrate_to_lastlog2)

- [1.1 Summary](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Summary)
- [1.2 Owner](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Owner)
- [1.3 Current status](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Current_status)
- [1.4 Detailed Description](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Detailed_Description)
- [1.5 Feedback](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Feedback)
- [1.6 Benefit to Fedora](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Benefit_to_Fedora)
- [1.7 Scope](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Scope)
- [1.8 Upgrade/compatibility impact](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Upgrade/compatibility_impact)
- [1.9 How To Test](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#How_To_Test)
- [1.10 User Experience](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#User_Experience)
- [1.11 Dependencies](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Dependencies)
- [1.12 Contingency Plan](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Contingency_Plan)
- [1.13 Documentation](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Documentation)
- [1.14 Release Notes](https://fedoraproject.org/wiki/Changes/Migrate_to_lastlog2#Release_Notes)

# Migrate to lastlog2

## Summary

Migrate lastlog and all functionality associated with it (e.g. pam_lastlog, the PAM service files) to lastlog2. util-linux provides lastlog2 functionality.

## Owner

- Name: [Iker Pedrosa](https://fedoraproject.org/w/index.php?title=User:Ipedrosa&action=edit&redlink=1)
- Email: <ipedrosa@redhat.com>

## Current status

- Targeted release: [Fedora Linux 43](https://docs.fedoraproject.org/en-US/releases/f43/)
- Last updated: 2025-04-16
- [Announced](https://lists.fedoraproject.org/archives/list/devel-announce@lists.fedoraproject.org/thread/RQ2XQVXXSX44NTP4LGHDU3ABXTPQX52F/)
- [Discussion thread](https://discussion.fedoraproject.org/t/f43-change-proposal-migrate-to-lastlog2-system-wide/146854)
- FESCo issue: [#3379](https://pagure.io/fesco/issue/3379)
- Tracker bug: [#2360108](https://bugzilla.redhat.com/show_bug.cgi?id=2360108)
- Release notes tracker: [#214](https://gitlab.com/fedora/docs/fedora-linux-documentation/release-notes/-/issues/214)

## Detailed Description

The Year 2038 problem (Y2038) poses a significant threat to systems relying on 32-bit time representations. It's crucial to address this before it impacts Fedora. This proposal suggests replacing the current lastlog utility with lastlog2, a robust and widely adopted solution already integrated into distributions like Debian and openSUSE.

lastlog reports the last login of a given user or of all users who did ever login on a system. The standard /var/log/lastlog implementation using lastlog.h from glibc uses a 32bit time_t in struct lastlog on bi-arch systems like x86-64 (so which can execute 64bit and 32bit binaries). So even if you have a pure 64bit system, on many architectures using glibc this leads to the [Y2038 problem](https://www.thkukuk.de/blog/Y2038_glibc_utmp_64bit/). lastlog2 is a Y2038 safe implementation of lastlog and it is considered as the natural replacement.

lastlog2 offers a comprehensive solution, including not just the lastlog2 binary for interacting with user login information, but also a dedicated library and a PAM module for seamless integration and accurate user tracking. Critically, lastlog2 provides a migration path for existing lastlog data.

## Feedback

This Fedora System-Wide Change is open to feedback and other proposals from the community.

## Benefit to Fedora

- Y2038 safe: it addresses the core issue by using a time representation that avoids the Y2038 overflow.
- SQLite3 backend: it leverages a modern and efficient database for storing login information.
- PAM module integration: data collection is handled exclusively through a PAM module, allowing any tool to utilize the information without requiring modifications to existing packages. This ensures broad compatibility and avoids cascading changes.
- Backward compatibility: output is designed to be as compatible as possible with the traditional lastlog format, minimizing disruption to existing workflows.
- Data migration: it provides a mechanism to import existing data from the legacy /var/log/lastlog file, preserving valuable login history.
- Scalability: the database size scales with the number of users, not the largest UID, offering improved performance and resource utilization, especially in environments with sparse UID allocation.

This change offers a proactive and well-supported solution to the Y2038 problem within Fedora, ensuring the continued reliability and security of the system. In short, lastlog2 is part of the util-linux project, which Fedora has been using for years, and which has been accepted by different communities such as Debian or OpenSuse as the ideal replacement for lastlog.

## Scope

- Proposal owners: shadow-utils, PAM and util-linux projects need to change their configuration to stop providing lastlog functionality and start providing lastlog2. In addition, util-linux creates a symbolic link to lastlog. authselect project needs to replace pam_lastlog by pam_lastlog2 in the PAM stack and update fedora content accordingly. Finally, util-linux needs to migrate the existing database from lastlog to lastlog2 format during the upgrade process.

- Other developers: N/A

- Release engineering: [#12608](https://pagure.io/releng/issue/12608)

- Policies and guidelines: N/A

- Trademark approval: N/A

- Alignment with the Fedora Strategy: N/A

## Upgrade/compatibility impact

Nothing will be affected, since the old database format from lastlog will be automatically migrated to the new lastlog2 format.

## How To Test

Two different scenarios come to my mind:

- New system: once the system is correctly installed authenticate as a user and run lastlog2 for this user. The timestamp of the last login should match the one that recently happened.
- Old system: once the system is correctly updated authenticate as root and run lastlog2 for another user. The timestamp of the last login should have happened before the update happened.

## User Experience

lastlog and pam_lastlog don't exist anymore and are replaced by lastlog2 and pam_lastlog2. In addition, util-linux creates a symbolic link to lastlog to simplify user access. They should behave in a similar manner to the aforementioned artifacts, thus no other change will be experienced by the user.

## Dependencies

There are no other dependencies.

## Contingency Plan

- Contingency mechanism: If any change was committed revert it.
- Contingency deadline: Just before beta freeze.
- Blocks release? No

## Documentation

- https://en.wikipedia.org/wiki/Year_2038_problem
- https://www.thkukuk.de/blog/Y2038_glibc_utmp_64bit/
- https://ikerexxe.github.io/idm/2023/06/14/Y2038-problem-in-shadow/

## Release Notes

Users should be informed that lastlog and pam_lastlog are replaced by lastlog2 and pam_lastlog2 in Fedora 43. In addition, the reasons for this change should be clearly explained. This FSWC proposal should be enough to understand the context, but if you have any doubts while writing the RN don't hesitate to contact the owner.

Retrieved from "https://fedoraproject.org/w/index.php?title=Changes/Migrate_to_lastlog2&oldid=736908"

[Categories](https://fedoraproject.org/wiki/Special:Categories):

- [ChangeAcceptedF43](https://fedoraproject.org/wiki/Category:ChangeAcceptedF43)
- [SystemWideChange](https://fedoraproject.org/wiki/Category:SystemWideChange)

About

[Get Fedora Linux](https://getfedora.org/)

[Sponsors](https://getfedora.org/en/sponsors/)

[Fedora Magazine](https://fedoramagazine.org)

[Legal](https://fedoraproject.org/wiki/Legal:Main#Legal)

Support

[Get Help](https://fedoraproject.org/wiki/Communicating_and_getting_help)

[Ask Fedora](https://ask.fedoraproject.org/)

[Common Issues](https://discussion.fedoraproject.org/c/ask/common-issues/)

[Fedora Developer Portal](https://developer.fedoraproject.org/)

Community

[Join Fedora](https://fedoraproject.org/wiki/Join)

[About Fedora](https://fedoraproject.org/wiki/Overview)

[Planet Fedora](http://fedoraplanet.org)

[Fedora Accounts](https://accounts.fedoraproject.org/)

This is a community maintained site. Red Hat is not responsible for content.

© 2026 Red Hat, Inc. and others. Content is available under Attribution-Share Alike 4.0 International unless otherwise noted.

Fedora is sponsored by Red Hat. [Learn more about the relationship between Red Hat and Fedora »](https://www.redhat.com/en/technologies/linux-platforms/articles/relationship-between-fedora-and-rhel)
