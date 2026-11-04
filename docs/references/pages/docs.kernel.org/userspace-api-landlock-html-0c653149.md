---
url: "https://docs.kernel.org/userspace-api/landlock.html"
title: "Landlock: unprivileged access control — The Linux Kernel documentation"
captured: "2026-11-02T18:30:05Z"
via: "live"
sha256: "53138f46deaea001289f20983b7e7e2cf8faf5b7c6063fcbbfeb070a44dba5c0"
wayback: "https://web.archive.org/web/20261102195338/https://docs.kernel.org/userspace-api/landlock.html"
cited_in:
  - "REVIEW.md"
---

# Landlock: unprivileged access control — The Linux Kernel documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.kernel.org/userspace-api/landlock.html>
> Snapshot: <https://web.archive.org/web/20261102195338/https://docs.kernel.org/userspace-api/landlock.html>

https://docs.kernel.org/index.html

# [The Linux Kernel](https://docs.kernel.org/index.html)

7.4.0-rc1

### Quick search

### Contents

Working with the development community

- [Development process](https://docs.kernel.org/process/development-process.html)
- [Submitting patches](https://docs.kernel.org/process/submitting-patches.html)
- [Code of conduct](https://docs.kernel.org/process/code-of-conduct.html)
- [Maintainer handbook](https://docs.kernel.org/maintainer/index.html)
- [All development-process docs](https://docs.kernel.org/process/index.html)

Internal API manuals

- [Core API](https://docs.kernel.org/core-api/index.html)
- [Driver APIs](https://docs.kernel.org/driver-api/index.html)
- [Subsystems](https://docs.kernel.org/subsystem-apis.html)
- [Locking](https://docs.kernel.org/locking/index.html)

Development tools and processes

- [Licensing rules](https://docs.kernel.org/process/license-rules.html)
- [Writing documentation](https://docs.kernel.org/doc-guide/index.html)
- [Development tools](https://docs.kernel.org/dev-tools/index.html)
- [Testing guide](https://docs.kernel.org/dev-tools/testing-overview.html)
- [Hacking guide](https://docs.kernel.org/kernel-hacking/index.html)
- [Tracing](https://docs.kernel.org/trace/index.html)
- [Fault injection](https://docs.kernel.org/fault-injection/index.html)
- [Livepatching](https://docs.kernel.org/livepatch/index.html)
- [Rust](https://docs.kernel.org/rust/index.html)

User-oriented documentation

- [Administration](https://docs.kernel.org/admin-guide/index.html)
- [Build system](https://docs.kernel.org/kbuild/index.html)
- [Reporting issues](https://docs.kernel.org/admin-guide/reporting-issues.html)
- [Userspace tools](https://docs.kernel.org/tools/index.html)
- [Userspace API](https://docs.kernel.org/userspace-api/index.html)

- [System calls](https://docs.kernel.org/userspace-api/index.html#system-calls)
- [Security-related interfaces](https://docs.kernel.org/userspace-api/index.html#security-related-interfaces)

- [No New Privileges Flag](https://docs.kernel.org/userspace-api/no_new_privs.html)
- [Seccomp BPF (SECure COMPuting with filters)](https://docs.kernel.org/userspace-api/seccomp_filter.html)
- [Landlock: unprivileged access control](https://docs.kernel.org/userspace-api/landlock.html#)
- [Linux Security Modules](https://docs.kernel.org/userspace-api/lsm.html)
- [Introduction of non-executable mfd](https://docs.kernel.org/userspace-api/mfd_noexec.html)
- [Speculation Control](https://docs.kernel.org/userspace-api/spec_ctrl.html)
- [TEE (Trusted Execution Environment) Userspace API](https://docs.kernel.org/userspace-api/tee.html)
- [Executability check](https://docs.kernel.org/userspace-api/check_exec.html)

- [Devices and I/O](https://docs.kernel.org/userspace-api/index.html#devices-and-i-o)
- [Everything else](https://docs.kernel.org/userspace-api/index.html#everything-else)

Firmware-related documentation

- [Firmware](https://docs.kernel.org/firmware-guide/index.html)
- [Firmware and Devicetree](https://docs.kernel.org/devicetree/index.html)

Architecture-specific documentation

- [CPU architectures](https://docs.kernel.org/arch/index.html)

Other documentation

- [Unsorted documentation](https://docs.kernel.org/staging/index.html)

Translations

- [Translations](https://docs.kernel.org/translations/index.html)

### This Page

- [Show Source](https://docs.kernel.org/_sources/userspace-api/landlock.rst.txt)

# Landlock: unprivileged access control[¶](https://docs.kernel.org/userspace-api/landlock.html#landlock-unprivileged-access-control)

Author:

Mickaël Salaün

Date:

October 2026

The goal of Landlock is to enable restriction of ambient rights (e.g. global filesystem or network access) for a set of processes. Because Landlock is a stackable LSM, it makes it possible to create safe security sandboxes as new security layers in addition to the existing system-wide access-controls. This kind of sandbox is expected to help mitigate the security impact of bugs or unexpected/malicious behaviors in user space applications. Landlock empowers any process, including unprivileged ones, to securely restrict themselves.

We can quickly make sure that Landlock is enabled in the running system by looking for “landlock: Up and running” in kernel logs (as root): dmesg | grep landlock || journalctl -kb -g landlock . Developers can also easily check for Landlock support with a [related system call](https://docs.kernel.org/userspace-api/landlock.html#landlock-abi-versions). If Landlock is not currently supported, we need to [configure the kernel appropriately](https://docs.kernel.org/userspace-api/landlock.html#kernel-support).

## Landlock rules[¶](https://docs.kernel.org/userspace-api/landlock.html#landlock-rules)

A Landlock rule describes the actions a process is allowed to perform on a specific resource. A set of rules is aggregated in a ruleset, which can then restrict the thread enforcing it, and its future children.

The existing types of rules are:

Filesystem rules

The rule key is a file hierarchy, and the actions it allows are defined with filesystem access rights.

Network rules (since ABI v4 for TCP and v10 for UDP)

For these rules, the object is a TCP or UDP port, and the related actions are defined with network access rights.

Capability rules (since ABI v12)

The rule body lists which Linux capabilities (see [capabilities(7)](https://man7.org/linux/man-pages/man7/capabilities.7.html)) the process may exercise; the action is defined with permission flags.

Namespace rules (since ABI v12)

The rule body lists which namespace types (see [namespaces(7)](https://man7.org/linux/man-pages/man7/namespaces.7.html)) the process may use; the action is defined with permission flags.

### Defining and enforcing a security policy[¶](https://docs.kernel.org/userspace-api/landlock.html#defining-and-enforcing-a-security-policy)

We first need to define the ruleset that will contain our rules.

For this example, the ruleset will contain rules that only allow some filesystem read actions and some specific UDP and TCP actions. Filesystem write actions and other TCP/UDP actions will be denied.

The ruleset then needs to handle all these kinds of actions. This is required for backward and forward compatibility (i.e. the kernel and user space may not know each other’s supported restrictions), hence the need to be explicit about the denied-by-default access rights.

```
struct landlock_ruleset_attr ruleset_attr = {
    .handled_access_fs =
        LANDLOCK_ACCESS_FS_EXECUTE |
        LANDLOCK_ACCESS_FS_WRITE_FILE |
        LANDLOCK_ACCESS_FS_READ_FILE |
        LANDLOCK_ACCESS_FS_READ_DIR |
        LANDLOCK_ACCESS_FS_REMOVE_DIR |
        LANDLOCK_ACCESS_FS_REMOVE_FILE |
        LANDLOCK_ACCESS_FS_MAKE_CHAR |
        LANDLOCK_ACCESS_FS_MAKE_DIR |
        LANDLOCK_ACCESS_FS_MAKE_REG |
        LANDLOCK_ACCESS_FS_MAKE_SOCK |
        LANDLOCK_ACCESS_FS_MAKE_FIFO |
        LANDLOCK_ACCESS_FS_MAKE_BLOCK |
        LANDLOCK_ACCESS_FS_MAKE_SYM |
        LANDLOCK_ACCESS_FS_REFER |
        LANDLOCK_ACCESS_FS_TRUNCATE |
        LANDLOCK_ACCESS_FS_IOCTL_DEV |
        LANDLOCK_ACCESS_FS_RESOLVE_UNIX,
    .handled_access_net =
        LANDLOCK_ACCESS_NET_BIND_TCP |
        LANDLOCK_ACCESS_NET_CONNECT_TCP |
        LANDLOCK_ACCESS_NET_BIND_UDP |
        LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,
    .scoped =
        LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET |
        LANDLOCK_SCOPE_SIGNAL,
    .handled_permissions =
        LANDLOCK_PERMISSION_NAMESPACE_USE |
        LANDLOCK_PERMISSION_CAPABILITY_USE,
};

```

Because we may not know which kernel version an application will be executed on, it is safer to follow a best-effort security approach. Indeed, we should try to protect users as much as possible whatever the kernel they are using.

To be compatible with older Linux versions, we detect the available Landlock ABI version, and only use the available subset of access rights:

```
int abi;

abi = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_VERSION);
if (abi < 0) {
    /* Degrades gracefully if Landlock is not handled. */
    perror("The running kernel does not enable to use Landlock");
    return 0;
}
switch (abi) {
case 1:
    /* Removes LANDLOCK_ACCESS_FS_REFER for ABI < 2 */
    ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_REFER;
    __attribute__((fallthrough));
case 2:
    /* Removes LANDLOCK_ACCESS_FS_TRUNCATE for ABI < 3 */
    ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_TRUNCATE;
    __attribute__((fallthrough));
case 3:
    /* Removes network support for ABI < 4 */
    ruleset_attr.handled_access_net &=
        ~(LANDLOCK_ACCESS_NET_BIND_TCP |
          LANDLOCK_ACCESS_NET_CONNECT_TCP);
    __attribute__((fallthrough));
case 4:
    /* Removes LANDLOCK_ACCESS_FS_IOCTL_DEV for ABI < 5 */
    ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_IOCTL_DEV;
    __attribute__((fallthrough));
case 5:
    /* Removes LANDLOCK_SCOPE_* for ABI < 6 */
    ruleset_attr.scoped &= ~(LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET |
                             LANDLOCK_SCOPE_SIGNAL);
    __attribute__((fallthrough));
case 6 ... 8:
    /* Removes LANDLOCK_ACCESS_FS_RESOLVE_UNIX for ABI < 9 */
    ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_RESOLVE_UNIX;
    __attribute__((fallthrough));
case 9:
    /* Removes LANDLOCK_ACCESS_NET_*_UDP for ABI < 10 */
    ruleset_attr.handled_access_net &=
        ~(LANDLOCK_ACCESS_NET_BIND_UDP |
          LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
    __attribute__((fallthrough));
case 10:
case 11:
    /* Removes LANDLOCK_PERMISSION_* for ABI < 12 */
    ruleset_attr.handled_permissions &=
        ~(LANDLOCK_PERMISSION_NAMESPACE_USE |
          LANDLOCK_PERMISSION_CAPABILITY_USE);
}

```

This enables the creation of an inclusive ruleset that will contain our rules.

```
int ruleset_fd;

ruleset_fd = landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
if (ruleset_fd < 0) {
    perror("Failed to create a ruleset");
    return 1;
}

```

We can now add a new rule to this ruleset thanks to the returned file descriptor referring to this ruleset. The rule will allow reading and executing the file hierarchy /usr. Without another rule, write actions would then be denied by the ruleset. To add /usr to the ruleset, we open it with the O_PATH flag and fill the &[struct landlock_path_beneath_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_path_beneath_attr) with this file descriptor.

```
int err = 0;
struct landlock_path_beneath_attr path_beneath = {
    .allowed_access =
        LANDLOCK_ACCESS_FS_EXECUTE |
        LANDLOCK_ACCESS_FS_READ_FILE |
        LANDLOCK_ACCESS_FS_READ_DIR,
};

path_beneath.allowed_access &= ruleset_attr.handled_access_fs;
if (path_beneath.allowed_access) {
    path_beneath.parent_fd = open("/usr", O_PATH | O_CLOEXEC);
    if (path_beneath.parent_fd < 0) {
        perror("Failed to open file");
        close(ruleset_fd);
        return 1;
    }
    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_PATH_BENEATH,
                            &path_beneath, 0);
    close(path_beneath.parent_fd);
    if (err) {
        perror("Failed to update ruleset");
        close(ruleset_fd);
        return 1;
    }
}

```

As shown above, masking the rule’s allowed_access against the ruleset’s handled_access_* is the recommended best-effort pattern: rights the running kernel does not support are dropped (the compatibility switch above already cleared them in handled_access_*), and the rule is skipped if no supported right remains.

For network access-control, we will add a set of rules to allow DNS queries, which requires both UDP and TCP. For TCP, we need to allow outbound connections to port 53, which can be handled and granted starting with ABI 4:

```
struct landlock_net_port_attr tcp_conn = {
    .allowed_access = LANDLOCK_ACCESS_NET_CONNECT_TCP,
    .port = 53,
};

tcp_conn.allowed_access &= ruleset_attr.handled_access_net;
if (tcp_conn.allowed_access)
    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
                            &tcp_conn, 0);

```

We also need to be able to send UDP datagrams to port 53, which requires granting LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP. Since our DNS client will emit datagrams without explicitly binding to a specific source port, its UDP socket will automatically bind an ephemeral port. To allow this behaviour, we also need to grant LANDLOCK_ACCESS_NET_BIND_UDP on port 0, as if the program explicitly called [bind(2)](https://man7.org/linux/man-pages/man2/bind.2.html) on port 0.

```
struct landlock_net_port_attr udp_send = {
    .allowed_access = LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,
    .port = 53,
};

udp_send.allowed_access &= ruleset_attr.handled_access_net;
if (udp_send.allowed_access)
    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
                            &udp_send, 0);
[...]

struct landlock_net_port_attr udp_bind = {
    .allowed_access = LANDLOCK_ACCESS_NET_BIND_UDP,
    .port = 0,
};

udp_bind.allowed_access &= ruleset_attr.handled_access_net;
if (udp_bind.allowed_access)
    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
                            &udp_bind, 0);

```

Capability and namespace rules use a different attribute layout: permissions identifies the permission category (a single LANDLOCK_PERMISSION_* flag) and a type-specific value field carries the bitmask to allow within it. See [Capability and namespace restrictions](https://docs.kernel.org/userspace-api/landlock.html#capability-and-namespace-restrictions) for the model.

For capability access-control, we can add rules that allow specific capabilities (see [capabilities(7)](https://man7.org/linux/man-pages/man7/capabilities.7.html) for the list of CAP_* values). For instance, to allow CAP_SYS_CHROOT (so the sandboxed process can call [chroot(2)](https://man7.org/linux/man-pages/man2/chroot.2.html) inside a user namespace):

```
struct landlock_capability_attr cap_attr = {
    .permissions = LANDLOCK_PERMISSION_CAPABILITY_USE,
    .allowed_capabilities = (1ULL << CAP_SYS_CHROOT),
};

cap_attr.permissions &= ruleset_attr.handled_permissions;
if (cap_attr.permissions)
    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_CAPABILITY,
                            &cap_attr, 0);

```

For namespace access-control, we can add rules that allow using specific namespace types (see [namespaces(7)](https://man7.org/linux/man-pages/man7/namespaces.7.html) for the list of CLONE_NEW* values): creating them via [unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html) / [clone(2)](https://man7.org/linux/man-pages/man2/clone.2.html) / [clone3(2)](https://man7.org/linux/man-pages/man2/clone3.2.html), joining them via [setns(2)](https://man7.org/linux/man-pages/man2/setns.2.html), or acquiring an fd reference via [open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html) / [fsmount(2)](https://man7.org/linux/man-pages/man2/fsmount.2.html). For instance, to allow creating user namespaces (which grants all capabilities inside the new namespace):

```
struct landlock_namespace_attr ns_attr = {
    .permissions = LANDLOCK_PERMISSION_NAMESPACE_USE,
    .allowed_namespace_types = CLONE_NEWUSER,
};

ns_attr.permissions &= ruleset_attr.handled_permissions;
if (ns_attr.permissions)
    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NAMESPACE,
                            &ns_attr, 0);

```

Together, these two rules allow an unprivileged process to create a user namespace and call [chroot(2)](https://man7.org/linux/man-pages/man2/chroot.2.html) inside it, while denying all other capabilities and namespace types. User namespace creation is the one operation that does not require CAP_SYS_ADMIN, so no capability rule is needed for it. See [Capability and namespace restrictions](https://docs.kernel.org/userspace-api/landlock.html#capability-and-namespace-restrictions) for details on capability requirements.

When passing a non-zero flags argument to landlock_restrict_self(), a similar backwards compatibility check is needed for the restrict flags (see [sys_landlock_restrict_self()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_restrict_self) documentation for available flags):

```
__u32 restrict_flags =
    LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON |
    LANDLOCK_RESTRICT_SELF_TSYNC |
    LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS;
switch (abi) {
case 1 ... 6:
    /* Removes logging flags for ABI < 7 */
    restrict_flags &= ~(LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF |
                        LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON |
                        LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF);
    __attribute__((fallthrough));
case 7:
    /*
     * Removes multithreaded enforcement flag for ABI < 8
     *
     * WARNING: Without this flag, calling landlock_restrict_self(2) is
     * only equivalent if the calling process is single-threaded. Below
     * ABI v8 (and as of ABI v8, when not using this flag), a Landlock
     * policy would only be enforced for the calling thread and its
     * children (and not for all threads, including parents and siblings).
     */
    restrict_flags &= ~LANDLOCK_RESTRICT_SELF_TSYNC;
    __attribute__((fallthrough));
case 8 ... 10:
    /* Removes no new privs flag for ABI < 11 */
    restrict_flags &= ~LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS;
}

```

The next step is to restrict the current thread from gaining more privileges (e.g. through a SUID binary). For unprivileged processes, setting the no_new_privs attribute is required by Landlock.

Processes with CAP_SYS_ADMIN in their namespace can enforce a ruleset without setting no_new_privs, but leaving no_new_privs unset is risky even when Landlock does not require this attribute: sandboxed processes could still execute set-user-ID, set-group-ID or file-capability binaries, which would then run with elevated privileges while being restricted by a Landlock domain they may not expect, making them potential confused deputies. no_new_privs should only be left unset if such a privilege transition is expected.

We now have a ruleset with the first rule allowing read and execute access to /usr while denying all other handled accesses for the filesystem, and two more rules allowing DNS queries.

```
/*
 * If the ABI > 10, we can tie setting no_new_privs with successful ruleset
 * enforcement and skip the manual prctl(PR_SET_NO_NEW_PRIVS, ...) call.
 */
if (!(restrict_flags & LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS) &&
    prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)) {
    perror("Failed to restrict privileges");
    close(ruleset_fd);
    return 1;
}

```

The current thread is now ready to sandbox itself with the ruleset.

```
if (landlock_restrict_self(ruleset_fd, restrict_flags)) {
    perror("Failed to enforce ruleset");
    close(ruleset_fd);
    return 1;
}
close(ruleset_fd);

```

If the landlock_restrict_self system call succeeds, the current thread is now restricted and this policy will be enforced on all its subsequently created children as well. Once a thread is landlocked, there is no way to remove its security policy; only adding more restrictions is allowed. These threads are now in a new Landlock domain, which is a merger of their parent one (if any) with the new ruleset.

Full working code can be found in [samples/landlock/sandboxer.c](https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/samples/landlock/sandboxer.c).

### Good practices[¶](https://docs.kernel.org/userspace-api/landlock.html#good-practices)

It is recommended to set access rights to file hierarchy leaves as much as possible. For instance, it is better to be able to have ~/doc/ as a read-only hierarchy and ~/tmp/ as a read-write hierarchy, compared to ~/ as a read-only hierarchy and ~/tmp/ as a read-write hierarchy. Following this good practice leads to self-sufficient hierarchies that do not depend on their location (i.e. parent directories). This is particularly relevant when we want to allow linking or renaming. Indeed, having consistent access rights per directory enables changing the location of such directories without relying on the destination directory access rights (except those that are required for this operation, see LANDLOCK_ACCESS_FS_REFER documentation).

Having self-sufficient hierarchies also helps to tighten the required access rights to the minimal set of data. This also helps avoid sinkhole directories, i.e. directories where data can be linked to but not linked from. However, this depends on data organization, which might not be controlled by developers. In this case, granting read-write access to ~/tmp/, instead of write-only access, would potentially allow moving ~/tmp/ to a non-readable directory and still keep the ability to list the content of ~/tmp/.

### Layers of file path access rights[¶](https://docs.kernel.org/userspace-api/landlock.html#layers-of-file-path-access-rights)

Each time a thread enforces a ruleset on itself, it updates its Landlock domain with a new layer of policy. This complementary policy is stacked with any other rulesets potentially already restricting this thread. A sandboxed thread can then safely add more constraints to itself with a new enforced ruleset.

One policy layer grants access to a file path if at least one of its rules encountered on the path grants the access. A sandboxed thread can only access a file path if all its enforced policy layers grant the access as well as all the other system access controls (e.g. filesystem DAC, other LSM policies, etc.).

### Bind mounts and OverlayFS[¶](https://docs.kernel.org/userspace-api/landlock.html#bind-mounts-and-overlayfs)

Landlock enables restricting access to file hierarchies, which means that these access rights can be propagated with bind mounts (cf. [Shared Subtrees](https://docs.kernel.org/filesystems/sharedsubtree.html)) but not with [Overlay Filesystem](https://docs.kernel.org/filesystems/overlayfs.html).

A bind mount mirrors a source file hierarchy to a destination. The destination hierarchy is then composed of the exact same files, on which Landlock rules can be tied, either via the source or the destination path. These rules restrict access when they are encountered on a path, which means that they can restrict access to multiple file hierarchies at the same time, whether these hierarchies are the result of bind mounts or not.

An OverlayFS mount point consists of upper and lower layers. These layers are combined in a merge directory, and that merged directory becomes available at the mount point. This merge hierarchy may include files from the upper and lower layers, but modifications performed on the merge hierarchy only reflect on the upper layer. From a Landlock policy point of view, all OverlayFS layers and merge hierarchies are standalone and each contains their own set of files and directories, which is different from bind mounts. A policy restricting an OverlayFS layer will not restrict the resulted merged hierarchy, and vice versa. Landlock users should then only think about file hierarchies they want to allow access to, regardless of the underlying filesystem.

### Inheritance[¶](https://docs.kernel.org/userspace-api/landlock.html#inheritance)

Every new thread resulting from a [clone(2)](https://man7.org/linux/man-pages/man2/clone.2.html) inherits Landlock domain restrictions from its parent. This is similar to seccomp inheritance (cf. [Seccomp BPF (SECure COMPuting with filters)](https://docs.kernel.org/userspace-api/seccomp_filter.html)) or any other LSM dealing with task’s [credentials(7)](https://man7.org/linux/man-pages/man7/credentials.7.html). For instance, one process’s thread may apply Landlock rules to itself, but they will not be automatically applied to other sibling threads (unlike POSIX thread credential changes, cf. [nptl(7)](https://man7.org/linux/man-pages/man7/nptl.7.html)).

When a thread sandboxes itself, we have the guarantee that the related security policy will stay enforced on all this thread’s descendants. This allows creating standalone and modular security policies per application, which will automatically be composed between themselves according to their runtime parent policies.

### Ptrace restrictions[¶](https://docs.kernel.org/userspace-api/landlock.html#ptrace-restrictions)

A sandboxed process has less privileges than a non-sandboxed process and must then be subject to additional restrictions when manipulating another process. To be allowed to use [ptrace(2)](https://man7.org/linux/man-pages/man2/ptrace.2.html) and related syscalls on a target process, a sandboxed process should have a superset of the target process’s access rights, which means the tracee must be in a sub-domain of the tracer.

### IPC scoping[¶](https://docs.kernel.org/userspace-api/landlock.html#ipc-scoping)

Similar to the implicit [Ptrace restrictions](https://docs.kernel.org/userspace-api/landlock.html#ptrace-restrictions), we may want to further restrict interactions between sandboxes. Therefore, at ruleset creation time, each Landlock domain can restrict the scope for certain operations, so that these operations can only reach out to processes within the same Landlock domain or in a nested Landlock domain (the “scope”).

The operations which can be scoped are:

LANDLOCK_SCOPE_SIGNAL

This limits the sending of signals to target processes which run within the same or a nested Landlock domain.

LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET

This limits the set of abstract [unix(7)](https://man7.org/linux/man-pages/man7/unix.7.html) sockets to which we can [connect(2)](https://man7.org/linux/man-pages/man2/connect.2.html) to socket addresses which were created by a process in the same or a nested Landlock domain.

A [sendto(2)](https://man7.org/linux/man-pages/man2/sendto.2.html) on a non-connected datagram socket is treated as if it were doing an implicit [connect(2)](https://man7.org/linux/man-pages/man2/connect.2.html) and will be blocked if the remote end does not stem from the same or a nested Landlock domain.

A [sendto(2)](https://man7.org/linux/man-pages/man2/sendto.2.html) on a socket which was previously connected will not be restricted. This works for both datagram and stream sockets.

Scoping does not support exceptions via [landlock_add_rule(2)](https://man7.org/linux/man-pages/man2/landlock_add_rule.2.html). If an operation is scoped within a domain, no rules can be added to allow access to resources or processes outside of the scope.

### Capability and namespace restrictions[¶](https://docs.kernel.org/userspace-api/landlock.html#capability-and-namespace-restrictions)

handled_permissions declares per-category permissions: each permission selects which members of a kernel-defined category (CAP_* capabilities, CLONE_NEW* namespace types) the process may use. Unlike per-object access rights (handled_access_*) or cross-domain scopes (scoped), per-category permissions constrain the sandboxed process’s own use of these enums; members not allowed by a rule are denied by default.

LANDLOCK_PERMISSION_NAMESPACE_USE gates acquisition of access to namespaces: creation via [unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html) / [clone(2)](https://man7.org/linux/man-pages/man2/clone.2.html) / [clone3(2)](https://man7.org/linux/man-pages/man2/clone3.2.html), entry via [setns(2)](https://man7.org/linux/man-pages/man2/setns.2.html), and fd-reference acquisition via [open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html) / [fsmount(2)](https://man7.org/linux/man-pages/man2/fsmount.2.html). Namespaces the process is already a member of when the domain is enforced are implicitly allowed (the process could not continue running otherwise); rules describe which new namespace types the process may acquire. LANDLOCK_PERMISSION_CAPABILITY_USE gates every exercise of a capability after the domain is enforced, regardless of how the capability was obtained (inherited credentials, CLONE_NEWUSER grant, setuid/file-cap-bearing [execve(2)](https://man7.org/linux/man-pages/man2/execve.2.html), etc.). Configuring both together restricts what privileges are available and the namespaces in which they take effect, which matters because user namespace creation has no capability check and grants all capabilities within the new namespace: gating only one of the two leaves a kernel attack-surface widening path open. When a domain handles both permissions, an operation is allowed only if each independently allows it: [unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html) with CLONE_NEWNET then requires both a rule allowing CLONE_NEWNET and a rule allowing CAP_SYS_ADMIN.

LANDLOCK_PERMISSION_CAPABILITY_USE complements [prctl(2)](https://man7.org/linux/man-pages/man2/prctl.2.html) PR_SET_NO_NEW_PRIVS but does not replace it. PR_SET_NO_NEW_PRIVS prevents privilege acquisition via [execve(2)](https://man7.org/linux/man-pages/man2/execve.2.html) (setuid, file capability xattrs, privilege-elevating LSM transitions) and is a prerequisite for unprivileged Landlock self-sandboxing. LANDLOCK_PERMISSION_CAPABILITY_USE restricts exercise of capabilities the process already holds, including those gained via CLONE_NEWUSER which PR_SET_NO_NEW_PRIVS does not block. Sandboxes typically set both.

Rules are added with LANDLOCK_RULE_CAPABILITY and &[struct landlock_capability_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_capability_attr) (each rule lists CAP_* values to allow), and with LANDLOCK_RULE_NAMESPACE and &[struct landlock_namespace_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_namespace_attr) (each rule lists CLONE_NEW* flags to allow). Landlock is purely restrictive: it can only deny what the traditional check would have allowed, never grant additional privileges.

Denials of specific capabilities or namespace types can be excluded from audit logging per rule, by listing them in the quiet_capabilities or quiet_namespace_types field of the rule (independently of the allowed members). Quieting requires the permission to be handled, is member-granular, and only suppresses the audit record of a denial attributed to the rule’s own layer; it never changes the denial itself or suppresses its trace event, which reports logged=0. Sandboxes should quiet only specific members known to be requested but expected to be denied (for example for compatibility with older callers), never a blanket set: a blanket set follows the running kernel’s known members and would hide surprising or future denials from audit.

Rule bodies silently accept values unknown to the current kernel (capabilities above CAP_LAST_CAP, unrecognised CLONE_NEW* bits): they have no runtime effect, so a rule compiled against future kernel headers loads without error on older kernels. Future kernels gain new members denied by default until a rule explicitly allows them.

The single LANDLOCK_PERMISSION_NAMESPACE_USE bit gates every kernel path that grants the calling process access to a namespace of the controlled types, whether by becoming a member of the namespace or by holding a file descriptor that references it. The covered syscall paths are:

-

[unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html) with CLONE_NEW*: the caller becomes a member of a newly-created namespace.

-

[clone(2)](https://man7.org/linux/man-pages/man2/clone.2.html) (or [clone3(2)](https://man7.org/linux/man-pages/man2/clone3.2.html)) with CLONE_NEW*: the child becomes a member of a newly-created namespace.

-

[setns(2)](https://man7.org/linux/man-pages/man2/setns.2.html): the caller becomes a member of an existing namespace referenced by file descriptor.

-

[open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html) with OPEN_TREE_NAMESPACE: the caller obtains a file descriptor referring to a newly-created mount namespace.

-

[open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html) with OPEN_TREE_CLONE: the caller obtains a file descriptor referring to a newly-created anonymous mount namespace.

-

[fsmount(2)](https://man7.org/linux/man-pages/man2/fsmount.2.html) with FSMOUNT_NAMESPACE: the caller obtains a file descriptor referring to a newly-created mount namespace.

-

[fsmount(2)](https://man7.org/linux/man-pages/man2/fsmount.2.html) (default): the caller obtains a file descriptor referring to a newly-created anonymous mount namespace.

Anonymous mount namespaces (created by open_tree(OPEN_TREE_CLONE) and the default [fsmount(2)](https://man7.org/linux/man-pages/man2/fsmount.2.html)) are intentionally covered by the bit even though the calling process does not become a member of them. Without this coverage, a sandboxed process could combine open_tree(OPEN_TREE_CLONE) with [move_mount(2)](https://man7.org/linux/man-pages/man2/move_mount.2.html) to graft mounts from a freshly-allocated mount namespace into its current namespace, bypassing a CLONE_NEWNS restriction (these mount operations require CAP_SYS_ADMIN, which a sandboxed process typically obtains by first creating a user namespace).

In practice, unprivileged processes first create a user namespace (which requires no capability and grants all capabilities within it), then use those capabilities to create other namespace types. All non-user namespace types require CAP_SYS_ADMIN for both creation and [setns(2)](https://man7.org/linux/man-pages/man2/setns.2.html) entry; mount namespace entry additionally requires CAP_SYS_CHROOT. For [setns(2)](https://man7.org/linux/man-pages/man2/setns.2.html), capabilities are checked relative to the target namespace, so a process in an ancestor user namespace naturally satisfies them; this includes joining user namespaces, which requires CAP_SYS_ADMIN. When LANDLOCK_PERMISSION_CAPABILITY_USE is also handled, each of these capabilities must be explicitly allowed by a rule.

Creating a non-user namespace always requires CAP_SYS_ADMIN, and LANDLOCK_PERMISSION_CAPABILITY_USE (when handled) gates that check regardless of which user namespace it targets. This is true whether the namespace is created on its own, combined with CLONE_NEWUSER in a single [unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html) call, or in a separate [unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html) call after a user namespace: in all cases the kernel checks CAP_SYS_ADMIN against the owning user namespace, and Landlock denies it unless a rule allows CAP_SYS_ADMIN. Combining CLONE_NEWUSER with another CLONE_NEW* flag therefore does not avoid the capability check; only user namespace creation itself needs no capability rule.

When creating child user namespaces, it is recommended to also create a dedicated Landlock domain with restrictions relevant to each namespace context.

Note that LANDLOCK_PERMISSION_CAPABILITY_USE restricts the use of capabilities, not their presence in the process’s credential. Capability sets can change after a domain is enforced through user namespace entry or [capset(2)](https://man7.org/linux/man-pages/man2/capset.2.html); privileged sandboxes that did not set PR_SET_NO_NEW_PRIVS may also gain capabilities through [execve(2)](https://man7.org/linux/man-pages/man2/execve.2.html) of binaries with file capabilities. In all cases, [capget(2)](https://man7.org/linux/man-pages/man2/capget.2.html) will report the credential’s capability sets, but any denied capability will fail with EPERM when exercised. Do not rely on [capget(2)](https://man7.org/linux/man-pages/man2/capget.2.html) to determine whether the policy permits a given capability; only the actual operation will return EPERM upon denial.

### Truncating files[¶](https://docs.kernel.org/userspace-api/landlock.html#truncating-files)

The operations covered by LANDLOCK_ACCESS_FS_WRITE_FILE and LANDLOCK_ACCESS_FS_TRUNCATE both change the contents of a file and sometimes overlap in non-intuitive ways. It is strongly recommended to always specify both of these together (either granting both, or granting none).

A particularly surprising example is [creat(2)](https://man7.org/linux/man-pages/man2/creat.2.html). The name suggests that this system call requires the rights to create and write files. However, it also requires the truncate right if an existing file under the same name is already present.

It should also be noted that truncating files does not require the LANDLOCK_ACCESS_FS_WRITE_FILE right. Apart from the [truncate(2)](https://man7.org/linux/man-pages/man2/truncate.2.html) system call, this can also be done through [open(2)](https://man7.org/linux/man-pages/man2/open.2.html) with the flags O_RDONLY | O_TRUNC.

At the same time, on some filesystems, [fallocate(2)](https://man7.org/linux/man-pages/man2/fallocate.2.html) offers a way to shorten file contents with FALLOC_FL_COLLAPSE_RANGE when the file is opened for writing, sidestepping the LANDLOCK_ACCESS_FS_TRUNCATE right.

The truncate right is associated with the opened file (see below).

### Rights associated with file descriptors[¶](https://docs.kernel.org/userspace-api/landlock.html#rights-associated-with-file-descriptors)

When opening a file, the availability of the LANDLOCK_ACCESS_FS_TRUNCATE and LANDLOCK_ACCESS_FS_IOCTL_DEV rights is associated with the newly created file descriptor and will be used for subsequent truncation and ioctl attempts using [ftruncate(2)](https://man7.org/linux/man-pages/man2/ftruncate.2.html) and [ioctl(2)](https://man7.org/linux/man-pages/man2/ioctl.2.html). The behavior is similar to opening a file for reading or writing, where permissions are checked during [open(2)](https://man7.org/linux/man-pages/man2/open.2.html), but not during the subsequent [read(2)](https://man7.org/linux/man-pages/man2/read.2.html) and [write(2)](https://man7.org/linux/man-pages/man2/write.2.html) calls.

As a consequence, it is possible that a process has multiple open file descriptors referring to the same file, but Landlock enforces different things when operating with these file descriptors. This can happen when a Landlock ruleset gets enforced and the process keeps file descriptors which were opened both before and after the enforcement. It is also possible to pass such file descriptors between processes, keeping their Landlock properties, even when some of the involved processes do not have an enforced Landlock ruleset.

## Compatibility[¶](https://docs.kernel.org/userspace-api/landlock.html#compatibility)

### Backward and forward compatibility[¶](https://docs.kernel.org/userspace-api/landlock.html#backward-and-forward-compatibility)

Landlock is designed to be compatible with past and future versions of the kernel. This is achieved thanks to the system call attributes and the associated bitflags, particularly the ruleset’s handled_access_fs. Making handled access rights explicit enables the kernel and user space to have a clear contract with each other. This is required to make sure sandboxing will not get stricter with a system update, which could break applications.

Developers can subscribe to the [Landlock mailing list](https://subspace.kernel.org/lists.linux.dev.html) to knowingly update and test their applications with the latest available features. In the interest of users, and because they may use different kernel versions, it is strongly encouraged to follow a best-effort security approach by checking the Landlock ABI version at runtime and only enforcing the supported features.

### Landlock ABI versions[¶](https://docs.kernel.org/userspace-api/landlock.html#landlock-abi-versions)

The Landlock ABI version can be read with the [sys_landlock_create_ruleset()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_create_ruleset) system call:

```
int abi;

abi = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_VERSION);
if (abi < 0) {
    switch (errno) {
    case ENOSYS:
        printf("Landlock is not supported by the current kernel.\n");
        break;
    case EOPNOTSUPP:
        printf("Landlock is currently disabled.\n");
        break;
    }
    return 0;
}
if (abi >= 2) {
    printf("Landlock supports LANDLOCK_ACCESS_FS_REFER.\n");
}

```

All Landlock kernel interfaces are supported by the first ABI version unless explicitly noted in their documentation.

### Landlock errata[¶](https://docs.kernel.org/userspace-api/landlock.html#landlock-errata)

In addition to ABI versions, Landlock provides an errata mechanism to track fixes for issues that may affect backwards compatibility or require userspace awareness. The errata bitmask can be queried using:

```
int errata;

errata = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_ERRATA);
if (errata < 0) {
    /* Landlock not available or disabled */
    return 0;
}

```

The returned value is a bitmask where each bit represents a specific erratum. If bit N is set (errata & (1 << (N - 1))), then erratum N has been fixed in the running kernel.

Warning

Most applications should NOT check errata. In 99.9% of cases, checking errata is unnecessary, increases code complexity, and can potentially decrease protection if misused. For example, disabling the sandbox when an erratum is not fixed could leave the system less secure than using Landlock’s best-effort protection. When in doubt, ignore errata.

#### Erratum 1: TCP socket identification[¶](https://docs.kernel.org/userspace-api/landlock.html#erratum-1-tcp-socket-identification)

This fix addresses an issue where IPv4 and IPv6 stream sockets (e.g., SMC, MPTCP, or SCTP) were incorrectly restricted by TCP access rights during [bind(2)](https://man7.org/linux/man-pages/man2/bind.2.html) and [connect(2)](https://man7.org/linux/man-pages/man2/connect.2.html) operations. This change ensures that only TCP sockets are subject to TCP access rights, allowing other protocols to operate without unnecessary restrictions.

Impact:

In kernels without this fix, using LANDLOCK_ACCESS_NET_BIND_TCP or LANDLOCK_ACCESS_NET_CONNECT_TCP would incorrectly restrict non-TCP stream protocols (SMC, MPTCP, SCTP), potentially breaking applications that rely on these protocols while using Landlock network restrictions.

#### Erratum 2: Scoped signal handling[¶](https://docs.kernel.org/userspace-api/landlock.html#erratum-2-scoped-signal-handling)

This fix addresses an issue where signal scoping was overly restrictive, preventing sandboxed threads from signaling other threads within the same process if they belonged to different domains. Because threads are not security boundaries, user space might assume that all threads within the same process can send signals between themselves (see [nptl(7)](https://man7.org/linux/man-pages/man7/nptl.7.html) and [libpsx(3)](https://man7.org/linux/man-pages/man3/libpsx.3.html)). Consistent with [ptrace(2)](https://man7.org/linux/man-pages/man2/ptrace.2.html) behavior, direct interaction between threads of the same process should always be allowed. This change ensures that any thread is allowed to send signals to any other thread within the same process, regardless of their domain.

Impact:

This problem only manifests when the userspace process is itself using [libpsx(3)](https://man7.org/linux/man-pages/man3/libpsx.3.html) or an equivalent mechanism to enforce a Landlock policy on multiple already-running threads at once. Programs which enforce a Landlock policy at startup time and only then become multithreaded are not affected. Without this fix, signal scoping could break multi-threaded applications that expect threads within the same process to freely signal each other.

#### Erratum 3: Disconnected directory handling[¶](https://docs.kernel.org/userspace-api/landlock.html#erratum-3-disconnected-directory-handling)

This fix addresses an issue with disconnected directories that occur when a directory is moved outside the scope of a bind mount. The change ensures that evaluated access rights include both those from the disconnected file hierarchy down to its filesystem root and those from the related mount point hierarchy. This prevents access right widening through rename or link actions.

Impact:

Without this fix, it was possible to widen access rights through rename or link actions involving disconnected directories, potentially bypassing LANDLOCK_ACCESS_FS_REFER restrictions. This could allow privilege escalation in complex mount scenarios where directories become disconnected from their original mount points.

#### Erratum 4: Creation of whiteout objects[¶](https://docs.kernel.org/userspace-api/landlock.html#erratum-4-creation-of-whiteout-objects)

This fix changes the access rights required for the creation of whiteout objects through [mknod(2)](https://man7.org/linux/man-pages/man2/mknod.2.html), [renameat2(2)](https://man7.org/linux/man-pages/man2/renameat2.2.html), or [link(2)](https://man7.org/linux/man-pages/man2/link.2.html). Creating whiteout objects is now guarded by LANDLOCK_ACCESS_FS_MAKE_REG instead of LANDLOCK_ACCESS_FS_MAKE_CHAR.

Whiteout objects are used in OverlayFS to mark the absence of a file in an upper file system. Despite being created with S_IFCHR, whiteout objects do not count as character devices.

Impact:

Sandboxed programs that create OverlayFS whiteouts (such as fuse-overlayfs) now require LANDLOCK_ACCESS_FS_MAKE_REG instead of LANDLOCK_ACCESS_FS_MAKE_CHAR.

#### How to check for errata[¶](https://docs.kernel.org/userspace-api/landlock.html#how-to-check-for-errata)

If you determine that your application needs to check for specific errata, use this pattern:

```
int errata = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_ERRATA);
if (errata >= 0) {
    /* Check for specific erratum (1-indexed) */
    if (errata & (1 << (erratum_number - 1))) {
        /* Erratum N is fixed in this kernel */
    } else {
        /* Erratum N is NOT fixed - consider implications for your use case */
    }
}

```

Important: Only check errata if your application specifically relies on behavior that changed due to the fix. The fixes generally make Landlock less restrictive or more correct, not more restrictive.

## Kernel interface[¶](https://docs.kernel.org/userspace-api/landlock.html#kernel-interface)

### Access rights[¶](https://docs.kernel.org/userspace-api/landlock.html#access-rights)

A set of actions on kernel objects may be defined by an attribute (e.g. [struct landlock_path_beneath_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_path_beneath_attr)) including a bitmask of access.

#### Filesystem flags[¶](https://docs.kernel.org/userspace-api/landlock.html#filesystem-flags)

These flags enable to restrict a sandboxed process to a set of actions on files and directories. Files or directories opened before the sandboxing are not subject to these restrictions.

The following access rights apply only to files:

-

LANDLOCK_ACCESS_FS_EXECUTE: Execute a file.

-

LANDLOCK_ACCESS_FS_WRITE_FILE: Open a file with write access. When opening files for writing, you will often additionally need the LANDLOCK_ACCESS_FS_TRUNCATE right. In many cases, these system calls truncate existing files when overwriting them (e.g., [creat(2)](https://man7.org/linux/man-pages/man2/creat.2.html)).

-

LANDLOCK_ACCESS_FS_READ_FILE: Open a file with read access.

-

LANDLOCK_ACCESS_FS_TRUNCATE: Truncate a file with [truncate(2)](https://man7.org/linux/man-pages/man2/truncate.2.html), [ftruncate(2)](https://man7.org/linux/man-pages/man2/ftruncate.2.html), [creat(2)](https://man7.org/linux/man-pages/man2/creat.2.html), or [open(2)](https://man7.org/linux/man-pages/man2/open.2.html) with O_TRUNC. This access right is available since the third version of the Landlock ABI.

-

LANDLOCK_ACCESS_FS_IOCTL_DEV: Invoke [ioctl(2)](https://man7.org/linux/man-pages/man2/ioctl.2.html) commands on an opened character or block device.

This access right applies to all ioctl(2) commands implemented by device drivers. However, the following common IOCTL commands continue to be invokable independent of the LANDLOCK_ACCESS_FS_IOCTL_DEV right:

-

IOCTL commands targeting file descriptors (FIOCLEX, FIONCLEX),

-

IOCTL commands targeting file descriptions (FIONBIO, FIOASYNC),

-

IOCTL commands targeting file systems (FIFREEZE, FITHAW, FIGETBSZ, FS_IOC_GETFSUUID, FS_IOC_GETFSSYSFSPATH)

-

Some IOCTL commands which do not make sense when used with devices, but whose implementations are safe and return the right error codes (FS_IOC_FIEMAP, FICLONE, FICLONERANGE, FIDEDUPERANGE)

This access right is available since the fifth version of the Landlock ABI.

-

LANDLOCK_ACCESS_FS_RESOLVE_UNIX: Look up pathname UNIX domain sockets ([unix(7)](https://man7.org/linux/man-pages/man7/unix.7.html)). On UNIX domain sockets, this restricts both calls to [connect(2)](https://man7.org/linux/man-pages/man2/connect.2.html) as well as calls to [sendmsg(2)](https://man7.org/linux/man-pages/man2/sendmsg.2.html) with an explicit recipient address.

This access right applies only to connections to UNIX server sockets which were created outside of the newly created Landlock domain (e.g. from within a parent domain or from an unrestricted process). Newly created UNIX servers within the same Landlock domain continue to be accessible. In this regard, LANDLOCK_ACCESS_FS_RESOLVE_UNIX has the same semantics as the LANDLOCK_SCOPE_* flags.

If a resolution attempt is denied, the operation returns an EACCES error, in line with other filesystem access rights (but different to denials for abstract UNIX domain sockets).

This access right is available since the ninth version of the Landlock ABI.

The rationale for this design is described in [Documentation/security/landlock.rst](https://docs.kernel.org/security/landlock.html#scoped-flags-interaction).

Whether an opened file can be truncated with [ftruncate(2)](https://man7.org/linux/man-pages/man2/ftruncate.2.html) or used with ioctl(2) is determined during [open(2)](https://man7.org/linux/man-pages/man2/open.2.html), in the same way as read and write permissions are checked during [open(2)](https://man7.org/linux/man-pages/man2/open.2.html) using LANDLOCK_ACCESS_FS_READ_FILE and LANDLOCK_ACCESS_FS_WRITE_FILE.

A directory can receive access rights related to files or directories. The following access right is applied to the directory itself, and the directories beneath it:

-

LANDLOCK_ACCESS_FS_READ_DIR: Open a directory or list its content.

However, the following access rights only apply to the content of a directory, not the directory itself:

-

LANDLOCK_ACCESS_FS_REMOVE_DIR: Remove an empty directory or rename one.

-

LANDLOCK_ACCESS_FS_REMOVE_FILE: Unlink (or rename) a file.

-

LANDLOCK_ACCESS_FS_MAKE_CHAR: Create (or rename or link) a character device.

-

LANDLOCK_ACCESS_FS_MAKE_DIR: Create (or rename) a directory.

-

LANDLOCK_ACCESS_FS_MAKE_REG: Create (or rename or link) a regular file. This also guards the creation of whiteout objects as used in OverlayFS.

-

LANDLOCK_ACCESS_FS_MAKE_SOCK: Create (or rename or link) a UNIX domain socket.

-

LANDLOCK_ACCESS_FS_MAKE_FIFO: Create (or rename or link) a named pipe.

-

LANDLOCK_ACCESS_FS_MAKE_BLOCK: Create (or rename or link) a block device.

-

LANDLOCK_ACCESS_FS_MAKE_SYM: Create (or rename or link) a symbolic link.

-

LANDLOCK_ACCESS_FS_REFER: Link or rename a file from or to a different directory (i.e. reparent a file hierarchy).

This access right is available since the second version of the Landlock ABI.

This is the only access right which is denied by default by any ruleset, even if the right is not specified as handled at ruleset creation time. The only way to make a ruleset grant this right is to explicitly allow it for a specific directory by adding a matching rule to the ruleset.

In particular, when using the first Landlock ABI version, Landlock will always deny attempts to reparent files between different directories.

In addition to the source and destination directories having the LANDLOCK_ACCESS_FS_REFER access right, the attempted link or rename operation must meet the following constraints:

-

The reparented file may not gain more access rights in the destination directory than it previously had in the source directory. If this is attempted, the operation results in an EXDEV error.

-

When linking or renaming, the LANDLOCK_ACCESS_FS_MAKE_* right for the respective file type must be granted for the destination directory. Otherwise, the operation results in an EACCES error.

-

When renaming, the LANDLOCK_ACCESS_FS_REMOVE_* right for the respective file type must be granted for the source directory. Otherwise, the operation results in an EACCES error.

If multiple requirements are not met, the EACCES error code takes precedence over EXDEV.

Warning

It is currently not possible to restrict some file-related actions accessible through these syscall families: [chdir(2)](https://man7.org/linux/man-pages/man2/chdir.2.html), [stat(2)](https://man7.org/linux/man-pages/man2/stat.2.html), [flock(2)](https://man7.org/linux/man-pages/man2/flock.2.html), [chmod(2)](https://man7.org/linux/man-pages/man2/chmod.2.html), [chown(2)](https://man7.org/linux/man-pages/man2/chown.2.html), [setxattr(2)](https://man7.org/linux/man-pages/man2/setxattr.2.html), [utime(2)](https://man7.org/linux/man-pages/man2/utime.2.html), [fcntl(2)](https://man7.org/linux/man-pages/man2/fcntl.2.html), [access(2)](https://man7.org/linux/man-pages/man2/access.2.html). Future Landlock evolutions will enable to restrict them.

#### Network flags[¶](https://docs.kernel.org/userspace-api/landlock.html#network-flags)

These flags enable to restrict a sandboxed process to a set of network actions.

The following access rights apply to TCP port numbers:

-

LANDLOCK_ACCESS_NET_BIND_TCP: Bind TCP sockets to the given local port. Support added in Landlock ABI version 4.

-

LANDLOCK_ACCESS_NET_CONNECT_TCP: Connect TCP sockets to the given remote port. Support added in Landlock ABI version 4.

And similarly for UDP port numbers:

-

LANDLOCK_ACCESS_NET_BIND_UDP: Bind UDP sockets to the given local port. Support added in Landlock ABI version 10.

-

LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP: Set the remote port of UDP sockets to the given port, or send datagrams to the given remote port ignoring any destination pre-set on a socket. Support added in Landlock ABI version 10.

Note

Setting a remote address or sending a first datagram auto-binds UDP sockets to an ephemeral local source port if not already bound. To allow this if both LANDLOCK_ACCESS_NET_BIND_UDP and LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP are handled, you need to either:

-

use a socket already bound to a port before the ruleset started being enforced;

-

or grant LANDLOCK_ACCESS_NET_BIND_UDP on port 0, meaning “any port in the ephemeral port range”;

-

or grant LANDLOCK_ACCESS_NET_BIND_UDP on a specific port, and call [bind(2)](https://man7.org/linux/man-pages/man2/bind.2.html) on that port before trying to [connect(2)](https://man7.org/linux/man-pages/man2/connect.2.html) or send datagrams.

Note

Sending datagrams to an AF_UNSPEC destination address family is not supported for IPv6 UDP sockets: you will need to use a NULL address instead.

#### Scope flags[¶](https://docs.kernel.org/userspace-api/landlock.html#scope-flags)

These flags enable to isolate a sandboxed process from a set of IPC actions. Setting a flag for a ruleset will isolate the Landlock domain to forbid connections to resources outside the domain.

This is supported since Landlock ABI version 6.

Scopes:

-

LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET: Restrict a sandboxed process from connecting to an abstract UNIX socket created by a process outside the related Landlock domain (e.g., a parent domain or a non-sandboxed process).

-

LANDLOCK_SCOPE_SIGNAL: Restrict a sandboxed process from sending a signal to another process outside the domain.

#### Permission flags[¶](https://docs.kernel.org/userspace-api/landlock.html#permission-flags)

These flags restrict the use of members of a category, each member being identified by a constant from another kernel subsystem (e.g. CLONE_NEW* namespace types, CAP_* capabilities). A flag covers every kernel path that uses a member of its category, and members that no rule explicitly allows are denied. Values unknown to the running kernel are silently accepted for forward compatibility and stay denied by default. See [Landlock LSM: kernel documentation](https://docs.kernel.org/security/landlock.html) for design details.

When a ruleset handles multiple permissions whose operations overlap (e.g. a non-user namespace needs both its namespace type and CAP_SYS_ADMIN), the operation is allowed only if each handled permission independently allows it. See [Landlock: unprivileged access control](https://docs.kernel.org/userspace-api/landlock.html#).

-

LANDLOCK_PERMISSION_NAMESPACE_USE: Restrict the use of specific namespace types: creation ([unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html), [clone(2)](https://man7.org/linux/man-pages/man2/clone.2.html), [clone3(2)](https://man7.org/linux/man-pages/man2/clone3.2.html)), joining ([setns(2)](https://man7.org/linux/man-pages/man2/setns.2.html)), and acquiring an fd reference ([open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html), [fsmount(2)](https://man7.org/linux/man-pages/man2/fsmount.2.html)). A process in a Landlock domain that handles this permission is denied from using namespace types that are not explicitly allowed by a LANDLOCK_RULE_NAMESPACE rule. Support added in Landlock ABI version 12.

-

LANDLOCK_PERMISSION_CAPABILITY_USE: Restrict the use of specific Linux capabilities. A process in a Landlock domain that handles this permission is denied from exercising capabilities that are not explicitly allowed by a LANDLOCK_RULE_CAPABILITY rule. This hook is purely restrictive: it can deny capabilities that the kernel would otherwise grant, but it can never grant capabilities that the kernel already denied. Support added in Landlock ABI version 12.

### Creating a new ruleset[¶](https://docs.kernel.org/userspace-api/landlock.html#creating-a-new-ruleset)

long sys_landlock_create_ruleset(const struct [landlock_ruleset_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_ruleset_attr) __user *const attr, const size_t size, const __u32 flags)[¶](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_create_ruleset)

Create a new ruleset

Parameters

const struct landlock_ruleset_attr __user *const attr

Pointer to a [struct landlock_ruleset_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_ruleset_attr) identifying the scope of the new ruleset.

const size_t size

Size of the pointed [struct landlock_ruleset_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_ruleset_attr) (needed for backward and forward compatibility).

const __u32 flags

Supported values:

Description

-

LANDLOCK_CREATE_RULESET_VERSION

-

LANDLOCK_CREATE_RULESET_ERRATA

This system call enables to create a new Landlock ruleset.

If LANDLOCK_CREATE_RULESET_VERSION or LANDLOCK_CREATE_RULESET_ERRATA is set, then attr must be NULL and size must be 0.

-

EOPNOTSUPP: Landlock is supported by the kernel but disabled at boot time;

-

EINVAL: unknown flags, access, scope, or permission, or too small size;

-

EINVAL: quiet_access_fs, quiet_access_net, or quiet_scoped is not a subset of the corresponding handled_access_fs, handled_access_net, or scoped;

-

E2BIG: attr or size inconsistencies;

-

EFAULT: attr or size inconsistencies;

-

ENOMSG: all handled access, scope, and permission fields are empty.

Flags

LANDLOCK_CREATE_RULESET_VERSION

Get the highest supported Landlock ABI version (starting at 1).

LANDLOCK_CREATE_RULESET_ERRATA

Get a bitmask of fixed issues for the current Landlock ABI version.

Return

The ruleset file descriptor on success, the Landlock ABI version if LANDLOCK_CREATE_RULESET_VERSION is set, the errata value if LANDLOCK_CREATE_RULESET_ERRATA is set, or -errno on failure. Possible returned errors are:

struct landlock_ruleset_attr[¶](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_ruleset_attr)

Ruleset definition.

Definition:

```
struct landlock_ruleset_attr {
    __u64 handled_access_fs;
    __u64 handled_access_net;
    __u64 scoped;
    __u64 quiet_access_fs;
    __u64 quiet_access_net;
    __u64 quiet_scoped;
    __u64 handled_permissions;
};

```

Members

handled_access_fs

Bitmask of handled filesystem actions (cf. [Filesystem flags](https://docs.kernel.org/userspace-api/landlock.html#filesystem-flags)).

handled_access_net

Bitmask of handled network actions (cf. [Network flags](https://docs.kernel.org/userspace-api/landlock.html#network-flags)).

scoped

Bitmask of scopes (cf. [Scope flags](https://docs.kernel.org/userspace-api/landlock.html#scope-flags)) restricting a Landlock domain from accessing outside resources (e.g. IPCs).

quiet_access_fs

Bitmask of filesystem actions which should not be submitted to audit if the per-object quiet flag is set.

quiet_access_net

Bitmask of network actions which should not be submitted to audit if the per-object quiet flag is set.

quiet_scoped

Bitmask of scoped actions which should not be submitted to audit.

handled_permissions

Bitmask of handled permissions (cf. [Permission flags](https://docs.kernel.org/userspace-api/landlock.html#permission-flags)).

Description

Argument of [sys_landlock_create_ruleset()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_create_ruleset).

This structure defines a set of handled access rights, a set of actions on different object types, which should be denied by default when the ruleset is enacted. Vice versa, access rights that are not specifically listed here are not going to be denied by this ruleset when it is enacted.

For historical reasons, the LANDLOCK_ACCESS_FS_REFER right is always denied by default, even when its bit is not set in handled_access_fs. In order to add new rules with this access right, the bit must still be set explicitly (cf. [Filesystem flags](https://docs.kernel.org/userspace-api/landlock.html#filesystem-flags)).

The explicit listing of handled access rights is required for backwards compatibility reasons. In most use cases, processes that use Landlock will handle a wide range or all access rights that they know about at build time (and that they have tested with a kernel that supported them all).

quiet_access_fs and quiet_access_net are bitmasks of actions for which a denial by this layer will not trigger an audit record if the corresponding object (or its children, for filesystem rules) is marked with the “quiet” bit via LANDLOCK_ADD_RULE_QUIET, even if logging would normally take place per landlock_restrict_self() flags. quiet_scoped is similar, except that it does not require marking any objects as quiet - if the ruleset is created with any bits set in quiet_scoped, then denial of such scoped resources will not trigger an audit record. These three fields are available since Landlock ABI version 10.

quiet_access_fs, quiet_access_net and quiet_scoped must be a subset of handled_access_fs, handled_access_net and scoped respectively.

This structure can grow in future Landlock versions.

### Extending a ruleset[¶](https://docs.kernel.org/userspace-api/landlock.html#extending-a-ruleset)

long sys_landlock_add_rule(const int ruleset_fd, const enum [landlock_rule_type](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_rule_type) rule_type, const void __user *const rule_attr, const __u32 flags)[¶](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_add_rule)

Add a new rule to a ruleset

Parameters

const int ruleset_fd

File descriptor tied to the ruleset that should be extended with the new rule.

const enum landlock_rule_type rule_type

Identify the structure type pointed to by rule_attr: LANDLOCK_RULE_PATH_BENEATH, LANDLOCK_RULE_NET_PORT, LANDLOCK_RULE_NAMESPACE, or LANDLOCK_RULE_CAPABILITY.

const void __user *const rule_attr

Pointer to a rule (matching the rule_type).

const __u32 flags

Must be 0 or LANDLOCK_ADD_RULE_QUIET.

Description

This system call enables to define a new rule and add it to an existing ruleset.

-

EOPNOTSUPP: Landlock is supported by the kernel but disabled at boot time;

-

EAFNOSUPPORT: rule_type is LANDLOCK_RULE_NET_PORT but TCP/IP is not supported by the running kernel;

-

EINVAL: flags is not valid;

-

EINVAL: The rule accesses are inconsistent (i.e. [landlock_path_beneath_attr.allowed_access](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_path_beneath_attr) or [landlock_net_port_attr.allowed_access](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_net_port_attr) is not a subset of the ruleset handled accesses)

-

EINVAL: A nonzero [landlock_namespace_attr.permissions](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_namespace_attr) is not LANDLOCK_PERMISSION_NAMESPACE_USE or is not handled by the ruleset;

-

EINVAL: A nonzero [landlock_capability_attr.permissions](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_capability_attr) is not LANDLOCK_PERMISSION_CAPABILITY_USE or is not handled by the ruleset;

-

EINVAL: [landlock_net_port_attr.port](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_net_port_attr) is greater than 65535;

-

EINVAL: LANDLOCK_ADD_RULE_QUIET is passed but the ruleset has no quiet access bits set for the corresponding rule type.

-

ENOMSG: Empty accesses (e.g. [landlock_path_beneath_attr.allowed_access](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_path_beneath_attr) is 0) and no flags;

-

ENOMSG: [landlock_namespace_attr.permissions](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_namespace_attr) is 0, or both [landlock_namespace_attr.allowed_namespace_types](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_namespace_attr) and [landlock_namespace_attr.quiet_namespace_types](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_namespace_attr) are 0;

-

ENOMSG: [landlock_capability_attr.permissions](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_capability_attr) is 0, or both [landlock_capability_attr.allowed_capabilities](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_capability_attr) and [landlock_capability_attr.quiet_capabilities](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_capability_attr) are 0;

-

EBADF: ruleset_fd is not a file descriptor for the current thread, or a member of rule_attr is not a file descriptor as expected;

-

EBADFD: ruleset_fd is not a ruleset file descriptor, or a member of rule_attr is not the expected file descriptor type;

-

EPERM: ruleset_fd has no write access to the underlying ruleset;

-

EFAULT: rule_attr was not a valid address.

Flags

LANDLOCK_ADD_RULE_QUIET

Together with the quiet_* fields in [struct landlock_ruleset_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_ruleset_attr), this flag controls whether Landlock will log audit messages when access to the objects covered by this rule is denied by this layer.

If logging is enabled, when Landlock denies an access, it will suppress the log if all of the following are true:

-

this layer is the innermost layer that denied the access;

-

all accesses denied by this layer are part of the quiet_* fields in the related [struct landlock_ruleset_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_ruleset_attr);

-

the object (or one of its parents, for filesystem rules) is marked as “quiet” via LANDLOCK_ADD_RULE_QUIET.

Because logging is only suppressed by a layer if the layer denies access, a sandboxed program cannot use this flag to “hide” access denials, without denying itself the access in the first place.

The effect of this flag does not depend on the value of allowed_access in the passed in rule_attr. When this flag is present, the caller is also allowed to pass in an empty allowed_access.

Return

0 on success, or -errno on failure. Possible returned errors are:

enum landlock_rule_type[¶](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_rule_type)

Landlock rule type

Constants

LANDLOCK_RULE_PATH_BENEATH

Type of a [struct landlock_path_beneath_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_path_beneath_attr) .

LANDLOCK_RULE_NET_PORT

Type of a [struct landlock_net_port_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_net_port_attr) .

LANDLOCK_RULE_NAMESPACE

Type of a [struct landlock_namespace_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_namespace_attr) .

LANDLOCK_RULE_CAPABILITY

Type of a [struct landlock_capability_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_capability_attr) .

Description

Argument of [sys_landlock_add_rule()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_add_rule).

struct landlock_path_beneath_attr[¶](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_path_beneath_attr)

Path hierarchy definition

Definition:

```
struct landlock_path_beneath_attr {
    __u64 allowed_access;
    __s32 parent_fd;
};

```

Members

allowed_access

Bitmask of allowed actions for this file hierarchy (cf. [Filesystem flags](https://docs.kernel.org/userspace-api/landlock.html#filesystem-flags)).

parent_fd

File descriptor, preferably opened with O_PATH, which identifies the parent directory of a file hierarchy, or just a file.

Description

Argument of [sys_landlock_add_rule()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_add_rule).

struct landlock_net_port_attr[¶](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_net_port_attr)

Network port definition

Definition:

```
struct landlock_net_port_attr {
    __u64 allowed_access;
    __u64 port;
};

```

Members

allowed_access

Bitmask of allowed network actions for a port (cf. [Network flags](https://docs.kernel.org/userspace-api/landlock.html#network-flags)).

port

Network port in host endianness.

It should be noted that port 0 passed to [bind(2)](https://man7.org/linux/man-pages/man2/bind.2.html) will bind to an available port from the ephemeral port range. This can be configured with the /proc/sys/net/ipv4/ip_local_port_range sysctl (also used for IPv6), and within that range, on a per-socket basis with setsockopt(IP_LOCAL_PORT_RANGE).

A Landlock rule with port 0 and the LANDLOCK_ACCESS_NET_BIND_TCP or LANDLOCK_ACCESS_NET_BIND_UDP right means that requesting to bind on port 0 is allowed and it will automatically translate to binding on a kernel-assigned ephemeral port.

Description

Argument of [sys_landlock_add_rule()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_add_rule).

struct landlock_namespace_attr[¶](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_namespace_attr)

Namespace type definition

Definition:

```
struct landlock_namespace_attr {
    __u64 permissions;
    __u64 allowed_namespace_types;
    __u64 quiet_namespace_types;
};

```

Members

permissions

Must be set to LANDLOCK_PERMISSION_NAMESPACE_USE.

allowed_namespace_types

Bitmask of namespace types (CLONE_NEW* flags) to allow under this rule. Unknown bits are silently ignored for forward compatibility.

quiet_namespace_types

Bitmask of namespace types (CLONE_NEW* flags) whose denial by this layer should not be submitted to audit, even when landlock_restrict_self() enables audit logging. Only audit records attributed to this layer are suppressed; denial tracepoints still fire (see [Permission flags](https://docs.kernel.org/userspace-api/landlock.html#permission-flags)). Bits also set in allowed_namespace_types have no effect, since an allowed type is never denied. Unknown bits are silently ignored.

At least one of allowed_namespace_types or quiet_namespace_types must be non-zero, otherwise the call returns -ENOMSG. The non-zero check runs on the raw input before unknown-bit masking, so a rule that sets only bits unknown to the running kernel succeeds but has no runtime effect. Programs should omit this rule when they neither allow nor quiet a namespace type.

Description

Argument of [sys_landlock_add_rule()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_add_rule) with LANDLOCK_RULE_NAMESPACE.

struct landlock_capability_attr[¶](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_capability_attr)

Capability definition

Definition:

```
struct landlock_capability_attr {
    __u64 permissions;
    __u64 allowed_capabilities;
    __u64 quiet_capabilities;
};

```

Members

permissions

Must be set to LANDLOCK_PERMISSION_CAPABILITY_USE.

allowed_capabilities

Bitmask of capabilities (1ULL << CAP_*) to allow under this rule. Bits above CAP_LAST_CAP are silently ignored for forward compatibility.

quiet_capabilities

Bitmask of capabilities (1ULL << CAP_*) whose denial by this layer should not be submitted to audit, even if audit logging would normally take place per landlock_restrict_self() flags. Only audit records attributed to this layer are suppressed; denial tracepoints still fire (see [Permission flags](https://docs.kernel.org/userspace-api/landlock.html#permission-flags)). Bits also set in allowed_capabilities have no effect, since an allowed capability is never denied. Bits above CAP_LAST_CAP are silently ignored.

At least one of allowed_capabilities or quiet_capabilities must be non-zero, otherwise the call returns -ENOMSG. The non-zero check runs on the raw input before unknown-bit masking, so a rule that sets only bits unknown to the running kernel (above CAP_LAST_CAP) succeeds but has no runtime effect. Programs should omit this rule when they neither allow nor quiet a capability.

Description

Argument of [sys_landlock_add_rule()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_add_rule) with LANDLOCK_RULE_CAPABILITY.

### Enforcing a ruleset[¶](https://docs.kernel.org/userspace-api/landlock.html#enforcing-a-ruleset)

long sys_landlock_restrict_self(const int ruleset_fd, const __u32 flags)[¶](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_restrict_self)

Enforce a ruleset on the calling thread

Parameters

const int ruleset_fd

File descriptor tied to the ruleset to merge with the target.

const __u32 flags

Supported values:

Description

-

LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF

-

LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON

-

LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF

-

LANDLOCK_RESTRICT_SELF_TSYNC

-

LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS

This system call enforces a Landlock ruleset on the current thread. Enforcing a ruleset requires that the task has CAP_SYS_ADMIN in its namespace or is running with no_new_privs. This avoids scenarios where unprivileged tasks can affect the behavior of privileged children.

With LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS, the no_new_privs attribute of the calling thread is set only once the enforcement of the ruleset succeeded, which fulfills the above requirement: no_new_privs is set if and only if the call succeeds.

-

EOPNOTSUPP: Landlock is supported by the kernel but disabled at boot time;

-

EINVAL: flags contains an unknown bit.

-

EBADF: ruleset_fd is not a file descriptor for the current thread;

-

EBADFD: ruleset_fd is not a ruleset file descriptor;

-

EPERM: ruleset_fd has no read access to the underlying ruleset, or LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS is not set while the current thread is not running with no_new_privs and doesn’t have CAP_SYS_ADMIN in its namespace.

-

E2BIG: The maximum number of stacked rulesets is reached for the current thread.

Flags

By default, denied accesses originating from programs that sandbox themselves are logged via the audit subsystem. Such events typically indicate unexpected behavior, such as bugs or exploitation attempts. However, to avoid excessive logging, access requests denied by a domain not created by the originating program are not logged by default. The rationale is that programs should know their own behavior, but not necessarily the behavior of other programs. This default configuration is suitable for most programs that sandbox themselves. For specific use cases, the following flags allow programs to modify this default logging behavior.

The LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF and LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON flags apply to the newly created Landlock domain.

LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF

Disables logging of denied accesses originating from the thread creating the Landlock domain, as well as its children, as long as they continue running the same executable code (i.e., without an intervening [execve(2)](https://man7.org/linux/man-pages/man2/execve.2.html) call). This is intended for programs that execute unknown code without invoking [execve(2)](https://man7.org/linux/man-pages/man2/execve.2.html), such as script interpreters. Programs that only sandbox themselves should not set this flag, so users can be notified of unauthorized access attempts via system logs.

LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON

Enables logging of denied accesses after an [execve(2)](https://man7.org/linux/man-pages/man2/execve.2.html) call, providing visibility into unauthorized access attempts by newly executed programs within the created Landlock domain. This flag is recommended only when all potential executables in the domain are expected to comply with the access restrictions, as excessive audit log entries could make it more difficult to identify critical events.

LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF

Disables logging of denied accesses originating from nested Landlock domains created by the caller or its descendants. This flag should be set according to runtime configuration, not hardcoded, to avoid suppressing important security events. It is useful for container runtimes or sandboxing tools that may launch programs which themselves create Landlock domains and could otherwise generate excessive logs. Unlike LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF, this flag only affects future nested domains, not the one being created. It can also be used with a ruleset_fd value of -1 to mute subdomain logs without creating a domain. When combined with LANDLOCK_RESTRICT_SELF_TSYNC and a ruleset_fd value of -1, this configuration is propagated to all threads of the current process.

The following flag supports policy enforcement in multithreaded processes:

LANDLOCK_RESTRICT_SELF_TSYNC

Applies the new Landlock configuration atomically to all threads of the current process, including the Landlock domain and logging configuration. This overrides the Landlock configuration of sibling threads, irrespective of previously established Landlock domains and logging configurations on these threads.

If the calling thread is running with no_new_privs, this operation enables no_new_privs on the sibling threads as well.

The following flag ties the no_new_privs attribute to the ruleset enforcement:

LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS

Sets the no_new_privs attribute of the calling thread only once the enforcement of the ruleset succeeded: no_new_privs is set if and only if [sys_landlock_restrict_self()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_restrict_self) succeeds. This removes the need for a prior [prctl(2)](https://man7.org/linux/man-pages/man2/prctl.2.html) PR_SET_NO_NEW_PRIVS call (or CAP_SYS_ADMIN use). This flag requires a ruleset. When combined with LANDLOCK_RESTRICT_SELF_TSYNC, no_new_privs is set on the sibling threads as well.

Return

0 on success, or -errno on failure. Possible returned errors are:

## Current limitations[¶](https://docs.kernel.org/userspace-api/landlock.html#current-limitations)

### Filesystem topology modification[¶](https://docs.kernel.org/userspace-api/landlock.html#filesystem-topology-modification)

Threads sandboxed with filesystem restrictions cannot modify filesystem topology, whether via [mount(2)](https://man7.org/linux/man-pages/man2/mount.2.html) or [pivot_root(2)](https://man7.org/linux/man-pages/man2/pivot_root.2.html). However, [chroot(2)](https://man7.org/linux/man-pages/man2/chroot.2.html) calls are not denied.

### Special filesystems[¶](https://docs.kernel.org/userspace-api/landlock.html#special-filesystems)

Access to regular files and directories can be restricted by Landlock, according to the handled accesses of a ruleset. However, files that do not come from a user-visible filesystem (e.g. pipe, socket), but can still be accessed through /proc/<pid>/fd/*, cannot currently be explicitly restricted. Likewise, some special kernel filesystems such as nsfs, which can be accessed through /proc/<pid>/ns/*, cannot currently be explicitly restricted. However, thanks to the [ptrace restrictions](https://docs.kernel.org/userspace-api/landlock.html#ptrace-restrictions), access to such sensitive /proc files are automatically restricted according to domain hierarchies. Future Landlock evolutions could still enable to explicitly restrict such paths with dedicated ruleset flags.

### Ruleset layers[¶](https://docs.kernel.org/userspace-api/landlock.html#ruleset-layers)

There is a limit of 16 layers of stacked rulesets. This can be an issue for a task willing to enforce a new ruleset in complement to its 16 inherited rulesets. Once this limit is reached, [sys_landlock_restrict_self()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_restrict_self) returns E2BIG. It is then strongly suggested to carefully build rulesets once in the life of a thread, especially for applications able to launch other applications that may also want to sandbox themselves (e.g. shells, container managers, etc.).

### Memory usage[¶](https://docs.kernel.org/userspace-api/landlock.html#memory-usage)

Kernel memory allocated to create rulesets is accounted and can be restricted by the [Memory Resource Controller](https://docs.kernel.org/admin-guide/cgroup-v1/memory.html).

### IOCTL support[¶](https://docs.kernel.org/userspace-api/landlock.html#ioctl-support)

The LANDLOCK_ACCESS_FS_IOCTL_DEV right restricts the use of [ioctl(2)](https://man7.org/linux/man-pages/man2/ioctl.2.html), but it only applies to newly opened device files. This means specifically that pre-existing file descriptors like stdin, stdout and stderr are unaffected.

Users should be aware that TTY devices have traditionally permitted to control other processes on the same TTY through the TIOCSTI and TIOCLINUX IOCTL commands. Both of these require CAP_SYS_ADMIN on modern Linux systems, but the behavior is configurable for TIOCSTI.

On older systems, it is therefore recommended to close inherited TTY file descriptors, or to reopen them from /proc/self/fd/* without the LANDLOCK_ACCESS_FS_IOCTL_DEV right, if possible.

Landlock’s IOCTL support is coarse-grained at the moment, but may become more fine-grained in the future. Until then, users are advised to establish the guarantees that they need through the file hierarchy, by only allowing the LANDLOCK_ACCESS_FS_IOCTL_DEV right on files where it is really required.

## Previous limitations[¶](https://docs.kernel.org/userspace-api/landlock.html#previous-limitations)

### File renaming and linking (ABI < 2)[¶](https://docs.kernel.org/userspace-api/landlock.html#file-renaming-and-linking-abi-2)

Because Landlock targets unprivileged access controls, it needs to properly handle composition of rules. Such property also implies rules nesting. Properly handling multiple layers of rulesets, each one of them able to restrict access to files, also implies inheritance of the ruleset restrictions from a parent to its hierarchy. Because files are identified and restricted by their hierarchy, moving or linking a file from one directory to another implies propagation of the hierarchy constraints, or restriction of these actions according to the potentially lost constraints. To protect against privilege escalations through renaming or linking, and for the sake of simplicity, Landlock previously limited linking and renaming to the same directory. Starting with the Landlock ABI version 2, it is now possible to securely control renaming and linking thanks to the new LANDLOCK_ACCESS_FS_REFER access right.

### File truncation (ABI < 3)[¶](https://docs.kernel.org/userspace-api/landlock.html#file-truncation-abi-3)

File truncation could not be denied before the third Landlock ABI, so it is always allowed when using a kernel that only supports the first or second ABI.

Starting with the Landlock ABI version 3, it is now possible to securely control truncation thanks to the new LANDLOCK_ACCESS_FS_TRUNCATE access right.

### TCP bind and connect (ABI < 4)[¶](https://docs.kernel.org/userspace-api/landlock.html#tcp-bind-and-connect-abi-4)

Starting with the Landlock ABI version 4, it is now possible to restrict TCP bind and connect actions to only a set of allowed ports thanks to the new LANDLOCK_ACCESS_NET_BIND_TCP and LANDLOCK_ACCESS_NET_CONNECT_TCP access rights.

### Device IOCTL (ABI < 5)[¶](https://docs.kernel.org/userspace-api/landlock.html#device-ioctl-abi-5)

IOCTL operations could not be denied before the fifth Landlock ABI, so [ioctl(2)](https://man7.org/linux/man-pages/man2/ioctl.2.html) is always allowed when using a kernel that only supports an earlier ABI.

Starting with the Landlock ABI version 5, it is possible to restrict the use of [ioctl(2)](https://man7.org/linux/man-pages/man2/ioctl.2.html) on character and block devices using the new LANDLOCK_ACCESS_FS_IOCTL_DEV right.

### Abstract UNIX socket (ABI < 6)[¶](https://docs.kernel.org/userspace-api/landlock.html#abstract-unix-socket-abi-6)

Starting with the Landlock ABI version 6, it is possible to restrict connections to an abstract [unix(7)](https://man7.org/linux/man-pages/man7/unix.7.html) socket by setting LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET to the scoped ruleset attribute.

### Signal (ABI < 6)[¶](https://docs.kernel.org/userspace-api/landlock.html#signal-abi-6)

Starting with the Landlock ABI version 6, it is possible to restrict [signal(7)](https://man7.org/linux/man-pages/man7/signal.7.html) sending by setting LANDLOCK_SCOPE_SIGNAL to the scoped ruleset attribute.

### Logging (ABI < 7)[¶](https://docs.kernel.org/userspace-api/landlock.html#logging-abi-7)

Starting with the Landlock ABI version 7, it is possible to control logging of Landlock audit events with the LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF, LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON, and LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF flags passed to [sys_landlock_restrict_self()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_restrict_self). These flags control audit record generation. Landlock tracepoints are not affected by these flags and always fire when enabled, providing an alternative observability channel for debugging and monitoring. See [Landlock: system-wide management](https://docs.kernel.org/admin-guide/LSM/landlock.html) for more details on audit and tracepoints, and [Landlock Trace Events](https://docs.kernel.org/trace/events-landlock.html) for the complete trace event reference.

### Thread synchronization (ABI < 8)[¶](https://docs.kernel.org/userspace-api/landlock.html#thread-synchronization-abi-8)

Starting with the Landlock ABI version 8, it is now possible to enforce Landlock rulesets across all threads of the calling process using the LANDLOCK_RESTRICT_SELF_TSYNC flag passed to [sys_landlock_restrict_self()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_restrict_self).

### Pathname UNIX sockets (ABI < 9)[¶](https://docs.kernel.org/userspace-api/landlock.html#pathname-unix-sockets-abi-9)

Starting with the Landlock ABI version 9, it is possible to restrict connections to pathname UNIX domain sockets ([unix(7)](https://man7.org/linux/man-pages/man7/unix.7.html)) using the new LANDLOCK_ACCESS_FS_RESOLVE_UNIX right.

### UDP bind, connect and send* (ABI < 10)[¶](https://docs.kernel.org/userspace-api/landlock.html#udp-bind-connect-and-send-abi-10)

Starting with the Landlock ABI version 10, it is possible to restrict setting the local port of UDP sockets with the LANDLOCK_ACCESS_NET_BIND_UDP right. This includes restricting the ability to trigger autobind of an ephemeral port by the kernel by e.g. sending a first datagram or setting the remote peer of a socket. The LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP right controls setting the remote port of UDP sockets (via [connect(2)](https://man7.org/linux/man-pages/man2/connect.2.html)), and sending datagrams to an explicit remote port (ignoring any destination set on UDP sockets, via e.g. [sendto(2)](https://man7.org/linux/man-pages/man2/sendto.2.html)).

### Quiet rule flag (ABI < 10)[¶](https://docs.kernel.org/userspace-api/landlock.html#quiet-rule-flag-abi-10)

Starting with the Landlock ABI version 10, it is possible to selectively suppress logs for specific denied accesses on a per-object basis with the LANDLOCK_ADD_RULE_QUIET flag of [sys_landlock_add_rule()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_add_rule), in combination with the quiet_access_fs and quiet_access_net fields of [struct landlock_ruleset_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_ruleset_attr). It is also now possible to suppress logs for scope accesses via the quiet_scoped field of [struct landlock_ruleset_attr](https://docs.kernel.org/userspace-api/landlock.html#c.landlock_ruleset_attr). The object is marked as quiet within a ruleset when at least one [sys_landlock_add_rule()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_add_rule) call is made for it with the LANDLOCK_ADD_RULE_QUIET flag, additional add-rule calls for the same object without this flag do not clear it.

### no_new_privs flag (ABI < 11)[¶](https://docs.kernel.org/userspace-api/landlock.html#no-new-privs-flag-abi-11)

Starting with the Landlock ABI version 11, [sys_landlock_restrict_self()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_restrict_self) accepts the LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS flag, which sets the no_new_privs attribute of the calling thread only once the enforcement of the ruleset succeeded: no_new_privs is set if and only if the call succeeds. This removes the need for a prior [prctl(2)](https://man7.org/linux/man-pages/man2/prctl.2.html) PR_SET_NO_NEW_PRIVS call (or CAP_SYS_ADMIN use). When combined with LANDLOCK_RESTRICT_SELF_TSYNC, no_new_privs is set on all threads of the process. As explained in the tutorial above, leaving no_new_privs unset is risky even when Landlock does not require it.

### Capability (ABI < 12)[¶](https://docs.kernel.org/userspace-api/landlock.html#capability-abi-12)

Starting with the Landlock ABI version 12, it is possible to restrict [capabilities(7)](https://man7.org/linux/man-pages/man7/capabilities.7.html) with the new LANDLOCK_PERMISSION_CAPABILITY_USE permission flag and LANDLOCK_RULE_CAPABILITY rule type.

### Namespace (ABI < 12)[¶](https://docs.kernel.org/userspace-api/landlock.html#namespace-abi-12)

Starting with the Landlock ABI version 12, it is possible to restrict namespace use (see [namespaces(7)](https://man7.org/linux/man-pages/man7/namespaces.7.html)) across creation ([unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html), [clone(2)](https://man7.org/linux/man-pages/man2/clone.2.html), [clone3(2)](https://man7.org/linux/man-pages/man2/clone3.2.html)), entry ([setns(2)](https://man7.org/linux/man-pages/man2/setns.2.html)), and fd-reference acquisition ([open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html), [fsmount(2)](https://man7.org/linux/man-pages/man2/fsmount.2.html)) with the new LANDLOCK_PERMISSION_NAMESPACE_USE permission flag and LANDLOCK_RULE_NAMESPACE rule type.

## Kernel support[¶](https://docs.kernel.org/userspace-api/landlock.html#kernel-support)

### Build time configuration[¶](https://docs.kernel.org/userspace-api/landlock.html#build-time-configuration)

Landlock was first introduced in Linux 5.13 but it must be configured at build time with CONFIG_SECURITY_LANDLOCK=y. Landlock must also be enabled at boot time like other security modules. The list of security modules enabled by default is set with CONFIG_LSM. The kernel configuration should then contain CONFIG_LSM=landlock,[...] with [...] as the list of other potentially useful security modules for the running system (see the CONFIG_LSM help).

### Boot time configuration[¶](https://docs.kernel.org/userspace-api/landlock.html#boot-time-configuration)

If the running kernel does not have landlock in CONFIG_LSM, then we can enable Landlock by adding lsm=landlock,[...] to [The kernel’s command-line parameters](https://docs.kernel.org/admin-guide/kernel-parameters.html) in the boot loader configuration.

For example, if the current built-in configuration is:

```
$ zgrep -h "^CONFIG_LSM=" "/boot/config-$(uname -r)" /proc/config.gz 2>/dev/null
CONFIG_LSM="lockdown,yama,integrity,apparmor"

```

...and if the cmdline doesn’t contain landlock either:

```
$ sed -n 's/.*\(\<lsm=\S\+\).*/\1/p' /proc/cmdline
lsm=lockdown,yama,integrity,apparmor

```

...we should configure the boot loader to set a cmdline extending the lsm list with the landlock, prefix:

```
lsm=landlock,lockdown,yama,integrity,apparmor

```

After a reboot, we can check that Landlock is up and running by looking at kernel logs:

```
# dmesg | grep landlock || journalctl -kb -g landlock
[    0.000000] Command line: [...] lsm=landlock,lockdown,yama,integrity,apparmor
[    0.000000] Kernel command line: [...] lsm=landlock,lockdown,yama,integrity,apparmor
[    0.000000] LSM: initializing lsm=lockdown,capability,landlock,yama,integrity,apparmor
[    0.000000] landlock: Up and running.

```

The kernel may be configured at build time to always load the lockdown and capability LSMs. In that case, these LSMs will appear at the beginning of the LSM: initializing log line as well, even if they are not configured in the boot loader.

### Network support[¶](https://docs.kernel.org/userspace-api/landlock.html#network-support)

To be able to explicitly allow TCP or UDP operations (e.g., adding a network rule with LANDLOCK_ACCESS_NET_BIND_TCP), the kernel must support the TCP/IP protocol suite (CONFIG_INET=y). Otherwise, [sys_landlock_add_rule()](https://docs.kernel.org/userspace-api/landlock.html#c.sys_landlock_add_rule) returns an EAFNOSUPPORT error, which can safely be ignored because this kind of TCP or UDP operation is already not possible.

## Questions and answers[¶](https://docs.kernel.org/userspace-api/landlock.html#questions-and-answers)

### What about user space sandbox managers?[¶](https://docs.kernel.org/userspace-api/landlock.html#what-about-user-space-sandbox-managers)

Using user space processes to enforce restrictions on kernel resources can lead to race conditions or inconsistent evaluations (i.e. [Incorrect mirroring of the OS code and state](https://www.ndss-symposium.org/ndss2003/traps-and-pitfalls-practical-problems-system-call-interposition-based-security-tools/)).

### What about namespaces and containers?[¶](https://docs.kernel.org/userspace-api/landlock.html#what-about-namespaces-and-containers)

Namespaces can help create sandboxes but they are not designed for access-control and then miss useful features for such use case (e.g. no fine-grained restrictions). Moreover, their complexity can lead to security issues, especially when untrusted processes can manipulate them (cf. [Controlling access to user namespaces](https://lwn.net/Articles/673597/)).

### How to disable Landlock audit records?[¶](https://docs.kernel.org/userspace-api/landlock.html#how-to-disable-landlock-audit-records)

You might want to put in place filters as explained here: [Landlock: system-wide management](https://docs.kernel.org/admin-guide/LSM/landlock.html)

## Additional documentation[¶](https://docs.kernel.org/userspace-api/landlock.html#additional-documentation)

-

[Landlock: system-wide management](https://docs.kernel.org/admin-guide/LSM/landlock.html)

-

[Landlock Trace Events](https://docs.kernel.org/trace/events-landlock.html)

-

[Landlock LSM: kernel documentation](https://docs.kernel.org/security/landlock.html)

-

https://landlock.io

©The kernel development community. | Powered by [Sphinx 5.3.0](https://www.sphinx-doc.org/) & [Alabaster 0.7.16](https://alabaster.readthedocs.io) | [Page source](https://docs.kernel.org/_sources/userspace-api/landlock.rst.txt)
