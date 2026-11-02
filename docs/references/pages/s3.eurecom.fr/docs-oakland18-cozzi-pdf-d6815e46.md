---
url: "https://www.s3.eurecom.fr/docs/oakland18_cozzi.pdf"
title: "Understanding Linux Malware"
captured: "2026-11-02T18:58:29Z"
via: "live"
sha256: "0b5d36f80df20ae54cf1f3ba0ce7509383bb83e80a2020955d572cc04a0a3c94"
wayback: "https://web.archive.org/web/20260804180239/https://www.s3.eurecom.fr/docs/oakland18_cozzi.pdf"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
  - "docs/threats/05-malware-families-and-detection.md"
---

# Understanding Linux Malware

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.s3.eurecom.fr/docs/oakland18_cozzi.pdf>
> Snapshot: <https://web.archive.org/web/20260804180239/https://www.s3.eurecom.fr/docs/oakland18_cozzi.pdf>

                            Understanding Linux Malware
          Emanuele Cozzi               Mariano Graziano                Yanick Fratantonio              Davide Balzarotti
               Eurecom                  Cisco Systems, Inc.                  Eurecom                        Eurecom




   Abstract—For the past two decades, the security community         on a short time-to-market combined with innovative features
has been fighting malicious programs for Windows-based operat-       to attract new users. Too often, this results in postponing
ing systems. However, the recent surge in adoption of embedded       (if not simply ignoring) any security and privacy concerns.
devices and the IoT revolution are rapidly changing the malware
landscape. Embedded devices are profoundly different than tradi-     With these premises, it does not come as a surprise that
tional personal computers. In fact, while personal computers run     the vast majority of these newly interconnected devices are
predominantly on x86-flavored architectures, embedded systems        routinely found vulnerable to critical security issues, ranging
rely on a variety of different architectures. In turn, this aspect   from Internet-facing insecure logins (e.g., easy-to-guess hard-
causes a large number of these systems to run some variants          coded passwords, exposed telnet services, or accessible debug
of the Linux operating system, pushing malicious actors to give
birth to “Linux malware.”                                            interfaces), to unsafe default configurations and unpatched
   To the best of our knowledge, there is currently no comprehen-    software containing well-known security vulnerabilities.
sive study attempting to characterize, analyze, and understand          Embedded devices are profoundly different from traditional
Linux malware. The majority of resources on the topic are
available as sparse reports often published as blog posts, while
                                                                     personal computers. For example, while personal computers
the few systematic studies focused on the analysis of specific       run predominantly on x86 architectures, embedded devices are
families of malware (e.g., the Mirai botnet) mainly by looking       built upon a variety of other CPU architectures—and often
at their network-level behavior, thus leaving the main challenges    on hardware with limited resources. To support these new
of analyzing Linux malware unaddressed.                              systems, developers often adopt Unix-like operating systems,
   This work constitutes the first step towards filling this gap.
After a systematic exploration of the challenges involved in
                                                                     with different flavors of Linux quickly gaining popularity in
the process, we present the design and implementation details        this sector.
of the first malware analysis pipeline specifically tailored for        Not surprisingly, the astonishing number of poorly secured
Linux malware. We then present the results of the first large-       devices that are now connected to the Internet has recently
scale measurement study conducted on 10,548 malware samples
(collected over a time frame of one year) documenting detailed       attracted the attention of malware authors. However, with the
statistics and insights that can help directing future work in the   exception of few anecdotal proof-of-concept examples, the an-
area.                                                                tivirus industry had largely ignored malicious Linux programs,
                                                                     and it is only by the end of 2014 that VirusTotal recognized
                      I. I NTRODUCTION
                                                                     this as a growing concern for the security community [2].
   The security community has been fighting malware for              Academia was even slower to react to this change, and to date
over two decades. However, despite the significant effort            it has not given much attention to this emerging threat. In the
dedicated to this problem by both the academic and indus-            meantime, available resources are often limited to blog posts
try communities, the automated analysis and detection of             (such as the excellent Malware Must Die [3]) that present the,
malicious software remains an open problem. Historically,            often manually performed, analysis of specific samples. One
the vast majority of malware was designed to target almost           of the few systematic works in this area is a recent study by
exclusively personal computers running Microsoft’s Windows           Antonakakis et al. [4] that focuses on the network behavior
operating system, mainly because of its very large market            of a specific malware family (the Mirai botnet). However,
share (currently estimated at 83% [1] for desktop computers).        no comprehensive study has been conducted to characterize,
Therefore, the security community has also been focusing             analyze, and understand the characteristics of Linux-based
its effort on Windows-based malware—resulting in several             malware.
hundreds of papers and a vast knowledge base on how to
detect, analyze, and defend from different classes of malicious         This work aims at filling this gap by presenting the first
programs.                                                            large-scale empirical study conducted to characterize and un-
   However, the recent exponential growth in popularity of           derstand Linux-based malware (for both embedded devices
embedded devices is causing the malware landscape to rapidly         and traditional personal computers). We first systematically
change. Embedded devices have been in use in industrial              enumerate the challenges that arise when collecting and ana-
environments for many years, but it is only recently that they       lyzing Linux samples. For example, we show how supporting
started to permeate every aspect of our society, mainly (but         malware analysis for “common” architectures such as x86 and
not only) driven by the so-called “Internet of Things” (IoT)         ARM is often insufficient, and we explore several challenges
revolution. Companies producing these devices are in a con-          including the analysis of statically linked binaries, the prepa-
stant race to increase their market share, thus focusing mainly      ration of a suitable execution environment, and the differential
analysis of samples run with different privileges. We also detail    they were supposed to run) it is very hard to properly configure
Linux-specific techniques that are used to implement different       the right execution environment.
aspects traditionally associated with malicious software, such       Computer Architectures. Linux is known to support tens
as anti-analysis tricks, packing and polymorphism, evasion,          of different architectures. This requires analysts to prepare
and attempts to gain persistence on the infected machine. These      different analysis sandboxes and port the different architecture-
insights were uncovered thanks to an analysis pipeline we            specific analysis components to support each of them. In a re-
specifically designed to analyze Linux-based malware and the         cent work covering the Mirai botnet [4], the authors supported
experiments we conducted with over 10K malicious samples.            three architectures: MIPS 32-bit, ARM 32-bit, and x86 32-bit.
Our results show that Linux malware is already a multi-faced         However, this covers a small fraction of the overall malware
problem. While still not as complex as its Windows coun-             landscape for Linux. For instance, these three architectures
terpart, we were able to identify many interesting behaviors—        together only cover about 32% of our dataset. Moreover, some
including the ability of certain samples to properly run in mul-     families (such as ARM) are particularly challenging to support
tiple operating systems, the use of privilege escalation exploits,   because of the large number of different CPU architectures they
or the custom modification of the UPX packer adopted to              contain.
protect their code. We also found that a considerable fraction of
Linux malware interacts with other shell utilities and, despite      Loaders and Libraries. The ELF file format allows a Linux
the lack of available malware analysis sandboxes, that some          program to specify an arbitrary loader, which is responsible
samples already implement a wide range of VM-detections              to load and prepare the executable in memory. Unfortunately,
approaches. Finally, we also performed a differential analysis       a copy of the requested loader may not be present in the
to study how the malware behavior changes when the same              analysis environment, thus preventing the sample from starting
sample is executed with or without root privileges.                  its execution. Moreover, dynamically linked binaries expect
                                                                     their required libraries to be available in the target system:
In summary, this paper brings the following contributions:
                                                                     once again, it is enough for a single library to be missing
   • We document the design and implementation of several            to prevent the successful execution of the program. Contrary
     tools we designed to support the analysis of Linux mal-         to what one would expect, in the context of this work these
     ware and we discuss the challenges involved when dealing        aspects affect a significant portion of our dataset. A common
     with this particular type of malicious files.                   example are Linux programs that are dynamically linked with
   • We present the first large-scale empirical study conducted      uClibc or musl, smaller and more performant alternatives to the
     on 10,548 Linux malware samples obtained over a period          traditional glibc. Not only does an analysis environment need
     of one year.                                                    to have these alternatives installed, but their corresponding
   • We uncover and discuss a number of low-level Linux-             loaders are also required.
     specific techniques employed by real-world malware and
     we provide detailed statistics on the current usage.            Operating System. This work focuses on Linux binaries.
                                                                     However, and quite unexpectedly, it can be challenging to
   We make the raw results of all our analyzed samples
                                                                     discern ELF programs compiled for Linux from other ELF-
available to the research community and we provide our entire
                                                                     compatible operating systems, such as FreeBSD or Android.
infrastructure as a free service to other researchers.
                                                                     The ELF headers include an “OS/ABI” field that, in principle,
                       II. C HALLENGES                               should specify which operating system is required for the
                                                                     program to run. In practice, this is rarely informative. For
  The analysis of generic (and potentially malicious) Linux          example, ELF binaries for both Linux and Android specify a
programs requires tackling a number of specific challenges.          generic “System V” OS/ABI. Moreover, current Linux kernels
This section presents a systematic exploration of the main           seem to ignore this field, and it is possible for a binary
problems we encountered in our study.                                that specifies “FreeBSD” as its OS/ABI to be a valid Linux
                                                                     program, a trick that was abused by one of the malware sample
A. Target Diversity                                                  we encountered in our experiments. Finally, while a binary
   The first problem relates to the broad diversity of the           compiled for FreeBSD can be properly loaded and executed
possible target environments. The general belief is that the         under Linux, this is only the case for dynamically linked pro-
main challenge is about supporting different architectures (e.g.,    grams. In fact, the syscalls numbers and arguments for Linux
ARM or MIPS), but this is in fact only one aspect of a               and FreeBSD do not generally match, and therefore statically
much more complex problem. Malware analysis systems for              linked programs usually crash when they encounter such a
Windows, MacOS, or Android executables can rely on de-               difference. These differences may also exist between different
tailed information about the underlying execution environment.       versions of the Linux kernel, and custom modifications are
Linux-based malware can instead target a very diverse set of         not too rare in the world of embedded devices. This has two
targets, such as Internet routers, printers, surveillance cameras,   important consequences for our work: On the one hand, it
smart TVs, or medical devices. This greatly complicates their        makes it hard to compile a dataset of Linux-based malware.
analysis. In fact, without the proper information about the          On the other hand, this also results in the fact that even well-
target (unfortunately, program binaries do not specify where         formed Linux binaries may not be guaranteed to run correctly
in a generic Linux system.                                          the few works looking at Linux-based malware focused only on
                                                                    botnets, thus using honeypots to build a representative dataset.
B. Static Linking
                                                                    Unfortunately, this approach would bias our study towards
   When a binary is statically linked, all its library dependen-    those samples that propagate themselves on random targets.
cies are included in the resulting binary as part of the com-
pilation process. Static linking can offer several advantages,                    III. A NALYSIS I NFRASTRUCTURE
including making the resulting binary more portable (as it is          The task of designing and implementing an analysis infras-
going to execute correctly even when its dependencies are not       tructure for Linux-based malware was complicated by the fact
installed in the target environment) and making it harder to        that when we started our experiments we still knew very little
reverse engineer (as it is difficult to identify which library      about how Linux malware worked and of which techniques
functions are used by the binary).                                  and components we would have needed to study its behavior.
   Static linking introduces also another, much less obvious        For instance, we did not know a priori any of the challenges
challenge for malware analysis. In fact, since these binaries       we discussed in the previous section and we often had wrong
include all their libraries, the resulting application does not     expectations about the prevalence of certain characteristics
rely on any external wrapper to execute system calls. Normal        (such as static linking or malformed file headers) or their
programs do not call system calls directly, but invoke instead      impact on our analysis strategy.
higher level API functions (typically part of the libc) that           Despite our extensive experience in analyzing malicious
in turn wrap the communication with the kernel. Statically          files for Windows and Android, we only had an anecdo-
linked binaries are more portable from a library dependency         tal knowledge of Linux-based malware that we obtained by
point of view, but less portable as they may crash at runtime if    reading online reports describing manual analysis of specific
the kernel ABI is different from what they expected (and what       families. Therefore, the design and implementation of an
was provided by the—unfortunately unknown—target system).           analysis pipeline became a trial-and-error process that we
C. Analysis Environment                                             tackled by following an incremental approach. Each analysis
   An ideal analysis sandbox should emulate as closely as           task was implemented as an independent component, which
possible the system in which the sample under analysis was          was integrated in an interactive framework responsible to
supposed to run. So far we have discussed challenges related        distribute the jobs execution among multiple parallel workers
to setting up an environment with the correct architecture,         and to provide a rich interface for human analysts to inspect
libraries, and operating system, but these only cover part          and visualize the data. As more samples were added to our
of the environment setup. Another important aspect is the           analysis environment every day, the system identified and
privileges the program should run with. Typically, malware          reported any anomaly in the results or any problem that was
analysis sandboxes execute samples as a normal, unprivileged        encountered in the execution of existing modules (such as
user. Administration privileges would give the malware the          new and unsupported architectures, errors that prevented a
ability to tamper with the sandbox itself and would make the        sample from being correctly executed in our sandboxes, or
instrumentation and observation of the program behavior much        unexpected crashes in the adopted tools). Whenever a certain
more complex. Moreover, it is very uncommon for a Windows           issue became widespread enough to impact the successful
sample to expect super-user privileges to work.                     analysis of a considerable number of samples, we introduced
   Unfortunately, Linux malware is often written with the           new analysis modules and designed new techniques to address
assumption (true for some classes of embedded targets) that         the problem. Our framework was also designed to keep track
its code would run with root privileges. However, since these       of which version of each module was responsible for the
details are rarely available to the analyst, it is difficult to     extraction of any given piece of information, thus allowing
identify these samples in advance. We will discuss how we           us to dynamically update and improve each analysis routine
deal with this problem by performing a differential analysis in     without the need to re-start each time the experiments from
Section III.                                                        scratch.
                                                                       Our final analysis pipeline included a collection of exist-
D. Lack of Previous Studies                                         ing state-of-the-art solutions (such as AVClass [5], IDA Pro,
   To the best of our knowledge, this is the first work that        radare2 [6], and Nucleus [7]) as well as completely new tools
attempts to perform a comprehensive analysis of the Linux           we explicitly designed for this paper. Due to space limitations
malware landscape. This mere fact introduces several addi-          we cannot present each component in details. Instead, in
tional challenges. First, it is not clear how to design and         the rest of this section we briefly summarize some of the
implement an analysis pipeline specifically tailored for Linux      techniques we used in our experiments, organized in three
malware. In fact, analysis tools are tailored to the characteris-   different groups: File and Metadata Analysis, Static Analysis,
tics of the existing malware samples. Unfortunately, the lack of    and Dynamic Analysis components.
information on how Linux-based malware works complicated
the design of our pipeline. Which aspects should we focus on?       A. Data Collection
Which architectures do we need to support? A second problem            To retrieve data for our study we used the VirusTotal
in this domain is the lack of a comprehensive dataset. One of       intelligence API to fetch the reports of every ELF file submitted
                File & Metadata Analysis               Static Analysis                          Dynamic Analysis

                                                           Code Analysis                                Packer
                                  AVClass
                                                                                                        Analysis
                     File
                  Recognition
                                                              Packing                      Sandbox                   Trace
                                ELF Anomaly                 Identification                              Emulation
                                                                                          Preparation               Analysis




                                                  Fig. 1. Overview of our analysis pipeline.



between November 2016 and November 2017. Based on the                        number of custom IDA Pro scripts to extract several code
content of the reports, we downloaded 200 candidate samples                  metrics—including the number of functions, their size and
per day. Our selection criteria were designed to minimize non-               cyclomatic complexity, their overall coverage (i.e., the fractions
Linux binaries and to select at least one sample for each family             of the .text section and PT_LOAD segments covered by the
observed during the day. We also split our selection in two                  recognized functions), the presence of overlapping instructions
groups: 140 samples taken from those with more than five AV                  and other assembly tricks, the direct invocation of system
positive matches, and 60 samples with an AV score between                    calls, and the number of direct/indirect branch instructions. In
one and five.                                                                this phase we also computed aggregated metrics, such as the
                                                                             distribution of opcodes, or a rolling entropy of the different
B. File & Metadata Analysis
                                                                             code and data sections. This information is used for statistical
   The first phase of our analysis focuses on the file itself.               purposes, but also integrated in other analysis components, for
Certain fields contained in the ELF file format are required                 instance to identify anti-analysis behaviors or packed samples.
at runtime by the operating system, and therefore need to                       The second task of the static analysis phase consists of
provide reliable information about the architecture on which                 combining the information extracted so far from the ELF
the application is supposed to run and the type of code                      headers and the binary code analysis to identify likely packed
(e.g., executable or shared object) contained in the file. We                applications (see Section V-E for more details). Binaries that
implemented our custom parser for the ELF format because                     could be statically unpacked (e.g., in the common case of UPX)
the existing ones (as explained in Section V-A) were often                   were processed at this stage and the result fed back to be
unable to cope with malformed fields, unexpected values, or                  statically analyzed again. Samples that we could not unpack
missing information.                                                         statically were marked in the database for a subsequent more
   We use the data extracted from each file for two purposes.                fine-grained dynamic attempt.
First, to filter out files that were not relevant for our analysis.
For instance, shared libraries, core dumps, corrupted files, or              D. Dynamic Analysis
executables designed for other operating systems (e.g., when
a sample imported an Android library). Second, we use the                       We performed two types of dynamic analysis in our study:
information to identify any anomalous file structure that, while             a five-minute execution inside an instrumented emulator, and
not preventing the sample to run, could still be used as anti-               a custom packing analysis and unpacking attempt. For the
analysis routine and prevent existing tools to correctly process             emulation, we implemented two types of dynamic sandboxes:
the file (see Section V-A for more details about our findings).              a KVM-based virtualized sandbox with hardware support for
   Finally, as part of this first phase of our pipeline, we also             x86 and x86-64 architectures, and a set of QEMU-based
extract from the VirusTotal reports the AV labels for each                   emulated sandboxes for ARM 32-bit little-endian, MIPS 32-
sample and fed them to the AVClass tool to obtain a normalized               bit big-endian, and PowerPC 32-bit. These five sandboxes
name for the malware family. AVClass, recently proposed by                   were nested inside an outer VM dedicated to dispatch each
Sebastián et al. [5], implements a state-of-the-art technique to            sample depending on its architecture. Our system also main-
normalize, remove generic tokens, and detect aliases among                   tained several snapshots of all VMs, each corresponding to a
a set of AV labels assigned to a malware sample. Therefore,                  different configurations to choose from (e.g., execution under
whenever it is able to output a name, it means that there was                user or root accounts and glibc or uClibc setup). All VMs
a general consensus among different antivirus on the class                   were equipped with additional libraries, the list of which was
(family) the malware belongs to.                                             collected during the static analysis phase, as well as popular
                                                                             loaders (such as the uClibc commonly used in embedded
C. Static Analysis                                                           systems).
  Our static analysis phase includes two tasks: binary code                     For the instrumentation we relied on SystemTap [8] to
analysis and packing detection. The first task relied on a                   implement kernel probes (kprobes) and user probes (uprobes).
While, according to its documentation, SystemTap should be                                       TABLE I
supported on a variety of different architectures (such as x86,         D ISTRIBUTION OF THE 10,548 DOWNLOADED SAMPLES ACROSS
                                                                                              ARCHITECTURES
x86-64, ARM, aarch64, MIPS, and PowerPC), in practice we
needed to patch its code to support ARM and MIPS with                          Architecture            Samples    Percentage
o32 ABI. Our patches include fixes on syscall numbers, CPU
                                                                               X86-64                   3018       28.61%
registers naming and offsets, and the routines required to                     MIPS I                   2120       20.10%
extract the syscall arguments from the stack. We designed our                  PowerPC                  1569       14.87%
SystemTap probes to collect every system call, along with its                  Motorola 68000           1216       11.53%
                                                                               Sparc                    1170       11.09%
arguments and return value, and the instruction pointer from                   Intel 80386              720         6.83%
which the syscall was invoked. We also recompiled the glibc                    ARM 32-bit               555         5.26%
to add uprobes designed to collect, when possible, additional                  Hitachi SH               130         1.23%
                                                                               AArch64 (ARM 64-bit)      47         0.45%
information on string and memory manipulation functions.                       others                    3         0.03%

   At the end of the execution, each sandbox returns a text
file containing the full trace of system calls and userspace
functions. This trace is then immediately parsed to identify                                  IV. DATASET
useful feedback information for the sandbox. For example, this         Our final dataset, after the filtering stage, consisted of 10,548
preliminary analysis can identify missing components (such as       ELF executables, covering more than ten different architectures
libraries and loaders) or detect if a sample tested its user per-   (see Table I for a breakdown of the collected samples). Note
missions or attempted to perform an action that failed because      again how the distribution differs from other datasets collected
of insufficient permissions. In this case, our system would         only by using honeypots: x86, ARM 32-bit, and MIPS 32-bit
immediately repeat the execution of the sample, this time           covered 75% of the data used by Antonakakis et al. [4] on the
with root privileges. As explained in Section V-D, we later         Mirai botnet, but only account for 32% of our samples.
compare the two traces collected with different users as part of       We report detailed statistics about the distribution of samples
our differential analysis to identify how the sample behavior       in our dataset in Appendix. Here we just want to focus on
was affected by the privilege level. Finally, the preliminary       their broad variety and on the large differences that exist
trace analysis can also report to the analyst any error that        among all features we extracted in our database. For example,
prevented the sample to run in our system. As an example of         our set of Linux-based malware vary considerably in size,
these warnings, we encountered a number of ARM samples              from a minimum of 134 bytes (a simple backdoor) to a
that crashed because of a four-byte misalignment between            maximum of 14.8 megabytes (a botnet coded in Go). IDA
the physical and virtual address of their LOAD segments.            Pro was able to recognize (in dynamically linked binaries)
These samples were probably designed to infect an ARM-              from a minimum of zero (in two samples) to a maximum of
based system whose kernel would memory map segments by              5685 unique functions. Moreover, we extracted from the ELF
considering their physical address, something that does not         header of dynamically linked malware the symbols imported
happen in common desktop Linux distributions. We extended           from external libraries—which can give an idea of the most
our system with a component designed to identify these cases        commonly used functionalities. Most samples import between
by looking at the ELF headers and fix the data alignment before     10 and 100 symbols. Interestingly, there are more than 10% of
passing them to the dynamic analysis stage.                         the samples that use malloc but never use free. And while
   To avoid hindering the execution and miss important code         socket is one of the most common functions (confirming the
paths, we gave samples partial network access, while mon-           importance that interconnected devices have nowadays) less
itoring the traffic for signs of abuse. Although not an ideal       than 50% of the binaries requests file-based routines (such as
solution, a similar approach has been previously adopted in         fopen). Finally, entropy plays an important role to identify
other behavioral analysis experiments [4], [9] as it is the only    potential packers or encrypted binary blobs. The vast majority
way to observe the full behavior of a sample.                       of the binaries in our dataset has entropy around six, a common
   Our system also record PCAP files of the network traffic,        value for compiled but not packed code. However, one sample
due to space limitations we will not discuss their analysis         had entropy of only 0.98, due to large blocks of null bytes
as this is the only aspect of Linux-based malware that was          inserted in the data segment.
already partially studied in previous works [4]. Finally, to
dynamically unpack unknown UPX variants we developed a              A. Malware Families
tool based on Unicorn [10]. The system emulates instructions           The AVClass tool was able to associate a family (108 in
on multiple architectures and behaves like a tiny kernel that       total) to 83% of the samples in our dataset. As expected, bot-
exports the limited set of system calls used by UPX during          nets, often dedicated to run DDoS attacks, dominate the Linux-
unpacking (supporting a combination of different system call        based malware landscape—accounting for 69% of our samples
tables and system call ABIs). As we explain in Section V-E,         spread over more than 25 families. One of the reasons for this
this approach allowed us to automatically unpack all but three      prevalence is that attackers often harvest poorly protected IoT
malware samples in our dataset.                                     devices to join large remotely controlled botnets. This task is
                             TABLE II                                                             TABLE III
                     ELF H EADER M ANIPULATION                            ELF SAMPLES THAT CANNOT BE PROPERLY PARSED BY KNOWN TOOLS

  Technique                                       Samples   Percentage              Program          Errors on Malformed Samples
 Segment header table pointing beyond file data      1        0.01%                readelf 2.26.1             166 / 211
 Overlapping ELF header/segment                      2        0.02%                GDB 7.11.1                 157 / 211
 Wrong string table index (e_shstrndx)               60       0.57%                pyelftools 0.24            107 / 211
 Section header table pointing beyond file data     178       1.69%                IDA Pro 7                   - / 211
 Total Corrupted                                    211       2.00%


                                                                         A. ELF headers Manipulation
greatly simplified by the availability of online services like              The Executable and Linkable Format (ELF) is the standard
Shodan [11] or scanning tools like ZMap [12] that can be                 format used to store (among others) all Linux executables. The
used to quickly locate possible targets. Moreover, while it may          format has a complex internal layout, and tampering with some
be difficult to monetize the compromise of small embedded                of its fields and structures provides attackers a first line of
devices that do not contain any relevant data, it is still easy to       defense against analysis tools.
combine their limited power to run large-scale denial of service            Some fields, such as e_ident (which identifies the
attacks. Another possible explanation for the large number of            type of file), e_type (which specifies the object type), or
botnet samples in our dataset is that the source code of some            e_machine (which contains the machine architecture), are
of these malware family is publicly available—resulting in a             needed by the kernel even before the ELF file is loaded in
large number of variations and copycat software.                         memory. Sections and segments are instead strictly dependent
   Despite their extreme popularity, botnets are not the only            on the source code and the compilation process, and are needed
form of Linux-based malware. In fact, our dataset contains also          respectively for linking and relocation purposes and to tell the
thousands of samples belonging to other categories, including            kernel how the binary must be loaded in memory for program
backdoors, ransomware, cryptocurrency miners, bankers, tradi-            execution.
tional file infectors, privilege escalation tools, rootkits, mailers,       Our data shows that malware developers often tamper with
worms, RAT programs used in APT campaigns, and even CGI-                 the ELF headers to fool the analyst or crash common analysis
based binary webshells. While these malware dominates the                tools. In particular, we identified two classes of modifications:
number of families in our dataset, many of them exist in a               those that resulted in anomalous files (but that still follow the
single variant, thus resulting in a lower number of samples.             ELF specifications), and those that produced invalid files—
   While we may discuss particular families when we present              which however can still be properly executed by the operating
our analysis results, in the rest of the paper we prefer to              system.
aggregate figures by counting individual samples. This is                Anomalous ELF. The most common example in the first
because even though samples in the same family may share                 category (5% of samples in our dataset) consists in removing
a common goal and overall structure, they can be very diverse            all information about the ELF sections. This is valid according
in the individual low-level techniques and tricks they employ            to the specifications (as sections information are not used at
(e.g., to achieve persistence or obfuscate the program code).            runtime), but it is an uncommon case that is never generated
We will return to this aspect of Linux malware and discuss its           by traditional compilers. Another example of this category
implications in Section VI.                                              consists of reporting false information about the executable.
                                                                         For example, a Linux program can report a different operating
                      V. U NDER THE H OOD                                system ABI (e.g., FreeBSD) and still be executed correctly
                                                                         by the kernel. Samples of the Mumblehard family report in
   In this section we present a detailed overview of a number            the header the fact that they require FreeBSD, but then test
of interesting behaviors we have identified in Linux malware             the system call table at runtime to detect the actual operating
and, when possible, we provide detailed statistics about the             system and execute correctly under both FreeBSD and Linux.
prevalence of each of these aspects. Our goal is not to                     For this reason, in our experiments we did not trust such
differentiate between different classes of malware or different          information and we always tried to execute a binary despite
malware families (i.e., to distinguish botnets from backdoors            the values contained in its identification field. If the required
from ransomware samples), but instead to focus on the tricks             ABI was indeed different, the program would crash at run-
and techniques commonly used by malware authors—such                     time trying to execute invalid system calls—a case that was
as packing, obfuscation, process injection, persistence, and             recognized by our system to filter out non-Linux programs.
evasion attempts. To date, this is the most comprehensive                Invalid ELF. This category includes instead those samples
discussion on the topic, and we hope that the insights we offer          with malformed or corrupted sections information (2% of sam-
will help to better understand how Linux-based malware works             ples in our dataset), typically the result of an invalid e_shoff
and will serve as a reference for future research focused on             (offset of the section header table), e_shnum (number of
improving the analysis of this type of malware.                          entries in the section header table), or e_shentsize (size
of section entries) fields in the ELF header. We also found                                  TABLE IV
evidence of samples exploiting the ELF header file format                   ELF BINARIES ADOPTING PERSISTENCE STRATEGIES
to create overlapping segments header. For instance, three
                                                                               Path                          Samples
samples belonging to the Mumblehard family declared a single                                            w/o root  w/ root
segment starting from the 44th byte of the ELF header itself and
                                                                              /etc/rc.d/rc.local           -       1393
zeroed out any field unused at runtime. Table II summarizes                   /etc/rc.conf                 -       1236
the most common ELF manipulation tricks we observed in our                    /etc/init.d/                 -        210
dataset.                                                                      /etc/rcX.d/                  -        212
                                                                              /etc/rc.local                -         11
Impact on Userspace Tools. To measure the consequences                        systemd service              -          2
of the previously discussed transformations, in Table III we                  ˜/.bashrc                    19        8
report how popular tools (used to work with ELF files) react to               ˜/.bash_profile              18        8
                                                                              X desktop autostart           3        1
unusual or malformed files. This includes readelf (part of GNU
Binutils), pyelftools (a convenient Python library to parse and               /etc/cron.hourly/            -        70
                                                                              /etc/crontab                 -        70
analyze ELF files), GDB (the de-facto standard debugger on                    /etc/cron.daily/             -        26
Linux and many UNIX-like systems), and IDA Pro 7 (the latest                  crontab utility              6         6
version, at the time of writing, of the most popular commercial               File replacement             -        110
disassembler, decompiler, and reverse engineering tool).                      File infection               5         26
   Our results show that all tools are able to properly process               Total                       1644 (21.10%)
anomalous files, but unfortunately often result in errors when
dealing with invalid fields. For example, readelf complained
for the absence of a valid table on hundreds of sample, but
                                                                   Subsystems Initialization. This appears to be the most com-
was able to complete the parsing of the remaining fields in
                                                                   mon approach adopted by malware authors and takes advantage
the ELF header. On the other side, pyelftools denies further
                                                                   of the well known Linux init system. Table IV shows that
analysis if the section header table is corrupted, while it can
                                                                   more than 1000 samples attempted to modify the system rc
instead parse ELF files if the table is declared as empty.
                                                                   script (executed at the end of each run-level). Instead, 210
Because of this poor management of erroneous conditions, for
                                                                   samples added themselves under the /etc/init.d/ folder
our experiments we decided to write our own custom ELF
                                                                   and then created soft-links to directories holding run-level
parser, which was specifically designed to work in presence of
                                                                   configurations. Overall, we found 212 binaries displacing links
unusual settings, inconsistencies, invalid values, or malformed
                                                                   from /etc/rc1.d to /etc.rc5.d, with 16 of them using
header information.
                                                                   the less common run-levels dedicated to machine halt and
   Despite its widespread use in the *nix world, GDB showed a      reboot operations. Note how malicious programs still largely
severe lack of resilience in dealing with corrupted information    rely on the System-V init system and only two samples in
coming from a malformed section header table. The presence         our dataset supported more recent initialization technologies
of an invalid value results in GDB not being able to recognize     (e.g., systemd). More important, this type of persistence only
the ELF binary and in its inability to start the program.          works if the running process has privileged permissions. If
   Finally, IDA Pro 7 was the only tool we used in our analysis    the user executing the ELF is not root or a user under
pipeline that was able to handle correctly the presence of any     privileged policies, it is usually impossible to modify services
corrupted section information or other fields that would not       and initialization configurations.
affect the program execution.
                                                                   Time-based Execution. This technique is the second choice
B. Persistence                                                     commonly used by malware and relies on the presence of cron,
   Persistence involves a configuration change of the infected     the time-based job scheduler for Unix systems. Malicious ELF
system such that the malicious executable will be able to          files try to modify, with success when running under adequate
run regardless of possible reboot and power-off operations         higher privileges, cron configuration files to get scheduled ex-
performed on the underlying machine. This, along with the          ecution at a fixed time interval. As for subsystem initialization,
ability to remain hidden, is one of the first objectives of        time-based persistence will not work if the malware is launched
malicious code.                                                    by unprivileged users unless the sample invokes the system
   A broad and well-documented set of techniques exists for        utility crontab (a SUID program specifically designed to mod-
malware authors to achieve persistence on Microsoft Windows        ify configuration files stored under /var/spool/cron/).
platforms. The vast majority of these techniques relies on the     File Infection and Replacement. Another approach for mal-
modification of Registry keys to run software at boot, when        ware to maintain a foothold in the system is by replacing
a user logs in, when certain events occurs, or to schedule         (or infecting) applications that already exist in the target.
particular services. Linux-based malware needs to rely on          This includes both a traditional virus-like behavior (where the
different strategies, which are so far more limited both in        malware locates and infect other ELF files without a strategy)
number and in nature. We group the techniques that we              as well as more targeted approaches that subvert the original
observed in our dataset in four categories, described next.        functionalities of specific system tools.
                           TABLE V                                                           TABLE VI
               ELF PROGRAMS RENAMING THE PROCESS                              ELF SAMPLES GETTING PRIVILEGES ERRORS OR
                                                                                             PROBING IDENTITIES
               Process name      Samples      Percentage
                                                                                Motivation              Samples      Percentage
               sshd                 406         5.21%
               telnetd               33         0.42%                          EPERM error                 986         12.65%
               cron                  31         0.40%                          EACCES error                716          9.19%
               sh                    14         0.18%                          Query user identity *      1609         20.65%
               busybox               11         0.14%                          Query group identity *      877         11.26%
               other tools           22         0.28%
                                                                               Total                      2637         33.84%
               empty               2034        26.11%
               other *             973         12.49%                          * Also include checks on effective and real identity
               random               618         7.93%
               Total               4091        52.50%                                         TABLE VII
                                                                         B EHAVIORAL DIFFERENCES BETWEEN USER / ROOT ANALYSIS
               * Names not representing system utilities

                                                                        Different behavior                       Samples      Percentage
                                                                       Execute privileged shell command             579         21.96%
   Our dynamically analysis reports allow us to observe infec-         Drop a file into a protected directory       426         16.15%
tion and replacement of system and user files. Examples in this        Achieve system-wide persistence              259         9.82%
category are samples in the family EbolaChan, which inject             Tamper with Sandbox                           61          2.31%
                                                                       Delete a protected file                       47          1.78%
their code at the beginning of the ls tool and append the origi-       Run ptrace request on another process        10          0.38%
nal code after the malicious data. Another example are samples
of the RST, Sickabs and Diesel families, which still use a 20
years old ELF infections techniques [13]. The first group limits
the infection to other ELF files located in the current working    program, or avoid showing unusual names in the list of running
directory, while the second adopts a system-wide infection         processes.
that also targets binaries in the /bin/ folder. Interestingly,        Overall, we noted that this behavior, already common on
samples of this family were first observed in 2001, according      Windows operating systems, is also widespread on Linux-
to a Sophos report they were still widespread in 2008 [14], and    based malware. Table V shows that over 50% of the samples
our study shows that they are still surprisingly active today. A   assumed different names once in memory, and also reports
different approach is taken by samples in the Gates family,        the top benign application that are impersonated. In total
which fully replace system tools in /bin/ or /usr/bin/             we counted more than 4K samples invoking the system call
folders (e.g., ps and netstat) after creating a backup copy        prctl with request PR_SET_NAME, or simply modifying
of the original code in /usr/bin/dpkgd/.                           the first command line argument of the program (the program
                                                                   name). Out of those, 11% adopted names taken from common
User Files Alteration. As shown in the middle part of
                                                                   utilities. For example, samples belonging to the Gafgyt family
Table IV, very few samples modify configuration files in the
                                                                   often disguise as sshd or telnetd. It is also interesting to
user home directory such as shell configurations. Malware
                                                                   discuss the difference between the two renaming techniques.
writers adopting this method can ensure persistence at user
                                                                   The first (based on the prctl call) results in a different
level, but other Linux users, beside the infected one, will not
                                                                   process name listed in /proc/<PID>/status (and used by
be affected by this persistence mechanism. While the most
                                                                   tools like pstree), while the second modifies the information
common, changes to the shell configuration are not the only
                                                                   reported in /proc/<PID>/cmdline (used by ps). Quite
form of per-user persistency. Few samples (such as those in
                                                                   strangely, none of the malware in our dataset combined the
the Handofthief family) that target desktop Linux installations,
                                                                   two techniques (and therefore could all be easily detected by
modified instead the .desktop startup files used by the
                                                                   looking for name inconsistencies).
windows manager.
                                                                      The remaining 88% of the samples either adopted an empty
   Table IV reports a summary of the amount of samples using
                                                                   name, a name of a fictitious (but not existing) file, or a random-
each technique. Surprisingly, only 21% of our ELF files imple-
                                                                   looking name often seeded by a combination of the current
mented at least one persistence strategy. However, samples that
                                                                   time and the process PID. This last behavior, implemented by
do try to be persistent often try multiple techniques in a row
                                                                   some of the Mirai samples, results in the fact that the malicious
to reach their objective. As an example, in our experiments
                                                                   process assumes a different name at every execution.
we noticed that user files alteration was a common fallback
mechanism when the sample failed to achieve system-wide            D. Required Privileges
persistency.
                                                                     Our tests show that the distinction between administrator
C. Deception                                                       (root) and normal user is very important for Linux-based
   Stealthy malware may try to hide their nature by assuming       malware. First, malicious samples can perform different actions
names that look genuine and innocuous at a first glance, with      and show a different behavior when they are executed with
the objective of tricking the user to open an apparently benign    super-user privileges. Second, especially when targeting low-
end embedded systems or IoT devices, malware may even be                                              TABLE VIII
designed to run as root—and thus fail to execute if analyzed                                         ELF PACKERS
with more limited privileges.                                                      Process name                Samples      Percentage
   Therefore, we first executed every sample with normal user
                                                                                  Vanilla UPX                     189         1.79%
privileges. If, during the execution, we detected any attempt                     Custom UPX Variant              188         1.78%
to retrieve the user or group identities (which could be used                      - Different Magic              129
by the program to decide the malware’s next actions) or to                         - Modified UPX strings          55
                                                                                   - Inserted junk bytes          126
access any resource that returned a EPERM or EACCES errors,                        - All of the previous           16
we repeated the analysis by running the sample with root                          Mumblehard Packer                3          0.03%
privileges. This was the case for 2637 samples (25% of the
dataset) and in 89% of them we detected differences in the
sample behavior extracted from the two execution traces.              when samples are executed with root privileges. Interestingly,
   Table VII presents a list of behaviors that were executed
                                                                      among the 2,637 malware samples we re-executed with root
when running as root but were not observed when running
                                                                      privileges, only 15 successfully loaded a kernel module and
as a normal user. Among these, privileged shell commands
                                                                      none of them performed an unload procedure. All these cases
and operations on files are predominant, with malware using
                                                                      involved the standard ip_tables.ko, necessary to setup IP
elevated privileges to create or delete files in protected folders.
                                                                      packet filter rules. We also identified 119 samples, belonging
For instance, samples of the Flooder and IoTReaper families
                                                                      to the Gates or Elknot families that attempted to load a custom
hide their traces by deleting all log files in /var/log,
                                                                      kernel module but failed as the corresponding .ko file was not
while samples of the Gafgyt family only delete last login
                                                                      present during the analysis.2
and logout information (/var/log/wtmp). Moreover, in few
cases malware running as root were able to tamper with the            E. Packing & Polymorphism
sandboxed execution: we found binaries that, upon detection
of the emulated execution environment, would kill the SSH                Runtime packing is at the same time one of the most com-
daemon or even delete the entire file system.                         mon and one of the most sophisticated obfuscation techniques
   We now look in more details at two specific actions that are       adopted by malware writers. If properly implemented, it com-
determined by the execution privileges: privileges escalation         pletely prevents any attempt to statically analyze the malware
exploits and interaction with the OS kernel.                          code and it also considerably slows down an eventual manual
                                                                      reverse engineering effort. While hundreds of commercial, free,
Privileges Escalation. On the one hand, one of the advantages
                                                                      and underground packers exist for Microsoft Windows, things
of using kernel probes for dynamic analysis is its ability to
                                                                      are different in the Linux world: only a handful of ELF packers
trace functions in the OS kernel—making possible for us
                                                                      have been proposed so far [15]–[17], and the vast majority
to detect signs of successful exploitations. For example, by
                                                                      of them are proof-of-concept projects. The only exception is
monitoring commit_creds we can detect when a new set
                                                                      UPX, a popular open source compression packer introduced in
of credentials has been installed on a running task. On the
                                                                      1998 to reduce the size of benign executables, which is freely
other hand, the sandboxes built to host the execution of each
                                                                      available for many operating systems.
sample were deployed with up-to-date and fully-patched Linux
                                                                         Automatic recognition and analysis of packers is a subtle
operating systems—which prevented binaries from exploiting
                                                                      problem, and it has been the focus on many academic and
old vulnerabilities.
                                                                      industrial studies [18]–[22]. For our experiment, we relied on
   According to our trace analysis, there was no evidence
                                                                      a set of heuristics based on the file segments entropy and on the
of samples that successfully elevated their privileges inside
                                                                      results of the static analysis phase (i.e., number of imported
our machines, or that had been able to perform privileged
                                                                      symbols, percentage of code section correctly disassembled,
actions under user credentials. Regarding older (and therefore
                                                                      and total number of functions identified) to flag samples that
unsuccessful) exploits, we developed custom signatures to
                                                                      were likely packed. Moreover, since UPX-like variants seem
identify the ten most common escalation attacks based on
                                                                      to dominate the scene, we decided to add to our pipeline a set
known vulnerabilities in the Linux kernel1 , for which an
                                                                      of custom analysis routines to identify possible UPX variants
exploitation proof-of-concept is available to the public. Our
                                                                      and a generic multi-architecture unpacker that can retrieve the
tests revealed that CVE-2016-5195 was the most frequently
                                                                      original code of samples packed with these techniques.
used vulnerability, with a total of 52 ELF programs that tried
to exploit it in our sandbox. We also detected five attempts to       UPX Variations. Vanilla UPX and its variants are by far the
exploit CVE-2015-1328, while the remaining eight checks did           most prevalent form of packing in our dataset. As shown in Ta-
not return any positive match.                                        ble VIII, out of 380 packed binaries only three did not belong
Kernel Modules. System calls tracing allows our system to             to this category. The table also highlights the modifications
track attempts to load or unload a kernel module, especially          made to the UPX format with the goal of breaking the standard
  1 CVE-2017-7308, CVE-2017-6074, CVE-2017-5123, CVE-2017-1000112,       2 This is a well-known problem affecting dynamic malware analysis systems,
CVE-2016-9793, CVE-2016-8655, CVE-2016-5195, CVE-2016-0728, CVE-      as samples are collected and submitted in isolation and can thus miss external
2015-1328, CVE-2014-4699.                                             components that were part of the same attack.
                           TABLE IX                                                              TABLE X
               T OP TEN COMMON SHELL COMMANDS                                    T OP TEN P ROC FILE SYSTEM ACCESSES BY
                           EXECUTED                                                        MALICIOUS SAMPLES

              Shell command    Samples   Percentage                             Path                 Samples   Percentage
             sh                  400       5.13%                               /proc/net/route        3368       43.22%
             sed                 243       3.12%                               /proc/filesystems      649        8.33%
             cp                  223       2.86%                               /proc/stat              515        6.61%
             rm                  216       2.77%                               /proc/net/tcp           498        6.39%
             grep                214       2.75%                               /proc/meminfo           354        4.54%
             ps                  131       1.68%                               /proc/net/dev           346        4.44%
             insmod              124       1.59%                               /proc/<PID>/stat       320        4.11%
             chmod               113       1.45%                               /proc/cmdline           278        3.57%
             cat                  93       1.19%                               /proc/<PID>/cmdline    259        3.32%
             iptables             84       1.08%                               /proc/cpuinfo           226        2.90%



UPX unpacking tool. This includes a modification to the magic       to “vaccinate” Mirai, uses iptables to close and open network
number (so that the file does not appear to be packed with UPX      ports, while Mirai tries to close vulnerable ports already used
anymore), the modification of UPX strings, and the insertion        to infect the system.
of junk bytes (to break the UPX utility). However, all these        Process Injection An attacker may want to inject new code
samples share the same underlying packing structure and the         into a running process to change its behavior, make the sample
same compression algorithm—showing that malware writers             more difficult to debug, or to hook interesting functions in
simply applied “cosmetic” variations to the open source UPX         order to steal information.
code.                                                                  Our system monitors three different techniques a process
Custom packers. Linux does not count on a large variety             can use to write to the memory of another program: 1)
of publicly available packers and UPX is usually the main           a ptrace syscall that requests the PTRACE_POKETEXT,
choice. However, we detected three samples (all belonging           PTRACE_POKEDATA, or PTRACE_POKEUSER functionali-
to the Mumblehard family) that implemented some form of             ties; 2) a PTRACE_ATTACH request followed by read/write
custom packing, where a single unpacking routine is executed        operations to /proc/<TARGET_PID>/mem; and 3) an in-
before transferring the control to the unpacked program [23].       vocation to the process_vm_writev system call.
In one case, the malware started a separate process running            It is important to mention that the Linux kernel has been
a perl interpreter and then used the main process to decrypt        hardened against ptrace calls in 2010. Since then it is
instructions and feed them into the interpreter.                    not possible to use ptrace on processes that are not direct
                                                                    descendant of the tracer process, unless the unprivileged user is
F. Process Interaction                                              granted the CAP_SYS_PTRACE capability. The same capabil-
   This section covers the techniques used by Linux malware to      ity is required to execute the process_vm_writev call, a
interact with child processes or other binaries already installed   new system call introduced in 2012 with kernel 3.2 to directly
or running in the system.                                           transfer data between the address spaces of two processes.
                                                                       We found a sample performing injection by using the
Multiple Processes. 25% of our samples consists of a single
                                                                    first technique mentioned above. It injects a dynamic li-
process, 9% spawn a new process, 43% involves three pro-
                                                                    brary in every active process that uses libc (but ex-
cesses in total (largely due to the “double-fork” pattern used
                                                                    cludes gnome-session, dbus and pulseaudio). In
to daemonize a program), while the remaining 23% created a
                                                                    the injected payload the malware uses the libc function
higher number of separate processes (up to 1684).
                                                                    __libc_dlopen_mode to load dynamic objects at run-
   Among the samples that spawn multiple processes we find
                                                                    time. This function is similar to the well-known dlopen,
many popular botnets such as Gafgyt, Tsunami, Mirai, and
                                                                    which is less preferable because implemented in libdl, not
XorDDos. For instance, Gafgyt creates a new process for every
                                                                    already included in the libc. After the new code is mapped
attempt to connect to its command and control (C&C) server.
                                                                    in memory, the malware issues ptrace requests to backup
XorDDos, instead, creates parallel DDos attack processes.
                                                                    the registers values of the victim process, hijack the control
Shell Commands. 13% of the samples we analyzed inside               flow to execute its malicious behavior, and restore the original
our sandbox executed at least one external shell command.           execution context.
In total, we registered the execution of 93 unique command-
line tools—the most prevalent of which are summarized in            G. Information Gathering
Table IX. Commands such as sed, cp, and chmod are often                Information gathering is an important step of malware
executed to achieve persistence on the target system, while         execution as the collected information can be used to detect
rm is used to unlink the sample itself or to delete the bash        the presence of a sandbox, or to control the execution of the
history file. Several malware families also try to kill previous    sample. Data stored on the system can also be exfiltrated to a
infections of the same malware. Hijami, the counter-malware         remote location, as it often happens with programs controlled
                             TABLE XI                                                             TABLE XIII
   T OP TEN S YSFS FILE SYSTEM ACCESSES BY MALICIOUS SAMPLES                         ELF PROGRAMS SHOWING EVASIVE FEATURES

    Path                                      Samples      Percentage               Type of evasion          Samples     Percentage
   /sys/devices/system/cpu/online               338          4.34%                 Sandbox detection             19        0.24%
   /sys/devices/system/node/node0/meminfo        26          0.33%                 Processes enumeration *      259        3.32%
   /sys/module/x tables/initstate               22           0.28%                 Anti-debugging                63        0.81%
   /sys/module/ip tables/initstate               22          0.28%                 Anti-execution                 3        0.04%
   /sys/class/dmi/id/sys vendor                 18           0.23%                 Stalling code                  0           -
   /sys/class/dmi/id/product name               18           0.23%
   /sys/class/net/<interface>tx queue len        9           0.12%                 * Not used for evasion but candidate behavior
   /sys/firmware/efi/systab                      3           0.04%
   /sys/devices/pci0000:00/<device>              3           0.04%                                  TABLE XIV
   /sys/bus/usb/devices/<device>                 2           0.03%              F ILE SYSTEM PATHS LEADING TO SANDBOX DETECTION

                                                                            Path                              Detected Environments      #
                           TABLE XII
            T OP TEN ACCESSES ON / E T C / BY MALICIOUS                    /sys/class/dmi/id/product name      VMware/VirtualBox         18
                                  SAMPLES                                  /sys/class/dmi/id/sys vendor             QEMU                 18
                                                                           /proc/cpuinfo                     CPU model/hypervisor flag    1
              Path                  Samples   Percentage                   /proc/sysinfo                             KVM                  1
                                                                           /proc/scsi/scsi                     VMware/VirtualBox          1
             /etc/rc.d/rc.local       1393      17.88%                     /proc/vz and /proc/bc                OpenVZ container          1
             /etc/rc.conf             1236      15.86%                     /proc/xen/capabilities                XEN hypervisor           1
             /etc/resolv.conf          641       8.23%                     /proc/<PID>/mountinfo                   chroot jail           1
             /etc/nsswitch.conf        453       5.81%
             /etc/hosts               423       5.43%
             /etc/passwd              244       3.13%
             /etc/host.conf            201       2.58%
             /etc/rc.local             170       2.18%
                                                                        retrieve the name of the running kernel image.
             /etc/localtime            165       2.12%                     Another common type of information gathering focuses
             /etc/cron.deny           101       1.30%                   on processes enumeration. This is used to prevent multiple
                                                                        executions of the same malware (e.g., by the Mirai family),
                                                                        or to identify other relevant programs running on the target
by a C&C server. In this section we look at which portions of           machine. As reported in Table IX, we found 131 samples
the file system are inspected by malware and discuss security-          executing the shell command ps, used as a fast interface to
relevant paths analysts should monitor when inspecting new              get the list of running processes. For example, 67 samples of
malware strains.                                                        the BitcoinMiner family invoke ps and then try to kill other
Proc and Sysfs File Systems. The proc and sysfs virtual                 crypto-miner processes that may interfere with their malicious
file systems contain, respectively, runtime system informa-             activity.
tion on processes, system and hardware configurations, and              Configuration Files. System configuration files are contained
information on the kernel subsystems, hardware devices, and             in the /etc/ folder. As reported in Table XII, configuration
kernel drivers. We divide the type of information collected             files required to achieve persistence are the ones accessed more
by malware samples in three macro categories: system con-               often. Network-related configuration files also appear to be
figuration, processes information, and network configuration.           popular, with /etc/resolv.conf (the DNS resolver) or
The network category is the most common in our dataset with             /etc/hosts (the mapping between hosts and IP addresses).
more than 3000 samples, as shown in Table X, which accessed             Among the top entries we also find /etc/passwd (list of
/proc/net/route (system routing table) to get the list                  registered accounts). For instance, Flooder samples use it to
of active network interfaces with their relative configuration.         check for the presence of a backdoor account on the system.
Additional information is extracted from /proc/net/tcp                  If not found, they add a new user by directly writing to
(active TCP sockets) and /proc/net/dev (sent and re-                    /etc/passwd and /etc/shadow.
ceived packets). Moreover, 111 samples in our dataset read
                                                                        H. Evasion
/proc/net/arp to retrieve the system ARP table. For the
sysfs counterpart, reported in Table XI, we found accesses                 The purpose of evasion is to hide the malicious behavior and
to /sys/class/net/ to get the transmission queue length,                remain undetected as long as possible. This typically requires
a relevant information for DDoS attacks.                                the sample to detect the presence of analysis tools, or to distin-
   The system configuration category is the second most com-            guish whether it is running within an analysis environment or
mon, with hundreds of samples that extracted the amount of              on a real target device. We now present more details about the
installed memory, the number of available CPU cores, and                different evasion techniques, whose prevalence in our dataset
other CPU characteristics. The files used for sandbox detection         is summarized in Table XIII.
and evasion also fall into this category (see Subsection V-H)           Sandbox Detection. Our string comparison instrumentation
as well as the lists of USB and PCI connected devices.                  detected a number of programs that attempted to detect the
This category also includes accesses to /proc/cmdline to                presence of a sandbox by comparing different pieces of
information extracted from the system with strings such as                                       TABLE XV
“VMware” or “QEMU.” Table XIV reports the files where                 T OP 20 LIBRARIES INCLUDED BY DYNAMICALLY LINKED EXECUTABLES
the information was collected. Ten samples who tested the
                                                                              Library     Percentage   Library         Percentage
sys_vendor file were able to detect our analysis environ-
ment when executed with root privileges (as we restricted                    glibc         74.21%      libscotch         1.23%
                                                                             uclibc        24.24%      libtinfo          0.75%
the permissions to files exposing the motherboard DMI zone                   libgcc         9.74%      libgmp            0.75%
information reported by the kernel). We also identified sam-                 libstdc++     7.12%       libmicrohttpd     0.64%
ples attempting to detect chroot()-based jails (by com-                      libz           5.24%      libkrb5           0.64%
                                                                             libcurl        3.64%      libcomerr         0.64%
paring /proc/1/mountinfo with /proc/<malware                                 libssl         2.35%      libperl           0.59%
PID>/mountinfo), OpenVZ containers [24], and even one                        libxml2        1.44%      libhwloc          0.59%
binary (from the Handofthief family) trying to evade IBM                     libjansson    1.39%       libedit           0.54%
                                                                             libncurses    1.28%       libopencl         0.54%
mainframes and IBM’s virtualization technology. It is also in-
teresting to note how some samples simply decide to exit when
they detect they are running in a virtual environment, while         We investigated whether Linux malware is already using
other adopt a more aggressive (but less stealthy) approach,          simple variants of this technique by scanning our execution
such as trying to delete the entire file system.                     traces for samples using time- or sleep-related functions. We
Processes Enumeration. It is common in Windows to evade              found that 64% of the binaries we analyzed make use of the
analysis by verifying the presence of a particular set of            nanosleep system call, with values ranging from less than
processes, or inspecting the goodness and authenticity of            a second to higher than three hours. However, none of them
companion processes that live on the system. We investigated         appear to use these delays to stall their execution (in fact, our
whether Linux malware samples already employ similar tech-           traces contained clear signs of their behavior), but rather to
niques and found 259 samples that perform a full scan of             coordinate child processes or network communications.
the /proc/<PID> directories. However, none of the samples
                                                                     I. Libraries
appeared to perform these scans for evasive purposes but
instead to test if the machine was already infected or to identify      There are two main ways an executable can make use
target processes to kill (as we explain in Section V-F).             of libraries. In the first (and more common) case, the exe-
                                                                     cutable is dynamically linked and external libraries are loaded
Anti-Debugging. The most common anti-debugging technique
                                                                     at run-time, permitting code reuse and localized upgrades.
is based on the ptrace system call that provides to debuggers
                                                                     Conversely, an executable that is statically linked includes
the ability to “attach” to a target process to programmati-
                                                                     the object files of its libraries as part of its executable file—
cally inspect and interact with it. As a given process can
                                                                     removing any external dependency of the application and thus
only have at most one debugger attached to it, one com-
                                                                     making it more portable.
mon evasion technique used by malware consists of invoking
                                                                        More than 80% of the samples we analyzed are statically
the ptrace system call with flags PTRACE_TRACEME or
                                                                     linked. Nevertheless, we note that only 24% of these samples
PTRACE_ATTACH on themselves to detect if another debugger
                                                                     have been stripped from their symbols, with the remaining
is already attached or prevent it to do so while the sample
                                                                     ones often including even functions and variables names used
is running. We found 63 samples employing this mechanism.
                                                                     by developers. Similarly for dynamically linked samples in our
We also identified one sample checking the presence of the
                                                                     dataset, only 33% of them are stripped. We find this trend very
LD_PRELOAD environment variable, which is often used to
                                                                     interesting as apparently malware developers lack motivation
override functions in dynamically loaded libraries (with the
                                                                     to obfuscate their code against manual analysis—which is
goal of dynamically instrumenting their execution).
                                                                     in sharp contrast with the complexity of evasive Windows
   It is important to note that the tracing system we use            malware.
in our sandbox is based on kernel probes (as described in
section III-D), and it cannot be detected or tampered with by        Common Libraries. Table XV lists the dynamic libraries that
using anti-debugging techniques.                                     are most often imported by malware samples in our dataset.
                                                                     This lists shows two important aspects. First, that while the
Anti-Execution. Our experiments detected samples belonging           GNU C library (glibc) is (expectedly) the most requested
to the DnsAmp malware family that did not manifest any               library, we found that 24% of samples link against smaller
behavior, except from comparing their own file name with             implementations like uClibc, often used in embedded systems.
a hardcoded string. A closer look at these samples showed            It is also interesting to see how almost 10% of the dataset links
that the malware authors used this trick as an evasive solu-         against libgcc, a library used by the GCC compiler to handle
tion, as many malware collection infrastructures and analysis        arithmetic operations that the target processor cannot perform
sandboxes often rename the files before their analysis.              directly (e.g., floating-point and fixed-point operations). This
Stalling Code. Windows malware is known to often employ              library is rarely used in the context of desktop environments,
stalling code that, as the name suggests, is a technique used to     but it is often used in embedded devices with architectures that
delay the execution of the malicious behavior – assuming an          do not support floating point operations. The second interesting
analysis sandbox would only run each sample for few minutes.         aspect is that, while in total we identified more than 200
different libraries, the distribution has a very long tail and it      Shazhad et al. [58] and by Bai et al. [59] extracts static features
drops very steeply. For instance, the tenth most popular library       from ELF binaries to train a classifier for malware detection.
is only used by 1% of the samples.                                     Unfortunately, these works are not comprehensive, do not take
                                                                       into account different architectures, or are easily evaded by
                 VI. I NTRA - FAMILY VARIETY                           stripping a binary or by using packing.
   In the previous section we described several characteristics           Researchers have also started to explore dynamic analysis
of Linux-based malware. For each of them, we presented                 for non-Windows malware only very recently. The few solu-
the number of samples instead of the count of families that            tions that are available at the moment support a limited number
exhibited a given trait. This is because we noted that samples         of platforms or provide very limited analysis capabilities. For
belonging to the same family often had very different charac-          example, Limon [60] is an analysis sandbox based on strace
teristics, probably due to the availability of the source codes        (and thus easily detectable), and it only supports the analysis
for several classes of Linux malware.                                  of x86-flavored binaries. Sysdig [61] and PayloadSecurity [62]
   As an example of this variety, we want to discuss the case          are affected by similar issues and they also only work for
of a popular malware family, Tsunami, for which we have                x86 binaries. Detux [63], instead, supports four different
743 samples in our dataset. Those samples are compiled for             architectures (i.e., x86, x86-64, ARM, and MIPS). However,
nine different architectures, the most common being x86-64,            it only performs a very basic analysis by running readelf
and the rarest being Hitachi SuperH. In total, 86% of them             and provides network dumps. Cuckoo sandbox [64] is another
are statically linked and 13% are stripped. Dynamically linked         available tool that supports the analysis of Linux samples.
Tsunami samples rely on different loaders, and their entropy           However, the Cuckoo project only provides the external orches-
varies from 1.85 to 7.99. Out of the 19 samples with higher            tration analysis framework, while the preparation of the various
entropy, one is packed with vanilla UPX while the other 18             sandbox images is left to the user. Last, in November 2017
use modified versions of the same algorithm.                           VirusTotal announced the integration of the Tencent HABO
   This variability is not limited to static features. For instance,   sandbox solution, which reportedly is able to analyze also
looking at our dynamic traces we noted the use of different            Linux-based malware [9]. Unfortunately, there is no public
persistence techniques with some samples only relying on               report on how the system works and it currently works only
user-level approached and other using run-level scripts or cron        for x86 binaries.
jobs for system-wide persistence. Concerning unprivileged and             One of the first systematic studies of IoT malware was done
privileged execution, only 15% of the Tsunami samples we               by Pa et al. [65]. In their paper, they present a Telnet honeypot
analyzed in our sandboxes tested the user privileges or got            to measure the current attack trends as well as the first sandbox
privileges-related errors. Differences arise even in terms of          environment based on Qemu and OpenWRT called IoTBOX
evasion: 17 samples contain code to evade the sandbox while            for analyzing IoT malware. They showed the issue of IoT
all the others did not include evasive functionalities.                devices exposing Telnet online and they collected few families
                                                                       actively targeting this service. Similarly, Antonakakis et al. [4]
                     VII. R ELATED W ORK                               studied in detail a specific Linux malware family, the Mirai
   In the past two decades the security community has focused          botnet. They measure systematically the evolution and growth
almost exclusively on fighting malware targeting Microsoft             of the botnet mainly from the network point of view. These
Windows. As a result, hundreds of papers have described                works are invaluable to the community, but only look at limited
techniques to analyze PE binaries [25]–[28], detecting ongoing         aspects of the entire picture: the samples network behavior.
threats [27], [29], [30], and preventing possible infection at-        We believe that our work can complement these efforts and
tempts [31]–[33] on Windows operating systems. The commu-              provide a clearer overview of how Linux malware actually
nity also developed many analysis tools for dissecting threats         works. Moreover, the datasets used in these previous studies
related to the Windows environment, ranging from dynamic               are not representative of the overall Linux malware, since they
analysis solutions [34]–[37] to dissectors for file formats used       were collected via telnet-based honeypots.
as attack vectors [38]–[40].
   With the exception of mobile malware, non-Windows ma-                                    VIII. C ONCLUSIONS
licious software did not receive the same level of atten-                 This paper presents the first comprehensive study of Linux-
tion. While the hacking community developed—almost two                 based malware. We document the design and implementation
decades ago—interesting techniques to implement malicious              of the first analysis pipeline specifically tailored for Linux
ELF files [13], [41]–[44], rootkits [45], [46], and tools to           malware, and we discuss the results of the first large-scale em-
dissect them [47]–[49], none of them has seen vast adoption.           pirical study on how Linux malware implements its malicious
In fact, the security industry has only recently started looking       behavior. While the complexity of current Linux malware is
at ELF files—mainly driven by newsworthy cases like the                not very high, we have identified a number of samples already
Mirai botnet [50] and Shellshock [51]. Many blog posts and             adopting techniques borrowed from their Windows counter-
papers were published for the analysis and dissection of spe-          parts. We believe these insights can be the foundation for more
cific families [52]–[57], but these investigations were mainly         systematic future works in the area, which is, unfortunately,
conducted by manual reverse engineering. Recent research by            bound to have an ever-increasing importance.
                              R EFERENCES                                         [31] P. Mell, K. Kent, and J. Nusbaum, Guide to malware incident prevention
                                                                                       and handling. US Department of Commerce, Technology Administra-
 [1] StatCounter, “Desktop Operating System Market Share Worldwide.”                   tion, National Institute of Standards and Technology, 2005.
     http://gs.statcounter.com/os-market-share/desktop/worldwide.                 [32] D. Harley, U. E. Gattiker, and R. Slade, Viruses revealed. McGraw-Hill
 [2] ZDnet,        “Google’s      VirusTotal      puts      Linux      malware         Professional, 2001.
     under          the        spotlight.”        http://www.zdnet.com/article/   [33] M. E. Locasto, K. Wang, A. D. Keromytis, and S. J. Stolfo, “Flips:
     googles-virustotal-puts-linux-malware-under-the-spotlight/.                       Hybrid adaptive intrusion prevention,” in RAID, pp. 82–101, Springer,
 [3] “Malware Must Die!.” http://blog.malwaremustdie.org/.                             2005.
 [4] Antonakakis et al., “Understanding the Mirai Botnet,” in Proceedings of      [34] “malwr.” https://www.malwr.com/.
     the USENIX Security Symposium, 2017.                                         [35] “CWsandbox.” http://www.mwanalysis.org.
 [5] M. Sebastián, R. Rivera, P. Kotzias, and J. Caballero, “AVclass: A Tool     [36] “Anubis.” https://anubis.iseclab.org.
     for Massive Malware Labeling,” in RAID, 2016.                                [37] “VirusTotal += Behavioural Information.” http://blog.virustotal.com/
 [6] “radare2, a portable reversing framework.” http://www.radare.org/.                2012/07/virustotal-behavioural-information.html.
 [7] D. Andriesse, A. Slowinska, and H. Bos, “Compiler-Agnostic Function          [38] “oletools - python tools to analyze OLE and MS Office files.” https:
     Detection in Binaries,” in IEEE European Symposium on Security and                //www.decalage.info/python/oletools.
     Privacy, 2017.                                                               [39] “peepdf - PDF Analysis Tool.” http://eternal-todo.com/tools/
 [8] “SystemTap.” https://sourceware.org/systemtap/.                                   peepdf-pdf-analysis-tool.
 [9] “Malware analysis sandbox aggregation: Welcome Tencent HABO.” http:          [40] “oledump-py.” https://blog.didierstevens.com/programs/oledump-py/.
     //blog.virustotal.com/2017/11/malware-analysis-sandbox-aggregation.          [41] Silvio Cesare, “Shared Library Redirection via ELF PLT Infection.” http:
     html.                                                                             //www.phrack.org/issues/56/7.html#article.
[10] Nguyen Anh Quynh, “Unicorn Emulator.” https://github.com/                    [42] Silvio Cesare, “Runtime kernel kmem patching.” https://github.
     unicorn-engine/unicorn.                                                           com/BuddhaLabs/PacketStorm-Exploits/blob/master/9901-exploits/
[11] “Shodan, the world’s first search engine for Internet-connected devices.”         runtime-kernel-kmem-patching.txt.
     https://www.shodan.io/.                                                      [43] Z0mbie, “Injected Evil.” http://z0mbie.daemonlab.org/infelf.html.
[12] Z. Durumeric, E. Wustrow, and A. Halderman, “ZMap: Fast Internet-            [44] Alexander Bartolich, “The ELF Virus Writing HOWTO.”
     wide Scanning and Its Security Applications,” in Proceedings of the               http://www.linuxsecurity.com/resource files/documentation/
     USENIX Security Symposium, 2013.                                                  virus-writing-HOWTO/ html/index.html.
[13] Silvio Cesare, “Unix ELF parasites and virus.” http://vxer.org/lib/vsc01.    [45] darkangel, “Mood-NT.” http://darkangel.antifork.org/codes/mood-nt.tgz.
     html.                                                                        [46] sd and devik, “Linux on-the-fly kernel patching without LKM.” http:
[14] SophosLabs,       “Botnets,    a    free   tool     and    6    years   of        //phrack.org/issues/58/7.html.
     Linux/Rst-B.”               https://nakedsecurity.sophos.com/2008/02/13/     [47] Mayhem, “The Cerberus ELF Interface.” http://phrack.org/issues/61/8.
     botnets-a-free-tool-and-6-years-of-linuxrst-b.                                    html.
[15] Team TESO, “Burneye ELF encryption program.” https:                          [48] elfmaster, “ftrace.” https://github.com/elfmaster/ftrace.
     //packetstormsecurity.com/files/30648/burneye-1.0.1-src.tar.bz2.html.        [49] elfmaster, “ECFS.” https://github.com/elfmaster/ecfs.
[16] elfmaster, “ELF Packer v0.3.” http://www.bitlackeys.org/projects/            [50] Nicky Woolf, “DDoS attack that disrupted internet was largest of its
     elfpacker.tgz.                                                                    kind in history, experts say.” https://www.theguardian.com/technology/
[17] grugq and scut, “Armouring the ELF: Binary encryption on the UNIX                 2016/oct/26/ddos-attack-dyn-mirai-botnet.
     platform.” http://phrack.org/issues/58/5.html.                               [51] Dave Lee, “Shellshock: ’Deadly serious’ new vulnerability found.” http:
[18] R. Lyda and J. Hamrock, “Using entropy analysis to find encrypted and             //www.bbc.com/news/technology-29361794.
     packed malware,” IEEE Security & Privacy, vol. 5, no. 2, 2007.               [52] Cathal, Mullaney and Sayali, Kulkarni, “VB2014 paper: Linux-
[19] X. Ugarte-Pedrero, D. Balzarotti, I. Santos, and P. G. Bringas, “RAMBO:           based Apache malware infections: biting the hand that serves
     Run-time packer Analysis with Multiple Branch Observation,” July 2016.            us        all.”      https://www.virusbulletin.com/virusbulletin/2016/01/
[20] S. Cesare and Y. Xiang, “Classification of malware using structured               paper-linux-based-apache-malware-infections-biting-hand-serves-us-all/.
     control flow,” in Proceedings of the Eighth Australasian Symposium on        [53] MMD, “MMD-0062-2017 - Credential harvesting by SSH Direct TCP
     Parallel and Distributed Computing-Volume 107, pp. 61–70, Australian              Forward attack via IoT botnet.” http://blog.malwaremustdie.org/2017/02/
     Computer Society, Inc., 2010.                                                     mmd-0062-2017-ssh-direct-tcp-forward-attack.html.
[21] R. Perdisci, A. Lanzi, and W. Lee, “Mcboost: Boosting scalability in         [54] MMD, “MMD-0030-2015 - New ELF malware on Shell-
     malware collection and analysis using statistical classification of exe-          shock:      the     ChinaZ.”      http://blog.malwaremustdie.org/2015/01/
     cutables,” in Computer Security Applications Conference, 2008. ACSAC              mmd-0030-2015-new-elf-malware-on.html.
     2008. Annual, pp. 301–310, IEEE, 2008.                                       [55] MMD, “MMD-0025-2014 - ITW Infection of ELF .IptabLex and .Ipta-
[22] M. Z. Shafiq, S. M. Tabish, F. Mirza, and M. Farooq, “Pe-miner: Mining            bLes China DDoS bots malware.” http://blog.malwaremustdie.org/2014/
     structural information to detect malicious executables in realtime.,”             06/mmd-0025-2014-itw-infection-of-elf.html.
     Springer.                                                                    [56] A. Wang, R. Liang, X. Liu, Y. Zhang, K. Chen, and J. Li, An Inside
[23] M.Léveillé, Marc-Etienne, “Unboxing Linux/Mumblehard.” https://www.             Look at IoT Malware.
     welivesecurity.com/wp-content/uploads/2015/04/mumblehard.pdf.                [57] P. Celeda, R. Krejci, J. Vykopal, and M. Drasar, “Embedded malware-
[24] “OpenVZ, a container-based virtualization for Linux.” https://openvz.org/         an analysis of the chuck norris botnet,” in Computer Network Defense
     Main Page.                                                                        (EC2ND), 2010 European Conference on, pp. 3–10, IEEE, 2010.
[25] G. Wicherski, “pehash: A novel approach to fast malware clustering,”         [58] F. Shahzad and M. Farooq, “Elf-miner: Using structural knowledge
     in Proceedings of the 2Nd USENIX Conference on Large-scale Exploits               and data mining methods to detect new (linux) malicious executables,”
     and Emergent Threats: Botnets, Spyware, Worms, and More, LEET’09,                 Knowledge and Information Systems, 2012.
     2009.                                                                        [59] J. Bai, Y. Yang, S. Mu, and Y. Ma, “Malware detection through mining
[26] Ferrie, Peter and Peter, Szr, “Hunting for metamorphic.” http://vxer.org/         symbol table of Linux executables,” Information Technology Journal,
     lib/apf39.html.                                                                   2013.
[27] M. Christodorescu, S. Jha, S. A. Seshia, D. Song, and R. E. Bryant,          [60] K. Monnappa, “Automating Linux Malware Analysis Using Limon
     “Semantics-aware malware detection,” in Proceedings of the 2005 IEEE              Sandbox,” Black Hat Europe 2015, 2015.
     Symposium on Security and Privacy, SP ’05, 2005.                             [61] “Sysdig.” https://www.sysdig.org/.
[28] C. Kruegel, W. Robertson, F. Valeur, and G. Vigna, “Static disassembly       [62] PayloadSecurity,      “VxStream       Sandbox     Linux.”    https://www.
     of obfuscated binaries,”                                                          payload-security.com/products/linux.
[29] S. J. Stolfo, K. Wang, and W.-J. Li, “Fileprint analysis for malware         [63] “Multiplatform Linux Sandbox.” https://detux.org/.
     detection,” ACM CCS WORM, 2005.                                              [64] “Cuckoo Sandbox 2.0 Release Candidate 1.” https://cuckoosandbox.org/
[30] D. Dagon, X. Qin, G. Gu, W. Lee, J. Grizzard, J. Levine, and H. Owen,             blog/cuckoo-sandbox-v2-rc1.
     “Honeystat: Local worm detection using honeypots,” in RAID, vol. 4,          [65] Y. P. Minn, S. Suzuki, K. Yoshioka, T. Matsumoto, and C. Rossow,
     pp. 39–58, Springer, 2004.                                                        “IoTPOT: Analysing the rise of IoT compromises,” in 9th USENIX
     Workshop on Offensive Technologies (WOOT). USENIX Association,
     2015.

                              A PPENDIX




                                                                          Fig. 5. Number of imported symbols in dynamically linked samples




       Fig. 2. File size distribution of ELF malware in the dataset




                                                                                     Fig. 6. Entropy distribution over the dataset



Fig. 3. Number of functions identified by IDA Pro in dynamically linked
samples




       Fig. 4. Percentage of LOAD segments analyzed by IDA Pro



                                                                              Fig. 7. Library imports for dynamically linked executables
