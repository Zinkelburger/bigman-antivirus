---
url: "https://urlhaus.abuse.ch/api/"
title: "URLhaus | Community API"
captured: "2026-11-02T18:48:44Z"
via: "live"
sha256: "d66248b4d9b35b8253e82d7aa0fc51eca660605f3898324f585c8d52c271f7dd"
cited_in:
  - "REVIEW.md"
---

# URLhaus | Community API

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://urlhaus.abuse.ch/api/>

# URLhaus Community API

####  Enhanced abuse.ch API for commercial usage

You are viewing the documentation for the URLhaus Community API. This API is available free of charge under the [fair use principles](https://abuse.ch/terms-of-use/#principles). Use of the API by companies, networks, or individuals with commercial or for- profit needs may require a paid subscription for the enhanced abuse.ch commercial API. The commercial API comes with additional benefits, such as better reliability and statibility as well as a unified query language across all abuse.ch APIs. Further information can be found below.

- [abuse.ch enhanced commercial API](https://www.spamhaus.com/data-access/abusech-api/)
- [abuse.ch Fair Use Principles](https://abuse.ch/terms-of-use/#principles)

URLhaus offers a community API to both, receive (download) and submit malware URLs from the URLhaus database. The API is documented below.

[Obtain an Auth-Key (Required)](https://urlhaus.abuse.ch/api/#auth_key) [Database dump (CSV)](https://urlhaus.abuse.ch/api/#csv) [Daily MISP Events](https://urlhaus.abuse.ch/api/#misp) [DNS Response Policy Zone (RPZ)](https://urlhaus.abuse.ch/api/#rpz) [Snort / Suricata IDS rulset](https://urlhaus.abuse.ch/api/#ids) [hostfile](https://urlhaus.abuse.ch/api/#hostfile) [Plain-Text URL List (URLs only)](https://urlhaus.abuse.ch/api/#plain-text) [Collected Payloads (CSV)](https://urlhaus.abuse.ch/api/#payloads) [ClamAV signatures](https://urlhaus.abuse.ch/api/#clamav) [Submit malware URLs](https://urlhaus.abuse.ch/api/#submit) [Submission Policy](https://urlhaus.abuse.ch/api/#policy) [Your Account](https://urlhaus.abuse.ch/api/#account) [API for automated bulk queries](https://urlhaus.abuse.ch/api/#automation)

## Obtain an Auth-Key (Required)

In order to interact with the URLhaus API, you need to obtain an Auth-Key first. If you don't have one you can get one for free here:

- [abuse.ch Authentication Portal](https://auth.abuse.ch/)

Whenever you try to download a dataset or file from below, you must include your Auth-Key in the URL. Example curl command:

curl -i "https://urlhaus-api.abuse.ch/v2/files/exports/YOUR-AUTH-KEY-HERE/recent.csv"

### Database dumps

We provide various URLhaus database dumps. Depending on your need, you can choose between a full dump (containing URLs that are either actively distributing malware or that have been added to URLhaus within the past 90 days), a dump of active malware distribution sites only or a dump of any URL added to URLhaus within the past 30 days. You can choose between CSV and JSON format. The dumps are generated every 5 minutes. Please do not fetch it more often than every 5 minutes.

Full database dump, contains URLs that are either actively distributing malware or that have been added to URLhaus within the past 90 days.

#### Login required

In order to view this documentation, you need to [log in](https://auth.abuse.ch/) and create an Auth-Key.

### Daily MISP Events

If you use [MISP](https://www.misp-project.org/) you can ingest URLhaus as a daily feed. To do so, you simply have to click on the button below to copy your personal MISP Feed URL to your clipboard. Afterwards, navigate to Sync Actions -> Feeds and update or create the URLhaus feed with your personal MISP Feed URL from your clipboard.

#### Login required

In order to view this documentation, you need to [log in](https://auth.abuse.ch/) and create an Auth-Key.

### DNS Response Policy Zone (RPZ)

By using an DNS Reponse Policy Zone (RPZ), also known as DNS firewall, you can block the resolution of certain domain names on your DNS resolver. URLhaus extracts the hostnames (FQDN, e.g. www.example.com) from malware URLs and offers them in an RPZ dataset. More information about DNS RPZ can be found on [dnsrpz.info](https://dnsrpz.info/).

To reduce the amount of false positives, URLhaus RPZ does only include domain names associated with malware URLs that are either active (malware sites that currently serve a payload) or that have been added to URLhaus in the past 48 hours. In addition to that, hostnames that belong to a domain name listed in [Tranco Top 1M](https://tranco-list.eu/) are excluded from the RPZ dataset.

The RPZ zone file gets generated every 5 minutes. To achieve the best protection, we recommend to fetch it every 5 minutes.

#### Login required

In order to view this documentation, you need to [log in](https://auth.abuse.ch/) and create an Auth-Key.

### Snort / Suricata IDS Ruleset

If you are using a network intrusion detection and preventation systems (IDS / IPS) like [Snort](https://www.snort.org/) or [Suricata](https://suricata-ids.org/) (or any other IDS that supports the Snort / Suricata Ruleset format), you may use the URLhaus IDS Ruleset to identify network traffic towards known malware URLs. The ruleset will only trigger on the extact URL in a HTTP stream (HTTP GET request).

Due to the amount of malware URLs tracked by URLhaus, the Snort / Suricata IDS ruleset do only include malware URLs that are either active (malware sites that currently serve a payload) or that have been added to URLhaus in the past 10 days. If you would like to watch out for offline malware URLs too, you should probably not use Snort or Suricata IDS.

The IDS ruleset gets generated every 5 minutes. To achieve the best protection, we recommend to fetch it every 5 minutes.

#### Login required

In order to view this documentation, you need to [log in](https://auth.abuse.ch/) and create an Auth-Key.

### host file (domains only)

Some commercial and open source security software (such as [Pi-hole](https://pi-hole.net/)) can block access to hostnames based on the host file format. For this purpose, URLhaus offers a list of hostnames associated with malware URLs below.

To reduce the amount of false positives, the URLhaus host file does only include hostnames associated with malware URLs that are either active (malware sites that currently serve a payload) or that have been added to URLhaus in the past 48 hours. In addition to that, hostnames that belong to a domain name listed in [Tranco Top 1M](https://tranco-list.eu/) are excluded from the RPZ dataset.

#### Login required

In order to view this documentation, you need to [log in](https://auth.abuse.ch/) and create an Auth-Key.

### Plain-Text URL List (URLs only)

The Plain-Text URL List is a dump of all malware URLs known to URLhaus. It does not contain anything else than one URL per line, which is useful if you want to use the URLhaus dataset as an IOC (Indicator Of Compromise). You can match them against certain log files of your security permieter, for example web proxy logs. In additional, you may also use it as a blocklist with a low false positive rate.

#### Login required

In order to view this documentation, you need to [log in](https://auth.abuse.ch/) and create an Auth-Key.

### Collected Payloads (CSV)

URLhaus regularely checks the content served by malicious URLs that are known to URLhaus. This CSV contains all payloads collected by URLhaus, identified by a hash (MD5 / SHA256 hash). Please consider that not all payloads are malicious. As a matter of fact, a URL can e.g. serve any content once it has been cleaned up.

#### Login required

In order to view this documentation, you need to [log in](https://auth.abuse.ch/) and create an Auth-Key.

### URLhaus ClamAV signatures

URLhaus generates a [ClamAV](https://www.clamav.net/) signature database which gets updated once per minute. This allows you to add almost real time detection of malware distribution sites (e.g. such ones being used by Emotet/Heodo) on your email gateway / spam filter. As the signature file only contains active malware distribution sites or such that have been added to URLhaus in past 48 hours, the false positive rate should be very low. You can download the signature file here:

#### Login required

In order to view this documentation, you need to [log in](https://auth.abuse.ch/) and create an Auth-Key.

## Submit malware URLs

Collecting and maintaing a list of malware URLs means a lot of work. I therefore appreciate any submissions from 3rd parties like security researchers, SOC analysts or vendors to URLhaus. If you would like to submit malware URLs to URLhaus, there are two ways to do so:

- Manual submissions through the URLhaus [web interface](https://urlhaus.abuse.ch/browse/) (note: you need to authenticate yourself with your [abuse.ch account](https://auth.abuse.ch/))
- Automated / bulk submissions through the URLhaus API (as documented below)

#### Note

For security reasons, and as I want to keep the URLhaus database clean from false positives, anonymous submissions are not accepted. If you want to submit a malware URL to the URLhaus database, you must authenticate yourself with your [abuse.ch account](https://auth.abuse.ch/). But don't worry, if you don't want to be associated with a submission, you can hide your abuse.ch handle by setting the option / flag anonymous report. While URLhaus will know where the submission came from, the source of the submission will not be shared with anyone.

### Submissions via web interface

You can use the [web interface](https://urlhaus.abuse.ch/browse/) to submit a malware URL to URLhaus. In order to do so, you will need to login with your [abuse.ch account](https://auth.abuse.ch/). Please consider that your abuse.ch handle will be public visible unless stated otherwise (by selecting the option anonymous report).

### Submissions via API

There is a web API you can use for automated or bulk submissions. You can call the API through Python or your prefered scripting language.

To submit a malware URL to URLhaus through bulk API, you must send a POST request to https://urlhaus.abuse.ch/api/. The post request must contain the following fields (JSON):

| anonymous | If set to 1, your submission will be anonymous (required)

| submission | List of URLs (required)

| URL | URL you want to submit (required)

| Threat | Threat (required, must be malware_download)

| Tags | Tag. Allowed characters: [A-Za-z0-9.- ] (optional)

In addition, your POST request must contain the Auth-Key field, containing your personal Auth-Key. If you don't have an Auth-Key yet, you can get one at the [abuse.ch Authentication Portal](https://auth.abuse.ch/).

If you want to send malware URLs to URLhaus using python, you can find a sample script here:

- https://github.com/abusech/URLhaus/blob/master/submit_url.py

More sample python scripts showing how to interact with the [URLhaus bulk API](https://urlhaus-api.abuse.ch/) are available here:

- https://github.com/abusech/URLhaus/

### Submission Policy

URLhaus is currently only collecting websites (URLs) that are directly being used to distribute malware. Please note that any other submissions will be ignored / deleted from URLhaus.
Before you start to submit URLs to URLhaus, I encourage you to read the following submission policy:

- Active malware distribution sites: Please ensure that you only submit active (online) malware distribution sites that are currently serving a payload (please see the definition of payload below). Malware URLs that are down and / or have already been cleaned should not be submitted to URLhaus.
- Payload: A payload can be any file (executable, script, document) that harms or infect a computer once downloaded and executed. Some examples: Windows executables, Office documents, PowerShell scripts, Bash scripts, hta, ELF.
- URL shorteners: Any URL submitted to URLhaus must host an active malware payload. Redirection sites or URL shorteners (e.g. bit.ly) that are just used for redirection and that are not hosting any payload should not be submitted to URLhaus.
- Adware is not Malware: Unlike Malware, most common Adware (aka Potential Unwanted Programs - PUPs) do need some sort of user interaction. In many cases, they also come with a licences agreement that the user has to accept and that is more or less transparent with regads to what the Adware does. Please refrain from submitting URLs to URLhaus that are distributing Adware.
- Phishing and Phishing kits: Phishing sites or websites that are hosting a phishing kit should not be submitted to URLhaus (Phishing is not Malware). If you would like to report phishing websites, you may want to report them to [AWPG](https://www.antiphishing.org/report-phishing/), [PhishTank](https://www.phishtank.com/) or [Netcraft](https://toolbar.netcraft.com/report_url).
- Automated submissions: Should you decide to make automated submissions to the URLhaus API, please ensure that your script has implemented proper URL verification. Please also ensure that you do not submit any private IP addresses ([RFC1597](https://tools.ietf.org/html/rfc1597)) or any IP addresses that are used for any other special purpose ([RFC6890](https://tools.ietf.org/html/rfc6890)).
- Exploit kits: Websites that are hosting an exploit kit should not be submitted to URLhaus unless the submitted URL serves the final payload.
- Geo IP filter: Some malware distribution sites may use a Geo IP filter to restrict the download of the payload to a specific country. You can tell URLhaus about this restriction by using the tag [geofenced](https://urlhaus.abuse.ch/browse/tag/geofenced/) and a tag with the [three letter NATO country code](https://en.wikipedia.org/wiki/List_of_NATO_country_codes) (e.g. [GBR](https://urlhaus.abuse.ch/browse/tag/GBR/) for Great Britain). URLhaus will then try to fetch the payload using an IP address from the specified Geo location.
- Duplicates: To avoid duplicates and ensure that the malware sites tracked by URLhaus can be properly used as IOC, please make sure that you submit URLs to URLhaus as you see them on the wire, returning HTTP 200 OK. For example, http://evil.tld/91BOYI/oamo/US would become https://evil.tld/91BOYI/oamo/US/ (note the tailing /) and https://evil.tld?thisisbad=1 would become https://evil.tld/?thisisbad=1 (note / after .tld).

Note: Should you repeatedly violate the submission policy documented above, your account may get banned from URLhaus.

## Your Account

In order to view your account status, submit malware URLs or obtain an Auth-Key, you must login with [your abuse.ch account](https://urlhaus.abuse.ch/api/login/)

## API for automated bulk queries

If you would like to query URLhaus for e.g. an URL or malware sample in an automated way, there is a dedicated API available for this purpose. It also allows you to download a specific malware sample or daily batches:

- [URLhaus API for automated bulk queries](https://urlhaus-api.abuse.ch/)
