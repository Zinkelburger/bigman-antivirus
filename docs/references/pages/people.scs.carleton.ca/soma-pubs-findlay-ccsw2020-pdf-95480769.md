---
url: "https://people.scs.carleton.ca/~soma/pubs/findlay-ccsw2020.pdf"
title: "bpfbox: Simple Precise Process Confinement with eBPF"
captured: "2026-11-02T18:39:36Z"
via: "live"
sha256: "0fb29086f184509020b134d540af971637a408c11fddabc5ce4c490c916ccd4b"
wayback: "https://web.archive.org/web/20250816105801/http://people.scs.carleton.ca/~soma/pubs/findlay-ccsw2020.pdf"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# bpfbox: Simple Precise Process Confinement with eBPF

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://people.scs.carleton.ca/~soma/pubs/findlay-ccsw2020.pdf>
> Snapshot: <https://web.archive.org/web/20250816105801/http://people.scs.carleton.ca/~soma/pubs/findlay-ccsw2020.pdf>

             bpfbox: Simple Precise Process Confinement with eBPF
                  William Findlay                                               Anil Somayaji                                      David Barrera
              Carleton University                                            Carleton University                                Carleton University
       williamfindlay@cmail.carleton.ca                                     soma@scs.carleton.ca                             david.barrera@carleton.ca

ABSTRACT                                                                                    designed specifically for this task. As a result, process confinement
Process confinement is a key requirement for workloads in the                               implementations must combine these disparate approaches in order
cloud and in other contexts. Existing process confinement mecha-                            to create working solutions.
nisms on Linux, however, are complex and inflexible because they                               When security mechanisms require complex implementations,
are implemented using a combination of primitive abstractions                               we can expect many vulnerabilities to be found, as they have for
(e.g., namespaces, cgroups) and complex security mechanisms (e.g.,                          every major Linux confinement implementation1 . But even worse,
SELinux, AppArmor) that were designed for purposes beyond basic                             complexity makes changes more difficult, stifling innovation. The
process confinement. We argue that simple, efficient, and flexible                          natural solution for this sort of challenge would be to implement
confinement can be better implemented today using eBPF, an emerg-                           an extension API which would allow for different process con-
ing technology for safely extending the Linux kernel. We present a                          finement solutions. The Linux community long ago recognized
proof-of-concept confinement application, bpfbox, that uses less                            the need to support different security abstractions, resulting in
than 2000 lines of kernelspace code and allows for confinement at                           the LSM (Linux Security Modules) framework [33]. LSM provides
the userspace function, system call, LSM hook, and kernelspace                              the interface used by mandatory access control systems such as
function boundaries—something that no existing process confine-                             AppArmor and SELinux. Developing new process containment
ment mechanism can do. Further, it does so using a policy language                          abstractions on top of LSM and other in-kernel APIs is certainly
simple enough to use for ad-hoc confinement purposes. This paper                            possible; however, kernel modules can be very expensive to develop
presents the motivation, design, implementation, and benchmarks                             and maintain, and small errors in them can lead to catastrophic
of bpfbox, including a sample web server confinement policy.                                failures. Due to stability and security concerns, many environments
                                                                                            will refuse to use solutions that require their own kernel modules.
CCS CONCEPTS                                                                                These challenges are why current confinement solutions build on
                                                                                            top of existing kernel functionality rather than adding their own,
• Security and privacy → Operating systems security; Access
                                                                                            despite the inevitable implementation complexity.
control.
                                                                                               The key insight of this paper is that recent additions to the Linux
                                                                                            kernel have enabled a third path to implementing process con-
KEYWORDS
                                                                                            finement: Extended Berkeley Packet Filter (eBPF) [47]. Originally
eBPF; Sandboxing; Operating System Security; Application Con-                               developed to improve packet filtering, in Linux BPF has been ex-
finement; Access Control; Linux                                                             tended to allow fine-grained introspection of kernel and userspace
ACM Reference Format:                                                                       behavior. As with kernel modules, code is loaded by the root user
William Findlay, Anil Somayaji, and David Barrera. 2020. bpfbox: Simple                     into the kernel; unlike kernel modules, eBPF is a bytecode language
Precise Process Confinement with eBPF. In 2020 Cloud Computing Security                     that is verified at load time and then just-in-time compiled before
Workshop (CCSW’20), November 9, 2020, Virtual Event, USA. ACM, New York,                    being linked into the kernel. This verification step, combined with
NY, USA, 13 pages. https://doi.org/10.1145/3411495.3421358
                                                                                            the limitations of eBPF bytecode, allows kernel functionality to be
                                                                                            extended in a safe yet performant way.
1     INTRODUCTION                                                                             Here we present bpfbox, a prototype process confinement sys-
Process confinement is a critical problem for cloud computing.                              tem consisting of a policy language and eBPF-based implementation
Whether one is running conventional servers, container-based                                that can run on Linux 5.8 and newer kernels. The key advantages of
workloads, or untrusted third-party code, a fundamental require-                            bpfbox are that it combines simple yet flexible policies with an im-
ment is that some processes need to be restricted to enforce least                          plementation that can be easily changed and improved, exploiting
privilege. Today, we have a wide variety of technologies on Linux                           the flexibility of eBPF. bpfbox is implemented in bcc [2], a devel-
for confining processes: user and group permissions, Linux capabil-                         opment toolchain for Linux eBPF programs that combines Python
ities, SELinux [45], AppArmor [12], seccomp(2) [41], cgroups [9],                           userspace code with eBPF code written in a restricted subset of
namespaces [26] and even the venerable ptrace(2) [36]. While                                C. While policy errors can impede the functioning of specific pro-
these technologies can be used to contain processes, none were                              grams, implementation errors in bpfbox will not harm the system,
                                                                                            due to the safety guarantees of eBPF.
Permission to make digital or hard copies of part or all of this work for personal or
classroom use is granted without fee provided that copies are not made or distributed          bpfbox policies advance the state of the art in process confine-
for profit or commercial advantage and that copies bear this notice and the full citation   ment by allowing simpler yet more precise policies to be written.
on the first page. Copyrights for third-party components of this work must be honored.
For all other uses, contact the owner/author(s).
                                                                                            Specifically, it is possible to go beyond system calls and network
CCSW’20, November 9, 2020, Virtual Event, USA
                                                                                            1 A small selection of CVEs show how common confinement vulnerabilities are.
© 2020 Copyright held by the owner/author(s).
ACM ISBN 978-1-4503-8084-3/20/11.                                                           seccomp: CVE-2019-2054, AppArmor: CVE-2019-18814, SELinux: CVE-2020-10751,
https://doi.org/10.1145/3411495.3421358                                                     Docker: CVE-2020-13401, snap: CVE-2019-11503, flatpak: CVE-2019-8308
CCSW’20, November 9, 2020, Virtual Event, USA                                                       William Findlay, Anil Somayaji, and David Barrera


connections and instead specify restrictions on userspace and ker-      they are authorized users of the system. Further, the root user can
nelspace functions. Instead of simply preventing a process from         do almost anything to a system. Fine-grained process confinement
making a call to execve(2) (or an execve of specific program bi-        thus typically requires additional mechanisms.
naries), bpfbox can specify that execve calls for a process must        Linux MAC: Mandatory access control (MAC) limits the privileges
originate in a specific function and should optionally pass certain     of all users, including the root user, and thus serves as another set of
functional checks in the kernel (e.g., that the program being run is    mechanisms that can help confine processes. On Linux, kernel mod-
an ELF binary rather than a script). While such additional instru-      ules implementing MAC policies use the Linux Security Module
mentation does result in additional runtime overhead, as we show        (LSM) API [33] to hook into the security checks made throughout
the overhead is quite modest in practice—for example, when alert-       the kernel. Whether the implementation is SELinux [45], AppAr-
ing on every security operation, bpfbox slows down a web server         mor [20], TOMOYO [21], or other, all MAC implementations limit
by around 8.4%, slightly less than running AppArmor in a similar        the access processes have to kernel resources, including files and
configuration. Further, bpfbox is implemented using less than 2000      network sockets, based on attributes associated with the process or
lines of in-kernel code, smaller than seccomp-bpf and much smaller      thread.
than SELinux. The relative simplicity of bpfbox means that it can       Namespaces and cgroups: On Linux, namespaces and cgroups
be used on its own to do ad-hoc process confinement; its flexibility,   allow system resources to be partitioned. Namespaces partition
however, means that it could be used to simplify and extend the         resources in terms of naming, giving a group of processes a private
confinement technology in container management systems or even          view of enumerable system resources such as process IDs, filesys-
in regular Linux distributions. Looking forward, we see bpfbox as a     tems, network sockets, and user IDs. Cgroups limit non-enumerable
proof of concept that demonstrates the potential of eBPF to enable      resources such as memory, CPU, and I/O bandwidth. By limiting the
more secure fine-grained resource sharing in the cloud.                 resources that a process can name and how much of that resource
   The rest of this paper proceeds as follows. We first review stan-    it can use, namespaces and cgroups provide the base abstractions
dard Linux security mechanisms in Section 2. Next, we describe the      for operating system virtualization. Because OS virtualization is
rationale for bpfbox (Section 3) in the context of existing process     fundamentally a way to separate groups of processes from each
confinement solutions. We explain eBPF and related technologies         other, these mechanisms can also be used for the simpler case of
in Section 4. In Section 5, we explain how bpfbox policies are en-      process confinement.
forced using eBPF, and we describe the bpfbox policy language in        Linux capabilities: The root user in Unix systems traditionally
Section 6. We present a web server policy example in Section 7. Sec-    has access to all resources; thus if an attacker gains control of a pro-
tion 8 presents a quantitative evaluation of bpfbox’s performance       cess running as root, all security controls are null and void. Linux
impact. Section 9 discusses limitations, potential enhancements         capabilities (based on the withdrawn POSIX.1e standard2 ) subdivide
to the current research prototype, and future research directions.      root’s privileges into individual capabilities which a privileged pro-
Section 10 concludes.                                                   cess can retain or keep individually. Alternately, program binaries
                                                                        can be set to have specific capabilities when execve’d, thus getting
                                                                        more privileges than normal, but not getting all privileges a setuid
2    LINUX CONFINEMENT MECHANISMS                                       root program would get. Linux capabilities help minimize the privi-
Process confinement is extensively used on desktop, mobile, and         leges given to processes that manage access to resources, whether
server/cloud systems to protect systems from running partially or       they be networking, filesystems, devices, or kernel modules.
fully untrusted code. On the desktop, web browsers isolate pro-         Ptrace: The ptrace(2) system call is used to allow one process to
cesses running JavaScript loaded by web applications. On mobile         monitor and control the execution of another process. Debugging
devices, third-party applications are confined so they have lim-        tools such as gdb and strace are fundamentally based on ptrace.
ited access to system resources and other applications. On servers,     Because a process controlled by ptrace can have its execution arbi-
containers separate applications from each other, enabling smooth       trarily monitored or changed, unprivileged users can only ptrace
deployment and load balancing. Many mechanisms are used to              their own unprivileged processes. Additionally, modern Linux dis-
implement confinement on Linux systems. In this section we dis-         tributions enable the Yama [8] LSM by default, which restricts
cuss these confinement mechanisms; in the following section we          access to ptrace such that only parent processes may ptrace their
examine why these are insufficient.                                     children; tracing other unrelated process requires changing a kernel
   On Linux, the main implementation technologies for containing        runtime option. While ptrace isn’t normally used in production
processes are Unix discretionary access controls, mandatory access      environments, it can be used to confine process behavior by intro-
controls, cgroups, namespaces, Linux capabilities, and seccomp. We      ducing traps on restricted behavior.
discuss each in turn below.                                             Seccomp: While Linux capabilities allow for restrictions on priv-
Unix DAC: The most basic way to confine processes in Linux is           ileged operations, they aren’t sufficient to limit the system calls
to make use of the traditional Unix user-oriented discretionary         that unprivileged processes can normally make. seccomp(2) [41]
access controls that determine how a process’s user and group IDs       is a system call that causes a process to enter a restricted state
restrict what files, system calls, and other processes it can access.   where it can only respond to signals, terminate, and read and write
Unprivileged users can only send signals to their own processes, and    to open file descriptors. Other system calls cause forcible process
access to files can be limited using per-inode permission bits. Unix
users, however, are given significant access to system resources, as
they do have a significant amount of trust given to them—after all,     2 POSIX 1003.1e Draft 17
bpfbox: Simple Precise Process Confinement with eBPF                                                   CCSW’20, November 9, 2020, Virtual Event, USA


termination. Seccomp-bpf [14] is an extension to seccomp that al-         and systems administrators, under the hood it must deal with the
lows additional system call rules to be defined through classic BPF       full complexity of securing a service on a Linux host.
(not eBPF) filters. For example, with seccomp-bpf open(2) can be              Desktop container solutions are no better. For example, snap [46],
allowed, but only for specific files. Note the filtering is very flexi-   Flatpak [16], Bubblewrap [5], and Firejail [15] also implement virtu-
ble because it is implemented using BPF; this flexibility, however,       alized operating systems through cgroups, namespaces, and filesys-
comes at the price of complexity.                                         tem mounts. This OS virtualization is augmented with extensive
   We explain more about classic BPF and eBPF in Section 4.               integration with the host operating system’s security mechanisms
                                                                          (i.e., Linux capabilities, seccomp, and MAC frameworks). While
                                                                          packaged in a friendly way, under the hood full Linux userlands
3    MOTIVATION                                                           must be secured for each installed application.
To understand the potential benefit of lightweight process confine-           Operating system virtualization on Linux was originally devel-
ment using eBPF, consider how existing systems separate processes         oped to support the needs of shared hosting, in particular to allow
in a cloud context. Cloud computing can be said to be (in part) an        multiple customers to administer their own web servers without
evolving platform for allocating computational resources so that          the overhead of separate physical boxes or multiple kernels. With
machine-level boundaries are less and less significant—applications       Docker and snaps, this same technology is used for application
can span multiple hosts, and multiple applications can share specific     management. While some containers can be very complex, using
hosts. Process confinement is a fundamental part of this evolving         multiple binaries, libraries, configuration files, and processes, oth-
cloud computing story. Here we discuss how non-Linux systems              ers can be as simple as a single statically-linked binary running as a
have addressed process confinement and the limitations of existing        single process. In such contexts the elaborate container machinery
approaches to process confinement on Linux.                               is really only being used as a way to conveniently limit the access
    The process confinement problem dates back half a century [28].       of a single binary.
Dozens of tools and frameworks, some more practical than others,              Consider that all of the mechanisms used to secure containers,
have been proposed to limit the impact of untrusted software on           from cgroups to SELinux, are simply different mechanisms for
the rest of the system [43]. The isolation provided by containers         telling the Linux kernel what resources processes can access. We
is really a form of operating system virtualization, something that       use these multiple mechanisms because they are available, each
has a long history on Unix, with implementations including the            having been built to solve specific problems from the past. If we
standard chroot(2), BSD jails [25] and Solaris Zones [38]. While          started from scratch, we would certainly have developed simpler,
OS virtualization partitions resources, on its own it does not really     more coherent mechanisms for isolating processes. bpfbox can thus
implement least privilege: processes can still have dangerous levels      be seen as an attempt to develop such a mechanism, made feasible
of access to system and network resources. Fine-grained access            by eBPF.
control can be used to complement OS virtualization, as shown
with Capsicum [23, 48]. Also, system call filtering has been widely
used to confine untrusted applications [17, 18, 24, 37, 39], both with    4   EXTENDING LINUX WITH EBPF
and without OS virtualization.                                            For this work, we wished to implement a new process confinement
    Modern containers on Linux are simply instances of OS virtu-          mechanism using eBPF, a new Linux technology for adding code to
alized systems that are packaged for easy deployment and man-             the kernel safely and efficiently. To understand the significance of
agement. To a developer they appear almost as separate hosts, just        eBPF, here we place it in the context of similar work in operating
without their own OS kernel and (often) some core system services.        systems research and practice. We also explain the history of eBPF,
The actual separation of processes, however, is hardly more than          present its high-level design, and discuss how it has been previously
the separation between processes on a regular Unix system. Of             used for security applications.
particular concern is that sufficiently privileged process inside of a       In research on extensible operating systems, there has been ex-
container can “break out” and access any resources on the system,         tensive work on ways to safely extend their functionality. Adding
and such privileges can be obtained using any number of privi-            code to an operating system is the perfect way to add abstractions
lege escalation attacks. Container security thus relies on exactly        and change system behavior; it is also the perfect way to make a
the same mechanisms to separate processes that are used on any            system crash or silently corrupt its filesystems. Most work on ex-
reasonably secured server.                                                tensible operating systems has been on microkernels such as Mach
    We can see this pattern with the separation mechanisms used by        [1] and L4 [31], which facilitate extensibility by putting most of the
Docker [13]. Docker uses Linux cgroups, namespaces, and filesys-          OS into userspace processes; however, systems like SPIN [3] allow
tem mounts to implement OS virtualization. Other security mecha-          for code written in a safe language to be added to kernelspace. eBPF
nisms are employed to actually make sure processes remain sepa-           thus has similarities to SPIN, except it was designed to facilitate
rated and operate with least privilege. Linux capabilities are dropped    system introspection, not application performance.
so that root processes inside of a container cannot break out. Sec-          The closest system and predecessor to eBPF is Solaris’s DTrace [6,
comp is used to restrict access to dangerous system calls, with           19], which was created to allow for safe introspection of production
the default policy restricting many system calls (such as those for       systems by dynamically extending the Solaris kernel. The idea
managing kernel modules). AppArmor or SELinux are frequently              was that small scripts could be written in a constrained language
used to further limit what resources container processes can access.      that could be compiled, verified for safety, and then loaded into
Thus, while Docker provides easy-to-use mechanisms to developers          kernelspace. For licensing and technical reasons, DTrace has not
CCSW’20, November 9, 2020, Virtual Event, USA                                                                          William Findlay, Anil Somayaji, and David Barrera


been ported to Linux. Instead, the Linux community has ended up                             beyond system call filtering, while retaining a small codebase, al-
building a superset of DTrace’s functionality in the form of eBPF.                          lowing fine-grained filtering, and avoiding the usual race conditions
    Classic BPF has a rich history in Unix-like systems. BPF [32] first                     that impact system call filtering systems [17].
emerged as a packet filtering solution for BSD Unix in 1993. The key                            Probably the closest work to our own is the eBPF-based sand-
insight of classic BPF was its use of a register-based filter machine                       boxing system Landlock [40], an out-of-tree patchset for Linux that
and buffering of results before returning them to userspace, result-                        allows exposing LSM hooks to unprivileged userspace applications.
ing in significant performance increases over contemporary packet                           Like bpfbox, Landlock attaches eBPF programs to LSM hooks to
filtering solutions. Since then, it has been incorporated into many                         make access control decisions. Unlike bpfbox, Landlock envisions
operating system kernels, notably Linux, OpenBSD, and FreeBSD,                              unprivileged processes specifying their own LSM restrictions using
where it continued to be predominantly used for packet filtering. A                         its C API. The key challenge faced by Landlock is that of securely
notable exception to this is the seccomp-bpf extension under the                            exposing eBPF and LSM to unprivileged processes, something that
Linux kernel, in which classic BPF programs may be written to                               the primary developer now believes is no longer possible in a world
augment seccomp, as we discuss in Section 2.                                                with CPU-level vulnerabilities [10]. In contrast, bpfbox runs with
    In 2014, Starovoitov and Borkmann [47] merged a major rewrite                           root privileges and assumes that policies have also been installed
of the BPF engine into the Linux kernel. This new incarnation of                            by a privileged user. Rather than requiring code-level additions,
BPF, dubbed Extended BPF (eBPF), added 10 new general purpose                               bpfbox separates policy generation from application development.
registers, a new instruction set with just-in-time (JIT) compilation
to native instruction sets, access to allowed kernel helpers, a rich                        5     BPFBOX IMPLEMENTATION
collection of specialized data structures, and new program types
                                                                                            In this section, we explain the implementation of bpfbox’s pol-
to enable tracing various aspects of system behavior. Since its in-
                                                                                            icy generation and enforcement mechanisms. First, we present an
ception, eBPF has seen rapid development with a myriad of new
                                                                                            overview of the userspace and kernelspace components of bpfbox
features in each kernel release3 . bpfbox’s in-kernel enforcement
                                                                                            (Section 5.1). Next, we discuss how bpfbox generates policy in
engine leverages many of the new features provided by eBPF and
                                                                                            userspace and stores and enforces it in kernelspace (Section 5.2)
requires no modification of existing kernel source code, meaning
                                                                                            and manage process state (Section 5.3). Finally, we discuss how
that any recent Linux kernel with support for eBPF LSM programs
                                                                                            bpfbox logs per-event audit data in userspace (Section 5.4). bpfbox
can run bpfbox without needing to be patched or recompiled.
                                                                                            is free and open source software, available under the GPLv2 license
    eBPF programs themselves consist of a set of eBPF instructions
                                                                                            at https://github.com/willfindlay/bpfbox.
that may be written by hand or optionally compiled from a restricted
subset of a higher level language such as C. Many development
toolchains such as bcc [2] and libbpf [30] exist to make this process                       5.1    Architectural Overview
easier. These eBPF instructions are then submitted to the kernel                            Figure 1 depicts bpfbox’s architecture, which consists of a userspace
using the bpf(2) system call where they undergo a verification                              daemon bpfboxd, a collection of eBPF programs for tracing events
process before being loaded for JIT compilation. This verification                          and enforcing policy, and several maps (eBPF data structures acces-
process involves checking for program safety to ensure that an eBPF                         sible from both kernelspace and userspace) for storing information
program cannot damage the running system. Since eBPF programs                               about active processes and loaded policy. bpfboxd parses policy
must be verifiably safe, they are fundamentally restricted in what                          files, compiles and loads eBPF code into the kernel, and interacts
they can do—eBPF is not Turing-complete. For instance, eBPF pro-                            with running eBPF code using eBPF maps. bpfboxd is primarily
grams are not allowed to exceed 1 million instructions, lack support                        written in Python; however, it links in a stub C library to facilitate
for advanced looping constructs, and have very limited support for                          interactions with eBPF code as explained below.
handling strings. Pointer arithmetic is also disallowed unless it can                           When eBPF code is loaded into the kernel, it is associated with a
be proved not to overflow or underflow the corresponding region                             specific event so that when that event occurs, the registered code
of memory. These restrictions afford eBPF a distinct advantage over                         is run. eBPF code can be associated with arbitrary functions in ker-
extending kernel behavior—modifying the source directly or adding                           nelspace (kprobes) or userspace (uprobes); however, as functions
kernel modules—particularly from a security perspective, as many                            can and do change, eBPF code can be associated with explicitly de-
traditional bugs such as buffer overflows and memory corruptions                            fined tracepoints in kernelspace or userspace (as USDTs). Through
are outright prevented.                                                                     KRSI (Kernel Runtime Security Instrumentation) [10, 44], eBPF code
    While nearly all system call filtering approaches have required                         can also be associated with any LSM hook (see Section 5.2).
OS-level modifications to implement security policy enforcement,                                While eBPF programs are loaded and eBPF maps can be accessed
seccomp-bpf can now act as a standard interface for these tools                             through the bpf(2) system call, USDT provides a way for userspace
on Linux. MBOX [27] is one such sandbox that uses seccomp-bpf                               code to directly call eBPF code in the kernel. An eBPF program
and redirects filesystem calls to a layered, app-specific filesystem.                       (loaded via the bpf(2) system call) defines kernelspace functions
Notably MBOX does not require any changes to the host operating                             and associates them with USDTs defined in userspace stub functions.
system. bpfbox uses eBPF instead of seccomp to sandbox processes                            When those stub functions are called, they trap directly to the
                                                                                            associated eBPF program. bpfboxd’s linked-in C library defines
                                                                                            such USDT stub functions, allowing it to define a custom interface
3 An up to date list of features: https://github.com/iovisor/bcc/blob/master/docs/kernel-   between its userspace and kernelspace portions, bypassing the
versions.md                                                                                 normal system call interface.
bpfbox: Simple Precise Process Confinement with eBPF                                                  CCSW’20, November 9, 2020, Virtual Event, USA




Figure 1: An overview of bpfbox’s components. bpfbox relies on three major categories of eBPF program: USDT probes which
instrument libbpfbox to load and update policy; tracepoints, kprobes, and uprobes to instrument application behavior and
manage process state; and LSM probes to enforce policy based on the information stored in policy and process state maps.


5.2     Implementing Policy                                                 (2) Determine the policy key by taking the executable’s inode
bpfbox policies are written using a custom policy language. bpfbox’s            number and filesystem device number together as a struct.
policy language allows for specific operations to be allowed, audited       (3) Look up the policy corresponding to the policy key calculated
(logged), and/or tainted. All operations not so specified are denied.           in step (2). If the process is tainted and no such policy exists,
Tainting is similar in spirit to Perl’s classic taint mode [22], how-           deny access.
ever, rather than marking data, it marks the entire process. Tainting       (4) If the process is not tainted and the current access corre-
allows for more restrictive policies to be enforced once a process              sponds to a TAINT rule, taint the process and grant access.
has engaged in specific unsafe operations, say by reading from a            (5) If the current access matches an ALLOW rule, grant access.
network socket. We present the design and syntax of the bpfbox                  Otherwise deny access.
policy language in Section 6; here we discuss the functionality it          (6) If the current access matches an AUDIT rule or access is
provides and how it is implemented.                                             denied, submit an audit event to userspace.
    bpfbox policies are per executable and are stored in an exclu-
                                                                             When a sandboxed application requests access, a corresponding
sively root-controlled directory (by default, /var/lib/bpfbox/),
                                                                         LSM hook is called which in turn traps to one of bpfbox’s LSM
written in bpfbox’s policy language (Section 6). When an executable
                                                                         probes. The probe queries the state of the currently running process
is loaded, bpfbox loads the corresponding policy file (if it exists)
                                                                         along with the policy corresponding to the requested access and
and translates it into a series of function calls to USDT stub func-
                                                                         takes these factors together to come to a policy decision.
tions. These function calls trigger the corresponding eBPF code,
                                                                             bpfbox can optionally augment the information provided by the
thus recording the policy in the policy maps as a set of policy struc-
                                                                         LSM hooks themselves with additional context obtained by instru-
tures. A policy structure consists of three distinct access vectors:
                                                                         menting other aspects of process behavior. For instance, profiles
one to define tainting operations, one to define allowed operations,
                                                                         may optionally define function contexts which determine the valid-
and one to define audited operations.
                                                                         ity of specified rules; a rule could specify that a certain filesystem
    In order to enforce policy, bpfbox leverages the KRSI patch by
                                                                         access must occur within a call to the function foo() or that it
KP Singh [10, 44] which was upstreamed in Linux 5.7. This patch
                                                                         must be audited within a call to the function bar(). The ability to
provides the necessary tools to implement MAC policies in eBPF by
                                                                         combine various aspects of system behavior, both in kernelspace
instrumenting probes on LSM hooks provided by the kernel. The
                                                                         and in userspace, is a key advantage of an eBPF-based solution over
eBPF program can then audit the event and optionally enforce pol-
                                                                         traditional techniques. This allows for the creation of extremely
icy by returning a negative error value. bpfbox instruments several
                                                                         fine-grained policies at the discretion of the policy author. The
LSM probes covering filesystem access, interprocess communica-
                                                                         mechanisms by which this is accomplished are discussed further in
tion, network sockets, ptrace(2), and even bpf(2) itself. When
                                                                         Section 5.3.
these hooks are called in the kernel, they trigger the execution of
                                                                             Due to bpfbox’s strict resolution of filesystem objects at policy
the associated eBPF program which is, in general, composed of the
                                                                         load time, a problem arises when dealing with applications that
following six steps:
                                                                         read or write temporary files on disk or create new files at runtime.
   (1) Look up the current process state. If no state is found, the      In order to circumvent this issue, bpfbox treats the creation of new
       process is not being traced, so grant access.                     files as a special case. In order for a new file to be created, the
CCSW’20, November 9, 2020, Virtual Event, USA                                                        William Findlay, Anil Somayaji, and David Barrera


process must have write access to the directory in which the files
will be created. Supposing, for instance, the temporary file would
be written to /tmp, this means that, at a minimum, the policy in
question must specify that /tmp is writable. When the sandboxed
application creates a new child inode of /tmp, bpfbox dynamically
creates a temporary rule that grants the application full read, write,
link, and unlink capabilities on the created file. This rule is keyed
using a combination of the standard filesystem policy key and
the PID (process ID) of the sandboxed process. This rule is then
automatically cleaned up when the process exits or transitions to a
new profile.
   Another important detail to consider is the possibility of other
applications using the bpf(2) system call to interfere with bpfbox’s
mediation of sandboxed applications. For instance, another appli-
cation might attempt to unload an LSM probe program or make
changes to the policy or process state maps. To prevent this, bpfbox      Figure 2: The various mechanisms that bpfbox uses to man-
instruments an additional LSM probe to mediate access to bpf. It          age process state. Probes marked sched:* are tracepoints in-
uses this probe to deny all calls to bpf that attempt to modify           strumenting scheduler events in the kernel. Uprobes and
bpfbox’s programs or maps that do not directly come from bpfbox           kprobes instrument userspace and kernelspace function
itself. Further, all sandboxed applications are strictly prohibited       calls respectively.
from making any calls to bpf—a sandboxed application has no busi-
ness performing the kind of powerful system introspection that
eBPF provides.
   Similarly to mandatory access control systems like SELinux [45]
and AppArmor [12], bpfbox supports the ability to run in either           state. bpfbox policy may optionally specify whether a transition
permissive mode or enforcing mode. When running in permissive             from one profile to another may occur in a given call to execve;
mode, bpfbox continues to audit denied operations, but allows them        this transition is disallowed by default.
to continue unobstructed. This enables the user to debug policies            Finally, the sched:process_exit tracepoint allows bpfbox to
before putting them into effect and also introduces the possibility       detect when a task exits. This tracepoint deletes the corresponding
of creating new policy based on the generated audit logs.                 entry in the process states map.

                                                                          5.3.1 Context-Aware Policy. If the policy for a given executable
5.3     Managing Process State                                            defines specific function call contexts for particular rules, bpfbox
In order for bpfbox to know what policy to apply to a given pro-          instruments these function calls using uprobes (for userspace func-
cess, it must track the lifecycle of processes through the instru-        tions) and kprobes (for kernelspace functions). Each instrumented
mentation of key events within the kernel. For this, bpfbox uses          function call is associated with a unique bit in the process’ state
three tracepoints exposed by the scheduler: sched:process_fork,           bitmask. A probe is triggered on entry that causes bpfbox to flip
sched:process_exec, and sched:process_exit. Figure 2 shows                the corresponding bit to a 1, and again on return, flipping the corre-
the events that bpfbox instruments in order to track process state,       sponding bit back to a 0. An inherent limitation of this approach is
along with their corresponding probe types. These tracepoints             that it prohibits recursive function calls from being used to specify
are used to create, update, and delete per-task entries in a global       rule contexts, though we believe that this limitation should be ac-
hashmap of process states. Each entry in the map is keyed by TID          ceptable in practice (see Section 9.1). In such cases, the policy can
(thread ID). The entries themselves consist of a data structure that      simply fall back to specifying ordinary rules.
tracks policy key association and a 64-bit vector representing the
state of the running process. This state vector is used to track
whether the process is currently tainted and what important func-         5.4    Collecting and Logging Audit Data
tion calls are currently in progress.                                     When an operation is denied or matches with an audit rule, bpfbox
   Instrumenting a tracepoint on sched:process_fork allows bpf-           submits an event to userspace for logging. To accomplish this, we
box to detect when a new task is created via the fork(2), vfork(2),       leverage the new ringbuf map type added in Linux 5.8 [35], a ring
or clone(2) system calls. This tracepoint creates an entry in the         buffer that is efficiently shared across all CPUs. In userspace, the
process states hashmap and initializes it according to the state of the   bpfbox daemon uses mmap(2) to map the corresponding memory
parent process; if the parent process is associated with a bpfbox         region and polls for new data at regular intervals. As events are
profile, its key is copied to the child until such time as the child      consumed in userspace they are removed from the ring buffer to
makes an execve(2) call.                                                  make room for new events. Since the ringbuf map provides strong
   The sched:process_exec tracepoint is triggered whenever a              order guarantees and high performance under contention, we can
task calls execve to load a new program. bpfbox uses this trace-          ensure that bpfbox always provides highly reliable and performant
point to manage the association of policy keys to a particular process    per-event auditing.
bpfbox: Simple Precise Process Confinement with eBPF                                                  CCSW’20, November 9, 2020, Virtual Event, USA


6     BPFBOX POLICY LANGUAGE
                                                                                      Listing 1: An example of a bpfbox policy.
In this section, we discuss the bpfbox policy definition language.
                                                                           /* This policy applies to the / usr / bin / foo
Our design goals for the bpfbox policy language were as follows:
                                                                            * executable */
Simplicity: To facilitate policy creation and auditability, bpfbox         #![ profile " / usr / bin / foo " ]

policies should be as short and simple as possible while remaining         /* Taint process state upon binding to
expressive enough to sandbox most applications effectively.                  * any IPv4 / IPv6 network socket */
Application Transparency: bpfbox policies should be applica-               #[ taint ] {
tion transparent—defining and implementing a bpfbox policy should               net ( inet , bind )
                                                                                net ( inet6 , bind )
not require any modification to the source code of the target appli-
                                                                           }
cation.
Flexibility: bpfbox policies should be flexible enough such that it        /* Allow the check_login function to read
is possible to be define both fine-grained and coarse-grained policy         * / etc / passwd and / etc / shadow */
according to the needs of the end user.                                    #[ func " check_login " ] {
                                                                                 fs ( " / etc / passwd " , read )
Security: bpfbox policies should be designed with security in                    fs ( " / etc / shadow " , read )
mind, adhering to all basic security principles. It should be difficult    }
to write an insecure policy.
                                                                           /* Allow the add_user function to read
  In the rest of this section we describe the basic constructs and           * and append to / etc / passwd , but log such
syntax used to define bpfbox policy and explain the rationale for          events to the audit logs */
                                                                           #[ func " add_user " ]
our design decisions.
                                                                           #[ audit ] {
                                                                                fs ( " / etc / passwd " , read | append )
6.1     Writing bpfbox Policy                                              }

bpfbox policies are a series of directives and rules. Directives deco-     /* Read and append to any immediate child
rate either individual rules or blocks of rules denoted by braces and       * of the / var / log / foo / directory */
are used to specify additional context or policy actions. The first line   fs ( " / var / log / foo /* " , read | append )
in a bpfbox policy is always a special profile directive, written as       /* Allow the execution of / bin / bash , transitioning
#![profile "/path/to/exe"], which marks the executable to which              * profiles to bash 's profile after the \ texttt { execve
the policy should be associated. Other than the profile directive, all            (2) }
others take the form of #[directive]{rule()}. Multiple directives            * and untainting the process */
                                                                           #[ transition ]
may be specified before a set of rules, meaning that all directives
                                                                           #[ untaint ] {
apply to each rule.                                                             fs ( " / bin / bash " , read | exec )
   Profile assignment occurs when a process makes an execve(2)             }
call that results in loading the specified executable. Once a process
has been assigned a profile, this profile cannot change again, unless
                                                                           Table 1: The filesystem access flags supported in bpfbox.
an execve(2) occurs which has been explicitly marked with the
#[transition] directive. This ensures that policy transitions only
occur when expected and prevents malicious execve(2) calls from               Flag        Meaning
changing bpfbox’s treatment of a process.                                     read        The subject may read the object.
   In the subsections that follow, we will describe the rule categories       write       The subject may write to the object.
supported by bpfbox (Sections 6.1.1 to 6.1.4) and the directives that         append      The subject may append to the object.
may optionally be used to decorate them (Sections 6.1.5 to 6.1.6).            exec        The subject may execute the object.
Listing 1 depicts a simple example bpfbox policy.                             setattr     The subject may change the object’s filesystem
                                                                                          attributes.
6.1.1 Filesystem Rules. Filesystem rules in bpfbox govern what                getattr     The subject may read the object’s filesystem
operations a process may perform on filesystem objects such as files                      attributes.
and directories. They are written as fs("pathname", access) where             rm          The subject may remove the object’s inode.
"pathname" is a string containing the pathname of the file and access         link        The subject may create a link to the object’s
is a list of one or more file access permissions joined by the vertical                   inode.
bar symbol (|). For instance, to represent read and append permis-            ioctl       The subject may perform an ioctl call on the
sions on /var/log/my_log, the corresponding bpfbox rule would                             object.
be fs("/var/log/my_log", read|append). In total, bpfbox supports
nine distinct filesystem access flags as shown in Table 1.
    bpfbox supports a limited globbing syntax when defining path-
names, allowing multiple rules matching similar files to be com-
bined into one. Although filesystem rules are specified using path-
names, bpfbox internally uses inode and device numbers rather
CCSW’20, November 9, 2020, Virtual Event, USA                                                        William Findlay, Anil Somayaji, and David Barrera


than the pathnames themselves. When loading policies, bpfbox au-          handled. For example, to specify the ability to send fatal signals to
tomatically resolves the provided pathnames into their respective         any process running /usr/bin/ls, the corresponding bpfbox rule
inode-device number pairs. This information is then used to look          would be signal("/usr/bin/ls", fatal). To narrow permissions
up the correct policy whenever a sandboxed application attempts           such that only SIGTERM and SIGINT are allowed, signal("/usr/bin
to access an inode. Since bpfbox does not check the pathnames             /ls", sigterm|sigint) could be used instead.
themselves when referring to files, it is able to defeat time-of-check-
to-time-of-use (TOCTOU) attacks, where an attacker quickly swaps          6.1.4 ptrace Rules. Just like with signals, ptrace access is spec-
out one file with a link to another in an attempt to circumvent           ified as ptrace("exe", access), where access is a list of allowed
access control restrictions in a privileged (most often setuid) bi-       ptrace modes separated by vertical bars. The child keyword is
nary [4]. In such a situation, bpfbox would simply see a different        also available for ptrace rules to allow tracing of any child process,
inode and deny access.                                                    regardless of the child’s current profile. For instance, a rule that
    In addition to regular filesystem rules, bpfbox provides a spe-       allows a process to read and attach to a child process would be writ-
cial rule type for /proc/pid entries in the procfs virtual filesys-       ten as ptrace(child, read|attach), while a rule that allows only
tem. procfs rules, written as proc("exe", access) where "exe" is          read access to processes running /usr/bin/ls would be written
a string containing the pathname of another executable and access         as ptrace("/usr/bin/ls", read). Note that currently ptrace rules
is the desired access. For example, read-only access to the procfs        do not override other ptrace restrictions, such as those imposed
entries of executables running /usr/bin/ls may be specified with          by Yama [8].
proc("/usr/bin/ls", read). Access to any procfs entry may be              6.1.5 Allow, Taint, and Audit Directives. bpfbox supports three dis-
specified using the special keyword any.                                  tinct directives for defining actions that should be taken when a
6.1.2 Network Rules. bpfbox implements networking policy at the           given access matches a rule. The #[allow] directive causes bpfbox
socket level, covering both Internet sockets as well as Unix domain       to allow the access; however, it is not typically necessary to explic-
sockets. Networking rules are specified using net(protocol, access        itly specify this as undecorated rules are assumed to be allowed
), where protocol is a networking protocol like inet, inet6, or           by default. Regardless, it may be desirable to decorate such rules
unix and access is a list of socket operations (Table 2) separated        with #[allow] to improve the clarity of the policy. #[taint] is used
by vertical bars. For example, a rule targeting bind, accept, and         to mark a rule as a taint rule, which causes the process to enter
connect operations on an inet6 socket would look like net(inet6           a tainted state when matched. These rules can be thought of as
, bind|connect|accept), while a rule targeting create operations          gateways into the rest of the policy. Once a process is tainted, this
on a unix socket would look like net(unix, create).                       cannot be reversed unless it makes an execve(2) call explicitly
                                                                          marked with #[untaint]. Finally, #[audit] may be combined with
  Table 2: The socket operation flags supported in bpfbox.                #[allow] to cause bpfbox to log the matching operation to its audit
                                                                          logs. This can be useful for marking rare behavior that should be
                                                                          investigated or for determining how often a given rule is matched
    Flag           Meaning
                                                                          in practice.
    connect        Subject may connect a socket to a remote ad-
                   dress.                                                 6.1.6 Func and Kfunc Directives. One of the key features of bpfbox
    bind           Subject may bind a socket to a local address.          is the ability to specify specific application-level and kernel-level
    accept         Subject may accept an incoming socket connec-          context for rules. In the policy language, this is done by decorat-
                   tion.                                                  ing rules with the #[func "fn_name"("filename")] and #[kfunc "
    listen         Subject may listen for incoming socket connec-         fn_name"] directives for userspace and kernelspace instrumentation
                   tions.                                                 respectively. Here, "fn_name" refers to the name of the function to
    send           Subject may send messages over a socket.               be instrumented and "filename" refers to the filename where the
    recv           Subject may receive messages over a socket.            function symbol should be looked up — this parameter is optional
    create         Subject may create new sockets.                        and allows for the instrumentation of shared libraries. These direc-
    shutdown       Subject may shut down a socket connection.             tives provide powerful tools for defining extremely fine-grained,
                                                                          sub-application level policy. For instance, to declare that read ac-
                                                                          cess to the file /etc/shadow should only occur during a call to
6.1.3 Signal Rules. Specifying signal behavior in bpfbox is done          the function check_password(), the corresponding bpfbox rule
using the signal("exe", access) where "exe" is the pathname of            would look like:
                                                                                              #[ func " check_password " ]
another executable and access is a list of signals allowed to be                              fs ( " / etc / shadow " , read )
sent, separated by vertical bars. Normally, only processes running
                                                                          A process that is sandboxed using this policy would be unable to
the executable "exe" are allowed to be signaled, but the special
                                                                          access /etc/shadow except within a call to the specified function.
keyword any may be used instead to specify the ability to signal
any process on the system. Two additional keywords, parent and
child, allow parent and child processes to be signaled instead. The
                                                                          7   APACHE HTTPD: AN EXAMPLE BPFBOX
access argument supports any Linux signal, in addition to a few               POLICY
helper keywords that can be used to specify broad categories, such        In this section, we present an example bpfbox policy for Apache’s
as fatal for fatal signals and nohandle for signals that cannot be        httpd webserver and offer a comparison to the AppArmor and
bpfbox: Simple Precise Process Confinement with eBPF                                                        CCSW’20, November 9, 2020, Virtual Event, USA


seccomp-bpf profiles used by snap’s httpd4 policy. We hope that             fs ( " / usr / bin / suexec " , getattr | read | exec )
this policy should serve both as an illustrative example of bpfbox’s
capabilities as well as a motivating comparison between bpfbox                Once tainted, we specify a set of allowed socket operations, both
policy and the state of the art in process confinement.                    for networking and for interprocess communication, as well the
   The bpfbox policy, depicted in Listing 2, begins by specifying the      ability for httpd to send fatal signals to other processes running the
location of the Apache httpd executable as well as the conditions          same profile. The file system rules for httpd are slightly more granu-
required for tainting the httpd process. These taint rules define the      lar. Some notable examples include the ability to append to log files,
boundary between httpd’s setup phase and its main work loop by             read configuration files, and read static content from /srv/http.
instrumenting the point at which it binds to either an IPv4 or IPv6        In practice, this could be easily reconfigured according the needs
network socket. If there is a taint rule, the rest of the policy file is   of the end user, say to add support for a writable subdirectory.
only enforced once the taint condition is satisfied. In other words,          This policy allows for the execution of all httpd shared library
with taint rules the policy only applies when the process begins           modules. If desired, this could be easily restricted to a subset of
processing untrusted input.                                                these modules by providing explicit pathnames. It also specifies that
                                                                           the suexec program, used by httpd to execute external programs
Listing 2: An example policy for the Apache httpd web-                     with different privileges, should cause the process to transition to
server.                                                                    a separate profile. The suexec profile can then be used to control
                                                                           specifically which programs should be run and what transitions (if
 #![ profile "/ bin / httpd " ]
                                                                           any) might occur as a result.
 /* Taint on binding to an inet or inet6 socket */                            Note this policy works at the LSM level rather than the system
 #[ taint ] {                                                              call level, with all LSM hooks that are not explicitly allowed being
      net ( inet , bind )                                                  denied. As a result, virtually all privileged operations are forbidden.
      net ( inet6 , bind )
 }
                                                                           Accesses to devices, kernel modules, other processes—these and
                                                                           more are excluded because they are not listed in the policy. By
 /* Specify allowed network access */                                      enabling direct access to process state and LSM, bpfbox policies
 net ( inet , any )                                                        work at an abstraction level that is both straightforward and precise.
 net ( inet6 , any )
 net ( unix , create | connect | send | recv )
 net ( netlink , create | bind | send | recv )
                                                                           7.1    A Comparison with Snap
 /* Allows kill (2) to check for process existence                         Snap packages define their security policy [46] through an app dec-
  * and to send fatal signals */                                           laration provided in the package’s snapcraft.yml file (depicted in
 signal ("/ bin / httpd " , check | fatal )
                                                                           Listing 3). The package author lists the "apps" provided by their
 /* Write to logs */                                                       package and assigns attributes and policy plugins for each app. This
 fs ("/ var / log / httpd /* log " , getattr | append )                    coarse-grained policy is then translated into corresponding AppAr-
 fs ("/ var / log / httpd " , getattr )                                    mor and seccomp-bpf policy files according to the information
                                                                           specified in the app declaration. For comparison with the bpfbox
 /* Create PID file */
 fs ("/ run / httpd /" , write )
                                                                           policy in Listing 2, we will consider both the snapcraft.yml dec-
 /* Delete or modify an existing PID file if necessary */                  larations as well as the generated policy files.
 fs ("/ run / httpd / httpd . pid " , getattr | rm | write )
                                                                           Listing 3: The relevant portions of httpd’s snapcraft.yml
 /* Serve files from / srv / html / and all subdirectories */              app declaration.
 fs ("/ srv / html /** " , read | getattr )
                                                                            apps:
 /* Read configuration */                                                     # Apache daemon
 fs ("/ usr / share / httpd /** " , read | getattr )                          apache:
 fs ("/ etc / httpd /" , getattr )                                              command: run - httpd -k start - DFOREGROUND
 fs ("/ etc / httpd / conf /** " , read | getattr )                             stop - command: httpd - wrapper -k stop
 fs ("/ usr / share / zoneinfo /** " , read | getattr )                         daemon: simple
                                                                                restart - condition: always
 fs ("/ etc / resolv . conf " , read | getattr )                                plugs: [ network , network - bind , removable - media ]
 fs ("/ etc / hosts " , read | getattr )
                                                                               In order to make writing policy for snap packages as easy as pos-
 fs ("/ proc / sys / kernel / random / boot_id " , read )                  sible, policy definitions within the app declaration are very coarse
 fs ("/ proc / sys / kernel / ngroups_max " , read )                       grained, emphasizing simplicity and terseness over expressiveness.
                                                                           snap’s httpd policy lists the network, network-bind, and removable-
 fs ("/ usr / lib / httpd / modules /*. so " , getattr | read | exec )
 fs ("/ usr / lib / libnss *. so .* " , getattr | read | exec )            media policy plugins in addition to the default snap policy. While
 fs ("/ usr / lib / libgcc_s . so .* " , getattr | read | exec )           this approach undoubtedly makes it simple and easy to write policy,
                                                                           it does little to improve policy auditability; at a glance, it is unclear
 /* Transition to a separate suexec policy */                              what effect each policy plugin has on the resulting security policy.
 #[ transition ]
                                                                               These snap declarations produce much more complex, often
4 The Nextcloud snap package: https://snapcraft.io/nextcloud               overly generalized AppArmor and seccomp-bpf policies. They are
CCSW’20, November 9, 2020, Virtual Event, USA                                                         William Findlay, Anil Somayaji, and David Barrera


rather large, consisting of 494 lines of policy (ignoring blank and           The results of the OSBench benchmark show that bpfbox im-
comment lines) for AppArmor and 410 lines for seccomp-bpf. Fur-            poses modest overhead on an unconfined system, with a difference
ther, the generated seccomp-bpf policy contains several generic            of under 4% in all tests. The "Memory Allocations" tests actually
rules that allow chown(2) to change the owner and group of files           show the bpfbox and AppArmor configurations slightly outper-
to root, despite the fact that httpd runs under its own httpd user.        forming the base system; however, given they are all separated
Even worse, the generated AppArmor policy permits the execution            by less than one standard deviation (1.87% standard deviation for
of 120 common shell utilities, many of which are unnecessary for           base), they should all be seen as essentially the same for this test.
httpd’s normal operation.                                                  When bpfbox was actively logging all security-sensitive operations
    The result of attempting to combine a very high-level approach         performed by the benchmarking programs, this overhead increased
with two complex, low-level confinement mechanisms is three                to about 15% at the highest. Predictably, the "Create Files" and
difficult-to-audit policy files. The app declaration is far too coarse-    "Launch Programs" tests exhibited the most overhead, as both test
grained to be precisely sure what each policy plugin is doing, while       cases resulted in significantly more instrumentation work on the
the AppArmor and seccomp-bpf policy files are so fine-grained,             part of bpfbox.
verbose, and overly generalized that their auditability suffers as well.      Although AppArmor exhibited slightly less overhead on uncon-
If this app declaration were replaced with a single bpfbox policy,         fined processes, the difference between the two is largely marginal,
we posit that it would not be significantly harder to write than the       within a few microseconds. In the worst case, bpfbox performed sig-
original app declaration while maintaining a steady advantage in           nificantly better than AppArmor in the "Create Files" and "Launch
auditability, as well as an advantage in terseness over the generated      Programs" tests. This difference is likely attributable to the fact
AppAmor and seccomp-bpf policy files.                                      that bpfbox’s kernelspace logging mechanisms transfer data to
                                                                           userspace more efficiently.
8     PERFORMANCE EVALUATION
In this section, we present a series of performance benchmarks             8.2    Kernel Compilation Benchmarks
aimed at ascertaining the overhead imposed by bpfbox on both               Kernel compilation benchmarks present an ideal test case for mea-
sandboxed applications and the system itself. Benchmarking data            suring overhead under a busy workload with execve(2) calls and
was collected using the Phoronix Test Suite [29], a comprehensive          significant disk I/O. This workload is more representative of the
benchmarking platform for the Linux operating system, and an               worst case for bpfbox overhead, especially when bpfbox is instru-
ad-hoc kernel compilation benchmark. To establish a metric for             menting the behavior of all involved processes. In particular, the
comparison against other process confinement mechanisms, we                "Permissive" phase of the benchmark involved the instrumentation
ran the same benchmarks under the AppArmor [12] mandatory                  of 15 distinct profiles and resulted in several thousand audit log
access control LSM. All benchmarks were run under three distinct           events per second.
conditions, summarized below.                                                 Each trial consisted of six timed Linux 5.8.0-rc6 kernel compila-
Base: No policy engine is running. This measures the baseline              tions, with the initial run discarded to eliminate transient disk I/O.
operation of the system.                                                   Compilations were timed using the GNU time(1) command line
Passive: The policy engine is running in the background without            utility.
any active profiles. This measures the impact of the policy engine            In the kernel compilation benchmarks, bpfbox and AppArmor
on unconfined processes.                                                   exhibit roughly equivalent overhead on unconfined processes. In
Permissive The policy engine is running in permissive mode in              the worst case, bpfbox exhibits significantly less overhead in ker-
the background with empty profiles for each benchmarking pro-              nelspace but loses out in total elapsed time by just over 130 sec-
gram. This represents the worst case scenario where the policy             onds. This disparity is likely due to the fact that the current ver-
engine complains about (i.e., logs) every security-sensitive opera-        sion of bpfbox does not buffer writes to its log file in userspace.
tion.                                                                      The sheer volume of events generated by this benchmark causes
   Tests were conducted on an x86_64 Arch Linux virtual machine            bpfbox’s userspace logging component to consume significantly
running a Linux 5.8.0-rc6 kernel, with eight virtual CPUs running          more CPU resources than in previous trials. This performance bot-
at 2.99GHz and 16GB of RAM. Our results show that bpfbox incurs            tleneck can be fixed with a relatively straightforward change to
acceptable overhead, on par with and in some cases better than             bpfbox’s userspace code.
AppArmor. The benchmarking results are presented in Table 3 and
are summarized in the subsections that follow.                             8.3    Phoronix Apache Benchmarks
                                                                           The Phoronix Apache benchmark provides an excellent test case for
8.1     Phoronix OSBench Benchmarks                                        measuring the performance impact of bpfbox on socket networking
The Phoronix OSBench benchmarking suite provides a series of               operations. This test runs the Apache httpd server and attempts as
tests that measure the performance of various aspects of operating         many requests as possible within a fixed amount of time. Higher
system functionality. In particular, it measures file creation, process    requests per second results in a better score.
creation, thread creation, program execution, and memory allo-                In this benchmark, bpfbox is shown to have a slightly higher base
cation. Data for each test was collected using strict-run mode,            impact on the system than in previous benchmarks at just under
which prioritizes the accuracy of results, running as many tests as        6%, but the worst-case "Permissive" impact on requests per second
needed.                                                                    is quite modest at under 9%, a difference of just over 1000 requests
bpfbox: Simple Precise Process Confinement with eBPF                                                       CCSW’20, November 9, 2020, Virtual Event, USA


Table 3: The results of the Phoronix OSBench, kernel compilation, and Phoronix Apache benchmarks. Percent differences
are given in parentheses. For the kernel compilation benchmarks, “User” and “System” represent total CPU time spent in
userspace and kernelspace while “Elapsed” represents real elapsed time.

                                                Base                   Passive                                      Permissive
                                                            bpfbox               AppArmor                bpfbox                  AppArmor
    Phoronix OSBench (lower is better):
        Create Files (𝜇𝑠)             27.86               28.94   (3.81%)       28.01    (0.55%)       32.31   (14.80%)        96.56     (110.44%)
        Create Threads (𝜇𝑠)           25.96               26.90   (3.56%)       26.28    (1.24%)       27.67   (6.39%)         26.09     (0.51%)
        Launch Programs (𝜇𝑠)          75.12               78.02   (3.79%)       77.64    (3.30%)       87.31   (15.01%)       102.43     (30.76%)
        Create Processes (𝜇𝑠)         51.32               52.53   (2.34%)       51.61    (0.57%)       51.85   (1.04%)         52.11     (1.54%)
        Memory Allocations (𝑛𝑠)      113.98              112.33   (-1.45%)     112.29    (-1.50%)     112.75   (-1.09%)       112.74     (-1.09%)
    Kernel Compilation (lower is better):
        User (𝑠)                   14457.01            14564.80   (0.74%)    14711.42    (1.74%)    14829.11   (2.54%)     14432.09      (-0.17%)
        System (𝑠)                  1712.59             1760.02   (2.73%)     1765.69    (3.05%)     1804.10   (5.20%)      2544.72      (39.09%)
        Elapsed (𝑠)                 2086.92             2114.83   (1.33%)     2130.38    (2.06%)     2397.48   (13.85%)     2261.09      (8.01%)
    Phoronix Apache (higher is better):
        Requests Per Second (𝑟 /𝑠) 14686.95            13887.59   (-5.59%)   13743.88    (-6.63%)   13504.23   (-8.39%)    13431.34      (-8.93%)


per second. bpfbox’s performance here is roughly equivalent to                the policy itself, the current implementation of bpfbox is unable
AppArmor.                                                                     to sandbox processes that started before bpfbox has run, as no
                                                                              profile will be associated with those processes. To overcome this
9     DISCUSSION                                                              limitation, future iterations of bpfbox could scan the entries in
Our work began with the dual insight that eBPF was now capable                /proc/{pid}/exe on start to manually perform the correct policy
of implementing kernelspace security mechanisms and that there                associations.
was an opportunity to create a better solution for Linux process                 Despite the fact that bpfbox does not require modifications to
confinement. bpfbox serves as a proof of concept of the former and            the Linux kernel source code, it does require a compatible Linux
a significant argument for the latter. bpfbox is a research prototype,        version that has been compiled to support specific eBPF features.
however, and as such it has a number of limitations and opportu-              In particular, bpfbox depends on the KRSI LSM instrumentation
nities for enhancements. We believe it also shows that there are a            framework (available in Linux 5.7+) and the ringbuf map type
number of unexplored opportunities in systems security that are               (available in Linux 5.8+), as well as a kernel that has been compiled
made feasible with eBPF. We discuss each of these in the rest of              with support for loading eBPF programs and exposes BTF (BPF Type
this section.                                                                 Format) debugging symbols. Many mainstream Linux distributions
                                                                              come with this support enabled by default, while others require
9.1     Limitations                                                           the user to manually set the appropriate configuration flags before
                                                                              compiling the kernel. It may be some time, however, before many
While our current implementation has a number of limitations,                 systems upgrade to Linux 5.8+ kernels.
we see most of them as opportunities for future work rather than
inherent limitations of our approach. For example, one limitation
of our approach to sub-application level confinement (i.e., the #[            9.2       Potential Improvements
func] directives discussed in Section 5.3.1 and Section 6.1.6) is that        Perhaps the most obvious extension to bpfbox is the introduction
it breaks for recursive function calls. This limitation exists due to         of seccomp-like functionality, through which policies could spec-
the nature of how uprobes work in eBPF. A uprobe temporarily re-              ify access at the system call level in addition to the LSM hook
places the address of a function call with a trap to the eBPF program         level. eBPF already supports instrumentation of processes at the
which trampolines back to the original address when it terminates.            system call level through the syscalls:* and raw_syscalls:*
Uretprobes work in much the same way, instrumenting the return                tracepoint families. Unlike traditional seccomp-bpf, these filters
address instead. This means that eBPF programs cannot distinguish             could be applied externally to the target application and without re-
between a return from a recursive function call and a return from a           lying on execution through a wrapper application (e.g., MBOX [27])
top-level function call. Since the current implementation of bpfbox           for application transparency. Filters could then be integrated with
relies on setting and unsetting per-process flags to manage state,            the existing LSM probe framework to provide one cohesive policy
this may lead to unexpected denials within such recursive function            definition language rather than relying on two distinct solutions.
calls. Having said this, we do not believe this limitation to be sig-            The extensibility and flexibility of eBPF is highly conducive to
nificant in practice, and if it were a factor, several workarounds are        integration with various aspects of the operating system, including
possible including explicitly defined userspace tracepoints (USDTs).          other security solutions. For instance, it may be possible to integrate
    Since bpfbox relies on instrumenting various eBPF programs in             future versions of bpfbox with intrusion detection and prevention
order to manage process-to-policy associations as well as enforce             systems, assigning specific detection thresholds to particular rules.
CCSW’20, November 9, 2020, Virtual Event, USA                                                      William Findlay, Anil Somayaji, and David Barrera


bpfbox could deny access to a specific set of files or prevent the         Exploits respect no fixed boundaries, yet standard security mech-
execution of specific programs if the system detects an attack in       anisms do. The kernel does not directly control how processes run,
progress. This integration could also be extended to other eBPF         and processes do not control the kernel; instead, they interact (pri-
programs monitoring various other aspects of system behavior.           marily) through the system call interface. When security problems
   Rather than strictly focusing on policy enforcement at the appli-    arise on one side of this divide, traditionally it must be solved on
cation and sub-application levels, bpfbox could be extended to also     that side.
enforce policy at the container level rather than just the process         bpfbox demonstrates that, with technologies like eBPF, we don’t
level. Policies could be written to harden containers and further       have to be so rigid in our thinking. Applications can change how
restrict access to external system resources. eBPF has excellent sup-   the kernel makes security decisions, and the kernel can directly
port for the instrumentation of individual cgroups, so this would       manipulate how procesess run to improve security. Implementing
be a natural extension to the current bpfbox implementation.            security mechanisms changes from an either/or problem—do we
   Thanks to bpfbox’s detailed event logging, it should be possible     make security enhancements in userspace or kernelspace?—to both.
to integrate audit2allow-like [42] functionality such that profiles     We can cross the boundaries to mitigate vulnerabilities just as
can be automatically generated through the analysis of permissive       attackers cross boundaries to exploit them.
mode audit logs. This should allow even inexperienced stakeholders         By exposing LSM hooks, bpfbox makes it feasible to add sandbox-
to quickly generate working bpfbox policies that capture normal         ing to any process. It also potentially enables security mitigations
application behavior. These policies should then be relatively easy     that go beyond any conventional sandbox. Authorizations can be
to audit and extend, due to bpfbox’s simple policy language syntax.     tied to functions in userspace and restrictions placed on core ker-
   Although bpfbox currently depends on the Python 3 bcc toolchain      nel functionality. Further, with modest extensions, security policy
and its LLVM backend, future versions may be ported to libbpf CO-       could be directly tied to program activity. In some contexts such as
RE [34] (Compile Once, Run Everywhere), a framework that uses           denial-of-service attacks, resource allocation can impact security.
BTF type information and an automatically generated header file         When allocations can be observed and manipulated system-wide
to allow eBPF programs to be compiled once and distributed to           and communicated to critical processes, resource allocations can be
any compatible Linux configuration. This will enable bpfbox to be       made in light of security priorities, rather than simply attempting
deployed on embedded systems where the disk space and runtime           to be fair to all requesters. The design space of potential mecha-
overhead of its dependencies would otherwise be prohibitive.            nisms is very large, and we believe there is a huge opportunity in
                                                                        exploring it.

9.3     Future Directions                                               10    CONCLUSION
eBPF provides a verifiably safe way to add functionality to the         We have presented the design and implementation of bpfbox, a
Linux kernel at runtime. Code can be triggered by and respond to        novel process confinement engine written entirely in eBPF using
a wide variety of kernel-level and userspace events. While this sort    the KRSI framework. We leverage the flexibility of eBPF to provide
of code could always be written, eBPF simplifies its development:       finer-grained instrumentation on application behavior than previ-
terminating the eBPF loading process will cause the eBPF code to be     ously possible and integrate this information with an extensible
safely unloaded without requiring a system reboot. Further, eBPF’s      enforcement framework that enables simple yet expressive poli-
shared maps and buffered event output mechanisms allows for             cies. We are confident that bpfbox can be extended to meet the
easy, performant data exchange between userspace and kernelspace.       confinement requirements of almost any application in practice.
Many others are excited about the performance gains that eBPF can          State-of-the-art confinement solutions like snap and docker
bring to applications [7, 11, 49]. We suspect, however, that eBPF       combine various complex and heterogeneous mechanisms for limit-
may have an even bigger impact on security.                             ing access to system resources. What these approaches all lack is a
   A central challenge in systems security is how to appropriately      unified solution built from the ground up for process confinement.
recognize and respond to security-critical events. Sometimes it is      Such a unified approach could potentially have significant benefits
clear, as with opening a file, that an action has potential security    for cloud applications by allowing for simplified and more secure
implications. With complex systems, such as those underpinning          container management, even enabling new application deployment
large cloud infrastructures, security problems can arise in almost      abstractions. We believe bpfbox is a step towards such a unified
any context. As attacks evolve, defenders must evolve as well. To-      solution.
day this evolution manifests as a never-ending series of software
updates. Applications must be updated because that is our primary       11    ACKNOWLEDGMENTS
means by which vulnerabilities can be mitigated. But what if we         This research is supported by the Natural Sciences and Engineering
had other options?                                                      Research Council of Canada (NSERC) through a Discovery Grant.
   Sandboxing is ultimately an aspiration rather than a technical
mechanism. In principle, a proper reference monitor architecture
would prevent untrusted code from ever violating system policy;
in practice, shared code, shared state, and imprecise policies allow
for numerous opportunities for sandbox escapes and privilege es-
calation. If we want to do better, we have to rethink our approach
to the boundaries around our computations.
bpfbox: Simple Precise Process Confinement with eBPF                                                                           CCSW’20, November 9, 2020, Virtual Event, USA


REFERENCES                                                                                 [24] K Jain and R Sekar. 2000. User-Level Infrastructure for System Call Interposition:
 [1] Mike Accetta, Robert Baron, William Bolosky, David Golub, Richard Rashid,                  A Platform for Intrusion Detection and Confinement. In NDSS. https://www.cs.
     Avadis Tevanian, and Michael Young. 1986. Mach: A new kernel foundation for                unc.edu/~fabian/course_papers/jain-userlevel.pdf
     UNIX development. In Proceedings of Summer Usenix. http://cseweb.ucsd.edu/            [25] Poul-Henning Kamp and Robert N M Watson. 2000. Jails: Confining the om-
     classes/wi11/cse221/papers/accetta86.pdf                                                   nipotent root. In 2nd International SANE Conference. http://ivanlef0u.fr/repo/
 [2] bcc authors. 2020. iovisor/bcc. The IOVisor Project. https://github.com/iovisor/bcc        madchat/sysadm/bsd/kamp.pdf
 [3] Brian N. Bershad, Craig Chambers, Susan Eggers, Chris Maeda, Dylan McNamee,           [26] Michael Kerrisk. 2013. Namespaces in operation, part 1: namespaces overview.
     Przemyslaw Pardyak, Stefan Savage, and Emin Gün Sirer. 1994. SPIN: An Exten-               LWN.net (January 4 2013). https://lwn.net/Articles/332974/
     sible Microkernel for Application-Specific Operating System Services. In Proceed-     [27] Taesoo Kim and Nickolai Zeldovich. 2013. Practical and Effective Sandboxing for
     ings of the 6th Workshop on ACM SIGOPS European Workshop: Matching Operating               Non-root Users. In 2013 USENIX Annual Technical Conference (USENIX ATC 13).
     Systems to Application Needs (Wadern, Germany) (EW 6). Association for Comput-             USENIX Association, San Jose, CA, 139–144. https://www.usenix.org/system/
                                                                                                files/conference/atc13/atc13-kim.pdf
     ing Machinery, New York, NY, USA, 68–71. https://doi.org/10.1145/504390.504408
                                                                                           [28] Butler W. Lampson. 1973. A note on the confinement problem. Commun. ACM
 [4] Matt Bishop and Michael Dilger. 1996. Checking for Race Conditions in File
                                                                                                16, 10 (Oct. 1973), 613–615. https://doi.org/10.1145/362375.362389
     Accesses. Computing Systems 9, 2 (1996), 131–152. https://static.usenix.org/
                                                                                           [29] Michael Larabel and Matthew Tippett. 2011. Phoronix Test Suite. http://www.
     publications/compsystems/1996/spr_bishop.pdf
                                                                                                phoronix-test-suite.com/
 [5] Bubblewrap authors. 2020. Bubblewrap.                https://github.com/containers/
                                                                                           [30] libbpf authors. 2020. libbpf. https://github.com/libbpf/libbpf
     bubblewrap
                                                                                           [31] J. Liedtke. 1995. On Micro-Kernel Construction. In Proceedings of the Fifteenth
 [6] Bryan M. Cantrill, Michael W. Shapiro, and Adam H. Leventhal. 2004. Dynamic
                                                                                                ACM Symposium on Operating Systems Principles (Copper Mountain, Colorado,
     Instrumentation of Production Systems. In Proceedings of the Annual Conference
                                                                                                USA) (SOSP ’95). Association for Computing Machinery, New York, NY, USA,
     on USENIX Annual Technical Conference (Boston, MA) (ATEC ’04). USENIX Asso-
                                                                                                237–250. https://doi.org/10.1145/224056.224075
     ciation, Berkeley, CA, USA, 2–2. https://www.usenix.org/legacy/publications/
                                                                                           [32] Steven McCanne and Van Jacobson. 1992. The BSD Packet Filter: A New Ar-
     library/proceedings/usenix04/tech/general/full_papers/cantrill/cantrill.pdf
                                                                                                chitecture for User-level Packet Capture. USENIX Winter 93 (1992). https:
 [7] C. Cassagnes, L. Trestioreanu, C. Joly, and R. State. 2020. The rise of eBPF for
                                                                                                //www.tcpdump.org/papers/bpf-usenix93.pdf
     non-intrusive performance monitoring. In NOMS 2020 - 2020 IEEE/IFIP Network
                                                                                           [33] James Morris, Stephen Smalley, Greg Kroah-Hartman, Chris Wright, and Crispin
     Operations and Management Symposium.
                                                                                                Cowan. 2002. Linux security modules: General security support for the linux
 [8] Kees Cook. 2010. [PATCH] security: Yama LSM. https://lkml.org/lkml/2010/6/21/
                                                                                                kernel. In USENIX Security Symposium. ACM Berkeley, CA, 17–31. https://www.
     407
                                                                                                usenix.org/legacy/event/sec02/full_papers/wright/wright.pdf
 [9] Jonathan Corbet. 2007. Notes from a container. LWN.net (October 29 2007).
                                                                                           [34] Andrii Nakryiko. 2020. BPF Portability and CO-RE. https://facebookmicrosites.
     https://lwn.net/Articles/256389/
                                                                                                github.io/bpf/blog/2020/02/19/bpf-portability-and-co-re.html
[10] Jonathan Corbet. 2019. KRSI — the other BPF security module. LWN.net (Decem-
                                                                                           [35] Andrii Nakryiko. 2020. BPF ring buffer. https://lwn.net/Articles/820559/
     ber 27 2019). https://lwn.net/Articles/808048/
                                                                                           [36] Pradeep Padala. 2002. Playing with ptrace, Part I. Linux Journal 2002, 103 (2002),
[11] Glauber Costa. 2020. How io_uring and eBPF Will Revolutionize Programming
                                                                                                5. https://www.linuxjournal.com/article/6100
     in Linux. https://thenewstack.io/how-io_uring-and-ebpf-will-revolutionize-
                                                                                           [37] David S Peterson, Matt Bishop, and Raju Pandey. 2002. A Flexible Containment
     programming-in-linux/
                                                                                                Mechanism for Executing Untrusted Code. In USENIX Security. https://www.
[12] Crispin Cowan, Steve Beattie, Greg Kroah-Hartman, Calton Pu, Perry Wagle, and
                                                                                                usenix.org/legacy/event/sec02/full_papers/peterson/peterson_html/
     Virgil Gligor. 2000. SubDomain: Parsimonious Server Security. In Proceedings
                                                                                           [38] Daniel Price and Andrew Tucker. 2004. Solaris Zones: Operating System Support
     of the 14st Large Installation Systems Administration Conference (LISA). USENIX
                                                                                                for Consolidating Commercial Workloads.. In LISA, Vol. 4. 241–254. SolarisZones:
     Association, New Orleans, LA, United States. https://www.usenix.org/legacy/
                                                                                                OperatingSystemSupportforConsolidatingCommercialWorkloads
     event/lisa2000/full_papers/cowan/cowan.pdf
                                                                                           [39] Niels Provos. 2003. Improving Host Security with System Call Policies. In USENIX
[13] Docker. 2020. Docker security. https://docs.docker.com/engine/security/security/
                                                                                                Security. https://www.usenix.org/legacy/events/sec03/tech/full_papers/provos/
[14] Will Drewry. 2012. Dynamic seccomp policies (using BPF filters). https://lwn.net/
                                                                                                provos_html/
     Articles/475019/
                                                                                           [40] Mickael Salaun. 2020. landlock.io. https://landlock.io/
[15] Firejail authors. 2020. Firejail. https://firejail.wordpress.com/
                                                                                           [41] Seccomp authors. 2020. Seccomp BPF (SECure COMPuting with filters). https:
[16] Flatpak authors. 2020. Flatpak. https://flatpak.org/
                                                                                                //www.kernel.org/doc/html/latest/userspace-api/seccomp_filter.html
[17] Tal Garfinkel, Ben Pfaff, and Mendel Rosenblum. 2004. Ostia: A Delegating
                                                                                           [42] SELinux authors. 2020. SELinux Userspace Tools.               https://github.com/
     Architecture for Secure System Call Interposition.. In Network and Distributed
                                                                                                SELinuxProject/selinux
     System Security (NDSS) Symposium. Internet Society, San Diego, California. https:
                                                                                           [43] Rui Shu, Peipei Wang, Sigmund A Gorski III, Benjamin Andow, Adwait Nadkarni,
     //www.ndss-symposium.org/wp-content/uploads/2017/09/Ostia-A-Delegating-
                                                                                                Luke Deshotels, Jason Gionta, William Enck, and Xiaohui Gu. 2016. A Study of
     Architecture-for-Secure-System-Call-Interposition-Tal-Garfinke.pdf
                                                                                                Security Isolation Techniques. Comput. Surveys 49, 3 (Dec. 2016), 1–37. https:
[18] Ian Goldberg, David Wagner, Randi Thomas, and Eric Brewer. 1996. A Se-
                                                                                                //doi.org/10.1145/2988545
     cure Environment for Untrusted Helper Applications (Confining the Wily
                                                                                           [44] KP Singh. 2019. MAC and Audit policy using eBPF (KRSI). https://lwn.net/ml/linux-
     Hacker). In USENIX Security. https://www.usenix.org/legacy/publications/
                                                                                                kernel/20191220154208.15895-1-kpsingh@chromium.org/
     library/proceedings/sec96/full_papers/goldberg/goldberg.pdf
                                                                                           [45] Stephen Smalley, Chris Vance, and Wayne Salamon. 2001. Implementing SELinux
[19] Brendan Gregg and Jim Mauro. 2011. DTrace: Dynamic Tracing in Oracle Solaris,
                                                                                                as a Linux security module. NAI Labs Report 1, 43, 139. https://www.cs.unibo.it/
     Mac OS X, and FreeBSD (1st ed.). Prentice Hall.
                                                                                                ~sacerdot/doc/so/slm/selinux-module.pdf
[20] Andreas Gruenbacher and Seth Arnold. 2007. AppArmor Technical Documenta-
                                                                                           [46] Snapcraft. 2020. Security policy and sandboxing. https://snapcraft.io/docs/
     tion. http://lkml.iu.edu/hypermail/linux/kernel/0706.1/0805/techdoc.pdf
                                                                                                security-sandboxing
[21] Toshiharu Harada, Takashi Horie, and Kazuo Tanaka. 2005. Towards a manage-
                                                                                           [47] Alexei Starovoitov and Daniel Borkmann. 2014.                    Rework/optimize
     able Linux security. In Linux Conference, Vol. 2005. https://osdn.net/projects/
                                                                                                internal BPF interpreter’s instruction set.          Kernel Patch.          https:
     tomoyo/docs/lc2005-en.pdf/en/2/lc2005-en.pdf
                                                                                                //git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=
[22] Andrew Hurst. 2004. Analysis of Perl’s taint mode. http://hurstdog.org/papers/
                                                                                                bd4cf0ed331a275e9bf5a49e6d0fd55dffc551b8
     hurst04taint.pdf
                                                                                           [48] Robert NM Watson, Jonathan Anderson, Ben Laurie, and Kris Kennaway. 2010.
[23] Mahya Soleimani Jadidi, Mariusz Zaborski, Brian Kidney, and Jonathan Anderson.
                                                                                                Capsicum: Practical Capabilities for UNIX.. In USENIX Security Symposium, Vol. 46.
     2019. CapExec: Towards Transparently-Sandboxed Services. In International
                                                                                                2. https://www.usenix.org/legacy/event/sec10/tech/full_papers/Watson.pdf
     Conference on Network and Service Management (CNSM). IEEE. https://doi.org/
                                                                                           [49] zoidbergwill et al. 2020. Awesome eBPF: A curated list of awesome projects related
     10.23919/CNSM46954.2019.9012736
                                                                                                to eBPF. https://github.com/zoidbergwill/awesome-ebpf
