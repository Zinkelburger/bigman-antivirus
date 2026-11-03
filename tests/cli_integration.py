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

import hardening_fakes

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

    # Hardening, with stand-in systemctl, pkexec, npm and kwriteconfig6 (see hardening_fakes.py).
    hhome = root / 'harden-home'
    htools = root / 'harden-tools'
    hhome.mkdir()
    henv = dict(env, HOME=str(hhome), XDG_CONFIG_HOME=str(hhome / '.config'), **hardening_fakes.install(htools))
    hardening_fakes.prepare_home(hhome)

    def harden(*args, codes=(0,)):
        return run('harden', *args, codes=codes, raw=True, custom_env=henv).stdout.decode()

    def refused(*args):
        return run('harden', *args, codes=(1, 2), raw=True, custom_env=henv).stderr.decode()

    review = json.loads(harden(codes=(1, 2)))  # lsblk and other tools are absent, so coverage is incomplete
    ids = {s['id']: s for s in review['suggestions']}
    assert {'harden.sshd', 'harden.screen_lock', 'harden.home.readable', 'harden.npm.ignore_scripts', 'harden.firewall.off'} <= ids.keys(), ids.keys()
    assert ids['harden.sshd']['fix'] == {'kind': 'bigman', 'admin': True, 'verb': 'Turn off'}
    assert ids['harden.firewall.off']['fix']['kind'] == 'manual'  # neither firewalld nor ufw is installed
    assert any(s['what'] == 'Disk encryption' for s in review['report']['coverage']['skipped'])

    # A preview changes nothing and names the exact command and who runs it.
    preview = harden('harden.sshd')
    assert 'systemctl disable --now sshd.service' in preview and 'Polkit asks for your password' in preview, preview
    assert hardening_fakes.pkexec_calls(htools) == [] and hardening_fakes.unit(htools, 'sshd.service')['enabled'] == 'enabled'

    # A dismissed password dialog changes nothing and records no undo.
    (htools / 'deny-pkexec').touch()
    assert 'Administrator approval was not given' in refused('harden.sshd', '--apply')
    (htools / 'deny-pkexec').unlink()
    assert hardening_fakes.unit(htools, 'sshd.service')['active'] == 'active'
    assert json.loads(harden(codes=(1, 2)))['applied'] == []

    # Applied through one pkexec call, checked again, and undone the same way.
    assert 'new setting is in place' in harden('harden.sshd', '--apply')
    assert hardening_fakes.pkexec_calls(htools)[-1] == ['systemctl', 'disable', '--now', 'sshd.service']
    assert hardening_fakes.unit(htools, 'sshd.service') == {'load': 'loaded', 'enabled': 'disabled', 'active': 'inactive'}
    after = json.loads(harden(codes=(1, 2)))
    assert 'harden.sshd' not in {s['id'] for s in after['suggestions']}
    assert [a['id'] for a in after['applied']] == ['harden.sshd'] and 'enable --now sshd.service' in after['applied'][0]['undo']
    assert 'Put back' in harden('harden.sshd', '--undo')
    assert hardening_fakes.pkexec_calls(htools)[-1] == ['systemctl', 'enable', '--now', 'sshd.service']
    assert hardening_fakes.unit(htools, 'sshd.service')['active'] == 'active'

    # Changes made as you: the home folder mode, npm's setting and KDE's screen lock.
    harden('harden.home.readable', '--apply')
    assert hhome.stat().st_mode & 0o777 == 0o750
    harden('harden.home.readable', '--undo')
    assert hhome.stat().st_mode & 0o777 == 0o755
    harden('harden.npm.ignore_scripts', '--apply')
    assert (htools / 'npm-ignore-scripts').read_text() == 'true'
    harden('harden.screen_lock', '--apply')
    assert 'Autolock=true' in (hhome / '.config' / 'kscreenlockerrc').read_text()
    harden('harden.screen_lock', '--undo')
    assert 'Autolock=false' in (hhome / '.config' / 'kscreenlockerrc').read_text()
    assert len(hardening_fakes.pkexec_calls(htools)) == 3, 'changes made as you never ask for a password'

    # Leave as it is: still listed, no longer counted; then suggested again.
    harden('harden.screen_lock', '--keep')
    left = json.loads(harden(codes=(1, 2)))
    assert next(s for s in left['suggestions'] if s['id'] == 'harden.screen_lock')['kept']
    assert all(f['rule'] != 'harden.screen_lock' for f in left['report']['findings'])
    harden('harden.screen_lock', '--suggest-again')
    assert not next(s for s in json.loads(harden(codes=(1, 2)))['suggestions'] if s['id'] == 'harden.screen_lock')['kept']

    # Manual steps and unknown ids are refused, and nothing runs.
    assert 'does not make this change itself' in refused('harden.firewall.off', '--apply')
    assert 'no longer applies' in refused('harden.nothing', '--apply')
    assert len(hardening_fakes.pkexec_calls(htools)) == 3
    print('PASS: scan mapping, missing scanner, quarantine/restore, nonexecuting inspection, script policy, reviewed baseline drift protection, real canary alerts, native boundaries, service preview, private history, hardening preview, apply, refusal and undo')
