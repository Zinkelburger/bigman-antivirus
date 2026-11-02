---
url: "https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/"
title: "Bootkitty: Analyzing the first UEFI bootkit for Linux"
captured: "2026-11-02T19:04:02Z"
via: "live"
sha256: "3acb8508b8689573e0e3690494289300d31c0be1beb26e7373835305063da519"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# Bootkitty: Analyzing the first UEFI bootkit for Linux

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/>

https://www.welivesecurity.com/en/

Cybersecurity research and insight for the AI era · [Moving soon to the ESET Blog](https://www.eset.com/blog/en/)

[Español](https://www.welivesecurity.com/es/)[Deutsch](https://www.welivesecurity.com/de/)[Português](https://www.welivesecurity.com/pt/)[Français](https://www.welivesecurity.com/fr/)

https://www.welivesecurity.com/en/

Cybersecurity research and insight for the AI era · <a href='https://www.eset.com/blog/en/' target='_blank'>Moving soon to the ESET Blog</a>

ESET Research

# Bootkitty: Analyzing the first UEFI bootkit for Linux

ESET researchers analyze the first UEFI bootkit designed for Linux systems

https://www.welivesecurity.com/en/our-experts/martin-smolar/

[Martin Smolár](https://www.welivesecurity.com/en/our-experts/martin-smolar/)

https://www.welivesecurity.com/en/our-experts/peter-strycek/

[Peter Strýček](https://www.welivesecurity.com/en/our-experts/peter-strycek/)

27 Nov 2024  •  ,  14 min. read

UPDATE (December 2nd, 2024): The bootkit described in this report seems to be part of a project created by cybersecurity students participating in Korea's Best of the Best (BoB) training program. As they informed us: "The primary aim of this project is to raise awareness within the security community about potential risks and to encourage proactive measures to prevent similar threats. Unfortunately, few bootkit samples were disclosed prior to the planned conference presentation." This supports our belief that it was an initial proof of concept rather than production-ready malware used by real threat actors. Nonetheless, the blog post remains accurate – it is a functional bootkit with limited support and represents the first UEFI bootkit proof of concept for Linux OS.

Over the past few years, the UEFI threat landscape, particularly that of UEFI bootkits, has evolved significantly. It all started with the first UEFI bootkit proof of concept (PoC) [described](https://www.theregister.com/2012/09/19/win8_rootkit/) by Andrea Allievi in 2012, which served as a demonstration of deploying bootkits on modern UEFI-based Windows systems, and was followed with many other PoCs ([EfiGuard](https://github.com/Mattiwatti/EfiGuard), [Boot Backdoor](https://github.com/Cr4sh/s6_pcie_microblaze/tree/master/python/payloads/DmaBackdoorBoot), [UEFI-bootkit](https://github.com/ajkhoury/UEFI-Bootkit)). It took several years until the first two real UEFI bootkits were discovered in the wild ([ESPecter](https://www.welivesecurity.com/2021/10/05/uefi-threats-moving-esp-introducing-especter-bootkit/), 2021 ESET; [FinSpy bootkit](https://securelist.com/finspy-unseen-findings/104322/), 2021 Kaspersky), and it took two more years until the infamous [BlackLotus](https://www.welivesecurity.com/2023/03/01/blacklotus-uefi-bootkit-myth-confirmed/) – the first UEFI bootkit capable of bypassing UEFI Secure Boot on up-to-date systems – appeared (2023, ESET).

A common thread among these publicly known bootkits was their exclusive targeting of Windows systems. Today, we unveil our latest discovery: the first UEFI bootkit designed for Linux systems, named Bootkitty by its creators. We believe this bootkit is merely an initial proof of concept, and based on our telemetry, it has not been deployed in the wild. That said, its existence underscores an important message: UEFI bootkits are no longer confined to Windows systems alone.

The bootkit’s main goal is to disable the kernel’s signature verification feature and to preload two as yet unknown ELF binaries via the Linux init process (which is the first process executed by the Linux kernel during system startup). During our analysis, we discovered a possibly related unsigned kernel module – with signs suggesting that it could have been developed by the same author(s) as the bootkit – that deploys an ELF binary responsible for loading yet another kernel module unknown during our analysis.

Key points of this blogpost:

- In November 2024, a previously unknown UEFI application, named bootkit.efi, was uploaded to [VirusTotal](https://www.virustotal.com/gui/file/f1f84819bdf395d42c36adb36ded0e7de338e2036e174716b5de71abc56f5d40).
- Our initial analysis confirmed it is a UEFI bootkit, named Bootkitty by its creators and surprisingly the first UEFI bootkit targeting Linux, specifically, a few Ubuntu versions.
- Bootkitty is signed by a self-signed certificate, thus is not capable of running on systems with UEFI Secure Boot enabled unless the attackers certificates have been installed.
- Bootkitty is designed to boot the Linux kernel seamlessly, whether UEFI Secure Boot is enabled or not, as it patches, in memory, the necessary functions responsible for integrity verification before GRUB is executed.
- bootkit.efi contains many artifacts suggesting this is more like a proof of concept than the work of an active threat actor.
- We discovered a possibly related kernel module, which we named BCDropper, that deploys an ELF program responsible for loading another kernel module.

## Bootkitty overview

As mentioned in the introduction, Bootkitty contains many artifacts suggesting that we might be dealing with a proof of concept instead of actively used malware. In this section, we look more closely at these artifacts, plus other basic information about the bootkit.

Bootkitty contains two unused functions, capable of printing special strings to the screen during its execution. The first function, whose output is depicted in Figure 1, can print ASCII art that we believe represents a possible name of the bootkit: Bootkitty.

Figure 1. ASCII art embedded in the bootkit

The second function, can print text, shown in Figure 2, containing the list of possible bootkit authors and other persons that perhaps somehow participated in its development. One of the names mentioned in the image can be found on GitHub, but the profile does not have any public repository that would contain or mention a UEFI bootkit project; therefore, we can neither confirm nor deny authenticity of the names mentioned in the bootkit.

Figure 2. List of names embedded in the bootkit (redacted)

During every boot, Bootkitty prints on screen the strings shown in Figure 3.

Figure 3. Bootkitty’s welcome message

Note that the BlackCat name is referenced also in the loadable kernel module described later. Despite the name, we believe there is no connection to the ALPHV/BlackCat ransomware group. This is because BlackCat is a name used by researchers and Bootkitty was developed in C, while the group calls itself ALPHV and develops its malware exclusively in Rust.

As mentioned earlier, Bootkitty currently supports only a limited number of systems. The reason is that to find the functions it wants to modify in memory, it uses hardcoded byte patterns. While byte-pattern matching is a common technique when it comes to bootkits, the authors didn’t use the best patterns for covering multiple kernel or GRUB versions; therefore, the bootkit is fully functional only for a limited number of configurations. What limits the use of the bootkit even more is the way it patches the decompressed Linux kernel: as shown in Figure 4, once the kernel image is decompressed, Bootkitty simply copies the malicious patches to the hardcoded offsets within the kernel image.

Figure 4. Bootkitty’s code responsible for patching the decompressed kernel before it is executed

We explain how the bootkit gets to the actual kernel patching later in the [Linux kernel image decompression hook](https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/#Linux kernel image decompression hook) section; for now, just note that due to the lack of kernel-version checks in the function shown in Figure 4, Bootkitty can get to the point where it patches completely random code or data at these hardcoded offsets, thus crashing the system instead of compromising it. This is one of the facts that supports proof of concept. On the other hand, it might be an initial not-production-ready version of malware created by malicious threat actors.

Last but not least, the bootkit binary is signed by the self-signed certificate shown in Figure 5.

Figure 5. Self-signed certificate used to sign the bootkit

## Techical analysis

We start with an overview of Bootkitty’s execution, as depicted in Figure 6. First, we briefly describe the main functionality and then in subsequent sections we go into more details.

There are three main parts we focus on:

- Execution of the bootkit and patching of the legitimate GRUB bootloader (points 4 and 5 in Figure 6).
- Patching of the Linux kernel’s EFI stub loader (points 6 and 7 in Figure 6).
- Patching of the decompressed Linux kernel image (points 8 and 9 in Figure 6).

Figure 6. Bootkitty execution overview

### Initialization and GRUB hooking

After Bootkitty is executed by the shim, it checks to see whether UEFI Secure Boot is enabled by examining the value of the SecureBoot UEFI variable, and proceeds to hook two functions from the UEFI authentication protocols if so (this process is shown in Figure 7):

- EFI_SECURITY2_ARCH_PROTOCOL.FileAuthentication: [this function](https://uefi.org/specs/PI/1.8/V2_DXE_Architectural_Protocols.html#efi-security2-arch-protocol-fileauthentication) is used by the firmware to measure and verify the integrity of UEFI PE images. Bootkitty’s hook function modifies the output of this function so that it always returns EFI_SUCCESS, meaning that the verification succeeded.
- EFI_SECURITY_ARCH_PROTOCOL.FileAuthenticationState: [this function](https://uefi.org/specs/PI/1.8/V2_DXE_Architectural_Protocols.html#efi-security-arch-protocol-fileauthenticationstate) is used by the firmware to execute a platform-specific policy in response to different authentication status values. Again, the bootkit’s hook modifies it in a way that it always returns EFI_SUCCESS, meaning that the firmware can use the file regardless of its actual authentication status.

Figure 7. Hooking of the UEFI security authentication protocols

After checking the status of UEFI Secure Boot, Bootkitty proceeds to load the legitimate GRUB from the hardcoded path on the EFI system partition: /EFI/ubuntu/grubx64-real.efi. This file should be a backup, created by the attacker, of a legitimate GRUB. Once GRUB is loaded (not yet executed), the bootkit starts patching and hooking the following code in GRUB’s memory:

- The start_image function within the peimage GRUB module (a module embedded inside GRUB). This function is responsible for starting an already loaded PE image, and it’s invoked by GRUB to start the Linux kernel’s EFI stub binary (known in general as vmlinuz.efi or vmlinuz). The hook function takes advantage of the fact that at the moment the hook is executed, vmlinuz is already loaded into memory (but hasn’t been executed yet), and patches the function responsible for decompressing the actual Linux kernel image inside vmlinuz (note that in some cases, due to the way the Linux kernel is compiled, it can be quite challenging to find the exact name of the function being patched; however, we believe that this time it should be the zstd_decompress_dctx function). More details about the decompression hook are in the [Linux kernel image decompression hook](https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/#Linux kernel image decompression hook) section.
- The shim_lock_verifier_init function, which is part of the shim_lock verifier mechanism inside GRUB – this should be activated automatically if UEFI Secure Boot is enabled. It is responsible for deciding whether the files provided (e.g., GRUB modules, Linux kernel, configurations…) should be verified or not during the boot. The installed hook, however, is somehow confusing and the author’s intentions are unclear because it modifies shim_lock_verifier_init’s output in a way that it sets the output flag to GRUB_VERIFY_FLAGS_SINGLE_CHUNK (value 2) for any file type provided, which should, according to the [GRUB manual](https://www.gnu.org/software/grub/manual/grub-dev/html_node/Verifiers-framework.html), strengthen the security even more. Interestingly, due to the hook described in the next point, this shim_lock_verifier_init function is not even called during the boot, thus becoming irrelevant.
- The grub_verifiers_open function. This function is invoked by GRUB anytime it opens a file, and is responsible for checking whether the installed GRUB file verifiers (this includes the shim_lock verifier described above) require integrity verification for the file being loaded. The function is hooked by the bootkit in a way that it returns immediately without proceeding to any signature checks (note that this means that it does not even execute the previously hooked shim_lock_verifier_init function).

### Linux kernel image decompression hook

This hook is responsible for patching the decompressed Linux kernel image. The hook is called right before the kernel image is decompressed, so the hook restores the original decompression function’s bytes and executes the original function to decompress the kernel image before proceeding to the kernel patching.

Now, as the kernel is decompressed and lies in the memory untouched (still hasn’t been executed), the hook code patches it at hardcoded offsets (in memory only). Specifically, as shown in Figure 8, it:

- Rewrites the kernel version and [Linux banner](https://github.com/torvalds/linux/blob/v6.8/init/version-timestamp.c#L28) strings with the text BoB13 (this has no significant impact on the system).
- Hooks the [module_sig_check](https://github.com/torvalds/linux/blob/v6.8/kernel/module/signing.c#L70) function.
- Patches pointer/address to the first environment variable of the init process.

Figure 8. Bootkitty’s kernel-decompression hook inside vmlinuz

The function module_sig_check is patched to always return 0. This function is responsible for checking whether the module is validly signed. By patching the function to return 0, the kernel will load any module without verifying the signature. On Linux systems with UEFI Secure Boot enabled, kernel modules need to be signed if they are meant to be loaded. This is also the case when the kernel is built with CONFIG_MODULE_SIG_FORCE enabled or when module.sig_enforce=1 is passed as a kernel command line argument, as described in the [Linux kernel documentation](https://www.kernel.org/doc/html/v4.15/admin-guide/module-signing.html#non-valid-signatures-and-unsigned-modules). The likely scenario is that at least one malicious kernel module is loaded at a later phase, such as the dropper analyzed below.

The first process that the Linux kernel executes is init from the first hardcoded path that works (starting with [/init from initramfs](https://github.com/torvalds/linux/blob/v6.8/init/main.c#L1467)), along with command line arguments and environment variables. The hook code replaces the first environment variable with LD_PRELOAD=/opt/injector.so /init. [LD_PRELOAD](https://man7.org/linux/man-pages/man8/ld.so.8.html) is an environment variable that is used to load ELF shared objects before others and can be used to override functions. It is a common [technique](https://attack.mitre.org/techniques/T1574/006/) used by attackers to load malicious binaries. In this case, the /opt/injector.so and /init ELF shared objects are loaded when the init process starts. This is where the intention becomes less clear, mainly why the second string /init is part of LD_PRELOAD.

We have not discovered any of these possibly malicious ELF shared objects, although just as this blogpost was being finalized for publication, a write-up describing the missing components mentioned in our report has been [published](https://humzak711.github.io/analyzing_IranuKit). Now it’s clear they are used just to load another stage.

### Impact and remediation

Apart from loading unknown ELF shared objects, Bootkitty leaves footprints in the system. The first is the intended, albeit not necessary, modification of kernel version and Linux banner strings. The former can be seen by running uname -v (Figure 9) and the latter by running dmesg (Figure 10).

Figure 9. BoB13 string in uname output

Figure 10. BoB13 string in dmesg output

During our analysis, the output of the command dmesg also included details about how the init process was run. As depicted in Figure 11, the process was run with the LD_PRELOAD environment variable (it was originally HOME=/ and was replaced with LD_PRELOAD=/opt/injector.so /init by the bootkit).

Figure 11. init process arguments and environment variables in dmesg output

Note in Figure 11 that the word /init in the first line corresponds to the legitimate program in initramfs that eventually passes control to systemd on default Ubuntu installations. The presence of the LD_PRELOAD environment variable can also be verified by inspecting the file /proc/1/environ.

After booting up a system with Bootkitty in our testing environment, we noticed that the kernel was marked as [tainted](https://docs.kernel.org/admin-guide/tainted-kernels.html) (command from Figure 12 can be used to check the tainted value), which was not the case when the bootkit was absent. Another way to tell whether the bootkit is present on the system with UEFI Secure Boot enabled is by attempting to load an unsigned dummy kernel module during runtime. If it’s present, the module will be loaded; if not – the kernel refuses to load it.

Figure 12. Tainted state right after the system has started with Bootkitty

A simple remedy tip to get rid of the bootkit is to move the legitimate /EFI/ubuntu/grubx64-real.efi file back to its original location, which is /EFI/ubuntu/grubx64.efi. This will make shim execute the legitimate GRUB and thus the system will boot up without the bootkit (note that this covers only the scenario when the bootkit is deployed as /EFI/ubuntu/grubx64.efi).

### BCDropper and BCObserver

In addition to the bootkit, we discovered a possibly related unsigned kernel module we named BCDropper, uploaded to VirusTotal around the same time and by the same submitter’s ID as the bootkit, containing hints that it might have been developed by the same author as the bootkit, such as:

- a BlackCat string in the output of the modinfo command’s output, shown in Figure 13,
- another presence of the blackcat string in the debug paths in the module’s binary, shown in Figure 14, and
- it contains an unused file-hiding function that hides specific entries from directory listings. As shown in Figure 15, one of the hardcoded filename string prefixes used to filter-out these entries is injector (note that Bootkitty tries to preload a shared-library from the path /opt/injector.so)

However, even with the evidence presented, we cannot say for sure whether or not the kernel module is related to Bootkitty (or was created by the same developer). Also, the kernel version mentioned in Figure 13 (6.8.0-48-generic) is not supported by the bootkit.

Figure 13. Dropper module information

Figure 14. Dropper debug symbols referencing blackcat

Figure 15. List of files, in the dropper, to hide

As its name suggests, the kernel module drops an embedded ELF file we named BCObserver, specifically to /opt/observer, and executes it via /bin/bash (Figure 17). On top of that, the module hides itself by [removing its entry from the module list](https://xcellerator.github.io/posts/linux_rootkits_05/). The kernel module also implements other rootkit-related functionalities like hiding files (those in Figure 15), processes, and open ports, but they are not directly used by the dropper.

Figure 16. Hex-Rays decompiled dropper code

BCObserver is a rather simple application that waits until the display manager [gdm3](https://manpages.ubuntu.com/manpages/lunar/man8/gdm3.8.html) is running, and then loads an unknown kernel module from /opt/rootkit_loader.ko via the [finit_module](https://man7.org/linux/man-pages/man2/init_module.2.html) system call. By waiting for the display manager to start, the code ensures that the kernel module is loaded after the system is fully booted up.

Figure 17. Hex-Rays decompiled observer code

While we cannot confirm whether the dropper is somehow related to the bootkit, and if so, how it is meant to be executed, we’re quite sure that the bootkit patches the module_sig_check function for a reason, and loading an unsigned kernel module (such as the dropper described here) would definitely make sense.

## Conclusion

Whether a proof of concept or not, Bootkitty marks an interesting move forward in the UEFI threat landscape, breaking the belief about modern UEFI bootkits being Windows-exclusive threats. Even though the current version from VirusTotal does not, at the moment, represent a real threat to the majority of Linux systems, it emphasizes the necessity of being prepared for potential future threats.

To keep your Linux systems safe from such threats, make sure that UEFI Secure Boot is enabled, your system firmware and OS are up-to-date, and so is your UEFI revocations list.

For any inquiries about our research published on WeLiveSecurity, please contact us at threatintel@eset.com.

ESET Research offers private APT intelligence reports and data feeds. For any inquiries about this service, visit the [ESET Threat Intelligence](https://www.eset.com/int/business/services/threat-intelligence/?utm_source=welivesecurity.com&utm_medium=referral&utm_campaign=wls-research&utm_content=bootkitty-analyzing-first-uefi-bootkit-linux&sfdccampaignid=7011n0000017htTAAQ) page.

## IoCs

A comprehensive list of indicators of compromise (IoCs) and samples can be found in [our GitHub repository](https://github.com/eset/malware-ioc/tree/master/bootkitty).

### Files

| SHA-1  | Filename  | Detection  | Description

| 35ADF3AED60440DA7B80F3C452047079E54364C1  | bootkit.efi  | EFI/Agent.A  | Bootkitty UEFI bootkit.

| BDDF2A7B3152942D3A829E63C03C7427F038B86D  | dropper.ko  | Linux/Rootkit.Agent.FM  | BCDropper.

| E8AF4ED17F293665136E17612D856FA62F96702D  | observer  | Linux/Rootkit.Agent.FM  | BCObserver.

## MITRE ATT&CK techniques

This table was built using [version 16](https://attack.mitre.org/resources/versions/) of the MITRE ATT&CK framework.

| Tactic  | ID  | Name  | Description

| Resource Development  | [T1587.001](https://attack.mitre.org/versions/v16/techniques/T1587/001)  | Develop Capabilities: Malware  | Bootkitty is a brand-new UEFI bootkit developed by an unknown author.

| [T1587.002](https://attack.mitre.org/versions/v16/techniques/T1587/002)  | Develop Capabilities: Code Signing Certificates  | Bootkitty sample is signed with a self-signed certificate.

| Execution  | [T1106](https://attack.mitre.org/versions/v16/techniques/T1106)  | Native API  | BCObserver uses the finit_module system call to load a kernel module.

| [T1129](https://attack.mitre.org/versions/v16/techniques/T1129)  | Shared Modules  | Bootkitty uses LD_PRELOAD to preload shared modules from a hardcoded path into the init process during system start.

| Persistence  | [T1574.006](https://attack.mitre.org/versions/v16/techniques/T1574/006)  | Hijack Execution Flow: Dynamic Linker Hijacking  | Bootkitty patches init’s environment variable with LD_PRELOAD so it loads a next stage when executed.

| [T1542.003](https://attack.mitre.org/versions/v16/techniques/T1542/003)  | Pre-OS Boot: Bootkit  | Bootkitty is a UEFI bootkit meant to be deployed on the EFI System Partition.

| Defense Evasion  | [T1014](https://attack.mitre.org/versions/v16/techniques/T1014)  | Rootkit  | BCDropper serves as a rootkit implemented as a loadable kernel module for Linux systems.

| [T1562](https://attack.mitre.org/versions/v16/techniques/T1562/)  | Impair Defenses  | Bootkitty disables signature verification features in the GRUB bootloader and Linux kernel.

| [T1564](https://attack.mitre.org/versions/v16/techniques/T1564)  | Hide Artifacts  | BCDropper hides itself by removing its module’s entry from the kernel’s modules list.

https://www.eset.com/int/business/services/threat-intelligence/?utm_source=welivesecurity.com&utm_medium=referral&utm_campaign=wls-research&utm_content=bootkitty-analyzing-first-uefi-bootkit-linux&sfdccampaignid=7011n0000017htTAAQ

##  Let us keep you
up to date

Sign up for our newsletters

####  Related Articles

[ESET Research

ESET APT Activity Report Q2 2026–Q3 2026

ESET Research

ESET APT Activity Report Q2 2026–Q3 2026](https://www.welivesecurity.com/en/eset-research/eset-apt-activity-report-q2-2026-q3-2026/)

[ESET Research

MythicRatatoskr: An Yggdrasil-enabled backdoor deployed in a UAC-0099-Sandworm collaboration

ESET Research

MythicRatatoskr: An Yggdrasil-enabled backdoor deployed in a UAC-0099-Sandworm collaboration](https://www.welivesecurity.com/en/eset-research/mythicratatoskr-yggdrasil-enabled-backdoor-deployed-uac0099-sandworm-collaboration/)

[ESET Research

MATCHBOIL: New tricks, same old evil intentions

ESET Research

MATCHBOIL: New tricks, same old evil intentions](https://www.welivesecurity.com/en/eset-research/matchboil-new-tricks-same-old-evil-intentions/)

https://www.eset.com/us/business/services/threat-intelligence/

###  Similar Articles

[ESET research

LoJax: First UEFI rootkit found in the wild, courtesy of the Sednit group](https://www.welivesecurity.com/2018/09/27/lojax-first-uefi-rootkit-found-wild-courtesy-sednit-group/)

[ESET research

UEFI threats moving to the ESP: Introducing ESPecter bootkit](https://www.welivesecurity.com/2021/10/05/uefi-threats-moving-esp-introducing-especter-bootkit/)

[ESET research

BlackLotus UEFI bootkit: Myth confirmed](https://www.welivesecurity.com/2023/03/01/blacklotus-uefi-bootkit-myth-confirmed/)

### Share Article

https://www.facebook.com/sharer/sharer.php?u=https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/ https://www.linkedin.com/shareArticle?mini=true&url=https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/ https://twitter.com/intent/tweet?url=https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/  https://www.welivesecurity.com/en/eset-research/bootkitty-analyzing-first-uefi-bootkit-linux/

### Discussion
