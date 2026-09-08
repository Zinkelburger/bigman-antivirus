#!/usr/bin/env python3
"""Hermetic CLI workflows. Uses synthetic files; never installs tools/services."""
import json
import os
from pathlib import Path
import signal
import struct
import subprocess
import sys
import tempfile
import time

binary = Path(sys.argv[1]).resolve()
with tempfile.TemporaryDirectory(prefix='bigman-integration-') as work:
    root = Path(work)
    home = root / 'home'
    home.mkdir()
    downloads = home / 'Downloads'
    downloads.mkdir()
    state = root / 'state'
    tools = root / 'tools'
    tools.mkdir()
    env = dict(os.environ, HOME=str(home), XDG_CONFIG_HOME=str(home / '.config'), PATH=str(tools))
    # A contract fixture implements ClamAV's output/exit conventions, not malware detection.
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

    def run(*args, codes=(0,), raw=False, input=None, custom_env=None):
        result = subprocess.run([str(binary), '--state', str(state), '--json', *map(str,args)], env=custom_env or env, input=input, capture_output=True, timeout=30)
        assert result.returncode in codes, (args, result.returncode, result.stdout.decode(), result.stderr.decode())
        if raw: return result
        return json.loads(result.stdout)

    benign = downloads / 'ordinary.txt'
    benign.write_text('ordinary fixture')
    flagged = downloads / 'flagged.txt'
    flagged.write_text('BIGMAN_TEST_DETECTION')
    report = run('scan', downloads, codes=(1,))
    assert report['errors'] == [] and len(report['findings']) == 1
    inventory = json.loads((state / 'artifacts' / f"{report['id']}.scan.json").read_text())
    index = next(i for i,v in enumerate(inventory) if v['detected'])
    q = run('quarantine', report['id'], index)
    assert benign.exists() and not flagged.exists()
    flagged.write_text('unrelated replacement')
    run('restore', q['id'], codes=(2,), raw=True)
    assert flagged.read_text() == 'unrelated replacement'
    flagged.unlink()
    run('restore', q['id'], raw=True)
    assert flagged.read_text() == 'BIGMAN_TEST_DETECTION'
    assert flagged.stat().st_mode & 0o777 == 0o600
    clam.unlink()
    missing = run('scan', benign, codes=(2,))
    assert missing['errors'] and not missing['findings']

    script = root / 'installer.sh'
    script.write_text('echo preview\ncat ~/.aws/credentials\ntouch MUST_NOT_RUN\n')
    inspected = run('inspect', script, codes=(2,))  # ShellCheck intentionally absent.
    assert any(f['rule'] == 'shell.credentials' for f in inspected['findings'])
    assert not (root / 'MUST_NOT_RUN').exists()
    assert (state / 'artifacts' / f"{inspected['id']}.sh").read_bytes() == script.read_bytes()
    run('sandbox', inspected['id'], '--approve-sha256', 'wrong', codes=(2,), raw=True)

    project = root / 'project'
    project.mkdir()
    (project / 'package.json').write_text(json.dumps({'name':'fixture','version':'1','scripts':{'postinstall':'touch MUST_NOT_RUN'}}))
    packages = run('packages', project, codes=(1,))
    assert any(f['rule'] == 'package.lifecycle' for f in packages['findings'])
    assert not (project / 'MUST_NOT_RUN').exists()
    run('package-policy', project, raw=True)
    assert not (project / '.npmrc').exists()
    run('package-policy', project, '--apply', raw=True)
    assert 'ignore-scripts=true' in (project / '.npmrc').read_text()

    # Baseline acceptance is bound to the exact inventory reviewed, with drift refused.
    startup = home / '.bashrc'
    startup.write_text('echo original startup\n')
    initial = run('changes')
    assert initial['sha256'] and not initial['errors']
    startup.write_text('echo changed after review\n')
    run('changes', '--accept-baseline', initial['id'], codes=(2,), raw=True)
    assert not (state / 'baseline.json').exists()
    reviewed = run('changes')
    accepted = run('changes', '--accept-baseline', reviewed['id'])
    assert accepted['sha256'] == reviewed['sha256']
    assert run('changes')['findings'] == []
    startup.write_text('echo newly changed\n')
    changed = run('changes', codes=(1,))
    assert any(f['rule'] == 'persistence.change' for f in changed['findings'])

    decoy = downloads / 'decoy.env'
    c = run('canary', 'create', decoy)
    run('canary', 'create', decoy, codes=(2,), raw=True)
    monitor = subprocess.Popen([str(binary), '--state', str(state), 'watch'], env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    try:
        deadline = time.monotonic() + 8
        while time.monotonic() < deadline:
            if (state / 'monitor-health.json').exists(): break
            assert monitor.poll() is None, monitor.stderr.read().decode()
            time.sleep(.1)
        health = json.loads((state / 'monitor-health.json').read_text())
        assert health['watched'] == 1
        run('canary', 'test', c['id'], raw=True)
        while time.monotonic() < deadline:
            history = run('history')
            if any(r['kind'] == 'Canary triggered' and r['subject'] == str(decoy) for r in history): break
            time.sleep(.2)
        else: raise AssertionError('Expected durable canary trigger')
        # Decoys are excluded from ordinary AV scans, including outside the state directory.
        assert decoy.exists()
    finally:
        monitor.send_signal(signal.SIGINT)
        monitor.communicate(timeout=5)
    assert not (state / 'monitor-health.json').exists()
    run('canary', 'disarm', c['id'], raw=True)
    assert decoy.exists() and run('canary', 'list') == []

    # Native protocol bounds, state separation, origin-independent local-path restriction.
    def native(payload):
        data = json.dumps(payload).encode()
        out = run('native-host', input=struct.pack('=I',len(data))+data, raw=True).stdout
        size = struct.unpack('=I',out[:4])[0]
        assert size == len(out)-4
        return json.loads(out[4:])
    assert native({'op':'health'})['status'] == 'AVAILABLE'
    assert native({'op':'inspect-url','url':str(script)})['status'] == 'INCOMPLETE'
    assert native({'op':'scan','filePath':str(script)})['status'] == 'INCOMPLETE'
    frame = struct.pack('=I', 0xffffffff)
    out = run('native-host', input=frame, raw=True).stdout
    assert json.loads(out[4:])['status'] == 'INCOMPLETE'

    preview = run('services', raw=True).stdout.decode()
    assert 'bigman-monitor.service' in preview
    assert not (home / '.config/systemd/user/bigman-monitor.service').exists()
    assert all((p.stat().st_mode & 0o077) == 0 for p in (state / 'reports').iterdir())
    print('PASS: scan mapping, missing scanner, quarantine/restore, nonexecuting inspection, script policy, reviewed baseline drift protection, real canary alerts, native boundaries, service preview, private history')
