---
url: "https://fedoraproject.org/wiki/Third_Party_Repository_Policy"
title: "Third Party Repository Policy - Fedora Project Wiki"
captured: "2026-11-02T18:30:10Z"
via: "live"
sha256: "b5fe4ecafe323f20aeb5e43ee4afbdf23732de62d32090019f5daa170c4bcdd8"
wayback: "https://web.archive.org/web/20260715050748/https://fedoraproject.org/wiki/Third_Party_Repository_Policy"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# Third Party Repository Policy - Fedora Project Wiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://fedoraproject.org/wiki/Third_Party_Repository_Policy>
> Snapshot: <https://web.archive.org/web/20260715050748/https://fedoraproject.org/wiki/Third_Party_Repository_Policy>

https://fedoraproject.org/wiki/Fedora_Project_Wiki

- [Links](https://fedoraproject.org/wiki/Third_Party_Repository_Policy#)

- [Get Fedora](https://getfedora.org/)
- [Fedora Docs](https://docs.fedoraproject.org/)
- [Fedora Magazine](https://fedoramagazine.org/)
- [What Can I Do?](https://whatcanidoforfedora.org/)

- [CodeOfConduct](https://fedoraproject.org/wiki/Third_Party_Repository_Policy#)

- [Code of Conduct](https://docs.fedoraproject.org/en-US/project/code-of-conduct/)

- [Subprojects](https://fedoraproject.org/wiki/Third_Party_Repository_Policy#)

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

- [Navigation](https://fedoraproject.org/wiki/Third_Party_Repository_Policy#)

- [Special pages](https://fedoraproject.org/wiki/Special:SpecialPages)

[Log In](https://fedoraproject.org/w/index.php?title=Special:UserLogin&returnto=Third+Party+Repository+Policy)

# Third Party Repository Policy

- [Page](https://fedoraproject.org/wiki/Third_Party_Repository_Policy)
- [Discussion](https://fedoraproject.org/w/index.php?title=Talk:Third_Party_Repository_Policy&action=edit&redlink=1)

- [Read](https://fedoraproject.org/wiki/Third_Party_Repository_Policy)
- [View source](https://fedoraproject.org/w/index.php?title=Third_Party_Repository_Policy&action=edit)
- [View history](https://fedoraproject.org/w/index.php?title=Third_Party_Repository_Policy&action=history)

From Fedora Project Wiki

https://fedoraproject.org/wiki/File:Important.svg

This page is deprecated
FESCo docs have moved to [docs.fp.o](https://docs.fedoraproject.org/en-US/fesco/) with source hosted in a [pagure repo](https://pagure.io/fesco/fesco-docs). This page is now at https://docs.fedoraproject.org/en-US/fesco/Third_Party_Repository_Policy/.

End users sometimes want to install software that is not provided by Fedora. This policy lays out the extent to which Fedora Products can make it easier for end users to do that.

At the moment, FESCo policy is that no third party repositories can be configured in package managers in Fedora. However, Fedora may ship application search software that searches for applications in some specific third party repositories in addition to the Fedora Main Repository and install packages from them. This application search software can search for applications in these specific third party repositories as long as the user is explicitly asked to enable the repository before installing packages from them.

## Copr Repositories

Fedora allows contributors to build rpms and host the output in some repositories on our servers. These are known as Copr repositories. Packages in these repositories are not held to the same packaging standards as packages in the Main Fedora Repositories but they are all held to the same Licensing and Legal requirements. Fedora Legal has the authority to remove packages from the Copr repositories or have problematic Copr repositories removed as Red Hat is liable for any legal issues that may arise here. Due to this relationship, we are a little more flexible in our policy for Copr repositories than other third party repositories.

- The COPR Repositories can provide RPMS with .repo files pointing to themselves because Red Hat is the provider and assumes liability
- It is permissible to ship RPM packages containing .repo files that point to COPR repositories in the Fedora package collection under the following conditions per ​[FESCo decree](https://fedorahosted.org/fesco/ticket/1421):

- The repo file has the setting enabled=0. This means that yum, dnf and other tools cannot install software from this repository without a manual step (such as --enablerepo=<repo>)
- The repo file has the setting enabled_metadata=1. This means that some tools can optionally retrieve the metadata from this repository to provide a list of its contents to the user. The option is not used by yum or dnf.

Application installers in the main Fedora repositories may search COPR repos for applications to install as long as they explicitly ask the user to enable the copr repository as noted in the introductory section.

## Other Repositories with only free (libre) software

Of course, Fedora doesn't have the only software repositories that contain free (libre) software. There are other third party repositories that Fedora users want to use. Since Red Hat has no relationship with these repositories as it does with Copr repositories, allowing things in Fedora to point users to these repositories would represent a new legal liability. Fedora Legal would need to audit the packages in these repositories for legal problems both when the repositories are initially approved and on an ongoing basis (as the software in the repositories is updated, Fedora Legal would need to check that the new versions of packages in the repository remained legally okay for us to point people at.) For this reason, the rules for including a non-Copr third party repository are more strict than for Copr repos.

- Third party repositories that host diverse pieces of software (a repository like Fedora before it became a Red Hat community project, for instance) cannot be searched or enabled. This is because it would simply be too much work for Fedora Legal to audit such a repository.
- Repositories that enable a specific piece of free software may be pointed at in the same way as COPRs. However, they must be approved by both FESCo and Fedora Legal first.
- Fedora Legal is not limited to simply evaluating the repositories on Legal criteria. Because they are responsible for auditing the third party repositories on an ongoing basis, they have discretion to say no for other reasons including (but not limited to) simply not having time to take on the auditing of more repositories.
- FESCo and Fedora Legal can remove approval as well as grant it. This is due in part to the work that ongoing maintenance represents to Fedora Legal and also to the fact that package updates in the repositories could mean we no longer want to point to them.

Application installers in the main Fedora repositories may search repositories that are currently approved under the above list as long as they explicitly ask the user to enable the third party repository as noted in the introductory section.

## Repositories with non-free (libre) software

Repositories that contain non-free software may be offered to users under the following conditions:

- Users must be presented with clear information about Fedora provided/Libre software vs Non-free/3rd party software.

- Users must explicitly opt in to such repositories after the information is presented to them.

- Non free software repositories must be approved by a active Fedora Working Group (for an edition), or FESCO (for all other deliverables) and are subject to the same critera as the section above on other free software repositories (ie, permission may be revoked, repositories with many different applications will be rejected as too difficult to police, etc)

Non-free software may not be presented to the user without explicit user enablement in any Fedora Edition or Spin

Retrieved from "https://fedoraproject.org/w/index.php?title=Third_Party_Repository_Policy&oldid=526243"

[Category](https://fedoraproject.org/wiki/Special:Categories):

- [FESCo policy](https://fedoraproject.org/wiki/Category:FESCo_policy)

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
