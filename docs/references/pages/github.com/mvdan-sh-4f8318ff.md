---
url: "https://github.com/mvdan/sh"
title: "GitHub - mvdan/sh: A shell parser, formatter, and interpreter with bash and zsh support; includes shfmt"
site: "GitHub"
captured: "2026-11-02T19:06:15Z"
via: "live"
sha256: "e4ded33b2675a83b6c3dfff7a9a943dc37a865be83ff7f973cc28a960eadc47d"
wayback: "https://web.archive.org/web/20261021184207/https://github.com/mvdan/sh"
cited_in:
  - "REVIEW.md"
---

# GitHub - mvdan/sh: A shell parser, formatter, and interpreter with bash and zsh support; includes shfmt

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/mvdan/sh>
> Snapshot: <https://web.archive.org/web/20261021184207/https://github.com/mvdan/sh>

# sh

https://github.com/mvdan/sh#sh

https://pkg.go.dev/mvdan.cc/sh/v3

A shell parser, formatter, and interpreter. Supports [POSIX Shell](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html), [Bash](https://www.gnu.org/software/bash/), [Zsh](https://www.zsh.org/), and [mksh](http://www.mirbsd.org/mksh.htm). Requires Go 1.26 or later.

### Quick start

https://github.com/mvdan/sh#quick-start

To parse shell scripts into a syntax tree, inspect them, and format them like shfmt does, see the [syntax package](https://pkg.go.dev/mvdan.cc/sh/v3/syntax).

For one-call helpers with shell semantics, see the [shell package](https://pkg.go.dev/mvdan.cc/sh/v3/shell): split a command line into arguments and quote them back like shlex, expand $VAR and ~ in strings, and glob with ** or match case-style patterns.

To run shell scripts without a system shell, including on Windows, see the [interp package](https://pkg.go.dev/mvdan.cc/sh/v3/interp); its handlers allow configuring what scripts can execute and access.

### shfmt

https://github.com/mvdan/sh#shfmt

```
go install mvdan.cc/sh/v3/cmd/shfmt@latest

```

shfmt formats shell programs. See [canonical.sh](https://github.com/mvdan/sh/blob/master/syntax/canonical.sh) for a quick look at its default style. For example:

```
shfmt -l -w script.sh

```

For more information, see [its manpage](https://github.com/mvdan/sh/blob/master/cmd/shfmt/shfmt.1.scd), which can be viewed directly as Markdown or rendered with [scdoc](https://sr.ht/~sircmpwn/scdoc/).

Packages are available on [Alpine](https://pkgs.alpinelinux.org/packages?name=shfmt), [Arch](https://archlinux.org/packages/extra/x86_64/shfmt/), [Debian](https://tracker.debian.org/pkg/golang-mvdan-sh), [Docker](https://hub.docker.com/r/mvdan/shfmt/), [Fedora](https://packages.fedoraproject.org/pkgs/golang-mvdan-sh-3/shfmt/), [FreeBSD](https://www.freshports.org/devel/shfmt), [Homebrew](https://formulae.brew.sh/formula/shfmt), [MacPorts](https://ports.macports.org/port/shfmt/details/), [NixOS](https://github.com/NixOS/nixpkgs/blob/HEAD/pkgs/tools/text/shfmt/default.nix), [OpenSUSE](https://build.opensuse.org/package/show/openSUSE:Factory/shfmt), [PyPI](https://pypi.org/project/shfmt-py/), [Scoop](https://github.com/ScoopInstaller/Main/blob/HEAD/bucket/shfmt.json), [Snapcraft](https://snapcraft.io/shfmt), [Void](https://github.com/void-linux/void-packages/blob/HEAD/srcpkgs/shfmt/template) and [webi](https://webinstall.dev/shfmt/).

### Sponsoring

https://github.com/mvdan/sh#sponsoring

If this project saves you or your company time, consider [sponsoring me on GitHub](https://github.com/sponsors/mvdan). Monthly tiers include benefits like your logo on a README, prioritized issues, or direct support in your company's chat app. One-time tiers offer a call about one of my projects or a Go consulting or mentorship session.

https://namespace.so/github-actions/?utm_source=oss&utm_campaign=sh

### Contributing

https://github.com/mvdan/sh#contributing

Bug reports and feature requests should be filed as detailed issues, ideally with an example which reproduces the bug or shows what feature you're after.

Unless you're an active user or contributor to the project, drive-by AI patches are not helpful. File detailed issues instead.

### Caveats

https://github.com/mvdan/sh#caveats

- When indexing Bash associative arrays, always use quotes. The static parser will otherwise have to assume that the index is an arithmetic expression.

```
$ echo '${array[spaced string]}' | shfmt
<standard input>:1:16: not a valid arithmetic operator: `string`
$ echo '${array[weird!key]}' | shfmt
<standard input>:1:8: reached `!` without matching `[` with `]`
$ echo '${array[dash-string]}' | shfmt
${array[dash - string]}
```

- $(( and (( ambiguity is not supported. Backtracking would complicate the parser and make streaming support via io.Reader impossible. The POSIX spec recommends to [space the operands](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_03) if $( ( is meant.

```
$ echo '$((foo); (bar))' | shfmt
1:1: reached ) without matching $(( with ))
```

-

export, let, and declare are parsed as keywords. This allows statically building their syntax tree, as opposed to keeping the arguments as a slice of words. It is also required to support declare foo=(bar).

-

The entire library is written in pure Go, which limits how closely the interpreter can follow POSIX Shell and Bash semantics. For example, Go does not support forking its own process, so subshells use a goroutine instead, meaning that real PIDs and file descriptors cannot be used directly.

### Formatting FAQs

https://github.com/mvdan/sh#formatting-faqs

-

The formatter cannot be disabled for ranges of lines; most users wanting this are working around a bug or they don't like how a piece of code is formatted. Instead, search the issue tracker and file a new issue if necessary. Formatting of partial files leads to lots of edge cases and complexity which this project has no resources for, nor interest in, getting into.

-

We avoid adding more formatting options where possible. Each added flag interacts with all others, multiplying the human cost of development, maintenance, testing, and properly documenting the behavior for end users.

-

The true value in a formatter is consistency, especially for teams of developers. We do not aim to satisfy every developer's personal choice of optimal formatting.

### JavaScript

https://github.com/mvdan/sh#javascript

The parser and formatter are available as a third party npm package called [sh-syntax](https://github.com/un-ts/sh-syntax), which bundles a version of this library compiled to WASM.

Previously, we maintained an npm package called [mvdan-sh](https://www.npmjs.com/package/mvdan-sh) which used GopherJS to bundle a JS version of this library. That npm package is now archived given its poor performance and GopherJS not being as actively developed. Any existing or new users should look at [sh-syntax](https://github.com/un-ts/sh-syntax) instead.

### Docker

https://github.com/mvdan/sh#docker

All release tags are published via [Docker](https://hub.docker.com/r/mvdan/shfmt/), such as v3.5.1. The latest stable release is currently published as v3, and the latest development version as latest. The images only include shfmt; -alpine variants exist on Alpine Linux.

To build a Docker image, run:

```
docker build -t my:tag -f cmd/shfmt/Dockerfile .

```

To use a Docker image, run:

```
docker run --rm -u "$(id -u):$(id -g)" -v "$PWD:/mnt" -w /mnt my:tag <shfmt arguments>

```

### Related projects

https://github.com/mvdan/sh#related-projects

The following editor integrations wrap shfmt:

- [BashSupport-Pro](https://www.bashsupport.com/manual/editor/formatter/) - Bash plugin for JetBrains IDEs
- [dockerfmt](https://github.com/reteps/dockerfmt) - Dockerfile formatter using shfmt
- [intellij-shellscript](https://www.jetbrains.com/help/idea/shell-scripts.html) - Intellij Jetbrains shell script plugin
- [micro](https://micro-editor.github.io/) - Editor with a built-in plugin
- [neoformat](https://github.com/sbdchd/neoformat) - (Neo)Vim plugin
- [vscode-shfmt](https://marketplace.visualstudio.com/items?itemName=mkhl.shfmt) - VS Code plugin
- [shfmt.el](https://github.com/purcell/emacs-shfmt/) - Emacs package
- [Trunk](https://trunk.io/check) - Universal linter, available as a CLI, VS Code plugin, and GitHub action

Other noteworthy integrations include:

- [modd](https://github.com/cortesi/modd) - A developer tool that responds to filesystem changes
- [prettier-plugin-sh](https://github.com/un-ts/prettier/tree/master/packages/sh) - [Prettier](https://prettier.io) plugin using [sh-syntax](https://github.com/un-ts/sh-syntax)
- [sh-checker](https://github.com/luizm/action-sh-checker) - A GitHub Action that performs static analysis for shell scripts
- [mdformat-shfmt](https://github.com/hukkin/mdformat-shfmt) - [mdformat](https://github.com/executablebooks/mdformat) plugin to format shell scripts embedded in Markdown with shfmt
- [pre-commit-shfmt](https://github.com/scop/pre-commit-shfmt) - [pre-commit](https://pre-commit.com) shfmt hook
- [tesh](https://github.com/feloy/tesh) - Run scripts with mocks, assertions, and coverage
