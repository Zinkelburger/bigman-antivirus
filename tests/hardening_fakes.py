"""Stand-in system tools for the hardening tests, so `bigman harden` runs
hermetically and its administrator changes are exercised without root.

- systemctl keeps unit states in a JSON file: sshd starts enabled and running,
  every other unit is not installed. `enable/disable --now` update the file.
- pkexec logs each command line, then runs it only when the program is one of
  these fakes; anything else (a script that would write /etc) is logged and not
  run. Creating `deny-pkexec` in the tools folder makes it exit 126, as when the
  password dialog is dismissed.
- npm 10 keeps `ignore-scripts` in a file; kwriteconfig6 edits the private
  kscreenlockerrc; dbus-send does nothing.
"""
import json
import os
from pathlib import Path
import sys

SYSTEMCTL = r'''
import json, pathlib, sys
state = pathlib.Path(__file__).with_name('units.json')
units = json.loads(state.read_text())
args = sys.argv[1:]
if args[:2] == ['--user', 'show-environment'] or 'display-manager.service' in args:
    sys.exit(0)
if args and args[0] == 'show':
    names = [a for a in args[1:] if not a.startswith('-') and '=' not in a and ',' not in a]
    blocks = []
    for name in names:
        u = units.get(name, {'load': 'not-found', 'enabled': '', 'active': 'inactive'})
        blocks.append(f"Id={name}\nLoadState={u['load']}\nUnitFileState={u['enabled']}\nActiveState={u['active']}")
    print('\n\n'.join(blocks))
    sys.exit(0)
if args[:1] in (['enable'], ['disable']) and '--now' in args:
    on = args[0] == 'enable'
    for name in args[1:]:
        if name in units:
            units[name].update(enabled='enabled' if on else 'disabled', active='active' if on else 'inactive')
    state.write_text(json.dumps(units))
    sys.exit(0)
sys.exit(1)
'''

PKEXEC = r'''
import json, os, pathlib, shutil, sys
here = pathlib.Path(__file__).parent
with open(here / 'pkexec.log', 'a') as log:
    log.write(json.dumps(sys.argv[1:]) + '\n')
if (here / 'deny-pkexec').exists():
    sys.exit(126)
program = shutil.which(sys.argv[1])
if program and pathlib.Path(program).parent == here:
    os.execv(program, sys.argv[1:])
'''

NPM = r'''
import pathlib, sys
setting = pathlib.Path(__file__).with_name('npm-ignore-scripts')
args = sys.argv[1:]
if args == ['--version']:
    print('10.9.8')
elif args[:3] == ['config', 'get', 'ignore-scripts']:
    print(setting.read_text() if setting.exists() else 'false')
elif args[:4] == ['config', 'set', 'ignore-scripts', 'true']:
    setting.write_text('true')
elif args[:3] == ['config', 'delete', 'ignore-scripts']:
    setting.unlink(missing_ok=True)
else:
    sys.exit(1)
'''

KWRITECONFIG = r'''
import os, pathlib, sys
args = sys.argv[1:]
opt = lambda name: args[args.index(name) + 1]
path = pathlib.Path(os.environ['XDG_CONFIG_HOME']) / opt('--file')
group, key = opt('--group'), opt('--key')
lines = path.read_text().splitlines() if path.exists() else []
out, section = [], None
for line in lines:
    if line.startswith('['):
        section = line.strip('[]')
    elif section == group and line.split('=', 1)[0] == key:
        continue
    out.append(line)
if '--delete' not in args:
    value = args[-1]
    if f'[{group}]' not in out:
        out.append(f'[{group}]')
    out.insert(out.index(f'[{group}]') + 1, f'{key}={value}')
path.write_text('\n'.join(out) + '\n')
'''


def install(tools: Path) -> dict:
    """Writes the fakes into `tools` and returns the environment additions."""
    tools.mkdir(parents=True, exist_ok=True)
    (tools / 'units.json').write_text(json.dumps({'sshd.service': {'load': 'loaded', 'enabled': 'enabled', 'active': 'active'}}))
    for name, body in [('systemctl', SYSTEMCTL), ('pkexec', PKEXEC), ('npm', NPM), ('kwriteconfig6', KWRITECONFIG), ('dbus-send', '')]:
        script = tools / name
        script.write_text(f'#!{sys.executable}\n{body}')
        script.chmod(0o700)
    # The home-folder change runs chmod itself; give it the real one.
    os.symlink('/usr/bin/chmod' if Path('/usr/bin/chmod').exists() else '/bin/chmod', tools / 'chmod')
    return {'PATH': str(tools), 'XDG_CURRENT_DESKTOP': 'KDE'}


def prepare_home(home: Path) -> None:
    """A home folder others can open, npm in use, and a screen that never locks."""
    (home / '.npm').mkdir(parents=True, exist_ok=True)
    config = home / '.config'
    config.mkdir(parents=True, exist_ok=True)
    (config / 'kscreenlockerrc').write_text('[Daemon]\nAutolock=false\n')
    home.chmod(0o755)


def pkexec_calls(tools: Path) -> list:
    log = tools / 'pkexec.log'
    return [json.loads(line) for line in log.read_text().splitlines()] if log.exists() else []


def unit(tools: Path, name: str) -> dict:
    return json.loads((tools / 'units.json').read_text())[name]
