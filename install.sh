#!/usr/bin/env bash
# Builds and installs bigman (CLI), bigmand (daemon) and bigman-app (desktop app)
# under a user prefix. Installs nothing system-wide and starts nothing.
set -euo pipefail
project_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
prefix="${1:-$HOME/.local}"
if [[ $prefix != /* || $prefix == *$'\n'* || $prefix == *$'\r'* || $prefix == *'%'* || $prefix == *'"'* || $prefix == *'`'* || $prefix == *'$'* || $prefix == *'\'* ]]; then
  echo 'Use an absolute installation prefix without desktop-file metacharacters.' >&2
  exit 2
fi
cargo_bin="$(command -v cargo || true)"
if [[ -z "$cargo_bin" && -x "$HOME/.cargo/bin/cargo" ]]; then cargo_bin="$HOME/.cargo/bin/cargo"; fi
if [[ -z "$cargo_bin" ]]; then echo 'Install a Rust toolchain first.' >&2; exit 2; fi
if ! command -v npm >/dev/null; then echo 'Install Node.js and npm to build the desktop app.' >&2; exit 2; fi
if ! pkg-config --exists webkit2gtk-4.1 2>/dev/null; then
  echo 'The desktop app needs WebKitGTK development files; see "Desktop app" in docs/development.md.' >&2
  exit 2
fi
(cd "$project_dir/crates/app/ui" && npm ci --ignore-scripts --no-audit --no-fund && npm run build)
"$cargo_bin" build --locked --release --manifest-path "$project_dir/Cargo.toml" --target-dir "$project_dir/target" \
  -p bigman-cli -p bigman-daemon -p bigman-app
for bin in bigman bigmand bigman-app; do
  install -Dm755 "$project_dir/target/release/$bin" "$prefix/bin/$bin"
done
mkdir -p "$prefix/share/applications" "$prefix/share/icons/hicolor/128x128/apps"
install -m644 "$project_dir/browser-extension/icons/128.png" "$prefix/share/icons/hicolor/128x128/apps/bigman.png"
# Named after the window's application ID (the program name), so Wayland desktops
# give the window and its taskbar entry this icon. Earlier versions wrote bigman.desktop.
rm -f "$prefix/share/applications/bigman.desktop"
cat > "$prefix/share/applications/bigman-app.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Name=Bigman
Comment=Deterministic security checks for your Linux desktop
Exec="$prefix/bin/bigman-app"
Icon=bigman
Terminal=false
Categories=Utility;Security;
DESKTOP
printf 'Installed %s/bin/bigman, bigmand and bigman-app. Launch Bigman from your applications menu.\n' "$prefix"
