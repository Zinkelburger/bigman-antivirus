#!/usr/bin/env bash
# Rasterizes the tray marks (src/design/tray/*.svg) and the application icon
# (browser-extension/icons/BigManAntivirus.png, the owner's logo) into the Tauri
# icons folder. Run after changing either source. Needs ImageMagick.
set -euo pipefail
ui="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
icons="$ui/../src-tauri/icons"
logo="$ui/../../../browser-extension/icons/BigManAntivirus.png"
mkdir -p "$icons"
for state in quiet attention problem; do
  magick -background none -density 384 "$ui/src/design/tray/tray-$state.svg" -resize 32x32 "$icons/tray-$state.png"
done
# The logo source is 180 px; larger sizes are upscaled until a vector logo exists.
magick "$logo" -background none -gravity center -extent 180x180 -resize 32x32 "$icons/32x32.png"
magick "$logo" -background none -resize 128x128 "$icons/128x128.png"
magick "$logo" -background none -resize 256x256 "$icons/128x128@2x.png"
magick "$logo" -background none -resize 512x512 "$icons/icon.png"
echo "Rendered icons into $icons"
