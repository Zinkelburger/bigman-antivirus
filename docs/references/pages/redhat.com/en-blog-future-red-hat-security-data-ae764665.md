---
url: "https://www.redhat.com/en/blog/future-red-hat-security-data"
title: "The future of Red Hat security data"
captured: "2026-11-02T18:58:27Z"
via: "live"
sha256: "f45ac7f5b17498ef3675c266700b133d57392d231bd43d4f9d0e2225d0a0ac4b"
wayback: "https://web.archive.org/web/20260713220440/https://www.redhat.com/en/blog/future-red-hat-security-data"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# The future of Red Hat security data

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.redhat.com/en/blog/future-red-hat-security-data>
> Snapshot: <https://web.archive.org/web/20260713220440/https://www.redhat.com/en/blog/future-red-hat-security-data>

# The future of Red Hat security data

April 28, 20237-minute read[Security](https://www.redhat.com/en/blog?f[0]=taxonomy_topic_tid:4491#rhdc-search-listing)

[Przemysław Roguski](https://www.redhat.com/en/authors/przemysław-roguski)

Principal Architect, Security Solutions

[Martin Prpic](https://www.redhat.com/en/authors/martin-prpic)

Share

https://www.facebook.com/sharer/sharer.php?u=https://www.redhat.com/en/blog/future-red-hat-security-datahttps://www.linkedin.com/shareArticle?mini=true&title=The%20future%20of%20Red%20Hat%20security%20data&summary=Red%20Hat%20security%20data%20is%20a%20central%20source%20of%20truth%20for%20Red%20Hat%20products%20regarding%20published%2C%20known%20vulnerabilities.%20The%20availability%20of%20accurate%20information%20in%20security%20data%20can%20help%20provide%20the%20correct%20risk%20assessment%20process%20in%20customers%27%20vulnerability%20management%20programs%2C%20which%20further%20helps%20with%20vulnerability%20patching%20prioritization.%20We%20work%20diligently%20to%20continuously%20improve%20our%20security%20data%20by%20adding%20more%20information%20to%20the%20existing%20data%2C%20introducing%20new%20data%20formats%20and%20cooperating%20with%20other%20vendors%2C%20including%20security%20scanner%20vendors%2C%20regarding%20the%20general%20approach%20to%20security%20data%20exchange.&url=https%3A%2F%2Fwww.redhat.com%2Fen%2Fblog%2Ffuture-red-hat-security-datahttps://twitter.com/intent/tweet?&url=https%3A%2F%2Fwww.redhat.com%2Fen%2Fblog%2Ffuture-red-hat-security-data

Subscribe to RSS https://www.redhat.com/en/rss/blog

- [Back to all posts](https://www.redhat.com/en/blog)

Red Hat security data is a central source of truth for Red Hat products regarding published, known vulnerabilities. The availability of accurate information in security data can help provide the correct risk assessment process in customers' vulnerability management programs, which further helps with vulnerability patching prioritization. We work diligently to continuously improve our security data by adding more information to the existing data, introducing new data formats and cooperating with other vendors, including security scanner vendors, regarding the general approach to security data exchange.

With new software flaws, vulnerabilities (assigned with CVE IDs) and exploits published daily, vulnerability information is almost immediately available to everyone, including customers and potential attackers. Based on various risk reports, including the recently published [Red Hat Product Security risk report 2022](https://www.redhat.com/en/resources/product-security-risk-report-2022), the volume of vulnerabilities with an assigned CVE ID grew 25% year-to-year from 2021 to 2022. Considering these numbers, it becomes evident that the “fixing everything” approach is unrealistic, primarily since just over 4% of published vulnerabilities represent a real risk to organizations.

What then, is the best approach to vulnerability management? The most important first step is developing a good understanding of software vulnerabilities and how to estimate the real risk correctly. To perform a good risk assessment, using accurate source security data from your vendor is essential. You can read more about this topic in the “[Demystifying risk using CVEs and CVSS](https://www.redhat.com/en/blog/demystifying-risk-using-cves-and-cvss)” blog post.

## Upcoming changes

Over the years, Red Hat published most vulnerability data using the [OVAL](https://access.redhat.com/security/data/oval/) and [CVRF](https://access.redhat.com/security/data/cvrf/) data formats to provide security information about Red Hat offerings. As with everything else, however, the security data landscape is constantly changing, and making adjustments and improvements to meet new industry standards and customer requirements is necessary.

### New security data formats

#### CSAF-VEX

In June 2022, Red Hat started publishing security advisories in a new [CSAF format](https://www.redhat.com/en/blog/common-security-advisory-framework-csaf-beta-files-now-available) as a version. In [February 2023, we officially announced](https://www.redhat.com/en/blog/csaf-vex-documents-now-generally-available) that the CSAF format for Red Hat security advisories is an official replacement to the old CVRF format, and all released advisories are publicly available under the https://access.redhat.com/security/data/csaf/v2/advisories/ path.

The production version of the CSAF files uses the [VEX profile](https://docs.oasis-open.org/csaf/csaf/v2.0/os/csaf-v2.0-os.html#45-profile-5-vex) to express which components of a specific product release have been patched to fix a particular CVE and which components are not affected by that CVE. Although we include information about not affected components in the specific product release, it is only related to the CVEs that are fixed in at least one of the components provided by the update. Information about the affectedness to all other existing CVEs is not included.

As a next step in Red Hat security data improvements, the entire VEX profile will be covered by separate VEX files that we will soon publish for every single CVE. By doing this, we will cover all product statuses defined by the VEX profile:

- Fixed: With a link to the released CSAF-VEX advisory
- Known Affected: Confirmation that the specific product and component is affected by a particular CVE
- Known Not Affected: Confirmation that the specific product and component is NOT affected by a particular CVE
- Under Investigation: Information that the Red Hat Product Security team is verifying the applicability (and its impact) of a specific CVE to a particular product and component

The main purpose of the VEX profile in the CSAF format is to allow vendors to assert whether specific vulnerabilities affect a product, and, if they are affected, what the remediation status is.

When CSAF advisories cover fixed and known-not-affected components in a specific product release, VEX files cover the applicability (communicated via the four statuses noted above) of a particular CVE to all related components for all Red Hat offerings. So, the key difference between CSAF and VEX files is that CSAF covers two statuses (fixed and not affected) for a specific product release, while VEX files cover all statuses for all products in one file for each CVE. VEX files will be published under a separate URL path.

#### SBOM

Red Hat is also starting to publish [SBOM (software bill of materials)](https://www.redhat.com/en/topics/security/what-is-sbom) files for core Red Hat offerings. An SBOM is a machine-readable, comprehensive inventory of software components and dependencies (manifest), with license and provenance information. SBOM files help establish reviews for procurement and audits of what is in a set of software applications/libraries. Combined with VEX, it helps an organization address its vulnerability risk assessment process. Together they provide information on where a potential risk may exist (where the vulnerable artifact is included and what is the correlation between this artifact and components or the product in general), and its current status to known vulnerabilities or exploits.

Currently, there are a lot of discussions in the industry about what the SBOM documents should include. Different types of SBOM have been proposed, as well as various ways to distribute them. Red Hat, together with other vendors, is working to define the specific requirements on publishing useful SBOMs that can be correlated with CSAF-VEX files, and inform consumers and partners about how to use this data. For now, SBOM files published by Red Hat are considered to be versions for customer testing and are available at https://security.access.redhat.com/data/sbom/v1/spdx/.

### Data relationships between CSAF, VEX and SBOM

When SBOMs help to identify the location of the potentially-affected components, VEX files help customers to find out if they are affected by a specific vulnerability. VEX provides the necessary information to perform a risk assessment (by providing risk metadata like Red Hat severity rating, Red Hat CVSS metrics or Red Hat CSAF advisory) that leads to a more effective approach to patching prioritization, and customers are able to focus on things that really matter.

https://www.redhat.com/rhdc/managed-files/red-hat-security-data-future-img1.png

The above diagram shows that CSAF files—representations of a single product release along with the CVEs that it fixes and the components that it ships—allow consumers to keep their SBOM up-to-date beyond its initial release. SBOM files are static documents that contain a listing of components in a specific version of a product. For example, the RHEL 9.2 release will have an accompanying SBOM that lists all components shipped by default in that release. Any security advisories released after RHEL 9.2 will update specific packages and provide information about component upgrades from those shipped in the 9.2 GA (generally available) version.

VEX files then provide a listing of affectedness of a CVE to each and every product release. For example, a VEX file of a hypothetical CVE-2099-1000 would contain information about whether or not RHEL 9.2 is affected by the vulnerability tracked by this CVE.

## What will happen to legacy data formats

With the production version of the CSAF files, the existing CVRF files are now considered deprecated and should not be used. The CVRF files will be published and updated until September 1, 2023. On that date, the available files will be bundled in a single archive file and placed under https://access.redhat.com/security/data/archive (this location is not yet accessible at the time this blog post is published).

In January 2023, [Red Hat officially announced](https://access.redhat.com/OVAL_v1_deprecation_announcement) the deprecation of OVAL and Data Stream (DS) v1 files.

As noted in that announcement, starting from April 1, 2023, new content is not published in the OVAL and DS v1 format. On July 1, 2023, all OVAL v1 and DS v1 data will be compressed and moved to the following archive locations:

- https://access.redhat.com/security/data/archive/oval_v1_<date>.tar.gz
- https://access.redhat.com/security/data/archive/ds_v1_<date>.tar.gz

Support of [OVAL v2](https://access.redhat.com/security/data/oval/v2/) content will continue until the end of 2024. Generally, the OVAL data format is not sufficient to support all new requirements for security scanning in containerized products with non-RPM content, or the representation of products and components version ranges. Data formats developed more recently by the security industry are better suited to support flexible and well-defined security data to exchange security information more efficiently and in a standardized way. This is why Red Hat will stop supporting OVAL v2 in favor of introducing these new data formats. In the next few months, Red Hat will provide information about the exact target OVAL v2 support end date.

## Changes to custom metrics data files

Red Hat publishes various metrics data to use for security analysis. Together with the introduction of new security data formats, we are going to stop publishing some of this metrics data. This deprecation is necessary to provide accurate and more consistent data across all available data formats, like CSAF-VEX files. Below is table with entire content of the https://access.redhat.com/security/data/metrics/ data with information on what future actions are taken and when (target effective date):

| File in the [https://access.redhat.com/](https://access.redhat.com/security/data/metrics/)
[security/data/metrics/](https://access.redhat.com/security/data/metrics/)
path | Action step | Effective date

| com.redhat.rhsa-all.xccdf.xml | This data is considered a part of the now deprecated OVAL v1 data, as announced in “[OVAL and Data Stream (DS) v1 deprecation announcement](https://access.redhat.com/OVAL_v1_deprecation_announcement)” article. | April 1, 2023

| container-name-repos-map.json | This file will be moved to the “/meta/v1” path. | November 1, 2023 *

| cpe-dictionary.xml | This file will be moved to the “/meta/v1” path. | November 1, 2023

|

CPE list files:

- cpelist-rhel6client-default-install.txt
- cpelist-rhel6server-default-install.txt
- cpelist-rhel7server-default-install.txt
- cpelist-rhel8-minimal-install.txt

| All files will be combined into a single compressed file and moved to the “/archive” directory under “/archive/cpelist.tar.gz” path. | November 1, 2023

| cve-metadata-from-bugzilla.xml | This file will be moved to the “/archive” directory. | November 1, 2023

| cvemap.xml | This file will be moved to the “/meta/v1” path. | November 1, 2023*

| cvemapcwe.txt | This file will be moved to the “/archive” directory. | November 1, 2023

|

daysofrisk.pl

And used by this script files:

- rhsamapcpe.txt
- cve_dates.txt
- release_dates.txt

| Script will be deleted together with associated files in favor of the [Security Data API](https://access.redhat.com/documentation/en-us/red_hat_security_data_api/1.0/html/red_hat_security_data_api/index) that can be used to perform any type of security data analysis. | November 1, 2023

| repository-to-cpe.json | This file will be moved to the “/meta/v1” path. | November 1, 2023*

| rhsa.rss | This file will be moved to the “/meta/v1” path. | November 1, 2023*

| rpm-to-cve.xml | This file will be moved to the “/archive” directory. | November 1, 2023

* Important note: Since this file is intensively used, a copy of this file will remain in the "/data/metrics/" path until the end of 2024 to allow smooth migration.

### Other coming changes related to the Red Hat security data

We also want to inform you that the [www.redhat.com](http://www.redhat.com) subdomain will not be used for serving security data starting January 2024. All security data will be available under the https://access.redhat.com/security/data path only.

## Future of Red Hat security data

As mentioned at the beginning of this blog post, our security data must continually evolve to meet new industry standards and customer needs. We want to allow our users to more easily find all necessary security data to make accurate risk assessments so they can take the necessary steps to mitigate or remediate vulnerabilities in their products and services. That is why having accurate, transparent and detailed security data is vital

Keep an eye on the [Red Hat Security Data Changelog](https://access.redhat.com/articles/5554431) to see any new changes to Red Hat's security data.

The new SBOM URL is publicly available, and if a SBOM is missing or incorrect, any Red Hat employee, customer or partner can open a ticket for [SBOM support](https://redhat.atlassian.net/servicedesk/customer/portal/68/user/login?destination=portal%2F68%2Fgroup%2F72%3FgroupId%3D72).

### About the authors

https://www.redhat.com/en/authors/przemysław-roguski

[###  Przemysław Roguski

Principal Architect, Security Solutions](https://www.redhat.com/en/authors/przemysław-roguski)

https://github.com/p-rog

https://pl.linkedin.com/in/przemys%C5%82aw-roguski-96969658

Przemysław “Rogue” Roguski is a Security Architect at Red Hat who specializes in shift-left security initiatives focusing on embedding security best practices and attestation into the earliest stages of the SDLC. He contributes security analysis work on Red Hat OpenShift and other OpenShift-related products. He also designs security solutions and processes across Red Hat.

He contributes to the security ecosystem as a member of the CISA SBOM/VEX working groups, an OASIS OpenEoX Technical Committee member and a key contributor to the CWE program.

[More from this author](https://www.redhat.com/en/authors/przemysław-roguski)

https://www.redhat.com/en/authors/martin-prpic

[###  Martin Prpic](https://www.redhat.com/en/authors/martin-prpic)

## More like this

Blog post

###  [Securing the enterprise software supply chain in the AI era](https://www.redhat.com/en/blog/securing-enterprise-software-supply-chain-ai-era)

Blog post

###  [Responsible disclosure in the age of Vulnpocalypse](https://www.redhat.com/en/blog/responsible-disclosure-age-vulnpocalypse)

Original podcast

###  [Can Compliance Be A Piece Of Cake? | Compiler](https://www.redhat.com/en/compiler-podcast/compliance)

Original podcast

###  [Collaboration In Product Security | Compiler](https://www.redhat.com/en/compiler-podcast/collaboration-in-product-security)

## Browse by channel

[Explore all channels](https://www.redhat.com/en/blog/channels)

### [Automation](https://www.redhat.com/en/blog/channel/management-and-automation)

The latest on IT automation for tech, teams, and environments

### [Artificial intelligence](https://www.redhat.com/en/blog/channel/artificial-intelligence)

Updates on the platforms that free customers to run AI workloads anywhere

### [Open hybrid cloud](https://www.redhat.com/en/blog/channel/hybrid-cloud-infrastructure)

Explore how we build a more flexible future with hybrid cloud

### [Security](https://www.redhat.com/en/blog/channel/security)

The latest on how we reduce risks across environments and technologies

### [Edge computing](https://www.redhat.com/en/blog/channel/edge-computing)

Updates on the platforms that simplify operations at the edge

### [Infrastructure](https://www.redhat.com/en/blog/channel/infrastructure)

The latest on the world’s leading enterprise Linux platform

### [Applications](https://www.redhat.com/en/blog/channel/applications)

Inside our solutions to the toughest application challenges

### [Virtualization](https://www.redhat.com/en/blog/channel/red-hat-virtualization)

The future of enterprise virtualization for your workloads on-premise or across clouds
