---
url: "https://diffoscope.org/"
title: "diffoscope: in-depth comparison of files, archives, and directories"
captured: "2026-11-02T18:26:33Z"
via: "live"
sha256: "0fc7a7a43b21519a20574bad2c34c12472556a8eeeb00ca8b9b19134b74de08e"
wayback: "https://web.archive.org/web/20261030204523/https://diffoscope.org/"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# diffoscope: in-depth comparison of files, archives, and directories

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://diffoscope.org/>
> Snapshot: <https://web.archive.org/web/20261030204523/https://diffoscope.org/>

https://salsa.debian.org/reproducible-builds/diffoscope

https://diffoscope.org/

#  [diffoscope](https://diffoscope.org/)

In-depth comparison of files, archives, and directories.

Latest release: 336 (30 Oct 2026)

# diffoscope

In-depth comparison of files, archives, and directories.

diffoscope tries to get to the bottom of what makes files or directories different. It will recursively unpack archives of many kinds and transform various binary formats into more human-readable form to compare them. It can compare two tarballs, ISO images, or PDF just as easily.

Latest release: 336 (30 Oct 2026)

You can try it now using Docker:

```
echo foo > file1
echo bar > file2
docker pull registry.salsa.debian.org/reproducible-builds/diffoscope
docker run --rm -t -w $(pwd) -v $(pwd):$(pwd):ro \
      registry.salsa.debian.org/reproducible-builds/diffoscope file1 file2

```

… or try it on [try.diffoscope.org](https://try.diffoscope.org/)!

## News

- [diffoscope 336 released](https://diffoscope.org/news/diffoscope-336-released/) (30 Oct 2026)
- [diffoscope 335 released](https://diffoscope.org/news/diffoscope-335-released/) (23 Oct 2026)
- [diffoscope 334 released](https://diffoscope.org/news/diffoscope-334-released/) (16 Oct 2026)
- [diffoscope 333 released](https://diffoscope.org/news/diffoscope-333-released/) (09 Oct 2026)
- [diffoscope 332 released](https://diffoscope.org/news/diffoscope-332-released/) (28 Sep 2026)

([RSS](https://diffoscope.org/feed.xml))

## Examples

https://diffoscope.org/examples/https-everywhere-5.0.6_vs_5.0.7.html

[Firefox extensions compared (HTML output)](https://diffoscope.org/examples/https-everywhere-5.0.6_vs_5.0.7.html)

https://diffoscope.org/examples/igerman98_20131206-5.txt

[Debian packages compared (text output)](https://diffoscope.org/examples/igerman98_20131206-5.txt)

… and more [examples on tests.reproducible-builds.org](https://tests.reproducible-builds.org/debian/unstable/amd64/index_FTBR.html). You can also view an [demonstration interactive shell session](https://diffoscope.org/static/images/progressbar.gif?1793391579101142219) (220K).

## Features

-

Command-line interface

-

Text and HTML ouput

-

File formats supported include: Android APK files, Android boot images, Android package resource table (ARSC), Apple Xcode mobile provisioning files, ar(1) archives, asar archives, ASM Function, Berkeley DB database files, bzip2 archives, character/block devices, ColorSync colour profiles (.icc), Coreboot CBFS filesystem images, cpio archives, Dalvik .dex files, Debian .buildinfo files, Debian .changes files, Debian source packages (.dsc), Device Tree Compiler blob files, directories, ELF binaries, ext2/ext3/ext4/btrfs/fat filesystems, eXtensible ARchive files, Filesystem image, Flattened Image Tree blob files, FreeDesktop Fontconfig cache files, FreePascal files (.ppu), Gettext message catalogues, GHC Haskell .hi files, GIF image files, Git repositories, GNU R database files (.rdb), GNU R Rscript files (.rds), Gnumeric spreadsheets, GPG keybox databases, Gzipped files, Hierarchical Data Format database, HTML files (.html), ISO 9660 CD images, Java .class files, Java .jmod modules, JavaScript files, JPEG images, JSON files, Linux kernel images, LLVM IR bitcode files, local (UNIX domain) sockets and named pipes (FIFOs), LZ4 compressed files, lzip compressed files, LZMA compressed files, macOS binaries, Microsoft Windows icon files, Microsoft Word .docx files, Mono ‘Portable Executable’ files, Mozilla-optimized .ZIP archives, Multimedia metadata, nar files, NuGet packages, OCaml interface files, Ogg Vorbis audio files, OpenOffice .odt files, OpenSSH public keys, OpenWRT package archives (.ipk), PDF documents, PE32 files, PGP signatures, PGP signed/encrypted messages, PNG images, PostScript documents, Public Key Cryptography Standards (PKCS) files (version #7), Python .pyc files, RPM archives, Rust object files (.deflate), Sphinx inventory files, SQLite databases, SquashFS filesystems, symlinks, tape archives (.tar), tcpdump capture files (.pcap), text files, TrueType font files, U-Boot legacy image files, UKI image (.efi), WebAssembly binary module, XML binary schemas (.xsb), XML files, XMLB files, XZ compressed files, ZIP archives and Zstandard compressed files.

-

Fallback on hexdump comparison

-

Fuzzy-matching to handle renamings

-

… and many more!

## Get diffoscope

-

[Try it online!](https://try.diffoscope.org/)

-

Via [pip](https://pypi.python.org/):
pip install diffoscope
Note: You might still want to install Python modules from [tlsh](https://github.com/trendmicro/tlsh) and [rpm](http://rpm.org/) and other external tools to get more meaningful results. Use diffoscope --list-tools to get the full list.

-

On [Debian](https://www.debian.org/) and derivatives:
apt install diffoscope

-

Via Docker:
docker run --rm -t -w $(pwd) -v $(pwd):$(pwd):ro registry.salsa.debian.org/reproducible-builds/diffoscope

-

On [Fedora](https://fedoraproject.org) based systems:
dnf install diffoscope

-

On [Gentoo](https://www.gentoo.org) based systems:
emerge dev-util/diffoscope

-

On [openSUSE](https://opensuse.org):
zypper in diffoscope

-

On [Arch Linux](https://archlinux.org):
pacman -S diffoscope

-

On [FreeBSD](https://freebsd.org) based systems:
pkg install py38-diffoscope

-

On [OpenBSD](https://openbsd.org) based systems ([packaging](https://github.com/openbsd/ports/tree/master/sysutils/diffoscope)):
pkg_add diffoscope

-

On [NixOS/nixpkgs](https://nixos.org/):
nix-shell -p diffoscope

-

On [Homebrew](https://brew.sh/):
brew install diffoscope

-

[Source tarballs](https://diffoscope.org/archive)

-

Through Git:
git clone https://salsa.debian.org/reproducible-builds/diffoscope.git

## Contribute

diffoscope is developed within the [“Reproducible builds” effort](https://reproducible-builds.org/).

-

[Git repository](https://salsa.debian.org/reproducible-builds/diffoscope)

- [Issues and feature requests](https://salsa.debian.org/reproducible-builds/diffoscope/issues)

- [File a new issue!](https://salsa.debian.org/reproducible-builds/diffoscope/issues/new) ([registration instructions](https://reproducible-builds.org/contribute/salsa/))

-

[Users and developers mailing-list](https://lists.reproducible-builds.org/listinfo/diffoscope)

- #reproducible-builds and/or #debian-reproducible on [OFTC](https://oftc.net/)

Extending diffoscope to support new formats is quite straightforward in most cases. It also has a comprehensive test suite. Patches welcome!

## Similar software

-

[pkgdiff](https://github.com/lvc/pkgdiff)

-

pkg-diff.sh from [Open Build Service build-compare](https://github.com/openSUSE/build-compare)

## License

diffoscope is free software licensed under the [GNU General Public License version 3](https://www.gnu.org/licenses/gpl.html) or later.

## Contributors

Akihiro Suda, Alex, Alexis Murzeau, Alyssa Ross, anthraxx, Balint Reczey, Baptiste Daroussin, Ben Hutchings, Benjamin Peterson, beuc@beuc.net, Brandon Maier, Brent Spillner, Brett Smith, Ceridwen, Chris Lamb, Christopher Baines, Clemens Lang, Colin Watson, Conrad Ratschan, Daniel Fullmer, Daniel Kahn Gillmor, Daniel Shahaf, Dhole, Dimitrios Apostolou, Ed Maste, Efraim Flashner, Eli Schwartz, Emanuel Bronshtein, Esa Peuha, FC (Fay) Stegerman, Felix Yan, Frazer Clews, fridtjof, Gianfranco Costamagna, Graham Christensen, Greg Chabala, Guangyuan Yang, Guillaume Girol, Hans-Christoph Steiner, Helmut Grohne, Holger Levsen, HW42, Ivan Trubach, James Addison, James Clarke, Jean-Romain Garnier, Jelle van der Waa, Jelmer Vernooĳ, Joachim Breitner, Jochen Sprickerhof, Johannes Schauer Marin Rodrigues, Juliana Oliveira, Jérémy Bobbio, Keith Smiley, klemens, Manuel Jacob, Marc Herbert, Marek Marczykowski-Górecki, Maria Glukhova, Marius Gedminas, Martin Abente Lahaye, Martin Joerg, Mattia Rizzolo, Michael Daniels, Michael Osipov, Michael R. Crusoe, Michel Messerschmidt, Mike Hommey, Mike McQuaid, Milena Boselli Rosa, Omair Majid, Paul Gevers, Paul Spooren, Paul Wise, Peter De Wachter, Rainer Müller, Reiner Herrmann, Ricardo Gaviria, Roland Clobus, Sam James, Santiago Torres Arias, Santiago Torres-Arias, Satyam Zode, Sergei Trofimovich, Seth Michael Larson, Siva Mahadevan, Vagrant Cascadian, Vekhir, Vibhu, Will Hollywood, Will Thompson, Xavier Briand, Ximin Luo, Yasushi SHOJI, Zachary T Welch, and Zbigniew Jędrzejewski-Szmek.
