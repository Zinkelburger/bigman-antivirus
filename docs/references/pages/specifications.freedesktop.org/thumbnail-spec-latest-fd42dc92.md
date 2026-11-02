---
url: "https://specifications.freedesktop.org/thumbnail-spec/latest/"
title: "Thumbnail Managing Standard"
captured: "2026-11-02T18:46:22Z"
via: "live"
source_url: "https://specifications.freedesktop.org/thumbnail/latest/"
sha256: "97127a3915e383eeb0d9b9a53b199ca6df6c5d53b53e2e2097571170f3ec6c47"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Thumbnail Managing Standard

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://specifications.freedesktop.org/thumbnail-spec/latest/>

[Jump to content](https://specifications.freedesktop.org/thumbnail/latest/#_content)[Jump to page navigation: previous page [access key p]/next page [access key n]](https://specifications.freedesktop.org/thumbnail/latest/#_bottom-navigation)

https://specifications.freedesktop.org/

[Thumbnail Managing Standard](https://specifications.freedesktop.org/thumbnail/latest/index.html)

[Show Contents: Thumbnail Managing Standard](https://specifications.freedesktop.org/thumbnail/latest/index.html)

[Top](https://specifications.freedesktop.org/thumbnail/latest/#)

- [1 History](https://specifications.freedesktop.org/thumbnail/latest/index.html#history)
- [2 Introduction](https://specifications.freedesktop.org/thumbnail/latest/introduction.html)
- [3 Issues To Solve](https://specifications.freedesktop.org/thumbnail/latest/issues.html)
- [4 Thumbnail Directory](https://specifications.freedesktop.org/thumbnail/latest/directory.html)
- [5 Thumbnail Creation](https://specifications.freedesktop.org/thumbnail/latest/creation.html)
- [6 Thumbnail Saving](https://specifications.freedesktop.org/thumbnail/latest/thumbsave.html)
- [7 Detect Modifications](https://specifications.freedesktop.org/thumbnail/latest/modifications.html)
- [8 Thumbnail Creation Failures](https://specifications.freedesktop.org/thumbnail/latest/failures.html)
- [9 Deleting Thumbnails](https://specifications.freedesktop.org/thumbnail/latest/delete.html)
- [10 Shared Thumbnail repositories](https://specifications.freedesktop.org/thumbnail/latest/shared.html)
- [11 Conclusion](https://specifications.freedesktop.org/thumbnail/latest/conclusion.html)
- [12 Thanks](https://specifications.freedesktop.org/thumbnail/latest/thanks.html)
- [13 Links](https://specifications.freedesktop.org/thumbnail/latest/links.html)

##  Thumbnail Managing Standard [#](https://specifications.freedesktop.org/thumbnail/latest/index.html)

Authors:

Jens Finke <jens@gnome.org>

Olivier Sessink <olivier@lx.student.wau.nl>

Publication Date: May 2012, Version: Version 0.8.0

[1 History](https://specifications.freedesktop.org/thumbnail/latest/index.html#history)

[2 Introduction](https://specifications.freedesktop.org/thumbnail/latest/introduction.html)

[3 Issues To Solve](https://specifications.freedesktop.org/thumbnail/latest/issues.html)

[4 Thumbnail Directory](https://specifications.freedesktop.org/thumbnail/latest/directory.html)

[5 Thumbnail Creation](https://specifications.freedesktop.org/thumbnail/latest/creation.html)

[6 Thumbnail Saving](https://specifications.freedesktop.org/thumbnail/latest/thumbsave.html)

[7 Detect Modifications](https://specifications.freedesktop.org/thumbnail/latest/modifications.html)

[8 Thumbnail Creation Failures](https://specifications.freedesktop.org/thumbnail/latest/failures.html)

[9 Deleting Thumbnails](https://specifications.freedesktop.org/thumbnail/latest/delete.html)

[10 Shared Thumbnail repositories](https://specifications.freedesktop.org/thumbnail/latest/shared.html)

[11 Conclusion](https://specifications.freedesktop.org/thumbnail/latest/conclusion.html)

[12 Thanks](https://specifications.freedesktop.org/thumbnail/latest/thanks.html)

[13 Links](https://specifications.freedesktop.org/thumbnail/latest/links.html)

## 1 History [#](https://specifications.freedesktop.org/thumbnail/latest/index.html#history)

-

December 2020, Version 0.9.0

-

Added x-large and xx-large thumbnail sizes

-

May 2012, Version 0.8.0

-

Modified to respect the [XDG Base Directory Specification (http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html)](http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html)

-

September 2004, Version 0.7.0

-

Added readonly support for shared thumbnail repositories

-

September 2002, Version 0.6.1

-

The subdirectories weren't a good idea. Removed them from this version.

-

Updated link to the MD5 implementation.

-

September 2002, Version 0.6

-

Added another sub directory level within the cache base directories to avoid too much clutter.

-

State not to create thumbnails for files within the thumbnail cache directory.

-

State when it's allowed to use thumbnails which haven't been checked for validity.

-

Some typo fixes.

-

Introduction and conclusion rewrite.

-

January 2002, Version 0.5

-

Changed handling of different thumbnail sizes.

-

Renamed directories.

-

Propose using temporary filenames to avoid problems with concurrent access.

-

Save thumbnails directly in the size dir without subdirs.

-

Added optional Thumb::Mimetype key

-

Give some more implementation notes.

-

Added "Thanks" section.

-

December 2001, Version 0.4

-

Destinction between required and optional thumbnail attributes.

-

Dropped distinction between global/local .thumbnail directories.

-

Use MD5 hashes as thumbnail filename.

-

Initial attempt to handle concurrent accesses by different programs.

-

Rewrote the "Deleting Thumbnails" section.

-

August 2001, Version 0.3

-

Rewrote this paper.

-

July 2001, Version 0.2

-

Removed distinction between low/high quality thumbnails.

-

Separate directory for failures.

-

Consider permission settings.

-

July 2001, Version 0.1

-

First public release.

[→Introduction](https://specifications.freedesktop.org/thumbnail/latest/introduction.html)

© 2026 Freedesktop.org
