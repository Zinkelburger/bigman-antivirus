#!/usr/bin/env bash
# Builds the release files into dist/:
#   bigman_VERSION_amd64.deb, bigman-VERSION-1.x86_64.rpm   (Tauri bundles)
#   bigman-VERSION-x86_64-linux.tar.gz                       (the four programs, icons and install.sh)
#   PKGBUILD                                                 (Arch bigman-bin, pointing at the tarball)
#   SHA256SUMS
# The release workflow runs this in an Ubuntu 22.04 container so the programs need
# glibc 2.35 or newer. Needs Rust, Node.js 22 with npm, and WebKitGTK 4.1 development
# files (see docs/development.md). Usage: scripts/build-packages.sh
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
cargo="${CARGO:-cargo}"
target="${CARGO_TARGET_DIR:-$root/target}"
python3 -I scripts/version.py check
if (($(id -u) > 999999)); then
  # Tauri writes the builder's uid into the .deb's ar headers, whose field has 6 digits.
  echo "warning: uid $(id -u) has more than 6 digits, so apt cannot read the .deb built here; build as root or in a container" >&2
fi
version="$(sed -n '/^\[workspace.package\]/,/^\[/s/^version = "\(.*\)"/\1/p' Cargo.toml)"
# Fixed timestamps and owners make the tarball depend only on the commit.
if [[ -z ${SOURCE_DATE_EPOCH:-} ]]; then
  SOURCE_DATE_EPOCH="$(git log -1 --format=%ct)"
fi
export SOURCE_DATE_EPOCH

(cd crates/app/ui && npm ci --ignore-scripts --no-audit --no-fund && npm run build)
# bigman-canary-helper is the optional root service for canary reader identification.
# It ships beside bigman, where Protect > Canaries looks for it; nothing installs or
# starts it until the user turns reader identification on.
"$cargo" build --locked --release -p bigman-cli -p bigman-daemon -p bigman-canary-helper

# tauri.conf.json bundles the CLI, the daemon and the helper from here, wherever the target dir is.
stage=crates/app/src-tauri/bundle-bin
rm -rf "$stage"
install -Dm755 -t "$stage" "$target/release/bigman" "$target/release/bigmand" "$target/release/bigman-canary-helper"
# Builds bigman-app in release mode, then the .deb and .rpm.
(cd crates/app/src-tauri && ../ui/node_modules/.bin/tauri build --ci --bundles deb,rpm -- --locked)

rm -rf dist
mkdir -p dist
cp "$target/release/bundle/deb/bigman_${version}_amd64.deb" dist/
cp "$target/release/bundle/rpm/bigman-${version}-1.x86_64.rpm" dist/

name="bigman-${version}-x86_64-linux"
tree="$(mktemp -d)"
trap 'rm -rf "$tree"' EXIT
dir="$tree/bigman-$version"
install -Dm755 -t "$dir/bin" "$target/release/bigman" "$target/release/bigmand" "$target/release/bigman-canary-helper" "$target/release/bigman-app"
install -Dm644 packaging/linux/bigman-app.desktop "$dir/share/applications/bigman-app.desktop"
icons=crates/app/src-tauri/icons
for pair in 32x32:32x32.png 128x128:128x128.png 256x256:128x128@2x.png 512x512:icon.png; do
  install -Dm644 "$icons/${pair#*:}" "$dir/share/icons/hicolor/${pair%%:*}/apps/bigman-app.png"
done
install -Dm755 packaging/tarball/install.sh "$dir/install.sh"
install -Dm644 LICENSE "$dir/LICENSE"
install -Dm644 README.md "$dir/README.md"
tar --sort=name --mtime="@$SOURCE_DATE_EPOCH" --owner=0 --group=0 --numeric-owner \
  -C "$tree" -cf - "bigman-$version" | gzip -9n > "dist/$name.tar.gz"

sum="$(sha256sum "dist/$name.tar.gz" | cut -d' ' -f1)"
sed -e "s/@VERSION@/$version/g" -e "s/@SHA256@/$sum/g" packaging/arch/PKGBUILD.in > dist/PKGBUILD
(cd dist && sha256sum ./*.deb ./*.rpm ./*.tar.gz PKGBUILD | sed 's# \./# #' > SHA256SUMS)
ls -l dist
