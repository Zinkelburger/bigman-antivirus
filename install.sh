#!/usr/bin/env bash
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
"$cargo_bin" build --locked --release --manifest-path "$project_dir/Cargo.toml" --target-dir "$project_dir/target"
install -Dm755 "$project_dir/target/release/bigman" "$prefix/bin/bigman"
mkdir -p "$prefix/share/applications" "$prefix/share/icons/hicolor/128x128/apps"
install -m644 "$project_dir/browser-extension/icons/128.png" "$prefix/share/icons/hicolor/128x128/apps/bigman.png"
cat > "$prefix/share/applications/bigman.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Name=Bigman Security Companion
Comment=Deterministic scans, install inspection and local canaries
Exec="$prefix/bin/bigman" gui
Icon=bigman
Terminal=false
Categories=Utility;Security;
DESKTOP
printf 'Installed %s/bin/bigman. Launch it from your applications menu.\n' "$prefix"
