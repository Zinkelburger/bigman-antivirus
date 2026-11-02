---
url: "https://direnv.net/"
title: "direnv – unclutter your .profile"
site: "direnv"
captured: "2026-11-02T18:26:35Z"
via: "live"
sha256: "8e88a5f075dc93c3983eb0de0bd4ed2507c1809e56b46f57bcac494cd974a1b9"
wayback: "https://web.archive.org/web/20261012152026/https://direnv.net/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# direnv – unclutter your .profile

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://direnv.net/>
> Snapshot: <https://web.archive.org/web/20261012152026/https://direnv.net/>

# [direnv](https://direnv.net/)

# direnv – unclutter your .profile

https://builtwithnix.org https://repology.org/project/direnv/versions https://repology.org/project/direnv/versions https://matrix.to/#/#direnv:numtide.com

direnv is an extension for your shell. It augments existing shells with a new feature that can load and unload environment variables depending on the current directory.

## Use cases

- Load [12factor apps](https://12factor.net/) environment variables
- Create per-project isolated development environments
- Load secrets for deployment

## How it works

Before each prompt, direnv checks for the existence of a .envrc file (and [optionally](https://direnv.net/man/direnv.toml.1.html#codeloaddotenvcode) a .env file) in the current and parent directories. If the file exists (and is authorized), it is loaded into a bash sub-shell and all exported variables are then captured by direnv and then made available to the current shell.

It supports hooks for all the common shells like bash, zsh, tcsh and fish. This allows project-specific environment variables without cluttering the ~/.profile file.

Because direnv is compiled into a single static executable, it is fast enough to be unnoticeable on each prompt. It is also language-agnostic and can be used to build solutions similar to rbenv, pyenv and phpenv.

## Getting Started

### Prerequisites

- Unix-like operating system (macOS, Linux, …)
- A supported shell (bash, zsh, tcsh, fish, elvish, powershell, murex, nushell)

### Basic Installation

- direnv is packaged in most distributions already. See [the installation documentation](https://direnv.net/docs/installation.html) for details.
-  [hook direnv into your shell](https://direnv.net/docs/hook.html).

Now restart your shell.

### Quick demo

To follow along in your shell once direnv is installed.

```
# Create a new folder for demo purposes.
$ mkdir ~/my-project
$ cd ~/my-project

# Show that the FOO environment variable is not loaded.
$ echo ${FOO-nope}
nope

# Create a new .envrc. This file is bash code that is going to be loaded by
# direnv.
$ echo export FOO=foo > .envrc
.envrc is not allowed

# The security mechanism didn't allow to load the .envrc. Since we trust it,
# let's allow its execution.
$ direnv allow .
direnv: reloading
direnv: loading .envrc
direnv export: +FOO

# Show that the FOO environment variable is loaded.
$ echo ${FOO-nope}
foo

# Exit the project
$ cd ..
direnv: unloading

# And now FOO is unset again
$ echo ${FOO-nope}
nope

```

Some stdlib functions store files in .direnv/. Add .direnv/ to your .gitignore. For local overrides that you do not commit, put source_env_if_exists .envrc.local in .envrc and also ignore .envrc.local.

### The stdlib

Exporting variables by hand is a bit repetitive so direnv provides a set of utility functions that are made available in the context of the .envrc file.

As an example, the PATH_add function is used to expand and prepend a path to the $PATH environment variable. Instead of export PATH=$PWD/bin:$PATH you can write PATH_add bin. It’s shorter and avoids a common mistake where $PATH=bin.

To find the documentation for all available functions check the [direnv-stdlib(1) man page](https://direnv.net/man/direnv-stdlib.1.html).

It’s also possible to create your own extensions by creating a bash file at ~/.config/direnv/direnvrc or ~/.config/direnv/lib/*.sh. This file is loaded before your .envrc and thus allows you to make your own extensions to direnv.

Note that this functionality is not supported in .env files. If the coexistence of both is needed, one can use .envrc for leveraging stdlib and append dotenv at the end of it to instruct direnv to also read the .env file next.

## Docs

- [Install direnv](https://direnv.net/docs/installation.html)
- [Hook into your shell](https://direnv.net/docs/hook.html)
- [Develop for direnv](https://direnv.net/docs/development.html)
- [Manage your rubies with direnv and ruby-install](https://direnv.net/docs/ruby.html)
- [Using direnv with GitHub Actions](https://direnv.net/docs/github-actions.html)
- [Community Wiki](https://github.com/direnv/direnv/wiki)

Make sure to take a look at the wiki! It contains all sorts of useful information such as common recipes, editor integration, tips-and-tricks.

### Man pages

- [direnv(1) man page](https://direnv.net/man/direnv.1.html)
- [direnv-fetchurl(1) man page](https://direnv.net/man/direnv-fetchurl.1.html)
- [direnv-stdlib(1) man page](https://direnv.net/man/direnv-stdlib.1.html)
- [direnv.toml(1) man page](https://direnv.net/man/direnv.toml.1.html)

### FAQ

Based on GitHub issues interactions, here are the top things that have been confusing for users:

-

direnv has a standard library of functions, a collection of utilities that I found useful to have and accumulated over the years. You can find it here: https://github.com/direnv/direnv/blob/master/stdlib.sh

-

It’s possible to override the stdlib with your own set of function by adding a bash file to ~/.config/direnv/direnvrc. This file is loaded and its content made available to any .envrc file.

-

direnv is not loading the .envrc into the current shell. It’s creating a new bash sub-process to load the stdlib, direnvrc and .envrc, and only exports the environment diff back to the original shell. This allows direnv to record the environment changes accurately and also work with all sorts of shells. It also means that aliases and functions are not exportable right now.

### Translations

The direnv documentation is also available in other languages:

- [日本語 (Japanese)](https://gemmaro.github.io/direnv/)

## Contributing

Bug reports, contributions and forks are welcome. All bugs or other forms of discussion happen on http://github.com/direnv/direnv/issues .

Or drop by on [Matrix](https://matrix.to/#/#direnv:numtide.com) to have a chat. If you ask a question make sure to stay around as not everyone is active all day.

### Testing

To run our tests, use these commands: (you may need to install [homebrew](https://brew.sh/))

```
brew bundle
make test

```

## Complementary projects

Here is a list of projects you might want to look into if you are using direnv.

-  [starship](https://starship.rs/) - A cross-shell prompt.
-  [Projects for Nix integration](https://github.com/direnv/direnv/wiki/Nix) - choose from one of a variety of projects offering improvements over Direnv’s built-in use_nix implementation.

## Related projects

Here is a list of other projects found in the same design space. Feel free to submit new ones.

-  [Environment Modules](http://modules.sourceforge.net/) - one of the oldest (in a good way) environment-loading systems
-  [autoenv](https://github.com/hyperupcall/autoenv) - older, popular, and lightweight.
-  [zsh-autoenv](https://github.com/Tarrasch/zsh-autoenv) - a feature-rich mixture of autoenv and [smartcd](https://github.com/cxreg/smartcd): enter/leave events, nesting, stashing (Zsh-only).
-  [asdf](https://github.com/asdf-vm/asdf) - a pure bash solution that has a plugin system. The [asdf-direnv](https://github.com/asdf-community/asdf-direnv) plugin allows using asdf managed tools with direnv.
-  [ondir](https://github.com/alecthomas/ondir) - OnDir is a small program to automate tasks specific to certain directories
-  [shadowenv](https://shopify.github.io/shadowenv/) - uses an s-expression format to define environment changes that should be executed
-  [quickenv](https://github.com/untitaker/quickenv) - an alternative loader for .envrc files that does not hook into your shell and favors speed over convenience.
-  [mise](https://github.com/jdx/mise) - direnv, make and asdf all in one tool.

## Commercial support

Looking for help or customization?

Get in touch with Numtide to get a quote. We make it easy for companies to work with Open Source projects: https://numtide.com/contact

## COPYRIGHT

[MIT licence](https://direnv.net/LICENSE) - Copyright (C) 2019 [@zimbatm](https://github.com/zimbatm) and [contributors](https://github.com/direnv/direnv/graphs/contributors)

This site is open source. [Improve this page](https://github.com/direnv/direnv/edit/master/README.md).
