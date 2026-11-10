#!/usr/bin/env bash
# Installs Bigman from this release tarball: bigman (CLI), bigmand (background
# service), bigman-app (desktop app) and bigman-canary-helper (only used if you
# turn on reader identification for canaries), with an application-menu entry.
#   ./install.sh                 under ~/.local, for you only
#   sudo ./install.sh /usr/local for every user
#   ./install.sh --uninstall [PREFIX]
# Installs nothing else and starts nothing. Needs WebKitGTK 4.1 (webkit2gtk-4.1 on
# Arch and Fedora, libwebkit2gtk-4.1-0 on Debian and Ubuntu) for the desktop app.
set -euo pipefail
here="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
uninstall=false
if [[ ${1:-} == --uninstall ]]; then uninstall=true; shift; fi
prefix="${1:-$HOME/.local}"
if [[ $prefix != /* || $prefix == *$'\n'* || $prefix == *$'\r'* || $prefix == *'%'* || $prefix == *'"'* || $prefix == *'`'* || $prefix == *'$'* || $prefix == *'\'* ]]; then
  echo 'Use an absolute installation prefix without desktop-file metacharacters.' >&2
  exit 2
fi
sizes=(32x32 128x128 256x256 512x512)
if $uninstall; then
  rm -f "$prefix/bin/bigman" "$prefix/bin/bigmand" "$prefix/bin/bigman-app" "$prefix/bin/bigman-canary-helper" \
    "$prefix/share/applications/bigman-app.desktop"
  for size in "${sizes[@]}"; do rm -f "$prefix/share/icons/hicolor/$size/apps/bigman-app.png"; done
  echo "Removed Bigman's programs from $prefix. Your settings and reports in ~/.local/share/bigman are kept."
  exit 0
fi
for bin in bigman bigmand bigman-app bigman-canary-helper; do
  install -Dm755 "$here/bin/$bin" "$prefix/bin/$bin"
done
for size in "${sizes[@]}"; do
  install -Dm644 "$here/share/icons/hicolor/$size/apps/bigman-app.png" \
    "$prefix/share/icons/hicolor/$size/apps/bigman-app.png"
done
# An absolute Exec line, since the prefix's bin folder may not be on the menu's PATH.
mkdir -p "$prefix/share/applications"
sed "s|^Exec=.*|Exec=\"$prefix/bin/bigman-app\"|" "$here/share/applications/bigman-app.desktop" \
  > "$prefix/share/applications/bigman-app.desktop"
chmod 644 "$prefix/share/applications/bigman-app.desktop"
printf 'Installed %s/bin/bigman, bigmand and bigman-app. Launch Bigman from your applications menu.\n' "$prefix"
