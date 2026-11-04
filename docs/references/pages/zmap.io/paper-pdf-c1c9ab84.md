---
url: "https://zmap.io/paper.pdf"
title: "This paper appeared in Proceedings of the 22nd USENIX Security Symposium, August 2013."
captured: "2026-11-02T19:04:42Z"
via: "live"
sha256: "33e7924037400b917095254bab06f5bd664f39f25961a8301058890caea9240d"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# This paper appeared in Proceedings of the 22nd USENIX Security Symposium, August 2013.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://zmap.io/paper.pdf>

             This paper appeared in Proceedings of the 22nd USENIX Security Symposium, August 2013.
               ZMap source code and documentation are available for download at https://zmap.io/.




        ZMap: Fast Internet-Wide Scanning and its Security Applications

            Zakir Durumeric                       Eric Wustrow                     J. Alex Halderman
          University of Michigan              University of Michigan              University of Michigan
                zakir@umich.edu                     ewust@umich.edu                   jhalderm@umich.edu




                       Abstract                               mid-range machine running ZMap is capable of scanning
                                                              for a given open port across the entire public IPv4 address
Internet-wide network scanning has numerous security
                                                              space in under 45 minutes—over 97% of the theoreti-
applications, including exposing new vulnerabilities and
                                                              cal maximum speed of gigabit Ethernet—without requir-
tracking the adoption of defensive mechanisms, but prob-
                                                              ing specialized hardware [11] or kernel modules [8, 28].
ing the entire public address space with existing tools is
                                                              ZMap’s modular architecture can support many types of
both difficult and slow. We introduce ZMap, a modular,
                                                              single-packet probes, including TCP SYN scans, ICMP
open-source network scanner specifically architected to
                                                              echo request scans, and application-specific UDP scans,
perform Internet-wide scans and capable of surveying
                                                              and it can interface easily with user-provided code to
the entire IPv4 address space in under 45 minutes from
                                                              perform follow-up actions on discovered hosts, such as
user space on a single machine, approaching the theo-
                                                              completing a protocol handshake.
retical maximum speed of gigabit Ethernet. We present
                                                                 Compared to Nmap—an excellent general-purpose net-
the scanner architecture, experimentally characterize its
                                                              work mapping tool, which was utilized in recent Internet-
performance and accuracy, and explore the security impli-
                                                              wide survey research [10, 14]—ZMap achieves much
cations of high speed Internet-scale network surveys, both
                                                              higher performance for Internet-scale scans. Experimen-
offensive and defensive. We also discuss best practices for
                                                              tally, we find that ZMap is capable of scanning the IPv4
good Internet citizenship when performing Internet-wide
                                                              public address space over 1300 times faster than the most
surveys, informed by our own experiences conducting a
                                                              aggressive Nmap default settings, with equivalent accu-
long-term research survey over the past year.
                                                              racy. These performance gains are due to architectural
                                                              choices that are specifically optimized for this application:
1   Introduction and Roadmap                                     Optimized probing While Nmap adapts its transmis-
                                                              sion rate to avoid saturating the source or target networks,
Internet-scale network surveys collect data by probing        we assume that the source network is well provisioned
large subsets of the public IP address space. While such      (unable to be saturated by the source host), and that the
scanning behavior is often associated with botnets and        targets are randomly ordered and widely dispersed (so
worms, it also has proved to be a valuable methodol-          no distant network or path is likely to be saturated by
ogy for security research. Recent studies have demon-         the scan). Consequently, we attempt to send probes as
strated that Internet-wide scanning can help reveal new       quickly as the source’s NIC can support, skipping the
kinds of vulnerabilities, monitor deployment of mitiga-       TCP/IP stack and generating Ethernet frames directly. We
tions, and shed light on previously opaque distributed        show that ZMap can send probes at gigabit line speed
ecosystems [10, 12, 14, 15, 25, 27]. Unfortunately, this      from commodity hardware and entirely in user space.
methodology has been more accessible to attackers than to        No per-connection state While Nmap maintains
legitimate researchers, who cannot employ stolen network      state for each connection to track which hosts have
access or spread self-replicating code. Comprehensively       been scanned and to handle timeouts and retransmis-
scanning the public address space with off-the-shelf tools    sions, ZMap forgoes any per-connection state. Since
like Nmap [23] requires weeks of time or many machines.       it is intended to target random samples of the address
   In this paper, we introduce ZMap, a modular and open-      space, ZMap can avoid storing the addresses it has already
source network scanner specifically designed for perform-     scanned or needs to scan and instead selects addresses
ing comprehensive Internet-wide research scans. A single      according to a random permutation generated by a cyclic
multiplicative group. Rather than tracking connection                2     ZMap: The Scanner
timeouts, ZMap accepts response packets with the cor-
rect state fields for the duration of the scan, allowing it          ZMap uses a modular design to support many types of
to extract as much data as possible from the responses it            probes and integration with a variety of research applica-
receives. To distinguish valid probe responses from back-            tions, as illustrated in Figure 1. The scanner core handles
ground traffic, ZMap overloads unused values in each                 command line and configuration file parsing, address gen-
sent packet, in a manner similar to SYN cookies [4].                 eration and exclusion, progress and performance monitor-
   No retransmission While Nmap detects connection                   ing, and reading and writing network packets. Extensible
timeouts and adaptively retransmits probes that are lost             probe modules can be customized for different kinds of
due to packet loss, ZMap (to avoid keeping state) always             probes, and are responsible for generating probe packets
sends a fixed number of probes per target and defaults               and interpreting whether incoming packets are valid re-
to sending only one. In our experimental setup, we esti-             sponses. Modular output handlers allow scan results to
mate that ZMap achieves 98% network coverage using                   be piped to another process, added directly to a database,
only a single probe per host, even at its maximum scan-              or passed on to user code for further action, such as com-
ning speed. We believe this small amount of loss will be             pleting a protocol handshake.
insignificant for typical research applications.                         We introduced the philosophy behind ZMap’s design in
   We further describe ZMap’s architecture and implemen-             Section 1. At a high level, one of ZMap’s most important
tation in Section 2, and we experimentally characterize              architectural features is that sending and receiving packets
its performance in Section 3. In Section 4, we investigate           take place in separate threads that act independently and
the implications of the widespread availability of fast,             continuously throughout the scan. A number of design
low-cost Internet-wide scanning for both defenders and               choices were made to ensure that these processes share as
attackers, and we demonstrate ZMap’s performance and                 little state as possible.
flexibility in a variety of security settings, including:                We implemented ZMap in approximately 8,900 SLOC
  Measuring protocol adoption, such as the transition                of C. It was written and tested on GNU/Linux.
      from HTTP to HTTPS. We explore HTTPS adoption
      based on frequent Internet-wide scans over a year.             2.1    Addressing Probes
  Visibility into distributed systems, such as the certificate       If ZMap simply probed every IPv4 address in numerical
      authority (CA) ecosystem. We collect and analyze               order, it would risk overloading destination networks with
      TLS certificates and identify misissued CA certs.              scan traffic and produce inconsistent results in the case of
  High-speed vulnerability scanning, which could allow at-           a distant transient network failure. To avoid this, ZMap
      tackers to widely exploit vulnerabilities within hours         scans addresses according to a random permutation of
      of their discovery. We build a UPnP scanner us-                the address space. To select smaller random samples of
      ing ZMap through which we find 3.4 million UPnP                the address space, we simply scan a subset of the full
      devices with known vulnerabilities [25].                       permutation.
  Uncovering unadvertised services, such as hidden Tor                  ZMap uses a simple and inexpensive method to traverse
      bridges. We show that ZMap can locate 86% of                   the address space, which lets it scan in a random permuta-
      hidden Tor bridges via comprehensive enumeration.              tion while maintaining only negligible state. We iterate
   High-speed scanning can be a powerful tool in the                 over a multiplicative group of integers modulo p, choos-
hands of security researchers, but users must be careful             ing p to be a prime slightly larger than 232 . By choosing
not to cause harm by inadvertently overloading networks              p to be a prime, we guarantee that the group is cyclic and
or causing unnecessary work for network administrators.              will reach all addresses in the IPv4 address space except
In Section 5, we discuss our experiences performing nu-              0.0.0.0 (conveniently an IANA reserved address) once per
merous large-scale scans over the past year, we report on            cycle. We choose to iterate over (Z/4, 294, 967, 311Z)× ,
the complaints and other reactions we have received, and             the multiplicative group modulo p for the smallest prime
we suggest several guidelines and best practices for good            larger than 232 : 232 + 15.
Internet citizenship while scanning.                                    To select a fresh random permutation for each scan,
   Internet-wide scanning has already shown great poten-             we generate a new primitive root of the multiplicative
tial as a research methodology [10, 12, 14, 25], and we              group and choose a random starting address. Because
hope ZMap will facilitate a variety of new applications by           the order of elements in a group is preserved by an iso-
drastically reducing the costs of comprehensive network              morphism, we efficiently find random primitive roots of
surveys and allowing scans to be performed with very fine            the multiplicative group by utilizing the isomorphism
time granularity. To facilitate this, we are releasing ZMap          (Z p−1 , +) ∼
                                                                                 = (Z∗p , ×) and mapping roots of (Z p−1 , +)
as an open source project that is documented and pack-               into the multiplicative group via the function f (x) = nx
aged for real world use. It is available at https://zmap.io/.        where n is a known primitive root of (Z/pZ)× . In our


                                                                 2
                                                                                 ZMap Internet Scanner

                                Validation Generation
                                                                     Packet                      Packet
                    State        Address Generation
                                                                    Generation                Transmission
         CLI         &
                   Config          Probe Scheduler

                                                          Framework Monitoring


                                                                     Response                   Receipt &
         Output Handler           Result Processing
                                                                   Interpretation               Validation



Figure 1: ZMap Architecture — ZMap is an open-source network scanner optimized for efficiently performing
Internet-scale network surveys. Modular packet generation and response interpretation components (blue) support
multiple kinds of probes, including TCP SYN scans and ICMP echo scans. Modular output handlers (red) allow users
to output or act on scan results in application-specific ways. The architecture allows sending and receiving components
to run asynchronously and enables a single source machine to comprehensively scan every host in the public IPv4
address space for a particular open TCP port in under 45 mins using a 1 Gbps Ethernet link.


specific case, we know that 3 is a primitive root of              used by routing tables [32, 34]. Excluded ranges can be
(Z/4, 294, 967, 311Z)× .                                          specified through a configuration file.
   Because we know that the generators of (Z p−1 , +) are
{s|(s, p − 1) = 1}, we can efficiently find the generators        2.2   Packet Transmission and Receipt
of the additive group by precalculating and storing the
factorization of p − 1 and checking addresses against the         ZMap is optimized to send probes as quickly as the
factorization at random until we find one that is coprime         source’s CPU and NIC can support. The packet genera-
with p − 1 and then map it into (Z∗p , ×). Given that there       tion component operates asynchronously across multiple
                                                                  threads, each of which maintains a tight loop that sends
exist approximately 109 generators, we expect to make
                                                                  Ethernet-layer packets via a raw socket.
four tries before finding a primitive root. While this pro-
                                                                     We send packets at the Ethernet layer in order to cache
cess introduces complexity at the beginning of a scan, it
                                                                  packet values and reduce unnecessary kernel overhead.
adds only a small amount of one-time overhead.
                                                                  For example, the Ethernet header, minus the packet check-
   Once a primitive root has been generated, we can easily
                                                                  sum, will never change during a scan. By generating and
iterate through the address space by applying the group
                                                                  caching the Ethernet layer packet, we prevent the Linux
operation to the current address (in other words, multi-
                                                                  kernel from performing a routing lookup, an arpcache
plying the current address by the primitive root modulo
                                                                  lookup, and netfilter checks for every packet. An addi-
232 + 15). We detect that a scan has completed when we
                                                                  tional benefit of utilizing a raw socket for TCP SYN scans
reach the initially scanned IP address. This technique
                                                                  is that, because no TCP session is established in the ker-
allows the sending thread to store the selected permuta-
                                                                  nel, upon receipt of a TCP SYN-ACK packet, the kernel
tion and progress through it with only three integers: the
                                                                  will automatically respond with a TCP RST packet, clos-
primitive root used to generate the multiplicative group,
                                                                  ing the connection. ZMap can optionally use multiple
the first scanned address, and the current address.
                                                                  source addresses and distribute outgoing probes among
Excluding Addresses Since ZMap is optimized for                   them in a round-robin fashion.
Internet-wide scans, we represent the set of targets as              We implement the receiving component of ZMap us-
the full IPv4 address space minus a set of smaller ex-            ing libpcap [17], a library for capturing network traffic
cluded address ranges. Certain address ranges need to be          and filtering the received packets. Although libpcap is
excluded for performance reasons (e.g., skipping IANA             a potential performance bottleneck, incoming response
reserved allocations [16]) and others to honor requests           traffic is a small fraction of outgoing probe traffic, since
from their owners to discontinue scanning. We efficiently         the overwhelming majority of hosts are unresponsive to
support address exclusion through the use of radix trees, a       typical probes, and we find that libpcap is easily capable
trie specifically designed to handle ranges and frequently        of handling response traffic in our tests (see Section 3).


                                                              3
Upon receipt of a packet, we check the source and des-             the active probe module. We chose to use the UMAC
tination port, discard packets clearly not initiated by the        function for these operations, based on its performance
scan, and pass the remaining packets to the active probe           guarantees [5]. In our TCP port scan module, we utilize
module for interpretation.                                         the source port and initial sequence number; for ICMP,
   While the sending and receiving components of ZMap              we use the ICMP identifier and sequence number. These
operate independently, we ensure that the receiver is ini-         fields are checked on packet receipt by the probe module,
tialized prior to sending probes and that the receiver con-        and ZMap discards any packets for which validation fails.
tinues to run for a period of time (by default, 8 seconds)            These inexpensive checks prevents the incorrect report-
after the sender has completed in order to process any             ing of spurious response packets due to background traffic
delayed responses.                                                 as well as responses triggered by previous scans. This
                                                                   design ultimately allows the receiver to validate responses
2.3   Probe Modules                                                while sharing only the scan secret and the initial configu-
                                                                   ration with the sending components.
ZMap probe modules are responsible for filling in the
body of probe packets and for validating whether incom-
ing packets are responsive to the probes. Making these             2.4   Output Modules
tasks modular allows ZMap to support a variety of prob-
ing methods and protocols and simplifies extensibility.            ZMap provides a modular output interface that allows
Out of the box, ZMap provides probe modules to support             users to output scan results or act on them in application-
TCP port scanning and ICMP echo scanning.                          specific ways. Output module callbacks are triggered
                                                                   by specific events: scan initialization, probe packet sent,
   At initialization, the scanner core provides an empty
                                                                   response received, regular progress updates, and scan ter-
buffer for the packet and the probe module fills in any
                                                                   mination. ZMap’s built-in output modules cover basic use,
static content that will be the same for all targets. Then,
                                                                   including simple text output (a file stream containing a list
for each host to be scanned, the probe module updates this
                                                                   of unique IP addresses that have the specified port open),
buffer with host-specific values. The probe module also
                                                                   extended text output (a file stream containing a list of all
receives incoming packets, after high-level validation by
                                                                   packet responses and timing data), and an interface for
the scanner core, and determines whether they are positive
                                                                   queuing scan results in a Redis in-memory database [29].
or negative responses to scan probes. Users can add new
scan types by implementing a small number of callback                 Output modules can also be implemented to trigger
functions within the probe module framework.                       network events in response to positive scan results, such
   For example, to facilitate TCP port scanning, ZMap im-          as completing an application-level handshake. For TCP
plements a probing technique known as SYN scanning or              SYN scans, the simplest way to accomplish this is to cre-
half-open scanning. We chose to implement this specific            ate a fresh TCP connection with the responding address;
technique instead of performing a full TCP handshake               this can be performed asynchronously with the scan and
based on the reduced number of exchanged packets. In               requires no special kernel support.
the dominant case where a host is unreachable or does
                                                                   forge_socket Some ZMap users may wish to complete
not respond, only a single packet is used (a SYN from
                                                                   the TCP handshake begun during a TCP SYN scan and
the scanner); in the case of a closed port, two packets
                                                                   exchange data with the remote host without the extra over-
are exchanged (a SYN answered with a RST); and in the
                                                                   head of establishing a new connection. While the initial
uncommon case where the port is open, three packets are
                                                                   SYN/SYN-ACK exchange has established a connection
exchanged (a SYN, a SYN-ACK reply, and a RST from
                                                                   from the destination’s perspective, ZMap bypasses the
the scanner).
                                                                   local system’s TCP stack and as such the kernel does not
Checking Response Integrity ZMap’s receiving com-                  recognize the connection.
ponents need to determine whether received packets are                In order to allow the scanning host to communicate
valid responses to probes originating from the scanner             over ZMap-initiated TCP sessions, we implemented
or are part of other background traffic. Probe mod-                forge_socket, a kernel module that allows user processes
ules perform this validation by encoding host- and scan-           to pass in session parameters (e.g. initial sequence num-
invocation–specific data into mutable fields of each probe         ber) using setsockopt. This allows application-level hand-
packet, utilizing fields that will have recognizable effects       shakes to be performed using the initial ZMap handshake
on fields of the corresponding response packets in a man-          and does not require the unnecessary transmission of a
ner similar to SYN cookies [4].                                    RST, SYN, or SYN-ACK packet that would be required
   For each scanned host, ZMap computes a MAC of the               to close the existing connection and initiate a new kernel-
destination address keyed by a scan-specific secret. This          recognized session. We are releasing forge_socket along
MAC value is then spread across any available fields by            with ZMap.


                                                               4
3     Validation and Measurement                                                           1.02
                                                                                                                                                            Hitrate
                                                                                           1.01
We performed a series of experiments to characterize the
                                                                                                 1
performance of ZMap. Under our test setup, we find


                                                                    Hit Rate (percent)
                                                                                           0.99
that a complete scan of the public IPv4 address space
takes approximately 44 minutes on an entry-level server                                    0.98

with a gigabit Ethernet connection. We estimate that                                       0.97

a single-packet scan can detect approximately 98% of                                       0.96
instantaneously listening hosts, and we measure a 1300 x                                   0.95
performance improvement over Nmap for Internet-wide
                                                                                           0.94
scanning, with equivalent coverage.                                                                            00
                                                                                                            00 0
                                                                                                         15 00
                                                                                                            00 0
                                                                                                         14 00
                                                                                                            00 0
                                                                                                         13 00
                                                                                                            00 0
                                                                                                         12 00
                                                                                                            00 0
                                                                                                         11 00
                                                                                                            00
                                                                                                         10 00
                                                                                                            00
                                                                                                                 0
                                                                                                         75 00
                                                                                                            00
   We performed the following measurements on an HP
                                                                                                         50 00
                                                                                                            00
                                                                                                         25 00
                                                                                                            00
                                                                                                         10 0
                                                                                                            00
                                                                                                         50 0
                                                                                                         10 00
                                                                                                         25 0
                                                                                                         50 00
                                                                                                            00
                                                                                                            00
                                                                                                         25
                                                                                                         10 00
ProLiant DL120 G7 with a Xeon E3-1230 3.2 GHz pro-                                                                      Scan Rate (packets per second)

cessor and 4 GB of memory running a stock install of
                                                                  Figure 2: Hit rate vs. Scan rate — We find no correla-
Ubuntu 12.04.1 LTS and the 3.2.0-32-generic Linux ker-
                                                                  tion between hit rate (positive responses/hosts probed)
nel. Experiments were conducted using the onboard NIC,
                                                                  and scan rate (probes sent/second). Shown are means and
which is based on the Intel 82574L chipset and uses the
                                                                  standard deviations over ten trials. This indicates that
stock e1000e network driver, or a quad-port Intel Ethernet
                                                                  slower scanning does not reveal additional hosts.
adapter based on the newer Intel 82580 chipset and using
the stock igb network driver. For experiments involving
                                                                                         89000
complete TCP handshakes, we disabled kernel modules                                                                                                       Hosts Found

used by iptables and conntrack. Experiments comparing                                    88500

ZMap with Nmap were conducted with Nmap 5.21.                                            88000
   These measurements were conducted using the normal


                                                                    Unique Hosts Found
                                                                                         87500
building network at the University of Michigan Computer
Science & Engineering division. We used a gigabit Eth-                                   87000

ernet uplink (a standard office network connection in our                                86500
facility); we did not arrange for any special network con-
                                                                                         86000
figuration beyond static IP addresses. The access layer
of the building runs at 10 gbps, and the building uplink                                 85500
to the rest of the campus is an aggregated 2 × 10 gigabit                                85000
port channel. We note that ZMap’s performance on other                                               0             5          10       15        20               25            30
                                                                                                                            Unique SYN Packets Sent
source networks may be worse than reported here due to
local congestion.                                                 Figure 3: Coverage for Multiple Probes — Discovered
                                                                  hosts plateau after ZMap sends about 8 SYNs to each. If
3.1    Scan Rate: How Fast is Too Fast?                           this plateau represents the true number of listening hosts,
                                                                  sending just 1 SYN will achieve about 98% coverage.
In order to determine whether our scanner and our up-
stream network can handle scanning at gigabit line speed,
we examine whether the scan rate, the rate at which ZMap                                 1.02
sends probe packets, has any effect on the hit rate, the
fraction of probed hosts that respond positively (in this                                  1
case, with a SYN-ACK). If libpcap, the Linux kernel, our
institutional network, or our upstream provider are unable                               0.98

to adequately handle the traffic generated by the scanner          Hitrate
                                                                                         0.96
at full speed, we would expect packets to be dropped and
the hit rate to be lower than at slower scan rates.                                      0.94
   We experimented by sending TCP SYN packets to
random 1% samples of the IPv4 address space on port                                      0.92
443 at varying scan rates. We conducted 10 trials at each
                                                                                                 22       00      2:   4:    6:    8:    10    12    14    16     18    20     22
of 16 scan rates ranging from 1,000 to 1.4 M packets per                                             :0  0   :0
                                                                                                             0    00   00     00    00    :0
                                                                                                                                          0
                                                                                                                                  Time of Day
                                                                                                                                                :0
                                                                                                                                                0     :0
                                                                                                                                                      0     :00    :0 0   :0
                                                                                                                                                                          0     :0
                                                                                                                                                                                 0
second. The results are shown in Figure 2.
   We find no statistically significant correlation between       Figure 4: Diurnal Effect on Hosts Found — We ob-
scan rate and hit rate. This shows that our ZMap setup            served a ±3.1% variation in ZMap’s hit rate depending
is capable of handling scanning at 1.4 M packets per              on the time of day the scan was performed. (Times EST.)

                                                              5
second and that scanning at lower rates provides no ben-            3.3   Variation by Time of Day
efit in terms of identifying additional hosts. From an
architectural perspective, this validates that our receiving        In previous work, Internet-wide scans took days to months
infrastructure based on libpcap is capable of processing            to execute, so there was little concern over finding the
responses generated by the scanner at full speed and that           optimal time of day to perform a scan. However, since
kernel modules such as PF_RING [8] are not necessary                ZMap scans can take less than an hour to complete, the
for gigabit-speed network scanning.                                 question as to the “right time” to perform a scan arises.
                                                                    Are there certain hours of the day or days of the week that
                                                                    are more effective for scanning than others?
3.2   Coverage: Is One SYN Enough?                                     In order to measure any diurnal effects on scanning, we
While scanning at higher rates does not appear to result            performed continuous scans of TCP port 443 targeting a
in a lower hit rate, this does not tell us what coverage we         random 1% sample of the Internet over a 24-hour period.
achieve with a single scan—what fraction of target hosts            Figure 4 shows the number of hosts found in each scan.
does ZMap actually find using its default single-packet                We observed a ±3.1% variation in hit rate dependent
probing strategy?                                                   on the time of day scans took place. The highest response
                                                                    rates were at approximately 7:00 AM EST and the lowest
   Given the absence of ground truth for the number of
                                                                    response rates were at around 7:45 PM EST.
hosts on the Internet with a specific port open, we cannot
measure coverage directly. This is further complicated                 These effects may be due to variation in overall net-
by the ever changing state of the Internet; it is inherently        work congestion and packet drop rates or due to a diurnal
difficult to detect whether a host was not included in a            pattern in the aggregate availability of end hosts that are
scan because it was not available at the time or because            only intermittently connected to the network. In less for-
packets were dropped between it and the scanner. Yet, this          mal testing, we did not notice any obvious variation by
question is essential to understanding whether performing           day of the week or day of the month.
fast, single-packet scans is an accurate methodology for
Internet-wide surveys.                                              3.4   Comparison with Nmap
   To characterize ZMap’s coverage, we estimate the num-            We performed several experiments to compare ZMap to
ber of hosts that are actually listening by sending multiple,       Nmap in Internet-wide scanning applications, focusing on
distinct SYN packets to a large address sample and ana-             coverage and elapsed time to complete a scan. Nmap and
lyzing the distribution of the number of positive responses         ZMap are optimized for very different purposes. Nmap is
received compared to the number of SYNs we send. We                 a highly flexible, multipurpose tool that is frequently used
expect to eventually see a plateau in the number of hosts           for probing a large number of open ports on a smaller
that respond regardless of the number of additional SYNs            number of hosts, whereas ZMap is optimized to probe a
we send. If this plateau exists, we can treat it as an esti-        single port across very large numbers of targets. We chose
mate of the real number of listening hosts, and we can              to compare the two because recent security studies used
use it as a baseline against which to compare scans with            Nmap for Internet-wide surveys [10, 14], and because,
fewer SYN packets.                                                  like ZMap, Nmap operates from within user space on
   We performed this experiment by sending 1, 2, 5, 8, 10,          Linux [23].
15, 20, and 25 SYN packets to random 1% samples of the                 We tested a variety of Nmap settings to find reasonable
IPv4 address space on port 443 and recording the number             configurations to compare. All performed a TCP SYN
of distinct hosts that sent SYN-ACK responses in each               scan on port 443 (-Ss -p 443). Nmap provides several
scan. The results indicate a clear plateau in the number of         defaults known as timing templates, but even with the
responsive hosts after sending 8 SYN packets, as shown              most aggressive of these (labeled “insane”), an Internet-
in Figure 3.                                                        wide scan would take over a year to complete. To make
   Based on the level of this plateau, we estimate that our         Nmap scan faster in our test configurations, we started
setup reaches approximately 97.9% of live hosts using               with the “insane” template (-T5), disabled host discovery
a single packet, 98.8% of hosts using two packets, and              and DNS resolutions (-Pn -n), and set a high minimum
99.4% of hosts using three packets. The single packet               packet rate (--min-rate 10000). The “insane” template
round-trip loss rate of about 2% is in agreement with pre-          retries each probe once after a timeout; we additionally
vious studies on random packet drop on the Internet [12].           tested a second Nmap configuration with retries disabled
   These results suggest that single-probe scans are suffi-         (--max-retries 0).
ciently comprehensive for typical research applications.               We used ZMap to select a random sample of 1 million
Investigators who require higher coverage can configure             IP addresses and scanned them for hosts listening on
ZMap to send multiple probes per host, at the cost of               port 443 with Nmap in the two configurations described
proportionally longer running scans.                                above and with ZMap in its default configuration and in a


                                                                6
                                                                            Coverage           Duration    Est. Time for
                                          Scan Type                         (normalized)       (mm:ss)     Internet-wide Scan
                                          Nmap, max 2 probes (default)      0.978              45:03       116.3 days
                                          Nmap, 1 probe                     0.814              24:12       62.5 days
                                          ZMap, 2 probes                    1.000              00:11       2:12:35
                                          ZMap, 1 probe (default)           0.987              00:10       1:09:45

Table 1: ZMap vs. Nmap Comparison — We scanned 1 million hosts on TCP port 443 using ZMap and Nmap and
averaged over 10 trials. Despite running hundreds of times faster, ZMap finds more listening hosts than Nmap, due to
Nmap’s low host timeout. Times for ZMap include a fixed 8 second delay to wait for responses after the final probe.


second configuration that sends two SYN probes to each                                  connection attempt to timeout on Linux. 99% of hosts
host (-P 2). We repeated this process for 10 trials over a                              that responded within 500 seconds did so within about
12 hour period and report the averages in Table 1.                                      1 second, and 99.9% responded within 8.16 seconds.
   The results show that ZMap scanned much faster than                                     As ZMap’s receiving code is stateless with respect to
Nmap and found more listening hosts than either Nmap                                    the sending code, a valid SYN-ACK that comes back
configuration. The reported durations for ZMap include                                  any time before the scan completes will be recorded as
time sent sending probes as well as a fixed 8-second delay                              a listening host. To assure a high level of coverage, the
after the sending process completes, during which ZMap                                  default ZMap settings incorporate an empirically derived
waits for late responses. Extrapolating to the time re-                                 8-second delay after the last probe is sent before the re-
quired for an Internet-wide scan, the fastest tested ZMap                               ceiving process terminates.
configuration would complete approximately 1300 times                                      In contrast, Nmap maintains timeouts for each probe.
faster than the fastest Nmap configuration.1                                            In the Nmap “insane” timing template we tested, the
Coverage and Timeouts To investigate why ZMap                                           timeout is initially 250 ms, by which time fewer than 85%
achieved higher coverage than Nmap, we probed a ran-                                    of responsive hosts in our test had responded. Over the
dom sample of 4.3 million addresses on TCP port 80 and                                  course of a scan, Nmap’s timeout can increase to 300 ms,
measured the latency between sending a SYN and receiv-                                  by which time 93.2% had responded. Thus, we would
ing a SYN-ACK from responsive hosts. Figure 5 shows                                     expect a single-probe Nmap scan with these timing values
the CDF of the results. The maximum round-trip time                                     to see 85–93% of the hosts that ZMap finds, which is
was 450 seconds, and a small number of hosts took more                                  roughly in line with the observed value of 82.5%.
than 63 seconds to respond, the time it takes for a TCP                                    With Nmap’s “insane” defaults, it will attempt to send a
                                                                                        second probe after a timeout. A response to either the first
   1 The extrapolated 1-packet Internet-wide scan time for ZMap is
                                                                                        or second SYN will be considered valid until the second
longer than the 44 minutes we report elsewhere for complete scans,
because this test used a slower NIC based on the Intel 82574L chipset.
                                                                                        times out, so this effectively raises the overall timeout to
                                                                                        500–600 ms, by which time we received 98.2–98.5% of
                                                                                        responses. Additional responses will likely be generated
                          1.0                                                           by the second SYN. We observed that the 2-probe Nmap
                                                                                        scan found 99.1% of the number of hosts that a 1-probe
                          0.8                                                           ZMap scan found.




CDF of responding hosts
                          0.6                                                           3.5   Comparison with Previous Studies
                                                                                        Several groups have previously performed Internet-wide
                          0.4
                                                                                        surveys using various methodologies. Here we compare
                                                                                        ZMap to two recent studies that focused on HTTPS cer-
                          0.2
                                                                                        tificates. Most recently, Heninger et al. performed a dis-
                                                                                        tributed scan of port 443 in 2011 as part of a global
                          0.0
                                0   0.2         0.4          0.6      0.8   1           analysis on cryptographic key generation [14]. Their
                                            response time (seconds)                     scan used Nmap on 25 Amazon EC2 instances and re-
                                                                                        quired 25 hours to complete, with a reported average of
Figure 5: SYN to SYN-ACK time — In an experiment                                        40,566 hosts scanned per second. A 2010 scan by the EFF
that probed 4.3 million hosts, 99% of SYN-ACKs arrived                                  SSL Observatory project used Nmap on 3 hosts and took
within about 1 second and 99.9% within 8.16 seconds.                                    3 months to complete [10].


                                                                                    7
       Scan                                 Date        Port 443 Open         TLS Servers     All Certs     Trusted Certs
       EFF SSL Observatory [10]             2010/12                 16.2 M          7.7 M         4.0 M           1.46 M
       Mining Ps and Qs [14]                2011/10                 28.9 M         12.8 M         5.8 M           1.96 M
       ZMap + certificate fetcher           2012/06                 31.8 M         19.0 M         7.8 M           2.95 M
       ZMap + certificate fetcher           2013/05                 34.5 M         22.8 M         8.6 M           3.27 M

Table 2: Comparison with Prior Internet-wide HTTPS Surveys — Due to growth in HTTPS deployment, ZMap
finds almost three times as many TLS servers as the SSL Observatory did in late 2010, yet this process takes only
10 hours to complete from a single machine using a ZMap-based workflow, versus three months on three machines.


    To compare ZMap’s performance for this task, we used                  Organization                    Certificates
it to conduct comprehensive scans of port 443 and used
a custom certificate fetcher based on libevent [24] and                   GoDaddy.com, Inc.           913,416     (31.0%)
OpenSSL [37] to retrieve TLS certificates from each re-                   GeoTrust Inc.               586,376     (19.9%)
sponsive host. With this methodology, we were able to                     Comodo CA Limited           374,769     (12.7%)
discover hosts, perform TLS handshakes, and collect and                   VeriSign, Inc.              317,934     (10.8%)
parse the resulting certificates in under 10 hours from a                 Thawte, Inc.                228,779      (7.8%)
single machine.                                                           DigiCert Inc                145,232      (4.9%)
                                                                          GlobalSign                  117,685      (4.0%)
    As shown in Table 2, we find significantly more TLS
                                                                          Starfield Technologies       94,794      (3.2%)
servers than previous work—78% more than Heninger
                                                                          StartCom Ltd.                88,729      (3.0%)
et al. and 196% more than the SSL Observatory—likely
                                                                          Entrust, Inc.                76,929      (2.6%)
due to increased HTTPS deployment since those studies
were conducted. Linear regression shows an average
growth in HTTPS deployment of about 540,000 hosts                   Table 3: Top 10 Certificate Authorities — We used
per month over the 29 month period between the SSL                  ZMap to perform regular comprehensive scans of HTTPS
Observatory scan and our most recent dataset. Despite               hosts in order gain visibility into the CA ecosystem. Ten
this growth, ZMap is able to collect comprehensive TLS              organizations control 86% of browser trusted certificates.
certificate data in a fraction of the time and cost needed
in earlier work. The SSL Observatory took roughly 650
times as much machine time to acquire the same kind of              viously opaque distributed systems on the Internet. For
data, and Heninger et al. took about 65 times as much.              instance, e-commerce and secure web transactions inher-
                                                                    ently depend on browser trusted TLS certificates. How-
                                                                    ever, there is currently little oversight over browser trusted
4     Applications and Security Implications                        certificate authorities (CAs) or issued certificates. Most
                                                                    CAs do not publish lists of the certificates they have
The ability to scan the IPv4 address space in under an hour         signed, and, due to delegation of authority to interme-
opens an array of new research possibilities, including the         diate CAs, it is unknown what set of entities have the
ability to gain visibility into previously opaque distributed       technical ability to sign browser-trusted certificates at any
systems, understand protocol adoption at a new resolution,          given time.
and uncover security phenomenon only accessible with a
                                                                       To explore this potential, we used ZMap and our cus-
global perspective [14]. However, high-speed scanning
                                                                    tom certificate fetcher to conduct regular scans over the
also has potentially malicious applications, such as find-
                                                                    past year and perform analysis on new high-profile certifi-
ing and attacking vulnerable hosts en masse. Furthermore,
                                                                    cates and CA certificates. Between April 2012 and June
many developers have the preconceived notion that the
                                                                    2013, we performed 1.81 billion TLS handshakes, ulti-
Internet is far too large to be fully enumerated, so the re-
                                                                    mately collecting 33.6 million unique X.509 certificates
ality of high speed scanning may disrupt existing security
                                                                    of which 6.2 million were browser trusted. We found and
models, such as by leading to the discovery of services
                                                                    processed an average of 220,000 new certificates, 15,300
previously thought to be well hidden. In this section, we
                                                                    new browser trusted certificates, and 1.2 new CA certifi-
use ZMap to explore several of these applications.
                                                                    cates per scan. In our most recent scan, we identified
                                                                    1,832 browser trusted signing certificates from 683 orga-
4.1    Visibility into Distributed Systems
                                                                    nizations and 57 countries. We observed 3,744 distinct
High-speed network scanning provides researchers with               browser-trusted signing certificates in total. Table 3 shows
the possibility for a new real-time perspective into pre-           the most prolific CAs by leaf certificates issued.


                                                                8
   Wide-scale visibility into CA behavior can help to                                                                          Port    Service             Hit Rate (%)
identify security problems [10, 18]. We found two
cases of misissued CA certificates. In the first case,                                                                          80     HTTP                         1.77
we found a CA certificate that was accidentally issued                                                                        7547     CWMP                         1.12
to a Turkish transit provider. This certificate, C=TR,                                                                         443     HTTPS                        0.93
ST=ANKARA, L=ANKARA, O=EGO, OU=EGO BILGI                                                                                        21     FTP                          0.77
ISLEM, CN=*.EGO.GOV.TR, was later found by Google                                                                               23     Telnet                       0.71
after being used to sign a Google wildcard certificate and                                                                      22     SSH                          0.57
has since been revoked and blacklisted in common web                                                                            25     SMTP                         0.43
browsers [20].                                                                                                                3479     2-Wire RPC                   0.42
   In the second case, we found approximately 1,300                                                                           8080     HTTP-alt/proxy               0.38
CA certificates that were misissued by the Korean Gov-                                                                          53     DNS                          0.38
ernment to government sponsored organizations such as
schools and libraries. While these certificates had been                                                              Table 4: Top 10 TCP ports — We scanned 2.15 million
issued with rights to sign additional certificates, a length                                                          hosts on TCP ports 0–9175 and observed what fraction
constraint on the grandparent CA certificate prevented                                                                were listening on each port. We saw a surprising number
these organizations from signing new certificates. We                                                                 of open ports associated with embedded devices, such as
do not include these Korean certificates in the CA to-                                                                ports 7547 (CWMP) and 3479 (2-Wire RPC).
tals above because they are unable to sign valid browser-
trusted certificates.                                                                                                    To illustrate this application, we tracked the adoption of
                                                                                                                      HTTPS using 158 Internet-wide scans over the past year.
4.2                           Tracking Protocol Adoption                                                              Notably, we find a 23% increase in HTTPS use among
Researchers have previously attempted to understand the                                                               Alexa Top 1 Million websites and a 10.9% increase in
adoption of new protocols, address depletion, common                                                                  the number of browser-trusted certificates. During this
misconfigurations, and vulnerabilities through active scan-                                                           period, the Netcraft Web Survey [26] finds only a 2.2%
ning [2, 10, 12, 14, 15, 27]. In many of these cases, these                                                           increase in the number of HTTP sites, but we observe an
analyses have been performed on random samples of the                                                                 8.5% increase in sites using HTTPS. We plot these trends
IPv4 address space due to the difficulty of performing                                                                in Figure 6.
comprehensive scans [15, 27]. In cases where full scans                                                                  We can also gain instantaneous visibility into the de-
were performed, they were completed over an extended                                                                  ployment of multiple protocols by performing many
period of time or through massive parallelization on cloud                                                            ZMaps scans of different ports. We scanned 0.05% sam-
providers [10, 14]. ZMap lowers the barriers to entry and                                                             ples of the IPv4 address space on each TCP port below
allows researchers to perform studies like these in a com-                                                            9175 to determine the percentage of hosts that were lis-
prehensive and timely manner, ultimately enabling much                                                                tening on each port. This experiment requires the same
higher resolution measurements than previously possible.                                                              number of packets as over 5 Internet-wide scans of a sin-
                                                                                                                      gle port, yet we completed it in under a day using ZMap.
                                                                                                                      Table 4 shows the top 10 open ports we observed.
                       1.25
                                        HTTPS Hosts
                                  Unique Certificates
                        1.2       Trusted Certificates                                                                4.3   Enumerating Vulnerable Hosts
                            Alexa Top 1 Mil. Domains
                                     E.V. Certificates                                                                With the ability to perform rapid Internet-wide scans


Trusted Certificates
                       1.15     Netcraft HTTP Hosts
                                                                                                                      comes the potential to quickly enumerate hosts that suf-
                        1.1                                                                                           fer from specific vulnerabilities [2]. While this can be a
                                                                                                                      powerful defensive tool for researchers—for instance, to
                       1.05                                                                                           measure the severity of a problem or to track the appli-
                                                                                                                      cation of a patch—it also creates the possibility for an
                          1
                                                                                                                      attacker with control of only a small number of machines
                       0.95
                                                                                                                      to scan for and infect all public hosts suffering from a new
                           06
                              /12
                                    07
                                     /12
                                           08
                                            /12
                                                  09
                                                   /12
                                                         10
                                                          /12
                                                                11
                                                                  /12
                                                                        12
                                                                         /12
                                                                               01
                                                                                /13
                                                                                      02
                                                                                       /13
                                                                                             03
                                                                                              /13
                                                                                                    04
                                                                                                     /13
                                                                                                           05
                                                                                                            /13       vulnerability within minutes.
                                                                Scan Date                                             UPnP Vulnerabilities To explore these applications,
                                                                                                                      we investigated several recently disclosed vulnerabilities
Figure 6: HTTPS Adoption — Data we collected using                                                                    in common UPnP frameworks. On January 29, 2013,
ZMap show trends in HTTPS deployment over one year.                                                                   HD Moore publicly disclosed several vulnerabilities in
We observed 19.6% growth in hosts serving HTTPS.                                                                      common UPnP libraries [25]. These vulnerabilities ulti-


                                                                                                                  9
mately impacted 1,500 vendors and 6,900 products, all of                                    Percentage of Certificates using Factorable RSA Keys
                                                                              0.62
which can be exploited to perform arbitrary code execu-                        0.6
tion with a single UDP packet. Moore followed responsi-                       0.58

ble disclosure guidelines and worked with manufacturers                       0.56
                                                                              0.54
to patch vulnerable libraries, and many of the libraries
                                                                              0.52
had already been patched at the time of disclosure. De-
                                                                                              Percentage of Certificates using Debian Weak Keys
spite these precautions, we found that at least 3.4 million                   0.06
devices were still vulnerable to the problem in February
                                                                              0.05
2013.
   To measure this, we created a custom ZMap probe mod-                       0.04


ule that performs a UPnP discovery handshake. We were                         0.03

able to develop this 150-SLOC module from scratch in
                                                                                             Browser Trusted Certificates with Debian Weak Keys
approximately four hours and performed a comprehen-                           140

sive scan of the IPv4 address space for publicly available                    130

                                                                              120
UPnP hosts on February 11, 2013, which completed in
                                                                              110
under two hours. This scan found 15.7 million publicly                        100
accessible UPnP devices, of which 2.56 million (16.5%)                         90

were running vulnerable versions of the Intel SDK for
                                                                                              Browser Trusted Certificates with Shared Citrix Key
UPnP Devices, and 817,000 (5.2%) used vulnerable ver-
                                                                               15
sions of MiniUPnPd.2
   Given that these vulnerable devices can be infected                         10

with a single UDP packet [25], we note that these 3.4 mil-
                                                                                5
lion devices could have been infected in approximately
the same length of time—much faster than network oper-                                      Browser Trusted Certificates with Factorable RSA Key
ators can reasonably respond or for patches to be applied                       5

to vulnerable hosts. Leveraging methodology similar to                          4
ZMap, it would only have taken a matter of hours from
                                                                                3
the time of disclosure to infect every publicly available
vulnerable host.                                                                2



Weak Public Keys As part of our regular scans of                                     07 /      09 /        11 /        01 /        03 /        05 /
                                                                                       12        12          12           13         13             13
the HTTPS ecosystem, we tracked the mitigation of the
                                                                                                              Scan Date
2008 Debian weak key vulnerability [3] and the weak and
shared keys described by Heninger et al. in 2012 [14].                        Figure 7: Trends in HTTPS Weak Key Usage — To
Figure 7 shows several trends over the past year.                             explore how ZMap can be used to track the mitigation
   In our most recent scan, we found that 44,600 unique                       of known vulnerabilities, we monitored the use of weak
certificates utilized factorable RSA keys and are served                      HTTPS public keys from May 2012 through June 2013.
on 51,000 hosts, a 20% decrease from 2011 [14]. Four
of these certificates were browser trusted; the last was
signed in August 2012. Similarly, we found 2,743 unique                       4.4    Discovering Unadvertised Services
certificates that contained Debian weak keys, of which
96 were browser trusted, a 34% decrease from 2011 [14].                       The ability to perform comprehensive Internet scans im-
The last browser trusted certificate containing a Debian                      plies the potential to uncover unadvertised services that
weak key was signed in January 2012. We also observed                         were previously only accessible with explicit knowledge
a 67% decrease in the number of browser-trusted certifi-                      of the host name or address. For example, Tor bridges
cates that contained default public keys used for Citrix                      are intentionally not published in order to prevent ISPs
remote access products [14].                                                  and government censors from blocking connections to
   We created an automated process that alerts us to the                      the Tor network [35]. Instead, the Tor Project provides
discovery of new browser-trusted certificates containing                      users with the IP addresses of a small number of bridges
factorable RSA keys, Debian weak keys, or default Citrix                      based on their source address. While Tor developers have
keys as soon as they are found, so that we can attempt to                     acknowledged that bridges can in principle be found by
notify the certificate owners about the vulnerability.                        Internet-wide scanning [9], the set of active bridges is con-
    2 Moore reported many more UPnP hosts [25] but acknowledges that          stantly changing, and the data would be stale by the time
his scans occurred over a 5 month period and did not account for hosts        a long running scan was complete. However, high-speed
being counted multiple times due to changing IP addresses.                    scanning might be used to mount an effective attack.


                                                                         10
   To confirm this, we performed Internet wide-scans on              4.6   Privacy and Anonymous Communication
ports 443 and 9001, which are common ports for Tor
bridges and relays, and applied a set of heuristics to iden-         The advent of comprehensive high-speed scanning raises
tify likely Tor nodes. For hosts with one of these ports             potential new privacy threats, such as the possibility of
open, we performed a TLS handshake using a specific                  tracking user devices between IP addresses. For instance,
set of cipher suites supported by Tor’s “v1 handshake.”              a company could track home Internet users between dy-
When a Tor relay receives this set of cipher suites, it will         namically assigned IP addresses based on the HTTPS cer-
respond with a two-certificate chain. The signing (“Certi-           tificate or SSH host key presented by many home routers
fiate Authority”) certificate is self-signed with the relay’s        and cable modems. This would allow tracking companies
identity public key and uses a subject name of the form              to extend existing IP-based tracking beyond the length of
“CN=www.X.com”, where X is a randomized alphanu-                     DHCP leases.
meric string. This pattern matched 67,342 hosts on port                  In another scenario, it may be possible to track travelers.
443, and 2,952 hosts on port 9001.                                   In 2006 Scholz et al. presented methods for fingerprinting
   We calculated each host’s identity fingerprint and                SIP devices [30] and other protocols inadvertently expose
checked whether the SHA1 hash appeared in the pub-                   unique identifiers such as cryptographic keys. Such fea-
lic Tor metrics list for bridge pool assignments. Hosts we           tures could be used to follow a specific mobile host across
found matched 1,170 unique bridge fingerprints on port               network locations. These unique fingerprints, paired with
443 and 419 unique fingerprints on port 9001, with a com-            publicly available network data and commercial geoloca-
bined total of 1,534 unique fingerprints (some were found            tion databases, could allow an attacker to infer relation-
on both ports). From the bridge pool assignment data, we             ships and travel patterns of a specific individual.
see there have been 1,767–1,936 unique fingerprints allo-                The ability to rapidly send a single packet to all IPv4 ad-
cated at any given time in the recent past, which suggests           dresses could provide the basis for a system of anonymous
that we were able to identify 79–86% of allocated bridges            communication. Rather than using the scanner to send
at the time of the scan. The unmatched fingerprints in               probes, it could be used to broadcast a short encrypted
the Tor metrics list may correspond to bridges we missed,            message to every public IP address. In this scenario, it
offline bridges, or bridges configured to use a port other           would be impossible to determine the desired destination
than 9001 or 443.                                                    host. If the sender is on a network that does not use
   In response to other discovery attacks against Tor                ingress filtering, it could also spoof source addresses to
bridges [38], the Tor project has started to deploy obfs-            obscure the sender’s identity. This style of communica-
proxy [36], a wrapper that disguises client–bridge con-              tion could be of particular interest to botnet operators,
nections as random data in order to make discovery by                because it would allow infected hosts to remain dormant
censors more difficult. Obfsproxy nodes listen on random-            indefinitely while waiting for instructions, instead of pe-
ized ports, which serves as a defense against discovery by           riodically checking in with command and control infras-
comprehensive scanning.                                              tructure and potentially revealing their existence.


4.5   Monitoring Service Availability
Active scanning can help identify Internet outages and
disruptions to service availability without an administra-
tive perspective. Previous studies have shown that active
surveying (ICMP echo request scans) can help track In-
ternet outages, but they have either scanned small subsets
of the address space based on preconceived notions of
where outages would occur or have performed random
sampling [9, 13, 31]. High speed scanning allows scans to
be performed at a high temporal resolution through sam-
pling or comprehensively. Similarly, scanning can help
service providers identify networks and physical regions
that have lost access to their service.                              Figure 8: Outages in the Wake of Hurricane Sandy —
   In order to explore ZMap’s potential for tracking ser-            We performed scans of port 443 across the entire IPv4
vice availability, we performed continuous scans of the              address space every 2 hours from October 29–31, 2013
IPv4 address space during Hurricane Sandy to track its               to track the impact of Hurricane Sandy on the East Coast
impact on the East Coast of the United States. We show a             of the United States. Here, we show locations with more
snapshot of outages caused by the hurricane in Figure 8.             than a 30% decrease in the number of listening hosts.


                                                                11
5    Scanning and Good Internet Citizenship                           1. Coordinate closely with local network admins to
                                                                         reduce risks and handle inquiries.
We worked with senior colleagues and our local network                2. Verify that scans will not overwhelm the local net-
administrators to consider the ethical implications of high-             work or upstream provider.
speed Internet-wide scanning and to develop a series of               3. Signal the benign nature of the scans in web pages
guidelines to identify and reduce any risks. Such scan-                  and DNS entries of the source addresses.
ning involves interacting with an enormous number of                  4. Clearly explain the purpose and scope of the scans
hosts and networks worldwide. It would be impossible                     in all communications.
to request permission in advance from the owners of all               5. Provide a simple means of opting out, and honor
these systems, and there is no IP-level equivalent of the                requests promptly.
HTTP robots exclusion standard [19] to allow systems                  6. Conduct scans no larger or more frequent than is
to signal that they desire not to be scanned. If we are to               necessary for research objectives.
perform such scanning at all, the most we can do is try to            7. Spread scan traffic over time or source addresses
minimize any potential for harm and give traffic recipients              when feasible.
the ability to opt out of further probes.
   High-speed scanning uses a large amount of bandwidth,            Table 5: Recommended Practices — We offer these sug-
so we need to ensure that our activities do not cause ser-          gestions for other researchers conducting fast Internet-
vice degradation to the source or target networks. We               wide scans as guidelines for good Internet citizenship.
confirmed with our local network administrators that our
campus network and upstream provider had sufficient
capacity for us to scan at gigabit speeds. To avoid over-           fic from these hosts was part of an academic research
whelming destination networks, we designed ZMap to                  study. Third, we coordinated with IT teams at our institu-
scan addresses according to a random permutation. This              tion who might receive inquiries about our scan traffic.
spreads out traffic to any given destination network across            For our ongoing Internet-wide HTTPS surveys (our
the length of the scan. In a single probe TCP scan, an              largest-volume scanning effort), we took additional steps
individual destination address receives one 40 byte SYN             to further reduce the rate of false alarms from intrusion
packet. If we scan at full gigabit speed, each /24 network          detection systems. Rather than scanning at full speed,
block will receive a packet about every 10.6 seconds (3.8           we conducted each of these scans over a 12 hour period.
bytes/s), each /16 network every 40 ms (1000 bytes/s),              We also configured ZMap to use a range of 64 source
and each /8 network every 161 µs (250,000 bytes/s) for              addresses and spread out probe traffic among them. We
the 44 minute duration of the scan. These traffic volumes           recognize that there is a difficult balance to strike here:
should be negligible for networks of these sizes.                   we do not want to conceal our activities from system
   Despite these precautions, there is a small but nonzero          administrators who would want to know about them, but
chance that any interaction with remote systems might               we also do not want to divert IT support resources that
cause operational problems. Moreover, users or network              would otherwise be spent dealing with genuine attacks.
administrators who observe our scan traffic might be                   We provide a summary of the precautions we took in Ta-
alarmed, in the mistaken belief that they are under at-             ble 5 as a starting point for future researchers performing
tack. Many may be unable to recognize that their systems            Internet-wide scans. It should go without saying that scan
are not being uniquely targeted and that these scans are            practitioners should refrain from exploiting vulnerabili-
not malicious in nature, and might waste resources re-              ties or accessing protected resources, and should comply
sponding. Some owners of target systems may simply                  with any special legal requirements in their jurisdictions.
be annoyed and want our scans to cease. To minimize
the risks from these scenarios, we took several steps to            5.1   User Responses
make it easy for traffic recipients to learn why they were
receiving probes and to have their addresses excluded               We performed approximately 200 Internet-wide scans
from scanning if so desired.                                        over the course of a year, following the practices described
   First, we configured our source addresses to present a           above. We received e-mail responses from 145 scan traf-
simple website on port 80 that describes the nature and             fic recipients, which we classify in Table 6. In most cases,
purpose of the scans The site explains that we are not              these responses were informative in nature, notifying us
targeting individual networks or attempting to obtain ac-           that we may have had infected machines, or were civil
cess to private systems, and it provides a contact email            requests to be excluded from future scans. The vast ma-
address to request exclusion from future scans. Second,             jority of these requests were received at our institution’s
we set reverse DNS records for our source addresses to              WHOIS abuse address or at the e-mail address published
“researchscanx.eecs.umich.edu” in order to signal that traf-        on the scan source IP addresses, but we also received


                                                               12
            Small/Medium Business          41                         Previous work has developed methods for sending and
            Home User                      38                      receiving packets at fast network line speeds, including
            Other Corporation              17                      PF_RING [8], PacketShader [11], and netmap [28], all
            Academic Institution           22                      of which replace parts of the Linux kernel network stack.
            Government/Military            15                      However, as discussed in Section 3.1, we find that the
            Internet Service Provider       2                      Linux kernel is capable of sending probe packets at giga-
            Unknown                        10                      bit Ethernet line speed without modification. In addition,
                                                                   libpcap is capable of processing responses without drop-
            Total Entities                145                      ping packets as only a small number of hosts respond to
                                                                   probes. The bottlenecks in current tools are in the scan
Table 6: Responses by Entity Type — We classify the                methodology rather than the network stack.
responses and complaints we received about our ongoing                Many projects have performed Internet-scale network
scans based on the type of entity that responded.                  surveys (e.g., [10, 12, 14, 15, 25, 27]), but this has typi-
                                                                   cally required heroic effort on the part of the researchers.
                                                                   In 2008, Heidemann et al. presented an Internet census
responses sent to our institution’s help desk, our chief
                                                                   in which they attempted to determine IPv4 address uti-
security officer, and our departmental administrator.
                                                                   lization by sending ICMP packets to allocated IP ad-
   We responded to each inquiry with information about
                                                                   dresses; their scan of the IPv4 address space took ap-
the purpose of our scans, and we immediately excluded
                                                                   proximately three months to complete and claimed to be
the sender’s network from future scans upon request. In
                                                                   the first Internet-wide survey since 1982 [12]. Two other
all, we excluded networks belonging to 91 organizations
                                                                   recent works were motivated by studying the security
or individuals, totaling 3,753,899 addresses (0.11% of the
                                                                   of HTTPS. In 2010, the Electronic Frontier Foundation
public IPv4 address space). About 49% of the blacklisted
                                                                   (EFF) performed a scan of the public IPv4 address space
addresses resulted from requests from two Internet service
                                                                   using Nmap [23] to find hosts with port 443 (HTTPS)
providers. We received 15 actively hostile responses that
                                                                   open as part of their SSL Observatory Project [10]; their
threatened to retaliate against our institution legally or
                                                                   scans were performed on three Linux servers and took
to conduct a denial-of-service (DOS) attack against our
                                                                   approximately three months to complete. Heninger et al.
network. In two cases, we received retaliatory DOS traffic,
                                                                   performed a scan of the IPv4 address space on port 443
which was blacklisted by our upstream provider.
                                                                   (HTTPS) in 2011 and on port 22 (SSH) in 2012 as part of
                                                                   a study on weak cryptographic keys [14]. The researchers
6   Related Work                                                   were able to perform a complete scan in 25 hours by
Many network scanning tools have been developed, the               concurrently performing scans from 25 Amazon EC2 in-
vast majority of which have been optimized to scan small           stances at a cost of around $300. We show that ZMap
network segments. The most popular and well respected              could be used to collect the same data much faster and at
is Nmap (“Network Mapper”) [23], a versatile, multipur-            far lower cost.
pose tool that supports a wide variety of probing tech-               Most recently, an anonymous group performed an il-
niques. Unlike Nmap, ZMap is specifically designed for             legal “Internet Census” in 2012, using the self-named
Internet-wide scanning, and it achieves much higher per-           Carna Botnet. This botnet used default passwords to log
formance in this application.                                      into thousands of telnet devices. After logging in, the
   Leonard and Loguinov introduced IRLscanner, an                  botnet scanned for additional vulnerable telnet devices
Internet–scale scanner with the demonstrated ability to            and performed several scans over the IPv4 space, com-
probe the advertised IPv4 address space in approximately           prising over 600 TCP ports and 100 UDP ports over a
24 hours, ultimately scanning at 24,421 packets per sec-           3-month period [1]. With this distributed architecture, the
ond [22]. IRLscanner is able to perform scanning at                authors claim to have been able to perform a single-port
this rate by utilizing a custom Windows network driver,            scan survey over the IPv4 space in about an hour. ZMap
IRLstack [33]. However, IRLscanner does not process                can achieve similar performance without making use of
responses, requires a custom network driver and a com-             stolen resources.
plete routing table for each scan, and was never released
to the research community. In comparison, we developed             7   Future Work
ZMap as a self-contained network scanner that requires
no custom drivers, and we are releasing it to the commu-           While we have demonstrated that efficiently scanning
nity under an open source license. We find that ZMap can           the IPv4 address space at gigabit line speeds is possible,
scan at 1.37 million packets per second, 56 times faster           there remain several open questions related to performing
than IRLScanner was shown to operate.                              network surveys over other protocols and at higher speeds.


                                                              13
Scanning IPv6 While ZMap is capable of rapidly scan-                speed for gigabit Ethernet and with an estimated 98%
ning the IPv4 address space, brute-force scanning meth-             coverage of publicly available hosts. We explored the
ods will not suffice in the IPv6 address space, which               security applications of high speed scanning, including
is far too large to be fully enumerated [7]. This places            the ability to track protocol adoption at Internet scale and
current researchers in a window of opportunity to take              to gain timely insight into opaque distributed systems
advantage of fast Internet-wide scanning methodologies              such as the certificate authority ecosystem. We further
before IPv6-only services become common place. New                  showed that high-speed scanning also provides new attack
methodologies will need to be developed specifically for            vectors that we must consider when defending systems,
performing surveys of the IPv6 address space.                       including the ability to uncover hidden services, the po-
                                                                    tential to track users between IP addresses, and the risk
10gigE Surveys ZMap is currently limited by the speed               of infection of vulnerable hosts en masse within minutes
of widely available gigabit networks, and we have not               of a vulnerability’s discovery.
tested how well its architecture will scale as 10gigE and               We hope ZMap will elevate Internet-wide scanning
faster networks become available. There is motivation to            from an expensive and time-consuming endeavor to a
perform the fastest scans possible as they will provide the         routine methodology for future security research. As
truest sense of a snapshot of the Internet at a given point         Internet-wide scanning is conducted more routinely, prac-
in time. However, these faster rates also open questions            titioners must ensure that they act as good Internet citizens
of overloading destination networks and hosts. The dy-              by minimizing risks to networks and hosts and being re-
namics of performing scans at 10gigE have not yet been              sponsive to inquiries from traffic recipients. We offer the
explored.                                                           recommendations we developed while performing our
Server Name Indication           Server Name Indication             own scans as a starting point for further conversations
(SNI) is a TLS protocol extension that allows a server              about good scanning practice.
to present multiple certificates on the same IP address [6].
SNI has not yet been widely deployed, primarily because             Acknowledgments
Internet Explorer does not support it on Windows XP
hosts [21]. However, its inevitable growth will make                The authors thank the exceptional sysadmins at the Uni-
scanning HTTPS sites more complicated, since simply                 versity of Michigan for their help and support throughout
enumerating the address space will miss certificates that           this project. This research would not have been possible
are only presented with the correct SNI hostname.                   without Kevin Cheek, Laura Fink, Paul Howell, Don Win-
                                                                    sor, and others from ITS, CAEN, and DCO. We thank
Scanning Exclusion Standards If Internet-wide scan-
                                                                    Michael Bailey for advice on many aspects of the work
ning becomes more widespread, it will become increas-
                                                                    and Oguz Durumeric for his discussion of generating per-
ingly burdensome for system operators who do not want
                                                                    mutations of the IPv4 address space. We also thank Brad
to receive such probe traffic to manually opt out from
                                                                    Campbell, Peter Eckersley, James Kasten, Pat Pannuto,
all benign sources. Further work is needed to standard-
                                                                    Amir Rahmati, Michael Rushanan, and Seth Schoen. This
ize an exclusion signaling mechanism, akin to HTTP’s
                                                                    work was supported in part by NSF grant CNS-1255153
robots.txt [19]. For example, a host could use a combi-
                                                                    and by an NSF Graduate Research Fellowship.
nation of protocol flags to send a “do-not-scan” signal,
perhaps by responding to unwanted SYNs with the SYN
and RST flags, or a specific TCP option set.                        References
                                                                     [1] Anonymous. Internet census 2012. http://census2012.
8   Conclusion                                                           sourceforge.net/paper.html, March 2013.
                                                                     [2] G. Bartlett, J. Heidemann, and C. Papadopoulos. Under-
We are living in a unique period in the history of the                   standing passive and active service discovery. In 7th ACM
Internet: typical office networks are becoming fast enough               SIGCOMM conference on Internet measurement (IMC),
to exhaustively scan the IPv4 address space, yet IPv6                    pages 57–70, 2007.
(with its much larger address space) has not yet been                [3] L. Bello. DSA-1571-1 OpenSSL—Predictable random
widely deployed. To help researchers make the most                       number generator, 2008. Debian Security Advisory. http://
of this window of opportunity, we developed ZMap, a                      www.debian.org/security/2008/dsa-1571.
network scanner specifically architected for performing              [4] D. J. Bernstein. SYN cookies. http://cr.yp.to/syncookies.
fast, comprehensive Internet-wide surveys.                               html, 1996.
   We experimentally showed that ZMap is capable of                  [5] J. Black, S. Halevi, H. Krawczyk, T. Krovetz, and P. Rog-
scanning the public IPv4 address space on a single port                  away. UMAC: Fast and secure message authentication. In
in under 45 minutes, at 97% of the theoretical maximum                   Advances in Cryptology—CRYPTO ’99, 1999.


                                                               14
 [6] S. Blake-Wilson, M. Nystrom, D. Hopwood, J. Mikkelsen,                 [21] E. Law. Understanding certificate name mismatches.
     and T. Wright. Transport Layer Security (TLS) Extensions.                   http://blogs.msdn.com/b/ieinternals/archive/2009/12/07/
     RFC 3546 (Proposed Standard), June 2003.                                    certificate-name-mismatch-warnings-and-server-name-
 [7] T. Chown. IPv6 Implications for Network Scanning. RFC                       indication.aspx, December 2009.
     5157 (Informational), March 2008.                                      [22] D. Leonard and D. Loguinov. Demystifying service discov-
                                                                                 ery: Implementing an Internet-wide scanner. In 10th ACM
 [8] L. Deri. Improving passive packet capture: Beyond device
                                                                                 SIGCOMM conference on Internet measurement (IMC),
     polling. In 4th International System Administration and
                                                                                 pages 109–122, 2010.
     Network Engineering Conference (SANE), 2004.
                                                                            [23] Gordon Fyodor Lyon. Nmap Network Scanning: The
 [9] R. Dingledine. Research problems: Ten ways to dis-
                                                                                 Official Nmap Project Guide to Network Discovery and
     cover Tor bridges. http://blog.torproject.org/blog/research-
                                                                                 Security Scanning. Insecure, USA, 2009.
     problems-ten-ways-discover-tor-bridges, October 2011.
                                                                            [24] N. Mathewson and N. Provos. libevent—An event notifi-
[10] P. Eckersley and J. Burns. An observatory for the SSLiv-
                                                                                 cation library. http://libevent.org.
     erse. Talk at Defcon 18 (2010). https://www.eff.org/files/
     DefconSSLiverse.pdf.                                                   [25] HD Moore.         Security flaws in universal plug
                                                                                 and play. Unplug. Don’t Play, January 2013.
[11] S. Han, K. Jang, K. Park, and S. Moon. PacketShader:
                                                                                 http://community.rapid7.com/servlet/JiveServlet/
     A GPU-accelerated software router. In ACM SIGCOMM,
                                                                                 download/2150-1-16596/SecurityFlawsUPnP.pdf.
     September 2010.
                                                                            [26] Netcraft, Ltd. Web server survey. http://news.netcraft.com/
[12] J. Heidemann, Y. Pradkin, R. Govindan, C. Papadopoulos,
                                                                                 archives/2013/05/03/may-2013-web-server-survey.html,
     G. Bartlett, and J. Bannister. Census and survey of the
                                                                                 May 2013.
     visible Internet. In 8th ACM SIGCOMM conference on
     Internet measurement (IMC), 2008.                                      [27] N. Provos and P. Honeyman. ScanSSH: Scanning the
                                                                                 Internet for SSH servers. In 16th USENIX Systems Admin-
[13] J. Heidemann, L. Quan, and Y. Pradkin. A preliminary
                                                                                 istration Conference (LISA), 2001.
     analysis of network outages during hurricane sandy. Tech-
     nical Report ISI-TR-2008-685b, USC/Information Sci-                    [28] Luigi Rizzo. netmap: A novel framework for fast packet
     ences Institute, November 2012.                                             I/O. In 2012 USENIX Annual Technical Conference, 2012.
[14] N. Heninger, Z. Durumeric, E. Wustrow, and J. A. Halder-               [29] S. Sanfilippo and P. Noordhuis. Redis. http://redis.io.
     man. Mining your Ps and Qs: Detection of widespread                    [30] H. Scholz. SIP stack fingerprinting and stack difference
     weak keys in network devices. In 21st USENIX Security                       attacks. Talk at Blackhat 2006. http://www.blackhat.com/
     Symposium, August 2012.                                                     presentations/bh-usa-06/BH-US-06-Scholz.pdf.
[15] R. Holz, L. Braun, N. Kammenhuber, and G. Carle. The                   [31] A. Schulman and N. Spring. Pingin’ in the rain. In 11th
     SSL landscape: A thorough analysis of the X.509 PKI                         ACM SIGCOMM conference on Internet measurement
     using active and passive measurements. In 11th ACM                          (IMC), pages 19–28, 2011.
     SIGCOMM conference on Internet measurement (IMC),
                                                                            [32] K. Sklower. A tree-based packet routing table for Berkeley
     pages 427–444, 2011.
                                                                                 Unix. In Winter USENIX Conference, 1991.
[16] IANA.       IPv4 address space registry.                http://
                                                                            [33] M. Smith and D. Loguinov. Enabling high-performance
     www.iana.org/assignments/ipv4-address-space/
                                                                                 Internet-wide measurements on Windows. In 11th Inter-
     ipv4-address-space.xml.
                                                                                 national Conference on Passive and Active Measurement
[17] V. Jacobson, C. Leres, and S. McCanne. libpcap. Lawrence                    (PAM), pages 121–130. Springer, 2010.
     Berkeley National Laboratory, Berkeley, CA. Initial re-
                                                                            [34] W. R. Stevens and G. R. Wright. TCP/IP Illustrated: The
     lease June 1994.
                                                                                 Implementation, volume 2. Addison-Wesley, 1995.
[18] J. Kasten, E. Wustrow, and J. A. Halderman. Cage: Taming
                                                                            [35] Tor Project. Tor Bridges. https://www.torproject.org/docs/
     certificate authorities by inferring restricted scopes. In 17th
                                                                                 bridges, 2008.
     International Conference on Financial Cryptography and
     Data Security (FC), 2013.                                              [36] Tor Project. obfsproxy. https://www.torproject.org/
                                                                                 projects/obfsproxy.html.en, 2012.
[19] M Koster. A standard for robot exclusion. http://www.
     robotstxt.org/orig.html, 1994.                                         [37] J. Viega, M. Messier, and P. Chandra. Network Security
                                                                                 with OpenSSL: Cryptography for Secure Communications.
[20] A. Langley. Enhancing digital certificate security. Google
                                                                                 O’Reilly, 2002.
     Online Security Blog, http://googleonlinesecurity.blogspot.
     com/2013/01/enhancing-digital-certificate-security.html,               [38] T. Wilde. Great Firewall Tor probing. https://gist.github.
     January 2013.                                                               com/twilde/da3c7a9af01d74cd7de7, 2012.




                                                                       15
