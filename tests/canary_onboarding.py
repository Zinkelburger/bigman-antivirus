#!/usr/bin/env python3
"""Real local canary onboarding; service manager is a fixture, no user settings changed."""
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time

binary = Path(sys.argv[1]).resolve()
with tempfile.TemporaryDirectory(prefix='bigman onboarding ') as work:
    root = Path(work)
    home = root / 'home'
    home.mkdir()
    state = root / 'state'
    tools = root / 'tools'
    tools.mkdir()
    env = dict(os.environ, HOME=str(home), XDG_CONFIG_HOME=str(home / '.config'), PATH=str(tools))
    def run(*args, codes=(0,)):
        result = subprocess.run([str(binary), '--state', str(state), '--json', 'canary', *map(str,args)], env=env, capture_output=True, timeout=30)
        assert result.returncode in codes, (args, result.returncode, result.stdout.decode(), result.stderr.decode())
        return json.loads(result.stdout)
    def verified(report):
        return any(f['rule'] == 'canary.verified' for f in report['findings'])
    def cleanup():
        try:
            pid = json.loads((state / 'monitor-health.json').read_text())['pid']
            os.kill(pid, signal.SIGINT)
            deadline = time.monotonic() + 5
            while (state / 'monitor-health.json').exists() and time.monotonic() < deadline:
                time.sleep(.1)
        except (FileNotFoundError, ProcessLookupError):
            pass
    try:
        # Default local path, detached process survives CLI exit, fresh read receipt.
        first = run('arm', '--session-only')
        assert verified(first) and not first['errors']
        # PATH has no notify-send here, so alerts are History-only and the report says so.
        assert any(f['rule'] == 'canary.notifications-unavailable' for f in first['findings'])
        canaries = run('list')
        assert len(canaries) == 1
        token = canaries[0]
        decoy = Path(token['path'])
        assert decoy == home / 'backup-access.env'
        assert decoy.stat().st_mode & 0o777 == 0o600
        receipt = state / 'canaries' / f"{token['id']}.read.json"
        previous = json.loads(receipt.read_text())
        again = run('arm', '--session-only')
        assert verified(again) and len(run('list')) == 1
        latest = json.loads(receipt.read_text())
        assert latest['id'] != previous['id'] and latest['monitor_pid'] == previous['monitor_pid']
        assert not (home / '.config/systemd').exists()
        run('stop', '--session-only')
        assert not (state / 'monitor-health.json').exists()
        assert 'bigman_decoy_' in decoy.read_text()
        stopped = run('test', token['id'], codes=(2,))
        assert stopped['errors'] and not verified(stopped)

        unrelated = home / 'existing.env'
        unrelated.write_text('preserve this file')
        rejected = run('arm', unrelated, '--session-only', codes=(2,))
        assert rejected['errors'] and unrelated.read_text() == 'preserve this file'
        assert len(run('list')) == 1

        resumed = run('start', '--session-only')
        assert not resumed['errors'] and len(run('list')) == 1
        assert verified(run('test', token['id']))
        run('stop', '--session-only')

        # No systemctl: local detection still works, but login startup is explicitly incomplete.
        fallback = run('arm', codes=(2,))
        assert verified(fallback) and any('Automatic startup unavailable' in e for e in fallback['errors'])
        run('stop')

        # Fixture service manager launches the real monitor from the generated unit.
        systemctl = tools / 'systemctl'
        systemctl.write_text(f'''#!{sys.executable}
import json,os,pathlib,shlex,signal,subprocess,sys,time
unit=pathlib.Path(os.environ['XDG_CONFIG_HOME'])/'systemd/user/bigman-monitor.service'
args=sys.argv[1:]
if 'enable' in args and '--now' in args:
 line=next(l for l in unit.read_text().splitlines() if l.startswith('ExecStart='))
 command=[s.replace('%%','%').replace('$$','$') for s in shlex.split(line.split('=',1)[1])]
 subprocess.Popen(command,stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,start_new_session=True)
if 'disable' in args:
 line=next(l for l in unit.read_text().splitlines() if l.startswith('ExecStart='))
 command=shlex.split(line.split('=',1)[1])
 health=pathlib.Path(command[command.index('--state')+1])/'monitor-health.json'
 if health.exists():
  try: os.kill(json.loads(health.read_text())['pid'],signal.SIGINT)
  except ProcessLookupError: pass
  for _ in range(50):
   if not health.exists(): break
   time.sleep(.1)
''')
        systemctl.chmod(0o700)
        automatic = run('arm')
        assert verified(automatic) and not automatic['errors']
        unit_dir = home / '.config/systemd/user'
        assert sorted(p.name for p in unit_dir.iterdir()) == ['bigman-monitor.service']
        assert verified(run('arm'))
        run('stop')
        assert not (unit_dir / 'bigman-monitor.service').exists()
        assert decoy.exists()

        # Preserve a unit configured for another profile; don't silently claim login coverage.
        unit = unit_dir / 'bigman-monitor.service'
        unit.write_text('unrelated configuration')
        conflict = run('arm', codes=(2,))
        assert verified(conflict) and any('different settings' in e for e in conflict['errors'])
        assert unit.read_text() == 'unrelated configuration'
        stopped_conflict = run('stop', codes=(2,))
        assert stopped_conflict['errors'] and not (state / 'monitor-health.json').exists()
        assert unit.read_text() == 'unrelated configuration'
        print('PASS: account-free local setup, detached monitoring, fresh read proof, idempotence, stopped detection, no overwrite, explicit fallback, isolated login service, safe stop')
    finally:
        cleanup()
