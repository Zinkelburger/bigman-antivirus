#!/usr/bin/env bash
# Installs a built package inside a fresh distribution container and checks that the
# programs run. Run as root in the container, from the repository root, after
# scripts/build-packages.sh:
#   podman run --rm -v "$PWD:/src:ro" -w /src fedora:44 scripts/test-packages.sh
# Picks the .deb (apt), the .rpm (dnf) or the PKGBUILD (pacman, built with makepkg
# from the local tarball) by which package manager the container has.
set -euo pipefail
dist="$(cd -- "${1:-dist}" && pwd)"
fixture="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../tests/golden/inputs" && pwd)/installer.sh"
version="$(sed -n 's/^pkgver=//p' "$dist/PKGBUILD")"
fail() { echo "FAIL: $*" >&2; exit 1; }

(cd "$dist" && sha256sum -c SHA256SUMS)
if command -v apt-get >/dev/null; then
  export DEBIAN_FRONTEND=noninteractive
  apt-get update -qq
  apt-get install -y -qq "$dist/bigman_${version}_amd64.deb" >/dev/null
elif command -v dnf >/dev/null; then
  dnf install -y -q "$dist/bigman-${version}-1.x86_64.rpm"
elif command -v pacman >/dev/null; then
  pacman -Syu --noconfirm --needed -q base-devel >/dev/null
  id builder >/dev/null 2>&1 || useradd -m builder
  build="$(mktemp -d)"
  cp "$dist/PKGBUILD" "$dist/bigman-${version}-x86_64-linux.tar.gz" "$build/"
  # The release is not published yet: point the source at the local tarball.
  sed -i "s|^source=.*|source=(\"bigman-\$pkgver-x86_64-linux.tar.gz\")|" "$build/PKGBUILD"
  chown -R builder "$build"
  su builder -c "cd '$build' && makepkg --nodeps --noconfirm" >/dev/null
  pacman -U --noconfirm "$build"/bigman-bin-"$version"-*.pkg.tar.zst >/dev/null
else
  fail 'no apt-get, dnf or pacman in this container'
fi

for bin in bigman bigmand bigman-app bigman-canary-helper; do
  [[ -x /usr/bin/$bin ]] || fail "/usr/bin/$bin is missing"
done
[[ -f /usr/share/applications/bigman-app.desktop ]] || fail 'the bigman-app.desktop menu entry is missing'
bigman --version | grep -qF "$version" || fail "bigman --version does not say $version"
bigmand --version | grep -qF "$version" || fail "bigmand --version does not say $version"
# Every library the desktop app links must come with the package's dependencies.
if ldd /usr/bin/bigman-app | grep 'not found'; then fail 'bigman-app has unresolved libraries'; fi

home="$(mktemp -d)"
export HOME="$home" XDG_RUNTIME_DIR="$home/run"
mkdir -m 700 "$XDG_RUNTIME_DIR"
set +e
out="$(bigman --state "$home/state" inspect "$fixture")"
code=$?
set -e
[[ $code == 2 && $out == *'Installer inspection'* ]] || fail "bigman inspect gave exit $code: $out"
# The daemon starts and keeps running.
mkdir -m 700 "$home/daemon"
bigmand --state "$home/daemon" & pid=$!
sleep 3
kill -0 "$pid" 2>/dev/null || fail 'bigmand exited at startup'
kill "$pid"
wait "$pid" 2>/dev/null || true
echo "PASS: bigman $version installs and runs on $(. /etc/os-release && echo "$PRETTY_NAME")"
