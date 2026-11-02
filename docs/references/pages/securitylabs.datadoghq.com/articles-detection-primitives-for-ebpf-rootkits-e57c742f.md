---
url: "https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/"
title: "Detection primitives for eBPF rootkits | Datadog Security Labs"
published: "2026-07-27T00:00:00Z"
captured: "2026-11-02T18:43:52Z"
via: "live"
sha256: "0284bd173428ef71b34d2adc2409c1edf089d058ba559dc592ae12ace3768e39"
wayback: "https://web.archive.org/web/20261005154355/https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Detection primitives for eBPF rootkits | Datadog Security Labs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/>
> Snapshot: <https://web.archive.org/web/20261005154355/https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/>

[Lorenzo Susini

Security Engineer II](https://securitylabs.datadoghq.com/articles/?author=Lorenzo_Susini) [Matt Muir

Senior Security Engineer](https://securitylabs.datadoghq.com/articles/?author=Matt_Muir)

## [Introduction - In the wild](https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/#introduction-in-the-wild)

Over the past couple of years, Linux malware authors have started using eBPF rootkits to hide from both traditional and modern defenses. Samples found in the wild bypass tools that have long been considered hard to evade, such as ss, as well as tools that assume attackers can't touch kernel-level introspection, such as bpftool. They also defeat common tracing primitives like ptrace. [LinkPro](https://www.synacktiv.com/en/publications/linkpro-ebpf-rootkit-analysis), [VoidLink](https://research.checkpoint.com/2026/voidlink-the-cloud-native-malware-framework/), and the more recent [Atomic Arch](https://www.sonatype.com/blog/atomic-arch-npm-campaign-adds-malicious-dependency) campaign all use these approaches.

Rather than describe each malware family in full, we picked one instructive feature from each and went deep, looking at how the technique works, what data defenders should focus on, and what detections we built from it. We start with VoidLink, which found an unexpectedly clean way to hide active connections from ss by turning a debug helper into a precise memory editor.

### [VoidLink: A creative use of bpf_probe_write_user()](https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/#voidlink-a-creative-use-of-bpf-probe-write-user)

The bpf_probe_write_user() helper overwrites user space memory from inside an eBPF program. [The documentation](https://docs.ebpf.io/linux/helper-function/bpf_probe_write_user/) describes it as a tool to "debug, divert, and manipulate execution of semi-cooperative processes." That makes it an appealing primitive for a rootkit, and VoidLink's authors used it to bypass ss.

The ss utility retrieves open-socket statistics through the Netlink subsystem. When running ss -tn to show active TCP connections, three system calls do the work:

- socket(AF_NETLINK, SOCK_RAW, NETLINK_INET_DIAG) opens a raw Netlink socket. AF_NETLINK tells the kernel this is an IPC channel to a kernel subsystem, not a network socket.
- sendmsg(fd, request) sends a structured request to the kernel. The message has two parts: (1) a Netlink header specifying the message type (SOCK_DIAG_BY_FAMILY) and flags (NLM_F_DUMP) to request a full dump, followed by (2) an inet_diag_req_v2 body. This body sets three fields: the family (AF_INET), the protocol (IPPROTO_TCP), and a socket ID, left zeroed so that it acts as a wildcard matching all sockets.
- recvmsg(fd, buffer) reads the kernel's response: a multipart stream of inet_diag_msg records, one per socket, terminated by NLMSG_DONE. Each record holds the source/destination IP and port, the TCP state, the owning UID, and the inode number.

The kernel structures these responses as a chain of TLV (type, length, value) messages: Each message starts with an nlmsghdr header carrying the type and the length, followed immediately by the payload. User space parsers walk this chain with macros like NLMSG_OK and NLMSG_NEXT, which use the length field as a cursor to advance from one message to the next.

VoidLink needs to hide connections on a specific port, a standard rootkit feature. It places a kprobe on __sys_recvmsg's entry point to capture the user space buffer pointer before the syscall runs, reading the pointer from CPU registers at the only point where the syscall arguments are available. A paired kretprobe fires after the kernel fills that buffer with socket data, but before user space reads it. At that point, bpf_probe_write_user tampers with the buffer in place: It inflates the length field of the message before the one to hide, so the inflated length swallows the hidden message entirely and the parser's cursor jumps straight over it.

[Before and after bpf_probe_write_user() tampering: nlmsg A's length field is inflated so NLMSG_NEXT jumps straight from A to C, skipping the hidden message B entirely (click to enlarge)](https://securitylabs.dd-static.net/img/detection-primitives-for-ebpf-rootkits/voidlink-nlmsg-buffer-tampering.png?auto=format)

NLMSG_NEXT advances the cursor by NLMSG_ALIGN(h->nlmsg_len). With A.nlmsg_len=112, the cursor jumps from position 0 straight to position 112, landing on message C. Message B, the one carrying the port the rootkit wants to hide, is skipped entirely.

This trick doesn't break ss's parsing, because NLMSG_OK checks the only condition that matters at this point: that nlmsg_len <= len, where len is the remaining byte count in the buffer. VoidLink inflates A's length by exactly NLMSG_ALIGN(B.nlmsg_len), the size of the bytes physically present in the buffer, so the inflated length always fits within what remains. NLMSG_OK keeps returning true, NLMSG_NEXT lands on the correct next message, and the parser never reads out of bounds or sees an inconsistency. From ss's perspective, message A is just a little longer than usual.

### [LinkPro: The power of bpf_override_return()](https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/#linkpro-the-power-of-bpf-override-return)

bpf_override_return() replaces the return value of a kernel function from inside an eBPF program. The kernel scopes this tightly: The helper is only callable from kprobe-type programs attached to functions marked ALLOW_ERROR_INJECTION in the kernel source, and the host kernel must be built with CONFIG_BPF_KPROBE_OVERRIDE=y. Under those conditions, when the kprobe fires, the helper writes the chosen value into the saved return register and skips the original function entirely. To user space, it looks as though the kernel itself produced that result. The kernel team designed this helper for fault injection in bpf_trace-style testing, but it's an obvious primitive for any rootkit that wants the kernel to lie on its behalf.

LinkPro uses bpf_override_return() to hide its own eBPF programs from enumeration. The malware's Hide module attaches a tracepoint on sys_enter_bpf (handleBpfEnter) that watches for three enumeration commands—BPF_PROG_GET_NEXT_ID (11), BPF_MAP_GET_NEXT_ID (12), and BPF_LINK_GET_NEXT_ID (31)—and stashes the call context in a map keyed by pid_tgid. A paired kretprobe on __x64_sys_bpf (handleBpfExit) fires on the return path, retrieves the stash, reads the next_id the kernel is about to hand back to user space, and checks it against main_ebpf_progs, an allowlist the userspace component populates with LinkPro's own program IDs. On a match, it calls bpf_override_return(ctx, -ENOENT). The caller (bpftool, libbpf, or anything else walking the program list) sees -ENOENT and concludes there are no more entries.

That detail matters for detection engineers. -ENOENT is the natural terminator of a GET_NEXT_ID walk, so returning it at any point in the iteration stops the enumerator rather than skipping past one entry. We confirmed this on a live kernel: We loaded the module, populated main_ebpf_progs with a single benign program ID, ran bpftool prog list, and watched the listing truncate at that ID. Every program with a higher ID also disappeared, regardless of whether it was on the allowlist. The mechanism of the Hide module is stronger than the authors likely intended, and it leaves a clear detection signal: A defender who compares the enumerated set against bpftool prog show id N (which uses BPF_PROG_GET_FD_BY_ID, a call this hook doesn't touch) will spot the discrepancy.

LinkPro also leaves behind user space artifacts that look unintentional. Both handleBpfEnter and handleBpfExit call bpf_printk with "BPF cmd: %d, start_id: %u" and "HIDING NEXT_ID: %u", and that output streams to /sys/kernel/debug/tracing/trace_pipe in plain text on every intercepted syscall. Defenders and security software can spot this easily, but they may treat it as a leftover from malware developers and shouldn't be relied upon as the primary detection mechanism.

```
---------------------------------------------------------------------------
kretprobe___x64_sys_bpf:0000000000002050
kretprobe___x64_sys_bpf:0000000000002050 LBB5_10:                                ; CODE XREF: handleBpfExit+138up j
kretprobe___x64_sys_bpf:0000000000002050                 mov            r2, r10
kretprobe___x64_sys_bpf:0000000000002058                 add            r2, -0xC
kretprobe___x64_sys_bpf:0000000000002060                 lddw           r1, 0
kretprobe___x64_sys_bpf:0000000000002070                 call           1
kretprobe___x64_sys_bpf:0000000000002078                 jeq            r0, 0, LBB5_9
kretprobe___x64_sys_bpf:0000000000002080                 ldxb           r1, [r0]
kretprobe___x64_sys_bpf:0000000000002088                 jne            r1, 1, LBB5_9
kretprobe___x64_sys_bpf:0000000000002090                 ldxw           r3, [r10-0xC]
kretprobe___x64_sys_bpf:0000000000002098                 lddw           r1, 26
kretprobe___x64_sys_bpf:00000000000020A8                 mov            r2, 19
kretprobe___x64_sys_bpf:00000000000020B0                 call           6        ; bpf_trace_printk("HIDING NEXT_ID: %u", 19, next_id): debug log confirming suppression;
kretprobe___x64_sys_bpf:00000000000020B0                                         ; visible in /sys/kernel/debug/tracing/trace_pipe
kretprobe___x64_sys_bpf:00000000000020B8                 mov            r1, r6
kretprobe___x64_sys_bpf:00000000000020C0                 mov            r2, -2   ; -ENOENT: the injected return value;
kretprobe___x64_sys_bpf:00000000000020C0                                         ; tells the caller the next_id does not exist
kretprobe___x64_sys_bpf:00000000000020C8                 call           58       ; call bpf_override_return(ctx, -ENOENT):
kretprobe___x64_sys_bpf:00000000000020C8                                         ; overwrites sys_bpf return value in pt_regs before userspace sees it.
kretprobe___x64_sys_bpf:00000000000020C8                                         ; Requires CONFIG_BPF_KPROBE_OVERRIDE.
kretprobe___x64_sys_bpf:00000000000020C8                                         ; Any tool calling BPF_PROG_GET_NEXT_ID/BPF_MAP_GET_NEXT_ID/BPF_LINK_GET_NEXT_ID
kretprobe___x64_sys_bpf:00000000000020C8                                         ; that receives this next_id will get -ENOENT and skip it, hiding the program from bpftool and libbpf enumeration.
kretprobe___x64_sys_bpf:00000000000020D0                 ja             LBB5_13
kretprobe___x64_sys_bpf:00000000000020D0 ; End of function handleBpfExit
kretprobe___x64_sys_bpf:00000000000020D0
kretprobe___x64_sys_bpf:00000000000020D0 ; end of 'kretprobe___x64_sys_bpf'
kretprobe___x64_sys_bpf:00000000000020D0
license:00000000000020D8 ; ===========================================================================

```

Annotated disassembly of LinkPro's handleBpfExit() function

### [Atomic Arch: Killing processes with bpf_send_signal()](https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/#atomic-arch-killing-processes-with-bpf-send-signal)

```
tp_syscalls_sys_enter_ptrace:0000000000000EE8 enter_ptrace:                           ; tracepoint: sys_enter_ptrace, kill callers trying to attach to hidden PIDs
tp_syscalls_sys_enter_ptrace:0000000000000EE8                 ldxdw          r2, [r1+0x10]
tp_syscalls_sys_enter_ptrace:0000000000000EF0                 jeq            r2, 0x4206, loc_F00 ; request == PTRACE_SEIZE (0x4206)?
tp_syscalls_sys_enter_ptrace:0000000000000EF8                 jne            r2, 0x10, unk_F50 ; request == PTRACE_ATTACH (0x10)?
tp_syscalls_sys_enter_ptrace:0000000000000F00
tp_syscalls_sys_enter_ptrace:0000000000000F00 loc_F00:                                ; CODE XREF: tp_syscalls_sys_enter_ptrace:0000000000000EF0up j
tp_syscalls_sys_enter_ptrace:0000000000000F00                 ldxdw          r1, [r1+0x18] ; load target pid from tracepoint ctx->pid (offset 0x18)
tp_syscalls_sys_enter_ptrace:0000000000000F08                 stxw           [r10-4], r1
tp_syscalls_sys_enter_ptrace:0000000000000F10                 mov            r2, r10
tp_syscalls_sys_enter_ptrace:0000000000000F18                 add            r2, -4
tp_syscalls_sys_enter_ptrace:0000000000000F20                 lddw           r1, 0    ; map fd: hidden_pids
tp_syscalls_sys_enter_ptrace:0000000000000F30                 call           1        ; bpf_map_lookup_elem(hidden_pids, &target_pid)
tp_syscalls_sys_enter_ptrace:0000000000000F38                 jeq            r0, 0, unk_F50 ; target pid not in hidden_pids, allow ptrace
tp_syscalls_sys_enter_ptrace:0000000000000F38 ; ---------------------------------------------------------------------------
tp_syscalls_sys_enter_ptrace:0000000000000F40                 db 0xB4                 ; mov32 r1, 9  [BPF_MOV32_IMM, unsupported by plugin] ; r1 = SIGKILL
tp_syscalls_sys_enter_ptrace:0000000000000F41                 db    1
tp_syscalls_sys_enter_ptrace:0000000000000F42                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F43                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F44                 db    9
tp_syscalls_sys_enter_ptrace:0000000000000F45                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F46                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F47                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F48 ; ---------------------------------------------------------------------------
tp_syscalls_sys_enter_ptrace:0000000000000F48                 call           0x6D     ; bpf_send_signal(SIGKILL), helper #109 (0x6D), kill the calling process
tp_syscalls_sys_enter_ptrace:0000000000000F48 ; ---------------------------------------------------------------------------
tp_syscalls_sys_enter_ptrace:0000000000000F50 unk_F50:        db 0xB4                 ; CODE XREF: tp_syscalls_sys_enter_ptrace:0000000000000EF8up j
tp_syscalls_sys_enter_ptrace:0000000000000F50                                         ; tp_syscalls_sys_enter_ptrace:0000000000000F38up j
tp_syscalls_sys_enter_ptrace:0000000000000F50                                         ; r0 = 0; exit
tp_syscalls_sys_enter_ptrace:0000000000000F51                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F52                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F53                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F54                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F55                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F56                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F57                 db    0
tp_syscalls_sys_enter_ptrace:0000000000000F58 ; ---------------------------------------------------------------------------
tp_syscalls_sys_enter_ptrace:0000000000000F58                 ret
tp_syscalls_sys_enter_ptrace:0000000000000F58 ; end of 'tp_syscalls_sys_enter_ptrace'

```

The disassembly above shows another creative use of eBPF helpers. This enter_ptrace program attaches to the tp/syscalls/sys_enter_ptrace tracepoint, giving it visibility into every ptrace() call before the kernel processes it. It checks the request argument first and acts only when the request is PTRACE_ATTACH or PTRACE_SEIZE, the two operations used to attach a debugger or tracing tool to a running process. Any other ptrace operation, such as reading memory from an already-traced process, proceeds normally.

When it detects an attach attempt, the program extracts the target PID and looks it up in hidden_pids, a map the rootkit embedded in the Atomic Arch campaign maintains with the PIDs of processes it protects. If the target PID matches, the program calls bpf_send_signal(9), but it sends SIGKILL to the calling process, not the target. It kills the debugger, not the process the debugger tried to attach to.

Because this runs in the kernel before ptrace() does anything, the attach never completes. From the analyst's perspective, gdb or strace simply crashes with no explanation and no visible sign that an eBPF rootkit caused it.

## [How we observe eBPF programs before they can act](https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/#how-we-observe-ebpf-programs-before-they-can-act)

These three techniques share a structural weakness: Each depends on a small set of eBPF helpers with almost no legitimate use in production tracing programs. Add the fact that all three attach only as kprobes or tracepoints, and the fingerprint becomes narrow enough to build detections on. That combination (a rare helper, plus a narrow program type) is the detection surface.

To see how the Datadog Agent's [Workload Protection (WP)](https://docs.datadoghq.com/security/workload_protection/) sensor captures this fingerprint, we need to understand how the kernel encodes helper calls and when that encoding is visible.

When compiling an eBPF program from C, a call like bpf_map_lookup_elem(&my_map, &key) doesn't become a normal function call. eBPF has no linker and no notion of kernel symbols. Instead, the compiler encodes the call as a BPF_CALL instruction whose 32-bit immediate field holds a small integer: a helper ID, one of the values in enum bpf_func_id (for example, BPF_FUNC_map_lookup_elem = 1). The program that reaches the kernel through the bpf(2) syscall is just an array of 8-byte instructions:

[A bpf_insn instruction encodes a BPF_CALL's helper ID in its 32-bit imm field; the verifier later rewrites that helper ID into a callable address (click to enlarge)](https://securitylabs.dd-static.net/img/detection-primitives-for-ebpf-rootkits/bpf-insn-helper-id-encoding.png?auto=format)

The verifier turns that helper ID into something callable, in two stages. First, check_helper_call() asks the program type's ops table, env->ops->get_func_proto(func_id, prog), for a struct bpf_func_proto. That struct carries the helper's argument types, return type, and a function pointer fn->func to the actual kernel implementation (for example, bpf_map_lookup_elem). The verifier uses this to typecheck the call, but it doesn't patch the instruction yet. Later, in do_misc_fixups(), it rewrites the instruction:

```
fn = env->ops->get_func_proto(insn->imm, env->prog);
insn->imm = fn->func - __bpf_call_base;
```

imm is only 32 bits, so the kernel encodes the helper as a signed offset from a fixed anchor symbol, __bpf_call_base. The interpreter and every JIT backend add __bpf_call_base back to recover the real address. By the time the program runs, call #1 has become call <bpf_map_lookup_elem>. But during verification at load time, the original helper ID still sits in insn->imm, exactly where a defender can read it.

That two-stage flow makes helper observation possible from another eBPF program. The Datadog Agent's WP sensor wraps the bpf(2) syscall with its own probes and enriches each event with metadata gathered from the Linux Security Module (LSM) and verifier hooks that fire during the syscall:

[The WP sensor's load-time pipeline: it caches the syscall on entry, reads the program's identity via the security_bpf_prog LSM hook, grabs the raw helper ID via a kprobe on check_helper_call before the verifier rewrites it, then stitches identity and helpers into one event on syscall exit (click to enlarge)](https://securitylabs.dd-static.net/img/detection-primitives-for-ebpf-rootkits/wp-sensor-load-time-fingerprint-pipeline.png?auto=format)

Concretely:

- Program type comes from the security_bpf_prog LSM hook, whose first argument is the freshly allocated struct bpf_prog. The probe reads prog->type, prog->expected_attach_type, prog->aux->id, prog->aux->name, and prog->tag via bpf_probe_read.
- Helpers come from a kprobe on check_helper_call, which fires before do_misc_fixups, while insn->imm still holds the raw helper ID. It then ORs the bit for that helper into a three-u64 bitmap on the per-PID syscall cache, a compact record of which of the roughly 192 helpers the program calls.
- The agent stitches these together. When security_bpf_prog fires for the same load, it copies the helper bitmap into the per-program record alongside type, name, and tag. On syscall exit, the agent emits a single event with the program's full fingerprint: type, attach type, name, tag, ID, associated map metadata, and the exact set of helpers it can invoke.

Load time is the last moment the kernel's view of an eBPF program can be trusted. Once a program attaches, the assumptions a detector relies on break down: A rootkit can hook getdents to hide its own file descriptors from user space enumeration, override bpf_get_current_pid_tgid results seen by other probes, rewrite syscall return values headed to security tooling, splice itself out of bpf(BPF_PROG_GET_NEXT_ID) listings, or simply detach the agent's own probes. Any inventory taken after that point looks through a lens the attacker can control.

At load time, none of that is possible yet. The program is still an inert blob of instructions under inspection by the verifier. It has no attach point, no execution context, and no way to interfere with the probes watching it. The fingerprint captured at this instant (program type, attach type, name, tag, map references, and the exact set of helpers reachable from its instruction stream) is the ground truth against which everything that follows gets compared. Capturing it here, once, before the program can act, is what makes downstream detection possible at all.

## [Detections](https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/#detections)

The load-time fingerprint translates directly into detection rules. Because the WP sensor captures the helper bitmap before the program can act, a rule checking for BPF_PROBE_WRITE_USER fires the moment VoidLink's buffer-tampering program is submitted to the kernel, before it ever touches a Netlink response:

```
bpf.cmd == BPF_PROG_LOAD && BPF_PROBE_WRITE_USER in bpf.prog.helpers

```

The same principle catches LinkPro's enumeration-hiding kretprobe at load:

```
bpf.cmd == BPF_PROG_LOAD && BPF_OVERRIDE_RETURN in bpf.prog.helpers

```

And it catches the ptrace-killing tracepoint before it can silence a single debugger:

```
bpf.cmd == BPF_PROG_LOAD && (BPF_SEND_SIGNAL in bpf.prog.helpers || BPF_SEND_SIGNAL_THREAD in bpf.prog.helpers)

```

All three rules share the same structure: They match on BPF_PROG_LOAD and a specific helper bit, with no dependency on program names, process ancestry, or any post-attach observable that a running rootkit could corrupt. Where legitimate software uses one of these helpers, you can scope the rules down—for example, by excluding a known tracing framework by process path or name—without weakening the signal against unknown loaders.

Each rule alone flags a single suspicious load. Chain them into a correlation rule with a one-hour window, and concurrent signals from two or more of these helpers—especially from an interactive shell or a binary in a world-writable directory—produce a high-confidence eBPF rootkit installation signal. That correlation catches what each individual rule only hints at: a program that needs to write user space memory, override kernel return values, and kill processes attaching to it is almost certainly not a tracing tool. And the agent recorded the proof before the program got a chance to hide.

## [Conclusion](https://securitylabs.datadoghq.com/articles/detection-primitives-for-ebpf-rootkits/#conclusion)

eBPF rootkits work well because they conceal the malware's hiding logic in trusted components: syscall return paths, user space buffers, tracing interfaces, and kernel-mediated enumeration. VoidLink, LinkPro and Atomic Arch use different helpers to achieve different effects, but they all expose the same defensive opportunity. Namely, helper capabilities are knowable at load time, before the program can attach and start shaping what the host reports back.

Detection engineers should treat BPF_PROG_LOAD as the point of highest trust and capture the full load-time fingerprint. Uncommon helpers, such as bpf_probe_write_user(), bpf_override_return() and bpf_send_signal(), should be treated with caution, particularly when they appear together or originate from suspicious loaders. Defenders have the highest chance of detecting eBPF rootkits at load time, before the rootkit runs and attempts to conceal itself. The techniques presented in the article are just examples observed in the wild, but the power of eBPF is truly immense, and attackers are free to unleash their creativity with it. There is definitely an opportunity to spend some more time trying to foresee how attackers might use eBPF to their own ends in the future.

-  [twitter](https://twitter.com/share?url=https%3A%2F%2Fsecuritylabs.datadoghq.com%2Farticles%2Fdetection-primitives-for-ebpf-rootkits%2F&text=Detection%20primitives%20for%20eBPF%20rootkits)
-  [reddit](https://www.reddit.com/submit?url=https%3A%2F%2Fsecuritylabs.datadoghq.com%2Farticles%2Fdetection-primitives-for-ebpf-rootkits%2F)

##  Did you find this article helpful?

Information Security Analyst II - Federal Programs

Security - Engineering
