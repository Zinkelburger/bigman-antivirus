#!/usr/bin/env bash
set -euo pipefail
if [[ $# != 1 || ! $1 =~ ^[a-p]{32}$ ]]; then
  echo 'Usage: install-native-messaging.sh CHROME_EXTENSION_ID (32 letters a-p)' >&2
  exit 2
fi
project_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
bigman_bin="${BIGMAN_BIN:-$HOME/.local/bin/bigman}"
if [[ ! -x "$bigman_bin" ]]; then
  bigman_bin="$project_dir/target/release/bigman"
fi
if [[ ! -x "$bigman_bin" ]]; then
  echo 'Build/install Bigman first with ./install.sh, then retry.' >&2
  exit 2
fi
exec "$bigman_bin" native-install "$1"
