---
url: "https://www.shellcheck.net/"
title: "ShellCheck – shell script analysis tool"
captured: "2026-11-02T18:59:00Z"
via: "live"
sha256: "1a3b92797d24118eb18b6d4f794ddbc7580e76fb31a9fc118019d44596c404b9"
wayback: "https://web.archive.org/web/20261016015030/https://www.shellcheck.net/"
cited_in:
  - "REVIEW.md"
---

# ShellCheck – shell script analysis tool

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.shellcheck.net/>
> Snapshot: <https://web.archive.org/web/20261016015030/https://www.shellcheck.net/>

#!/bin/sh ## Example: a typical script with several problems for f in $(ls *.m3u) do grep -qi hq.*mp3 $f \ && echo -e 'Playlist $f contains a HQ file in mp3 format' done

#!/bin/sh ## Example: The shebang says 'sh' so shellcheck warns about portability ## Change it to '#!/bin/bash' to allow bashisms for n in {1..$RANDOM} do str="" if (( n % 3 == 0 )) then str="fizz" fi if [ $[n%5] == 0 ] then str="$strbuzz" fi if [[ ! $str ]] then str="$n" fi echo "$str" done

#!/bin/bash ## Example: ShellCheck can detect some higher level semantic problems while getopts "nf:" param do case "$param" in f) file="$OPTARG" ;; v) set -x ;; esac done case "$file" in *.gz) gzip -d "$file" ;; *.zip) unzip "$file" ;; *.tar.gz) tar xzf "$file" ;; *) echo "Unknown filetype" ;; esac if [[ "$$(uname)" == "Linux" ]] then echo "Using Linux" fi

#!/bin/bash ## Example: ShellCheck can detect many different kinds of quoting issues if ! grep -q backup=true.* "~/.myconfig" then echo 'Backup not enabled in $HOME/.myconfig, exiting' exit 1 fi if [[ $1 =~ "-v(erbose)?" ]] then verbose='-printf "Copying %f\n"' fi find backups/ \ -iname *.tar.gz \ $verbose \ -exec scp {} “myhost:backups” +

[➯](https://github.com/koalaman/shellcheck)

# ShellCheck

finds bugs in your shell scripts.

# ShellCheck

finds bugs in your shell scripts.

You can cabal, apt, dnf, pkg or brew install it locally right now.

Paste a script to try it out:

📄

Your Editor ([Ace](https://github.com/ajaxorg/ace) – loading 800kb of JS)

▼

▲

[Load random example](https://www.shellcheck.net/#)

Apply fixes  Report bug  Mobile paste:

```

```

📄

ShellCheck Output

If you paste a script in the editor above, this window will show shellcheck output.

## ShellCheck is...

- [GPLv3](https://github.com/koalaman/shellcheck/blob/master/LICENSE): free as in freedom
- documented on the [ShellCheck Wiki](https://www.shellcheck.net/wiki/Home)
- available on [GitHub](https://github.com/koalaman/shellcheck) (as is [this website](https://github.com/koalaman/shellcheck.net))
- already packaged for your [distro or package manager](https://github.com/koalaman/shellcheck#user-content-installing)
- supported as an [integrated linter](https://github.com/koalaman/shellcheck#user-content-in-your-editor) in major editors
- available in [CodeClimate](https://codeclimate.com/), [Codacy](https://www.codacy.com/) and [CodeFactor](https://www.codefactor.io/) to auto-check your GitHub repo
- written in Haskell, if you're into that sort of thing.

A special thanks to: [Gitpod](https://gitpod.io) • [BashSupport Pro](https://www.bashsupport.com/pro/)  • [Route4Me](https://route4me.com/) • [Siemens](https://github.com/siemens) • [per1234](https://github.com/per1234) • [cavcrosby](https://github.com/cavcrosby) • [dcminter](https://github.com/dcminter) • [photostructure](https://github.com/photostructure) • [Cronitor](https://cronitor.io) • [steve-chavez](https://github.com/steve-chavez) • [ChrLau](https://github.com/ChrLau) • [cjgibson](https://github.com/cjgibson) • [Celebian](https://celebian.com/)  • [LLC.org loves open source](https://www.llc.org/)  • [BestKru](https://bestkru.com/)

[Wiki Sitemap](https://www.shellcheck.net/wiki/)
