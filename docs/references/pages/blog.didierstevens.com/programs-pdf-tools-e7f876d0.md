---
url: "https://blog.didierstevens.com/programs/pdf-tools/"
title: "PDF Tools"
site: "Didier Stevens"
published: "2008-10-30T17:12:18+00:00"
captured: "2026-11-02T18:22:46Z"
via: "live"
sha256: "c5cfe1b30b1610bea28ade1b7a14ab82a3db7b07fa972250525574636647b500"
wayback: "https://web.archive.org/web/20261102193014/https://blog.didierstevens.com/programs/pdf-tools/"
cited_in:
  - "REVIEW.md"
---

# PDF Tools

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://blog.didierstevens.com/programs/pdf-tools/>
> Snapshot: <https://web.archive.org/web/20261102193014/https://blog.didierstevens.com/programs/pdf-tools/>

# [Didier Stevens](https://blog.didierstevens.com/)

### [PDF Tools](https://blog.didierstevens.com/programs/pdf-tools/)

Here is a set of free YouTube videos showing how to use my tools: [Malicious PDF Analysis Workshop](https://www.youtube.com/watch?v=F3rpZT0gKXw&list=PLa-ohdLO29_Y2FeT24w-c9nA_AH84MIpp).

pdf-parser.py

This tool will parse a PDF document to identify the [fundamental elements](https://blog.didierstevens.com/2008/04/09/quickpost-about-the-physical-and-logical-structure-of-pdf-files/) used in the analyzed file. It will not render a PDF document. The code of the parser is quick-and-dirty, I’m not recommending this as text book case for PDF parsers, but it gets the job done.

You can see the parser in action in [this screencast](https://blog.didierstevens.com/2008/10/20/analyzing-a-malicious-pdf-file/).

The stats option display statistics of the objects found in the PDF document. Use this to identify PDF documents with unusual/unexpected objects, or to classify PDF documents. For example, I generated statistics for 2 malicious PDF files, and although they were very different in content and size, the statistics were identical, proving that they used the same attack vector and shared the same origin.

The search option searches for a string in indirect objects (not inside the stream of indirect objects). The search is not case-sensitive, and is susceptible to the [obfuscation techniques I documented](https://blog.didierstevens.com/2008/04/29/pdf-let-me-count-the-ways/) (as I’ve yet to encounter these obfuscation techniques in the wild, I decided no to resort to canonicalization).

filter option applies the filter(s) to the stream. For the moment, only FlateDecode is supported (e.g. zlib decompression).

The raw option makes pdf-parser output raw data (e.g. not the printable Python representation).

objects outputs the data of the indirect object which ID was specified. This ID is not version dependent. If more than one object have the same ID (disregarding the version), all these objects will be outputted.

reference allows you to select all objects referencing the specified indirect object. This ID is not version dependent.

type allows you to select all objects of a given type. The type is a Name and as such is case-sensitive and must start with a slash-character (/).

[pdf-parser_V0_7_14.zip](https://didierstevens.com/files/software/pdf-parser_V0_7_14.zip) ([http](http://didierstevens.com/files/software/pdf-parser_V0_7_14.zip))
MD5: EB3808ACE5497B428138594AFDC5205F
SHA256: 6A60223D52B75F8AFF8C8CF19A58699A20829AC758C251B405B08EC734EF6A4A

make-pdf tools
make-pdf-javascript.py allows one to create a simple PDF document with embedded JavaScript that will execute upon opening of the PDF document. It’s essentially glue-code for the mPDF.py module which contains a class with methods to create headers, indirect objects, stream objects, trailers and XREFs.

If you execute it without options, it will generate a PDF document with JavaScript to display a message box (calling app.alert).

To provide your own JavaScript, use option –javascript for a script on the command line, or –javascriptfile for a script contained in a file.

make-pdf-embedded.py creates a PDF file with an embedded file.

Download:

[make-pdf_V0_1_8.zip](http://didierstevens.com/files/software/make-pdf_V0_1_8.zip) ([https](https://didierstevens.com/files/software/make-pdf_V0_1_8.zip))
MD5: 4BEB862F383465C524717BD9BE57C512
SHA256: B9DEF588F2B7577D0AE055E5E1DD4BA2927C1972F0BA1B5C58484295FE821231

pdfid.py
This tool is not a PDF parser, but it will scan a file to look for certain PDF keywords, allowing you to identify PDF documents that contain (for example) JavaScript or execute an action when opened. PDFiD will also handle [name obfuscation](https://blog.didierstevens.com/2008/04/29/pdf-let-me-count-the-ways/).

The idea is to use this tool first to triage PDF documents, and then [analyze the suspicious ones with my pdf-parser](https://blog.didierstevens.com/2008/10/20/analyzing-a-malicious-pdf-file/).

An important design criterium for this program is simplicity. Parsing a PDF document completely requires a very complex program, and hence it is bound to contain many (security) bugs. To avoid the risk of getting exploited, I decided to keep this program very simple (it is even simpler than pdf-parser.py).

PDFiD will scan a PDF document for a given list of strings and count the occurrences (total and obfuscated) of each word:

- obj
- endobj
- stream
- endstream
- xref
- trailer
- startxref
- /Page
- /Encrypt
- /ObjStm
- /JS
- /JavaScript
- /AA
- /OpenAction
- /JBIG2Decode
- /RichMedia
- /Launch
- /XFA

Almost every PDF documents will contain the first 7 words (obj through startxref), and to a lesser extent stream and endstream. I’ve found a couple of PDF documents without xref or trailer, but these are rare (BTW, this is not an indication of a malicious PDF document).

/Page gives an indication of the number of pages in the PDF document. Most malicious PDF document have only one page.

/Encrypt indicates that the PDF document has DRM or needs a password to be read.

/ObjStm counts the number of object streams. An object stream is a stream object that can contain other objects, and can therefor be used to obfuscate objects (by using different filters).

/JS and /JavaScript indicate that the PDF document contains JavaScript. Almost all malicious PDF documents that I’ve found in the wild contain JavaScript (to exploit a JavaScript vulnerability and/or to execute a heap spray). Of course, you can also find JavaScript in PDF documents without malicious intend.

/AA and /OpenAction indicate an automatic action to be performed when the page/document is viewed. All malicious PDF documents with JavaScript I’ve seen in the wild had an automatic action to launch the JavaScript without user interaction.

The combination of automatic action and JavaScript makes a PDF document very suspicious.

/JBIG2Decode indicates if the PDF document uses JBIG2 compression. This is not necessarily and indication of a malicious PDF document, but requires further investigation.

/RichMedia is for embedded Flash.

/Launch counts launch actions.

/XFA is for XML Forms Architecture.

A number that appears between parentheses after the counter represents the number of obfuscated occurrences. For example, /JBIG2Decode 1(1) tells you that the PDF document contains the name /JBIG2Decode and that it was obfuscated (using hexcodes, e.g. /JBIG#32Decode).

BTW, all the counters can be skewed if the PDF document is saved with [incremental updates](https://blog.didierstevens.com/2008/05/07/solving-a-little-pdf-puzzle/).

Because PDFiD is just a string scanner (supporting name obfuscation), it will also generate false positives. For example, a simple text file starting with %PDF-1.1 and containing words from the list will also be identified as a PDF document.

Download:

[pdfid_v0_2_10.zip](https://didierstevens.com/files/software/pdfid_v0_2_10.zip) ([http](http://didierstevens.com/files/software/pdfid_v0_2_10.zip))
MD5: E2F369B34D7148BE4D5C4C02430E7983
SHA256: A677336B1CF51386E35DBDED8FDB79F27368FD5D5ECC3FC5C8DA020A029CB6B6

pdftool.py

pdftool.py is a tool that takes commands. This first version has only one command: iu (incremental updates).

With command iu, you can check if a PDF contains incremental updates, and select the versions you want.

[pdftool_V0_0_1.zip](http://didierstevens.com/files/software/pdftool_V0_0_1.zip) ([https](https://didierstevens.com/files/software/pdftool_V0_0_1.zip))
MD5: ED2BBE886008C737CC06E22F4F0FE8A1
SHA256: 401E88FBFAEC4382A50FE59430D04FE6111F9911958AB09BA7530C26043FDA87
PDFTemplate.bt
This is a [010 Editor](http://www.sweetscape.com/010editor/) template for the PDF file format.
It’s particularly useful for malformed PDF files, like this example with PDFUnknown structures:

Download:

[PDFTemplate.zip](http://didierstevens.com/files/software/PDFTemplate.zip) ([https](https://didierstevens.com/files/software/PDFTemplate.zip))

MD5: C124200C3317ACA9C17C2AE2579FCFEB

SHA256: 24C4FEAD2CABAD82EC336DDCFD404915E164D7B48FBA7BA1295E12BBAF8EB15D

### Share this:

- [Facebook](https://blog.didierstevens.com/programs/pdf-tools/?share=facebook)
- [X](https://blog.didierstevens.com/programs/pdf-tools/?share=x)
-

[Comments (411)](https://blog.didierstevens.com/programs/pdf-tools/#comments)

## 411 Comments [»](https://blog.didierstevens.com/programs/pdf-tools/#postcomment)

-

[…] PDF, Quickpost — Didier Stevens @ 11:57 Per request, a more detailed post on how I use my pdf-parser stats […]

Pingback by [Quickpost: Fingerprinting PDF Files « Didier Stevens](https://blog.didierstevens.com/2008/11/01/quickpost-fingerprinting-pdf-files/) — Saturday 1 November 2008 @ [11:57](https://blog.didierstevens.com/programs/pdf-tools/#comment-33647)

-

I’d like to be able to view a scanned pdf file (with handwriting in some fields) and black
out boxes on the form whose fields contain info I don’t want published.

Can that kind of thing be automated in a batch so that I don’t even have to open the files ?

That would be cool …

Can you point me in the right direction ? I’m not looking for you to code, but sending me in the right direction for this would be useful, and it looks like you’re cognizant of this kind of information.

Comment by james — Friday 21 November 2008 @ [15:03](https://blog.didierstevens.com/programs/pdf-tools/#comment-33761)

-

@james

I’ve no experience with such tools, but you can start to look in the forum of PDF Planet.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 22 November 2008 @ [8:46](https://blog.didierstevens.com/programs/pdf-tools/#comment-33768)

-

[…] download my Python program to generate these PoC PDF documents here, it needs the mPDF module of my PDF-tools. Quickpost info Possibly related posts: (automatically generated)PDF Stream ObjectsUsing DLE In An […]

Pingback by [Quickpost: /JBIG2Decode Essentials « Didier Stevens](https://blog.didierstevens.com/2009/03/02/quickpost-jbig2decode-essentials/) — Monday 2 March 2009 @ [23:12](https://blog.didierstevens.com/programs/pdf-tools/#comment-34248)

-

[…] PDF — Didier Stevens @ 7:08 I’ve developed a new tool to triage PDF documents, PDFiD. It helps you differentiate between PDF documents that could be malicious and those that are most […]

Pingback by [PDFiD « Didier Stevens](https://blog.didierstevens.com/2009/03/31/pdfid/) — Tuesday 31 March 2009 @ [7:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-34460)

-

Thanks Didier for sharing _yet_again_ I appreciate it very much!

Comment by Mitch Impey — Wednesday 1 April 2009 @ [20:17](https://blog.didierstevens.com/programs/pdf-tools/#comment-34478)

-

[…] developed a new tool to triage PDF documents, PDFiD. It helps you differentiate between PDF documents that could be malicious and those that are most […]

Pingback by [Didier Stevens posted PDFid « Betterworldforus](http://betterworldforus.wordpress.com/2009/04/03/didier-stevens-posted-pdfid/) — Friday 3 April 2009 @ [18:34](https://blog.didierstevens.com/programs/pdf-tools/#comment-34491)

-

[…] will give you statistics of some very basic elements of the PDF language. This helps you decide if a PDF could be malicious or […]

Pingback by [PDFiD On VirusTotal « Didier Stevens](https://blog.didierstevens.com/2009/04/21/pdfid-on-virustotal/) — Tuesday 21 April 2009 @ [16:59](https://blog.didierstevens.com/programs/pdf-tools/#comment-34667)

-

[…] We can go a number of ways with this now, but since the point of all this is the fun we can have with obfuscated scripts in Adobe PDFs we’ll run it through PDFiD from Didier Stevens. […]

Pingback by [L1pht Offensive Labs » From Bloodhound to Acrobat JS](http://www.l1pht.com/2009/04/from-bloodhound-to-acrobat-javascript/) — Saturday 25 April 2009 @ [2:59](https://blog.didierstevens.com/programs/pdf-tools/#comment-34695)

-

Hello — I am using pdf-parser and python for the first time so please excuse my ignorance.

I’m using Python 3.0.1 on Windows XP. I’ve copied the pdf-parser.py file into the C:\Python30 directory which contains the python executable. Below is the error I get when attempting to execute your utility:

C:\Python30>python.exe pdf-parser.py
File “pdf-parser.py”, line 180
print ‘todo 1: %s’ % (self.token[1] + self.token2[1])
^
SyntaxError: invalid syntax

————

Any ideas? Thanks for your time.

Comment by Mike — Tuesday 5 May 2009 @ [19:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-34869)

-

@rMike No problem.

I’ve not tested my Python tools on Python 3

You should remove Python 3.0.1 and install Python 2.6. This will probably fix your problems.

I’ve still to decide when I upgrade my tools to Python 3

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 5 May 2009 @ [20:59](https://blog.didierstevens.com/programs/pdf-tools/#comment-34871)

-

Hello — I’m now using Python 2.6.2. It appears to be working, however I am getting so much output from every pdf I examine, I wonder if I am doing something wrong.

My syntax is

pdf-parser.py –search javascript malware.pdf

The utility spits out hundreds maybe thousands of lines of returned information. At the very top there appears to be useful data, however there are hundreds of lines that look like:

todo 10: 3 ‘X1\x1e\x1b\x03\x12\x05X60B

Am I doing something incorrectly here or is there a way to filter the rest of this data out?

Thanks for your help.

PS, I watched your video on pdf-parser and it doesn’t have any audio.

Comment by Mike — Thursday 7 May 2009 @ [20:40](https://blog.didierstevens.com/programs/pdf-tools/#comment-34911)

-

Also, just a note I am do have two hyphens in front of search (–search)

Comment by Mike — Thursday 7 May 2009 @ [20:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-34912)

-

No, what you’re doing is correct. These todo 10 messages indicate that your PDF document is possibly corrupt. I’ve seen one such corrupt document before, the malware authors had inserted their payload in the PDF without respecting the PDF syntax.

I’m sending you an e-mail to see if you can share your sample with me.

About the video: most of my videos have no sound, it takes me much more time to produce one with audio, and I’m never satisfied with the result.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 7 May 2009 @ [21:07](https://blog.didierstevens.com/programs/pdf-tools/#comment-34913)

-

[…] updated my PDF-tools to support […]

Pingback by [PDF Filter Abbreviations « Didier Stevens](https://blog.didierstevens.com/2009/05/11/pdf-filter-abbreviations/) — Monday 11 May 2009 @ [0:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-34932)

-

Didier,
Any idea whether/when Filiol, Blonce, & Frayssignes might release their ‘PDF StructAzer’ tool? My Google research doesn’t show anything from them since their presentation at Blackhat Europe last year. According to that paper, they planned to release the tool “very soon”, but maybe they’ve run into some red tape with their employer over it.
John

Comment by [John McCash](http://sansforensics.wordpress.com/author/johnmccash/) — Tuesday 12 May 2009 @ [13:03](https://blog.didierstevens.com/programs/pdf-tools/#comment-34942)

-

Hi John,

No, still no news about a release, but I’ll ask him again.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 12 May 2009 @ [13:51](https://blog.didierstevens.com/programs/pdf-tools/#comment-34943)

-

[…] added some new features to my PDF tools to handle malformed PDF […]

Pingback by [Malformed PDF Documents « Didier Stevens](https://blog.didierstevens.com/2009/05/14/malformed-pdf-documents/) — Thursday 14 May 2009 @ [7:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-34954)

-

PDF Structazer is available here:
http://www.esiea-recherche.eu/

The tool: http://www.esiea-recherche.eu/data/PDF%20Structazer.exe
The document (PDF): http://www.esiea-recherche.eu/data/PDF%20Structazer%20Short%20User%20Manual.pdf

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 26 May 2009 @ [13:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-35021)

-

[…] PDFiD – https://blog.didierstevens.com/2009/03/31/pdfid/ PDF Tools – https://blog.didierstevens.com/programs/pdf-tools/ Security Justice – http://securityjustice.com/ Exotic Liability – http://exoticliability.ning.com/ […]

Pingback by [SecuraBit Episode 32 PDF Love! | SecuraBit](http://securabit.com/2009/05/27/securabit-episode-32-pdf-love/) — Wednesday 27 May 2009 @ [14:33](https://blog.didierstevens.com/programs/pdf-tools/#comment-35027)

-

[…] Finding and detecting Malicious PDF’s. Tyler’s Snort signature. Didier Steven’s fantastic PDF analysis tools. […]

Pingback by [Security Justice » Blog Archive » Security Justice - Episode 13](http://securityjustice.com/archives/65) — Saturday 6 June 2009 @ [2:30](https://blog.didierstevens.com/programs/pdf-tools/#comment-35072)

-

Didier,
Does your script pdf-parser.py works on Encrypted PDF streams ?

Comment by [NeoIsOffline](http://null.co.in) — Tuesday 14 July 2009 @ [12:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-35342)

-

No, I didn’t add code to decrypt PDF streams. And I’m not sure if I ever will, because then it could also be considered as a copyright infringement tool, and I don’t want to deal with that.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 14 July 2009 @ [19:13](https://blog.didierstevens.com/programs/pdf-tools/#comment-35345)

-

Im getting errors when running the python script:

C:\Documents and Settings\yo\Desktop\Tools\pdf>pdf-parser.py
File “C:\Documents and Settings\yo\Desktop\Tools\pdf\pdf-parser.py”, line 198
print ‘todo 1: %s’ % (self.token[1] + self.token2[1])
^
SyntaxError: invalid syntax

Im getting errors when trying to run the script. Im using activepyton 3.1 on windows xp. Launching it from the commandline. Was there any recent modifications which broke the script?

Thanks

Comment by Dave — Friday 17 July 2009 @ [16:48](https://blog.didierstevens.com/programs/pdf-tools/#comment-35366)

-

@Dave

That looks the same error as in comment 10. Read comments 10 to 12 for a solution.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 17 July 2009 @ [17:12](https://blog.didierstevens.com/programs/pdf-tools/#comment-35367)

-

wow I feel ignorant, i just breezed through the comments fast. Thanks!

Comment by Dave — Friday 17 July 2009 @ [18:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-35369)

-

@Dave

No problem.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 17 July 2009 @ [18:58](https://blog.didierstevens.com/programs/pdf-tools/#comment-35370)

-

Hello Didier,

This tool is very cool, I am wondering how to integrate this to the ironport (mail filter) so that all attachments like pdf will be scanned and if found that there are openaction or javascript then probably we can filter that. Also, maybe this can be integrated in the Proxy or firewall. Do you have any link to see a topic on integrating this tool.

Thank you very much, indeed this is very helpful

Comment by Yaggi — Tuesday 28 July 2009 @ [0:12](https://blog.didierstevens.com/programs/pdf-tools/#comment-35425)

-

What interface options does ironport offer? Does it support ICAP? http://en.wikipedia.org/wiki/Internet_Content_Adaptation_Protocol

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 28 July 2009 @ [18:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-35429)

-

Hello Didier,

I talked to our notes admin and this can somehow be integrated in the sendmail. He ask me some technicalities on the integration. maybe you can also provide some link. HE told me ironport will not be used.

Where can we possibly put this script for internet filter, is it in the proxy server or possibly to content filter boxes like from 8e6 technologies? Im hoping to finish this integration so we can maximize your script.

Comment by Yaggi — Wednesday 29 July 2009 @ [1:30](https://blog.didierstevens.com/programs/pdf-tools/#comment-35432)

-

@Yaggi

How do you interface with other scanner, like AV software?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 31 July 2009 @ [12:48](https://blog.didierstevens.com/programs/pdf-tools/#comment-35441)

-

[…] Taking a quick look at the rest of the file it was clear that it is just a “simple” exploit using obfuscated javascript. So I extracted the scripts with the pdf-tools from Didier Stevens. […]

Pingback by [hong10.net » Blog Archive » Analyzing malicious PDF Documents](http://hong10.net/?p=68) — Monday 3 August 2009 @ [21:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-35448)

-

I was looking over your code in the pdf parser, and trying very very hard to get it to extract something from a pdf. I have tried using pdfs I’ve been given, as well as going directly into the files to try to mess things up, and I have yet to see the code go into the cPDFElementMalformed method. What kinds of data (objects?) do you expect to fall into this category?

Thank you -ld

Comment by Delucci — Tuesday 4 August 2009 @ [14:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-35451)

-

@Delucci

It’s for sometinh like this:

%PDF-1.4

1 0 obj
<>
endobj

unexpected

2 0 obj
<>
endobj

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 4 August 2009 @ [15:33](https://blog.didierstevens.com/programs/pdf-tools/#comment-35453)

-

[…] the PDF analysis, I used the excellent PDF-Tools from Didier Stevens that can be located here. The main python script that was used was pdf-parser seen […]

Pingback by [PDF Malware Analysis – Part 1 | isolated-threat](http://www.isolatedthreat.com/?p=111) — Tuesday 18 August 2009 @ [21:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-35500)

-

[…] can download PDFiD here. Leave a […]

Pingback by [Update: PDFiD Version 0.0.9 to Detect Another Adobe 0Day « Didier Stevens](https://blog.didierstevens.com/2009/10/13/update-pdfid-version-0-0-9-to-detect-another-adobe-0day/) — Tuesday 13 October 2009 @ [21:24](https://blog.didierstevens.com/programs/pdf-tools/#comment-35907)

-

[…] Here is a set of tools that can embed Javascript into a PDF. […]

Pingback by [Adobe Reader 0-day exploit FINALLY fixed. | Invariable Truth](http://blog.invariabletruth.com/?p=6) — Sunday 18 October 2009 @ [9:35](https://blog.didierstevens.com/programs/pdf-tools/#comment-35935)

-

Hi,

Great set of tools, I have noticed while checking PDF files for embedded links (i.e. URI tags), some PDFs contain hyperlinks but do not contain the URI tag, is there an alternative method of embedding hyperlinks, should I be searching for some other keyword?

Comment by Tye — Monday 19 October 2009 @ [15:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-35956)

-

Check if the URLs you see are in the metadata.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 20 October 2009 @ [16:48](https://blog.didierstevens.com/programs/pdf-tools/#comment-35964)

-

Hi,

I’ve been using your tool to decode the malicious PDF file that I have. It’s using /Filter /ASCIIHexDecode /FlateDecode.

I used the following command:
pdf-parser.py -f -w malpdf.1 > mal.1

The resulting file didn’t show any JavaScript code, instead it showed “ASCIIHexDecode decompress failed”.

Wepawet is able to decode it though (http://wepawet.cs.ucsb.edu/view.php?hash=c9aad1ecee10ddcf1985ae4961e18fbf&type=js).

Are my parameters for the tool incorrect? Or doesn’t the tool support this?

Thanks in advance.

Comment by anima — Friday 30 October 2009 @ [5:40](https://blog.didierstevens.com/programs/pdf-tools/#comment-36049)

-

@anima

I’ve e-mailed you a request for the sample.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 5 November 2009 @ [17:45](https://blog.didierstevens.com/programs/pdf-tools/#comment-36086)

-

[…] my method: Use the tools from here. First of all pdfid can tell you if a pdf has Javascript included as well as autorun functionality […]

Pingback by [PDF file check question - Remote Exploit Forums](http://forums.remote-exploit.org/general-discussion/29434-pdf-file-check-question.html#post164977) — Sunday 6 December 2009 @ [22:25](https://blog.didierstevens.com/programs/pdf-tools/#comment-36821)

-

[…] first tool we’ll be using is pdf-parser.py from the PDF Tools suite. This script will search through a PDF file’s sections, display raw data in the sections, […]

Pingback by [Reversing the Adobe 0-day APSA09-07 Exploit – Part 1 | Missouri S&T ACM SIG-SEC|Reversing](http://acm.mst.edu/reversing/?p=41) — Wednesday 16 December 2009 @ [3:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-37187)

-

[…] Countermeasures __________________ Either you're part of the problem or you're part of the solution or you're just part of the landscape. […]

Pingback by [Using-an-adobe-exploit-in-a-email-attack - Remote Exploit Forums](http://forums.remote-exploit.org/backtrack-4-howto/29987-using-adobe-exploit-email-attack.html#post167286) — Tuesday 22 December 2009 @ [15:46](https://blog.didierstevens.com/programs/pdf-tools/#comment-37316)

-

[…] pdf-parser.py https://blog.didierstevens.com/programs/pdf-tools/ Lets decompression some of the zlib compressed code inside of the PDF and send the raw output to a […]

Pingback by [Reversing MerryChristmas.pdf - Sp8sCorp](http://www.sp8scorp.com/blog/?p=33) — Thursday 31 December 2009 @ [5:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-37363)

-

[…] pdf-parser.py or PDF Structazer to analyze PDF files […]

Pingback by [Can You Trust That File? « Aggressive Virus Defense](http://aggressivevirusdefense.wordpress.com/2009/09/23/can-you-trust-that-file/) — Thursday 31 December 2009 @ [22:40](https://blog.didierstevens.com/programs/pdf-tools/#comment-37367)

-

[…] […]

Pingback by [How to encode a PDF payload in metasploit? - Remote Exploit Forums](http://forums.remote-exploit.org/pentesting/30400-how-encode-pdf-payload-metasploit.html#post169092) — Tuesday 5 January 2010 @ [14:10](https://blog.didierstevens.com/programs/pdf-tools/#comment-37389)

-

[…] thanks to Didier Stevens for his free PDF tools and for providing some […]

Pingback by [PDF file loader to extract and analyse shellcode « c0llateral Blog](http://c0llateral.wordpress.com/2010/01/07/pdf-file-loader-to-extract-and-analyse-shellcode/) — Wednesday 6 January 2010 @ [23:19](https://blog.didierstevens.com/programs/pdf-tools/#comment-37399)

-

Hey Didier,
Thanks for excellent tool and great PDF analysis blog. I enjoyed every minute and in addition I have become much more paranoid when it comes to carelessly downloading tons of PDF material. Now I run all my PDFs through your “pdfid” tool, if I have downloaded anything from a suspicious site…

But I can’t help thinking that this should be implemented as an automatic plug-in/add-on to Firefox? You know, when you click on PDFs, they usually automatically open in the browser, which is nice if it was safe. But in the cyber-war era of today it is simply very bad, at it’s best!

Comment by E:V:A — Saturday 16 January 2010 @ [18:40](https://blog.didierstevens.com/programs/pdf-tools/#comment-37460)

-

I’m looking into this, but the problem is to prevent the download PDF from being opened after it’s downloaded and before it’s scanned. I talked to the developer of the Fireclam add-on and he has the same issue.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 19 January 2010 @ [9:29](https://blog.didierstevens.com/programs/pdf-tools/#comment-37497)

-

Only thing i get are syntax error!

C:\pdfid_v0_0_10>pdfid.py
File “C:\pdfid_v0_0_10\pdfid.py”, line 271
print ‘/%s -> /%s’ % (HexcodeName2String(wordExact), wordExactSwapped)

Comment by sheldor — Sunday 31 January 2010 @ [22:18](https://blog.didierstevens.com/programs/pdf-tools/#comment-37559)

-

Are you using Python 3? Haven’t tested PDFiD on Python 3. Use Python 2.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 31 January 2010 @ [22:20](https://blog.didierstevens.com/programs/pdf-tools/#comment-37560)

-

got it!! just read the comments!

Comment by sheldor — Sunday 31 January 2010 @ [22:35](https://blog.didierstevens.com/programs/pdf-tools/#comment-37562)

-

whow just noticed your quick response! thank you didier! great tool!

Comment by sheldor — Sunday 31 January 2010 @ [22:36](https://blog.didierstevens.com/programs/pdf-tools/#comment-37563)

-

Is the File Size limited? Everytime i scan larger PDF files i get exceptions like this:

***Error occured***
Traceback (most recent call last):
File “C:\PDFtools\pdfid.py”, line 363, in PDFiD
(bytesHeader, pdfHeader) = FindPDFHeaderRelaxed(oBinaryFile)
File “C:\PDFtools\pdfid.py”, line 218, in FindPDFHeaderRelaxed
bytes = oBinaryFile.bytes(1024)
File “C:\PDFtools\pdfid.py”, line 70, in bytes
inbytes = self.infile.read(size – len(self.ungetted))
IOError: [Errno 9] Bad file descriptor

Comment by sheldor — Monday 8 February 2010 @ [13:50](https://blog.didierstevens.com/programs/pdf-tools/#comment-37662)

-

@sheldor: No, I didn’t code an explicit file size limit. I tried on PDF files up to 41MB without problems. How large is your PDF file?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 8 February 2010 @ [14:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-37664)

-

I have this issue with PDFs 20MB and up! Well,.. then there must be another reason! Still can’t figure it out!
Anyhow, thank you!

Comment by sheldor — Monday 8 February 2010 @ [17:27](https://blog.didierstevens.com/programs/pdf-tools/#comment-37665)

-

@sheldor: If you can point me to an online PDF document that causes the problem you experience, I’ll take a look at it.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 8 February 2010 @ [20:24](https://blog.didierstevens.com/programs/pdf-tools/#comment-37666)

-

[…] Didier Stevens has provided a fantastic resource and tools for analyzing PDF files. Some of these resources have been incorporated into VirusTotal. Didier Stevens: https://blog.didierstevens.com/programs/pdf-tools/ […]

Pingback by [PDF Exploitation & Forensic Resources « MadMark's Blog](http://kohi10.wordpress.com/2010/02/16/pdf-exploitation-forensic-resources/) — Tuesday 16 February 2010 @ [19:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-37710)

-

[…] we see in Pyew? The output of PDFId (a great tool by Didier Stevens) is shown as well as the hexadecimal output of the first block (512 […]

Pingback by [Unintended Results » Blog Archive » Analyzing PDF exploits with Pyew](http://joxeankoret.com/blog/?p=95) — Sunday 21 February 2010 @ [14:50](https://blog.didierstevens.com/programs/pdf-tools/#comment-37733)

-

Very cool man, I tried to use PDF tools to unwind a drive-by ZeuS pdf infection. Unfortunately, it gave me some problems because I was using a newer version of Python (and it looks like the El Fiesta Exploit kit might use some kind of different zLib encoding to compress its payloads). Good stuff though!
http://www.mdl4.com/2010/02/28/reverse-engineering-zeus/

Comment by mdl4 — Tuesday 2 March 2010 @ [11:15](https://blog.didierstevens.com/programs/pdf-tools/#comment-37765)

-

[…] https://blog.didierstevens.com/programs/pdf-tools/ […]

Pingback by [PDF Malware Analysis Tools | Tahir's Security Blog](http://securityblog.igearnetwork.com/?p=130) — Wednesday 31 March 2010 @ [18:11](https://blog.didierstevens.com/programs/pdf-tools/#comment-37999)

-

[…] For the PDF analysis, I used the excellent PDF-Tools from Didier Stevens that can be located here. The main python script that was used was pdf-parser and pdfid seen […]

Pingback by [PDF Launch Command without javascript - isolated-threat](http://www.isolatedthreat.com/?p=214) — Thursday 1 April 2010 @ [10:47](https://blog.didierstevens.com/programs/pdf-tools/#comment-38051)

-

[…] the PDF with Didier Steven’s pdfid.py showed that there was an OpenAction in the PDF, but no JavaScript. Interesting. Using […]

Pingback by [/Launch Malicious PDF | Portable Digital Video Recorder](http://www.portable-digital-video-recorder.com/launch-malicious-pdf/) — Tuesday 27 April 2010 @ [22:48](https://blog.didierstevens.com/programs/pdf-tools/#comment-38370)

-

[…] @ 10:11 Now that malicious PDFs using the /Launch action become more prevalent, I release a new PDFiD version to detect (and disarm) the /Launch […]

Pingback by [Update: PDFiD Version 0.0.11 to Detect /Launch « Didier Stevens](https://blog.didierstevens.com/2010/04/29/update-pdfid-version-0-0-11-to-detect-launch/) — Thursday 29 April 2010 @ [10:11](https://blog.didierstevens.com/programs/pdf-tools/#comment-38375)

-

[…] الباحث ديدر ستفينز أداة جديدة (pdfid.py)، تساعد الكشف عمّا إذا كان ملف pdf يحتوي […]

Pingback by [اطلاق أداة جديدة تقوم بالكشف على ملفات pdf قبل تشغيلها | مجتمع الحماية العربي](http://www.security4arabs.com/2010/04/29/new-tool-to-scan-pdf-documents-for-keywords/) — Thursday 29 April 2010 @ [18:48](https://blog.didierstevens.com/programs/pdf-tools/#comment-38381)

-

[…] PDFiD v0.0.11 – didierstevens.com I release a new PDFiD version to detect (and disarm) the /Launch action. […]

Pingback by [Week 17 in Review – 2010 | Portable Digital Video Recorder](http://www.portable-digital-video-recorder.com/week-17-in-review-%e2%80%93-2010/) — Monday 3 May 2010 @ [6:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-38399)

-

[…] PDFiD v0.0.11 – didierstevens.com I release a new PDFiD version to detect (and disarm) the /Launch action. […]

Pingback by [Week 17 in Review – 2010 | Infosec Events](http://infosecevents.net/2010/05/02/week-17-in-review-2010/) — Tuesday 4 May 2010 @ [9:40](https://blog.didierstevens.com/programs/pdf-tools/#comment-38405)

-

I have a large volume of pdfs coming soon from a vendor, does pdfid.py handle compressed (gzip, bzip2, zip) files? If so, how. If not is it something that can be worked around or accomplished with another program?

BTW
really appreciate your work, your blog and website have been a treasure trove of information.

Comment by Johnny — Tuesday 4 May 2010 @ [16:34](https://blog.didierstevens.com/programs/pdf-tools/#comment-38411)

-

@Johnny No, but it has an option to scan all files in a folder. Unzip all PDFs to a folder and use that option.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 4 May 2010 @ [20:57](https://blog.didierstevens.com/programs/pdf-tools/#comment-38413)

-

[…] non fidate può essere utile eseguire un’analisi automatizzata ricorrendo al tool pdfid.py di Didier Stevens. Si tratta di uno script, funzionante su Windows, Linux e qualsiasi sistema che […]

Pingback by [Analizzare e “disinfettare” file PDF con pdfid](http://www.bufferoverflow.it/2010/05/04/analizzare-e-disinfettare-file-pdf-con-pdfid/) — Tuesday 4 May 2010 @ [21:13](https://blog.didierstevens.com/programs/pdf-tools/#comment-38414)

-

[…] fonctions suspectes cachées dans le PDF (à savoir exécution de Javascript et d'exécutables) : pdfid et pdf-parser. Avant de découvrir les fonctionnalités de ces deux outils, il est important de connaitre la […]

Pingback by [Les outils d’analyse de PDF « Elevenses blog](http://blog.elevenses.fr/?p=11) — Monday 10 May 2010 @ [14:45](https://blog.didierstevens.com/programs/pdf-tools/#comment-38504)

-

[…] Il n'a pas fallu longtemps pour que ce PoC (Proof Of Concept) ne soit utilisé par dans des PDF malicieux, permettant ainsi d'installer un trojan sur la machine cible. Didier Stevens a développé deux scripts Python permettant d'analyser les PDF pour y découvrir d'éventuelles fonctions suspectes cachées dans le PDF (entre autres exécution de Javascript et d'exécutables) : pdfid et pdf-parser. […]

Pingback by [Les outils d’analyse de PDF « ELEVENSES BLOG](http://elevenses.fr/blog/index.php/2010/05/les-outils-danalyse-de-pdf/) — Thursday 27 May 2010 @ [15:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-38688)

-

[…] }; "> — Classificat com a: Eines — Comentari (0) — Lectures: 2130 abril 2010PDFID.py és una eina que analitza un fitxer PDF i mostra les característiques de les que fa ús. Per […]

Pingback by [Eina: PDFID.py | L’home dibuixat](http://caballe.cat/wp/eina-pdfidpy/) — Saturday 5 June 2010 @ [20:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-38819)

-

[…] you used my pdf-parser, you’ve also encountered a problem. The objects lack the endobj keyword. A simple solution: […]

Pingback by [Solving the Win7 Puzzle « Didier Stevens](https://blog.didierstevens.com/2010/06/25/solving-the-win7-puzzle/) — Friday 25 June 2010 @ [9:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-38977)

-

With PDFiD, I’ve noticed I get a lot of false positives on the /JS and /AA tags, since in most cases (that I’ve looked at) they seem to be simply text in a compressed image or something similar. I haven’t seen a /JS used on it’s own for Javascript, but it does seem that if there is a /JS then there is also a /S/JavaScript to go with it.

Is this always the case, or just in the samples I’ve looked at so far (same applies for AA)? Finding the text JavaScript is much less likely to lead to a false positive than JS.

Comment by Russell — Monday 28 June 2010 @ [23:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-38994)

-

@Russell Good observation, I almost always see /JavaScript together with /JS. I’ve seen some cases without /JavaScript, but it looks like these were non-functional.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 29 June 2010 @ [9:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-38997)

-

This is a complementing post. Work you have done is adorable I liked it how do you get this all in mind??? 🙂 but anyways I found this great and keep going. keep making us explore each security aspect.
thanks
Sushant

Comment by Sushant — Friday 9 July 2010 @ [10:50](https://blog.didierstevens.com/programs/pdf-tools/#comment-39069)

-

[…] plików PDF: Didier’s PDF tools, Origami framework, Jsunpack-n, […]

Pingback by [» REMnux — programy do analizy złośliwego oprogramowania -- Niebezpiecznik.pl --](http://niebezpiecznik.pl/post/remnux-programy-do-analizy-zlosliwego-oprogramowania/) — Monday 12 July 2010 @ [9:15](https://blog.didierstevens.com/programs/pdf-tools/#comment-39081)

-

[…] any known viruses, when run through a total of 32 anti-virus programs. Processing the file with PDFiD reveals that the file contains no JavaScript objects, but it does contain a single JS object. […]

Pingback by [Al-Qaeda Magazine is Cupcake Recipe Book | Public Intelligence](http://publicintelligence.net/al-qaeda-magazine-is-cupcake-recipe-book/) — Monday 12 July 2010 @ [21:18](https://blog.didierstevens.com/programs/pdf-tools/#comment-39085)

-

Possible bug: PDFiD fails sometime in cPDFEOF when using –extra option for entropy, stating cntCharsAfterLastEOF doesn’t exist. Defining it in init seems to fix the issue.

Other Notes: Is it possible to use pdf-parser to parse pdf-parser output? For example, I can see a use of this when using pdf-parser to obtain contents of object streams, but then it would be nice if it were possible to use pdf-parser on THAT output to display all Launch commands, for example (similar to piping into PDFiD, but actually seeing the contents instead of just the count). Then again, object stream structure is a bit different so perhaps that’s why it doesn’t play nice. I haven’t figured it out yet…

Comment by Russell — Thursday 15 July 2010 @ [23:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-39109)

-

[…] PDF analysis: Didier’s PDF tools, Origami framework, Jsunpack-n, […]

Pingback by [Malware Analysis Tools Set Up for Linux « Wikihead's Blog](http://wikihead.wordpress.com/2010/07/17/malware-analysis-tools-set-up-for-linux/) — Saturday 17 July 2010 @ [9:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-39117)

-

@Russell Thanks for the feedback. I’ve had similar reports, and defining it in the init fixes the issue, but I also would like to understand the bug. Can you share a sample?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 19 July 2010 @ [11:30](https://blog.didierstevens.com/programs/pdf-tools/#comment-39152)

-

[…] and Flare. Furthermore, it contains several applications for analyzing malicious PDFs, such as the Didier Steven’s analysis tools. The OS also provides a lot of tools for de-obfucating JavaScript, including Rhino […]

Pingback by [New Linux OS REMnux Designed For Reverse Engineering Malware « The FORWARD project blog](http://blogs.ict-forward.eu/forward/new-linux-os-remnux-designed-for-reverse-engineering-malware/) — Tuesday 20 July 2010 @ [10:36](https://blog.didierstevens.com/programs/pdf-tools/#comment-39192)

-

[…] Il n'a pas fallu longtemps pour que ce PoC (Proof Of Concept) ne soit utilisé par dans des PDF malicieux, permettant ainsi d'installer un trojan sur la machine cible. Didier Stevens a développé deux scripts Python permettant d'analyser les PDF pour y découvrir d'éventuelles fonctions suspectes cachées dans le PDF (entre autres exécution de Javascript et d'exécutables) : pdfid et pdf-parser. […]

Pingback by [Les outils d’analyse de « ELEVENSES BLOG](http://blog.elevenses.fr/index.php/2010/08/02/les-outils-danalyse-de/) — Monday 2 August 2010 @ [8:29](https://blog.didierstevens.com/programs/pdf-tools/#comment-39378)

-

[…] I highly recommend any security conscious sysadmins add this tool to their toolkit, as the number of PDF exploits are likely to continue rising for the forseeable future. PDFiD can be downloaded from Didier Stevens website at https://blog.didierstevens.com/programs/pdf-tools. […]

Pingback by [PDFiD: Analyzing suspicious PDFs « Life as a cmddot](http://cmddot.wordpress.com/2010/08/03/pdfid-analyzing-suspicious-pdfs/) — Tuesday 3 August 2010 @ [7:03](https://blog.didierstevens.com/programs/pdf-tools/#comment-39389)

-

[…] Font Format) stream that looked suspicious enough for us to decode it (thanks to the excellent pdf-parser tool from Didier Stevens). In the now clear-text stream, we could identify at least one manifest […]

Pingback by [iPhone 4 / iPad: The Keys Out Of Prison | Fortinet Security Blog](http://blog.fortinet.com/iphone-4-ipad-the-keys-out-of-prison/) — Thursday 5 August 2010 @ [8:27](https://blog.didierstevens.com/programs/pdf-tools/#comment-39418)

-

[…] – Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD: pdf-parser and […]

Pingback by [Security tools « Eikonal Blog](http://eikonal.wordpress.com/2010/07/28/security-tools/) — Monday 9 August 2010 @ [14:29](https://blog.didierstevens.com/programs/pdf-tools/#comment-39447)

-

[…] i PDF-tools di Didier Stevens si riesce ad analizzare la struttura dei file PDF, anche se tutti risultano […]

Pingback by [Honeynet Project: Challenge 3/2010 (II parte) « Il non-blog di Mario Pascucci](http://www.ismprofessional.net/pascucci/index.php/2010/08/honeynet-project-challenge-32010-ii-parte/) — Thursday 19 August 2010 @ [3:04](https://blog.didierstevens.com/programs/pdf-tools/#comment-39587)

-

Is there a licensing agreement with using pdfid or pdf-parser? Can it be used as part of software that will be sold?

Comment by Jon — Thursday 2 September 2010 @ [14:59](https://blog.didierstevens.com/programs/pdf-tools/#comment-39738)

-

[…] Here is a PDF template for the 010 Editor. It’s particularly useful for malformed PDF files, like this example with PDFUnknown structures: […]

Pingback by [PDFTemplate « Didier Stevens](https://blog.didierstevens.com/2010/09/03/pdftemplate/) — Friday 3 September 2010 @ [10:36](https://blog.didierstevens.com/programs/pdf-tools/#comment-39747)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and mPDF) […]

Pingback by [Python tools for penetration testers | Secondary Logic – There is always a theory !!!](http://slogic.net/training/python-tools-for-penetration-testers) — Saturday 4 September 2010 @ [8:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-39764)

-

@Jon Can’t contact you, you didn’t provide an e-mail address.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 5 September 2010 @ [21:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-39793)

-

[…] & pdftools – Two frameworks for analysing malicious PDF […]

Pingback by [Mercury – Live Honeypot DVD « Infosanity's Blog](http://blog.infosanity.co.uk/2010/09/22/mercury-live-honeypot-dvd/) — Wednesday 22 September 2010 @ [14:26](https://blog.didierstevens.com/programs/pdf-tools/#comment-40006)

-

[…] https://blog.didierstevens.com/programs/pdf-tools/ […]

Pingback by [BruCON 2010 : Day 0×2 | Peter Van Eeckhoutte's Blog](http://www.corelan.be:8800/index.php/2010/09/25/brucon-2010-day-0x2/) — Saturday 25 September 2010 @ [20:54](https://blog.didierstevens.com/programs/pdf-tools/#comment-40028)

-

[…] Font Format) stream that looked suspicious enough for us to decode it (thanks to the excellent pdf-parser tool from Didier Stevens). In the now clear-text stream, we could identify at least one manifest […]

Pingback by [» iPhone 4 / iPad: The Keys Out Of Prison](http://blog.muchomorek.eu/?p=4) — Saturday 25 September 2010 @ [22:53](https://blog.didierstevens.com/programs/pdf-tools/#comment-40030)

-

[…] Analyse verdächtiger Dateien hält Stevens verschiedene selbstentwickelte Tools auf seiner Website vorrätig, deren Nutzung für technisch unversierte Lesefreunde allerdings wenig praktikabel ist. Weil schon […]

Pingback by [Schadhafte pdf-Dateien identifizieren » Software » lesen.net](http://www.lesen.net/ebooks/schadhafte-pdf-dateien-identifizieren-4033/) — Monday 27 September 2010 @ [17:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-40084)

-

[…] Didier Stevens’ PDF tools Over the weekend, I was reading Didier Stevens’ chapter on malicious PDF analysis and I have to give credit to him to break down the technical part of a PDF into something simple and easy to understand (er … maybe I am the only one who is coming to term with PDF for the first time). Reading the article brought me to his PDF-tools. pdfid and pdf-parser is definitely a must try if you really want to get your hands-on on PDF analysis. […]

Pingback by [Hunger 4 Knowledge #10 « David Koepi](http://davidkoepi.wordpress.com/2010/10/03/h4k010/) — Sunday 3 October 2010 @ [1:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-40151)

-

[…] and wonder where to start. Get a Linux distro, install Python, and use Didier Stevens PDF parser [Didier Stevens]. This is a script that will structure all the objects for you, making them more readable. This is […]

Pingback by [Analyzing malicious PDFs](http://detect.no:8090/?p=17) — Monday 11 October 2010 @ [19:03](https://blog.didierstevens.com/programs/pdf-tools/#comment-40231)

-

[…] and dump the zipped sections of a PDF file. In my opinion, the best are Didier Steven’s PDF Tools. Unfortunately, in this case, none of them worked for me, so I had to do it manually. I selected […]

Pingback by [Reverse engineering a Facebook ZeuS infection](http://www.mdl4.com/2010/02/reverse-engineering-zeus/) — Monday 25 October 2010 @ [2:24](https://blog.didierstevens.com/programs/pdf-tools/#comment-40377)

-

[…] was about malicious PDF analysis, given by “Mr PDF” himself, Didier Stevens. Using his toolbox, several malicious PDF files were analyzed with a growing complexity. Very interesting and this […]

Pingback by [Hack.lu Day #1 Wrap-up « /dev/random](http://blog.rootshell.be/2010/10/27/hack-lu-day-1-wrap-up/) — Wednesday 27 October 2010 @ [21:51](https://blog.didierstevens.com/programs/pdf-tools/#comment-40400)

-

[…] pdfid.py and pdf-parser.py. Get them from from Didier Stevens PDF Tools page. […]

Pingback by [Analysing a Malicious PDF Document](http://www.youstar.org.ru/analysing-a-malicious-pdf-document.html) — Saturday 6 November 2010 @ [12:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-40464)

-

[…] Download: click here […]

Pingback by [Malware Analysis: Handy tools for analysing PDF files « Brainfold's blog](http://brainfoldsecurity.com/2010/11/16/malware-analysis-handy-tools-for-analysing-pdf-files/) — Tuesday 16 November 2010 @ [3:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-40514)

-

[…] I ran pdf-parser.py against the pdf file. The output indicated that there were 2 “interesting” objects […]

Pingback by [Malicious pdf analysis : from price.zip to flashplayer.exe | Peter Van Eeckhoutte's Blog](http://www.corelan.be:8800/index.php/2010/11/18/malicious-pdf-analysis-from-price-zip-to-flashplayer-exe/) — Thursday 18 November 2010 @ [13:50](https://blog.didierstevens.com/programs/pdf-tools/#comment-40524)

-

Didier,
Is there a way to embed a .exe in a pdf and have it automatically execute when the pdf is opened? I have tried to use your .py tool but it does not run the .exe after being opened.
Thanks,
Willie

Comment by Willie — Saturday 20 November 2010 @ [6:37](https://blog.didierstevens.com/programs/pdf-tools/#comment-40536)

-

@Willie That’s normal, Adobe Reader doesn’t allow you to extract executable files. I found one way to deliver executable files: https://blog.didierstevens.com/2010/03/29/escape-from-pdf/
But Adobe has updated their reader to prevent this /Launch action.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 20 November 2010 @ [8:49](https://blog.didierstevens.com/programs/pdf-tools/#comment-40538)

-

[…] obvious choice were the pdftools from Didier Stevens. What […]

Pingback by [Malware PDF. Analysis of a very simple sample. | Brundle Lab](http://blog.carlosgarciaprado.com/?p=441) — Tuesday 23 November 2010 @ [18:35](https://blog.didierstevens.com/programs/pdf-tools/#comment-40560)

-

[…] Didier’s own pdf-parser.py, the PDF’s meta information for the creation date is as […]

Pingback by [Praetorian Prefect | The Anonymous PR Guy and a Greece Connection](http://praetorianprefect.com/archives/2010/12/the-anonymous-pr-guy-and-a-greece-connection/) — Sunday 12 December 2010 @ [0:57](https://blog.didierstevens.com/programs/pdf-tools/#comment-41177)

-

[…] Il n’a pas fallu longtemps pour que ce PoC (Proof Of Concept) ne soit utilisé par dans des PDF malicieux, permettant ainsi d’installer un trojan sur la machine cible. Didier Stevens a développé deux scripts Python permettant d’analyser les PDF pour y découvrir d’éventuelles fonctions suspectes cachées dans le PDF (entre autres exécution de Javascript et d’exécutables) : pdfid et pdf-parser. […]

Pingback by [Secur-IT](http://www.secur-it.net/2011/01/05/les-outils-danalyse-de-pdf/) — Thursday 6 January 2011 @ [13:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-41371)

-

[…] second Didier Steven’s PDF Tools. PDF Tools includes pdf-parser.py, make-pdf-javascript.py, and pdfid.py. Pdf-parser and pdfid are […]

Pingback by [Tools](http://blog.jeremyscott.org/?p=398) — Saturday 29 January 2011 @ [18:34](https://blog.didierstevens.com/programs/pdf-tools/#comment-41665)

-

[…] PDF-Parser (https://blog.didierstevens.com/programs/pdf-tools/) […]

Pingback by [Attributes of a Zero Dollar Malware Analysis Environment « SecAnalysis](http://secanalysis.com/?p=88) — Tuesday 8 February 2011 @ [3:07](https://blog.didierstevens.com/programs/pdf-tools/#comment-41754)

-

hi!
i tried using your make-pdf-javascript.py. i gave it a javascript file which executes notepad, but though it got embedded( i checked it with pdf-parser.py), it did not run.
wen i run the js file directly it executes, but when i embed it , it does not run.

Comment by pret — Tuesday 15 February 2011 @ [11:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-41826)

-

@pret And how do you start Notepad?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 15 February 2011 @ [17:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-41831)

-

i ran notepad directly from js file using ws.run command , but wen i run the script outside pdf, it runs, wen i embed it in pdf and run, it gets embedded but does not run. pls tell how can i make it run.

Comment by pret — Thursday 17 February 2011 @ [5:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-41835)

-

@pret You are using a Windows JavaScript feature, that’s not supported by Adobe’s JavaScript. There is no feature to run arbitrary programs.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 17 February 2011 @ [7:06](https://blog.didierstevens.com/programs/pdf-tools/#comment-41837)

-

I am new to Python. I have installed Python 27 and have tried running pdfid.py with no success.
The syntax >>>pdfid.py MidtermChazaraQuestions.pdf returns Invalid Syntax error in the input.
What am I doing wrong? It is extremely important I analyze this file. It may be the key to the identity theft that is destroying me. Please, help!

Comment by Joseph Ainbinder — Friday 18 February 2011 @ [19:33](https://blog.didierstevens.com/programs/pdf-tools/#comment-41861)

-

@joseph You need to use 2.6, a module in 2.7 was deprecated.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 18 February 2011 @ [20:07](https://blog.didierstevens.com/programs/pdf-tools/#comment-41862)

-

[…] pdf-parser.py – https://blog.didierstevens.com/programs/pdf-tools/ (éditer le source pour modifier la version maximale de python acceptée)- pdfid.py – […]

Pingback by [escape from PDF | Linux-backtrack.com](http://www.linux-backtrack.com/2011/02/escape-from-pdf/) — Saturday 19 February 2011 @ [21:20](https://blog.didierstevens.com/programs/pdf-tools/#comment-41881)

-

[…] – Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD: pdf-parser and […]

Pingback by [Malware analysis « Eikonal Blog](http://eikonal.wordpress.com/2011/02/28/malware-analysis/) — Monday 28 February 2011 @ [16:33](https://blog.didierstevens.com/programs/pdf-tools/#comment-41987)

-

[…] pdf-parser.py […]

Pingback by [PDF Analysis for Humans « P4r4n0id Reversing Lab](http://p4r4n0id.com/?p=389) — Friday 18 March 2011 @ [15:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-42114)

-

i had a problem with “make-pdf-javascript”

first use with the original package:

C:\Documents and Settings\abdelmoumen bacetti\mpdf1>python make-pdf-javascript.py test.pdf
File “make-pdf-javascript.py”, line 29
print ”
^
SyntaxError: invalid syntax
###############################################################################################
so i changed the lines 29,30,31,32,33,55,61 in “make-pdf-javascript.py” and line 110 in “mPDF.py” because the “prints” are without parenthesis
###############################################################################################
after fixing the prints problem:

C:\Documents and Settings\abdelmoumen bacetti\mpdf>python make-pdf-javascript.py down.pdf
Traceback (most recent call last):
File “make-pdf-javascript.py”, line 71, in
Main()
File “make-pdf-javascript.py”, line 44, in Main
oPDF.stream(5, 0, ‘BT /F1 12 Tf 100 700 Td 15 TL (JavaScript example) Tj ET’)
File “C:\Documents and Settings\abdelmoumen bacetti\mpdf\mPDF.py”, line 69, in stream
self.appendBinary(streamdata)
File “C:\Documents and Settings\abdelmoumen bacetti\mpdf\mPDF.py”, line 39, in appendBinary
fPDF.write(str)
TypeError: ‘str’ does not support the buffer interface
###############################################################################################
config:

Windows XP SP2
Python 3.2

Comment by bmoumen — Sunday 10 April 2011 @ [13:37](https://blog.didierstevens.com/programs/pdf-tools/#comment-42349)

-

@bmoumen Yes, my Python programs are not designed for Python 3. Neither do most of my programs work on 2.7, because of a deprecated module I use to parse command lines. It’s something I hope to solve in a near future (i.e. make my Python programs compatible with Python 2.5, 2.6, 2.7 and 3.x).

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 11 April 2011 @ [7:05](https://blog.didierstevens.com/programs/pdf-tools/#comment-42355)

-

[…] of python tools which can be used for analysing PDFs. I downloaded two of his tools from this page https://blog.didierstevens.com/programs/pdf-tools/, pdf-parser.py and […]

Pingback by [Solving the Security BSides London Challenge – Number 2 | 4armed](http://4armed.wordpress.com/2011/04/15/solving-the-security-bsides-london-challenge-number-2/) — Thursday 21 April 2011 @ [14:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-42494)

-

[…] a look at my Analyzing Malicious Documents Cheat Sheet. From the tools perspective, Didier Steven’s pdf-parser is an all-time favorite. Another excellent tool, which sports a user-friendly GUI, is PDF Stream […]

Pingback by [How to Extract Flash Objects from Malicious PDF Files](http://blogs.sans.org/computer-forensics/2011/05/04/extract-flash-from-malicious-pdf-files/) — Wednesday 4 May 2011 @ [15:18](https://blog.didierstevens.com/programs/pdf-tools/#comment-42655)

-

[…] PDF Tools by Didier Stevens is the classic toolkit that established the foundation for our understanding of the PDF analysis process. It includes pdfid.py to quickly scan the PDF for risky objects and, most usefully, pdf-parser.py to examine their contents. […]

Pingback by [6 Free Tools for Analyzing Malicious PDF Files « AfterShell.com](http://www.aftershell.com/2011/05/11/6-free-tools-for-analyzing-malicious-pdf-files/) — Wednesday 11 May 2011 @ [17:46](https://blog.didierstevens.com/programs/pdf-tools/#comment-42754)

-

[…] Signatures work with a few open source tools. The first one is pdf-parser.py which is part of the PDF Tools by Didier […]

Pingback by [The Anatomy of a PDF Signature < experiment nr.: 1598](http://x1598.at/2011/05/the-anatomy-of-a-pdf-signature/) — Wednesday 11 May 2011 @ [19:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-42756)

-

[…] But did you notice the inclusion of my PDFiD and pdf-parser tools? […]

Pingback by [BackTrack 5 Includes PDFiD and pdf-parser « Didier Stevens](https://blog.didierstevens.com/2011/05/12/backtrack-5-includes-pdfid-and-pdf-parser/) — Thursday 12 May 2011 @ [21:13](https://blog.didierstevens.com/programs/pdf-tools/#comment-42768)

-

[…] my PDF tools […]

Pingback by [Malicious PDF Analysis Workshop Screencasts « Didier Stevens](https://blog.didierstevens.com/2011/05/25/malicious-pdf-analysis-workshop-screencasts/) — Wednesday 25 May 2011 @ [15:59](https://blog.didierstevens.com/programs/pdf-tools/#comment-43088)

-

[…] Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and […]

Pingback by [基于python渗透测试工具](http://www.codesec.info/pythonpentest.html) — Sunday 29 May 2011 @ [3:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-43200)

-

[…] here. In the past I have also used […]

Pingback by [Checking a PDF for exploits Drija](http://drija.com/pdf/56206/checking-a-pdf-for-exploits/) — Thursday 9 June 2011 @ [4:14](https://blog.didierstevens.com/programs/pdf-tools/#comment-43525)

-

[…] encodings to name like JBIG2Decode and DCTDecode. FlateDecode usually can be decoded by using pdf-parser […]

Pingback by [Analyzing malicious PDF « lab69](http://lab69.com/?p=91) — Thursday 23 June 2011 @ [17:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-43850)

-

[…] suo interno l’exploit vero e proprio. Sinceramente non sono riuscito a decomprimerlo né con pdf-parser di Didier Stevens, né con PDF Stream Dumper, né con Ghostscript come spiegato qui. Diciamo che […]

Pingback by [Jailbreakme: ecco come funziona il jailbreak per iPad 2](http://www.bufferoverflow.it/2011/07/06/jailbreakme-ecco-come-funziona-il-jailbreak-per-ipad-2/) — Wednesday 6 July 2011 @ [21:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-44174)

-

Hi Didier,

May I ask you which tools are you using for Python (debuggers,..)

Thanks

Comment by [zudqg](http://www.laposte.net/) — Wednesday 20 July 2011 @ [13:59](https://blog.didierstevens.com/programs/pdf-tools/#comment-44510)

-

[…] primero que nos interesa es determinar el contenido del PDF y para ello utilizamos las PDFtools que nos permiten analizar PDF. Ejecutamos la herramienta pdfid para ver el contenido del fichero y […]

Pingback by [Reconstructing JavaScript Exploit « Simon Roses Femerling – Blog](http://www.simonroses.com/reconstructing-javascript-exploit/) — Wednesday 20 July 2011 @ [20:32](https://blog.didierstevens.com/programs/pdf-tools/#comment-44513)

-

@zudqg I’m going to disappoint you, for Python, I just use a text editor.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 21 July 2011 @ [6:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-44519)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and mPDF) […]

Pingback by [Repost:Lista de ferramentas de segurança feitas em Python. « VSLA – Virtual Security Labs Anywhere](http://firebitsbr.wordpress.com/2011/08/01/repostlista-de-ferramentas-de-seguranca-feitas-em-python/) — Monday 1 August 2011 @ [15:51](https://blog.didierstevens.com/programs/pdf-tools/#comment-44825)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and mPDF) […]

Pingback by [Attack Attack » Python tools for penetration testers](http://blog.dis9.com/archives/python-tools-for-penetration-testers/) — Monday 8 August 2011 @ [4:17](https://blog.didierstevens.com/programs/pdf-tools/#comment-45020)

-

[…] javascript heap overflow in PDF. More info to come. I used Didier Steven’s pdfid and pdf-parser to extract the javascript. The Javascript which is called when the document is opened creates a […]

Pingback by [The Spy Hunter, Part II – Solution « wirewatcher](http://wirewatcher.wordpress.com/2011/08/14/the-spy-hunter-part-ii-solution/) — Sunday 14 August 2011 @ [20:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-45220)

-

Just a “wowie” comment – thanks for sharing these tools, they’re fantastic.

Comment by B. Oceander — Monday 26 September 2011 @ [14:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-46168)

-

Hi Didier,

Do you have a tool, or know of a tool, that can take an existing PDF and add JS to it? I would like the ability to add javascript to multiple existing files. It would basically have the same functionality as your current make-pdf.py script, but you’d provide it an existing PDF, as well as a JS file that it would be merged with.

Thx for your help!

Comment by Sagui — Thursday 13 October 2011 @ [12:35](https://blog.didierstevens.com/programs/pdf-tools/#comment-46551)

-

@Sagui Look for phptk, it can merge 2 PDF files.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 14 October 2011 @ [20:51](https://blog.didierstevens.com/programs/pdf-tools/#comment-46584)

-

Hi Didier, thanks for providing these tools, would you have any objection to me adding them to a public github repo so people can contribute any fixes/extensions they have?

Comment by Tom — Sunday 16 October 2011 @ [13:03](https://blog.didierstevens.com/programs/pdf-tools/#comment-46628)

-

@Tom No problem, let me know where.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 16 October 2011 @ [13:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-46629)

-

All done https://github.com/thomcarver/pdf-tools

Comment by Tom — Sunday 16 October 2011 @ [15:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-46632)

-

Hello,

Can some one help me to figure out how to use this pdfid tool. I have python inerpretor installed but would like to know how I can specify which file or directory I want this tool to parse.

I am new to Python.

Comment by Ishwar — Tuesday 18 October 2011 @ [12:20](https://blog.didierstevens.com/programs/pdf-tools/#comment-46672)

-

@Ishwar: I assume you’re running Windows? Then you install Python 2.X (not version 3), open a command line (cmd.exe) and type pdfid.py test.pdf where test.pdf is the file you want to check.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 19 October 2011 @ [16:52](https://blog.didierstevens.com/programs/pdf-tools/#comment-46700)

-

[…] purpose, or write a custom tool ourselves. For the sake of this tutorial, I’ll stick with Didier Steven’s excellent “make-pdf” python script (which uses the mPDF […]

Pingback by [Exploit writing tutorial part 11 : Heap Spraying Demystified | Corelan Team](https://www.corelan.be/index.php/2011/12/31/exploit-writing-tutorial-part-11-heap-spraying-demystified/) — Saturday 31 December 2011 @ [23:32](https://blog.didierstevens.com/programs/pdf-tools/#comment-49187)

-

Hello Didier,

Thank you for providing these tools.

I have scanned a PDF I suspect may be malicious with your pdfid script, and it returned 0 for everything but ” /AcroForm 1″. I see above that acroform is not described in the pdfid summary. Could you please tell the meaning of this, and how to tell whether it is harmful?

Comment by Inkblots — Wednesday 11 January 2012 @ [19:40](https://blog.didierstevens.com/programs/pdf-tools/#comment-49556)

-

@InkBlots Take a look at my PDF workshop, I’ve an exercise for AcroForm. AcroForm can contain JavaScript that is executed when a document is opened.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 11 January 2012 @ [20:15](https://blog.didierstevens.com/programs/pdf-tools/#comment-49558)

-

[…] PDF-Parser (https://blog.didierstevens.com/programs/pdf-tools/) […]

Pingback by [Attributes of a Zero Dollar Malware Analysis System « secanalysis.com](http://secanalysis.com/?p=73) — Monday 16 January 2012 @ [17:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-49771)

-

[…] Related great tools: https://blog.didierstevens.com/programs/pdf-tools/ […]

Pingback by [Re: pdf attacks vectors | Net Cleaner](http://www.netcleaner.it/re-pdf-attacks-vectors/) — Saturday 21 January 2012 @ [18:29](https://blog.didierstevens.com/programs/pdf-tools/#comment-49952)

-

[…] First we use a new version of my PDF tools to create a PDF file with embedded file: […]

Pingback by [Teensy PDF Dropper Part 2 « Didier Stevens](https://blog.didierstevens.com/2012/02/27/teensy-pdf-dropper-part-2/) — Monday 27 February 2012 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-51040)

-

[…] https://blog.didierstevens.com/programs/pdf-tools/ http://www.mozilla.org/js/spidermonkey/ https://code.google.com/p/jsunpack-n/ http://malzilla.sourceforge.net/ […]

Pingback by [| web güvenlik , SKaracan.com , Web Güvenlik , Sistem Güvenliği ve Kişisel Güvenliğe Dair Herşey |](http://skaracan.com/kisisel-guvenlik/205.html) — Friday 9 March 2012 @ [17:19](https://blog.didierstevens.com/programs/pdf-tools/#comment-51351)

-

[…] PDF Tools – https://blog.didierstevens.com/programs/pdf-tools/ […]

Pingback by [SecuraBit Episode 32: PDF Love! « SecuraBit](http://dev.securabit.com/2009/05/20/securabit-episode-32-pdf-love/) — Tuesday 13 March 2012 @ [15:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-51510)

-

[…] can find these tools on the PDF Tools page. Like this:LikeBe the first to like this post. Leave a […]

Pingback by [Update: PDFid And pdf-parser « Didier Stevens](https://blog.didierstevens.com/2012/03/14/update-pdfid-and-pdf-parser/) — Wednesday 14 March 2012 @ [9:15](https://blog.didierstevens.com/programs/pdf-tools/#comment-51530)

-

[…] PDF-parser Wieloplatformowy, konsolowy program do przetwarzania i analizy dokumentów PDF. Potrafi wyodrębnić surowe dane z dokumentu takie jak skompresowane obrazy. Dobrze radzi sobie z uszkodzonymi oraz zaciemnionymi plikami. […]

Pingback by [Edytory PDF 2 | Linuxiarze.pl](http://linuxiarze.pl/edytory-pdf2/) — Saturday 24 March 2012 @ [0:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-51812)

-

[…] тематику анализа PDF файлов. Getting Owned By Malicious PDF – Analysis PDF Tools от Didier […]

Pingback by [Информация по анализу PDF файлов. « clickf1 web log.](http://corehook.wordpress.com/2012/04/02/%d0%b8%d0%bd%d1%84%d0%be%d1%80%d0%bc%d0%b0%d1%86%d0%b8%d1%8f-%d0%bf%d0%be-%d0%b0%d0%bd%d0%b0%d0%bb%d0%b8%d0%b7%d1%83-pdf-%d1%84%d0%b0%d0%b9%d0%bb%d0%be%d0%b2/) — Monday 2 April 2012 @ [8:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-52134)

-

[…] – look forcat_open_xml.pl; other tools available, as well Skype Extractor – PDF Tools – from Didier Stevens; some of Didier’s tools have been incorporated into the VirusTotal […]

Pingback by [Herouxapps (Home of Freeware)](http://herouxapps.com/wordpress/archives/9893) — Wednesday 18 April 2012 @ [23:06](https://blog.didierstevens.com/programs/pdf-tools/#comment-52529)

-

Fantastic tools! Many thanks, Didier. I had a pdf send to me by would-be fraudsters. It was a great relief to find that the document itself was not malicious.

Comment by John — Wednesday 9 May 2012 @ [19:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-53159)

-

[…] Dider Stevens的PDFiD.pf和pdf-parser.py (https://blog.didierstevens.com/programs/pdf-tools/)写的界面。PDFiD.py和pdf- […]

Pingback by [PDF恶意文档分析-PDFScope- FreebuF.COM](http://www.freebuf.com/tools/3283.html) — Tuesday 5 June 2012 @ [2:19](https://blog.didierstevens.com/programs/pdf-tools/#comment-53911)

-

[…] PDF Tools “Didier Stevens” – Didier tiene una gran colección de herramientas locales. […]

Pingback by [Securización de lectores PDF « marian1105](http://marian1105.wordpress.com/2012/06/06/securizacion-de-lectores-pdf/) — Wednesday 6 June 2012 @ [16:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-53981)

-

I receive this message when trying to use pdf-parser, can you help?

C:\Program Files\IronPython 2.6>ipy pdf-parser.py -help
Traceback (most recent call last):
File “pdf-parser.py”, line 50, in
ImportError: No module named zlib
C:\Program Files\IronPython 2.6>

same with any file i try to scan

Comment by dannybpcr — Tuesday 3 July 2012 @ [22:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-54923)

-

@dannybpcr My tools are not developed for IronPython. You must use Python.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 3 July 2012 @ [22:51](https://blog.didierstevens.com/programs/pdf-tools/#comment-54924)

-

thnx for sharing

Comment by [raef](http://www.najah.edu/uchrd) — Tuesday 28 August 2012 @ [11:43](https://blog.didierstevens.com/programs/pdf-tools/#comment-56749)

-

[…] can be learned from this data. Didier has published a pdf parsing tool written in python called pdf-parser.py, which looks to be very promising in analyzing pdf files. I just started playing with the tool […]

Pingback by [sudosecure.net » Blog Archive » Analyzing PDF files and Shellcode](http://www.sudosecure.net/archives/313) — Thursday 18 October 2012 @ [16:38](https://blog.didierstevens.com/programs/pdf-tools/#comment-59524)

-

[…] PDF ise bu defa amaç, zararlı kod içerebilecek Javascript kodunu tespit etmektir. Bunun için de pdf-parser.py, peepdf ve Origami gibi araçlardan […]

Pingback by [Zararlı PDF Analizi | Hack 4 Career](http://www.mertsarica.com/zararli-pdf-analizi/) — Thursday 6 December 2012 @ [20:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-63220)

-

A couple of observations about pdf-parser.py.

First, it is very slow on files which have large images embedded in them. I think this comes from the tokenizer code which contains lines such as
self.token = self.token + chr(self.byte)
There is a good analysis of the speed of this compared to other methods at http://www.skymind.com/~ocrow/python_string/. When I changed it so that self.token is a StringIO buffer, I got an huge increase in speed. In particular, one file which has not completed parsing after 30 minutes was now processed in a few seconds.

Secondly, I noticed that Decompress was not called on some stream data. This turned out to be because the stream was ASCII85 encoded and ended like this:
T.5*QV#Ts4I~>endstream
Note that there is no end of line character between the ASCII85 end marker (~>) and the endstream keyword. According to the PDF 1.7 specification, this is not approved of but is allowed:
“It is recommended that there be an end-of-line marker after the data and before endstream; this marker is not included in the stream length.” on page 61 of http://wwwimages.adobe.com/www.adobe.com/content/dam/Adobe/en/devnet/pdf/pdfs/pdf_reference_1-7.pdf. The file in question was generated by reportlab.

My first thought for fixing this was to change
if self.content[i][0] == CHAR_REGULAR and self.content[i][1] == ‘endstream’:
to
if self.content[i][0] == CHAR_REGULAR and self.content[i][1].endswith(‘endstream’)
and then trimming the keyword off the data. However, this does not work, as self.content[i][1] actually ends with a newline character, and self.content[i][0] has the value CHAR_DELIMITER. Something like
if self.content[i][1].strip().endswith(‘endstream’):
end = self.content[i][1].rindex(‘endstream’)
data += self.content[i][1][:end]
might do the job, though it’s ugly. The ideal solution would really be to use the length attribute from the dictionary, though this seems to be a bigger change.

Otherwise, the code looks great, and is really helping me with a project I am working on.

Comment by David Elworthy — Tuesday 1 January 2013 @ [19:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-65118)

-

@David Interesting, thanks for the observations. Will do some profiling.
What I’m curious about: how come you are parsing PDF files that require so much time? Are these malicious?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 1 January 2013 @ [22:54](https://blog.didierstevens.com/programs/pdf-tools/#comment-65129)

-

My end goal is writing a scanner application which will build archive versions of documents from photographs of pages. I’m a long way off this, and so was using a PDF build from some photos of landscapes, but even so the files were only a few megabytes. Eventually I want to generate my own PDFs, as I don’t much like reportlab and pyPDF, but for now reportlab is what I am using. I was looking at your code as a way of understanding the file format. As a shorter term project, I also want to write something which willtake files with 600 dpi images from a flatbed scanner and either downsample them to a lower dpi or increase the JPEG compression, as I sometimes find the 600 dpi scans (which are meant to be archive quality) are a bit large for emailing when there’s a lot of pages. Of course there are plenty of applications which allow you to manipulate PDFs interactively, but I’m a command line kind of guy, so a python script would be ideal.

Comment by David Elworthy — Tuesday 1 January 2013 @ [23:10](https://blog.didierstevens.com/programs/pdf-tools/#comment-65131)

-

Hi there, thanks for the amazing script, it makes the life easier. I have a PDF with a postscript-type image embedded (an eps actually). I am reading the PDF reference and I think for that kind of image, it will be stored as postscript command in the stream so I am wondering if it is possible to extract the postscript in the stream directly. Thanks

Comment by Anonymous — Sunday 6 January 2013 @ [6:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-65443)

-

@Anonymous Yes, you can extract it.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 8 January 2013 @ [9:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-65623)

-

@David pdf-parser is designed to parse malicious PDF documents, so I assume that the PDF document contains wrong information. For example, that’s why I don’t rely on the /Length value to parse a stream.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 8 January 2013 @ [9:17](https://blog.didierstevens.com/programs/pdf-tools/#comment-65624)

-

[…] A month before my PDF training at HITB, it’s time to release new versions of my pdf tools. […]

Pingback by [Update: PDFiD Version 0.1.0 | Didier Stevens](https://blog.didierstevens.com/2013/03/07/update-pdfid-version-0-1-0/) — Thursday 7 March 2013 @ [5:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-69545)

-

Hi Didier
Great work you are doing with the PDF format. One quick question about the browsers supporting pdf documents. Is it a good idea to think of browsers as better pdf readers because they are supposed to have sealed most javascript vulnerablities ?? would love to hear your opinion on that
Thanks
Jiss

Comment by Jiss — Tuesday 12 March 2013 @ [17:37](https://blog.didierstevens.com/programs/pdf-tools/#comment-69950)

-

@Jiss The idea of PDF readers in beowers like Firefox’s pdf.js, is that they are written in a higher language than standard readers (hence not in C), and thus that bugs can’t be exploited like in C.

pdf.js is written in JavaScript. Say you find a bug in pdf.js and that you try to develop an exploit for it. The best you’ll be able to do, is execute arbitrary JavaScript.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 12 March 2013 @ [23:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-69968)

-

Using Reader 10.1.6 on MacOSX 10.7, get the following error when embedding an EXE:
Acrobat EScript Built-in Functions Version 10.0
Acrobat SOAP 10.0

TypeError: Invalid argument type.
Doc.exportDataObject:1:Doc undefined:Open
===> Parameter cName.
TypeError: Invalid argument type.
Doc.exportDataObject:1:Doc undefined:Open
===> Parameter cName.

Comment by [Phil](http://twitter.com/SecPhil) — Wednesday 13 March 2013 @ [15:30](https://blog.didierstevens.com/programs/pdf-tools/#comment-70060)

-

@Phil What options did you use to create this document?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 13 March 2013 @ [21:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-70079)

-

Didier, this was my third attempt. This one used -a -m.

Comment by [Phil](http://twitter.com/SecPhil) — Wednesday 13 March 2013 @ [21:26](https://blog.didierstevens.com/programs/pdf-tools/#comment-70082)

-

@Phil OK, I was sure you used option -a. You have to know that PDF readers like Adobe Reader do not allow you to extract executable files. To determine if a file is executable or not, Adobe Reader looks at the extension. So you can’t extract .exe files (unless you change the extension to something that is not executable, like .txt).
Option -a instructs my tool to add JavaScript to the PDF document to extract the embedded file automatically. But since this is not allowed for an .exe file, the script fails, and that is what you see in the error messages.

FYI because you are doing this on OSX: Python (.py) is allowed as executable file type.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 13 March 2013 @ [21:33](https://blog.didierstevens.com/programs/pdf-tools/#comment-70084)

-

Thanks. Suspected that much. Was able to unpackage Acrobat to determine the list of disallowed extensions. For everyone else, that list is: .ade:3|.adp:3|.app:3|.arc:3|.arj:3|.asp:3|.bas:3|.bat:3|.bz:3|.bz2:3|.cab:3|.chm:3|.class:3|.cmd:3|.com:3|.command:3|.cpl:3|.crt:3|.csh:3|.desktop:3|.dll:3|.dylib:3|.exe:3|.fxp:3|.gz:3|.hex:3|.hlp:3|.hqx:3|.hta:3|.inf:3|.ini:3|.ins:3|.isp:3|.its:3|.jar:3|.job:3|.js:3|.jse:3|.ksh:3|.lnk:3|.lzh:3|.mad:3|.maf:3|.mag:3|.mam:3|.maq:3|.mar:3|.mas:3|.mat:3|.mau:3|.mav:3|.maw:3|.mda:3|.mdb:3|.mde:3|.mdt:3|.mdw:3|.mdz:3|.msc:3|.msi:3|.msp:3|.mst:3|.o:3|.ocx:3|.out:3|.ops:3|.pcd:3|.pi:3|.pif:3|.pkg:3|.prf:3|.prg:3|.pst:3|.rar:3|.reg:3|.scf:3|.scr:3|.sct:3|.sea:3|.sh:3|.shb:3|.shs:3|.sit:3|.tar:3|.taz:3|.tgz:3|.tmp:3|.url:3|.vb:3|.vbe:3|.vbs:3|.vsmacros:3|.vss:3|.vst:3|.vsw:3|.webloc:3|.ws:3|.wsc:3|.wsf:3|.wsh:3|.z:3|.zip:3|.zlo:3|.zoo:3|.term:3|.tool:3|.pdf:2|.fdf:2

Comment by [Phil](http://twitter.com/SecPhil) — Wednesday 13 March 2013 @ [21:48](https://blog.didierstevens.com/programs/pdf-tools/#comment-70085)

-

@Phil IIRC, the number following the extension indicates what is allowed or not. Look at the end of the list: .pdf and .fdf have number 2.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 13 March 2013 @ [21:53](https://blog.didierstevens.com/programs/pdf-tools/#comment-70087)

-

Hello. Sir
When we look for tags like /JS, I think they should be seen when an object starts.

Consider this file: [http://www.mcafee.com/in/resources/white…/wp-new-era-of-botnets.pdf](http://www.mcafee.com/in/resources/white.../wp-new-era-of-botnets.pdf)
pdfid.py shows /JS in this file but this /js is actually written as part of text.

can you please help me on this. is this really a javascript into this document or not. I try this pdf file with pdfextract and this also could not extract any javascript.

please help
i will be very grateful to you on this.

Comment by himanshu — Wednesday 20 March 2013 @ [9:52](https://blog.didierstevens.com/programs/pdf-tools/#comment-70621)

-

@Himanshu

analyze the file with pdf-parser and search for /JS.
If pdf-parser can’t find it, then it is not a name in a dictionary but most likely a string in a stream.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 20 March 2013 @ [10:15](https://blog.didierstevens.com/programs/pdf-tools/#comment-70628)

-

hello mr stevens.

i closely follow your post and i am facing a problem when i m trying to run .exe embedded in a pdf through make-pdf-embedded.py . it is not running on the windows 7 machine. also it is not supported by adobe x and above. is there a way out for this prob

yours sao zumin

Comment by sao zumin — Friday 22 March 2013 @ [5:50](https://blog.didierstevens.com/programs/pdf-tools/#comment-70861)

-

@sao That doesn’t work. Please take a look at comments 105 and 106.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 22 March 2013 @ [7:45](https://blog.didierstevens.com/programs/pdf-tools/#comment-70870)

-

Hi Didier, Great info and tools.
I noticed that extension .py was missing from the list of disallowed extensions. Is it possible to use python to assist in launching an executable?

Thanks
Matahachi

Comment by Matahachi — Friday 12 April 2013 @ [8:40](https://blog.didierstevens.com/programs/pdf-tools/#comment-72948)

-

@Matahachi Yes, Python is allowed, I used it as an example in my training class.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 12 April 2013 @ [19:56](https://blog.didierstevens.com/programs/pdf-tools/#comment-73008)

-

[…] Didier Stevens PDF tool kit to the rescue! Didier has created some great forensic tools for working with PDF […]

Pingback by [BSides 2013 – Challenge 4 – CSCUK Challenge | TabChalk - Beware the devil inside!](http://tabchalk.com/blog/?p=268) — Saturday 27 April 2013 @ [12:34](https://blog.didierstevens.com/programs/pdf-tools/#comment-74873)

-

[…] the PDF using Didier Stevens’ PDFiD tool shows that the two PDFs are very similar. They may not be identical, but the similarities […]

Pingback by [Malicious PDFs On The Rise | Security Intelligence Blog | Trend Micro](http://blog.trendmicro.com/trendlabs-security-intelligence/malicious-pdfs-on-the-rise/) — Tuesday 30 April 2013 @ [9:54](https://blog.didierstevens.com/programs/pdf-tools/#comment-75336)

-

[…] Ok, we have our PDF now and we are ready to begin our analysis. We need a tool for inspection, in our case we’ll use Didier Steven’s pdf-parser.py […]

Pingback by [Analysis of CVE-2010-0188 PDF from RedKit ExploitKit](http://quequero.org/2013/03/analysis-of-cve-2010-0188-from-redkit/) — Friday 10 May 2013 @ [20:11](https://blog.didierstevens.com/programs/pdf-tools/#comment-76983)

-

[…] PDFiD will give you false positives for /JS and /AA. This happens with files of a couple of MBs or […]

Pingback by [PDFiD: False Positives | Didier Stevens](https://blog.didierstevens.com/2013/06/10/pdfid-false-positives/) — Monday 10 June 2013 @ [8:49](https://blog.didierstevens.com/programs/pdf-tools/#comment-85070)

-

Can I Decompress file in Mac-OS(Macintosh)??

Comment by Sandeep Vasoya — Monday 8 July 2013 @ [7:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-90562)

-

@Sandeep My Python programs work on OSX too.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 8 July 2013 @ [18:07](https://blog.didierstevens.com/programs/pdf-tools/#comment-90626)

-

Thanks Didier…

Comment by Sandeep — Tuesday 9 July 2013 @ [4:59](https://blog.didierstevens.com/programs/pdf-tools/#comment-90696)

-

[…] PDF Parser […]

Pingback by [Tools » Damul's Blog](http://www.webtown.kr/archives/169) — Thursday 29 August 2013 @ [2:11](https://blog.didierstevens.com/programs/pdf-tools/#comment-98055)

-

Hi,
I have a PDF that only has about 500 pages, but your pdfid shows /Page to be around 1100. Any ideas what’s going on?

PS. Thanks again for keeping these tools up to date. I’ve been following your work since early versions.

Comment by CurlyBird — Wednesday 4 September 2013 @ [16:14](https://blog.didierstevens.com/programs/pdf-tools/#comment-99852)

-

@CurlyBird This counter counts the number of instances of the /Page name found in the document. There could be several reasons why pdfid finds more /Page instances than there are pages.
Without having the document, it’s hard to tell. But one reason can be that your document is made with incremental updates (e.g. that the document contains
previous versions of the PDF document, and thus that pdfid counts these too).

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 4 September 2013 @ [17:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-99871)

-

@Didier: Great! Very quick answer. That would make sense. I noticed some discrepancies in the document regarding file size vs content and suspected missing information, which is why I decided to inspect it closer in the first place. Is there any way to retrieve this previous version? (Or revert these incremental updates?)

Comment by CurlyBird — Wednesday 4 September 2013 @ [19:44](https://blog.didierstevens.com/programs/pdf-tools/#comment-99901)

-

@CurlyBird Yes, it’s something I teach in my training. Search for %%EOF not at the end of the file.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 4 September 2013 @ [22:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-99937)

-

@Didier: Great! Sure enough there were some missing stuff in there, but there were
6 counts of “EOF%%” but I can only tell any obvious difference between the
1st and 2nd versions. The later ones “look” the same. I wish there were
some kind of more visual PDF diffing utility…

BTW. I got the offsets by:
strings -n 4 -t x -e s weird.pdf |grep -i -E “%%EOF”

Then extracted the versions with:
dd if=weird.pdf of=weird_a.pdf bs= count=1

Thanks again.

PS. Sorry, I can’t attend your training as I live very far away.

Comment by CurlyBird — Thursday 5 September 2013 @ [12:47](https://blog.didierstevens.com/programs/pdf-tools/#comment-100036)

-

Web Magic above: “bs=” should be “bs=offset+6”

Comment by CurlyBird — Thursday 5 September 2013 @ [12:53](https://blog.didierstevens.com/programs/pdf-tools/#comment-100038)

-

Hello Didier
.I don’t know anything in Python and in pdf security…
Today i downloaded a pdf file. When i opened it, “cmd” appeared and many lines displayed quickly inside…
Soi have to check this strange pdf file…
So i installed Python 3.3.2 (my pc’s OS is Windows 7) but i have a problem when i test:

C:\Python33>python.exe pdf-parser.py
File “pdf-parser.py”, line 486
except zlib.error, e:
^
SyntaxError: invalid syntax

Thanks for your help. It’s very importan and urgent for me to check this file and to know if my pc has a problem.
Mathias

Comment by Mathias Rollet — Saturday 7 September 2013 @ [22:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-100457)

-

@Mathias Try with Python 2.7 But if you don’t know anything about PDF, my tools will not help you. In your case it’s better to upload the PDF file to VirusTotal and see if it is detected by AV.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 8 September 2013 @ [8:45](https://blog.didierstevens.com/programs/pdf-tools/#comment-100511)

-

[…] utilizada: https://blog.didierstevens.com/programs/pdf-tools (script […]

Pingback by [Uso de PDF para Exploração de Vulnerabilidades | Yuri Diogenes](http://yuridiogenes.wordpress.com/2013/10/09/uso-de-pdf-para-explorao-de-vulnerabilidades/) — Wednesday 9 October 2013 @ [18:20](https://blog.didierstevens.com/programs/pdf-tools/#comment-104890)

-

Hi Didier,

I find the PFDiD.py interesting but I am having a difficult time trying to get it to work. I have a VM with Windows XP installed and python 2.6.6 installed and appears to be working fine. If I enter PDFiD MyFile.pdf I get a syntax error. although your example shows PDFiD 0.0.2 test.pdf I just don’t understand what significance the number has and what the correct syntax is to get my example to work. Another example I’m having difficulties with is pdf-parser.py that should work by implementing pdf-parser.py MyFile.pdf –search=javascript. Any help would be greatly appreciated

Thanks,

Michael

Comment by Michael — Sunday 26 January 2014 @ [3:36](https://blog.didierstevens.com/programs/pdf-tools/#comment-124026)

-

@Michael Can you report the syntax error?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 26 January 2014 @ [16:58](https://blog.didierstevens.com/programs/pdf-tools/#comment-124101)

-

Didier,

Thanks for getting back to me so soon! Below will be the syntax errors:

Example 1: PDFiD

Python 2.6.6 (r266:84297, Aug 24 2010, 18:46:32) [MSC v.1500 32 bit (Intel)] on
win32
Type “help”, “copyright”, “credits” or “license” for more information.
>>> PDFiD TheFlyv3_EN4Rdr.pdf
File “”, line 1
PDFiD TheFlyv3_EN4Rdr.pdf
^
SyntaxError: invalid syntax
>>>

Example 2: pdf-parser.py

>>> pdf-parser.py TheFlyv3_EN4Rdr.pdf –search=javascript
File “”, line 1
pdf-parser.py TheFlyv3_EN4Rdr.pdf –search=javascript
^
SyntaxError: invalid syntax
>>>

Thanks,

Michael

Comment by Michael — Sunday 26 January 2014 @ [19:43](https://blog.didierstevens.com/programs/pdf-tools/#comment-124123)

-

OK, I see what’s wrong. You start Python and then launch pdfid or pdf-parser.
That’s not how you should do it, you should launch these tools from the command line (cmd.exe).

Start cmd.exe and type pdfid.py test.pdf, where test.pdf is one of your PDFs, and make sure pdfid is in the working folder c:\test:
C:\Test\>pdfid.py test.pdf

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 26 January 2014 @ [22:25](https://blog.didierstevens.com/programs/pdf-tools/#comment-124138)

-

Didier,

I think another problem with this is that I don’t have pdfid install properly where do I get that?

Thanks,

Michael

Comment by Michael — Monday 27 January 2014 @ [3:48](https://blog.didierstevens.com/programs/pdf-tools/#comment-124174)

-

@Michael. There is no install program, it’s just a Python program.
Maybe you’re not familiar with the command-line in Windows. I suggest you save pdfid.py and your pdf in the same directory, and then open a command-line in that directory.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 27 January 2014 @ [19:34](https://blog.didierstevens.com/programs/pdf-tools/#comment-124304)

-

Didier,

Awesome! Thank you for all your support it worked perfectly.

Michael

Comment by Anonymous — Tuesday 28 January 2014 @ [3:32](https://blog.didierstevens.com/programs/pdf-tools/#comment-124365)

-

Quick question – I am trying to use PDFid to detect malicious flash inside a PDF. When I run PDFid it shows zeros for RichMedia, EmbeddedFile, OpenAction, and AA. What output from PDFid should indicate the flash?

A sample PDF with the flash can be found here: http://contagiodump.blogspot.com/2011/04/apr-22-cve-2011-0611-pdf-swf-marshall.html

Thanks.

Comment by Todd — Monday 10 February 2014 @ [0:45](https://blog.didierstevens.com/programs/pdf-tools/#comment-127117)

-

@Todd Can you post the pdfid analysis?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 12 February 2014 @ [23:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-127731)

-

Sure can..

C:\malwaresandbox>pdfid.py “Marshall Plan for the North Africa.pdf”
PDFiD 0.1.2 Marshall Plan for the North Africa.pdf
PDF Header: %PDF-1.7
obj 35
endobj 35
stream 24
endstream 24
xref 1
trailer 1
startxref 5
/Page 3
/Encrypt 0
/ObjStm 8
/JS 0
/JavaScript 0
/AA 0
/OpenAction 0
/AcroForm 0
/JBIG2Decode 0
/RichMedia 0
/Launch 0
/EmbeddedFile 0
/XFA 0
/Colors > 2^24 0

Comment by Todd — Thursday 13 February 2014 @ [0:38](https://blog.didierstevens.com/programs/pdf-tools/#comment-127738)

-

@Todd OK, I see, you have 8 object streams (/ObjStm). These are streams that contain object, that’s where the Flash code is hiding. You need to inflate these streams and pipe that to pdfid. That is something I explain in my PDF Analysis workshop: http://DidierStevensLabs.com

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 13 February 2014 @ [23:20](https://blog.didierstevens.com/programs/pdf-tools/#comment-127946)

-

[…] The shellcode contains the URL that the exploit will contact to download the malicious payload. We can extract it using pdf-parser: […]

Pingback by [Email-borne exploits: the not-so innocuous killers targeting small business | Malwarebytes Unpacked](http://blog.malwarebytes.org/exploits-2/2014/05/email-borne-exploits-the-not-so-innocuous-killers-targeting-small-business/) — Monday 12 May 2014 @ [18:24](https://blog.didierstevens.com/programs/pdf-tools/#comment-164523)

-

I ran make-pdf-javascript.py on my windows platform but nothing happened … C:\Users\Suleiman JK\Desktop>Python make-pdf-javascript.py “C:\Users\Suleiman JK\Desktop\suleiman.pdf” … I suppose a pdf file with a name “suleiman” to be created on desktop but nothing happened. do I ran the tool correctly ?//

Comment by Suleiman Kheetan — Tuesday 1 July 2014 @ [21:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-185397)

-

Yes. But make sure you use Python 2 for the make tools.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 3 July 2014 @ [15:11](https://blog.didierstevens.com/programs/pdf-tools/#comment-185969)

-

do you mean to use any version of python 2 till 2.77 ?

Comment by Suleiman Kheetan — Friday 4 July 2014 @ [16:07](https://blog.didierstevens.com/programs/pdf-tools/#comment-186361)

-

Yes, Python 2 is 2.x.x. I recommend the latest.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 4 July 2014 @ [16:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-186368)

-

I knew where was my mistake that I’m using windows 8. I tried it in windows xp and worked fine. but how could we make it works on windows 8 ?

Comment by Suleiman Kheetan — Friday 4 July 2014 @ [22:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-186455)

-

[…] PDF Tools de Didier Stevens. PDFStreamDumper – Esta es una herramienta gratuita para el análisis PDFs maliciosos. SWF Mastah – Programa en Python que extrae stream SWF de ficheros PDF. […]

Pingback by [Listado de Herramientas Forenses | ROOTAGAINSTTHEMACHINE](http://rootagainstthemachine.com/?p=495) — Monday 7 July 2014 @ [12:14](https://blog.didierstevens.com/programs/pdf-tools/#comment-187325)

-

[…] PDFiD […]

Pingback by [بررسی پرونده‌های آلوده - ایمن وب](http://securitexperts.com/fa/papers/%d8%a8%d8%b1%d8%b1%d8%b3%db%8c-%d9%be%d8%b1%d9%88%d9%86%d8%af%d9%87%e2%80%8c%d9%87%d8%a7%db%8c-%d8%a2%d9%84%d9%88%d8%af%d9%87) — Wednesday 9 July 2014 @ [18:49](https://blog.didierstevens.com/programs/pdf-tools/#comment-188188)

-

[…] embedf Create a blank PDF document with an embedded file. This is for research purposes to show how files can be embedded in PDFs. This command imports Didier Stevens Make-pdf-embedded.py script as a module. (https://blog.didierstevens.com/programs/pdf-tools/) […]

Pingback by [ParanoiDF - PDF Analysis Tool](http://hashtec.org/paranoidf-pdf-analysis-tool/) — Sunday 17 August 2014 @ [5:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-200517)

-

Didier, I need some help. I have recently upgraded to Adobe XI and some of my previously OK adobe pdf files now cannot be read. Adobe indicate that they have increased “security” and enforced some compliance in their document headers. I have gone back though my useful bits of software that may be able to help me and come across your 010 editor. I have compared a couple of files that are ok and still working but I am not able to spot any significant differences (other than length, width height etc.).

Have you com across this problem before ? Any ideas as to how to resolve the problem. BTW the file is quite large 25MB and my programming skills are now quite poor – used to be an assembler / c programmer about 20 years ago!!!

Comment by Paul Kirikal — Sunday 24 August 2014 @ [15:19](https://blog.didierstevens.com/programs/pdf-tools/#comment-204651)

-

@Paul Can you read the files with Sumatra PDF?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 24 August 2014 @ [20:36](https://blog.didierstevens.com/programs/pdf-tools/#comment-204730)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parserand make-pdf and mPDF) […]

Pingback by [Python：渗透测试开源项目 – Arschloch](http://b.nop.pw/2014/09/1101.html) — Friday 5 September 2014 @ [19:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-210439)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and mPDF) […]

Pingback by [Lufsec – Python tools for penetration testers](http://www.lufsec.com/?p=2434) — Saturday 6 September 2014 @ [4:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-210574)

-

[…] technique is using Didier Stevens suite of tools to analyze the content of the PDF and look for suspicious elements. One of those tools is Pdfid […]

Pingback by [Malicious Documents – PDF Analysis in 5 steps | Count Upon Security](http://countuponsecurity.com/2014/09/22/malicious-documents-pdf-analysis-in-5-steps/) — Monday 22 September 2014 @ [10:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-217061)

-

[…] this PDF was another red herring considering the image in the PDF, but I persisted and fired up pdf-parser and got into the internals of the PDF file (Figure 8). Admittedly I am still learning PDF […]

Pingback by [CSAW 2014 walkthrough – Fluffy No More | Overflow Security](http://overflowsecurity.com/csaw-2014-walkthrough-fluffy-no-more/) — Thursday 25 September 2014 @ [1:24](https://blog.didierstevens.com/programs/pdf-tools/#comment-219287)

-

[…] PDF ise bu defa amaç, zararlı kod içerebilecek Javascript kodunu tespit etmektir. Bunun için de pdf-parser.py, peepdf ve Origami gibi araçlardan […]

Pingback by [Zararlı PDF Dosyalarının Adım Adım Analizi | caglar's space](http://caglararli.wordpress.com/2014/09/30/zararli-pdf-dosyalarinin-adim-adim-analizi/) — Tuesday 30 September 2014 @ [13:45](https://blog.didierstevens.com/programs/pdf-tools/#comment-224007)

-

Dear mr. steven
I’m trying to use pdfid in windows 8 with python 2.7.8 when I ran it by cmd.exe I have this error :
C:\Users\Test\Desktop>pdfid.py MultiplePages.pdf
Traceback (most recent call last):
File “C:\Users\Test\Desktop\pdfid.py”, line 25, in
import urllib.request
File “C:\Python27\lib\urllib.py”, line 33, in
from urlparse import urljoin as basejoin
File “C:\Python27\lib\urlparse.py”, line 119, in
from collections import namedtuple
File “C:\Python27\lib\collections.py”, line 12, in
import heapq as _heapq
ImportError: No module named heapq

could you help me please

Comment by Suleiman Khitan — Friday 3 October 2014 @ [21:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-226607)

-

regarding to the 232 question i have this error

C:\Users\Test\Desktop>pdfid.py MultiplePages.pdf
Traceback (most recent call last):
File “C:\Users\Test\Desktop\pdfid.py”, line 20, in
import zipfile
File “C:\Python27\lib\zipfile.py”, line 4, in
import struct, os, time, sys, shutil
File “C:\Python27\lib\shutil.py”, line 12, in
import collections
File “C:\Python27\lib\collections.py”, line 12, in
import heapq as _heapq
ImportError: No module named heapq

Comment by Suleiman Khitan — Friday 3 October 2014 @ [21:26](https://blog.didierstevens.com/programs/pdf-tools/#comment-226610)

-

@Suleiman I can not reproduce your problem, it works fine with 2.7.8. Check pdfid.py, because the reported line numbers (20 and 25) are not normal.
The first 50 lines are pdfid are comment, so they can not create an import error.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 4 October 2014 @ [8:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-226866)

-

[…] DE MALWARE PDF Tools de Didier Stevens. PDFStreamDumper – Esta es una herramienta gratuita para el análisis PDFs […]

Pingback by [HERRAMIENTAS USADAS EN LA COMPUTACIÓN FORENSE | RECOLECCIÓN DE EVIDENCIA DIGITAL](http://joselodev.wordpress.com/2014/11/18/herramientas-usadas-en-la-computacion-forense/) — Wednesday 19 November 2014 @ [14:07](https://blog.didierstevens.com/programs/pdf-tools/#comment-246606)

-

Hi Planning to build a PDF tester application, my job involved testing Pdf for formats issues and check values in each cell of the report for format and length.
Is there a tool to check all the objects in an present it in tree view.

I have plans to build a tool which has the meta data on how the object should look like in the resultant PDF. The tool will evaluate and check with the meta data for discrepancies.

Comment by udhay — Thursday 20 November 2014 @ [11:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-246905)

-

@udhay I know tools like PDF Dissector and PDF Structazer do this, but these tools are discontinued. You could take a look at the PDF template I developed for 010 Editor.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 22 November 2014 @ [17:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-247681)

-

[…] nombrar algunas herramientas específicas comenzaremos por pdfid, una aplicación sencilla para explorar “de un vistazo” la estructura del documento (cabecera, […]

Pingback by [Herramientas para el análisis de documentos PDF maliciosos « DabacodLAB](http://dabacodlab.wordpress.com/2014/11/28/herramientas-para-el-analisis-de-documentos-pdf-maliciosos/) — Friday 28 November 2014 @ [14:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-249524)

-

In your pdf-parser.py you have:

def CharacterClass(byte):
if byte == 0 or byte == 9 or byte == 10 or byte == 12 or byte == 13 or byte == 32:
return CHAR_WHITESPACE
…
Might be faster if you wrote:

def CharacterClass(byte):
if byte in b”\x00\x09\x0A\x0C\x0D\x20″:
return CHAR_WHITESPACE
…

Or

def CharacterClass(byte):
if byte in frozenset((0, 9, 10, 12, 13, 32)):
return CHAR_WHITESPACE

Comment by [Mark Summerfield](http://www.qtrac.eu) — Monday 5 January 2015 @ [11:58](https://blog.didierstevens.com/programs/pdf-tools/#comment-260672)

-

I’m sure your last example is not faster. Each time you want to test a byte, you create a frozenset. For performance, you should create this frozenset only once (for example global variable).

I believe more speed can be gained by not using a function., e.g. do the test inline.
But then you loose readability and maintainability. That’s why performance is less important as design requirement to me.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 5 January 2015 @ [21:05](https://blog.didierstevens.com/programs/pdf-tools/#comment-260762)

-

[…] In that case one of the best tool available is oledump.py from Didier Stevens (also known for his PDF tools…but we will talk about that in an upcoming […]

Pingback by [Word document analysis with oledump.py | SimonGaniere.ch](http://www.simonganiere.ch/2015/01/12/word-document-analysis-with-oledump-py/) — Monday 12 January 2015 @ [13:58](https://blog.didierstevens.com/programs/pdf-tools/#comment-262341)

-

[…] PDFs using “pdfcop”, “pdf-parser”, “pdfid”, “pdfdecompress” and […]

Pingback by [REMnux Usage Tips for Malware Analysis on Linux](http://worker.zeltser.com/remnux-usage-tips-for-malware-analysis-on-linux/) — Thursday 15 January 2015 @ [3:09](https://blog.didierstevens.com/programs/pdf-tools/#comment-262875)

-

[…] The course now teaches steps for analyzing malicious Adobe PDF documents, making use of utilities such as Origami and Didier Stevens’ PDF Tools. […]

Pingback by [Expansion of the SANS Reverse-Engineering Malware (REM) Course FOR610 in 2010](http://worker.zeltser.com/for610-course-expansion-to-5-days/) — Tuesday 27 January 2015 @ [18:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-265449)

-

[…] PDFiD identifies PDFs that contain strings associated with scripts and actions. […]

Pingback by [Analyzing Malicious Documents Cheat Sheet](http://worker.zeltser.com/analyzing-malicious-documents/) — Tuesday 27 January 2015 @ [18:11](https://blog.didierstevens.com/programs/pdf-tools/#comment-265451)

-

[…] PDFs using “pdfcop”, “pdf-parser”, “pdfid”, “pdfdecompress” and […]

Pingback by [REMnux Usage Tips for Malware Analysis on Linux](http://worker.zeltser.com/remnux-malware-analysis-tips/) — Tuesday 27 January 2015 @ [19:56](https://blog.didierstevens.com/programs/pdf-tools/#comment-265468)

-

[…] PDFiD identifies PDFs that contain strings associated with scripts and actions. […]

Pingback by [Analyzing Malicious Documents Cheat Sheet | iTeam Developers](http://www.iteamdevelopers.org/analyzing-malicious-documents-cheat-sheet/) — Monday 2 February 2015 @ [7:33](https://blog.didierstevens.com/programs/pdf-tools/#comment-266707)

-

May anyone help me get all of objects in a PDF file extracted or viewed?

Comment by udhay — Wednesday 18 March 2015 @ [9:12](https://blog.didierstevens.com/programs/pdf-tools/#comment-273485)

-

@udhay just run pdf-parser

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 18 March 2015 @ [9:14](https://blog.didierstevens.com/programs/pdf-tools/#comment-273486)

-

Got an error:
C:\Users\root\Downloads>python pdf-parser.py -w pagrindinis_brezinys.pdf
PDF Comment %PDF-1.5

Traceback (most recent call last):
File “pdf-parser.py”, line 1201, in
Main()
File “pdf-parser.py”, line 1094, in Main
print(‘PDF Comment %s’ % FormatOutput(object.comment, options.raw))
File “C:\Python33\lib\encodings\cp775.py”, line 19, in encode
return codecs.charmap_encode(input,self.errors,encoding_map)[0]
UnicodeEncodeError: ‘charmap’ codec can’t encode characters in position 13-15: c
haracter maps to

Comment by Donatas — Thursday 19 March 2015 @ [6:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-273721)

-

@Donatas Can you share the PDF?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 19 March 2015 @ [8:30](https://blog.didierstevens.com/programs/pdf-tools/#comment-273745)

-

[…] 1℃ 0评论 While attack vectors based on Malicious PDF are a well known topic (SANS, Didier’s tools), understanding how those vectors are spread up nowadays is an interesting “research” […]

Pingback by [PDF Versions Malicious Content Distribution – ThinfoSec.COM关注通化信息安全](http://www.thinfosec.com/?p=1056) — Tuesday 24 March 2015 @ [7:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-275145)

-

[…] For about half a year now, I’ve been adding YARA support to several of my analysis tools. Like pdf-parser. […]

Pingback by [pdf-parser And YARA | Didier Stevens](https://blog.didierstevens.com/2015/03/31/pdf-parser-and-yara/) — Tuesday 31 March 2015 @ [21:13](https://blog.didierstevens.com/programs/pdf-tools/#comment-277183)

-

[…] Stevens' PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parserand make-pdf and […]

Pingback by [Python：渗透测试开源项目【源码值得精读】 - 有Bug](http://www.havebug.com/archives/7454.html) — Saturday 11 April 2015 @ [5:53](https://blog.didierstevens.com/programs/pdf-tools/#comment-279895)

-

[…] updated and references updated/removed. To automate this process as much as possible, I updated my pdf-parser program to generate a Python program that in turn, generates the original […]

Pingback by [pdf-parser: A Method To Manipulate PDFs Part 1 | Didier Stevens](https://blog.didierstevens.com/2015/04/16/pdf-parser-a-method-to-manipulate-pdfs-part-1/) — Thursday 16 April 2015 @ [0:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-281098)

-

Hi!

Any chance to support utf-8 in files names? They are attached, but displayed with gibberish in Adobe Reader.

Thanks.

Comment by Anonymous — Friday 17 April 2015 @ [19:05](https://blog.didierstevens.com/programs/pdf-tools/#comment-281577)

-

@Anonymous Can you provide more details? What did you do exactly and what is the problem?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 17 April 2015 @ [19:07](https://blog.didierstevens.com/programs/pdf-tools/#comment-281580)

-

——-@Anonymous Can you provide more details?

It’s me, thanks for quick answer )

Cent OS 7 64, Python 2.7.5

I have file with non-english characters in name and embed it with make-make-pdf-embedded.
Try following (it’s Cyrillic letters)
————————————————————–
echo > йцукенг.txt
make-pdf-embedded.py -b йцукенг.txt embed.pdf
————————————————————-

Open embed.pdf and see what happen to embedded name.

Comment by vovodroid — Saturday 18 April 2015 @ [5:19](https://blog.didierstevens.com/programs/pdf-tools/#comment-281693)

-

@vovodroid I tried to reproduce your problem, but I’m not making progress. I can create a file called йцукенг.txt, but I can’t issue a command with that name in cmd.exe. I changed the codepage to Cyrillic (855 if I remember correctly), but that did not help.
I see that you used CentOS. I’ll try that when I have time to install it in a VM.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 18 April 2015 @ [20:05](https://blog.didierstevens.com/programs/pdf-tools/#comment-281869)

-

Hi!
——-I can’t issue a command with that name in cmd.exe
If you use Windows you can just open notepad.exe and paste йцукенг.txt to Save dialog.

——I see that you used CentOS
I guess problem persists in any Linux distribution. It happens in Windows as well, but name is corrupted in other way, due to different file system encoding (UTF16 in Windows and UTF8 in Linux).

Right name: йцукенг.txt
LInux: –¹—Ü—É–º–μ–½–³.txt
Windows: ÈˆÛÍÂÌ„.txt

You see, in Linux name is twice in Length because of utf8.

Thanks.

Comment by vovodroid — Sunday 19 April 2015 @ [3:42](https://blog.didierstevens.com/programs/pdf-tools/#comment-281964)

-

[…] some searching, I found Didier’s Steven’s work, realizing I should have looked there first. Didier has PDFid.py for summary analysis and mPDF.py […]

Pingback by [PowerShell | computer security and system designs](http://blog.bluenotch.com/2015/04/24/powershell/) — Friday 24 April 2015 @ [0:22](https://blog.didierstevens.com/programs/pdf-tools/#comment-283246)

-

[…] and decompressing a stream (for example containing a JavaScript script) is easy with pdf-parser. You select the object that contains the stream (example object 5: -o 5) and you “filter” the […]

Pingback by [pdf-parser: A Method To Manipulate PDFs Part 2 | Didier Stevens](https://blog.didierstevens.com/2015/04/29/pdf-parser-a-method-to-manipulate-pdfs-part-2/) — Wednesday 29 April 2015 @ [0:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-284412)

-

Thanks for the great tool.
I recently tested a 38 pages PDF that after all is not malicious and has been created with PDF-XChange 4.0.194.0
pdf-parser quits with an error message:
obj 214 0
Type: /Annot
Referencing:
Traceback (most recent call last):
File “C:\workdir\PY\PDF\pdf-parser.py”, line 1359, in
Main()
File “C:\workdir\PY\PDF\pdf-parser.py”, line 1307, in Main
PrintObject(object, options)
File “C:\workdir\PY\PDF\pdf-parser.py”, line 999, in PrintObject
PrintOutputObject(object, options)
File “C:\workdir\PY\PDF\pdf-parser.py”, line 773, in PrintOutputObject
oPDFParseDictionary = cPDFParseDictionary(object.content, options.nocanonicalizedoutput)
File “C:\workdir\PY\PDF\pdf-parser.py”, line 635, in __init__
self.parsed = self.ParseDictionary(dataTrimmed)[0]
TypeError: ‘NoneType’ object has no attribute ‘__getitem__’

pdfid states:
PDF Header: %PDF-1.4
obj 216
endobj 216
stream 114
endstream 114
xref 1
trailer 1
startxref 1
/Page 38
/Encrypt 0
/ObjStm 0
/JS 0
/JavaScript 0
/AA 0
/OpenAction 0
/AcroForm 0
/JBIG2Decode 0
/RichMedia 0
/Launch 0
/EmbeddedFile 0
/XFA 0
/Colors > 2^24 0
Thanks

Comment by Michael O — Wednesday 27 May 2015 @ [6:44](https://blog.didierstevens.com/programs/pdf-tools/#comment-291214)

-

Can you share this document so that I can check the error?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 27 May 2015 @ [19:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-291328)

-

Thanks for your reply. Sorry, no I can’t. But I’ll try to find the problem or generate a PDF that produces the same error.

Comment by Michael O — Thursday 28 May 2015 @ [7:12](https://blog.didierstevens.com/programs/pdf-tools/#comment-291421)

-

[…] PDF tools — поиск и выявление подозрительных объектов в PDF документах, анализ элементов PDF. […]

Pingback by [Хакерский дистрибутив на базе Windows. - Cryptoworld](https://cryptoworld.su/%d1%85%d0%b0%d0%ba%d0%b5%d1%80%d1%81%d0%ba%d0%b8%d0%b9-%d0%b4%d0%b8%d1%81%d1%82%d1%80%d0%b8%d0%b1%d1%83%d1%82%d0%b8%d0%b2-%d0%bd%d0%b0-%d0%b1%d0%b0%d0%b7%d0%b5-windows/) — Thursday 25 June 2015 @ [15:48](https://blog.didierstevens.com/programs/pdf-tools/#comment-297765)

-

[…] this new version of pdf-parser, option -H will now also calculate the MD5 hashes of the unfiltered and filtered stream of selected […]

Pingback by [Update: pdf-parser Version 0.6.4 | Didier Stevens](https://blog.didierstevens.com/2015/08/13/update-pdf-parser-0-6-4/) — Thursday 13 August 2015 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-308517)

-

[…] AnalyzePDF, Pdfobjflow, pdfid, pdf-parser, peepdf, Origami, PDF X-RAY Lite, PDFtk, […]

Pingback by [REMnux: Distribución de Linux especializada en en el análisis de malware | Skydeep](https://pipefg.wordpress.com/2015/08/20/remnux-distribucion-de-linux-especializada-en-en-el-analisis-de-malware/) — Thursday 20 August 2015 @ [1:49](https://blog.didierstevens.com/programs/pdf-tools/#comment-309748)

-

I’m using pdf-parser.py parsing a pdf-file and it works great, but in some cases I get the error message “FlateDecode decompress failed, unexpected compression method: fd. zlib.error Error -3 while decompressing data: incorrect header check”. Sometimes instead of “fd.” it says “0e.”.

As an example: http://pastebin.com/uNpMMDBP

And I can’t really understand how to get around this. Can you help me? Thanks!

Comment by miggedy — Thursday 24 September 2015 @ [19:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-317872)

-

This happens when the Python compression module does not support thet compression method that was used, or when the compressed data is actually not compressed data, or when it is corrupt.
I recently wrote a SANS Internet Storm Center diary entry showing how to deal with this problem:
https://isc.sans.edu/forums/diary/Handling+Special+PDF+Compression+Methods/19597/

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 24 September 2015 @ [19:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-317874)

-

Thanks for the quick reply!

Comment by miggedy — Thursday 24 September 2015 @ [20:56](https://blog.didierstevens.com/programs/pdf-tools/#comment-317890)

-

Hi friend, thank you so much for share. It is wonderful tool.

Comment by eliasbernier — Friday 9 October 2015 @ [21:15](https://blog.didierstevens.com/programs/pdf-tools/#comment-321873)

-

Hi,
All Fedora users can now install ‘pdfid’ and ‘pdf-parser’ from my Copr repos: fszymanski/pdfid, fszymanski/pdf-parser.

Comment by [Filip](http://None) — Tuesday 3 November 2015 @ [9:47](https://blog.didierstevens.com/programs/pdf-tools/#comment-328774)

-

[…] useful programs were exiftool and peepdf and Didier Steven’s pdf-tools. I also used pdfgrep, but I had to download the latest source, and then compile it with the perl […]

Pingback by [Scanning for confidential information on external web servers | The Grymoire](https://grymoire.wordpress.com/2016/02/06/scanning-for-confidential-information-on-external-web-servers/) — Saturday 6 February 2016 @ [16:51](https://blog.didierstevens.com/programs/pdf-tools/#comment-346865)

-

[…] Herramientas PDF Didier Stevens : analizar, identificar y crear archivos PDF (incluye PDFiD , pdf-parser y maquillaje pdf y MPDF) […]

Pingback by [Herramientas Python para pruebas de penetración | Blog IhackLabs - Hacking](http://blog.ihacklabs.com/herramientas-python-para-pruebas-de-penetracion/) — Wednesday 2 March 2016 @ [10:50](https://blog.didierstevens.com/programs/pdf-tools/#comment-349173)

-

[…] firmware-mod-kit 固件拆包/组包工具forensics pdf-parser PDF文件挖掘工具forensics scrdec […]

Pingback by [CTF工具集合安装脚本操作姿势 | 安全渗透军火库|SHENTOU.ORG](http://shentou.org/hacker/ctf%e5%b7%a5%e5%85%b7%e9%9b%86%e5%90%88%e5%ae%89%e8%a3%85%e8%84%9a%e6%9c%ac%e6%93%8d%e4%bd%9c%e5%a7%bf%e5%8a%bf/) — Saturday 12 March 2016 @ [2:09](https://blog.didierstevens.com/programs/pdf-tools/#comment-349871)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and mPDF) […]

Pingback by [渗透测试之Python工具箱 | 安全小飞侠的窝](http://avfisher.win/archives/458) — Sunday 13 March 2016 @ [16:49](https://blog.didierstevens.com/programs/pdf-tools/#comment-350001)

-

[…] Python 编写的PDF文件分析工具，可以帮助检测恶意的PDF文件 Didier Stevens’ PDF tools: 分析，识别和创建 PDF 文件(包含PDFiD，pdf-parser，make-pdf 和 mPDF) Opaf: 开放 […]

Pingback by [Python渗透测试工具合集 - 征服者785号征服者785号](http://www.vincentsui.com/wenzhang/python%e6%b8%97%e9%80%8f%e6%b5%8b%e8%af%95%e5%b7%a5%e5%85%b7%e5%90%88%e9%9b%86.html) — Sunday 27 March 2016 @ [10:30](https://blog.didierstevens.com/programs/pdf-tools/#comment-350982)

-

[…] Stevens’ PDF tools: analiza, identifica y crea ficheros PDF (incluye PDFiD, pdf-parser, make-pdf y […]

Pingback by [Coleccion de herramientas de hacking hechas en Python – Underc0de Blog](http://blog.underc0de.org/coleccion-de-herramientas-de-hacking-hechas-en-python/) — Thursday 21 April 2016 @ [12:59](https://blog.didierstevens.com/programs/pdf-tools/#comment-352387)

-

[…] Didier Stevens’ PDF tools: analiza, identifica y crea ficheros PDF (incluye PDFiD, pdf-parser, make-pdf y mPDF) […]

Pingback by [Coleccion de herramientas de hacking hechas en Python – Underc0de Blog](http://blog.underc0de.org/2016/04/27/coleccion-de-herramientas-de-hacking-hechas-en-python/) — Wednesday 27 April 2016 @ [12:32](https://blog.didierstevens.com/programs/pdf-tools/#comment-352626)

-

[…] Stevens’ PDF tools: analiza, identifica y crea ficheros PDF (incluye PDFiD, pdf-parser, make-pdf y […]

Pingback by [Coleccion De Herramientas De Hacking Hechas En Python – A Security Breach](http://blackdrake.es/2016/05/07/coleccion-de-herramientas-de-hacking-hechas-en-python/) — Saturday 7 May 2016 @ [9:35](https://blog.didierstevens.com/programs/pdf-tools/#comment-352984)

-

hey, I love the tools I used them quite a bit.
I wonder though if it would be difficult to add a feature to add the java script to an existing PDF

Comment by Anonymous — Thursday 2 June 2016 @ [0:51](https://blog.didierstevens.com/programs/pdf-tools/#comment-353828)

-

This is something I teach in my training.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 5 June 2016 @ [16:25](https://blog.didierstevens.com/programs/pdf-tools/#comment-353978)

-

[…] Stevens’ PDF tools: analiza, identifica y crea ficheros PDF (incluye PDFiD, pdf-parser,make-pdf y […]

Pingback by [Coleccion De Herramientas De Hacking Hechas En Python - TecnologosRD](http://www.tecnologosrd.com/coleccion-herramientas-hacking-hechas-python/) — Friday 8 July 2016 @ [12:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-355317)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and mPDF) […]

Pingback by [Python tools for Penetration Testers​ | Think outside the Box](http://kali-linux.co/2016/07/12/python-tools-for-penetration-testers%e2%80%8b/) — Tuesday 12 July 2016 @ [16:47](https://blog.didierstevens.com/programs/pdf-tools/#comment-355488)

-

[…] PDF Tools – pdfid, pdf-parser, and more from Didier Stevens. […]

Pingback by [Awesome Malware Analysis Lists – vulnerablelife](https://vulnerablelife.wordpress.com/2016/08/04/awesome-malware-analysis-lists/) — Wednesday 3 August 2016 @ [18:26](https://blog.didierstevens.com/programs/pdf-tools/#comment-356416)

-

[…] Didier Stevens’ PDF tools: analiza, identifica y crea ficheros PDF (incluye PDFiD, pdf-parser, make-pdf y mPDF) […]

Pingback by [Computo Forense y Hacking » Coleccion de herramientas de hacking hechas en Python](https://noticias.duriva.com/coleccion-de-herramientas-de-hacking-hechas-en-python/) — Wednesday 28 September 2016 @ [2:38](https://blog.didierstevens.com/programs/pdf-tools/#comment-358444)

-

[…] Didier Stevens’ PDF tools: 分析，识别和创建 PDF 文件(包含PDFiD，pdf-parser，make-pdf 和 mPDF) Opaf: 开放 PDF 分析框架，可以将 PDF 转化为 XML […]

Pingback by [Python渗透测试工具合集-技术客](http://blog.31mo.com/530.html) — Monday 24 October 2016 @ [2:46](https://blog.didierstevens.com/programs/pdf-tools/#comment-359569)

-

[…] Python 编写的PDF文件分析工具，可以帮助检测恶意的PDF文件 Didier Stevens’ PDF tools: 分析，识别和创建 PDF 文件(包含PDFiD，pdf-parser，make-pdf 和 mPDF) Opaf: 开放 […]

Pingback by [Python渗透测试工具合集 | 技术客](http://blog.31mo.com/232.html) — Monday 7 November 2016 @ [10:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-360006)

-

[…] new version of pdf-parser is a bugfix for […]

Pingback by [Update: pdf-parser Version 0.6.6 | Didier Stevens](https://blog.didierstevens.com/2016/11/28/update-pdf-parser-version-0-6-6/) — Monday 28 November 2016 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-360708)

-

[…] Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parserand make-pdf and […]

Pingback by [Python：渗透测试开源项目 | 歪布IT笔记](http://www.luckerworks.com/archives/14431) — Tuesday 29 November 2016 @ [16:45](https://blog.didierstevens.com/programs/pdf-tools/#comment-360744)

-

[…] https://blog.didierstevens.com/programs/pdf-tools/ […]

Pingback by [Introduction to PDF Analysis – RIT Computing Security Blog](https://ritcsec.wordpress.com/2016/12/11/introduction-to-pdf-analysis/) — Monday 12 December 2016 @ [2:08](https://blog.didierstevens.com/programs/pdf-tools/#comment-361068)

-

Didier Great Tool. I have incorporated this tool into our EnCase Integrated Threat Toolkit (EITT) and our customers absolutely love this tool. I have a need for a tool that can parse all office docs looking for embedded objects just like your PDF tools do. Do you currently have a tool that would do this or is there a way to customize this tool to include all office docs. thanks.

Comment by Mark Morgan — Tuesday 20 December 2016 @ [22:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-361272)

-

Yes, my oledump.py tool.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 21 December 2016 @ [19:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-361293)

-

[…] When you receive a suspicious PDF these days, it could be just a scam without malicious code. Let’s see how to analyze such samples with PDF Tools. […]

Pingback by [PDF Analysis: Back To Basics | NVISO LABS – blog](http://blog.nviso.be/2016/12/28/pdf-analysis-back-to-basics/) — Wednesday 28 December 2016 @ [11:28](https://blog.didierstevens.com/programs/pdf-tools/#comment-361451)

-

Hi Didier,

I recently received a PDF document that I have attempted to analyze using your tools. When opened, it was obvious that the document has a link to a credential harvesting site that it tempts users to click on. However, using pdf-parser, I am unable to locate the URI object. I attempted to decompress the 4 object streams but received errors relating to unexpected compression method. I then attempted to follow the method you posted regarding the handling of special PDF compression methods but also to no avail. Is this a new technique or is there something I have missed? Of note, there appears to be some form of DRM/encryption also applied as there are also 2 /Encrypt objects. I have uploaded the file to VT (SHA 256: 7d2b615630efd2fa3713d97e57afb9972f43e7d4a67cc706af7c789dd1dbe47f) if you are interested in taking a look.

Tom

Comment by Tom — Thursday 5 January 2017 @ [18:54](https://blog.didierstevens.com/programs/pdf-tools/#comment-361670)

-

You need to decrypt the PDF with a tool like qpdf.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 9 January 2017 @ [20:03](https://blog.didierstevens.com/programs/pdf-tools/#comment-361776)

-

[…] Didier Stevens’ın PDF Araçları – PDF dosyalarını analiz, tanıma ve yaratma (PDFiD, pdf-parser, mPDF ve make-pdf dahil) […]

Pingback by [Python ve Güvelik Modülleri - Python Türkiye](http://www.python.tc/python-guvelik-modulleri/) — Monday 23 January 2017 @ [21:19](https://blog.didierstevens.com/programs/pdf-tools/#comment-362115)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser andmake-pdf and mPDF) […]

Pingback by [Python Tools – Toor](https://toor.000webhostapp.com/2017/02/26/python-tools/) — Sunday 26 February 2017 @ [8:52](https://blog.didierstevens.com/programs/pdf-tools/#comment-363277)

-

[…] of the name /JavaScript. However it is easy to write a program that normalizes obfuscated names (pdfid does this for […]

Pingback by [Developing complex Suricata rules with Lua – part 1 | NVISO LABS – blog](http://blog.nviso.be/2017/03/10/developing-complex-suricata-rules-with-lua-part-1/) — Friday 10 March 2017 @ [7:46](https://blog.didierstevens.com/programs/pdf-tools/#comment-363811)

-

[…] Files: PDF pdfid pdfid Locate common suspicious artifacts in a PDF file remnux-didier (APT) https://blog.didierstevens.com/programs/pdf-tools/ Examine Document Files: PDF Pdfobjflow pdf-parser.py | pdfobjflow.py Visualizes the output from […]

Pingback by [Remnux-A tool for reverse engineering Malware – Infohub](https://infoupdatesection.wordpress.com/2017/04/08/remnux-a-tool-for-reverse-engineering-malware/) — Saturday 8 April 2017 @ [22:40](https://blog.didierstevens.com/programs/pdf-tools/#comment-365026)

-

[…] and reading I came accross Didier Stevens’s blog, which contained information on a bunch PDF tools he had written and how to use them. After some fiddling and searching, I found some JavaScript […]

Pingback by [ZonkSec - DakotaCon 2017 CTF Write Ups](https://zonksec.com/blog/dakotacon-2017-ctf-write-ups/) — Wednesday 12 April 2017 @ [17:53](https://blog.didierstevens.com/programs/pdf-tools/#comment-365242)

-

[…] document 123-148752488-reg-invoice.pdf is a PDF with an embedded file and JavaScript. Here is pdfid’s […]

Pingback by [Malicious Documents: The Matryoshka Edition | Didier Stevens](https://blog.didierstevens.com/2017/04/20/malicious-documents-the-matryoshka-edition/) — Thursday 20 April 2017 @ [0:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-365503)

-

[…] Tools: make-pdf tools […]

Pingback by [Bash Bunny Dropping PDF Via HID | Didier Stevens Videos](http://videos.didierstevens.com/2017/04/22/bash-bunny-dropping-pdf-via-hid/) — Saturday 22 April 2017 @ [22:32](https://blog.didierstevens.com/programs/pdf-tools/#comment-365659)

-

[…] pdf tools, oledump.py, […]

Pingback by [Malicious Documents: The Matryoshka Edition | Didier Stevens Videos](http://videos.didierstevens.com/2017/04/23/malicious-documents-the-matryoshka-edition/) — Sunday 23 April 2017 @ [19:36](https://blog.didierstevens.com/programs/pdf-tools/#comment-365684)

-

[…] I create a pure ASCII PDF file with an embedded executable using my make-pdf-embedded.py […]

Pingback by [Bash Bunny PDF Dropper | Didier Stevens](https://blog.didierstevens.com/2017/04/24/bash-bunny-pdf-dropper/) — Monday 24 April 2017 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-365696)

-

[…] make-pdf 0.1.5 This tool will embed javascript inside a PDF document. https://blog.didierstevens.com/programs/pdf-tools/ […]

Pingback by [List of some Penetration Testing Tools – Doxsec](https://doxsec.wordpress.com/2017/04/26/list-of-some-penetration-testing-tools/) — Wednesday 26 April 2017 @ [20:40](https://blog.didierstevens.com/programs/pdf-tools/#comment-365823)

-

[…] https://blog.didierstevens.com/programs/pdf-tools/ […]

Pingback by [CH Magazine | Content-Type Attack: Dark Hole in a Secure Environment](https://www.chmag.in/articles/techgyan/content-type-attack-dark-hole-in-a-secure-environment/) — Friday 12 May 2017 @ [4:13](https://blog.didierstevens.com/programs/pdf-tools/#comment-366698)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and mPDF) […]

Pingback by [Python for penetration testers – vulnerablelife](https://vulnerablelife.wordpress.com/2017/05/13/python-for-penetration-testers/) — Saturday 13 May 2017 @ [5:03](https://blog.didierstevens.com/programs/pdf-tools/#comment-366760)

-

[…] pdf-parser.py from https://blog.didierstevens.com/programs/pdf-tools/ location to be set in first line of […]

Pingback by [DATA - Credential Phish Analysis and Automation - Sapsi Security Services](http://sapsi.org/data-credential-phish-analysis-and-automation/) — Wednesday 7 June 2017 @ [17:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-368676)

-

Hi and thanks for sharing such tools.
Unfortunately when I try pdf_parser I always get the following error even when calling it with just -h argument :
pdf-parser.py”, line 561
except zlib.error as e:
^
SyntaxError: invalid syntax

OS : Windows10
Python 2.6.6

Do I have to install something else ?
Cheers.

Comment by AdV — Tuesday 25 July 2017 @ [16:12](https://blog.didierstevens.com/programs/pdf-tools/#comment-372516)

-

Can you try with the latest version of Python 2.7? I.e. 2.7.13

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 25 July 2017 @ [16:15](https://blog.didierstevens.com/programs/pdf-tools/#comment-372517)

-

Thanks for fast reply !
Sorry Didier I probably didn’t type in the correct syntax.
I tried again with full path to Python.exe and it worked (at least with -h).
I’m gonna try on my pdf now.

Sorry for that unuseful comment and you might want to delete it. No problem.
Cheers.

Comment by AdV — Tuesday 25 July 2017 @ [16:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-372519)

-

Hi Didier,
I’m wondering if you’ve released any new versions of pdfid.py and pdf-parser.py? Kali folks just released, for free, “Kali linux revealed” and I wanted to take a look, however, pdfid.py hangs while trying to analyze this file. After ctrl-c this is the output I get:
$ python pdfid.py Kali_Revealed_1st_edition (1).pdf
^CTraceback (most recent call last):
File “pdfid.py”, line 930, in
Main()
File “pdfid.py”, line 927, in Main
PDFiDMain(filenames, options)
File “pdfid.py”, line 885, in PDFiDMain
ProcessFile(filename, options, plugins)
File “pdfid.py”, line 704, in ProcessFile
xmlDoc = PDFiD(filename, options.all, options.extra, options.disarm, options.force)
File “pdfid.py”, line 513, in PDFiD
attErrorOccured.nodeValue = ‘True’
AttributeError: ‘NoneType’ object has no attribute ‘nodeValue’

I re-downloaded the file, so I don’t think it’s corrupt… Maybe some new features pdf format added that aren’t accounted for in pdfid.py?

Comment by Gene — Thursday 27 July 2017 @ [0:34](https://blog.didierstevens.com/programs/pdf-tools/#comment-372667)

-

Can you rename the file (delete ” (1)” from the filename) and try again?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 28 July 2017 @ [17:26](https://blog.didierstevens.com/programs/pdf-tools/#comment-372810)

-

Didier,
I edited the path out of the name, when I posted, and forgot to get rid of the (1). I downloaded this twice, just to make sure, so that’s why one name was changed by the browser. I ran the utility against both filenames, the original name without the (1) and the one with (1), same result. The file name was in quotes when I ran this in the shell, just to re-iterate. It is failing because of some pdf format, not because of a file name.

Comment by Gene — Wednesday 2 August 2017 @ [7:26](https://blog.didierstevens.com/programs/pdf-tools/#comment-373211)

-

Sorry, but I can not reproduce your problem:

PDFiD 0.2.1 Kali_Revealed_1st_edition.pdf
PDF Header: %PDF-1.4
obj 3304
endobj 3304
stream 486
endstream 486
xref 1
trailer 1
startxref 1
/Page 344
/Encrypt 0
/ObjStm 0
/JS 0
/JavaScript 0
/AA 0
/OpenAction 0
/AcroForm 0
/JBIG2Decode 0
/RichMedia 0
/Launch 0
/EmbeddedFile 0
/XFA 0
/Colors > 2^24 0

The MD5 of the file is 40CD00C451F9037A32352031CBED84E5, check if you have the same file.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 2 August 2017 @ [17:32](https://blog.didierstevens.com/programs/pdf-tools/#comment-373238)

-

Hey Didier,
Great work, BTW. I had a suggestion for what I think would be a useful feature for pdfid. In addition to the strings you’re currently counting, also count “/URI (http”. I think that all of the malicious PDF files I’ve seen for the last couple of years have just been vehicles to get malicious links past email filtering. It would be useful as well, to actually parse out the links, as pdf-parser does, but that’s probably beyond your intended scope for this tool. Another possible alternate way to do this would be to count only ‘suspicious’ http URI values, such as those using bare IP addresses, shortened URLs, or other criteria.
Thanks
John McCash

Comment by [John McCash](https://digital-forensics.sans.org/blog/author/johnmccash) — Thursday 21 September 2017 @ [15:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-377415)

-

[…] a tool such as pdfid.py or peepdf.py to perform some reckon on the documents (e.g. suspicious tags, potential JavaScript […]

Pingback by [(Not) All She Wrote: Rigged PDFs | Security Over Simplicity](https://securityoversimplicity.wordpress.com/2017/09/28/not-all-she-wrote-rigged-pdfs/) — Thursday 28 September 2017 @ [17:18](https://blog.didierstevens.com/programs/pdf-tools/#comment-377917)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and mPDF) […]

Pingback by [Python for penetration testers – CISO Tunisia](https://cisotn.wordpress.com/2017/03/17/python-for-penetration-testers/) — Sunday 22 October 2017 @ [11:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-379857)

-

Didier – I’m tying to use pdf-parser.py to extract images from some small pdf documents. So far I’ve been able to use –stats to find the /XObjects, and -o to save those as files. The images in the documents I am playing with are very small, and are embedded withing the text. Is there a way to expose/extract the location of the bounding box (or otherwise locate the text surrounding the images)?

Comment by Joey Quinn — Tuesday 14 November 2017 @ [23:37](https://blog.didierstevens.com/programs/pdf-tools/#comment-381680)

-

[…] PDF Tools by Didier Stevens […]

Pingback by [Checking for maliciousness in Acroform objects on PDF files – Furoner.CAT](https://furoner.wordpress.com/2017/11/15/checking-for-maliciousness-in-acroform-objects-on-pdf-files/) — Wednesday 15 November 2017 @ [15:22](https://blog.didierstevens.com/programs/pdf-tools/#comment-381758)

-

Yes, you have to look into the stream of the objects that put the images on the page.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 16 November 2017 @ [9:27](https://blog.didierstevens.com/programs/pdf-tools/#comment-381813)

-

[…] this new version of pdfid.py, a new option was added: […]

Pingback by [Update: pdfid.py Version 0.2.3 | Didier Stevens](https://blog.didierstevens.com/2017/11/27/update-pdfid-py-version-0-2-3/) — Monday 27 November 2017 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-382674)

-

[…] PDF Tools de Didier Stevens. PDFStreamDumper – Esta es una herramienta gratuita para el análisis PDFs maliciosos. SWF Mastah – Programa en Python que extrae stream SWF de ficheros PDF. […]

Pingback by [Forensics PowerTools (Listado de herramientas forenses) – Securiza Neuquen](https://securizanqn.wordpress.com/2017/12/12/primera-entrada-del-blog/) — Wednesday 13 December 2017 @ [0:33](https://blog.didierstevens.com/programs/pdf-tools/#comment-383872)

-

[…] pdfid.py confirms the PDF is encrypted (name /Encrypt): […]

Pingback by [Cracking Encrypted PDFs – Part 1 | Didier Stevens](https://blog.didierstevens.com/2017/12/26/cracking-encrypted-pdfs-part-1/) — Tuesday 26 December 2017 @ [17:15](https://blog.didierstevens.com/programs/pdf-tools/#comment-384893)

-

[…] keys, you can always check the /Encrypt dictionary of the PDF you created, for example with my pdf-parser (in this example /Length 128 tells us a 128-bit key is […]

Pingback by [Cracking Encrypted PDFs – Conclusion | Didier Stevens](https://blog.didierstevens.com/2017/12/29/cracking-encrypted-pdfs-conclusion/) — Friday 29 December 2017 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-385017)

-

[…] Tools: pdfid.py, pdf-parser.py […]

Pingback by [PDF’s /URI – Didier Stevens Videos](http://videos.didierstevens.com/2017/12/31/pdfs-uri/) — Sunday 31 December 2017 @ [17:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-385191)

-

Getting these errors. I have tried doing some stuff suggested here with no luck

C:\Python26>python pdfid.py sampls/pdf-thisCreator.file
‘module’ object has no attribute ‘OrderedDict’

C:\Python26>python pdfid.py samples/pdf-thisCreator.file
‘module’ object has no attribute ‘OrderedDict’

C:\Python26>pdfid.py BouncingButton.pdf
‘module’ object has no attribute ‘OrderedDict’

Comment by Mick — Sunday 4 February 2018 @ [18:46](https://blog.didierstevens.com/programs/pdf-tools/#comment-386837)

-

Try Python 2.7

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 4 February 2018 @ [19:46](https://blog.didierstevens.com/programs/pdf-tools/#comment-386842)

-

That worked,

C:\Python27>python pdfid.py samples/pdf-thisCreator.file

C:\Python27>

I have pdf files in the folder that have malicious files in them

Comment by [morrim03](http://mtdpeenie.wordpress.com) — Sunday 4 February 2018 @ [23:37](https://blog.didierstevens.com/programs/pdf-tools/#comment-386848)

-

[…] Didier’s PDF Tools […]

Pingback by [Most Important Tools and Resources For Security Researcher, Malware Analyst, Reverse Engineer – My (Yet Another) Cybersecurity Blog](https://mycybersecblog.wordpress.com/2018/02/26/most-important-tools-and-resources-for-security-researcher-malware-analyst-reverse-engineer/) — Wednesday 28 February 2018 @ [8:13](https://blog.didierstevens.com/programs/pdf-tools/#comment-388248)

-

Using latest version of pdfid.py (0.2.4) and noticed that it’s looking for /AA and /JS inside of images within the XMP packet (). If –disarm is used, it will replace those occurrences and break the images.

Comment by Kerri — Saturday 7 April 2018 @ [7:46](https://blog.didierstevens.com/programs/pdf-tools/#comment-390398)

-

Please read this: https://blog.didierstevens.com/2013/06/10/pdfid-false-positives/

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 7 April 2018 @ [7:55](https://blog.didierstevens.com/programs/pdf-tools/#comment-390399)

-

[…] tool pdfid.py can now be extended to report /GoToE and /GoToR usage in a PDF file, without having to change the […]

Pingback by [PDFiD: GoToE and GoToR Detection (“NTLM Credential Theft”) | Didier Stevens](https://blog.didierstevens.com/2018/05/31/pdfid-gotoe-and-gotor-detection-ntlm-credential-theft/) — Thursday 31 May 2018 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-393654)

-

I downloaded the pdfid and the MD5 of it is 27552a07710951bdcf992f8691153920 which doesn’t match the one you provided.

Comment by Alina — Thursday 14 June 2018 @ [20:37](https://blog.didierstevens.com/programs/pdf-tools/#comment-394763)

-

Please pay attention to the information I publish: I provide the hash (MD5/SHA256) of the

ZIP

file.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 14 June 2018 @ [23:20](https://blog.didierstevens.com/programs/pdf-tools/#comment-394773)

-

[…] pdfid.py, we start the analysis […]

Pingback by [Extracting a Windows Zero-Day from an Adobe Reader Zero-Day PDF | NVISO LABS – blog](http://blog.nviso.be/2018/07/03/extracting-a-windows-zero-day-from-an-adobe-reader-zero-day-pdf/) — Tuesday 3 July 2018 @ [20:48](https://blog.didierstevens.com/programs/pdf-tools/#comment-396217)

-

[…] of this variant is not difficult. First with pdfid.py, the presence of /EmbeddedFile, /JavaScript and /AutoOpen are a strong indicator for such malicious […]

Pingback by [Shortcomings of blacklisting in Adobe Reader and what you can do about it | NVISO LABS – blog](http://blog.nviso.be/2018/07/26/shortcomings-of-blacklisting-in-adobe-reader-and-what-you-can-do-about-it/) — Thursday 26 July 2018 @ [14:10](https://blog.didierstevens.com/programs/pdf-tools/#comment-397915)

-

[…] when performing PDF document analysis, PDFiD is the starting place: it gives us an idea what we can expect to find inside the document. We take […]

Pingback by [Differential Malware Analysis: An Example – NVISO Labs](http://blog.nviso.be/2018/08/31/differential-malware-analysis-an-example/) — Friday 31 August 2018 @ [6:01](https://blog.didierstevens.com/programs/pdf-tools/#comment-400993)

-

[…] the compressed OLE file, but then I remembered I had fixed a problem with zlib extraction in pdf-parser.py. Taking this code into plugin_ppt.py fixed the decompression […]

Pingback by [Analyzing PowerPoint Maldocs with oledump Plugin plugin_ppt | Didier Stevens](https://blog.didierstevens.com/2018/10/25/analyzing-powerpoint-maldocs-with-oledump-plugin-plugin_ppt/) — Thursday 25 October 2018 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-406748)

-

[…] new version of pdf-parser brings support for analysis of stream objects (/ObjStm). Use new option -O to enable this […]

Pingback by [Update: pdf-parser.py Version 0.7.0 | Didier Stevens](https://blog.didierstevens.com/2019/02/28/update-pdf-parser-py-version-0-7-0/) — Thursday 28 February 2019 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-424641)

-

[…] Tools: pdfid.py, pdf-parser.py […]

Pingback by [PDF: Stream Objects (/ObjStm) – Didier Stevens Videos](http://videos.didierstevens.com/2019/02/28/pdf-stream-objects-objstm/) — Thursday 28 February 2019 @ [0:24](https://blog.didierstevens.com/programs/pdf-tools/#comment-424651)

-

[…] First I start the analysis with pdfid.py: […]

Pingback by [Analyzing a Phishing PDF with /ObjStm | Didier Stevens](https://blog.didierstevens.com/2019/03/07/analyzing-a-phishing-pdf-with-objstm/) — Thursday 7 March 2019 @ [0:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-425985)

-

[…] Tools: pdfid.py, pdf-parser.py […]

Pingback by [Analyzing a Phishing PDF with /ObjStm – Didier Stevens Videos](http://videos.didierstevens.com/2019/03/11/analyzing-a-phishing-pdf-with-objstm/) — Monday 11 March 2019 @ [9:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-426851)

-

[…] Didier Stevens’ PDF tools: analyse, identify and create PDF files (includes PDFiD, pdf-parser and make-pdf and mPDF) […]

Pingback by [Python Cyber Security Testing Tool Collection - Cyber Security Memo](https://www.51sec.org/2019/01/python-cyber-security-testing-tool-collection/) — Wednesday 13 March 2019 @ [15:59](https://blog.didierstevens.com/programs/pdf-tools/#comment-427503)

-

[…] perform a quick check of an online PDF document, that I expect to be benign, I will just point my PDF tools to the online document. When you provide a URL argument to pdf-parser, it will download the […]

Pingback by [Quickpost: PDF Tools Download Feature | Didier Stevens](https://blog.didierstevens.com/2019/03/23/quickpost-pdf-tools-download-feature/) — Saturday 23 March 2019 @ [9:34](https://blog.didierstevens.com/programs/pdf-tools/#comment-430591)

-

File “./pdf-parser.py”, line 1329
dKeywords = {keyword: [] for keyword in keywords}
^
SyntaxError: invalid syntax

My Python version is 2.6.6

Comment by Hoop — Thursday 11 April 2019 @ [17:14](https://blog.didierstevens.com/programs/pdf-tools/#comment-436138)

-

Indeed, that syntax is not supported by Python 2.6.6.

You can replace that line with the following three lines, and it will work for you:

dKeywords = {}
for keyword in keywords:
dKeywords[keyword] = []

Update: it’s not appearing in the comment, but that last line needs 4 extra space characters for indentation.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 11 April 2019 @ [17:24](https://blog.didierstevens.com/programs/pdf-tools/#comment-436144)

-

Thanks. That worked. I’m surprised, since 2.6.6 is newer than the minimum version 2.5.1.

Comment by Hoop — Thursday 11 April 2019 @ [18:05](https://blog.didierstevens.com/programs/pdf-tools/#comment-436151)

-

I no longer test these old versions. I only do when an issue is reported.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 11 April 2019 @ [18:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-436152)

-

[…] Tools: pdfid.py, pdf-parser.py […]

Pingback by [Analysis of PDFs Created with OpenOffice/LibreOffice – Didier Stevens Videos](http://videos.didierstevens.com/2019/05/19/analysis-of-pdfs-created-with-openoffice-libreoffice/) — Sunday 19 May 2019 @ [8:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-440558)

-

@Didier, on 5 May 2009 you wrote “I’ve still to decide when I upgrade my tools to Python 3”
According to https://www.python.org/dev/peps/pep-0373/
maintenance for Python 2.7 ends January 2020

Comment by Joe — Thursday 23 May 2019 @ [6:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-440870)

-

@Joe And I decided in 2009 to support Python 3. pdf-parser.py and pdfid.py run on Python 2 and Python 3 for 10 years now.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 24 May 2019 @ [17:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-440947)

-

Hello, is it possible to make some suggestions/fixes or pull requests for for tool “pdfid.py”? We are using this as one tool in our open source malware analysis platform (CinCan), but I noticed that tool does not provide option to produce output in json/xml format even though methods are implemented in source code. This would be handful.

Comment by [Niklas Saari](https://cincan.io) — Friday 31 May 2019 @ [12:43](https://blog.didierstevens.com/programs/pdf-tools/#comment-441301)

-

Is your tool written in Python?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 31 May 2019 @ [14:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-441303)

-

We are using your tool as standalone tool among many other tools to make analysis for sample file(s), and produce output data from them. This is implemented by building CI/CD pipeline which finally generates results combining result data of different tools. In practice, we are using your tool from command line, and we can’t import it as Python library. It would be nice to be able to produce json/xml output with command line arguments from the official version of the pdfid.

If you are interested to see pdfid’s role, more can be seen here https://gitlab.com/CinCan/pipelines/tree/master/document-pipeline
We are using triage plugin.

Comment by [Niklas Saari](https://cincan.io) — Friday 31 May 2019 @ [17:35](https://blog.didierstevens.com/programs/pdf-tools/#comment-441310)

-

What is a CI/CD pipeline?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 2 June 2019 @ [19:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-441399)

-

Continuous Integration/Delivery. Here is typical pipeline explained: https://www.michielrook.nl/2018/01/typical-ci-cd-pipeline-explained/
However, our use case is bit a different. We are pushing malware samples into the git repository, which triggers CI pipeline, as in this case the analysis process for samples. Different kind of standalone tools have been executed, sometimes based on output of previous tool. Output of different tools have been finally gathered and pushed in to “results” git repository/branch.

Comment by [Niklas Saari](https://cincan.io) — Monday 3 June 2019 @ [12:26](https://blog.didierstevens.com/programs/pdf-tools/#comment-441431)

-

I’ll add it to my todo list.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 11 June 2019 @ [16:04](https://blog.didierstevens.com/programs/pdf-tools/#comment-441760)

-

[…] pdfid.py and pdf-parser.py, QPDF and […]

Pingback by [Encrypted Sextortion PDFs – Didier Stevens Videos](http://videos.didierstevens.com/2019/09/22/encrypted-sextortion-pdfs/) — Sunday 22 September 2019 @ [17:56](https://blog.didierstevens.com/programs/pdf-tools/#comment-453469)

-

[…] PDF Tools de Didier Stevens.PDFStreamDumper – Esta es una herramienta gratuita para el análisis PDFs maliciosos.SWF Mastah – Programa en Python que extrae stream SWF de ficheros PDF.Proccess explorer – Muestra información de los procesos.Captura BAT – Permite la monitorización de la actividad del sistema o de un ejecutable.Regshot – Crea snapshots del registro pudiendo comparar los cambios entre ellosBintext – Extrae el formato ASCII de un ejecutable o fichero.LordPE – Herramienta para editar ciertas partes de los ejecutables y volcado de memoria de los procesos ejecutados.Firebug – Analisis de aplicaciones web.IDA Pro – Depurador de aplicaciones.OllyDbg – Desemsamblador y depurador de aplicaciones o procesos.Jsunpack-n – Emula la funcionalidad del navegador al visitar una URL. Su propósito es la detección de exploitsOfficeMalScanner – Es una herramienta forense cuyo objeto es buscar programas o ficheros maliciosos en Office.Radare – Framework para el uso de ingeniería inversa.FileInsight – Framework para el uso de ingeniería inversa.Volatility Framework con los plugins malfind2 y apihooks.shellcode2exe – Conversor de shellcodes en binarios. […]

Pingback by [Herramientas análisis de malware | WhiteSuit Hacking](https://blog.whitesuithacking.com/2019/08/22/herramientas-analisis-de-malware/) — Monday 23 September 2019 @ [0:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-453498)

-

[…] 00. Vor allem führte ich eine Analyse durch pdfid.py von Didier Stevens durch (https://blog.didierstevens.com/programs/pdf-tools/). […]

Pingback by [The nAbAt a ICC soApboX » Trojaner im Dienst an NATO](https://www.cyberguerrilla.org/blog/trojaner-im-dienst-an-nato/) — Friday 18 October 2019 @ [14:35](https://blog.didierstevens.com/programs/pdf-tools/#comment-457557)

-

[…] A quick search will point us in the right direction which is the code author’s website […]

Pingback by [How to remove malicious code from PDF files – Ernst Renner](https://www.ernstrenner.com/how-to-remove-malicious-code-from-pdf-files/) — Sunday 20 October 2019 @ [21:20](https://blog.didierstevens.com/programs/pdf-tools/#comment-457961)

-

Excellent tools – still useful 11+ Years after release! Thank you for creating these. Any plans on updating mPDF.py / make-pdf-javascript.py / etc.. for Python 3? Fedora no longer ships with v2. I’m trying to create some sample PDF’s for lab use.

Comment by Anthony — Sunday 17 November 2019 @ [0:53](https://blog.didierstevens.com/programs/pdf-tools/#comment-461965)

-

I’ll try to do this by the end of the month, I forgot that the “make tools” were not Python 3 ready. I’m busy with oledump and plugins right now.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 17 November 2019 @ [7:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-462011)

-

Hello I have a PDF with a CIDFont CID TrueType composite font embedded as a stream object in the PDF. I’d like to be able to see the character glyphs in this CID composite font. With your parser tool can I extract this object for processing by something like FontForge? Also will your parser run in Windows? Thanks

Comment by Justin Latus — Saturday 18 April 2020 @ [19:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-490228)

-

I don’t know, you will have to try. I analyze malicious PDFs, your’s is not malicious I assume. Yes, my tools work on Windows.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 19 April 2020 @ [8:49](https://blog.didierstevens.com/programs/pdf-tools/#comment-490433)

-

Hello, a new file is generated and an error is reported. Do you know the solution?
“Traceback (most recent call last):
File “replace_ascii.py”, line 34, in
Main()
File “replace_ascii.py”, line 24, in Main
oPDF.comment(‘¸©Ñ¦’)
File “E:\Debug\PythonDbg\replace_ascii\mPDF.py”, line 163, in comment
self.appendString(‘%’ + comment + ‘\n’)
File “E:\Debug\PythonDbg\replace_ascii\mPDF.py”, line 108, in appendString
fPDF.write(str)
UnicodeEncodeError: ‘gbk’ codec can’t encode character ‘\xb8’ in position 1: illegal multibyte sequence”

Comment by Anonymous — Thursday 21 May 2020 @ [5:49](https://blog.didierstevens.com/programs/pdf-tools/#comment-500843)

-

I need more context: I did not write that replace ascii program. If you wrote it, then you should at least post a code snippet where the error occurs.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 21 May 2020 @ [11:36](https://blog.didierstevens.com/programs/pdf-tools/#comment-500923)

-

The above error is “pdf- parser.py -g “generated, I want to remove the confusion of PDF, and then generate a PDF file ,

Comment by Anonymous — Friday 22 May 2020 @ [2:22](https://blog.didierstevens.com/programs/pdf-tools/#comment-501096)

-

What version of Python are you using?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 25 May 2020 @ [22:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-502176)

-

version python 3.8.2 ,Can the code generated by “pdf-parser.py -g” remove confusion? I still use it “oPDF.indirectobject(2, 0, ‘<>’)
oPDF.indirectobject(3, 0, ‘<>’)”

Comment by Anonymous — Wednesday 27 May 2020 @ [2:25](https://blog.didierstevens.com/programs/pdf-tools/#comment-502508)

-

I don’t think I’ve updated mpdf to Python 3. What is confusion?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 27 May 2020 @ [22:02](https://blog.didierstevens.com/programs/pdf-tools/#comment-502683)

-

Hey ya, just wanted to say really thank you for this awesome tool.

Comment by Anonymous — Thursday 15 October 2020 @ [13:25](https://blog.didierstevens.com/programs/pdf-tools/#comment-539151)

-

Hey man, awesome tool, thank you!

Comment by Anonymous — Friday 6 November 2020 @ [8:57](https://blog.didierstevens.com/programs/pdf-tools/#comment-545513)

-

Thank you for these tools and the helpful videos on your website and YouTube! However, since I am a complete layman, I still have some difficulties. I’ve tried to reproduce/apply what you did in your exercise videos on YouTube. I have a PDF file which contains 1 /JS keyword and 1 /OpenAction keyword. However I can’t find the object from which this /JS keyword stems. Using -s javascript or -s js doesn’t yield any results. Any ideas how I can find out the “location” of this /JS keyword?

Comment by Anonymous — Saturday 21 November 2020 @ [4:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-549564)

-

This will probably help you: https://blog.didierstevens.com/2013/06/10/pdfid-false-positives/

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 21 November 2020 @ [9:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-549606)

-

[…] Tool: pdftool.py […]

Pingback by [pdftool.py: Incremental Updates – Didier Stevens Videos](http://videos.didierstevens.com/2021/01/30/pdftool-py-incremental-updates/) — Saturday 30 January 2021 @ [22:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-567627)

-

I’m having a few issues with pdftool.py. I run it with the input “iu” and I get…

C:\Users\jocaw\OneDrive\Documents\Python>pdftool.py iu snn.pdf
File: snn.pdf
Traceback (most recent call last):
File “C:\Users\jocaw\OneDrive\Documents\Python\pdftool.py”, line 1727, in
Main()
File “C:\Users\jocaw\OneDrive\Documents\Python\pdftool.py”, line 1720, in Main
ProcessBinaryFiles(command, oExpandFilenameArguments.Filenames(), oLogfile, options, oParserFlag)
File “C:\Users\jocaw\OneDrive\Documents\Python\pdftool.py”, line 1655, in ProcessBinaryFiles
ProcessBinaryFile(command, filename, None, cutexpression, flag, oOutput, oLogfile, options, oParserFlag)
File “C:\Users\jocaw\OneDrive\Documents\Python\pdftool.py”, line 1631, in ProcessBinaryFile
PDFIncrementalUpdates(data, oOutput, options)
File “C:\Users\jocaw\OneDrive\Documents\Python\pdftool.py”, line 1585, in PDFIncrementalUpdates
newVersions = PDFIncrementalUpdatesSub(data, oOutput, options)
File “C:\Users\jocaw\OneDrive\Documents\Python\pdftool.py”, line 1549, in PDFIncrementalUpdatesSub
accumulate.write(token)
AttributeError: ‘cStringIO.StringI’ object has no attribute ‘write’

It doesn’t think the “accumulate” has the “write” attribute. Help, any advice?

Many thanks James.

Comment by James C — Friday 6 August 2021 @ [22:51](https://blog.didierstevens.com/programs/pdf-tools/#comment-583423)

-

Link to the pdf I’m using: https://drive.google.com/file/d/1OFXRCw2U1mo7BjHUSGs_1fVjDsQLRo0V/view?usp=drivesdk

Comment by James C — Friday 6 August 2021 @ [23:37](https://blog.didierstevens.com/programs/pdf-tools/#comment-583425)

-

The pdftool.py is throwing an error for me:

C:\Users\jocaw\Desktop\Python1>pdftool.py iu snn.pdf
File: snn.pdf
Traceback (most recent call last):
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1727, in
Main()
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1720, in Main
ProcessBinaryFiles(command, oExpandFilenameArguments.Filenames(), oLogfile, options, oParserFlag)
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1655, in ProcessBinaryFiles
ProcessBinaryFile(command, filename, None, cutexpression, flag, oOutput, oLogfile, options, oParserFlag)
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1631, in ProcessBinaryFile
PDFIncrementalUpdates(data, oOutput, options)
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1585, in PDFIncrementalUpdates
newVersions = PDFIncrementalUpdatesSub(data, oOutput, options)
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1549, in PDFIncrementalUpdatesSub
accumulate.write(token)
AttributeError: ‘cStringIO.StringI’ object has no attribute ‘write’

I believe it thinks the “accumalate” variable doesn’t have the “write” attribute. Help, any advice to get the script to work for me? Thank you.

Link to the pdf I’m using: https://drive.google.com/file/d/1OFXRCw2U1mo7BjHUSGs_1fVjDsQLRo0V/view?usp=drivesdk

Comment by James C — Friday 6 August 2021 @ [23:42](https://blog.didierstevens.com/programs/pdf-tools/#comment-583426)

-

I was hoping someone could help. When I try and run the pdftool.py script it throws the below error:

C:\Users\jocaw\Desktop\Python1>pdftool.py iu snn.pdf
File: snn.pdf
Traceback (most recent call last):
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1727, in
Main()
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1720, in Main
ProcessBinaryFiles(command, oExpandFilenameArguments.Filenames(), oLogfile, options, oParserFlag)
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1655, in ProcessBinaryFiles
ProcessBinaryFile(command, filename, None, cutexpression, flag, oOutput, oLogfile, options, oParserFlag)
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1631, in ProcessBinaryFile
PDFIncrementalUpdates(data, oOutput, options)
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1585, in PDFIncrementalUpdates
newVersions = PDFIncrementalUpdatesSub(data, oOutput, options)
File “C:\Users\jocaw\Desktop\Python1\pdftool.py”, line 1549, in PDFIncrementalUpdatesSub
accumulate.write(token)
AttributeError: ‘cStringIO.StringI’ object has no attribute ‘write’

It doesn’t believe “accumulate” has the attribute “write”. But I don’t belive this is correct.

I’ve uploaded the pdf I’m using it on if that helps: https://drive.google.com/file/d/1OFXRCw2U1mo7BjHUSGs_1fVjDsQLRo0V/view?usp=drivesdk

Does anyone have a solution to the above error? Thank you

Comment by James C — Saturday 7 August 2021 @ [21:29](https://blog.didierstevens.com/programs/pdf-tools/#comment-583491)

-

Please don’t spam my blog reiterating the same question!

You get this error because you are using Python 2.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 7 August 2021 @ [21:37](https://blog.didierstevens.com/programs/pdf-tools/#comment-583492)

-

Apologies, I thought the earlier ones didn’t go through as they didn’t show (until now). I thought I was doing something wrong so tried again. Feel free to delete the iterations, sorry.

Comment by James C — Sunday 8 August 2021 @ [9:10](https://blog.didierstevens.com/programs/pdf-tools/#comment-583514)

-

Hello, I have been getting this error when trying to extract an object from a pdf: https://cdn.discordapp.com/attachments/457348570157416448/875727564109402122/unknown.png

I’m aware that my python version is ‘too new’ or something (not much of a coding person), but when i omit the keyword -f, it works fine (though is not in a useful state). I tried using the solution you posted here https://isc.sans.edu/forums/diary/Handling+Special+PDF+Compression+Methods/19597/ , but that yields a different set of errors: https://cdn.discordapp.com/attachments/457348570157416448/875729299846615070/unknown.png

I’m nearly at my wit’s end; could someone help me 😦

Comment by manny — Friday 13 August 2021 @ [13:16](https://blog.didierstevens.com/programs/pdf-tools/#comment-583952)

-

For such a case, we can not help unless you tell us were we can find that PDF document.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Friday 13 August 2021 @ [16:19](https://blog.didierstevens.com/programs/pdf-tools/#comment-583972)

-

the pdf was downloaded online from here https://unicode.org/charts/PDF/U10530.pdf

Comment by manny — Friday 13 August 2021 @ [21:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-583982)

-

That PDF is encrypted. You have to decrypt it first.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 18 August 2021 @ [19:04](https://blog.didierstevens.com/programs/pdf-tools/#comment-584395)

-

I used this command in the attempt to read the URI content

pdf-parser.py -k /URI -w ‘aaaaaaa.pdf’

and I received this output.

/URI “(r´’\x8e\x99Òn\x1c®}H¡Ø\x86\x18\x8a}z\x92H)”

How can I decode / interpret the output?

Comment by Anonymous — Saturday 23 October 2021 @ [13:54](https://blog.didierstevens.com/programs/pdf-tools/#comment-589055)

-

Looks like your pdf is encrypted. I have blog posts on encrypted pdfs, take a look.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 23 October 2021 @ [14:00](https://blog.didierstevens.com/programs/pdf-tools/#comment-589056)

-

Thank you for your quick response and all the work you put into your tools. Great work there!
I ran the tool pdfid.py and it returns that the same pdf has no encryption. Within the obj there is no /Lenght indication either. There is instead a “/Subtype /Link”, which indicate a link of some sort. The aim would be to decode that link.

Comment by Anonymous — Saturday 23 October 2021 @ [15:09](https://blog.didierstevens.com/programs/pdf-tools/#comment-589060)

-

I’ll have a look if you can share the file.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Saturday 23 October 2021 @ [15:46](https://blog.didierstevens.com/programs/pdf-tools/#comment-589063)

-

[…] I collected some hundreds of PDFs and converted the PDFs to Python script using Didier Stevens’s pdf-parser -g flag. The fuzzer uses cPDF that I modified to mutate the stream using Charlie’s 10liner, every […]

Pingback by [Fuzzing PDFs like its 1990s – News Priviw](https://news.priviw.com/tech-examples/fuzzing-pdfs-like-its-1990s/) — Sunday 31 October 2021 @ [4:36](https://blog.didierstevens.com/programs/pdf-tools/#comment-589640)

-

Hello,
it says that the license tyo is a “public domain”. I just want to know if its a what kind of license it is (example: GNU GPL, Apache etc.)?
Thanks for the answer

Comment by [Marc](https://fortschritt.co/) — Wednesday 27 April 2022 @ [8:12](https://blog.didierstevens.com/programs/pdf-tools/#comment-597749)

-

Sorry?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Wednesday 4 May 2022 @ [7:31](https://blog.didierstevens.com/programs/pdf-tools/#comment-598267)

-

Hi Dst67, Can you please give a code / demonstrate the how-to:.

1. Insert text which is hyperlinked (to a website) in PDF as content (using javascript)
2. Insert Image which is hyperlinked (to a website) in PDF as content (using javascript)
3. Insert a URL directly which is hyperlinked (to a website).

Comment by Jasim Abdulsalam — Monday 15 August 2022 @ [17:38](https://blog.didierstevens.com/programs/pdf-tools/#comment-607057)

-

Hi Didier Stevens, Can you please give a code / demonstrate the how-to:.

1. Insert text which is hyperlinked (to a website) in PDF as content (using javascript)
2. Insert Image which is hyperlinked (to a website) in PDF as content (using javascript)
3. Insert a URL directly which is hyperlinked (to a website).

Which will open the site in the browser with a click (Image/ Text/ URL).

Comment by Jasim Abdulsalam — Monday 15 August 2022 @ [17:47](https://blog.didierstevens.com/programs/pdf-tools/#comment-607059)

-

Hi Didier Stevens,

Can you please give a code / demonstrate the how-to:.

1. Insert text which is hyperlinked (to a website) in PDF as content (using javascript)
2. Insert Image which is hyperlinked (to a website) in PDF as content (using javascript)
3. Insert a URL directly which is hyperlinked (to a website).

Which will open the site in the browser on click (Image/ Text/ URL

Comment by Jasim Abdulsalam — Monday 15 August 2022 @ [19:54](https://blog.didierstevens.com/programs/pdf-tools/#comment-607066)

-

Hi Didier Stevens
i want to extract the pdf 23 keywords and their values displayed by the pdfid.py and tips on how i can go about it

Comment by Aliyu Musa — Tuesday 1 November 2022 @ [14:39](https://blog.didierstevens.com/programs/pdf-tools/#comment-611133)

-

Can you be more specific? What is the problem are you encountering?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Sunday 6 November 2022 @ [10:36](https://blog.didierstevens.com/programs/pdf-tools/#comment-611310)

-

I’m getting an error with the -O option:

dan@localhost:~/Downloads/malsamples$ pdf-parser.py –version
pdf-parser.py 0.7.8
dan@localhost:~/Downloads/malsamples$ python3 –version
Python 3.8.10
dan@localhost:~/Downloads/malsamples$ pdf-parser.py -a -O ComplaintApril_836056093.pdf
Traceback (most recent call last):
File “/usr/local/bin/pdf-parser.py”, line 644, in Decompress
data = FlateDecode(data)
File “/usr/local/bin/pdf-parser.py”, line 1021, in FlateDecode
return zlib.decompress(C2BIP3(data))
zlib.error: Error -3 while decompressing data: incorrect header check

During handling of the above exception, another exception occurred:

Traceback (most recent call last):
File “/usr/local/bin/pdf-parser.py”, line 1760, in
Main()
File “/usr/local/bin/pdf-parser.py”, line 1559, in Main
indexes = list(map(int, C2SIP3(object.Stream())[:offsetFirstObject].strip().split(‘ ‘)))
File “/usr/local/bin/pdf-parser.py”, line 627, in Stream
return self.Decompress(data, filters)
File “/usr/local/bin/pdf-parser.py”, line 649, in Decompress
return message + ‘. zlib.error %s’ % e.message
AttributeError: ‘error’ object has no attribute ‘message’

Comment by Dan Nelson — Friday 14 April 2023 @ [15:19](https://blog.didierstevens.com/programs/pdf-tools/#comment-617529)

-

Can you share the sample?

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 1 May 2023 @ [20:41](https://blog.didierstevens.com/programs/pdf-tools/#comment-618215)

-

When pdfid is showing /JS /JavaScript with value 1, does that mean that there is one JavaScript object in the PDF or are there scenarios possible, where /JS and /Javascript show different values?

Comment by Anonymous — Tuesday 9 January 2024 @ [14:58](https://blog.didierstevens.com/programs/pdf-tools/#comment-627599)

-

Since pdfid doesn’t parse objects, just keywords, there’s no way to tell wih pdfid only. You should check with pdf-parser too.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 15 January 2024 @ [16:56](https://blog.didierstevens.com/programs/pdf-tools/#comment-627663)

-

do you have example on how it works opening in android os (like mobile, tablet)?

Comment by Anonymous — Monday 30 December 2024 @ [1:34](https://blog.didierstevens.com/programs/pdf-tools/#comment-629786)

-

No, I don’t.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Tuesday 31 December 2024 @ [16:21](https://blog.didierstevens.com/programs/pdf-tools/#comment-629791)

-

Didier,
I am using your nice mPDF.py module to assemble digital monochrome images (all same size) and generating a PDF with spot colors (separations).
There is a point in my program I would like to improve and that is the compression of the images.
I am compressing the images with zlib, given the contents of the image:

filtro = ” /Filter /FlateDecode”
streamdata = zlib.compress(streamdata)
dictionary = (“<</ImageName/%s/Name/%s” % (name,name) + filtro +
“/BitsPerComponent 8/Subtype/Image/Type/XObject” +
“/ColorSpace %s/Width %d/Height %d/Length %d>>”)
self.appendString(“n”)
self.indirectObjects[index] = self.filesize()
self.appendString((“%d %d objn” + dictionary + “nstreamn”) %
(index, version, colorspace, w, h, len(streamdata)))

However, I know that most of the PDFs generated by Adobe use a /Predictor 2 field to improve compression.
zlib uses a lower level function, ‘compressobj’, but I can’t figure out how to set it up (a dict, the documentation says) to obtain the compressed data.

Could you throw some light how to do it, please?

Comment by Anonymous — Monday 3 February 2025 @ [2:15](https://blog.didierstevens.com/programs/pdf-tools/#comment-629897)

-

Unfortunately, I can’t help you with that, I’m not a PDF expert (I’m a malicious PDF expert).

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Monday 3 February 2025 @ [15:23](https://blog.didierstevens.com/programs/pdf-tools/#comment-629899)

-

wow from 2008 to 2025 people are still active its great to see the community

Comment by Anonymous — Friday 18 April 2025 @ [12:25](https://blog.didierstevens.com/programs/pdf-tools/#comment-630105)

-

@Didier Stevens – Any interest in having PDFid.py inspect or warn of potential evasion-like embedded activity? Recently came across an eicar+JS embedded PDF example that passed with no flagging of embedded objects such as JS, JavaScript, OpenAction and etc. Since obfuscation seems to be something an attacker would use, have you ever considered having some sort of logic or attribute that shows obfuscation may be at play?

Comment by Anonymous — Wednesday 3 September 2025 @ [20:53](https://blog.didierstevens.com/programs/pdf-tools/#comment-630465)

-

pdfid handles name obfuscation.
Can you share your sample? Or just the hash if it’s on VT.

Comment by [Didier Stevens](https://didierstevens.wordpress.com/) — Thursday 4 September 2025 @ [17:29](https://blog.didierstevens.com/programs/pdf-tools/#comment-630468)

[RSS feed for comments on this post.](https://blog.didierstevens.com/programs/pdf-tools/feed/) [TrackBack URI](https://blog.didierstevens.com/programs/pdf-tools/trackback/)

### Leave a Reply (comments are moderated)

This site uses Akismet to reduce spam. [Learn how your comment data is processed.](https://akismet.com/privacy/)

-

## Pages

- [About](https://blog.didierstevens.com/about/)
- [Didier Stevens Suite](https://blog.didierstevens.com/didier-stevens-suite/)
- [Links](https://blog.didierstevens.com/links/)
- [My Python Templates](https://blog.didierstevens.com/my-python-templates/)
- [My Software](https://blog.didierstevens.com/my-software/)
- [Professional](https://blog.didierstevens.com/professional/)
- [Programs](https://blog.didierstevens.com/programs/)

- [Ariad](https://blog.didierstevens.com/programs/ariad/)
- [Authenticode Tools](https://blog.didierstevens.com/programs/authenticode-tools/)
- [Binary Tools](https://blog.didierstevens.com/programs/binary-tools/)
- [CASToggle](https://blog.didierstevens.com/programs/castoggle/)
- [Cobalt Strike Tools](https://blog.didierstevens.com/programs/cobalt-strike-tools/)
- [Disitool](https://blog.didierstevens.com/programs/disitool/)
- [EICARgen](https://blog.didierstevens.com/programs/eicargen/)
- [ExtractScripts](https://blog.didierstevens.com/programs/extractscripts/)
- [FileGen](https://blog.didierstevens.com/programs/filegen/)
- [FileScanner](https://blog.didierstevens.com/programs/filescanner/)
- [HeapLocker](https://blog.didierstevens.com/programs/heaplocker/)
- [MyJSON Tools](https://blog.didierstevens.com/programs/myjson-tools/)
- [Network Appliance Forensic Toolkit](https://blog.didierstevens.com/programs/network-appliance-forensic-toolkit/)
- [Nokia Time Lapse Photography](https://blog.didierstevens.com/programs/nokia-time-lapse-photography/)
- [oledump.py](https://blog.didierstevens.com/programs/oledump-py/)
- [OllyStepNSearch](https://blog.didierstevens.com/programs/ollystepnsearch/)
- [PDF Tools](https://blog.didierstevens.com/programs/pdf-tools/)
- [Shellcode](https://blog.didierstevens.com/programs/shellcode/)
- [SpiderMonkey](https://blog.didierstevens.com/programs/spidermonkey/)
- [Translate](https://blog.didierstevens.com/programs/translate/)
- [USBVirusScan](https://blog.didierstevens.com/programs/usbvirusscan/)
- [UserAssist](https://blog.didierstevens.com/programs/userassist/)
- [VirusTotal Tools](https://blog.didierstevens.com/programs/virustotal-tools/)
- [XORSearch & XORStrings](https://blog.didierstevens.com/programs/xorsearch/)
- [YARA Rules](https://blog.didierstevens.com/programs/yara-rules/)
- [ZIPEncryptFTP](https://blog.didierstevens.com/programs/zipencryptftp/)

- [Public Drafts](https://blog.didierstevens.com/public-drafts/)

- [Cisco Tricks](https://blog.didierstevens.com/public-drafts/cisco-tricks/)

- [Screencasts & Videos](https://blog.didierstevens.com/screencasts-videos/)

-
-

## Top Posts

- [PDF Tools](https://blog.didierstevens.com/programs/pdf-tools/)
- [Overview of Content Published in October](https://blog.didierstevens.com/2026/11/01/overview-of-content-published-in-october-12/)
- [Quickpost: WT20 Welding Electrodes & Radiation](https://blog.didierstevens.com/2026/10/31/quickpost-wt20-welding-electrodes-radiation/)
- [Didier Stevens Suite](https://blog.didierstevens.com/didier-stevens-suite/)
- ["A Ring Tone That I Cannot Hear" - Twenty Years Later](https://blog.didierstevens.com/2026/10/30/a-ring-tone-that-i-cannot-hear-twenty-years-later/)

-

## Categories

- [.NET](https://blog.didierstevens.com/category/net/)
- [010 Editor](https://blog.didierstevens.com/category/010-editor/)
- [Announcement](https://blog.didierstevens.com/category/announcement/)
- [Arduino](https://blog.didierstevens.com/category/arduino/)
- [Bash Bunny](https://blog.didierstevens.com/category/bash-bunny/)
- [Beta](https://blog.didierstevens.com/category/beta/)
- [bpmtk](https://blog.didierstevens.com/category/bpmtk/)
- [Certification](https://blog.didierstevens.com/category/certification/)
- [Didier Stevens Labs](https://blog.didierstevens.com/category/didier-stevens-labs/)
- [Eee PC](https://blog.didierstevens.com/category/eee-pc/)
- [Elec](https://blog.didierstevens.com/category/elec/)
- [Encryption](https://blog.didierstevens.com/category/encryption/)
- [Entertainment](https://blog.didierstevens.com/category/entertainment/)
- [Fellow Bloggers](https://blog.didierstevens.com/category/fellow-bloggers/)
- [Forensics](https://blog.didierstevens.com/category/forensics/)
- [Hacking](https://blog.didierstevens.com/category/hacking/)
- [Hardware](https://blog.didierstevens.com/category/hardware/)
- [maldoc](https://blog.didierstevens.com/category/maldoc/)
- [Malware](https://blog.didierstevens.com/category/malware/)
- [My Software](https://blog.didierstevens.com/category/my-software/)
- [N800](https://blog.didierstevens.com/category/n800/)
- [Networking](https://blog.didierstevens.com/category/networking/)
- [Nonsense](https://blog.didierstevens.com/category/nonsense/)
- [nslu2](https://blog.didierstevens.com/category/nslu2/)
- [OSX](https://blog.didierstevens.com/category/osx/)
- [PDF](https://blog.didierstevens.com/category/pdf/)
- [Personal](https://blog.didierstevens.com/category/personal/)
- [Physical Security](https://blog.didierstevens.com/category/physical-security/)
- [Poll](https://blog.didierstevens.com/category/poll/)
- [Puzzle](https://blog.didierstevens.com/category/puzzle/)
- [Quickpost](https://blog.didierstevens.com/category/quickpost/)
- [Release](https://blog.didierstevens.com/category/release/)
- [Reverse Engineering](https://blog.didierstevens.com/category/reverse-engineering/)
- [RFID](https://blog.didierstevens.com/category/rfid/)
- [Shellcode](https://blog.didierstevens.com/category/shellcode/)
- [smart card](https://blog.didierstevens.com/category/smart-card/)
- [Spam](https://blog.didierstevens.com/category/spam/)
- [technology](https://blog.didierstevens.com/category/technology/)
- [UltraEdit](https://blog.didierstevens.com/category/ultraedit/)
- [Uncategorized](https://blog.didierstevens.com/category/uncategorized/)
- [Update](https://blog.didierstevens.com/category/update/)
- [video](https://blog.didierstevens.com/category/video/)
- [Vulnerabilities](https://blog.didierstevens.com/category/vulnerabilities/)
- [WiFi](https://blog.didierstevens.com/category/wifi/)
- [Windows 7](https://blog.didierstevens.com/category/windows-7/)
- [Windows 8](https://blog.didierstevens.com/category/windows-8/)
- [Windows Vista](https://blog.didierstevens.com/category/windows-vista/)
- [Wireshark](https://blog.didierstevens.com/category/wireshark/)

-

https://blog.didierstevens.com/feed/

-

## Blog Stats

- 8,038,674 hits

-

## [Twitter @DidierStevens](http://twitter.com/DidierStevens)

[Tweets by DidierStevens](https://twitter.com/DidierStevens)
-

## Archives

- [November 2026](https://blog.didierstevens.com/2026/11/)
- [October 2026](https://blog.didierstevens.com/2026/10/)
- [September 2026](https://blog.didierstevens.com/2026/09/)
- [August 2026](https://blog.didierstevens.com/2026/08/)
- [July 2026](https://blog.didierstevens.com/2026/07/)
- [June 2026](https://blog.didierstevens.com/2026/06/)
- [May 2026](https://blog.didierstevens.com/2026/05/)
- [April 2026](https://blog.didierstevens.com/2026/04/)
- [March 2026](https://blog.didierstevens.com/2026/03/)
- [February 2026](https://blog.didierstevens.com/2026/02/)
- [January 2026](https://blog.didierstevens.com/2026/01/)
- [December 2025](https://blog.didierstevens.com/2025/12/)
- [November 2025](https://blog.didierstevens.com/2025/11/)
- [October 2025](https://blog.didierstevens.com/2025/10/)
- [September 2025](https://blog.didierstevens.com/2025/09/)
- [August 2025](https://blog.didierstevens.com/2025/08/)
- [July 2025](https://blog.didierstevens.com/2025/07/)
- [June 2025](https://blog.didierstevens.com/2025/06/)
- [May 2025](https://blog.didierstevens.com/2025/05/)
- [April 2025](https://blog.didierstevens.com/2025/04/)
- [March 2025](https://blog.didierstevens.com/2025/03/)
- [February 2025](https://blog.didierstevens.com/2025/02/)
- [January 2025](https://blog.didierstevens.com/2025/01/)
- [December 2024](https://blog.didierstevens.com/2024/12/)
- [November 2024](https://blog.didierstevens.com/2024/11/)
- [October 2024](https://blog.didierstevens.com/2024/10/)
- [September 2024](https://blog.didierstevens.com/2024/09/)
- [August 2024](https://blog.didierstevens.com/2024/08/)
- [July 2024](https://blog.didierstevens.com/2024/07/)
- [June 2024](https://blog.didierstevens.com/2024/06/)
- [May 2024](https://blog.didierstevens.com/2024/05/)
- [April 2024](https://blog.didierstevens.com/2024/04/)
- [March 2024](https://blog.didierstevens.com/2024/03/)
- [December 2023](https://blog.didierstevens.com/2023/12/)
- [November 2023](https://blog.didierstevens.com/2023/11/)
- [October 2023](https://blog.didierstevens.com/2023/10/)
- [September 2023](https://blog.didierstevens.com/2023/09/)
- [August 2023](https://blog.didierstevens.com/2023/08/)
- [July 2023](https://blog.didierstevens.com/2023/07/)
- [June 2023](https://blog.didierstevens.com/2023/06/)
- [May 2023](https://blog.didierstevens.com/2023/05/)
- [April 2023](https://blog.didierstevens.com/2023/04/)
- [March 2023](https://blog.didierstevens.com/2023/03/)
- [February 2023](https://blog.didierstevens.com/2023/02/)
- [January 2023](https://blog.didierstevens.com/2023/01/)
- [December 2022](https://blog.didierstevens.com/2022/12/)
- [November 2022](https://blog.didierstevens.com/2022/11/)
- [October 2022](https://blog.didierstevens.com/2022/10/)
- [September 2022](https://blog.didierstevens.com/2022/09/)
- [August 2022](https://blog.didierstevens.com/2022/08/)
- [July 2022](https://blog.didierstevens.com/2022/07/)
- [June 2022](https://blog.didierstevens.com/2022/06/)
- [May 2022](https://blog.didierstevens.com/2022/05/)
- [April 2022](https://blog.didierstevens.com/2022/04/)
- [March 2022](https://blog.didierstevens.com/2022/03/)
- [February 2022](https://blog.didierstevens.com/2022/02/)
- [January 2022](https://blog.didierstevens.com/2022/01/)
- [December 2021](https://blog.didierstevens.com/2021/12/)
- [November 2021](https://blog.didierstevens.com/2021/11/)
- [October 2021](https://blog.didierstevens.com/2021/10/)
- [September 2021](https://blog.didierstevens.com/2021/09/)
- [August 2021](https://blog.didierstevens.com/2021/08/)
- [July 2021](https://blog.didierstevens.com/2021/07/)
- [June 2021](https://blog.didierstevens.com/2021/06/)
- [May 2021](https://blog.didierstevens.com/2021/05/)
- [April 2021](https://blog.didierstevens.com/2021/04/)
- [March 2021](https://blog.didierstevens.com/2021/03/)
- [February 2021](https://blog.didierstevens.com/2021/02/)
- [January 2021](https://blog.didierstevens.com/2021/01/)
- [December 2020](https://blog.didierstevens.com/2020/12/)
- [November 2020](https://blog.didierstevens.com/2020/11/)
- [October 2020](https://blog.didierstevens.com/2020/10/)
- [September 2020](https://blog.didierstevens.com/2020/09/)
- [August 2020](https://blog.didierstevens.com/2020/08/)
- [July 2020](https://blog.didierstevens.com/2020/07/)
- [June 2020](https://blog.didierstevens.com/2020/06/)
- [May 2020](https://blog.didierstevens.com/2020/05/)
- [April 2020](https://blog.didierstevens.com/2020/04/)
- [March 2020](https://blog.didierstevens.com/2020/03/)
- [February 2020](https://blog.didierstevens.com/2020/02/)
- [January 2020](https://blog.didierstevens.com/2020/01/)
- [December 2019](https://blog.didierstevens.com/2019/12/)
- [November 2019](https://blog.didierstevens.com/2019/11/)
- [October 2019](https://blog.didierstevens.com/2019/10/)
- [September 2019](https://blog.didierstevens.com/2019/09/)
- [August 2019](https://blog.didierstevens.com/2019/08/)
- [July 2019](https://blog.didierstevens.com/2019/07/)
- [June 2019](https://blog.didierstevens.com/2019/06/)
- [May 2019](https://blog.didierstevens.com/2019/05/)
- [April 2019](https://blog.didierstevens.com/2019/04/)
- [March 2019](https://blog.didierstevens.com/2019/03/)
- [February 2019](https://blog.didierstevens.com/2019/02/)
- [January 2019](https://blog.didierstevens.com/2019/01/)
- [December 2018](https://blog.didierstevens.com/2018/12/)
- [November 2018](https://blog.didierstevens.com/2018/11/)
- [October 2018](https://blog.didierstevens.com/2018/10/)
- [September 2018](https://blog.didierstevens.com/2018/09/)
- [August 2018](https://blog.didierstevens.com/2018/08/)
- [July 2018](https://blog.didierstevens.com/2018/07/)
- [June 2018](https://blog.didierstevens.com/2018/06/)
- [May 2018](https://blog.didierstevens.com/2018/05/)
- [April 2018](https://blog.didierstevens.com/2018/04/)
- [March 2018](https://blog.didierstevens.com/2018/03/)
- [February 2018](https://blog.didierstevens.com/2018/02/)
- [January 2018](https://blog.didierstevens.com/2018/01/)
- [December 2017](https://blog.didierstevens.com/2017/12/)
- [November 2017](https://blog.didierstevens.com/2017/11/)
- [October 2017](https://blog.didierstevens.com/2017/10/)
- [September 2017](https://blog.didierstevens.com/2017/09/)
- [August 2017](https://blog.didierstevens.com/2017/08/)
- [July 2017](https://blog.didierstevens.com/2017/07/)
- [June 2017](https://blog.didierstevens.com/2017/06/)
- [May 2017](https://blog.didierstevens.com/2017/05/)
- [April 2017](https://blog.didierstevens.com/2017/04/)
- [March 2017](https://blog.didierstevens.com/2017/03/)
- [February 2017](https://blog.didierstevens.com/2017/02/)
- [January 2017](https://blog.didierstevens.com/2017/01/)
- [December 2016](https://blog.didierstevens.com/2016/12/)
- [November 2016](https://blog.didierstevens.com/2016/11/)
- [October 2016](https://blog.didierstevens.com/2016/10/)
- [September 2016](https://blog.didierstevens.com/2016/09/)
- [August 2016](https://blog.didierstevens.com/2016/08/)
- [July 2016](https://blog.didierstevens.com/2016/07/)
- [June 2016](https://blog.didierstevens.com/2016/06/)
- [May 2016](https://blog.didierstevens.com/2016/05/)
- [April 2016](https://blog.didierstevens.com/2016/04/)
- [March 2016](https://blog.didierstevens.com/2016/03/)
- [February 2016](https://blog.didierstevens.com/2016/02/)
- [January 2016](https://blog.didierstevens.com/2016/01/)
- [December 2015](https://blog.didierstevens.com/2015/12/)
- [November 2015](https://blog.didierstevens.com/2015/11/)
- [October 2015](https://blog.didierstevens.com/2015/10/)
- [September 2015](https://blog.didierstevens.com/2015/09/)
- [August 2015](https://blog.didierstevens.com/2015/08/)
- [July 2015](https://blog.didierstevens.com/2015/07/)
- [June 2015](https://blog.didierstevens.com/2015/06/)
- [May 2015](https://blog.didierstevens.com/2015/05/)
- [April 2015](https://blog.didierstevens.com/2015/04/)
- [March 2015](https://blog.didierstevens.com/2015/03/)
- [February 2015](https://blog.didierstevens.com/2015/02/)
- [January 2015](https://blog.didierstevens.com/2015/01/)
- [December 2014](https://blog.didierstevens.com/2014/12/)
- [November 2014](https://blog.didierstevens.com/2014/11/)
- [October 2014](https://blog.didierstevens.com/2014/10/)
- [September 2014](https://blog.didierstevens.com/2014/09/)
- [August 2014](https://blog.didierstevens.com/2014/08/)
- [July 2014](https://blog.didierstevens.com/2014/07/)
- [June 2014](https://blog.didierstevens.com/2014/06/)
- [May 2014](https://blog.didierstevens.com/2014/05/)
- [April 2014](https://blog.didierstevens.com/2014/04/)
- [March 2014](https://blog.didierstevens.com/2014/03/)
- [February 2014](https://blog.didierstevens.com/2014/02/)
- [January 2014](https://blog.didierstevens.com/2014/01/)
- [December 2013](https://blog.didierstevens.com/2013/12/)
- [November 2013](https://blog.didierstevens.com/2013/11/)
- [October 2013](https://blog.didierstevens.com/2013/10/)
- [September 2013](https://blog.didierstevens.com/2013/09/)
- [August 2013](https://blog.didierstevens.com/2013/08/)
- [July 2013](https://blog.didierstevens.com/2013/07/)
- [June 2013](https://blog.didierstevens.com/2013/06/)
- [May 2013](https://blog.didierstevens.com/2013/05/)
- [April 2013](https://blog.didierstevens.com/2013/04/)
- [March 2013](https://blog.didierstevens.com/2013/03/)
- [February 2013](https://blog.didierstevens.com/2013/02/)
- [January 2013](https://blog.didierstevens.com/2013/01/)
- [December 2012](https://blog.didierstevens.com/2012/12/)
- [November 2012](https://blog.didierstevens.com/2012/11/)
- [October 2012](https://blog.didierstevens.com/2012/10/)
- [September 2012](https://blog.didierstevens.com/2012/09/)
- [August 2012](https://blog.didierstevens.com/2012/08/)
- [July 2012](https://blog.didierstevens.com/2012/07/)
- [June 2012](https://blog.didierstevens.com/2012/06/)
- [May 2012](https://blog.didierstevens.com/2012/05/)
- [April 2012](https://blog.didierstevens.com/2012/04/)
- [March 2012](https://blog.didierstevens.com/2012/03/)
- [February 2012](https://blog.didierstevens.com/2012/02/)
- [January 2012](https://blog.didierstevens.com/2012/01/)
- [December 2011](https://blog.didierstevens.com/2011/12/)
- [November 2011](https://blog.didierstevens.com/2011/11/)
- [October 2011](https://blog.didierstevens.com/2011/10/)
- [September 2011](https://blog.didierstevens.com/2011/09/)
- [August 2011](https://blog.didierstevens.com/2011/08/)
- [July 2011](https://blog.didierstevens.com/2011/07/)
- [June 2011](https://blog.didierstevens.com/2011/06/)
- [May 2011](https://blog.didierstevens.com/2011/05/)
- [April 2011](https://blog.didierstevens.com/2011/04/)
- [March 2011](https://blog.didierstevens.com/2011/03/)
- [February 2011](https://blog.didierstevens.com/2011/02/)
- [January 2011](https://blog.didierstevens.com/2011/01/)
- [December 2010](https://blog.didierstevens.com/2010/12/)
- [November 2010](https://blog.didierstevens.com/2010/11/)
- [October 2010](https://blog.didierstevens.com/2010/10/)
- [September 2010](https://blog.didierstevens.com/2010/09/)
- [August 2010](https://blog.didierstevens.com/2010/08/)
- [July 2010](https://blog.didierstevens.com/2010/07/)
- [June 2010](https://blog.didierstevens.com/2010/06/)
- [May 2010](https://blog.didierstevens.com/2010/05/)
- [April 2010](https://blog.didierstevens.com/2010/04/)
- [March 2010](https://blog.didierstevens.com/2010/03/)
- [February 2010](https://blog.didierstevens.com/2010/02/)
- [January 2010](https://blog.didierstevens.com/2010/01/)
- [December 2009](https://blog.didierstevens.com/2009/12/)
- [November 2009](https://blog.didierstevens.com/2009/11/)
- [October 2009](https://blog.didierstevens.com/2009/10/)
- [September 2009](https://blog.didierstevens.com/2009/09/)
- [August 2009](https://blog.didierstevens.com/2009/08/)
- [July 2009](https://blog.didierstevens.com/2009/07/)
- [June 2009](https://blog.didierstevens.com/2009/06/)
- [May 2009](https://blog.didierstevens.com/2009/05/)
- [April 2009](https://blog.didierstevens.com/2009/04/)
- [March 2009](https://blog.didierstevens.com/2009/03/)
- [February 2009](https://blog.didierstevens.com/2009/02/)
- [January 2009](https://blog.didierstevens.com/2009/01/)
- [December 2008](https://blog.didierstevens.com/2008/12/)
- [November 2008](https://blog.didierstevens.com/2008/11/)
- [October 2008](https://blog.didierstevens.com/2008/10/)
- [September 2008](https://blog.didierstevens.com/2008/09/)
- [August 2008](https://blog.didierstevens.com/2008/08/)
- [July 2008](https://blog.didierstevens.com/2008/07/)
- [June 2008](https://blog.didierstevens.com/2008/06/)
- [May 2008](https://blog.didierstevens.com/2008/05/)
- [April 2008](https://blog.didierstevens.com/2008/04/)
- [March 2008](https://blog.didierstevens.com/2008/03/)
- [February 2008](https://blog.didierstevens.com/2008/02/)
- [January 2008](https://blog.didierstevens.com/2008/01/)
- [December 2007](https://blog.didierstevens.com/2007/12/)
- [November 2007](https://blog.didierstevens.com/2007/11/)
- [October 2007](https://blog.didierstevens.com/2007/10/)
- [September 2007](https://blog.didierstevens.com/2007/09/)
- [August 2007](https://blog.didierstevens.com/2007/08/)
- [July 2007](https://blog.didierstevens.com/2007/07/)
- [June 2007](https://blog.didierstevens.com/2007/06/)
- [May 2007](https://blog.didierstevens.com/2007/05/)
- [April 2007](https://blog.didierstevens.com/2007/04/)
- [March 2007](https://blog.didierstevens.com/2007/03/)
- [February 2007](https://blog.didierstevens.com/2007/02/)
- [January 2007](https://blog.didierstevens.com/2007/01/)
- [December 2006](https://blog.didierstevens.com/2006/12/)
- [November 2006](https://blog.didierstevens.com/2006/11/)
- [October 2006](https://blog.didierstevens.com/2006/10/)
- [September 2006](https://blog.didierstevens.com/2006/09/)
- [August 2006](https://blog.didierstevens.com/2006/08/)
- [July 2006](https://blog.didierstevens.com/2006/07/)
- [June 2006](https://blog.didierstevens.com/2006/06/)

-

November 2026

| M  | T  | W  | T  | F  | S  | S

|   | [1](https://blog.didierstevens.com/2026/11/01/)

| 2 | 3 | 4 | 5 | 6 | 7 | 8

| 9 | 10 | 11 | 12 | 13 | 14 | 15

| 16 | 17 | 18 | 19 | 20 | 21 | 22

| 23 | 24 | 25 | 26 | 27 | 28 | 29

| 30  |

[Blog at WordPress.com.](https://wordpress.com/?ref=footer_blog)

-  Subscribe Subscribed

[Didier Stevens](https://blog.didierstevens.com)

Have a WordPress.com account? [Log in now.](https://wordpress.com/log-in?redirect_to=https%3A%2F%2Fr-login.wordpress.com%2Fremote-login.php%3Faction%3Dlink%26back%3Dhttps%253A%252F%252Fblog.didierstevens.com%252Fprograms%252Fpdf-tools%252F)

-

[Didier Stevens](https://blog.didierstevens.com)

[Copy shortlink](https://wp.me/P16Sp-dv)[View post in Reader](https://wordpress.com/reader/blogs/264765/posts/837)

[Manage subscriptions](https://subscribe.wordpress.com/)[Sign up](https://wordpress.com/start/)[Log in](https://wordpress.com/log-in?redirect_to=https%3A%2F%2Fr-login.wordpress.com%2Fremote-login.php%3Faction%3Dlink%26back%3Dhttps%253A%252F%252Fblog.didierstevens.com%252Fprograms%252Fpdf-tools%252F)

[Report this content](https://wordpress.com/abuse/?report_url=https://blog.didierstevens.com/programs/pdf-tools/)

Collapse this bar

https://blog.didierstevens.com/programs/pdf-tools/# https://blog.didierstevens.com/programs/pdf-tools/#

Loading Comments...

https://blog.didierstevens.com/programs/pdf-tools/#
