# Security model

Bigman is a deterministic Linux security companion, not a trust oracle or a tamper-resistant endpoint agent.

- Inspection reads data and does not run install hooks. Restricted rehearsal is a separate explicit action tied to a saved artifact hash.
- Findings, missing coverage and operational errors remain distinct. Unknown content never produces a “SAFE” verdict.
- Rootless operation is the default. Setup invokes fixed package-manager argument lists through polkit only after an explicit Apply action. Browser requests cannot invoke setup or arbitrary commands.
- Native requests are size-bounded, require known operations, and restrict file access to ordinary downloads. There is no listening network socket.
- Quarantine is explicit and hash-bound. It saves recovery data first. Restoration does not overwrite files or restore execute permissions. Concurrent malicious changes by the same user are outside this boundary; inspect and preserve evidence if you suspect active compromise.
- Local canaries detect supported filesystem events only while their monitor is alive. They do not grant access, prevent reads, identify readers, or prove exfiltration. Deliberate tests and ordinary indexers can trigger them.
- External parser and rehearsal profiles omit host home directories and service sockets. Runtime availability and kernel policy can cause incomplete results. Bubblewrap is not a dedicated malware analysis VM; UID-wide process limits are not per-job cgroup accounting.
- The extension complements browser-native protection. Page warnings and click handling do not cover every navigation or social-engineering method. Feed import and rule exceptions are explicit local choices.
- Reports contain sensitive context such as paths and installer source. No Bigman feature automatically uploads files. Exported reports and optional upstream tools have their own disclosure implications.

For a security issue, provide a minimal synthetic reproduction, affected version, expected behavior, and observed evidence to the repository owner. Do not post live credentials or confidential files in a public issue.

## Dependency maintenance

On 2026-09-08, the updated lockfiles had zero known RustSec vulnerabilities and zero production npm audit vulnerabilities. RustSec still reported four unmaintained transitive dependencies in the desktop stack: `derivative`, `instant`, `paste`, and `ttf-parser`. These maintenance warnings are unresolved and require following upstream replacements. This is a point-in-time dependency check, not a security certification; repeat audits before releases.
