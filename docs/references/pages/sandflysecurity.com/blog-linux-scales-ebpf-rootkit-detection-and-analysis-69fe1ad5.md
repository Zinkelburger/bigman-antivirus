---
url: "https://sandflysecurity.com/blog/linux-scales-ebpf-rootkit-detection-and-analysis"
title: "Linux Scales eBPF Rootkit Detection and Analysis"
captured: "2026-11-02T18:42:01Z"
via: "live"
sha256: "29ce45d5a92e7ad274c693aae5c1b467659df36aa76a8d9b0aa86335f1621657"
wayback: "https://web.archive.org/web/20260715064608/https://sandflysecurity.com/blog/linux-scales-ebpf-rootkit-detection-and-analysis"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Linux Scales eBPF Rootkit Detection and Analysis

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://sandflysecurity.com/blog/linux-scales-ebpf-rootkit-detection-and-analysis>
> Snapshot: <https://web.archive.org/web/20260715064608/https://sandflysecurity.com/blog/linux-scales-ebpf-rootkit-detection-and-analysis>

7 min read

# Linux Scales eBPF Rootkit Detection and Analysis

[Sandfly Security - Agentless Linux EDR and Incident Response](https://sandflysecurity.com/blog/author/sandfly-security-agentless-linux-edr-and-incident-response) :    Published June 23, 2026   Updated September 30, 2026

[Rootkits](https://sandflysecurity.com/blog/tag/rootkits) [Malware](https://sandflysecurity.com/blog/tag/malware) [Linux Forensics](https://sandflysecurity.com/blog/tag/linux-forensics)

Publication Date: 24 June 2026

The Scales Linux rootkit was recently discovered and used to target Arch User Repository (AUR) supply-chains (dubbed Atomic Arch) affecting more than 1,500 packages. This malware incorporated an integrated eBPF rootkit to hide, a built-in Tor client for command and control (C2) traffic, and had extensive features to steal credentials to enable further attacks.

This write-up will discuss technical details of the malware, how Sandfly detects it already, and command line forensics to verify if a system is affected.

### Malware Features

This malware is large at around 3 MB in size. The binary uses multiple methods to hide, communicate, and steal credentials:

- Deploys an eBPF rootkit to hide malicious process and network activity.
- Deploys a Tor client to enable anonymous C2 traffic to attackers.
- Deploys persistence mechanisms in systemd to restart the malware on system boot.
- Deploys multiple methods to find credentials, tokens, and other valuable access data.
- Has additional features such as malicious sudo replacement to steal local user passwords.

### Sandfly Detection

Because Sandfly looks for Linux attack tactics like process cloaking, it detects the Scales rootkit evasion out of the box without requiring custom rule updates or prior knowledge of the specific malware signatures. Customers do not need to upgrade as they are already covered.

Below is a complete list of alerts you may receive from a system infected with the Scales rootkit. This list includes standard enabled checks such as process de-cloaking, and also [drift detection alerts](https://sandflysecurity.com/platform/drift-detection) you may receive if you have enabled that feature on a host.

The main detections customers will see are the following:

process_running_hidden_stealth - Two processes running that are being actively cloaked by the rootkit.

dirs_hidden_bin - Directory containing trojaned sudo command to steal user credentials.

Other detections shown will vary depending if a customer is using drift detection features such as new processes being started, ebpf enabled processes, etc. The two alerts above though are extremely high confidence indicators regardless of drift detection or other detection modules being used.

### eBPF Rootkit Hiding and Detection

The rootkit has an embedded eBPF program that is loaded on startup to hide processes, process names, and inodes to conceal network activity.

The rootkit will start two processes. The main process is the rootkit binary and has a random name and copies itself under the /var/lib directory so it can persist between reboots.

The second process runs under a bogus dbus-daemon name and is the Tor client used to anonymize the C2 communications to the attacker.

In the image below we see the two processes de-cloaked by Sandfly and in this run it uses the name simece, but again this is random each time it runs so it can't be used for searching across systems.

The rootkit also renames the command line to various selections. In the example here it picked a name to mimic a kernel thread kworker/0:3. We can see the outcome of this name stomping in the process forensics as the command line is padded out. This is belt-and-suspenders hiding presumably in case the eBPF rootkit didn't load. In that case, the malware can still conceal itself under a bogus name.

The rootkit has hidden the above process with PID 764 and the Tor client will also be a PID near the main process. If you are using command line tools like ps, the process will not be visible.

ps -auxw

We can go further and look at the /proc directory to see if the PID is being listed there. With this eBPF rootkit active, the directory entry is actually hidden from this as well. The listing shows a gap where no PID 764 is seen where it should be.

ls -al /proc

### Manually Investigating Scales eBPF Hidden Processes

Since we have had Sandfly already de-cloak the hidden process we can isolate the system and then do some quick checks to confirm something is actually wrong for incident response escalation.

Using our PID recovered by Sandfly, the first thing we can do is a simple stat command in /proc to see if the directory really exists. This rootkit hides the directory listing from view with tools like ls, but the stat command is not affected:

stat /proc/PID

When the above command is run, it will come back with directory status information if it exists. This indicates that the process is there, but is simply being hidden by the rootkit.

Next we can go into the process directory and look around.

cd /proc/PID

A simple ls shows the exe path to the suspicious binary.

The link from the exe will point to /var/lib but the directory and binary name will be random when the malware first goes active. However, this directory is visible and easily accessed to obtain the binary from the full path.

### Grabbing A Process Binary

As of Sandfly 5.8, we can grab the process binary directly from the UI using our response feature. The binary will be stored in the server where security teams can download it for further analysis or sandbox detonation. This can all be done without logging into the remote system, which may be safer during the initial investigation steps.

However, operators can also recover the binary by doing the following on the host:

cp /proc/PID/exe /tmp/recovered_bin

Operators can also grab the hash of the file for a quick check in online malware databases. Below we grab a SHA256 hash of the binary using two methods.

In general, poking around inside an active malware directory is not advised as it may have anti-debug protections that may activate. So while running sha256sum can be done directly on any process exe in /proc, it may be better to copy it somewhere safe before running any commands against the binary.

If Sandfly found the malicious process, the UI will already have the correct hash of the binary so there is no need to do this step.

Method 1 - Operating malware process:  sha256sum /proc/PID/exe  Method 2 - Recovered process binary:  sha256sum /tmp/recovered_bin

While we generally discourage using cryptographic hashes for finding Linux malware ([see video here](https://www.youtube.com/watch?v=LjSNJmHuqh4)), they can be used to confidently identify a certain binary is known malware.

### Investigating Hidden Network Operations

The Scales malware runs a full Tor client for command and control. The binary is hidden along with the main malware process itself, plus the network connections are being obscured. Below we run the ss command and we can see legitimate connected services that show the process that owns it.

ss -tupna

We see two connections to Tor entry nodes (TCP 9100 and 1993) in our listing in established state. These two connections do not show the process name nor the PID that owns that process. It is simply blank. This is due to the malware hiding this information. A legitimate service of a user connected to the SSH daemon is shown for comparison.

The IP addresses shown here are at this time known Tor entry nodes, but due to how Tor works these can change at any moment so hunting for these IPs is not advised.

The Tor process is running under a bogus dbus-daemon name and is also decloaked by Sandfly during the stealth rootkit alerting.

### Investigating eBPF Features

This rootkit deploys an advanced rootkit using eBPF to hide the process and network activity. eBPF is a more reliable way to deploy stealth rootkit hiding on Linux vs. older Loadable Kernel Modules (LKM) style rootkits.

The main advantage over LKM is that eBPF can load easily into newer kernels without needing to compile against specific versions and risk crashing the system. This makes this rootkit far more capable and reliable than what security teams may have encountered in the past. However, even though eBPF is very capable of hiding, it does leave behind evidence that can help teams find this malware or variants.

The first indicator that this process is using eBPF to hide is the /proc/PID/fd directory having many open file descriptors pointing to eBPF usage. When you combine this with the fact that the PID is being hidden, it is certain that eBPF rootkits are involved.

This command shows the issue:

ls -al /proc/PID/fd

### Digging Further Into eBPF Rootkit Functions

We can do a couple things now to see if we can determine what may be happening without getting into full blown memory analysis on the host.

First, we'll use the built-in bpftool to dump information about what eBPF programs and names are running on this system. We'll use the following commands.

bpftool link show bpftool map show bpftool prog show

First we see with bpftool link show that we have someone that has decided to do something with critical system calls that manage directory entries (getdents), plus it looks like file and network socket operations. If admins are not running a system monitoring tool on a host and this activity is present, something is wrong.

bpftool link show

Next we'll dump the eBPF maps which conveniently list out several extremely suspicious items.

bpftool map show

Clearly seeing hidden_names, hidden_pids, and hidden_inodes is all but certain that eBPF is being abused. Note that these names can be changed by an attacker, but don't look a gift horse in the mouth.

Finally, we'll use bpftool prog show to dump the eBPF programs to see if they have any interesting names, and they do. Below we again get confirmation of several very suspicious names that match the other eBPF information we've already dumped. This again includes interest in directory entries (to hide), operations to perform on execution, and again our ptrace program which is used for anti-debug by this malware.

#### Checking for eBPF Pinning

An interesting thing with eBPF is you can "pin" a program so it remains available to other callers after the original process exits. The Scales eBPF rootkit does this with three critical functions that investigators can see by doing the following:

cd /sys/fs/bpf ls -al

The malware appears to make a directory using part of the /etc/machine-id value on the host. The directory name is visible in the ls listing and going into the directory you see three files:

hidden_inodes
hidden_names
hidden_pids

Using cat to dump the data inside each file will reveal the currently running PID, inodes being hidden, and the name being hidden of dbus-daemon for the Tor client. Again these can all be easily renamed by the attacker, but the artifact is so certain for this variant that there is no way to refute what has happened.

### Trojaned Sudo Command

The malware did other things on the system as well such as install a script to mimic the sudo command to write captured passwords to a secret file under /tmp/.cache.

Sandfly identified the main directory under /usr/bin/.local/ which contained the malicious script, however it also installed the same script under the root user's directory under /root/.local/bin/sudo which was found with drift detection.

Second copy of trojan sudo under /root directory:

The malicious sudo script mimics the sudo command to grab additional credentials and is shown in part below.

### Persistence with Systemd

The malware installs a malicious systemd service. The service name will be the same random name it selected for the process binary. Using Sandfly's drift detection the file is identified like below.

Forensic view of the data shows the full path to the malicious binary.

Operators can also identify where the service file was inserted for systemd processing.

The service file is a simple affair and works reliably to put the malware back into operation on reboot.

### Credential Theft and Rotating Secrets

The Scales malware is an aggressive credential harvester. It will look for a variety of credential types, tokens, and also cryptocurrency wallet data. Some of the data it will seek out includes:

- SSH keys
- Browser cookies and tokens
- Developer repository accounts such as GitHub and npm
- Vault tokens and Docker/Podman credentials
- Other secrets such as VPNs, Slack, Discord, etc.

As it attacked the supply chain for Arch, it's reasonable to assume the perpetrators will use stolen credentials to press a new attack using a similar mechanism. Developers that have been compromised with this malware must rotate all keys immediately and assume all secrets on the affected hosts have been compromised.

### eBPF Sophistication for Smash and Grab Attack

The Scales malware has sophisticated multi-faceted attack chains with enough stealth to hide from security teams that are not looking for it. eBPF rootkits represent an increased level of sophistication that will allow this malware to remain undetected for quite a while on developer systems. We encourage security teams to check for this malware as the attackers can grab credentials and move quickly to new targets.

eBPF rootkits are designed not only to hide from casual view, but to blind traditional auditing tools and security agents. Sandfly's agentless approach operates outside this compromised environment, allowing defenders to de-cloak hidden processes and recover malicious binaries safely.

##

## References

This malware was initially disclosed by Sonatype in this report:

[Atomic Arch: Attackers Hijack Trusted AUR Packages to Deliver Rootkit-Like Malware](https://www.sonatype.com/blog/atomic-arch-npm-campaign-adds-malicious-dependency)

Another write-up breaks down the eBPF core of this malware:

[Scales - Carving an Embedded eBPF Rootkit](https://sha0coder.github.io/scales/)

Post Tags:

[Rootkits](https://sandflysecurity.com/blog/tag/rootkits) [Malware](https://sandflysecurity.com/blog/tag/malware) [Linux Forensics](https://sandflysecurity.com/blog/tag/linux-forensics)

Share this post:

[← Return to Blog](https://sandflysecurity.com/blog)

-   [Tweet](https://twitter.com/share)
-
-

https://sandflysecurity.com/blog/basic-linux-malware-process-forensics-for-incident-responders

#### [Basic Linux Malware Process Forensics for Incident Responders](https://sandflysecurity.com/blog/basic-linux-malware-process-forensics-for-incident-responders)

[Sandfly Security - Agentless Linux EDR and Incident Response](https://sandflysecurity.com/blog/author/sandfly-security-agentless-linux-edr-and-incident-response) : 30 September 2019

We going to do some basic Linux malware process forensics using the command line and some simple investigation techniques for incident responders.

[Rootkits](https://sandflysecurity.com/blog/tag/rootkits)  [Malware](https://sandflysecurity.com/blog/tag/malware)  [Linux Security](https://sandflysecurity.com/blog/tag/linux-security)  [Linux Forensics](https://sandflysecurity.com/blog/tag/linux-forensics)

[Read More](https://sandflysecurity.com/blog/basic-linux-malware-process-forensics-for-incident-responders)

https://sandflysecurity.com/blog/detecting-linux-memfd-create-fileless-malware-with-command-line-forensics

#### [Linux Malware Detection | Ubuntu, Debian, Redhat, Suse, Fedora, Raspberry Pi etc Malware Detect](https://sandflysecurity.com/blog/detecting-linux-memfd-create-fileless-malware-with-command-line-forensics)

[Sandfly Security - Agentless Linux EDR and Incident Response](https://sandflysecurity.com/blog/author/sandfly-security-agentless-linux-edr-and-incident-response) : 08 July 2020

How to investigate fileless malware on Linux for different distributions: Ubuntu, Debian, Redhat, Suse, Fedora, Raspberry Pi, Arch Linux, CentOS......

[Rootkits](https://sandflysecurity.com/blog/tag/rootkits)  [Malware](https://sandflysecurity.com/blog/tag/malware)  [Linux Security](https://sandflysecurity.com/blog/tag/linux-security)  [Linux Forensics](https://sandflysecurity.com/blog/tag/linux-forensics)

[Read More](https://sandflysecurity.com/blog/detecting-linux-memfd-create-fileless-malware-with-command-line-forensics)

https://sandflysecurity.com/blog/how-to-recover-a-deleted-binary-from-active-linux-malware

#### [How To Recover A Deleted Binary From Active Linux Malware](https://sandflysecurity.com/blog/how-to-recover-a-deleted-binary-from-active-linux-malware)

[Sandfly Security - Agentless Linux EDR and Incident Response](https://sandflysecurity.com/blog/author/sandfly-security-agentless-linux-edr-and-incident-response) : 03 June 2019

You can easily recover a deleted Linux process binary that has been removed by malware or other malcious actor. We'll show you how to do it.

[Rootkits](https://sandflysecurity.com/blog/tag/rootkits)  [Malware](https://sandflysecurity.com/blog/tag/malware)  [Linux Security](https://sandflysecurity.com/blog/tag/linux-security)  [Linux Forensics](https://sandflysecurity.com/blog/tag/linux-forensics)

[Read More](https://sandflysecurity.com/blog/how-to-recover-a-deleted-binary-from-active-linux-malware)
