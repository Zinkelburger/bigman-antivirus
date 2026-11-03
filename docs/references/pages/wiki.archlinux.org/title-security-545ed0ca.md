---
url: "https://wiki.archlinux.org/title/Security"
title: "Security - ArchWiki"
captured: "2026-11-03T02:58:37Z"
via: "live"
sha256: "9f1e7a3de80b83092548dbf32d7afb3b7becd14dd099d4ea4b776ca122ce4492"
wayback: "https://web.archive.org/web/20261026194955/https://wiki.archlinux.org/title/Security"
cited_in:
  - "docs/research/hardening.md"
---

# Security - ArchWiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.archlinux.org/title/Security>
> Snapshot: <https://web.archive.org/web/20261026194955/https://wiki.archlinux.org/title/Security>

# Security

- [Deutsch](https://wiki.archlinux.de/title/Sicherheit)
- [Español](https://wiki.archlinux.org/title/Security_(Español))
- [Magyar](https://wiki.archlinux.org/title/Security_(Magyar))
- [日本語](https://wiki.archlinux.jp/index.php/%E3%82%BB%E3%82%AD%E3%83%A5%E3%83%AA%E3%83%86%E3%82%A3)
- [Polski](https://wiki.archlinux.org/title/Security_(Polski))
- [Português](https://wiki.archlinux.org/title/Security_(Português))
- [Русский](https://wiki.archlinux.org/title/Security_(Русский))
- [中文（简体）](https://wiki.archlinuxcn.org/wiki/Security)

From ArchWiki

Related articles

- [Arch Security Team](https://wiki.archlinux.org/title/Arch_Security_Team)
- [General recommendations](https://wiki.archlinux.org/title/General_recommendations)
- [Identity management](https://wiki.archlinux.org/title/Identity_management)
- [Capabilities](https://wiki.archlinux.org/title/Capabilities)
- [List of Applications/Security](https://wiki.archlinux.org/title/List_of_Applications/Security)
- [Arch package guidelines/Security](https://wiki.archlinux.org/title/Arch_package_guidelines/Security)

This article contains recommendations and best practices for [hardening](https://en.wikipedia.org/wiki/Hardening_(computing)) an Arch Linux system.

## Concepts

- It is possible to tighten security to the point where the system is unusable. Security and convenience must be balanced. The trick is to create a secure and useful system.
- The biggest threat is, and will always be, the user.
- The [principle of least privilege](https://en.wikipedia.org/wiki/Principle_of_least_privilege): Each part of a system should only be able to access what is strictly required, and nothing more.
- Defense in depth: Security works better in independent layers. When one layer is breached, another should stop the attack.
- Be a little paranoid. And be suspicious. If anything sounds too good to be true, it probably is!
- You can never make a system 100% secure unless you unplug the machine from all networks, turn it off, lock it in a safe, smother it in concrete and never use it.
- Prepare for failure. Create a plan ahead of time to follow when your security is broken.

## Passwords

Passwords are key to a secure system. They secure your [user accounts](https://wiki.archlinux.org/title/Users_and_groups), [encrypted file systems](https://wiki.archlinux.org/title/Data-at-rest_encryption), and [SSH](https://wiki.archlinux.org/title/SSH_keys)/[GPG](https://wiki.archlinux.org/title/GPG) keys. They are the main way a computer chooses to trust the person using it, so a big part of security is just about picking secure passwords and protecting them.

### Choosing secure passwords

Passwords must be complex enough to not be easily guessed from e.g. personal information, or [cracked](https://en.wikipedia.org/wiki/Password_cracking) using methods like brute-force attacks. The tenets of strong passwords are based on length and randomness. In cryptography the quality of a password is often referred to as its [entropy](https://en.wikipedia.org/wiki/Password_strength#Entropy_as_a_measure_of_password_strength).

Insecure passwords include those containing or those using as a base before substitution/variation:

- Personally identifiable information (e.g., your dog's name, date of birth, area code, favorite video game)
- Simple character substitutions on words (e.g., k1araj0hns0n), as modern dictionary attacks can easily work with these
- Root "words" or common strings followed or preceded by added numbers, symbols, or characters (e.g., DG091101%)
- Common phrases or short strings of common dictionary words (e.g. photocopyhauntbranchexpose) including with character substitution (e.g. Ph0toc0pyh4uN7br@nch3xp*se) (See Diceware below for when a combination of dictionary words can be secure)
- Any of the [most common passwords](https://en.wikipedia.org/wiki/List_of_the_most_common_passwords)

The best choice for a password is something long (the longer, the better) and generated from a random source. It is important to use a long password. [Weak hash algorithms allow an 8-character password hash to be compromised in just a few hours.](https://www.theregister.com/2019/02/14/password_length)

Tools like [pwgen](https://archlinux.org/packages/?name=pwgen) or [apg](https://aur.archlinux.org/packages/apg/)AUR can generate random passwords. However, these passwords can be difficult to memorize. One memorization technique (for ones often typed) is to generate a long password and memorize a minimally secure number of characters, temporarily writing down the full generated string. Over time, increase the number of characters typed - until the password is ingrained in muscle memory and need not be remembered. This technique is more difficult, but can provide confidence that a password will not turn up in wordlists or "intelligent" brute force attacks that combine words and substitute characters.

Apart from password management, [keepassxc](https://archlinux.org/packages/?name=keepassxc) offers password/passphrase generation. It is possible to customize the generation in a GUI. Dictionary based passphrases are also supported.

One technique for memorizing a password is to use a mnemonic phrase, where each word in the phrase reminds you of the next character in the password. Take for instance “the girl is walking down the rainy street” could be translated to t6!WdtR5 or, less simply, t&6!RrlW@dtR,57. This approach could make it easier to remember a password, but note that the various letters have very different probabilities of being found at the start of words ([Wikipedia:Letter frequency](https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_the_first_letters_of_a_word_in_the_English_language)).

Another effective technique can be to write randomly generated passwords down and store them in a safe place, such as in a wallet, purse, or document safe. Most people do a generally good job of protecting their physical valuables from attack, and it is easier for most people to understand physical security best practices compared to digital security practices.

It is also very effective to combine the mnemonic and random technique by saving long randomly generated passwords with a [password manager](https://wiki.archlinux.org/title/Password_manager), which will be in turn accessed with a memorable "master password"/primary password that must be used only for that purpose. The master password must be memorized and never saved. This requires the password manager to be installed on a system to easily access the password (which could be seen as an inconvenience or a security feature, depending on the situation). Some password managers also have smartphone apps which can be used to display passwords for manual entry on systems without that password manager installed (if that is a common use case, you could still use easily typeable but secure passwords for each service instead of completely random ones, see below). Note that a password manager introduces a single point of failure if you ever forget the master password. Some password managers compute the contained passwords based on the master password and the service name where you want to log in instead of encrypting them, making it possible to use it on a new system without syncing any data.

It can be effective to use a memorable long series of unrelated words as a password. The theory is that if a sufficiently long phrase is used, the gained entropy from the password's length can counter the lost entropy from the use of dictionary words. This [xkcd comic](https://xkcd.com/936/) demonstrates the entropy tradeoff of this method, taking into account the limited set of possible words for each word in the passphrase. If the set of words you choose from is large (multiple thousand words) and you choose 5-7 or even more random words from it, this method provides great entropy, even assuming the attacker knows the set of possible words chosen from and the number of words chosen. The number of possible passphrases after settling on a set of words and number of words is: (number of words in the set of words to select from) to the power of (the number of words chosen for the passphrase). See e.g. [Diceware](https://www.rempe.us/diceware/) for more.

See [The passphrase FAQ](https://www.iusmentis.com/security/passphrasefaq/) or [Wikipedia:Password strength](https://en.wikipedia.org/wiki/Password_strength) for some additional background.

### Maintaining passwords

Once you pick a strong password, be sure to keep it safe. Watch out for [keyloggers](https://en.wikipedia.org/wiki/Keylogger) (software and hardware), screen loggers, [social engineering](https://en.wikipedia.org/wiki/Social_engineering_(security)), [shoulder surfing](https://en.wikipedia.org/wiki/Shoulder_surfing_(computer_security)), and avoid reusing passwords so insecure servers cannot leak more information than necessary. [Password managers](https://wiki.archlinux.org/title/List_of_applications/Security#Password_managers) can help manage large numbers of complex passwords: if you are copy-pasting the stored passwords from the manager to the applications that need them, make sure to clear the copy buffer every time, and ensure they are not saved in any kind of log (e.g. do not paste them in plain terminal commands, which would store them in files like .bash_history). Note that password managers that are implemented as browser extensions may be vulnerable to [side channel attacks](https://www.spookjs.com). These can be mitigated by using password managers that run as separate applications.

As a rule, do not pick insecure passwords just because secure ones are harder to remember. Passwords are a balancing act. It is better to have an encrypted database of secure passwords, guarded behind a key and one strong master password, than it is to have many similar weak passwords. Writing passwords down is perhaps equally effective [[1]](https://www.schneier.com/blog/archives/2005/06/write_down_your.html), avoiding potential vulnerabilities in software solutions while requiring physical security.

Another aspect of the strength of the passphrase is that it must not be easily recoverable from other places.

If you use the same passphrase for disk encryption as you use for your login password (useful e.g. to auto-mount the encrypted partition or folder on login), make sure that /etc/shadow ends up on an encrypted partition or/and uses a strong key derivation function (i.e. yescrypt/argon2 or sha512 with PBKDF2, but not md5 or low iterations in PBKDF2) for the stored password hash (see [SHA password hashes](https://wiki.archlinux.org/title/SHA_password_hashes) for more information).

Tip In 2023 Arch Linux switched the [default hashing](https://archlinux.org/news/changes-to-default-password-hashing-algorithm-and-umask-settings/) algorithm to yescrypt. If you have not customized the default, executing a password change with passwd is necessary (and sufficient) to apply the new default.

If you are backing up your password database, make sure that each copy is not stored behind any other passphrase which in turn is stored in it, e.g. an encrypted drive or an authenticated remote storage service, or you will not be able to access it in case of need; a useful trick is to protect the drives or accounts where the database is backed up using a simple cryptographic hash of the master password. Maintain a list of all the backup locations: if one day you fear that the master passphrase has been compromised you will have to change it immediately on all the database backups and the locations protected with keys derived from the master password.

Version-controlling the database in a secure way can be very complicated: if you choose to do it, you must have a way to update the master password of all the database versions. It may not always be immediately clear when the master password is leaked: to reduce the risk of somebody else discovering your password before you realize that it leaked, you may choose to change it on a periodical basis. If you fear that you have lost control over a copy of the database, you will need to change all the passwords contained in it within the time that it may take to brute-force the master password, according to its entropy.

### Password hashes

A hash is a one-way function, i.e. it is designed to make it impossible to deduct the input without computing the hash function with it (example: MD5, SHA).

A password-hash function is designed to make deducting a user-input (password) impossible without computing the hash function with it (example: bcrypt). A [key derivation function](https://en.wikipedia.org/wiki/Key_derivation_function) (KDF; examples: yescrypt, scrypt, PBKDF2) is a cryptographic algorithm designed to derive secret keys (e.g. an AES key, a password hash) from an input (a master key, a password). Hence, a KDF can serve multiple applications, including those of a password-hash function.

By default, Arch stores the hashed user passwords in the root-only-readable /etc/shadow file, separated from the other user parameters stored in the world-readable /etc/passwd file, see [Users and groups#User database](https://wiki.archlinux.org/title/Users_and_groups#User_database). See also [#Restricting root](https://wiki.archlinux.org/title/Security#Restricting_root).

Passwords are set with the passwd command, which [stretches](https://en.wikipedia.org/wiki/Key_stretching) them with the system's crypt function and then saves them in /etc/shadow. The passwords are also [salted](https://en.wikipedia.org/wiki/Salt_(cryptography)) in order to defend them against [rainbow table](https://en.wikipedia.org/wiki/Rainbow_table) attacks. See also [How are passwords stored in Linux (Understanding hashing with shadow utils)](https://www.slashroot.in/how-are-passwords-stored-linux-understanding-hashing-shadow-utils).

Since password hashes follow a defined format, the method and parameter can be configured for subsequent new invocations of the passwd command. Hence, the individual hashes stored in the /etc/shadow file can be a heterogeneous mix of the hash functions supported by the system.

See [crypt(5)](https://man.archlinux.org/man/crypt.5) for more information on the format, hashing methods and parameters.

The /etc/login.defs file configures the [default password hashing](https://archlinux.org/news/changes-to-default-password-hashing-algorithm-and-umask-settings/) method ENCRYPT_METHOD YESCRYPT and its parameter YESCRYPT_COST_FACTOR.

For example, an increment of the default YESCRYPT_COST_FACTOR parameter will lead to a logarithmic increase of the compute time required to deduce the hash from a password. This applies, likewise, to a third-party trying to obtain the password secret, and the system to authenticate a user log-in.

In contrast, the compute time for the SHA-512 hash function is configured by a parameter with a linear influence. See [SHA password hashes](https://wiki.archlinux.org/title/SHA_password_hashes) for information on the previous Arch default. Note the yescrypt algorithm internally uses SHA-256, HMAC and PBKDF2 to compute its password-hash. The main reason is to combine positive attributes of these widely used and tested functions for an enhanced resistance to attacks. For example, the usability of SHA for various purposes has resulted in hardware support for the function, i.e. the performance to compute a pure SHA hash has accelerated considerably, making its application as a password-hash function more and more derelict.

### Disallow empty passwords

Allowing empty passwords for authentication can increase the attack surface, e.g. see [CVE-2020-27780](https://nvd.nist.gov/vuln/detail/CVE-2020-27780), or [dirtyfrag (CVE-2026-43284 , CVE-2026-43500)](https://github.com/V4bel/dirtyfrag) - a manipulated /etc/shadow with a present nullok option from pam_unix.

To disallow empty passwords remove the nullok option in /etc/pam.d/system-auth:

```
/etc/pam.d/system-auth
```

```
auth       [success=1 default=bad]     pam_unix.so          try_first_pass
```

To list users with currently empty passwords:

```
# getent shadow | awk -F: '($2=="") {print $1}'

```

If the output lists users without an account on the local system (e.g. authenticating via [OpenLDAP](https://wiki.archlinux.org/title/OpenLDAP)), these remain unaffected by the nullok option removal.

### Enforcing strong passwords with pam_pwquality

PAM stands for the Pluggable Authentication Modules. pam_pwquality provides protection against [Dictionary attacks](https://en.wikipedia.org/wiki/Dictionary_attack) and helps configure a password policy that can be enforced throughout the system. It is based on pam_cracklib, so it is backwards compatible with its options.

[Install](https://wiki.archlinux.org/title/Install) the [libpwquality](https://archlinux.org/packages/?name=libpwquality) package.

Warning The root account is not affected by this policy by default.

Note

- You can use the root account to set a password for a user that bypasses the desired/configured policy. This is useful when setting temporary passwords.
- Current security guidelines around passwords, e.g. from NIST, but also from others, do not recommend enforcing special characters, since they often only lead to predictable alterations.

If for example you want to enforce this policy:

- prompt 2 times for password in case of an error (retry option)
- 10 characters minimum length (minlen option)
- at least 6 characters should be different from old password when entering a new one (difok option)
- at least 1 digit (dcredit option)
- at least 1 uppercase (ucredit option)
- at least 1 lowercase (lcredit option)
- at least 1 other character (ocredit option)
- cannot contain the words "myservice" and "mydomain"
- enforce the policy for root

Edit the /etc/pam.d/passwd file to read as:

```
#%PAM-1.0
password required pam_pwquality.so retry=2 minlen=10 difok=6 dcredit=-1 ucredit=-1 ocredit=-1 lcredit=-1 [badwords=myservice mydomain] enforce_for_root
password required pam_unix.so use_authtok yescrypt shadow
```

The password required pam_unix.so use_authtok instructs the pam_unix module to not prompt for a password but rather to use the one provided by pam_pwquality.

You can refer to the [pam_pwquality(8)](https://man.archlinux.org/man/pam_pwquality.8) and [pam_unix(8)](https://man.archlinux.org/man/pam_unix.8) man pages for more information.

## CPU

### Microcode

See [microcode](https://wiki.archlinux.org/title/Microcode) for information on how to install important security updates for your CPU's microcode.

### Hardware vulnerabilities

Some CPUs contain hardware vulnerabilities. See the [kernel documentation on hardware vulnerabilities](https://docs.kernel.org/admin-guide/hw-vuln/) for a list of these vulnerabilities, as well as mitigation selection guides to help customize the kernel to mitigate these vulnerabilities for specific usage scenarios.

To check if you are affected by a known vulnerability, run the following:

```
$ grep -r . /sys/devices/system/cpu/vulnerabilities/

```

In most cases, updating the kernel and microcode will mitigate vulnerabilities.

#### Simultaneous multithreading (hyper-threading)

[Simultaneous multithreading](https://en.wikipedia.org/wiki/Simultaneous_multithreading) (SMT), also called hyper-threading on Intel CPUs, is a hardware feature that may be a source of [L1 Terminal Fault](https://docs.kernel.org/admin-guide/hw-vuln/l1tf.html) and [Microarchitectural Data Sampling](https://docs.kernel.org/admin-guide/hw-vuln/mds.html) vulnerabilities. The Linux kernel and microcode updates contain mitigations for known vulnerabilities, but [disabling SMT may still be required on certain CPUs if untrusted virtualization guests are present](https://docs.kernel.org/admin-guide/hw-vuln/l1tf.html#virtualization-with-untrusted-guests).

Note Disabling SMT is something mostly hypervisors benefit from.[[2]](https://security.stackexchange.com/questions/219753/sacrificing-30-of-my-cpu-performance-by-disabling-hyper-threading-to-fully-mi/219759#219759) On an ordinary system it has very little to no security benefits.

SMT can often be disabled in your system's firmware. Consult your motherboard or system documentation for more information. You can also disable SMT in the kernel by adding the following [kernel parameter](https://wiki.archlinux.org/title/Kernel_parameter):

```
mitigations=auto,nosmt

```

## Memory

The Linux kernel has settings for [memory management](https://docs.kernel.org/admin-guide/mm/index.html). The repository kernels enable the Linux [Address Space Layout Randomization (ASLR)](https://en.wikipedia.org/wiki/Address_space_layout_randomization):

```
$ cat /proc/sys/kernel/randomize_va_space
```

```
2

```

A runtime configuration of 2 is the maximum, but further configuration can be done, see [#Userspace ASLR comparison](https://wiki.archlinux.org/title/Security#Userspace_ASLR_comparison).

The kernel's memory allocation also allows configuration for the sanitization of memory.[[3]](https://www.openwall.com/lists/kernel-hardening/2019/06/26/5) For example, [linux](https://archlinux.org/packages/?name=linux) defaults to:

```
# journalctl -kg 'mem auto-init'
```

```
archlinux kernel: mem auto-init: stack:all(zero), heap alloc:on, heap free:off

```

The CPU stack clearing (stack:all(zero]) is set during compilation, the memory pool sanitization (heap allow:on and heap free:off) can be configured via the [kernel parameters](https://wiki.archlinux.org/title/Kernel_parameters) init_on_alloc and init_on_free. For example, after adding init_on_free=1:

```
# journalctl -kg 'mem auto-init'
```

```
archlinux kernel: mem auto-init: stack:all(zero), heap alloc:on, heap free:on

```

### Hardened malloc

[hardened_malloc](https://aur.archlinux.org/packages/hardened_malloc/)AUR is a hardened replacement for [glibc](https://en.wikipedia.org/wiki/GNU_C_Library)'s malloc(). The project was originally developed for integration into Android's [Bionic](https://en.wikipedia.org/wiki/Bionic_(software)) and [musl](https://en.wikipedia.org/wiki/musl) by Daniel Micay, of [GrapheneOS](https://en.wikipedia.org/wiki/GrapheneOS), but he has also built in support for standard Linux distributions on the x86_64 architecture.

## Storage

### Data-at-rest encryption

[Data-at-rest encryption](https://wiki.archlinux.org/title/Data-at-rest_encryption), preferably full-disk encryption with a [strong passphrase](https://wiki.archlinux.org/title/Security#Passwords), is the only way to guard data against physical recovery. This provides data confidentiality when the computer is turned off or the disks in question are unmounted.

Once the computer is powered on and the drive is mounted, however, its data becomes just as vulnerable as an unencrypted drive. It is therefore best practice to unmount data partitions as soon as they are no longer needed.

You may also [encrypt a drive with the key stored in a TPM](https://wiki.archlinux.org/title/Trusted_Platform_Module#LUKS_encryption), although it has had [vulnerabilities in the past](https://tpm.fail) and the key can be extracted by a [bus sniffing attack](https://pulsesecurity.co.nz/articles/TPM-sniffing).

Certain programs, like [dm-crypt](https://wiki.archlinux.org/title/Dm-crypt), allow the user to encrypt a loop file as a virtual volume. This is a reasonable alternative to full-disk encryption when only certain parts of the system need to be secure.

While the block-device or file-system-based encryption types compared in the [data-at-rest encryption](https://wiki.archlinux.org/title/Data-at-rest_encryption) article are useful at protecting data on physical media, most can not be used to protect data on a remote system that you can not control (such as [cloud storage](https://wiki.archlinux.org/title/Data-at-rest_encryption#Cloud-storage_optimized)). In some cases, individual file encryption will be useful.

These are some methods to encrypt files:

- [age](https://archlinux.org/packages/?name=age) is a simple and easy to use file encryption tool. It supports multiple recipients and encryption using SSH keys, which is useful for secure file sharing.
- Some [archiving and compression](https://wiki.archlinux.org/title/Archiving_and_compression) tools provide basic encryption. One should not rely on them without understanding the security properties of a particular tool and its configuration, as they may use weak algorithms for compatibility reasons, especially when using the zip file format[[4]](https://math.ucr.edu/~mike/zipattacks.pdf). Examples include [7-Zip](https://wiki.archlinux.org/title/7-Zip) (-p flag), [zip](https://archlinux.org/packages/?name=zip) (-e flag).
- [GnuPG](https://wiki.archlinux.org/title/GnuPG) can be used to [encrypt files](https://wiki.archlinux.org/title/GnuPG#Encrypt_and_decrypt). However it requires substantial training and knowledge to use securely[[5]](https://www.gnupg.com/20260122-39C3_reply_gpg_fail.html)[[6]](https://gpg.fail/), has some problems especially when using asymmetric cryptography[[7]](https://www.latacora.com/blog/2019/07/16/the-pgp-problem/#the-answers)[[8]](https://blog.cryptographyengineering.com/2014/08/13/whats-matter-with-pgp/) and includes [GnuPG-proprietary extensions](https://wiki.archlinux.org/title/GnuPG#OpenPGP_compatibility) incompatible with the OpenPGP standard. GnuPG maintainers recommend using the graphical [kleopatra](https://archlinux.org/packages/?name=kleopatra) application instead of the GPG CLI to prevent common user errors[[9]](https://www.gnupg.com/20260122-39C3_reply_gpg_fail.html).

### File systems

The kernel now prevents security issues related to hardlinks and symlinks if the fs.protected_hardlinks and fs.protected_symlinks sysctl switches are enabled, so there is no longer a major security benefit from separating out world-writable directories.

File systems containing world-writable directories can still be kept separate as a coarse way of limiting the damage from disk space exhaustion. However, filling /var or /tmp is enough to take down services. More flexible mechanisms for dealing with this concern exist (like [quotas](https://wiki.archlinux.org/title/Disk_quota)), and some [file systems](https://wiki.archlinux.org/title/File_systems) include related features themselves (Btrfs has quotas on subvolumes).

#### Mount options

Following the principle of least privilege, file systems should be mounted with the most restrictive mount options possible (without losing functionality).

Relevant mount options are:

- nodev: Do not interpret character or block special devices on the file system.
- nosuid: Do not allow set-user-identifier or set-group-identifier bits to take effect.
- noexec: Do not allow direct execution of any binaries on the mounted file system.

- Setting noexec on /home disallows executable scripts and breaks [Wine](https://wiki.archlinux.org/title/Wine), [Steam](https://wiki.archlinux.org/title/Steam), PyCharm, [.NET](https://wiki.archlinux.org/title/.NET), etc.

- Wine does not need the exec flag for opening Windows binaries. It is only needed when Wine itself is installed in /home.
- To keep [Steam](https://wiki.archlinux.org/title/Steam) working you can mount /home/user/.local/share/Steam as exec in [fstab](https://wiki.archlinux.org/title/Fstab) by adding the following:

```
/home/user/.local/share/Steam  /home/user/.local/share/Steam  none defaults,bind,user,nodev,nosuid,exec,nofail  0  0
```

- Some packages (building [nvidia-open-dkms](https://archlinux.org/packages/?name=nvidia-open-dkms) for example) may require exec on /var.

File systems used for data should always be mounted with nodev, nosuid and noexec.

Potential file system mounts to consider:

- /var
- /home
- /dev/shm
- /tmp
- /boot

Tip When using [GPT partition automounting](https://wiki.archlinux.org/title/Systemd#GPT_partition_automounting), the ESP and XBOOTLDR partitions are [always hardened](https://github.com/systemd/systemd-stable/commit/49804cfb71d3a79f433096e4cfb5616980171336) with noexec,nosuid,nodev.

##### SUID-free

If you are able to accept some loss of functionality by switching to [run0](https://wiki.archlinux.org/title/Run0) instead of [sudo](https://wiki.archlinux.org/title/Sudo) and all other SUID and file system [capabilities](https://wiki.archlinux.org/title/Capabilities), it is possible to mount all file systems including the root file system with nosuid.

First, find the [#SUID and SGID files](https://wiki.archlinux.org/title/Security#SUID_and_SGID_files) on the system.

Next, consider binaries with file system capabilities:

```
$ getcap -r /

```

Systems with [NVIDIA](https://wiki.archlinux.org/title/NVIDIA) GPUs need the following drop-in to compensate for loss of SUID for nvidia-modprobe:

```
/etc/systemd/system/local-nvidia.service
```

```
[Service]
Type=simple
ExecStart=/usr/bin/nvidia-modprobe -m -l -s

[Install]
WantedBy=multi-user.target
```

In the SUID-free setup, it is also possible to globally enable NoNewPrivileges=yes, which also disables SUID/SGID and FS capabilities:

```
/etc/systemd/system.conf.d/local-nnp.conf
```

```
[Manager]
NoNewPrivileges=yes
```

It is also possible to remove individual capabilities from PID 1 and all its children, or from individual units. See [Capabilities](https://wiki.archlinux.org/title/Capabilities) and [systemd/Sandboxing#Capabilities](https://wiki.archlinux.org/title/Systemd/Sandboxing#Capabilities) for details.

#### Snapshots

When utilizing file system snapshots, e.g. with [Btrfs](https://wiki.archlinux.org/title/Btrfs), [LVM](https://wiki.archlinux.org/title/LVM), or [ZFS](https://wiki.archlinux.org/title/ZFS), it is essential to be aware that snapshots may retain sensitive information that users expect to be deleted. This is especially true when automatic snapshotting tools like [Snapper](https://wiki.archlinux.org/title/Snapper) are configured, as they can capture snapshots at regular intervals or in response to system events. Here are some examples of how sensitive information in /home/ can persist within snapshots:

- Deleted files and directories: Even though files or directories are deleted from the file system, they may still exist within older snapshots. This is expected most of the time, but consider whether files and directories such as .local/share/Trash/, .history, etc. should be retained.
- Temporary files and cache: Temporary files and cached data generated by applications may be included in snapshots. For example, files kept in encrypted directories might generate thumbnails (.cache/thumbnails) or work copies when opened, which might in turn be included in snapshots. The same applies e.g. to browsing history (.mozilla/, .config/chromium/, etc.), which could have been included in a snapshot before being purged.

If this is supported, consider excluding such directories from snapshots altogether. For example, if using [Btrfs](https://wiki.archlinux.org/title/Btrfs), you can create subvolumes for example .cache/, .config/, .local/, .var/ or any other directory according to your use-case.

Note Moving .local/share/Trash to a separate subvolume might break the trash feature in some cases, e.g. with [GNOME/Files](https://wiki.archlinux.org/title/GNOME/Files).

### File access permissions

The factual accuracy of this article or section is disputed.

Reason: chmod go-r does not "take away all permissions", it only removes the read permission. (Discuss in [Talk:Security](https://wiki.archlinux.org/title/Talk:Security))

The default [file permissions](https://wiki.archlinux.org/title/File_permissions) allow read access to almost everything and changing the permissions can hide valuable information from an attacker who gains access to a non-root account such as the http or nobody users. You can use [chmod](https://wiki.archlinux.org/title/Chmod) to take away all permissions from the group and others:

```
# chmod go-r path_to_hide

```

Warning Do not apply this broadly. Try this for one config at a time, ensuring that it is worth hiding, and that it will not break program functionality. You may need to remove the g from the command (or re-add the permission with chmod g+r path if already ran) if the group is relied on.

Some paths to consider are:

- /boot: The [boot directory](https://wiki.archlinux.org/title/Partitioning#/boot), which may include traditional [vmlinuz](https://wiki.archlinux.org/title/Vmlinuz) and [initramfs](https://wiki.archlinux.org/title/Initramfs) images, or a [Unified kernel image](https://wiki.archlinux.org/title/Unified_kernel_image). Note that safe permissions are used by default when using [systemd#GPT partition automounting](https://wiki.archlinux.org/title/Systemd#GPT_partition_automounting).
- /etc/nftables.conf: The [nftables](https://wiki.archlinux.org/title/Nftables) configuration, applicable to [nftables](https://archlinux.org/packages/?name=nftables) and [iptables](https://archlinux.org/packages/?name=iptables).
- /etc/iptables: The legacy [iptables](https://wiki.archlinux.org/title/Iptables) configuration, applicable to [iptables-legacy](https://archlinux.org/packages/?name=iptables-legacy).

The default [umask](https://wiki.archlinux.org/title/Umask) 0022 can be changed to improve security for newly created files. The [NSA RHEL5 Security Guide](https://apps.nsa.gov/iaarchive/library/ia-guidance/security-configuration/operating-systems/guide-to-the-secure-configuration-of-red-hat-enterprise.cfm) suggests a umask of 0077 for maximum security, which makes new files not readable by users other than the owner. To change this, see [Umask#Set the mask value](https://wiki.archlinux.org/title/Umask#Set_the_mask_value). If you use [sudo](https://wiki.archlinux.org/title/Sudo), consider configuring it to use the [default root umask](https://wiki.archlinux.org/title/Sudo#Permissive_umask).

### SUID and SGID files

It is important to be aware of any files with the [Setuid](https://en.wikipedia.org/wiki/Setuid) (SUID) or Setgid (SGID) bit. To search for files with either the SUID or SGID bit:

```
$ find / -perm "/u=s,g=s" -type f 2>/dev/null

```

Common examples of relevant files with the SUID bit set:

- [unix_chkpwd](https://wiki.archlinux.org/title/PAM)
- chage, expiry, gpasswd, [passwd](https://wiki.archlinux.org/title/Passwd), sg ([shadow](https://archlinux.org/packages/?name=shadow))
- [fusermount3](https://wiki.archlinux.org/title/FUSE), fusermount2
- [pkexec](https://wiki.archlinux.org/title/Polkit)
- [ssh-keysign](https://wiki.archlinux.org/title/OpenSSH)
- chfn, chsh, mount, newgrp, umount, wall, write ([util-linux](https://archlinux.org/packages/?name=util-linux))
- [sudo](https://wiki.archlinux.org/title/Sudo), [sudo-rs](https://archlinux.org/packages/?name=sudo-rs), [doas](https://wiki.archlinux.org/title/Doas), [su](https://wiki.archlinux.org/title/Su), su-rs, [ksu](https://wiki.archlinux.org/title/Kerberos)
- [firejail](https://wiki.archlinux.org/title/Firejail)
- [dbus-daemon-launch-helper](https://wiki.archlinux.org/title/Dbus)
- [chromium-sandbox](https://wiki.archlinux.org/title/Chromium)
- [Xorg.wrap](https://wiki.archlinux.org/title/Xorg)

The prominent risks of such executable files include privilege escalation vulnerabilities, see e.g [Wikipedia:Setuid#Security impact](https://en.wikipedia.org/wiki/Setuid#Security_impact).[[10]](https://www.cvedetails.com/vulnerability-list/vendor_id-16224/product_id-36412/Calibre-ebook-Calibre.html)[[11]](https://www.cvedetails.com/product/32625/Sudo-Project-Sudo.html?vendor_id=15714)[[12]](https://www.cvedetails.com/vulnerability-list/vendor_id-16191/Firejail-Project.html)

Files with the SUID bit set and not owned by root, or files with the SGID bit set typically have less potential impact but can theoretically still do decent damage if vulnerable. It is usually possible to avoid using SUID or SGID by assigning [Capabilities](https://wiki.archlinux.org/title/Capabilities) instead.

Tip It is vital to be vigilant in keeping packages which provide SUID/SGID executables up to date in order to prevent having a vulnerable system; also see [#SUID-free](https://wiki.archlinux.org/title/Security#SUID-free) as an option.

### Backups

Backups are essential for security because they enable recovery from [ransomware](https://en.wikipedia.org/wiki/Ransomware), data corruption, accidental deletion, and system compromise without permanent data loss. See [System backup](https://wiki.archlinux.org/title/System_backup).

### SATA SSD frozen mode

SATA SSD are most likely vulnerable to ATA SECURITY ERASE UNIT commands, when waking up from S3 sleep, to mitigate see [Solid state drive#Setting the SATA SSD state to frozen mode after waking up from sleep](https://wiki.archlinux.org/title/Solid_state_drive#Setting_the_SATA_SSD_state_to_frozen_mode_after_waking_up_from_sleep).

## User setup

### Do not use the root account for daily use

Following the principle of least privilege, do not use the root user for daily use. Create a non-privileged user account for each person using the system. See [List of applications/Security#Privilege elevation](https://wiki.archlinux.org/title/List_of_applications/Security#Privilege_elevation) for ways of temporarily gaining privileged access.

### Enforce a delay after a failed login attempt

Add the following line to /etc/pam.d/system-login to add a delay of at least 4 seconds between failed login attempts:

```
/etc/pam.d/system-login
```

```
auth optional pam_faildelay.so delay=4000000
```

Note This line needs to be the first line in the file.

4000000 is the time in microseconds to delay.

Other PAM modules besides pam_faildelay can also suggest such a delay; if multiple modules do so, PAM will use the longest one.

In particular, both pam_unix and pam_faillock set a minimum delay of 2 seconds by default. In order to completely remove this delay, you need to add the nodelay parameter to any auth lines of these modules, for example

```
/etc/pam.d/system-auth
```

```
auth       [success=1 default=bad]     pam_unix.so          try_first_pass nullok nodelay
```

### Lock out user after three failed login attempts

Since [pambase](https://archlinux.org/packages/?name=pambase) 20200721.1-2, pam_faillock.so is enabled by default to lock out users for 10 minutes after 3 failed login attempts in a 15 minute period (see [FS#67644](https://bugs.archlinux.org/task/67644)). The lockout only applies to password authentication (e.g. login and sudo), public key authentication over SSH is still accepted. To prevent complete denial-of-service, this lockout is disabled for the root user by default.

To unlock a user, do:

```
$ faillock --user username --reset

```

By default, the lock mechanism is a file per-user located at /run/faillock/. Deleting or emptying the file unlocks that user—the directory is owned by root, but the file is owned by the user, so the faillock command only empties the file, therefore does not require root.

The module pam_faillock.so can be configured with the file /etc/security/faillock.conf. The lockout parameters:

- unlock_time — the lockout time (in seconds, default 10 minutes).
- fail_interval — the time in which failed logins can cause a lockout (in seconds, default 15 minutes).
- deny — the number of failed logins before lockout (default 3).

Tip The primary purpose for the lockout is to slow down brute-force attacks so that they become infeasible. Hence, if lockouts due to mistyping of passwords become too frequent, relaxing the number of attempts may be preferred to reducing the lockout time.

Note deny = 0 will disable the lockout mechanism entirely.

By default, all user locks are lost after reboot. If your attacker can reboot the machine, it is more secure if locks persist. To make locks persist, change the dir parameter in /etc/security/faillock.conf to /var/lib/faillock.

No restart is required for changes to take effect. See [faillock.conf(5)](https://man.archlinux.org/man/faillock.conf.5) for further configuration options, such as enabling lockout for the root account, disabling for centralized login (e.g. LDAP), etc.

Note If you make locks persistent, following the changes introduced in polkit 127: you may have to relax the sandbox of its helper agent in order to keep it functional. The best way is to create a drop-in for its systemd unit via systemctl edit polkit-agent-helper\@.service and add:

```
[Service]
ReadWritePaths=/var/lib/faillock

```

### Limit amount of processes

On systems with many, or untrusted users, it is important to limit the number of processes each can run at once, therefore preventing [fork bombs](https://en.wikipedia.org/wiki/Fork_bomb) and other denial of service attacks. The /etc/security/limits.conf configuration determines how many processes each user, or group can have open, and is empty (except for useful comments) by default. Adding the following lines to this file will limit all users to 100 active processes, unless they use the prlimit command to explicitly raise their maximum to 200 for that session. These values can be changed according to the appropriate number of processes a user should have running, or the hardware of the box you are administrating.

```
* soft nproc 100
* hard nproc 200

```

The current number of threads for each user can be found with ps --no-headers -Leo user | sort | uniq --count. This may help with determining appropriate values for the users' limits; see also [limits.conf](https://wiki.archlinux.org/title/Limits.conf).

### Display server

Prefer using [Wayland](https://wiki.archlinux.org/title/Wayland) over [Xorg](https://wiki.archlinux.org/title/Xorg) as [Display server](https://wiki.archlinux.org/title/List_of_applications/Other#Display_servers). Xorg's design predates modern security practices and is [considered insecure](https://security.stackexchange.com/questions/4641/why-are-people-saying-that-the-x-window-system-is-not-secure/4646#4646) by many. For example, Xorg applications may record keystrokes while inactive.

For Xorg applications within Wayland, the Xwayland compatibility layer will automatically use rootless Xorg. If you must run [Xorg](https://wiki.archlinux.org/title/Xorg), it is always recommended to [avoid running it as root](https://wiki.archlinux.org/title/Xorg#Rootless_Xorg) and to [make sure it does not listen on abstract and TCP sockets](https://wiki.archlinux.org/title/Xorg#Sockets).

## Restricting root

The root user is, by definition, the most powerful user on a system. It is also difficult to [audit](https://wiki.archlinux.org/title/Audit) the root user account. It is therefore important to restrict usage of the root user account as much as possible. There are a number of ways to keep the power of the root user while limiting its ability to cause harm.

### Use sudo instead of su

Using [sudo](https://wiki.archlinux.org/title/Sudo) for privileged access is preferable to [su](https://wiki.archlinux.org/title/Su) for a number of reasons:

- It keeps a log of which normal privilege user has run each privileged command.
- The root user password need not be given out to each user who requires root access.
- sudo prevents users from accidentally running commands as root that do not need root access, because a full root terminal is not created. This aligns with the [principle of least privilege](https://en.wikipedia.org/wiki/Principle_of_least_privilege).
- Individual programs may be enabled per user, instead of offering complete root access just to run one command.

See [Sudo#Configuration](https://wiki.archlinux.org/title/Sudo#Configuration).

#### Editing files using sudo

See [Sudo#Editing files](https://wiki.archlinux.org/title/Sudo#Editing_files). Alternatively, you can use editors like rvim or rnano which have restricted capabilities in order to be safe to run as root.

### Restricting root login

Once [sudo](https://wiki.archlinux.org/title/Sudo) is properly configured, full root access can be heavily restricted or denied without losing much usability. To disable root, but still allowing to use [sudo](https://wiki.archlinux.org/title/Sudo), you can use [passwd(1)](https://man.archlinux.org/man/passwd.1) with passwd --lock root.

#### Allow only certain users

The [PAM](https://wiki.archlinux.org/title/PAM) pam_wheel.so lets you allow only users in the group wheel to login using [su](https://wiki.archlinux.org/title/Su). See [su#su and wheel](https://wiki.archlinux.org/title/Su#su_and_wheel).

#### Denying SSH login

Even if you do not wish to deny root login for local users, it is always good practice to [deny root login via SSH](https://wiki.archlinux.org/title/OpenSSH#Deny). The purpose of this is to add an additional layer of security before a user can completely compromise your system remotely.

#### Specify acceptable login combinations with access.conf

Warning If you are using GNOME 49 or later, you should make sure the group gdm can log in locally. This can be done with a +:(gdm):LOCAL rule. [[13]](https://gitlab.gnome.org/GNOME/gdm/-/issues/1021)

When someone attempts to log in with [PAM](https://wiki.archlinux.org/title/PAM), /etc/security/access.conf is checked for the first combination that matches their login properties. Their attempt then fails or succeeds based on the rule for that combination.

```
+:root:LOCAL
-:root:ALL

```

Rules can be set for specific groups and users. In this example, the user archie is allowed to login locally, as are all users in the wheel and adm groups. All other logins are rejected:

```
+:archie:LOCAL
+:(wheel):LOCAL
+:(adm):LOCAL
-:ALL:ALL

```

Read more at [access.conf(5)](https://man.archlinux.org/man/access.conf.5)

## Mandatory access control

[Mandatory access control](https://en.wikipedia.org/wiki/Mandatory_Access_Control) (MAC) is a type of security policy that differs significantly from the [discretionary access control](https://en.wikipedia.org/wiki/Discretionary_Access_Control) (DAC) used by default in Arch and most Linux distributions. MAC essentially means that every action a program could perform that affects the system in any way is checked against a security ruleset. This ruleset, in contrast to DAC methods, cannot be modified by users. Using virtually any mandatory access control system will significantly improve the security of your computer, although there are differences in how it can be implemented.

### Pathname MAC

Pathname-based access control is a simple form of access control that offers permissions based on the path of a given file. The downside to this style of access control is that permissions are not carried with files if they are moved around the system. On the positive side, pathname-based MAC can be implemented on a much wider range of file systems, unlike labels-based alternatives.

- [AppArmor](https://wiki.archlinux.org/title/AppArmor) is a [Canonical](https://en.wikipedia.org/wiki/Canonical_(company))-maintained MAC implementation seen as an "easier" alternative to SELinux.
- [TOMOYO](https://wiki.archlinux.org/title/TOMOYO) is another simple, easy-to-use system offering mandatory access control. It is designed to be both simple in usage and in implementation, requiring very few dependencies.

### Labels MAC

Labels-based access control means the extended attributes of a file are used to govern its security permissions. While this system is arguably more flexible in its security offerings than pathname-based MAC, it only works on file systems that support these extended attributes.

- [SELinux](https://wiki.archlinux.org/title/SELinux), based on an [NSA](https://en.wikipedia.org/wiki/NSA) project to improve Linux security, implements MAC completely separate from system users and roles. It offers an extremely robust multi-level MAC policy implementation that can easily maintain control of a system that grows and changes past its original configuration.

### Access Control Lists

[Access Control Lists](https://wiki.archlinux.org/title/Access_Control_Lists) (ACLs) are an alternative to attaching rules directly to the file system in some way. ACLs implement access control by checking program actions against a list of permitted behavior.

## Kernel hardening

### Kernel self-protection / exploit mitigation

The [linux-hardened](https://archlinux.org/packages/?name=linux-hardened) package uses a [basic kernel hardening patch set](https://github.com/anthraxx/linux-hardened) and more security-focused compile-time configuration options than the [linux](https://archlinux.org/packages/?name=linux) package. A custom build can be made to choose a different compromise between security and performance than the security-leaning defaults.

However, it should be noted that several packages (such as [throttled](https://archlinux.org/packages/?name=throttled)) will not work when using this kernel.

If you use an out-of-tree driver such as [NVIDIA](https://wiki.archlinux.org/title/NVIDIA), you may need to switch to its [DKMS](https://wiki.archlinux.org/title/DKMS) package.

#### Userspace ASLR comparison

The [linux-hardened](https://archlinux.org/packages/?name=linux-hardened) package provides an improved implementation of Address Space Layout Randomization for userspace processes. The [paxtest](https://archlinux.org/packages/?name=paxtest) command can be used to obtain an estimate of the provided entropy:

##### 64-bit processes

```
linux-hardened 5.4.21.a-1-hardened
```

```
Anonymous mapping randomization test     : 32 quality bits (guessed)
Heap randomization test (ET_EXEC)        : 40 quality bits (guessed)
Heap randomization test (PIE)            : 40 quality bits (guessed)
Main executable randomization (ET_EXEC)  : 32 quality bits (guessed)
Main executable randomization (PIE)      : 32 quality bits (guessed)
Shared library randomization test        : 32 quality bits (guessed)
VDSO randomization test                  : 32 quality bits (guessed)
Stack randomization test (SEGMEXEC)      : 40 quality bits (guessed)
Stack randomization test (PAGEEXEC)      : 40 quality bits (guessed)
Arg/env randomization test (SEGMEXEC)    : 44 quality bits (guessed)
Arg/env randomization test (PAGEEXEC)    : 44 quality bits (guessed)
Offset to library randomisation (ET_EXEC): 34 quality bits (guessed)
Offset to library randomisation (ET_DYN) : 34 quality bits (guessed)
Randomization under memory exhaustion @~0: 32 bits (guessed)
Randomization under memory exhaustion @0 : 32 bits (guessed)

```

```
linux 5.5.5-arch1-1
```

```
Anonymous mapping randomization test     : 28 quality bits (guessed)
Heap randomization test (ET_EXEC)        : 28 quality bits (guessed)
Heap randomization test (PIE)            : 28 quality bits (guessed)
Main executable randomization (ET_EXEC)  : 28 quality bits (guessed)
Main executable randomization (PIE)      : 28 quality bits (guessed)
Shared library randomization test        : 28 quality bits (guessed)
VDSO randomization test                  : 20 quality bits (guessed)
Stack randomization test (SEGMEXEC)      : 30 quality bits (guessed)
Stack randomization test (PAGEEXEC)      : 30 quality bits (guessed)
Arg/env randomization test (SEGMEXEC)    : 22 quality bits (guessed)
Arg/env randomization test (PAGEEXEC)    : 22 quality bits (guessed)
Offset to library randomisation (ET_EXEC): 28 quality bits (guessed)
Offset to library randomisation (ET_DYN) : 28 quality bits (guessed)
Randomization under memory exhaustion @~0: 29 bits (guessed)
Randomization under memory exhaustion @0 : 29 bits (guessed)

```

```
linux-lts 4.19.101-1-lts
```

```
Anonymous mapping randomization test     : 28 quality bits (guessed)
Heap randomization test (ET_EXEC)        : 28 quality bits (guessed)
Heap randomization test (PIE)            : 28 quality bits (guessed)
Main executable randomization (ET_EXEC)  : 28 quality bits (guessed)
Main executable randomization (PIE)      : 28 quality bits (guessed)
Shared library randomization test        : 28 quality bits (guessed)
VDSO randomization test                  : 19 quality bits (guessed)
Stack randomization test (SEGMEXEC)      : 30 quality bits (guessed)
Stack randomization test (PAGEEXEC)      : 30 quality bits (guessed)
Arg/env randomization test (SEGMEXEC)    : 22 quality bits (guessed)
Arg/env randomization test (PAGEEXEC)    : 22 quality bits (guessed)
Offset to library randomisation (ET_EXEC): 28 quality bits (guessed)
Offset to library randomisation (ET_DYN) : 28 quality bits (guessed)
Randomization under memory exhaustion @~0: 28 bits (guessed)
Randomization under memory exhaustion @0 : 28 bits (guessed)

```

##### 32-bit processes (on an x86_64 kernel)

```
linux-hardened
```

```
Anonymous mapping randomization test     : 16 quality bits (guessed)
Heap randomization test (ET_EXEC)        : 22 quality bits (guessed)
Heap randomization test (PIE)            : 27 quality bits (guessed)
Main executable randomization (ET_EXEC)  : No randomization
Main executable randomization (PIE)      : 18 quality bits (guessed)
Shared library randomization test        : 16 quality bits (guessed)
VDSO randomization test                  : 16 quality bits (guessed)
Stack randomization test (SEGMEXEC)      : 24 quality bits (guessed)
Stack randomization test (PAGEEXEC)      : 24 quality bits (guessed)
Arg/env randomization test (SEGMEXEC)    : 28 quality bits (guessed)
Arg/env randomization test (PAGEEXEC)    : 28 quality bits (guessed)
Offset to library randomisation (ET_EXEC): 18 quality bits (guessed)
Offset to library randomisation (ET_DYN) : 16 quality bits (guessed)
Randomization under memory exhaustion @~0: 18 bits (guessed)
Randomization under memory exhaustion @0 : 18 bits (guessed)

```

```
linux
```

```
Anonymous mapping randomization test     : 8 quality bits (guessed)
Heap randomization test (ET_EXEC)        : 13 quality bits (guessed)
Heap randomization test (PIE)            : 13 quality bits (guessed)
Main executable randomization (ET_EXEC)  : No randomization
Main executable randomization (PIE)      : 8 quality bits (guessed)
Shared library randomization test        : 8 quality bits (guessed)
VDSO randomization test                  : 8 quality bits (guessed)
Stack randomization test (SEGMEXEC)      : 19 quality bits (guessed)
Stack randomization test (PAGEEXEC)      : 19 quality bits (guessed)
Arg/env randomization test (SEGMEXEC)    : 11 quality bits (guessed)
Arg/env randomization test (PAGEEXEC)    : 11 quality bits (guessed)
Offset to library randomisation (ET_EXEC): 8 quality bits (guessed)
Offset to library randomisation (ET_DYN) : 13 quality bits (guessed)
Randomization under memory exhaustion @~0: No randomization
Randomization under memory exhaustion @0 : No randomization

```

##### Manual configuration

You can achieve the same effect on kernels other than [linux-hardened](https://archlinux.org/packages/?name=linux-hardened) using the following sysctl parameters:

```
/etc/sysctl.d/51-aslr.conf
```

```
vm.mmap_rnd_bits = 32
vm.mmap_rnd_compat_bits = 16
```

### Restricting access to kernel pointers in the proc file system

Setting kernel.kptr_restrict to 1 will hide kernel symbol addresses in /proc/kallsyms from regular users without CAP_SYSLOG, making it more difficult for kernel exploits to resolve addresses/symbols dynamically. This will not help that much on a pre-compiled Arch Linux kernel, since a determined attacker could just download the kernel package and get the symbols manually from there, but if you are compiling your own kernel, this can help mitigating local root exploits. This will break some [perf](https://archlinux.org/packages/?name=perf) commands when used by non-root users (but many [perf](https://archlinux.org/packages/?name=perf) features require root access anyway). See [FS#34323](https://bugs.archlinux.org/task/34323) for more information.

Setting kernel.kptr_restrict to 2 will hide kernel symbol addresses in /proc/kallsyms regardless of privileges.

```
/etc/sysctl.d/51-kptr-restrict.conf
```

```
kernel.kptr_restrict = 1
```

Note [linux-hardened](https://archlinux.org/packages/?name=linux-hardened) sets kptr_restrict=2 by default rather than 0.

### BPF hardening

BPF is a system used to load and execute bytecode within the kernel dynamically during runtime. It is used in a number of Linux kernel subsystems such as networking (e.g. XDP, tc), tracing (e.g. kprobes, uprobes, tracepoints) and security (e.g. seccomp). It is also useful for advanced network security, performance profiling and dynamic tracing.

BPF was originally an acronym of [Berkeley Packet Filter](https://en.wikipedia.org/wiki/Berkeley_Packet_Filter) since the original classic BPF was used for packet capture tools for BSD. This eventually evolved into Extended BPF (eBPF), which was shortly afterwards renamed to just BPF (not an acronym). BPF should not be confused with packet filtering tools like iptables or netfilter, although BPF can be used to implement packet filtering tools.

BPF code may be either interpreted or compiled using a [Just-In-Time (JIT) compiler](https://en.wikipedia.org/wiki/Just-in-time_compilation). The Arch kernel is built with CONFIG_BPF_JIT_ALWAYS_ON which disables the BPF interpreter and forces all BPF to use JIT compilation. This makes it harder for an attacker to use BPF to escalate attacks that exploit SPECTRE-style vulnerabilities. See [the kernel patch which introduced CONFIG_BPF_JIT_ALWAYS_ON](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=290af86629b25ffd1ed6232c4e9107da031705cb) for more details.

The kernel includes a hardening feature for JIT-compiled BPF which can mitigate some types of JIT spraying attacks at the cost of performance and the ability to trace and debug many BPF programs. It may be enabled by setting net.core.bpf_jit_harden to 1 (to enable hardening of unprivileged code) or 2 (to enable hardening of all code).

See the net.core.bpf_* settings in the [kernel documentation](https://docs.kernel.org/admin-guide/sysctl/net.html) for more details.

Tip

- [linux-hardened](https://archlinux.org/packages/?name=linux-hardened) sets net.core.bpf_jit_harden=2 by default rather than 0.
- By default, BPF programs can be run even by unprivileged users. To change that behaviour set kernel.unprivileged_bpf_disabled=1[[14]](https://access.redhat.com/security/cve/cve-2021-33624).

### ptrace scope

The [ptrace(2)](https://man.archlinux.org/man/ptrace.2) syscall provides a means by which one process (the "tracer") may observe and control the execution of another process (the "tracee"), and examine and change the tracee's memory and registers. ptrace is commonly used by debugging tools including gdb, strace, perf, reptyr and other debuggers. However, it also provides a means by which a malicious process can read data from and take control of other processes.

Arch enables the [Yama LSM](https://docs.kernel.org/admin-guide/LSM/Yama.html) by default, which provides a kernel.yama.ptrace_scope [kernel parameter](https://wiki.archlinux.org/title/Kernel_parameter). This parameter is set to 1 (restricted) by default which prevents tracers from performing a ptrace call on traces outside of a restricted scope unless the tracer is privileged or has the CAP_SYS_PTRACE [capability](https://wiki.archlinux.org/title/Capabilities). This is a significant improvement in security compared to the classic permissions. Without this module, there is no separation between processes running as the same user (in the absence of additional security layers such as [pid_namespaces(7)](https://man.archlinux.org/man/pid_namespaces.7)).

Note By default, you can still use tools which require ptrace by running them as privileged processes, e.g. using [sudo](https://wiki.archlinux.org/title/Sudo).

If you do not need to use debugging tools, consider setting kernel.yama.ptrace_scope to 2 (admin-only) or 3 (no ptrace possible) to harden the system.

Note Some anti-cheat and DRM implementations rely on ptrace to work, including Easy Anti-Cheat and Ubisoft Connect under Wine. Setting this parameter to 2 or higher might prevent games using these solutions from launching.

### hidepid

This article or section needs expansion.

Reason: [Linux 5.8 implemented private instances](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=0fb5ce62c5920b6e0a8a061f2fe80e0403281e10) and new values for hidepid=. Systemd picked it up with v247 (ProtectProc, ProcSubset, DynamicUser) for service configuration. [#ptrace scope](https://wiki.archlinux.org/title/Security#ptrace_scope) can be used globally. (Discuss in [Talk:Security](https://wiki.archlinux.org/title/Talk:Security))

The factual accuracy of this article or section is disputed.

Reason: Enabling hidepid globally is not a supported way of operation by [systemd](https://wiki.archlinux.org/title/Systemd), nor does it have any practical improvements security-wise when systemd is running as service manager. [[15]](https://github.com/systemd/systemd/issues/29893#issuecomment-1798030108) (Discuss in [Talk:Security](https://wiki.archlinux.org/title/Talk:Security))

Warning

- This may cause issues for certain applications like an application running in a sandbox and [Xorg](https://wiki.archlinux.org/title/Xorg) (see workaround).
- This causes issues with [D-Bus](https://wiki.archlinux.org/title/D-Bus), [Polkit](https://wiki.archlinux.org/title/Polkit), [PulseAudio](https://wiki.archlinux.org/title/PulseAudio) and [bluetooth](https://wiki.archlinux.org/title/Bluetooth) when using [systemd](https://archlinux.org/packages/?name=systemd) > 237.64-1.

The kernel has the ability to hide other users' processes, normally accessible via /proc, from unprivileged users by mounting the proc file system with the hidepid= and gid= options documented in https://docs.kernel.org/filesystems/proc.html.

This greatly complicates an intruder's task of gathering information about running processes, whether some daemon runs with elevated privileges, whether other user runs some sensitive program, whether other users run any program at all, makes it impossible to learn whether any user runs a specific program (given the program does not reveal itself by its behavior), and, as an additional bonus, poorly written programs passing sensitive information via program arguments are now protected against local eavesdroppers.

The proc [group](https://wiki.archlinux.org/title/Users_and_groups#System_groups), provided by the [filesystem](https://archlinux.org/packages/?name=filesystem) package, acts as a whitelist of users authorized to learn other users' process information. If users or services need access to /proc/<pid> directories beyond their own, [add them to the group](https://wiki.archlinux.org/title/Users_and_groups#Group_management).

For example, to hide process information from other users except those in the proc group:

```
/etc/fstab
```

```
proc	/proc	proc	nosuid,nodev,noexec,hidepid=2,gid=proc	0	0
```

For user sessions to work correctly, an exception needs to be added for systemd-logind:

```
/etc/systemd/system/systemd-logind.service.d/hidepid.conf
```

```
[Service]
SupplementaryGroups=proc
```

### Restricting module loading

The default Arch kernel has CONFIG_MODULE_SIG_ALL enabled, which signs all kernel modules built as part of the [linux](https://archlinux.org/packages/?name=linux) package. This allows the kernel to only load modules signed with a valid key, i.e. out-of-tree modules compiled locally or provided by packages such as [nvidia-open](https://archlinux.org/packages/?name=nvidia-open) cannot be loaded. You can use modinfo to verify currently loaded modules have signatures; verifying the signatures by hand is slightly more involved [[16]](https://unix.stackexchange.com/a/496800).

Kernel module loading can be restricted by setting the module.sig_enforce=1 [kernel parameter](https://wiki.archlinux.org/title/Kernel_parameter). More information can be found in the [kernel documentation](https://docs.kernel.org/admin-guide/module-signing.html).

Further, unneeded individual modules can be [blacklisted](https://wiki.archlinux.org/title/Blacklist), see [secureblue](https://github.com/secureblue/secureblue/blob/live/files/system/usr/lib/modprobe.d/secureblue.conf) for examples.

Automatic module loading by the kernel can be hardened by restricting [capabilities](https://wiki.archlinux.org/title/Capabilities) with [sysctl](https://wiki.archlinux.org/title/Sysctl#Restrict_capabilities_for_usermode_helpers).

### Disable loading modules

Some security vulnerabilities like [Copy Fail](https://copy.fail/) are in kernel modules that most users do not need. Disabling the loading of all extraneous modules can prevent such vulnerabilities from being exploited. For this to be viable, your kernel must usually be in a stable state, meaning that it typically does not need to load any more modules once the system has fully started. If and when your kernel reaches that state, you can toggle [modules_disabled](https://docs.kernel.org/admin-guide/sysctl/kernel.html#modules-disabled) to irreversibly disable loading and unloading modules until the next reboot.

### Disable kexec

Kexec allows replacing the current running kernel.

```
/etc/sysctl.d/70-kexec-restrict.conf
```

```
kernel.kexec_load_disabled = 1
```

Tip Kexec is disabled (CONFIG_KEXEC unset[[17]](https://gitlab.archlinux.org/archlinux/packaging/packages/linux-hardened/-/blob/main/config.x86_64?ref_type=heads#L353)) in [linux-hardened](https://archlinux.org/packages/?name=linux-hardened).

### Kernel lockdown mode

Linux supports an optional [lockdown feature](https://mjg59.dreamwidth.org/55105.html), intended to strengthen the boundary between UID 0 (root) and the kernel. When enabled some applications may cease to work which rely on low-level access to either hardware or the kernel.

To use lockdown, its LSM must be initialized and a lockdown mode must be set.

All [officially supported kernels](https://wiki.archlinux.org/title/Kernel#Officially_supported_kernels) initialize the LSM, but none of them enforce any lockdown mode.

Tip Initialized LSMs can be verified by running cat /sys/kernel/security/lsm.

Lockdown has two modes of operation:

- integrity: kernel features that allow userland to modify the running kernel are disabled (e.g. kexec, bpf).
- confidentiality: kernel features that allow userland to extract confidential information from the kernel are also disabled.

It is recommended to use integrity, unless your specific threat model dictates otherwise.

To enable kernel lockdown at runtime, run:

```
# echo mode > /sys/kernel/security/lockdown

```

To enable kernel lockdown on boot, use the [kernel parameter](https://wiki.archlinux.org/title/Kernel_parameter) lockdown=mode.

Note

- Kernel lockdown cannot be disabled at runtime.
- Kernel lockdown disables [hibernation](https://wiki.archlinux.org/title/Hibernation).
- Versions <6.17 of the [kernel_lockdown(7)](https://man.archlinux.org/man/kernel_lockdown.7) man page incorrectly state that "lockdown will be automatically enabled if the system boots in EFI Secure Boot mode". This is not the behaviour of the upstream kernel, nor Arch's packaged [kernels](https://wiki.archlinux.org/title/Kernel).

See also [kernel_lockdown(7)](https://man.archlinux.org/man/kernel_lockdown.7).

### Linux Kernel Runtime Guard (LKRG)

[LKRG](https://www.openwall.com/lkrg/) ([lkrg-dkms](https://aur.archlinux.org/packages/lkrg-dkms/)AUR) is a kernel module which performs integrity checking of the kernel and detection of exploit attempts.

### Disable emergency shell

The factual accuracy of this article or section is disputed.

Reason: Masking emergency.target and emergency.service will have no effect on those units being added to the initramfs and run in early userspace. Even with them in the initramfs, mkinitcpio's systemd hook locks the root account[[18]](https://gitlab.archlinux.org/archlinux/mkinitcpio/mkinitcpio/-/commit/375bf0c6bfa139100b40ac0ad21b8a5379df7877)[[19]](https://gitlab.archlinux.org/archlinux/mkinitcpio/mkinitcpio/-/commit/e6c96788413dbd753d608dc007407027cb0b3507) for "security reasons" (see [FS#70408](https://bugs.archlinux.org/task/70408)). The solution for the issue in the linked article, if even needed, would be to prevent rescue.target, rescue.service, emergency.target and emergency.service from being added to the initramfs image. (Discuss in [Talk:Security](https://wiki.archlinux.org/title/Talk:Security))

The emergency shell is used to interactively troubleshoot the machine during the boot process. However, it is also a gadget that an attacker can use to access secure resources such as the TPM. See [this article](https://pulsesecurity.co.nz/advisories/tpm-luks-bypass) for a practical example. The difficulty of attacks can be increased by disabling the emergency shell, at the tradeoff of removing a tool to troubleshoot early boot failures.

To disable the emergency shell, See [systemd#Disable emergency mode on remote machine](https://wiki.archlinux.org/title/Systemd#Disable_emergency_mode_on_remote_machine).

### Disable unprivileged user namespaces

[user_namespaces(7)](https://man.archlinux.org/man/user_namespaces.7) is a Linux feature that allows users (especially unprivileged users) to become limited root in their own user namespaces. Unprivileged user namespaces are useful for rootless containers and [sandboxing applications](https://wiki.archlinux.org/title/Security#sandboxing_applications), but this feature also increases the attack surface for local privilege escalation; see [AppArmor's Wiki](https://gitlab.com/apparmor/apparmor/-/wikis/unprivileged_userns_restriction) and [FS#36969](https://bugs.archlinux.org/task/36969).

However, alternatives to unprivileged user namespaces (such as [setuid](https://wiki.archlinux.org/title/Security#SUID_and_SGID_files) executables, or IPC with a privileged daemon) might be more insecure and have a larger attack surface, so there are some trade-offs. Disabling this feature can break some [sandboxing applications](https://wiki.archlinux.org/title/Security#sandboxing_applications) such as [bubblewrap](https://wiki.archlinux.org/title/Security#bubblewrap), [nsjail](https://archlinux.org/packages/?name=nsjail), and [rootless podman](https://wiki.archlinux.org/title/Podman#Rootless_Podman). [Chromium](https://wiki.archlinux.org/title/Chromium) based applications need a SUID bit for chrome-sandbox to work with this feature disabled.

There is no easy way in the upstream kernel to prevent unprivileged users from using this feature, so Arch Linux applies [a patch](https://github.com/archlinux/linux/commit/ba7ed06f76e8f258578069593faced505dca159b) to all [officially supported kernels](https://wiki.archlinux.org/title/Kernel#Officially_supported_kernels), which introduces a kernel.unprivileged_userns_clone [sysctl](https://wiki.archlinux.org/title/Sysctl) to toggle it. Unprivileged user namespaces usage is enabled by default in all [officially supported kernels](https://wiki.archlinux.org/title/Kernel#Officially_supported_kernels) except for [linux-hardened](https://archlinux.org/packages/?name=linux-hardened).

Disabling unprivileged user namespaces:

- use the [linux-hardened](https://archlinux.org/packages/?name=linux-hardened) kernel, which disables unprivileged user namespaces by default, or
- set the kernel.unprivileged_userns_clone [sysctl](https://wiki.archlinux.org/title/Sysctl) to 0.

## Restricting system services

It's possible to limit what system services are allowed to do. Use the systemd-analyze security command to review the security of the systemd service units on your machine, and read the dedicated article [systemd/Sandboxing](https://wiki.archlinux.org/title/Systemd/Sandboxing) to learn more.

## Sandboxing applications

See also [Wikipedia:Sandbox (computer security)](https://en.wikipedia.org/wiki/Sandbox_(computer_security)).

### Firejail

[Firejail](https://wiki.archlinux.org/title/Firejail) is an easy to use tool for sandboxing applications and servers alike. It was originally created for browsers and internet facing applications, but supports a large number of applications by now. To establish a sandboxed environment with a variety of features, it is installed as a suid binary and builds a sandboxed runtime environment for the target application based on black and white lists.

### bubblewrap

[bubblewrap](https://wiki.archlinux.org/title/Bubblewrap) is a sandbox application developed for unprivileged container tools like [Flatpak](https://wiki.archlinux.org/title/Flatpak) with a significantly smaller resource footprint and complexity than Firejail. While it lacks certain features such as file path whitelisting, bubblewrap does offer bind mounts as well as the creation of user/IPC/PID/network/cgroup namespaces and can support both simple and complex sandboxes.

bubblewrap requires unprivileged [user_namespaces(7)](https://man.archlinux.org/man/user_namespaces.7), so for the [linux-hardened](https://archlinux.org/packages/?name=linux-hardened) kernel you will need to enable the kernel.unprivileged_userns_clone [sysctl](https://wiki.archlinux.org/title/Sysctl) manually.

[Bubblejail](https://wiki.archlinux.org/title/Bubblejail) sandbox is based on [bubblewrap](https://wiki.archlinux.org/title/Bubblewrap) and provides a resource oriented permission model with a graphical interface to tweak permissions.

### Portable

[Portable](https://github.com/Kraftland/portable) is a sandboxing framework which utilizes [bubblewrap](https://wiki.archlinux.org/title/Bubblewrap) and many other tools to lockdown running applications. It is designed to be simple for packagers and efficient for users, yet cuts off security holes and monitors background processes by default.

See [portable-arch](https://github.com/Kraftland/portable-arch) for a repository of applications sandboxed by portable.

If a sandboxed application does not utilize the Portal file chooser, portable can pass files to the sandbox (by passing --actions share-files).

Portable is fully functional on GNOME, while other desktops may lack small amounts of features like advanced background monitoring and ScreenShot portal.

### chroots

Manual [chroot](https://wiki.archlinux.org/title/Chroot) jails can also be constructed to build sandboxed process environments. It is much more limited than other sandboxing technologies; the extent of its sandboxing is file path isolation.

### Linux containers

[Linux Containers](https://wiki.archlinux.org/title/Linux_Containers) are another good option when you need more separation than the other options (short of [full system virtualization](https://wiki.archlinux.org/title/Security#Full_virtualization_options)) provide. LXC is run on top of the existing kernel in a pseudo-chroot with their own virtual hardware.

### gVisor

The [gVisor](https://gvisor.dev/) project, led by Google, is providing a sandboxing application with a focus on containers following the [OCI initiative](https://opencontainers.org/), such as [Docker](https://wiki.archlinux.org/title/Docker) and [Kubernetes](https://wiki.archlinux.org/title/Kubernetes). It isolates containers and individual applications from the host by intercepting a majority of system calls to the kernel and presenting itself as guest kernel.

A key difference to other intercepting sandboxing projects is that gVisor re-implements system calls in the Go programming language, as described in its [design overview](https://gvisor.dev/docs/architecture_guide/intro/). Details for the list of [re-implemented syscalls support](https://gvisor.dev/docs/user_guide/compatibility/linux/amd64/) can be seen in [git](https://github.com/google/gvisor/blob/master/pkg/sentry/syscalls/linux/linux64.go). For usage examples, limitations and special features see the project [documentation](https://gvisor.dev/docs/).

The application is available as [gvisor-git](https://aur.archlinux.org/packages/gvisor-git/)AUR and [gvisor-bin](https://aur.archlinux.org/packages/gvisor-bin/)AUR.

### Full virtualization options

Using full virtualization options such as [VirtualBox](https://wiki.archlinux.org/title/VirtualBox), [KVM](https://wiki.archlinux.org/title/KVM), [Xen](https://wiki.archlinux.org/title/Xen) or [Qubes OS](https://www.qubes-os.org/) (based on Xen) can also improve isolation and security in the event you plan on running risky applications or browsing dangerous websites.

## Network and firewalls

### Firewalls

While the stock Arch kernel is capable of using [Netfilter](https://en.wikipedia.org/wiki/Netfilter)'s [nftables](https://wiki.archlinux.org/title/Nftables) (and its predecessor [iptables](https://wiki.archlinux.org/title/Iptables)), the services are not [enabled](https://wiki.archlinux.org/title/Enable) by default. It is highly recommended to set up some form of firewall to protect the services running on the system. Many resources (including ArchWiki) do not state explicitly which services are worth protecting, so enabling a firewall is a good precaution.

- See [nftables](https://wiki.archlinux.org/title/Nftables) for general information.
- See [Uncomplicated Firewall](https://wiki.archlinux.org/title/Uncomplicated_Firewall) for a guide on setting up a basic firewall.
- See [Category:Firewalls](https://wiki.archlinux.org/title/Category:Firewalls) for other ways of setting up netfilter.
- See [Ipset](https://wiki.archlinux.org/title/Ipset) for blocking lists of ip addresses, such as those from Bluetack.

#### Open ports

This article or section needs language, wiki syntax or style improvements. See [Help:Style](https://wiki.archlinux.org/title/Help:Style) for reference.

Reason: "Open ports" is not a good title since it disregards interfaces and addresses that the application may be bound to. From the firewalls' point of view, ports may be "open" even if no application listens on them at the moment. (Discuss in [Talk:Security](https://wiki.archlinux.org/title/Talk:Security))

Some services listen for inbound traffic on open network ports. It is important to only bind these services to the addresses and interfaces that are strictly necessary. It may be possible for a remote attacker to [exploit flawed network protocols to access exposed services](https://samy.pl/slipstream/). This can even happen with [processes bound to localhost](https://nvd.nist.gov/vuln/detail/CVE-2019-13450).

In general, if a service only needs to be accessible to the local system, bind to a Unix domain socket ([unix(7)](https://man.archlinux.org/man/unix.7)) or a loopback address such as localhost instead of a non-loopback address like 0.0.0.0/0.

If a service needs to be accessible to other systems via the network, control the access with strict [firewall](https://wiki.archlinux.org/title/Firewall) rules and configure authentication, authorization and encryption whenever possible.

You can list all current open ports with ss -l. To show all listening processes and their numeric tcp and udp port numbers:

```
# ss -lpntu

```

See [ss(8)](https://man.archlinux.org/man/ss.8) for more options.

### Kernel parameters

Kernel parameters which affect networking can be set using [Sysctl](https://wiki.archlinux.org/title/Sysctl). For how to do this, see [Sysctl#TCP/IP stack hardening](https://wiki.archlinux.org/title/Sysctl#TCP/IP_stack_hardening).

### SSH

To mitigate [brute-force attacks](https://en.wikipedia.org/wiki/Brute-force_attack) it is recommended to enforce key-based authentication. For OpenSSH see [OpenSSH#Protection](https://wiki.archlinux.org/title/OpenSSH#Protection) for more recommendations. Alternatively [Fail2ban](https://wiki.archlinux.org/title/Fail2ban) or [Sshguard](https://wiki.archlinux.org/title/Sshguard) offer lesser forms of protection by monitoring logs and writing [firewall](https://wiki.archlinux.org/title/Firewall) rules but open up the potential for a denial of service, since an attacker can [spoof](https://en.wikipedia.org/wiki/Spoofing_attack#Spoofing_and_TCP.2FIP) packets as if they came from the administrator after identifying their address. Spoofing IP has lines of defense, such as by [reverse path filtering](https://wiki.archlinux.org/title/Sysctl#Reverse_path_filtering) and [disabling ICMP redirects](https://wiki.archlinux.org/title/Sysctl#Disable_ICMP_redirects).

You may want to harden authentication even more by using two-factor authentication. [Google Authenticator](https://wiki.archlinux.org/title/Google_Authenticator) provides a two-step authentication procedure using one-time passcodes (OTP).

Denying root login is also a good practice, both for tracing intrusions and adding an additional layer of security before root access. For OpenSSH, see [OpenSSH#Deny](https://wiki.archlinux.org/title/OpenSSH#Deny).

Mozilla publishes an [OpenSSH configuration guide](https://infosec.mozilla.org/guidelines/openssh.html) which configures more verbose audit logging and restricts ciphers.

### DNS

The default domain name resolution (DNS) configuration is highly compatible but has security weaknesses. See [DNS privacy and security](https://wiki.archlinux.org/title/Domain_name_resolution#Privacy_and_security) for more information.

### Proxies

Proxies are commonly used as an extra layer between applications and the network, sanitizing data from untrusted sources. The attack surface of a small proxy running with lower privileges is significantly smaller than a complex application running with the end user privileges.

For example the DNS resolver is implemented in [glibc](https://archlinux.org/packages/?name=glibc), that is linked with the application (that may be running as root), so a bug in the DNS resolver might lead to a remote code execution. This can be prevented by installing a DNS caching server, such as [dnsmasq](https://wiki.archlinux.org/title/Dnsmasq), which acts as a proxy. [[20]](https://googleonlinesecurity.blogspot.it/2016/02/cve-2015-7547-glibc-getaddrinfo-stack.html)

### Managing TLS certificates

See [TLS#Trust management](https://wiki.archlinux.org/title/TLS#Trust_management).

## Physical security

Physical access to a computer is root access given enough time and resources. However, a high practical level of security can be obtained by putting up enough barriers.

An attacker can gain full control of your computer on the next boot by simply attaching a malicious IEEE 1394 (FireWire), Thunderbolt or PCI Express device as they are given full memory access by default.[[21]](https://web.archive.org/web/20210312083421/http://breaknenter.org/2014/09/inception-metasploit-integration/) For Thunderbolt, you can restrict the direct memory access completely or to known devices, see [user device authorization](https://wiki.archlinux.org/title/Thunderbolt#User_device_authorization). For Firewire and PCI Express, there is little you can do from preventing this, or modification of the hardware itself - such as flashing malicious firmware onto a drive. However, the vast majority of attackers will not be this knowledgeable and determined.

[Data-at-rest encryption](https://wiki.archlinux.org/title/Security#Data-at-rest_encryption) will prevent access to your data if the computer is stolen, but malicious firmware can be installed to obtain this data upon your next log in by a resourceful attacker.

### Locking down BIOS

Adding a password to the BIOS prevents someone from booting into removable media, which is basically the same as having root access to your computer. You should make sure your drive is first in the boot order and disable the other drives from being bootable if you can.

### Boot loaders

It is highly important to protect your [boot loader](https://wiki.archlinux.org/title/Boot_loader). An unprotected boot loader can bypass any login restrictions, e.g. by setting the init=/bin/sh [kernel parameter](https://wiki.archlinux.org/title/Kernel_parameter) to boot directly to a shell.

#### Syslinux

[Syslinux](https://wiki.archlinux.org/title/Syslinux) supports [password-protecting your boot loader](https://wiki.archlinux.org/title/Syslinux#Security). It allows you to set either a per-menu-item password or a global boot loader password.

#### GRUB

[GRUB](https://wiki.archlinux.org/title/GRUB) supports boot loader passwords as well. See [GRUB/Tips and tricks#Password protection of GRUB menu](https://wiki.archlinux.org/title/GRUB/Tips_and_tricks#Password_protection_of_GRUB_menu) for details. It also has support for [encrypted /boot](https://wiki.archlinux.org/title/GRUB#Encrypted_/boot), which only leaves some parts of the boot loader code unencrypted. GRUB's configuration, [kernel](https://wiki.archlinux.org/title/Kernel) and [initramfs](https://wiki.archlinux.org/title/Initramfs) are encrypted.

#### systemd-boot

[systemd-boot](https://wiki.archlinux.org/title/Systemd-boot) disables editing of kernel parameters when [Secure Boot](https://wiki.archlinux.org/title/Security#Secure_Boot) is enabled. Alternatively, you can set [kernel parameters for password protection](https://wiki.archlinux.org/title/Systemd-boot#Kernel_parameters_editor_with_password_protection) in systemd-boot for a more traditional password-based option.

### Secure Boot

[Secure Boot](https://wiki.archlinux.org/title/Secure_Boot) is a feature of [UEFI](https://wiki.archlinux.org/title/UEFI) that allows authentication of the files your computer boots. This helps preventing some [evil maid attacks](https://en.wikipedia.org/wiki/Evil_maid_attack) such as replacing files inside the boot partition. Normally computers come with keys that are enrolled by vendors (OEM). However these can be removed and allow the computer to enter Setup Mode which allows the user to enroll and manage their own keys.

The secure boot page guides you through how to set secure boot up by [using your own keys](https://wiki.archlinux.org/title/Unified_Extensible_Firmware_Interface/Secure_Boot#Using_your_own_keys).

### Trusted Platform Module (TPM)

[TPMs](https://wiki.archlinux.org/title/Trusted_Platform_Module) are hardware microprocessors which have cryptographic keys embedded. This forms the fundamental root of trust of most modern computers and allows end-to-end verification of the boot chain. They can be used as internal smartcards, attest the firmware running on the computer and allow users to insert secrets into a tamper-proof and brute-force resistant store.

### Boot partition on removable flash drive

One popular idea is to place the boot partition on a flash drive in order to render the system unbootable without it. Proponents of this idea often use [full-disk encryption](https://wiki.archlinux.org/title/Security#Data-at-rest_encryption) alongside, and some also use [detached encryption headers](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Encrypted_system_using_a_detached_LUKS_header) placed on the boot partition.

This method can also be merged with [encrypting /boot](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Encrypted_/boot_and_a_detached_LUKS_header_on_USB).

### Automatic logout

If you are using [Bash](https://wiki.archlinux.org/title/Bash) or [Zsh](https://wiki.archlinux.org/title/Zsh), you can set TMOUT for an automatic logout from shells after a timeout.

For example, the following will automatically log out from virtual consoles (but not terminal emulators in X11):

```
/etc/profile.d/shell-timeout.sh
```

```
TMOUT="$(( 60*10 ))";
[ -z "$DISPLAY" ] && export TMOUT;
case $( /usr/bin/tty ) in
	/dev/tty[0-9]*) export TMOUT;;
esac

```

If you really want EVERY Bash/Zsh prompt (even within X) to timeout, use:

```
$ export TMOUT="$(( 60*10 ))";

```

Note that this will not work if there is some command running in the shell (eg.: an SSH session or other shell without TMOUT support). But if you are using VC mostly for restarting frozen GDM/Xorg as root, then this is very useful.

### Protect against rogue USB devices

The kernel has [settings to deactivate](https://docs.kernel.org/usb/authorization.html) USB ports to protect your computer against rogue USB devices (a.k.a. [BadUSB](https://en.wikipedia.org/wiki/BadUSB), [PoisonTap](https://github.com/samyk/poisontap) or [LanTurtle](https://lanturtle.com/)). They can be set at runtime and automated via [sysctl](https://wiki.archlinux.org/title/Sysctl).

For more control install [USBGuard](https://wiki.archlinux.org/title/USBGuard), which is a software framework implementing basic whitelisting and blacklisting capabilities based on device attributes.

### Volatile data collection

A computer that is powered on may be vulnerable to [volatile data collection](https://web.archive.org/web/20210420075636/https://fedvte.usalearning.gov/courses/CSI/course/videos/pdf/CSI_D01_S05_T01_STEP.pdf). It is a best practice to turn a computer completely off at times it is not necessary for it to be on, or if the computer's physical security is temporarily compromised (e.g. when passing through a security checkpoint).

## Packages

### Authentication

[Attacks on package managers](https://www2.cs.arizona.edu/stork/packagemanagersecurity/attacks-on-package-managers.html#overview) are possible without proper use of package signing, and can affect even package managers with [proper signature systems](https://www2.cs.arizona.edu/stork/packagemanagersecurity/faq.html). Arch uses package signing by default and relies on a web of trust from 5 trusted master keys. See [Pacman-key](https://wiki.archlinux.org/title/Pacman-key) for details.

### Upgrades

It is important to regularly [upgrade the system](https://wiki.archlinux.org/title/System_maintenance#Upgrading_the_system).

### Follow vulnerability alerts

Subscribe to the Common Vulnerabilities and Exposure (CVE) Security Alert updates, made available by National Vulnerability Database, and found on the [NVD Download webpage](https://nvd.nist.gov/download.cfm).

You should also consider subscribing to the release notifications for software you use, especially if you install software through means other than the main repositories or AUR. Some software have mailing lists you can subscribe to for security notifications. Source code hosting sites often offer RSS feeds for new releases. See [Arch Security Team#Resources](https://wiki.archlinux.org/title/Arch_Security_Team#Resources).

### Rebuilding packages

Packages can be rebuilt and stripped of undesired functions and features as a means to reduce attack surface. For example, [bzip2](https://archlinux.org/packages/?name=bzip2) can be rebuilt without bzip2recover in an attempt to circumvent [CVE-2016-3189](https://security.archlinux.org/CVE-2016-3189). Custom hardening flags can also be applied either manually or via a wrapper.

This article or section is a candidate for merging with [Arch package guidelines/Security](https://wiki.archlinux.org/title/Arch_package_guidelines/Security).

Notes: Security related build flags have their own article. (Discuss in [Talk:Security](https://wiki.archlinux.org/title/Talk:Security))

The factual accuracy of this article or section is disputed.

Reason: Copy-pasted from a 3 years old blog post. The compiler flags are specific to [GCC](https://wiki.archlinux.org/title/GCC), some are hardly security related. (Discuss in [Talk:Security](https://wiki.archlinux.org/title/Talk:Security))

| Flag  | Purpose

| -D_FORTIFY_SOURCE=2  | Run-time buffer overflow detection

| -D_GLIBCXX_ASSERTIONS  | Run-time bounds checking for C++ strings and containers

| -fasynchronous-unwind-tables  | Increased reliability of backtraces

| -fexceptions  | Enable table-based thread cancellation

| -fpie -Wl,-pie  | Full ASLR for executables

| -fpic -shared  | No text relocations for shared libraries

| -fplugin=annobin  | Generate data for hardening quality control

| -fstack-clash-protection  | Increased reliability of stack overflow detection

| -fstack-protector, -fstack-protector-all or -fstack-protector-strong  | Stack smashing protector

| -grecord-gcc-switches  | Store compiler flags in debugging information

| -mcet -fcf-protection  | Control flow integrity protection

| -Werror=format-security  | Reject potentially unsafe format string arguments

| -Werror=implicit-function-declaration  | Reject missing function prototypes

| -Wl,-z,defs  | Detect and reject underlinking

| -Wl,-z,now  | Disable lazy binding

| -Wl,-z,relro  | Read-only segments after relocation

- [Flags and info source](https://developers.redhat.com/blog/2018/03/21/compiler-and-linker-flags-gcc/)

## See also

- [Arch Linux Security Tracker](https://security.archlinux.org/)
- [CentOS Wiki: OS Protection](https://wiki.centos.org/HowTos/OS_Protection)
- [Hardening the Linux desktop](https://web.archive.org/web/20210712001756/https://developer.ibm.com/technologies/linux/articles/l-harden-desktop/)
- [Hardening the Linux server](https://web.archive.org/web/20190701140035/https://www.ibm.com/developerworks/linux/tutorials/l-harden-server/index.html)
- [Linux Foundation: Linux workstation security checklist](https://github.com/lfit/itpol/blob/master/linux-workstation-security.md)
- [privacyguides.org Privacy Resources](https://www.privacyguides.org/)
- [Red Hat Enterprise Linux 7 Security Guide](https://access.redhat.com/documentation/en-US/Red_Hat_Enterprise_Linux/7/html/Security_Guide/)
- [Securing Debian Manual](https://www.debian.org/doc/manuals/securing-debian-manual/index.en.html)
- [The paranoid #! Security Guide](https://web.archive.org/web/20140220055801/http://crunchbang.org:80/forums/viewtopic.php?id=24722)

Retrieved from "https://wiki.archlinux.org/index.php?title=Security&oldid=888759"

[Categories](https://wiki.archlinux.org/title/Special:Categories):

- [Security](https://wiki.archlinux.org/title/Category:Security)
- [File systems](https://wiki.archlinux.org/title/Category:File_systems)
- [Networking](https://wiki.archlinux.org/title/Category:Networking)

Hidden categories:

- [Pages or sections flagged with Template:Accuracy](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Accuracy)
- [Pages or sections flagged with Template:Expansion](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Expansion)
- [Pages or sections flagged with Template:Style](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Style)
- [Pages or sections flagged with Template:Merge](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Merge)
