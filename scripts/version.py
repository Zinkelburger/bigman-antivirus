#!/usr/bin/env python3
"""Checks or sets Bigman's version in every file that carries it.

    scripts/version.py check            every file agrees with Cargo.toml
    scripts/version.py check v1.0.0     ...and with this release tag
    scripts/version.py set 1.0.0        writes the version everywhere

Cargo.toml's [workspace.package] version is the source of truth. The release
workflow runs `check $TAG`, so a tag that does not match the files fails early."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# Plain X.Y.Z: Chrome's manifest takes only dotted numbers, rpm forbids '-' in a
# version and dpkg would sort 1.0.0-rc.1 after 1.0.0. Rehearse a release with the
# release workflow's dry run instead of a pre-release tag.
VERSION = re.compile(r'^\d+\.\d+\.\d+$')
WORKSPACE = re.compile(r'(\[workspace\.package\][^\[]*?\nversion = ")([^"]+)(")', re.S)
MEMBERS = ('bigman-app', 'bigman-cli', 'bigman-core', 'bigman-daemon', 'bigman-native-host')
# JSON files and the key paths inside them that hold the version.
JSON_FILES = {
    'crates/app/src-tauri/tauri.conf.json': [('version',)],
    'crates/app/ui/package.json': [('version',)],
    'crates/app/ui/package-lock.json': [('version',), ('packages', '', 'version')],
    'browser-extension/package.json': [('version',)],
    'browser-extension/package-lock.json': [('version',), ('packages', '', 'version')],
    'browser-extension/manifest.json': [('version',)],
}


def lock_entry(name: str) -> re.Pattern[str]:
    return re.compile(r'(\[\[package\]\]\nname = "' + re.escape(name) + r'"\nversion = ")([^"]+)(")')


def source_version() -> str:
    m = WORKSPACE.search((ROOT / 'Cargo.toml').read_text())
    if not m:
        sys.exit('Cargo.toml has no [workspace.package] version')
    return m.group(2)


def found() -> list[tuple[str, str | None]]:
    """(place, version) for every place the version lives; None when it is missing."""
    out = []
    lock = (ROOT / 'Cargo.lock').read_text()
    for name in MEMBERS:
        m = lock_entry(name).search(lock)
        out.append((f'Cargo.lock {name}', m.group(2) if m else None))
    for rel, paths in JSON_FILES.items():
        data = json.loads((ROOT / rel).read_text())
        for path in paths:
            value = data
            for key in path:
                value = value.get(key) if isinstance(value, dict) else None
            # Joined outside the f-string: before Python 3.12 an f-string
            # expression cannot hold a backslash, and the release build runs
            # this on Ubuntu 22.04 (Python 3.10).
            dotted = '.'.join(k or '""' for k in path)
            out.append((f'{rel} {dotted}', value))
    return out


def check(tag: str | None) -> int:
    want = source_version()
    problems = [f'{place}: {got} (Cargo.toml says {want})' for place, got in found() if got != want]
    if tag is not None and tag != f'v{want}':
        problems.append(f'tag {tag} does not match version {want} (expected v{want})')
    if not VERSION.match(want):
        problems.append(f'{want} is not a version like 1.0.0')
    for p in problems:
        print(p, file=sys.stderr)
    if not problems:
        print(f'version {want} is consistent' + (f' with tag {tag}' if tag else ''))
    return 1 if problems else 0


def set_version(new: str) -> int:
    if not VERSION.match(new):
        sys.exit(f'{new} is not a version like 1.0.0')
    cargo = ROOT / 'Cargo.toml'
    cargo.write_text(WORKSPACE.sub(lambda m: m.group(1) + new + m.group(3), cargo.read_text(), count=1))
    lock_path = ROOT / 'Cargo.lock'
    lock = lock_path.read_text()
    for name in MEMBERS:
        lock, n = lock_entry(name).subn(lambda m: m.group(1) + new + m.group(3), lock)
        if n != 1:
            sys.exit(f'Cargo.lock has {n} entries for {name}')
    lock_path.write_text(lock)
    for rel, paths in JSON_FILES.items():
        # Replace the text in place rather than re-serialising, so formatting stays.
        # The wanted keys are the first "version" entries in each of these files;
        # check() below parses every file again and confirms they changed.
        path_ = ROOT / rel
        text = path_.read_text()
        text, n = re.subn(r'("version":\s*")[^"]*(")', lambda m: m.group(1) + new + m.group(2),
                          text, count=len(paths))
        if n != len(paths):
            sys.exit(f'{rel}: found {n} version entries, expected {len(paths)}')
        path_.write_text(text)
    return check(None)


def main(argv: list[str]) -> int:
    if len(argv) == 2 and argv[1] == 'check':
        return check(None)
    if len(argv) == 3 and argv[1] == 'check':
        return check(argv[2])
    if len(argv) == 3 and argv[1] == 'set':
        return set_version(argv[2])
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == '__main__':
    sys.exit(main(sys.argv))
