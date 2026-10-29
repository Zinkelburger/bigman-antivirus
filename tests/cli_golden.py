#!/usr/bin/env python3
"""Golden output parity for the CLI. Hermetic: synthetic inputs, empty PATH, private HOME.

Run with --update to rewrite the golden files after an intentional output change."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile

binary = Path(sys.argv[1]).resolve()
update = '--update' in sys.argv
here = Path(__file__).resolve().parent / 'golden'
inputs = here / 'inputs'
UUID = re.compile(r'[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}')


def normalize(text, root):
    text = text.replace(str(root), '<root>').replace(str(inputs), '<inputs>')
    text = UUID.sub('<uuid>', text)
    return re.sub(r'"created": \d+', '"created": 0', text)


with tempfile.TemporaryDirectory(prefix='bigman-golden-') as work:
    root = Path(work)
    home = root / 'home'
    (home / 'Downloads').mkdir(parents=True)
    (home / '.bashrc').write_text('echo golden startup fixture\n')
    tools = root / 'tools'
    tools.mkdir()
    env = dict(os.environ, HOME=str(home), XDG_CONFIG_HOME=str(home / '.config'),
               XDG_DATA_HOME=str(home / '.local/share'), PATH=str(tools))
    cases = [
        ('doctor.json', ['--json', 'doctor'], None),
        ('inspect.txt', ['inspect', inputs / 'installer.sh'], None),
        ('inspect.json', ['--json', 'inspect', inputs / 'installer.sh'], None),
        ('stdin.txt', [], (inputs / 'installer.sh').read_bytes()),
        ('packages.json', ['--json', 'packages', inputs / 'project'], None),
    ]
    failures = []
    for name, args, stdin in cases:
        result = subprocess.run([str(binary), '--state', str(root / 'state'), *map(str, args)],
                                env=env, input=stdin, capture_output=True, timeout=60)
        actual = f'exit {result.returncode}\n' + normalize(result.stdout.decode(), root)
        golden = here / name
        if update:
            golden.write_text(actual)
        elif not golden.exists() or golden.read_text() != actual:
            failures.append(name)
            (root / name).write_text(actual)
            subprocess.run(['diff', '-u', str(golden), str(root / name)])
    if failures:
        sys.exit(f'FAIL: golden output differs for {", ".join(failures)}')
    print('UPDATED' if update else 'PASS', ', '.join(name for name, _, _ in cases))
