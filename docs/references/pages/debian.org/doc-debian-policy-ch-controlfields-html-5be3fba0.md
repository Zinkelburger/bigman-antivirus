---
url: "https://www.debian.org/doc/debian-policy/ch-controlfields.html"
title: "5. Control files and their fields — Debian Policy Manual v4.7.4.1"
captured: "2026-11-02T18:53:40Z"
via: "live"
sha256: "4d526c151f0a7dfcbe190f7b6fe4de5cc1a0f2208fb83e3ea40d60c7db9f8e15"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# 5. Control files and their fields — Debian Policy Manual v4.7.4.1

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.debian.org/doc/debian-policy/ch-controlfields.html>

- https://www.debian.org/doc/debian-policy/index.html
- 5. Control files and their fields
-  [View page source](https://www.debian.org/doc/debian-policy/_sources/ch-controlfields.rst.txt)

[Previous](https://www.debian.org/doc/debian-policy/ch-source.html) [Next](https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html)

# 5. Control files and their fields[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#control-files-and-their-fields)

The package management system manipulates data represented in a common format, known as control data, stored in control files. Control files are used for source packages, binary packages and the .changes files which control the installation of uploaded files. [[1]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id18)

## 5.1. Syntax of control files[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#syntax-of-control-files)

A control file consists of one or more stanzas of fields. [[2]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id19) The stanzas are separated by empty lines. Parsers may accept lines consisting solely of spaces and tabs as stanza separators, but control files should use empty lines. Some control files allow only one stanza; others allow several, in which case each stanza usually refers to a different package. (For example, in source packages, the first stanza refers to the source package, and later stanzas refer to binary packages generated from the source.) The ordering of the stanzas in control files is significant.

Each stanza consists of a series of data fields. Each field consists of the field name followed by a colon and then the data/value associated with that field. The field name is composed of US-ASCII characters excluding control characters, space, and colon (i.e., characters in the ranges U+0021 (!) through U+0039 (9), and U+003B (;) through U+007E (~), inclusive). Field names must not begin with the comment character (U+0023 #), nor with the hyphen character (U+002D -).

The field ends at the end of the line or at the end of the last continuation line (see below). Horizontal whitespace (spaces and tabs) may occur immediately before or after the value and is ignored there; it is conventional to put a single space after the colon. For example, a field might be:

```
Package: libc6

```

the field name is Package and the field value libc6.

Empty field values are only permitted in source package template control files (debian/control). Such fields are ignored.

A stanza must not contain more than one instance of a particular field name.

There are three types of fields:

simple

The field, including its value, must be a single line. Folding of the field is not permitted. This is the default field type if the definition of the field does not specify a different type.

folded

The value of a folded field is a logical line that may span several lines. The lines after the first are called continuation lines and must start with a space or a tab. Whitespace, including any newlines, is not significant in the field values of folded fields. [[3]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id20)

multiline

The value of a multiline field may comprise multiple continuation lines. The first line of the value, the part on the same line as the field name, often has special significance or may have to be empty. Other lines are added following the same syntax as the continuation lines of the folded fields. Whitespace, including newlines, is significant in the values of multiline fields.

Whitespace must not appear inside names (of packages, architectures, files or anything else) or version numbers, or between the characters of multi-character version relationships.

The presence and purpose of a field, and the syntax of its value, may differ between types of control files.

Field names are not case-sensitive, but it is usual to capitalize the field names using mixed case as shown below. Field values are case-sensitive unless the description of the field says otherwise.

Stanza separators (empty lines), and lines consisting only of U+0020 SPACE and U+0009 TAB, are not allowed within field values or between fields. Empty lines in field values are usually escaped by representing them by a U+0020 SPACE followed by a U+002E (.).

Lines starting with U+0023 (#), without any preceding whitespace, are comment lines that are only permitted in source package control files (debian/control). These comment lines are ignored, even between two continuation lines. They do not end logical lines.

All control files must be encoded in UTF-8.

## 5.2. Debian source package template control files – debian/control[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#debian-source-package-template-control-files-debian-control)

The debian/control file contains the most vital (and version-independent) information about the source package and about the binary packages it creates.

The first stanza of the control file contains information about the source package in general. The subsequent stanzas each describe a binary package that the source tree builds. Each binary package built from this source package has a corresponding stanza, except for any automatically-generated debug packages that do not require one.

The fields in the source package stanza (the first one) are:

-

[Source](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-source) (mandatory)

-

[Maintainer](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-maintainer) (mandatory)

-

[Uploaders](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-uploaders)

-

[Section](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-section) (recommended)

-

[Priority](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-priority)

-

[Build-Depends et al](https://www.debian.org/doc/debian-policy/ch-relationships.html#s-sourcebinarydeps)

-

[Standards-Version](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-standards-version) (mandatory)

-

[Homepage](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-homepage)

-

[Version Control System (VCS) fields](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-vcs-fields)

-

[Testsuite](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-testsuite)

-

[Rules-Requires-Root](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-rules-requires-root)

The fields in the binary package stanzas are:

-

[Package](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-package) (mandatory)

-

[Architecture](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-architecture) (mandatory)

-

[Section](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-section) (recommended)

-

[Priority](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-priority)

-

[Essential](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-essential)

-

[Depends et al](https://www.debian.org/doc/debian-policy/ch-relationships.html#s-binarydeps)

-

[Description](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-description) (mandatory)

-

[Homepage](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-homepage)

-

[Built-Using](https://www.debian.org/doc/debian-policy/ch-relationships.html#s-built-using)

-

[Package-Type](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-package-type)

-

Build-Profiles

The syntax and semantics of the fields are described below.

These fields are used by dpkg-gencontrol to generate control files for binary packages (see below), by dpkg-genchanges to generate the .changes file to accompany the upload, and by dpkg-source when it creates the .dsc source control file as part of a source archive. Some fields are folded in debian/control, but not in any other control file. These tools are responsible for removing the line breaks from such fields when using fields from debian/control to generate other control files. They are also responsible for discarding empty fields.

The fields here may contain variable references - their values will be substituted by dpkg-gencontrol, dpkg-genchanges or dpkg-source when they generate output control files. See [Variable substitutions: debian/substvars](https://www.debian.org/doc/debian-policy/ch-source.html#s-substvars) for details.

## 5.3. Debian binary package control files – DEBIAN/control[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#debian-binary-package-control-files-debian-control)

The DEBIAN/control file contains the most vital (and version-dependent) information about a binary package. It consists of a single stanza.

The fields in this file are:

-

[Package](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-package) (mandatory)

-

[Source](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-source)

-

[Version](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-version) (mandatory)

-

[Section](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-section) (recommended)

-

[Priority](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-priority) (recommended)

-

[Architecture](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-architecture) (mandatory)

-

[Essential](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-essential)

-

[Depends et al](https://www.debian.org/doc/debian-policy/ch-relationships.html#s-binarydeps)

-

[Installed-Size](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-installed-size)

-

[Maintainer](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-maintainer) (mandatory)

-

[Description](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-description) (mandatory)

-

[Homepage](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-homepage)

-

[Built-Using](https://www.debian.org/doc/debian-policy/ch-relationships.html#s-built-using)

## 5.4. Debian source package control files – .dsc[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#debian-source-package-control-files-dsc)

This file consists of a single stanza, possibly surrounded by an OpenPGP signature. The fields of that stanza are listed below. Their syntax is described above, in [Syntax of control files](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-controlsyntax).

-

[Format](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-format) (mandatory)

-

[Source](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-source) (mandatory)

-

[Binary](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-binary)

-

[Architecture](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-architecture)

-

[Version](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-version) (mandatory)

-

[Maintainer](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-maintainer) (mandatory)

-

[Uploaders](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-uploaders)

-

[Homepage](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-homepage)

-

[Version Control System (VCS) fields](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-vcs-fields)

-

[Testsuite](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-testsuite)

-

[Dgit](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-dgit)

-

[Git-Tag-Tagger](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-git-tag-tagger)

-

[Git-Tag-Info](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-git-tag-info)

-

[Standards-Version](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-standards-version) (mandatory)

-

[Build-Depends et al](https://www.debian.org/doc/debian-policy/ch-relationships.html#s-sourcebinarydeps)

-

[Package-List](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-package-list) (recommended)

-

[Checksums-Sha1 and Checksums-Sha256](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-checksums) (mandatory)

-

[Files](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-files) (mandatory)

The Debian source package control file is generated by dpkg-source when it builds the source archive, from other files in the source package, described above. When unpacking, it is checked against the files and directories in the other parts of the source package.

## 5.5. Debian upload changes control files – .changes[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#debian-upload-changes-control-files-changes)

The .changes files are used by the Debian archive maintenance software to process updates to packages. They consist of a single stanza, possibly surrounded by an OpenPGP signature. That stanza contains information from the debian/control file and other data about the source package gathered via debian/changelog and debian/rules.

.changes files have a format version that is incremented whenever the documented fields or their meaning change. This document describes format 1.8.

The fields in this file are:

-

[Format](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-format) (mandatory)

-

[Date](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-date) (mandatory)

-

[Source](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-source) (mandatory)

-

[Binary](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-binary) (mandatory in some cases)

-

[Architecture](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-architecture) (mandatory)

-

[Version](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-version) (mandatory)

-

[Distribution](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-distribution) (mandatory)

-

[Urgency](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-urgency) (recommended)

-

[Maintainer](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-maintainer) (mandatory)

-

[Changed-By](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-changed-by)

-

[Git-Tag-Tagger](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-git-tag-tagger)

-

[Git-Tag-Info](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-git-tag-info)

-

[Description](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-description) (mandatory in some cases)

-

[Closes](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-closes)

-

[Changes](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-changes) (mandatory)

-

[Checksums-Sha1 and Checksums-Sha256](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-checksums) (mandatory)

-

[Files](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-files) (mandatory)

## 5.6. List of fields[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#list-of-fields)

### 5.6.1. Source[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#source)

This field identifies the source package name.

In debian/control or a .dsc file, this field must contain only the name of the source package.

In a binary package control file or a .changes file, the source package name may be followed by a version number in parentheses. [[4]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id21) This version number may be omitted (and is, by dpkg-gencontrol) if it has the same value as the Version field of the binary package in question. The field itself may be omitted from a binary package control file when the source package has the same name and version as the binary package.

Package names (both source and binary, see [Package](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-package)) must consist only of lower case letters (a-z), digits (0-9), plus (+) and minus (-) signs, and periods (.). They must be at least two characters long and must start with an alphanumeric character.

### 5.6.2. Maintainer[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#maintainer)

The package maintainer’s name and email address. The name must come first, then the email address inside angle brackets <> (in RFC822 format).

If the maintainer’s name contains a full stop then the whole field will not work directly as an email address due to a misfeature in the syntax specified in RFC822; a program using this field as an address must check for this and correct the problem if necessary (for example by putting the name in round brackets and moving it to the end, and bringing the email address forward).

See [The maintainer of a package](https://www.debian.org/doc/debian-policy/ch-binary.html#s-maintainer) for additional requirements and information about package maintainers.

### 5.6.3. Uploaders[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#uploaders)

List of the names and email addresses of co-maintainers of the package, if any. If the package has other maintainers besides the one named in the [Maintainer field](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-maintainer), their names and email addresses should be listed here. The format of each entry is the same as that of the Maintainer field, and multiple entries must be comma separated.

This is normally an optional field, but if the Maintainer control field names a group of people and a shared email address, the Uploaders field must be present and must contain at least one human with their personal email address.

The Uploaders field in debian/control can be folded.

### 5.6.4. Changed-By[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#changed-by)

The name and email address of the person who prepared this version of the package, usually a maintainer. The syntax is the same as for the [Maintainer field](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-maintainer).

### 5.6.5. Section[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#section)

This field specifies an application area into which the package has been classified. See [Sections](https://www.debian.org/doc/debian-policy/ch-archive.html#s-subsections).

When it appears in the debian/control file, it gives the value for the subfield of the same name in the Files field of the .changes file. It also gives the default for the same field in the binary packages.

### 5.6.6. Priority[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#priority)

This field represents how important it is that the user have the package installed. See [Priorities](https://www.debian.org/doc/debian-policy/ch-archive.html#s-priorities).

When it appears in the debian/control file, it gives the value for the subfield of the same name in the Files field of the .changes file. It also gives the default for the same field in the binary packages.

### 5.6.7. Package[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#package)

The name of the binary package.

Binary package names must follow the same syntax and restrictions as source package names. See [Source](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-source) for the details.

### 5.6.8. Architecture[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#architecture)

Depending on context and the control file used, the Architecture field can include the following sets of values:

-

A unique single word identifying a Debian machine architecture as described in [Architecture specification strings](https://www.debian.org/doc/debian-policy/ch-customized-programs.html#s-arch-spec).

-

An architecture wildcard identifying a set of Debian machine architectures, see [Architecture wildcards](https://www.debian.org/doc/debian-policy/ch-customized-programs.html#s-arch-wildcard-spec). any matches all Debian machine architectures and is the most frequently used.

-

all, which indicates an architecture-independent package.

-

source, which indicates a source package.

In the main debian/control file in the source package, this field may contain the special value all, the special architecture wildcard any, or a list of specific and wildcard architectures separated by spaces. If all or any appears, that value must be the entire contents of the field. Most packages will use either all or any.

Specifying a specific list of architectures indicates that the source will build an architecture-dependent package only on architectures included in the list. Specifying a list of architecture wildcards indicates that the source will build an architecture-dependent package on only those architectures that match any of the specified architecture wildcards. Specifying a list of architectures or architecture wildcards other than any is for the minority of cases where a program is not portable or is not useful on some architectures. Where possible, the program should be made portable instead.

In the Debian source package control file .dsc, this field contains a list of architectures and architecture wildcards separated by spaces. When the list contains the architecture wildcard any, the only other value allowed in the list is all.

The list may include (or consist solely of) the special value all. In other words, in .dsc files unlike the debian/control, all may occur in combination with specific architectures. The Architecture field in the Debian source package control file .dsc is generally constructed from the Architecture fields in the debian/control in the source package.

Specifying only any indicates that the source package isn’t dependent on any particular architecture and should compile fine on any one. The produced binary package(s) will be specific to whatever the current build architecture is.

Specifying only all indicates that the source package will only build architecture-independent packages.

Specifying any all indicates that the source package isn’t dependent on any particular architecture. The set of produced binary packages will include at least one architecture-dependent package and one architecture-independent package.

Specifying a list of architectures or architecture wildcards indicates that the source will build an architecture-dependent package, and will only work correctly on the listed or matching architectures. If the source package also builds at least one architecture-independent package, all will also be included in the list.

In a .changes file, the Architecture field lists the architecture(s) of the package(s) currently being uploaded. This will be a list; if the source for the package is also being uploaded, the special entry source is also present. all will be present if any architecture-independent packages are being uploaded. Architecture wildcards such as any must never occur in the Architecture field in the .changes file.

See [Main building script: debian/rules](https://www.debian.org/doc/debian-policy/ch-source.html#s-debianrules) for information on how to get the architecture for the build process.

### 5.6.9. Essential[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#essential)

This is a boolean field which may occur only in the control file of a binary package or in a binary package stanza of a source package template control file.

If set to yes then the package management system will refuse to remove the package (upgrading and replacing it is still possible). The other possible value is no, which is the same as not having the field at all.

### 5.6.10. Package interrelationship fields: Depends, Pre-Depends, Recommends, Suggests, Breaks, Conflicts, Provides, Replaces, Enhances[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#package-interrelationship-fields-depends-pre-depends-recommends-suggests-breaks-conflicts-provides-replaces-enhances)

These fields describe the package’s relationships with other packages. Their syntax and semantics are described in [Declaring relationships between packages](https://www.debian.org/doc/debian-policy/ch-relationships.html).

### 5.6.11. Standards-Version[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#standards-version)

The most recent version of the standards (the policy manual and associated texts) with which the package complies. See [Standards conformance](https://www.debian.org/doc/debian-policy/ch-source.html#s-standardsversion).

The version number has four components: major and minor version number and major and minor patch level. When the standards change in a way that requires every package to change the major number will be changed. Significant changes that will require work in many packages will be signaled by a change to the minor number. The major patch level will be changed for any change to the meaning of the standards, however small; the minor patch level will be changed when only cosmetic, typographical or other edits are made which neither change the meaning of the document nor affect the contents of packages.

Thus only the first three components of the policy version are significant in the Standards-Version control field, and so either these three components or all four components may be specified. [[5]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id22)

udebs and source packages that only produce udebs do not use Standards-Version.

### 5.6.12. Version[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#version)

The version number of a package. The format is: [epoch:]upstream_version[-debian_revision].

The three components here are:

epoch

This is a single (generally small) unsigned integer. It may be omitted, in which case zero is assumed.

Epochs can help when the upstream version numbering scheme changes, but they must be used with care. You should not change the epoch, even in experimental, without getting consensus on debian-devel first.

upstream_version

This is the main part of the version number. It is usually the version number of the original (“upstream”) package from which the .deb file has been made, if this is applicable. Usually this will be in the same format as that specified by the upstream author(s); however, it may need to be reformatted to fit into the package management system’s format and comparison scheme.

The comparison behavior of the package management system with respect to the upstream_version is described below. The upstream_version portion of the version number is mandatory.

The upstream_version must contain only alphanumerics [[6]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id23) and the characters . + - ~ (full stop, plus, hyphen, tilde) and should start with a digit. If there is no debian_revision then hyphens are not allowed.

debian_revision

This part of the version number specifies the version of the Debian package based on the upstream version. It must contain only alphanumerics and the characters + . ~ (plus, full stop, tilde) and is compared in the same way as the upstream_version is.

It is conventional to restart the debian_revision at 1 each time the upstream_version is increased.

The package management system will break the version number apart at the last hyphen in the string (if there is one) to determine the upstream_version and debian_revision. The absence of a debian_revision is equivalent to a debian_revision of 0.

Presence of the debian_revision part indicates this package is a non-native package (see [Source packages](https://www.debian.org/doc/debian-policy/ch-source.html#s-source-packages)). Absence indicates the package is a native package.

When comparing two version numbers, first the epoch of each are compared, then the upstream_version if epoch is equal, and then debian_revision if upstream_version is also equal. epoch is compared numerically. The upstream_version and debian_revision parts are compared by the package management system using the following algorithm:

The strings are compared from left to right.

First the initial part of each string consisting entirely of non-digit characters is determined. These two parts (one of which may be empty) are compared lexically. If a difference is found it is returned. The lexical comparison is a comparison of ASCII values modified so that all the letters sort earlier than all the non-letters and so that a tilde sorts before anything, even the end of a part. For example, the following parts are in sorted order from earliest to latest: ~~, ~~a, ~, the empty part, a. [[7]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id24)

Then the initial part of the remainder of each string which consists entirely of digit characters is determined. The numerical values of these two parts are compared, and any difference found is returned as the result of the comparison. For these purposes an empty string (which can only occur at the end of one or both version strings being compared) counts as zero.

These two steps (comparing and removing initial non-digit strings and initial digit strings) are repeated until a difference is found or both strings are exhausted.

#### 5.6.12.1. Epochs should be used sparingly[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#epochs-should-be-used-sparingly)

Note that the purpose of epochs is to cope with situations where the upstream version numbering scheme changes and to allow us to leave behind serious mistakes. If you think that increasing the epoch is the right solution, you should consult debian-devel and get consensus before doing so (even in experimental).

Epochs should not be used when a package needs to be rolled back. In that case, use the +really convention: for example, if you uploaded 2.3-3 and now you need to go backwards to upstream 2.2, call your reverting upload something like 2.3+really2.2-1. Eventually, when we upload upstream 2.4, the +really part can go away.

Epochs are also not intended to cope with version numbers containing strings of letters which the package management system cannot interpret (such as ALPHA or pre-), or with silly orderings. [[8]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id25)

#### 5.6.12.2. Special version conventions[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#special-version-conventions)

The following special version numbering conventions are used in the Debian archive:

-

The absence of debian_revision, and therefore of a hyphen in the version number, indicates that the package is native.

-

The presence of +really in the upstream_version component indicates that a newer upstream version has been rolled back to an older upstream version. The part of the upstream_version component following +really is the true upstream version. See [Epochs should be used sparingly](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-avoid-epochs) for an example of when this is used.

Non-maintainer uploads:

-

debian_revision components ending in . (period) followed by a number indicate this version of the non-native package was uploaded by someone other than the maintainer (an NMU or non-maintainer upload). This is used for a upload including a source package; for uploads of only binary packages without source changes, see the binary NMU convention below.

-

upstream_version components in native packages ending in +nmu followed by a number indicate an NMU of a native package. As with the convention for non-native packages, this is used for an upload including a source package, not for uploads of only binary packages without source changes.

-

upstream_version components in native packages or debian_revision components in non-native packages ending in +b followed by a number indicate a binary NMU: an upload of a binary package without any source changes and hence without any corresponding source package upload or version change.

Stable updates:

-

debian_revision components in non-native packages ending in debNuX also indicate a stable update. Either ~ or + will be used before this string depending on the details of the update. N is the major version number of the Debian stable release to which the package was uploaded, and X is a number, starting at 1, that is increased for each stable upload of this package.

There are three cases for non-native packages:

-

For stable updates that use the same upstream version, the debian_revision component will end in +debNuX. The portion of the version before that string is the original package version in the stable release.

-

For stable updates to a new upstream version that is based on a newer unstable package, the debian_revision component will end in ~debNuX. The portion before that string will be the unstable version on which the package is based.

-

If a stable update is based on a new upstream version but is not based on a newer unstable package, the convention is to form the version number by taking the upstream version, appending -0, and then appending +debNuX (so the debian_revision component will be 0+debNuX).

In all cases, these versions are chosen so that they will sort earlier than a subsequent unstable package of the same upstream version and thus that the stable package will upgrade to a newer version during a subsequent system upgrade.

For example, suppose Debian 10 released with a package with version 1.4-5. If that package later receives a stable update in Debian 10 that uses the same upstream version, the first update would have the version 1.4-5+deb10u1. A subsequent update would have version 1.4-5+deb10u2.

If instead the package receives a stable update based on a 1.5-1 unstable package, the first such stable update would have the version 1.5-1~deb10u1 and a subsequent update would have the version 1.5-1~deb10u2.

If there were no unstable 1.5-1 package, but there were a stable update to an upstream 1.5 release, the first such stable update would have the version 1.5-0+deb10u1.

-

upstream_version components in native packages ending in +debNuX indicate a stable update. This is a version of the package uploaded directly to a stable release, and the version is chosen to sort before any later version of the package uploaded to Debian’s unstable or a later stable distribution. As with non-native packages, N is the major version number of the Debian stable release to which the package was uploaded, and X is a number, starting at 1, that is increased for each stable upload of this package.

For example, suppose Debian 10 released with a package with version 1.4. The first stable update of that package would have the version 1.4+deb10u1, and a subsequent update would have the version 1.4+deb10u2. These versions are chosen to sort before 1.5 (the next unstable version) or 1.4+deb11u1 (a stable update to a subsequent Debian 11 release).

Backports:

-

upstream_version components in native packages or debian_revision components in non-native packages ending in ~bpoNuX indicate a backport of a version of the package to an older stable release. The part of the version before ~bpo is the version of the package being backported, N is the major version number of the Debian stable release to which the package was backported, and X is a number, starting at 1, that is increased for each revision of the backport of that package version. The rationale is the same as for stable updates, with the additional goal of ensuring a backported version sorts earlier than a stable update with the same upstream version.

Be aware that the stable update and backport conventions can stack. If, for example, Debian 10 contains a package with version 1.4-5+deb10u1 and that package is backported to Debian 9, the version of the Debian 9 backport would be 1.4-5+deb10u1~bpo9u1 (although this scenario is rare).

This list of version conventions is not exhaustive.

### 5.6.13. Description[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#description)

In a source template control file or binary control file, the Description field contains a description of the binary package, consisting of two parts, the synopsis or the short description, and the long description. It is a multiline field with the following format:

```
Description: single line synopsis
 extended description over several lines

```

The lines in the extended description can have these formats:

-

Those starting with a single space are part of a paragraph. Successive lines of this form will be word-wrapped when displayed. The leading space will usually be stripped off. The line must contain at least one non-whitespace character.

-

Those starting with two or more spaces. These will be displayed verbatim. If the display cannot be panned horizontally, the displaying program will line wrap them “hard” (i.e., without taking account of word breaks). If it can they will be allowed to trail off to the right. None, one or two initial spaces may be deleted, but the number of spaces deleted from each line will be the same (so that you can have indenting work correctly, for example). The line must contain at least one non-whitespace character.

-

Those containing a single space followed by a single full stop character. These are rendered as blank lines. This is the only way to get a blank line. [[9]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id26)

-

Those containing a space, a full stop and some more characters. These are for future expansion. Do not use them.

Do not use tab characters. Their effect is not predictable.

See [The description of a package](https://www.debian.org/doc/debian-policy/ch-binary.html#s-descriptions) for further information on this.

In a .changes file, the Description field contains a summary of the descriptions of the binary packages being uploaded. If no binary packages are being uploaded, this field will not be present.

When used inside a .changes file, the Description field has a different format than in source or binary control files. It is a multiline field with one line per binary package. The first line of the field value (the part on the same line as Description:) is always empty. Each subsequent line is indented by one space and contains the name of a binary package, a space, a hyphen (-), a space, and the short description line from that package.

### 5.6.14. Distribution[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#distribution)

In a .changes file or parsed changelog output this contains the (space-separated) name(s) of the distribution(s) where this version of the package should be installed. Valid distributions are determined by the archive maintainers. [[10]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id27) The Debian archive software only supports listing a single distribution. Migration of packages to other distributions is handled outside of the upload process.

### 5.6.15. Date[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#date)

This field includes the date the package was built or last edited. It must be in the same format as the date in a debian/changelog entry.

The value of this field is usually extracted from the debian/changelog file - see [Debian changelog: debian/changelog](https://www.debian.org/doc/debian-policy/ch-source.html#s-dpkgchangelog)).

### 5.6.16. Format[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#format)

In [.changes](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-debianchangesfiles) files, this field declares the format version of that file. The syntax of the field value is the same as that of a [package version number](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-version) except that no epoch or Debian revision is allowed. The format described in this document is 1.8.

In [.dsc Debian source control](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-debiansourcecontrolfiles) files, this field declares the format of the source package. The field value is used by programs acting on a source package to interpret the list of files in the source package and determine how to unpack it. The syntax of the field value is a numeric major revision, a period, a numeric minor revision, and then an optional subtype after whitespace, which if specified is an alphanumeric word in parentheses. The subtype is optional in the syntax but may be mandatory for particular source format revisions. [[11]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id28)

### 5.6.17. Urgency[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#urgency)

This is a description of how important it is to upgrade to this version from previous ones. It consists of a single keyword taking one of the values low, medium, high, emergency, or critical [[12]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id29) (not case-sensitive) followed by an optional commentary (separated by a space) which is usually in parentheses. For example:

```
Urgency: low (HIGH for users of diversions)

```

The value of this field is usually extracted from the debian/changelog file - see [Debian changelog: debian/changelog](https://www.debian.org/doc/debian-policy/ch-source.html#s-dpkgchangelog).

### 5.6.18. Changes[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#changes)

This multiline field contains the human-readable changes data, describing the differences between the last version and the current one.

The first line of the field value (the part on the same line as Changes:) is always empty. The content of the field is expressed as continuation lines, with each line indented by at least one space. Blank lines must be represented by a line consisting only of a space and a full stop (.).

The value of this field is usually extracted from the debian/changelog file - see [Debian changelog: debian/changelog](https://www.debian.org/doc/debian-policy/ch-source.html#s-dpkgchangelog).

Each version’s change information should be preceded by a “title” line giving at least the version, distribution(s) and urgency, in a human-readable way.

If data from several versions is being returned the entry for the most recent version should be returned first, and entries should be separated by the representation of a blank line (the “title” line may also be followed by the representation of a blank line).

### 5.6.19. Binary[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#binary)

This folded field is a list of binary packages. Its syntax and meaning varies depending on the control file in which it appears.

When it appears in the .dsc file, it lists binary packages which a source package can produce, separated by commas [[13]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id30). The source package does not necessarily produce all of these binary packages for every architecture. The source control file doesn’t contain details of which architectures are appropriate for which of the binary packages.

When it appears in a .changes file, it lists the names of the binary packages being uploaded, separated by whitespace (not commas). If no binary packages are being uploaded, this field will not be present.

### 5.6.20. Installed-Size[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#installed-size)

This field appears in the binary package control files, and in the Packages files. It gives an estimate of the total amount of disk space required to install the named package. Actual installed size may vary based on block size, file system properties, or actions taken by package maintainer scripts.

The disk space is given as the accumulated size of each regular file and symlink rounded to 1 KiB used units, and a baseline of 1 KiB for any other filesystem object type.

### 5.6.21. Files[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#files)

This field contains a list of files with information about each one. The exact information and syntax varies with the context.

In all cases, Files is a multiline field. The first line of the field value (the part on the same line as Files:) is always empty. The content of the field is expressed as continuation lines, one line per file. Each line must be indented by one space and contain a number of sub-fields, separated by spaces, as described below.

In the .dsc file, each line contains the MD5 checksum, size and filename of the tar file and (if applicable) diff file which make up the remainder of the source package. [[14]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id31) For example:

```
Files:
 c6f698f19f2a2aa07dbb9bbda90a2754 571925 example_1.2.orig.tar.gz
 938512f08422f3509ff36f125f5873ba 6220 example_1.2-1.diff.gz

```

The exact forms of the filenames are described in [Source packages as archives](https://www.debian.org/doc/debian-policy/ap-pkg-sourcepkg.html#s-pkg-sourcearchives).

In the .changes file this contains one line per file being uploaded. Each line contains the MD5 checksum, size, section and priority and the filename. For example:

```
Files:
 4c31ab7bfc40d3cf49d7811987390357 1428 text extra example_1.2-1.dsc
 c6f698f19f2a2aa07dbb9bbda90a2754 571925 text extra example_1.2.orig.tar.gz
 938512f08422f3509ff36f125f5873ba 6220 text extra example_1.2-1.diff.gz
 7c98fe853b3bbb47a00e5cd129b6cb56 703542 text extra example_1.2-1_i386.deb

```

The [section](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-section) and [priority](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-priority) are the values of the corresponding fields in the source template control file. If no section or priority is specified then - should be used, though section and priority values must be specified for new packages to be installed properly.

The special value byhand for the section in a .changes file indicates that the file in question is not an ordinary package file and must be installed by hand by the distribution maintainers. If the section is byhand the priority should be -.

If a new Debian revision of a package is being shipped and no new original source archive is being distributed the .dsc must still contain the Files field entry for the original source archive package_upstream-version.orig.tar.gz, but the .changes file should leave it out. In this case the original source archive on the distribution site must match exactly, byte-for-byte, the original source archive which was used to generate the .dsc file and diff which are being uploaded.

### 5.6.22. Closes[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#closes)

A space-separated list of bug report numbers that the upload governed by the .changes file closes.

### 5.6.23. Homepage[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#homepage)

The URL of the web site for this package, preferably (when applicable) the site from which the original source can be obtained and any additional upstream documentation or information may be found. The content of this field is a simple URL without any surrounding characters such as <>.

### 5.6.24. Checksums-Sha1 and Checksums-Sha256[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#checksums-sha1-and-checksums-sha256)

These multiline fields contain a list of files with a checksum and size for each one. Both Checksums-Sha1 and Checksums-Sha256 have the same syntax and differ only in the checksum algorithm used: SHA-1 for Checksums-Sha1 and SHA-256 for Checksums-Sha256.

Checksums-Sha1 and Checksums-Sha256 are multiline fields. The first line of the field value (the part on the same line as Checksums-Sha1: or Checksums-Sha256:) is always empty. The content of the field is expressed as continuation lines, one line per file. Each line consists of the checksum, a space, the file size, a space, and the file name. For example (from a .changes file):

```
Checksums-Sha1:
 1f418afaa01464e63cc1ee8a66a05f0848bd155c 1276 example_1.0-1.dsc
 a0ed1456fad61116f868b1855530dbe948e20f06 171602 example_1.0.orig.tar.gz
 5e86ecf0671e113b63388dac81dd8d00e00ef298 6137 example_1.0-1.debian.tar.gz
 71a0ff7da0faaf608481195f9cf30974b142c183 548402 example_1.0-1_i386.deb
Checksums-Sha256:
 ac9d57254f7e835bed299926fd51bf6f534597cc3fcc52db01c4bffedae81272 1276 example_1.0-1.dsc
 0d123be7f51e61c4bf15e5c492b484054be7e90f3081608a5517007bfb1fd128 171602 example_1.0.orig.tar.gz
 f54ae966a5f580571ae7d9ef5e1df0bd42d63e27cb505b27957351a495bc6288 6137 example_1.0-1.debian.tar.gz
 3bec05c03974fdecd11d020fc2e8250de8404867a8a2ce865160c250eb723664 548402 example_1.0-1_i386.deb

```

In the .dsc file, these fields list all files that make up the source package. In the .changes file, these fields list all files being uploaded. The list of files in these fields must match the list of files in the Files field.

### 5.6.25. DM-Upload-Allowed[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#dm-upload-allowed)

Obsolete, see [below](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-dm-upload-allowed).

### 5.6.26. Version Control System (VCS) fields[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#version-control-system-vcs-fields)

Debian source packages are increasingly developed using VCSs. The purpose of the following fields is to indicate a publicly accessible repository where the Debian source package is developed.

Vcs-Browser

URL of a web interface for browsing the repository.

Here is an example of the form the two values take for Debian’s GitLab instance, Salsa:

```
Vcs-Browser: https://salsa.debian.org/debian/grep
Vcs-Git: https://salsa.debian.org/debian/grep.git

```

Vcs-<type>

The field name identifies the VCS. The field’s value uses the version control system’s conventional syntax for describing repository locations and should be sufficient to locate the repository used for packaging. Ideally, it also locates the branch used for development of new versions of the Debian package.

The following values for <type> are supported, with the corresponding VCS indicated in parentheses if it isn’t obvious:

-

Arch

-

Bzr (Bazaar)

-

Cvs (CVS)

-

Darcs

-

Git

-

Hg (Mercurial)

-

Mtn (Monotone)

-

Svn (Subversion)

In the case of Git, the value must have the following syntax:

```
<url> [ " -b " <branch> ] [ " [" <path> "]" ]

```

where the portions enclosed in brackets are optional and the portions enclosed in double quotes are literal strings. <url> indicates the repository. If the <branch> portion is present, it names a branch in the indicated repository. If no branch is specified, the packaging should be on the default branch. If the <path> portion is present, it specifies the relative path to the top of the packaging tree (the parent directory of the debian directory). If no path is specified, it defaults to . (the top level of the indicated repository and branch).

For example:

```
Vcs-Git: https://example.org/repo -b debian [p/package]

```

indicates a subdirectory named p/package in the debian branch of the repository at https://example.org/repo.

In the case of Mercurial, the value must have the following syntax:

```
<url> [ " -b " <branch> ]

```

This is interpreted the same way as the Git syntax except a path within the repository is not supported.

A package control file must not have more than one Vcs-<type> field. If the package is maintained in multiple version control systems, the maintainer should specify the one that they would prefer other people to use as the basis for proposing changes to the package.

For both fields, any URLs given should use a scheme that provides confidentiality (https, for example, rather than http or git) if the VCS repository supports it.

### 5.6.27. Package-List[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#package-list)

Multiline field listing all the packages that can be built from the source package, considering every architecture. The first line of the field value is empty. Each one of the next lines describes one binary package, by listing its name, type, section and priority separated by spaces. Fifth and subsequent space-separated items may be present and parsers must allow them. See the [Package-Type](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-package-type) field for a list of package types.

### 5.6.28. Package-Type[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#package-type)

Simple field containing a word indicating the type of package: deb for binary packages and udeb for micro binary packages. Other types not defined here may be indicated. In source package template control files, the Package-Type field should be omitted instead of giving it a value of deb, as this value is assumed for stanzas lacking this field.

### 5.6.29. Dgit[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#dgit)

Folded field containing a single git commit hash, presented in full, followed optionally by whitespace and other data to be defined in future extensions.

Declares that the source package corresponds exactly to a referenced commit in a Git repository available at the canonical location called dgit-repos, used by dgit, a bidirectional gateway between the Debian archive and Git. The commit is reachable from at least one reference whose name matches refs/dgit/*. See the manual page of dgit for further details.

### 5.6.30. Testsuite[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#testsuite)

Simple field containing a comma-separated list of values allowing test execution environments to discover packages which provide tests.

This field is automatically added to Debian source control files (.dsc) by dpkg, with the value autopkgtest, when a debian/tests/control file is present in the source package. This field may also be used in source package template control files (debian/control) if needed in other situations.

### 5.6.31. Rules-Requires-Root[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#rules-requires-root)

Simple field that defines if the source package requires access to root (or fakeroot) during selected targets in the [Main building script: debian/rules](https://www.debian.org/doc/debian-policy/ch-source.html#s-debianrules).

The field can consist of exactly one of the following three items:

-

no (default): Declares that neither root nor fakeroot is required. Package builders (e.g. dpkg-buildpackage) may choose to invoke any target in debian/rules with an unprivileged user.

-

binary-targets (previous default): Declares that the package will need the root (or fakeroot) when either of the binary, binary-arch or binary-indep targets are called. This is how every tool behaved before this field was defined.

-

A space separated list of keywords described below. These keywords must always contain a forward slash, which sets them apart from the other possible values of Rules-Requires-Root. When this list is provided, the builder must provide a “gain root command” (as defined in [debian/rules and Rules-Requires-Root](https://www.debian.org/doc/debian-policy/ch-source.html#s-debianrules-gainrootapi)) or pretend that the value was set to binary-targets, and both the builder and the package’s debian/rules script must downgrade accordingly (see below).

The default depends on the dpkg-build-api level (see dpkg-build-api(7)). If the dpkg-build-api level is undeclared, it is equivalent to level 0.

If the package builder supports the Rules-Requires-Root field and wants to enable the feature, then it must set the environment variable DEB_RULES_REQUIRES_ROOT when invoking the package building script debian/rules. The value of DEB_RULES_REQUIRES_ROOT should be one of:

-

The value of Rules-Requires-Root if the builder can support that value. The builder may trim unnecessary whitespace used to format the field for readability.

-

The value binary-targets if it cannot support the value of Rules-Requires-Root.

A compliant builder may also leave DEB_RULES_REQUIRES_ROOT unset or set it to binary-targets if it has been requested to test whether the package it builds correctly implements the fall-back for legacy builders.

#### 5.6.31.1. Remarks[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#remarks)

All packages and builders must support binary-targets as this was the historical behaviour prior to the introduction of this field.

Any tool (particularly older versions of them) may be unaware of this field and behave like the field was set to binary-targets. The package build must gracefully cope with this and produce a semantically equivalent result.

This field intentionally does not enable a package to request a true root over fakeroot.

#### 5.6.31.2. Definition of the keywords[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#definition-of-the-keywords)

The keywords have the format <namespace>/<case>, where:

-

<namespace> must consist entirely of printable ASCII characters except for any whitespace and the forward slash (/). It must consist of at least 2 characters.

-

/ (between <namespace> and <case>) is a single ASCII forward slash.

-

<case> must consist entirely of printable ASCII characters except for any whitespace. It must consist of at least 2 characters.

These keywords define where the package build script debian/rules, or the tools called by that script, will need access to root or fakeroot.

In addition to the keywords defined in the next section, each tool or package may define keywords within a namespace named after that tool or package. The package or tool is considered to own that namespace.

A tool is permitted to use the “gain root command” to do something under (fake)root if and only if the tool defines an appropriate keyword in its namespace, and the package lists that keyword in Rules-Requires-Root.

All tools must ignore keywords under namespaces they do not know or own. A tool may emit a warning, or abort with an error, if it finds unknown keywords in namespaces it owns, but it is not required to do this for all keywords in the namespace.

#### 5.6.31.3. Provided keywords[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#provided-keywords)

The following keywords are defined:

-

dpkg/target-subcommand: declares that there exists a command that the debian/rules file must run under (fake)root

-

dpkg/target/foo: declares that the additional, package-specific target foo (that is, not one of the targets specified in [Main building script: debian/rules](https://www.debian.org/doc/debian-policy/ch-source.html#s-debianrules)) must be run under (fake)root

This list is intentionally incomplete. You should consult the documentation of the tool or package in question for which keywords it defines and when they are needed.

### 5.6.32. Git-Tag-Tagger[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#git-tag-tagger)

Name and e-mail address of the person who made the Git tag from which this upload was generated (and to which it corresponds) in accordance with the tagging protocol described in the tag2upload(5) manual page and [TAG2UPLOAD-DESIGN.txt](https://salsa.debian.org/dgit-team/dgit/-/blob/master/TAG2UPLOAD-DESIGN.txt). The syntax is the same as for the [Maintainer field](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-maintainer). The values for the name and e-mail address originate in the tagger line of the raw Git tag; they are transformed as necessary to satisfy the field’s syntax.

Uploads signed by an implemention of the tag2upload service must include this field. Uploads not generated in accordance with the tag2upload protocol must not include this field.

### 5.6.33. Git-Tag-Info[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#git-tag-info)

Other information about the Git tag from which this upload was generated (and to which it corresponds) in accordance with the tagging protocol described in the tag2upload(5) manual page and [TAG2UPLOAD-DESIGN.txt](https://salsa.debian.org/dgit-team/dgit/-/blob/master/TAG2UPLOAD-DESIGN.txt).

The value is of the form tag=TAGOBJID fp=FINGERPRINT where TAGOBJID is the Git object ID of the Git annotated tag object, [[15]](https://www.debian.org/doc/debian-policy/ch-controlfields.html#id32) and FINGERPRINT is the fingerprint (in hexadecimal, without spaces) of the PGP key used to sign the Git tag. Other space-separated keyword=value items may be introduced in the future, and users of this field must ignore items with unknown keywords.

FINGERPRINT is taken from the first field of the VALIDSIG line emitted by gpgv(1), as specified in /usr/share/doc/gnupg/DETAILS.gz from the gnupg package. This will generally be the fingerprint of the signing subkey, if one was used, and the primary key’s fingerprint otherwise.

The Git annotated tag object is obtainable from the dgit-repos server, as described under Dgit, above.

Uploads signed by an implemention of the tag2upload service must include this field. Uploads not generated in accordance with the tag2upload protocol must not include this field.

### 5.6.34. Multi-Arch[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#multi-arch)

A Debian installation can combine packages from multiple architectures. The Multi-Arch field enables individual packages to declare their support for this feature, and influences the way dependencies are handled. It can be declared in binary package sections of a source package template control file and in binary package control files. The permitted field values are no (default), foreign, same and allowed. Their semantics are described in the following sections.

#### 5.6.34.1. Multi-Arch: no[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#multi-arch-no)

This means that a Depends, Pre-Depends, Provides, Recommends or Suggests relation between packages is satisfied only by packages of the same architecture. There is no effect on Breaks, Conflicts and Replaces. Furthermore, multiple instances of a package with the same name but different architectures cannot be installed at the same time. Architecture-independent packages are treated as though they had the architecture value of the installed dpkg package.

Note that, due to limitations in the archive management software, this value cannot currently be specified explicitly in binary package control files. Instead, the field needs to be absent in order to imply its default value and debhelper discards it when transforming a source package control template.

#### 5.6.34.2. Multi-Arch: foreign[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#multi-arch-foreign)

This means that the interfaces the package provides are independent of its architecture, and that it therefore satisfies dependencies on a package of this name regardless of the architecture of the package declaring the dependency. Any virtual packages provided (see [Virtual packages - Provides](https://www.debian.org/doc/debian-policy/ch-relationships.html#s-virtual)) inherit this property.

There are five main areas that can contribute to the interface of a package. If any of them provides an architecture-dependent interface, a package must not be marked with Multi-Arch: foreign. Note that architecture-independent packages can reasonably provide architecture-dependent interfaces and therefore architecture-independent packages are not implicitly treated as Multi-Arch: foreign.

-

The installed files of a package: Architecture-dependent packages may install different sets of files or files with different content for different architectures. These differences may contribute to the interface (e.g. an endianness-dependent database file). For architecture-independent binary packages, this aspect does not apply.

-

The behavior of package’s installed files: packages usually have an expected way of having their files used. They may be executed or interpreted. If the architecture of the package that installed them is exposed in the process of their intended use, their interface is considered architecture-dependent.

For instance, shared and static libraries are inherently architecture-dependent. They therefore prohibit the use of Multi-Arch: foreign unless they are not intended to be used by other packages. Whilst binary executables generally differ across architectures, the exposed interface can be independent of the architecture used to execute. Architecture exposure can happen via the command line arguments, the data exchanged or the way they process files. Last but not least, data files can have architecture-dependent structure.

-

Maintainer scripts and triggers: A package can behave in an architecture dependent way, when the behavior of maintainer scripts or invoked triggers depends on the architecture. For instance, byte-compiling source files into architecture-dependent bytecode during postinst may turn the interface of a package architecture-dependent.

-

The dependencies of a package: A package may expose functionality of other packages by depending on them and thereby provide an architecture-dependent interface itself. The ABI of a shared library linked into an executable does not usually contribute to the interface, but the behavior of library functions used may do so. Development packages for shared libraries necessarily expose their own dependencies as their interface since they enable others to link such libraries. An extreme example is transitional packages, whose main purpose is to expose the functionality of their dependency. Therefore, transitional packages should copy the Architecture and Multi-Arch field from their target.

-

Implicit and foreign dependencies of a package: Essential packages are implicitly depended upon and need not be included in Depends. Yet their behaviour can be architecture-dependent. For instance, using dpkg --print-architecture can be used to emit the native architecture even though dpkg is marked Multi-Arch: foreign. Similarly, calling pkgconf (without a prefix) will behave differently on different architectures as its search path is architecture-dependent even though pkgconf-bin is considered Multi-Arch: foreign.

The interfaces of a package are determined by its maintainer. However, some packages might expose architecture-specific dependencies when other packages use them in a manner not intended by the maintainer. This can happen when it is not clear which parts of the package are its interfaces.

In such cases, where the package satisfies the criterion for Multi-Arch: foreign but might expose architecture dependency, because it is not clear which parts of the package are its interfaces, the interfaces of the package should be described in the file debian/README.multiarch.

Conversely, packages must not use other packages in ways declined by their debian/README.multiarch.

#### 5.6.34.3. Multi-Arch: same[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#multi-arch-same)

This means that other packages with the exact same name and version, but different architectures, can be installed concurrently. All those packages must be declared Multi-Arch: same.

For a given file name, if the package for one architecture installs a file there, then the packages for every other architecture must either install no file there, or a bit-identical file. Given this restriction, such packages usually install most of their files below /usr/lib/${DEB_HOST_MULTIARCH}.

The maintainer scripts must be prepared to be configured or deconfigured multiple times. In particular, postrm must not assume that all instances of the package (e.g., instances for other architectures) are gone. It may check the DPKG_MAINTSCRIPT_PACKAGE_REFCOUNT environment variable set by dpkg.

#### 5.6.34.4. Multi-Arch: allowed[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#multi-arch-allowed)

This means the same as Multi-Arch: no except that a package depending on this package may choose to treat it as though it were marked Multi-Arch: foreign. This is done by appending :any to the name of the package in the dependency relation field. For example, where foo is Multi-Arch: allowed, bar might have Depends: foo:any (>= 1.0). Then bar’s dependency on foo is satisfied even when bar and foo have different architectures.

Only a Multi-Arch: allowed package satisfies a dependency with the :any suffix.

This value should be used rarely. It exists for cases where the package can be used in an architecture-dependent way or in an architecture-independent way and the decision of which applies is deferred to the depender. The most common use is with programming language interpreters that enable loading architecture-dependent plugins.

Since removing this value tends to break reverse dependencies that employ :any, uses of it should be discussed with debian-devel@lists.debian.org first.

### 5.6.35. Build-Profiles[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#build-profiles)

This field occurs in the binary package sections of a source template control file. It expresses whether the binary package is to be produced, given a set of enabled build profiles. The condition uses the same restriction formula syntax <s-restrictions> as the Build-Depends field. In the following example, libglib2.0-tests is only built if both the noinsttest and nogir build profiles are not enabled:

```
# Build if (!noinsttest) && (!nogir)
Package: libglib2.0-tests
Build-Profiles: <!noinsttest !nogir>

```

If a binary package stanza in a source package template control file does not contain a Build-Profiles field, this means that it builds unconditionally with respect to build profiles. If a binary package stanza in a source package template control file is annotated with a Build-Profiles field, then that binary package is generated if and only if the condition expressed by field’s value evaluates to true.

## 5.7. User-defined fields[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#user-defined-fields)

Additional user-defined fields may be added to the source package template control file. Such fields will be ignored, and not copied to (for example) binary or Debian source control files or Debian upload changes control files.

If you wish to add additional unsupported fields to these output files you should use the mechanism described here.

Fields in the source package template control file with names starting X, followed by one or more of the letters BCS and a hyphen -, will be copied to the output files. Only the part of the field name after the hyphen will be used in the output file. Where the letter B is used the field will appear in binary package control files, where the letter S is used in Debian source control files and where C is used in Debian upload changes control files.

For example, if the source package template control file contains the field

```
XBS-Comment: I stand between the candle and the star.

```

then the binary and Debian source control files will contain the field

```
Comment: I stand between the candle and the star.

```

## 5.8. Obsolete fields[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#obsolete-fields)

The following fields have been obsoleted and may be found in packages conforming with previous versions of the Policy.

### 5.8.1. DM-Upload-Allowed[](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-dm-upload-allowed)

Indicates that Debian Maintainers may upload this package to the Debian archive. The only valid value is yes. This field was used to regulate uploads by Debian Maintainers, See the General Resolution [Endorse the concept of Debian Maintainers](https://www.debian.org/vote/2007/vote_003) for more details.
