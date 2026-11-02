#!/usr/bin/env python3
"""The desktop app's workflows against a real bigmand over its socket.

Hermetic like cli_integration.py: synthetic files, a ClamAV contract fixture,
private HOME, and no service manager. Usage: daemon_integration.py target/debug"""
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import time

bin_dir = Path(sys.argv[1]).resolve()
runtime = Path(os.environ.get('XDG_RUNTIME_DIR', tempfile.gettempdir()))
with tempfile.TemporaryDirectory(prefix='bigman-daemon-', dir=runtime) as work:
    root = Path(work)
    home = root / 'home'
    downloads = home / 'Downloads'
    downloads.mkdir(parents=True)
    tools = root / 'tools'
    tools.mkdir()
    sock = root / 'run' / 'bigmand.sock'
    env = dict(os.environ, HOME=str(home), XDG_CONFIG_HOME=str(home / '.config'), PATH=str(tools), BIGMAN_SOCKET=str(sock))
    clam = tools / 'clamscan'
    clam.write_text(f'''#!{sys.executable}
import pathlib,sys
files=pathlib.Path(sys.argv[sys.argv.index('--file-list')+1]).read_text().splitlines()
detected=False
for file in files:
 if b'BIGMAN_TEST_DETECTION' in pathlib.Path(file).read_bytes():
  print(file+': Bigman.Test.Fixture FOUND');detected=True
print('Scanned files: '+str(len(files)))
sys.exit(1 if detected else 0)
''')
    clam.chmod(0o700)

    daemon = subprocess.Popen([str(bin_dir / 'bigmand'), '--state', str(root / 'state')], env=env, stderr=subprocess.PIPE)
    try:
        deadline = time.monotonic() + 10
        while not sock.exists():
            assert daemon.poll() is None, daemon.stderr.read().decode()
            assert time.monotonic() < deadline, 'bigmand did not create its socket'
            time.sleep(0.05)

        counter = [0]

        def ask(request):
            """One request on its own connection, as the desktop app does."""
            counter[0] += 1
            with socket.socket(socket.AF_UNIX) as s:
                s.settimeout(60)
                s.connect(str(sock))
                s.sendall((json.dumps({'id': counter[0], 'request': request}) + '\n').encode())
                data = b''
                while not data.endswith(b'\n'):
                    chunk = s.recv(65536)
                    assert chunk, 'daemon closed the connection'
                    data += chunk
            frame = json.loads(data)
            # A frame the daemon cannot parse is answered with id 0.
            assert frame['id'] in (counter[0], 0), frame
            return frame['response']

        def value(request, kind):
            response = ask(request)
            assert response['type'] == kind, (request, response)
            return response.get('value')

        # Scan, preview a quarantine (changes nothing), apply it, restore.
        (downloads / 'ordinary.txt').write_text('ordinary fixture')
        flagged = downloads / 'flagged.txt'
        flagged.write_text('BIGMAN_TEST_DETECTION')
        scan = value({'type': 'scan', 'path': str(downloads)}, 'report')
        assert scan['errors'] == [] and len(scan['findings']) == 1, scan
        items = value({'type': 'scan_items', 'report': scan['id']}, 'scan_items')
        index = next(i for i, item in enumerate(items) if item['detected'])
        action = {'kind': 'quarantine', 'report': scan['id'], 'index': index}
        plan = value({'type': 'preview', 'action': action}, 'plan')
        assert plan['verb'] == 'Quarantine' and str(flagged) in plan['details'] and flagged.exists()
        done = value({'type': 'apply', 'action': action}, 'done')
        assert not flagged.exists() and 'recovery copy is saved' in done['message']
        quarantined = value({'type': 'quarantined'}, 'quarantined')
        assert len(quarantined) == 1 and quarantined[0]['original'] == str(flagged)
        # The outcome carries its own undo: restoring this exact quarantine entry.
        assert done['undo'] == {'kind': 'restore', 'id': quarantined[0]['id']}
        value({'type': 'apply', 'action': done['undo']}, 'done')
        assert flagged.read_text() == 'BIGMAN_TEST_DETECTION'

        # The inspector is chosen from the target: a folder lists package scripts.
        routed = value({'type': 'inspect_any', 'target': str(downloads)}, 'report')
        assert routed['kind'] == 'Package script inventory', routed['kind']

        # Inspection never runs the script; the full report is available by id.
        script = root / 'installer.sh'
        script.write_text('echo preview\ncat ~/.aws/credentials\ntouch MUST_NOT_RUN\n')
        inspected = value({'type': 'inspect', 'target': str(script)}, 'report')
        assert any(f['rule'] == 'shell.credentials' for f in inspected['findings'])
        assert not (root / 'MUST_NOT_RUN').exists()
        assert value({'type': 'report', 'id': inspected['id']}, 'report')['details'].startswith('echo preview')
        error = ask({'type': 'preview', 'action': {'kind': 'rehearse', 'report': inspected['id'], 'sha256': 'wrong'}})
        assert error['type'] == 'plan'  # A preview only describes; the hash is checked on apply.
        assert ask({'type': 'apply', 'action': {'kind': 'rehearse', 'report': inspected['id'], 'sha256': 'wrong'}})['type'] == 'error'

        # Package scripts and the ignore-scripts policy, preview first.
        project = root / 'project'
        project.mkdir()
        (project / 'package.json').write_text(json.dumps({'name': 'fixture', 'version': '1', 'scripts': {'postinstall': 'touch MUST_NOT_RUN'}}))
        packages = value({'type': 'packages', 'path': str(project)}, 'report')
        assert any(f['rule'] == 'package.lifecycle' for f in packages['findings'])
        value({'type': 'preview', 'action': {'kind': 'package_policy', 'path': str(project)}}, 'plan')
        assert not (project / '.npmrc').exists()
        value({'type': 'apply', 'action': {'kind': 'package_policy', 'path': str(project)}}, 'done')
        assert 'ignore-scripts=true' in (project / '.npmrc').read_text()

        # Startup baseline: compare, accept exactly what was reviewed, then no changes.
        (home / '.bashrc').write_text('echo original startup\n')
        reviewed = value({'type': 'changes'}, 'report')
        value({'type': 'preview', 'action': {'kind': 'accept_baseline', 'report': reviewed['id']}}, 'plan')
        value({'type': 'apply', 'action': {'kind': 'accept_baseline', 'report': reviewed['id']}}, 'done')
        assert value({'type': 'changes'}, 'report')['findings'] == []

        # A session-only canary with a real monitor, verified by a fresh read.
        decoy = home / 'decoy.env'
        armed = value({'type': 'apply', 'action': {'kind': 'arm_canary', 'path': str(decoy), 'session_only': True}}, 'done')
        assert any(f['rule'] == 'canary.verified' for f in armed['report']['findings']), armed
        state = value({'type': 'canaries'}, 'canaries')
        assert state['monitor'] and len(state['canaries']) == 1
        canary_id = state['canaries'][0]['id']
        tested = value({'type': 'canary_test', 'id': canary_id}, 'report')
        assert any(f['rule'] == 'canary.verified' for f in tested['findings'])
        value({'type': 'apply', 'action': {'kind': 'stop_monitor', 'session_only': True}}, 'done')
        value({'type': 'apply', 'action': {'kind': 'disarm_canary', 'id': canary_id}}, 'done')
        assert decoy.exists() and value({'type': 'canaries'}, 'canaries')['canaries'] == []

        # History lists what ran, newest first; unknown requests are refused.
        history = value({'type': 'history', 'limit': 50}, 'history')
        assert {r['kind'] for r in history} >= {'File scan', 'Installer inspection', 'Persistence changes'}
        assert ask({'type': 'run_shell'})['type'] == 'error'
        print('PASS: scan, previewed quarantine and restore, nonexecuting inspection, package policy, reviewed baseline, verified canary, history over the daemon socket')
    finally:
        daemon.terminate()
        daemon.wait(timeout=10)
