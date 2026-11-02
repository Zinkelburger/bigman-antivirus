---
url: "https://www.phishtank.com/developer_info.php"
title: "PhishTank > Developer Information"
captured: "2026-11-02T18:57:24Z"
via: "live"
sha256: "c73cdfe02c42e8fd3bdd7c38f589dffdb38da2cffd8f7901cb1f2fb7c76ab377"
wayback: "https://web.archive.org/web/20251221040137/https://www.phishtank.com/developer_info.php"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# PhishTank > Developer Information

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.phishtank.com/developer_info.php>
> Snapshot: <https://web.archive.org/web/20251221040137/https://www.phishtank.com/developer_info.php>

PhishTank is operated by [Cisco Talos Intelligence Group](https://talosintelligence.com/).

# https://www.phishtank.com/

[Register](https://www.phishtank.com/register.php) | [Forgot Password](https://www.phishtank.com/forgot_password.php)

- [Home](https://www.phishtank.com/index.php)
- [Add A Phish](https://www.phishtank.com/add_web_phish.php)
- [Verify A Phish](https://www.phishtank.com/verify_a_phish.php)
- [Phish Search](https://www.phishtank.com/phish_archive.php)
- [Stats](https://www.phishtank.com/stats.php)
- [FAQ](https://www.phishtank.com/faq.php)
- [Developers](https://www.phishtank.com/developer_info.php)
- [Mailing Lists](https://www.phishtank.com/mailing_lists.php)
- [My Account](https://www.phishtank.com/account.php)

- [Developer Information](https://www.phishtank.com/developer_info.php)
- [API Information](https://www.phishtank.com/api_info.php)
- [Manage Applications](https://www.phishtank.com/api_register.php)

## Developer Information

### User Agent String

We require that you use a descriptive User Agent string in your application to identify the application. If your User Agent is blank or generic, you may recieve an increased number of rate limited requests or be redirected to additional security checks. The recommended format is:

Header Parameters

| Name  | Value

| User-Agent  | Descriptive user agent string, e.g.

```
phishtank/[username]
```

### Get the Database

If you'll be doing lots of lookups, the best option is to take advantage of our downloadable databases. Available in multiple formats and updated hourly, these make it easy to have fast and up to date phishing detection built into your application.

The data is available in a variety of formats to make it as easy as possible for you to implement. We're always open to suggestions for additional formats we could provide, so if you have any thoughts, please join our [developers list](https://www.phishtank.com/mailing_lists.php) and let us know!

If you do intend to fetch these files automatically, please [register for an application key](https://www.phishtank.com/api_register.php) and see below for instructions on how to use it to request files. Without this key, you will be limited to a few downloads per day.

### Format Options

| XML

| https://data.phishtank.com/data/online-valid.xml

| https://data.phishtank.com/data/online-valid.xml.gz

| https://data.phishtank.com/data/online-valid.xml.bz2

| CSV

| https://data.phishtank.com/data/online-valid.csv

| https://data.phishtank.com/data/online-valid.csv.gz

| https://data.phishtank.com/data/online-valid.csv.bz2

| Serialized PHP

| https://data.phishtank.com/data/online-valid.php_serialized

| https://data.phishtank.com/data/online-valid.php_serialized.gz

| https://data.phishtank.com/data/online-valid.php_serialized.bz2

| JSON

| https://data.phishtank.com/data/online-valid.json

| https://data.phishtank.com/data/online-valid.json.gz

| https://data.phishtank.com/data/online-valid.json.bz2

### Formats Definitions

XML

```

<output>
    <meta>
        <total_entries>1234</total_entries>
    </meta>
    <entries>
        <entry>
            <url><![CDATA[https://www.example.com/]]></url>
            <phish_id>123456</phish_id>
            <phish_detail_url>https://www.phishtank.com/phish_detail.php?phish_id=123456</phish_detail_url>
            <details>
                <detail>
                    <ip_address>1.2.3.4</ip_address>
                    <cidr_block>1.2.3.0/24</ip_address>
                    <announcing_network>1234</announcing_network>
                    <rir>arin</rir>
                    <detail_time>2009-06-20T15:37:31+00:00</detail_time>
                </detail>
            </details>
            <submission>
                <submission_time>2009-06-19T15:15:47+00:00</submission_time>
            </submission>
            <verification>
                <verified>yes</verified>
                <verification_time>2009-06-19T15:37:31+00:00</verification_time>
            </verification>
            <status>
                <online>yes</online>
            </status>
            <target>1st National Example Bank</target>
        </entry>
        ...
    </entries>
</output>

```

CSV

```

phish_id,url,phish_detail_url,submission_time,verified,verification_time,online,target
123456,https://www.example.com/,https://www.phishtank.com/phish_detail.php?phish_id=123456,2009-06-19T15:15:47+00:00,yes,2009-06-19T15:37:31+00:00,yes,1st National Example Bank

```

PHP/JSON

```

array(
      array(
            'phish_id' => 123456,
            'url' => 'https://www.example.com/',
            'phish_detail_url' => 'https://www.phishtank.com/phish_detail.php?phish_id=123456',
            'submission_time' => '2009-06-19T15:15:47+00:00',
            'verified' => 'yes',
            'verification_time' => '2009-06-19T15:37:31+00:00',
            'online' => 'yes',
            'target' => '1st National Example Bank',
            'details' => array(
                               array(
                                     'ip_address' => '1.2.3.4',
                                     'cidr_block' => '1.2.3.0/24',
                                     'announcing_network' => '1234',
                                     'rir' => 'arin',
                                     'detail_time' => '2006-10-01T02:30:54+00:00'
                                    )
                               )
           )
)

```

### Column Definitions

phish_id The ID number by which Phishtank refers to a phish submission. All data in PhishTank is tied to this ID. This will always be a positive integer.

phish_detail_url PhishTank detail url for the phish, where you can view data about the phish, including a screenshot and the community votes.

url The phish URL. This is always a string, and in the XML feeds may be a CDATA block.

submission_time The date and time at which this phish was reported to Phishtank. This is an ISO 8601 formatted date.

verified Whether or not this phish has been verified by our community. In these data files, this will always be the string 'yes' since we only supply verified phishes in these files.

verification_time The date and time at which the phish was verified as valid by our community. This is an ISO 8601 formatted date.

online Whether or not the phish is online and operational. In these data files, this will always be the string 'yes' since we only supply online phishes in these files.

target The name of the company or brand the phish is impersonating, if it's known.

### Using your application key

Once you have registered for an application key, make your request as you normally would, but format the URL this way:

```

https://data.phishtank.com/data/<your app key>/online-valid.json.bz2

```

### Checking for updates

If you're passing an application key with your request, you may make unlimited HTTP HEAD requests to check the ETag header. These requests won't count against any rate limiting. Note that the files are still only updated every hour on the hour, so there's no need to check every minute.

[Friends of PhishTank](https://www.phishtank.com/friends.php) | [Terms of Use](https://www.cisco.com/c/en/us/about/legal/cloud-and-software/end_user_license_agreement.html) | [Privacy](http://www.cisco.com/c/en/us/about/legal/privacy-full.html) | [Contact](https://www.phishtank.com/contact.php)
PhishTank is operated by [Cisco Talos Intelligence Group](https://talosintelligence.com/) (Talos). Learn more about [PhishTank](https://www.phishtank.com/about.php) or [Talos](https://talosintelligence.com/about).
