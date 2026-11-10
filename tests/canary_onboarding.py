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
# The monitor runs inside bigmand, whose socket path must stay short.
runtime = Path(os.environ.get('XDG_RUNTIME_DIR', tempfile.gettempdir()))
with tempfile.TemporaryDirectory(prefix='bigman onboarding ', dir=runtime) as work:
    root = Path(work)
    home = root / 'home'
    home.mkdir()
    state = root / 'state'
    tools = root / 'tools'
    tools.mkdir()
    env = dict(os.environ, HOME=str(home), XDG_CONFIG_HOME=str(home / '.config'), PATH=str(tools), BIGMAN_SOCKET=str(root / 'd.sock'))
    def run(*args, codes=(0,)):
        result = subprocess.run([str(binary), '--state', str(state), '--json', 'canary', *map(str,args)], env=env, capture_output=True, timeout=30)
        assert result.returncode in codes, (args, result.returncode, result.stdout.decode(), result.stderr.decode())
        return json.loads(result.stdout)
    def verified(report):
        return any(f['rule'] == 'canary.verified' for f in report['findings'])
    health_file = state / 'monitor-health.json'
    def daemon_stop():
        subprocess.run([str(binary), '--state', str(state), 'daemon', 'stop'], env=env, capture_output=True, timeout=30)
    def wait_gone(pid):
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline:
            try:
                if Path(f'/proc/{pid}/stat').read_text().rsplit(') ', 1)[1].startswith('Z'): return
            except FileNotFoundError:
                return
            time.sleep(.1)
        raise AssertionError(f'process {pid} is still running')
    cleanup = daemon_stop
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

        # Fixture service manager: bigmand itself is started by Bigman, as the app does.
        systemctl = tools / 'systemctl'
        systemctl.write_text(f'#!{sys.executable}\n')
        systemctl.chmod(0o700)
        automatic = run('arm')
        assert verified(automatic) and not automatic['errors']
        unit_dir = home / '.config/systemd/user'
        assert sorted(p.name for p in unit_dir.iterdir()) == ['bigmand.service']
        assert verified(run('arm'))
        run('stop')
        assert not (unit_dir / 'bigmand.service').exists()
        assert decoy.exists()

        # A stopped monitor stays off when bigmand comes back.
        daemon_stop()
        subprocess.Popen([str(binary.parent / 'bigmand'), '--state', str(state)], env=env, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
        time.sleep(3)
        assert not health_file.exists()

        # The monitor is bigmand's: stopping bigmand ends it, and so does killing it.
        armed = run('arm', '--session-only')
        assert verified(armed)
        pid = json.loads(health_file.read_text())['pid']
        assert 'bigmand' in Path(f'/proc/{pid}/cmdline').read_text()
        daemon_stop()
        wait_gone(pid)
        assert not health_file.exists()
        armed = run('arm', '--session-only')
        assert verified(armed)
        pid = json.loads(health_file.read_text())['pid']
        os.kill(pid, signal.SIGKILL)
        wait_gone(pid)
        stale = run('test', token['id'], codes=(2,))
        assert stale['errors'] and not verified(stale)
        run('stop', '--session-only')

        # A login service from an earlier version moves over to bigmand.
        exe = str(binary).replace('%', '%%').replace('$', '$$')
        st = str(state).replace('%', '%%').replace('$', '$$')
        old = unit_dir / 'bigman-monitor.service'
        old.write_text(f'[Unit]\nDescription=Bigman local canary and persistence monitor\n[Service]\nType=simple\nExecStart="{exe}" --state "{st}" watch\nRestart=on-failure\nRestartSec=5\nNoNewPrivileges=true\nUMask=0077\n[Install]\nWantedBy=default.target\n')
        daemon_stop()
        subprocess.Popen([str(binary.parent / 'bigmand'), '--state', str(state)], env=env, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
        deadline = time.monotonic() + 10
        while not health_file.exists() and time.monotonic() < deadline: time.sleep(.1)
        assert health_file.exists() and not old.exists() and (unit_dir / 'bigmand.service').exists()
        run('stop')

        # Preserve a unit configured for another profile; don't silently claim login coverage.
        unit = unit_dir / 'bigmand.service'
        unit.write_text('unrelated configuration')
        conflict = run('arm', codes=(2,))
        assert verified(conflict) and any('different settings' in e for e in conflict['errors'])
        assert unit.read_text() == 'unrelated configuration'
        stopped_conflict = run('stop', codes=(2,))
        assert stopped_conflict['errors'] and not (state / 'monitor-health.json').exists()
        assert unit.read_text() == 'unrelated configuration'
        print('PASS: account-free local setup, detached monitoring, fresh read proof, idempotence, stopped detection, no overwrite, explicit fallback, isolated login service, safe stop, monitor ends with bigmand')
    finally:
        cleanup()
