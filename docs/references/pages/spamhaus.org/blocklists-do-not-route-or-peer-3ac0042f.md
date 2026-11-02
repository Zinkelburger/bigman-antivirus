---
url: "https://www.spamhaus.org/blocklists/do-not-route-or-peer/"
title: "Don't Route Or Peer Lists (DROP) | Use with firewalls & BGP"
site: "The Spamhaus Project"
captured: "2026-11-02T19:00:04Z"
via: "live"
sha256: "e5fdb8be366d668fe1cfd437292b3fab9ae2922745f1d31864b5f15da92c4f74"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Don't Route Or Peer Lists (DROP) | Use with firewalls & BGP

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.spamhaus.org/blocklists/do-not-route-or-peer/>

https://www.spamhaus.org/

Resource Hub

[All Resources](https://www.spamhaus.org/resource-hub/)

[DNS Blocklists](https://www.spamhaus.org/resource-hub/dnsbl/)

[IP & domain reputation](https://www.spamhaus.org/resource-hub/ip-domain-reputation/)

[Malware](https://www.spamhaus.org/resource-hub/malware/)

[Service providers](https://www.spamhaus.org/resource-hub/service-providers/)

[Threat intelligence](https://www.spamhaus.org/resource-hub/threat-intelligence/)

More Resources

[Events](https://www.spamhaus.org/events/)

[FAQs](https://www.spamhaus.org/faqs/)

[Glossary](https://www.spamhaus.org/glossary/)

[Restrict Port 25](https://www.spamhaus.org/restrict-port-25/)

[Sender Guidelines](https://www.spamhaus.org/sender-guidelines/)

About the Research

[About Spamhaus Research](https://www.spamhaus.org/about-the-research/)

[Domain Reputation](https://www.spamhaus.org/domain-reputation/)

[IP Reputation](https://www.spamhaus.org/ip-reputation/)

Reputation Statistics

Overview

Countries

Country code top-level domains (ccTLD)

General top-level domains (gTLD)

Networks

Registrars

[Who Is Spamhaus?](https://www.spamhaus.org/who-is-spamhaus/)

[History](https://www.spamhaus.org/history/)

[Media Centre](https://www.spamhaus.org/media-centre/)

[Affiliates](https://www.spamhaus.org/affiliates/)

[Community Home](https://submit.spamhaus.org)

[Guest Submission](https://submit.spamhaus.org/submit)

[Login](https://auth.spamhaus.org)

Blocklists

[Spamhaus' Blocklists](https://www.spamhaus.org/blocklists/)

[Combined Spam Sources (CSS)

Email Spam IPs](https://www.spamhaus.org/blocklists/combined-spam-sources/)

[Domain Blocklist (DBL)

Low Reputation Domains](https://www.spamhaus.org/blocklists/domain-blocklist/)

[Exploits Blocklist (XBL)

Compromised IPs](https://www.spamhaus.org/blocklists/exploits-blocklist/)

[Policy Blocklist (PBL)

Non-mail Emitting IPs](https://www.spamhaus.org/blocklists/policy-blocklist/)

[Spamhaus Blocklist (SBL)

Malicious Network Ranges](https://www.spamhaus.org/blocklists/spamhaus-blocklist/)

[ZEN Blocklist](https://www.spamhaus.org/blocklists/zen-blocklist/)

[DNSBL Fair Use Policy](https://www.spamhaus.org/blocklists/dnsbl-fair-use-policy/)

Commercial Datasets

[Commercial-Grade Reputation Data](https://www.spamhaus.org/blocklists/commercial/)

[Blocklists via DNS Query | Commercial

(Data Query Service)](https://www.spamhaus.com/data-access/real-time-dns-blocklists/)

[Blocklists via DNS Query | Free

(Data Query Service)](https://www.spamhaus.com/data-access/free-data-query-service/)

[DNS Response Policy Zones | Commercial

(DNS Firewall)](https://www.spamhaus.com/resource-center/response-policy-zones-rp-zs-via-spamhaus-dns-firewall/)

[DNS Response Policy Zones | Free

(DNS Firewall)](https://www.spamhaus.com/data-access/free-dns-firewall/)

[Intelligence API | Commercial

(IP & Domain Intelligence)](https://www.spamhaus.com/data-access/intelligence-api/)

[Intelligence API | Free

(IP & Domain Intelligence)](https://www.spamhaus.com/data-access/intelligence-api-developer-license/)

Network Protection

[Datasets for network protection](https://www.spamhaus.org/blocklists/network-protection/)

[Botnet Controller List (BCL)

Botnet C2s](https://www.spamhaus.org/blocklists/botnet-controller-list/)

[Do Not Route or Peer (DROP)

Highly Malicious Networks](https://www.spamhaus.org/blocklists/do-not-route-or-peer/)

[DROP Fair Use Policy](https://www.spamhaus.org/blocklists/drop-fair-use-policy/)

Tools & Portals

[Blocklist Tester](https://blt.spamhaus.com)

[CERT Portal](https://portal.spamhaus.org/cert/login/)

[DNS Checker](https://www.spamhaus.org/sender-guidelines/#dns-checker)

[ISP Portal](https://portal.spamhaus.org/isp/start/)

[Reputation Checker](https://check.spamhaus.org/)

# Don't Route Or Peer Lists (DROP)

## Highly Malicious Networks

## About the Data

Don't Route Or Peer (DROP) lists the worst of the worst IP traffic. It is an advisory “drop all traffic”, containing IP ranges which are so dangerous to internet users that Spamhaus provides access to anyone who wants to add this layer of protection, free of charge.

### Policy Statement

The Spamhaus DROP lists consist of netblocks that are leased or stolen by professional spam or cyber-crime operations, and used for dissemination of malware, trojan downloaders, botnet controllers, or other kinds of malicious activity.

The DROP lists are a subset of the [Spamhaus Blocklist (SBL)](https://www.spamhaus.org/blocklists/spamhaus-blocklist/), designed for a total protection from all the activity involving the listed networks over all the Internet protocols. This also specifically includes traffic directed to these networks, such as access to web sites hosted there. The DROP lists are also designed for use by Tier-1 and backbone providers in firewalls and routing equipment to filter out the malicious traffic from listed netblocks.

Networks are inserted in the DROP lists only after dedicated investigators and forensics specialists have gathered evidence that they are controlled by cybercrime groups or by "bulletproof" hosters that either ignore abuse reports or, more frequently, move abusive customers to different IPs to evade targeted listings. With IPv4 depletion, assignments of netblocks to customers are now typically done by IPv4 brokers and are much more dynamic than in the past. Furthermore, ASNs are rotated very rapidly together with company names by malicious actors. For these reasons, the DROP lists change on a daily basis tracking the continuous and relentless movement of rogue networks trying to avoid detection.

### Benefits of this data

Protect from activity directly originating from rogue networks, such as spam campaigns, encryption via ransomware, DNS-hijacking and exploit attempts, authentication attacks to discover working access credentials, harvesting, DDoS attacks. Also, gain automatic protection to immediately stop infected devices from communicating with adversaries with "bulletproof hosting" on listed networks. Users are often unaware of these background communications, so this infrastructure-level protection should be an important part of your overall security stack.

IP address space under the control of any legitimate network will never be listed, and false positives are extremely low, given the high confidence nature of this dataset.

### How it works

The free DROP datasets are provided in JSON format to be parsed out and implemented on nearly any kind of device or software that is capable of processing IP networks for making a decision e.g., network gateways, firewalls, web-proxies, DNS resolvers etc.

The DROP lists available are:

- DROP - https://www.spamhaus.org/drop/drop_v4.json
- DROPv6 - https://www.spamhaus.org/drop/drop_v6.json
- ASN-DROP - https://www.spamhaus.org/drop/asndrop.json

For long-term users of the DROP files in text format, we recommend you update your configuration with the above JSON files as soon as your cycles allow. If you require continued long-term use of a text file, [the jq command](https://www.spamhaus.org/faqs/do-not-route-or-peer-drop/#how-do-i-convert-the-json-drop-files-to-txt) can always be used to convert the JSON.

N.B. The text files are still being populated however, in time, these will be deprecated; users will be notified with ample notice before deprecation takes place.

### Accessing this data, for free

Spamhaus believes that due to the vital nature of the DROP list data, it should be available at no cost, regardless of size or business type, to protect internet users. We do ask, when used in a product, credit must be given to Spamhaus Project, and the date and © text should remain with the file and data.

For a more commercially-focused solution, which also includes communities listing compromised and dedicated [botnet command and controller (C&Cs)](https://www.spamhaus.org/blocklists/botnet-controller-list/), we make data available via our partner Spamhaus Technology. Find out more about accessing the data with [Border Gateway Protocol (BGP)](https://www.spamhaus.com/data-access/border-gateway-protocol/).

### Removal

Ranges in DROP are connected to the corresponding Spamhaus Blocklist (SBL) record mentioned in the DROP files. Once the SBL record is removed, the ranges will automatically leave DROP also. Visit the [SBL page](https://www.spamhaus.org/blocklists/spamhaus-blocklist/) for more information on removals.

## FAQs

-

-

Yes. DROP listings are re-evaluated on a daily basis. Many factors can trigger changes to listings, including contact with involved parties, observation or notification of false positives, automatically detected network reassignments, and more.

-

The DROP list contains IP ranges which are so dangerous to internet users that Spamhaus provides it to anyone who wants to use it, free of charge. Spamhaus believes that due to the vital nature of the DROP list data, it should be available at no cost to anyplace, regardless of size or business type, to protect internet users.

-

A “hijacked” or a “zombie” netblock is a block of IPs that have been “brought back from the dead”, often by a spammer:

- The original owner of the block leaves it derelict for any number of reasons.
- Squatters then reclaim it with various ploys. This may include registering an abandoned domain name to accept email for the domain contact, printing false letterhead, or doing some social engineering over the telephone.
- Some hijackers even outright steal IP-space allocated to someone else just by announcing it under their BGP [Autonomous System Number](https://www.spamhaus.org/glossary/#a).
- Autonomous Systems Numbers can be hijacked as well. Abandoned ASNs are taken by a spammer or spammer’s supplier to announce various IP ranges, so it’s possible to have a hijacked netblock advertised by a hijacked ASN!

Hijacked netblocks can be found in ranges assigned by every [Regional Internet Registry (RIR)](https://www.spamhaus.org/glossary/#r).

Restoring the proper ownership of a hijacked netblock means finding the original owner – which is often a dissolved company – and jumping through RIR hoops. It’s a slow and laborious process, important but not suitable to stopping today’s spam.

The peering/transit arrangements for these netblocks changes very quickly.

- Spamhaus lists the entire hijacked netblock in the SBL, categorized by RIR, and then provides additional pointer records for networks carrying the traffic for that netblock.
- While such records are often only a single router’s IP address, the record will indicate the greater problem and the full range of IPs.
- Spamhaus may also provide additional SBL records within a hijacked netblock because [SWiPs](https://teamarin.net/2018/05/03/which-swip-type-is-right-for-you/) or single IPs within the netblock are assigned to different spammers.

- These can serve as pointers to the upstream, as the block is sometimes SWiPed as portable subnets. Each spammer is then left to find their own transit.

Many of these hijacked netblocks find their way into [a ROKSO record specifically for them](https://www.spamhaus.org/faqs/rokso/?rokso_id=ROK2493). Spamhaus lists entire hijacked networks. Some of these netblocks are known to be controlled by a particular spammer and are thus listed under that spammer’s ROKSO records.

-

Other good uses for [DROP](https://www.spamhaus.org/blocklists/do-not-route-or-peer/) include:

- Logging customer queries for DNS servers in any DROP-listed IP space. This is a very good way to discover which are infected with malware.
- Vetting new transit customers proposed IP ranges against DROP; those ranges are often looking for new routing options.
- Scoring DROP ranges extra high in such software as [SpamAssassin](http://spamassassin.org).
- Using DROP ranges in a DNS RPZ zone to invalidate lookups in these ranges. The Spamhaus Technology website offers more information on using DROP in a [DNS Firewall Threat Feed](https://www.spamhaus.com/dataset/dns-firewall-threat-feeds/).

-

eDROP data was combined into the DROP list on 10 April 2024. You will still benefit from this protection by ensuring your set up is configured to utilize DROP list alone.

To understand why this update was made, read here https://www.spamhaus.org/resource-hub/network-security/spamhaus-drop-and-edrop-to-become-a-single-list/

-

-

Don’t Route Or Peer (DROP) is an advisory “drop all traffic” list.

[DROP](https://www.spamhaus.org/blocklists/do-not-route-or-peer/) is a tiny subset of the [SBL](https://www.spamhaus.org/blocklists/spamhaus-blocklist/) which is designed for use by firewalls and routing equipment.

- The DROP list will not include any IP space allocated to a legitimate network and then reassigned – even if reassigned to confirmed spammers.
- DROP does include netblocks that have been hijacked or are leased by professional spam or cyber-crime operations and are used for dissemination of malware, trojan downloaders, botnet controllers, etc.
- DROP is composed of direct allocations from ARIN, RIPE, APNIC, LACNIC, or other Regional Internet Registries and “portable allocations” (known as “PI”) from RIPE.

Spamhaus strongly recommends the use of DROP by Tier-1 and backbone networks. Consulting the [DROP list webpage](https://www.spamhaus.org/blocklists/do-not-route-or-peer/) when someone asks you to route some suspicious IPs can help avoid customers that will cause big problems on your network.

-

Update: eDROP data was combined into the DROP list on 10 April 2024 - [learn more here](https://www.spamhaus.org/faqs/do-not-route-or-peer-drop#where-has-edrop-gone).

Extended Don’t Route Or Peer (EDROP) is an extended version of the DROP list.

- [EDROP](https://www.spamhaus.org/blocklists/do-not-route-or-peer) includes netblocks controlled by professional spamming operations and cyber criminals, that are not directly allocated.
- Direct allocations will only be listed in DROP.
- EDROP only includes netblocks that are sub-allocations.

Spamhaus strongly recommends the use of EDROP by Tier-1 and backbone networks. Consulting the [DROP list webpage](https://www.spamhaus.org/blocklists/do-not-route-or-peer) when someone asks you to route some suspicious IPs can help avoid taking on customers that will cause big problems on your network.

-

-

All the networks listed in DROP are also listed in the SBL list. A DNS lookup for SBL and ZEN does return a listed status for those networks.

- A 127.0.0.9 return code indicates listings in DROP.

-

For those who use PC routers, here’s a little Perl script to turn the [CIDR](https://www.spamhaus.org/faqs/general-questions) blocks in the [DROP](https://www.spamhaus.org/blocklists/do-not-route-or-peer/) list into Unix route commands. Different versions of route have slightly different syntax, so you need to pick the one that works with your version. Some versions of route take CIDR notation, others require netmasks, so un-comment the one that works for you. (Note the obvious perl one-liner to turn a bit number into a dotted quad.)

To make day-to-day changes, use -o oldfile where oldfile is the previous version, and it’ll give you just route delete and route add for the changes. This script is set up to fetch the current list and update once a day, which is frequent enough for nearly all networks, given the slow day-to-day churn and very conservative listing policy of the DROP list.

Please DO NOT auto-fetch the DROP list more than once per hour!

These are all old, but still useful. Use at your own discretion!

```
#!/usr/bin/perl

# -\*- perl -\*-
use Getopt::Std;
use strict;
use vars qw{%nets $n $m $opt\_o};

getopts(‘o:’);
while(<>) {
	if(($n, $m) = m{(\\d+\\.\\d+\\.\\d+\\.\\d+)/(\\d+)}) {
    	# local sanity check
        die “local network $n”

        if $n =~ /^127.|^208.31./;

        $nets{$n} = $m;
    } else {
    	print “#??? $\_\\n”;
    }
}

# do old thing here sometime
if($opt\_o) { open(OLD, $opt\_o) or die “Cannot open $opt\_o $!”;</p> <p> while(<OLD>) { if(($n, $m) = m{(\\d+\\.\\d+\\.\\d+\\.\\d+)/(\\d+)}) { my $mask = join ‘.’,unpack “CCCC”, pack “N”, -1<<(32-$m);</p> <p> if(exists $nets{$n} and $nets{$n} == $m) { print “# exists route add -net $n/$m\\n”; delete $nets{$n}; } else { #print “route delete -net $n -netmask $mask\\n”; print “route delete -net $n/$m\\n”; } } else { print “#??? $\_\\n”; } } close OLD; }</p> <p>while(($n, $m) = each %nets) { my $mask = join ‘.’,unpack “CCCC”, pack “N”, -1<<(32-$m);</p> <p> #print “route add -net $n -netmask $m 127.1 -blackhole # m\\n”; print “route add -net $n/$m 127.1 -blackhole\\n”;</p> <p>}

```

Here’s the code for using DROP in Linux firewalls:

```
#!/bin/bash
#Script to add firewall rules to a linux system to completely block
#all traffic to and from networks in the spamhaus drop list.
#Copyright 2009, William Stearns, wstearns@pobox.com
#Released under the GPL. This and other tools can be found at
#http://www.stearns.org/
#Sole (optional) command line parameter is the file location of the
#drop list, such as:
#cd /var/lib/
#wget http://www.spamhaus.org/drop/drop.lasso
# ./spamhaus-drop /var/lib/drop.lasso
#While the DROP file should be regularly updated, this should
#probably be about once per day or less frequently; do _not_
#download DROP more than once an hour.
if [ -n “$1″ ];
then DropList=”$1″
else DropList=”./drop.lasso”
fi

if [ ! -s “$DropList” ]; then
echo “Unable to find drop list file $DropList . Perhaps do:” >&2
echo “wget http://www.spamhaus.org/drop/drop.lasso -O $DropList”
echo “exiting.” >&2
exit 1
fi
if [ ! -x /sbin/iptables ]; then
echo “Missing iptables command line tool, exiting.” >&2
exit 1
fi
cat “$DropList”
| sed -e 's/;.*//'
| grep -v '^ *$'
| while read OneNetBlock ; do
/sbin/iptables -I INPUT -s “$OneNetBlock” -j DROP
/sbin/iptables -I OUTPUT -d “$OneNetBlock” -j DROP
/sbin/iptables -I FORWARD -s “$OneNetBlock” -j DROP
/sbin/iptables -I FORWARD -d “$OneNetBlock” -j DROP
done

```

Bill Stearns also provides this tcpdump command line. It’s tested on Linux and probably works in any Unix/Posix environment, including Cygwin on Windows. A tcpdump error message “Warning: Kernel filter failed: Cannot allocate memory” seems to indicate that the filter can’t fit in kernel memory, and it appears that tcpdump then switches over to filtering in userspace.

```
tcpdump -tnp \`cat /var/lib/drop.lasso
| sed -e 's/;.\*//'
| grep -v '^ \*$'
| ( read OneAddr ; echo -n "net $OneAddr" ; while read OneAddr ; do echo -n " or net $OneAddr" ; done ; echo )\`

```

Here is a script that converts the DROP list into Qmail’s tcpserver blacklist:

```
#!/usr/bin/perl
#
# This script converts CIDR notation from STDIN or specified files to
# the format used for generating tcpserver’s blacklist.
#
# Usage example:
#
# Add this cronjob:
# 0 0 \* \* \* /usr/bin/wget http://www.spamhaus.org/drop/drop.lasso -O- 2>/dev/null|/usr/bin/drop2blacklist.pl >/etc/tcp.smtp.droplist
#
# Then generate your tcpserver’s CDB files as follow (or modify qmailctl as needed):
#
# cat /etc/tcp.smtp /etc/tcp.smtp.droplist|tcprules /etc/tcp.smtp.cdb /etc/.tcp.smtp.tmp
# chmod 644 /etc/tcp.smtp.cdb
#
#
# Author: Thomas Guyot-Sionnest <tguyot@gmail.com>
# This script has been released into the public domain.
# This script comes with absolutely no warranty.

use strict;
use warnings;
use Net::IP;
# Make this an empty string to remove the trailing dot
my $dot = ‘.’;
while (<>) {
next if (m/^\\s\*(?:;.\*)?$/);
s/\\s\*;.\*//;
my $ip = Net::IP->new($\_) or die (“Invalid IP Address: ‘$\_’\\n”);
die (“IPv6 not supported\\n”) if ($ip->version() != 4);
print map { “$\_:deny\\n” } rangeexpand($ip->ip(), $ip->last\_ip());
}</p> <p>
sub rangeexpand {
my $lowr = shift;
my $highr = shift;</p> <p>
return ipexpand(split(/\\./, “$lowr.$highr”));
}</p> <p>
sub ipexpand {
my @low = splice(@\_,0,4);
my @high = splice(@\_,0,4);</p> <p>
my @list;
if ($low\[0\] != $high\[0\]) {
push(@list, ipexpand(@low,$low\[0\],255,255,255));
for (my $i=$low\[0\]+1; $i<$high\[0\]; $i++) {
push(@list, ipexpand($i,0,0,0,$i,255,255,255));
}
push(@list,ipexpand($high\[0\],0,0,0,@high));
} elsif ($low\[1\] != $high\[1\]) {
if ($low\[1\] == 0 && $high\[1\] == 255) {
push(@list,”$low\[0\]$dot”);
} else {
push(@list,ipexpand(@low,$low\[0\],$low\[1\],255,255));
for (my $i=$low\[1\]+1; $i<$high\[1\]; $i++) {
push(@list,ipexpand($low\[0\],$i,0,0,$low\[0\],$i,255,255));
}
push(@list,ipexpand($high\[0\],$high\[1\],0,0,@high));
}
} elsif ($low\[2\] != $high\[2\]) {
if ($low\[2\] == 0 && $high\[2\] == 255) {
push(@list,”$low\[0\].$low\[1\]$dot”);
} else {
push(@list,ipexpand(@low,$low\[0\],$low\[1\],$low\[2\],255));
for (my $i=$low\[2\]+1; $i<$high\[2\]; $i++) {
push(@list,ipexpand($low\[0\],$low\[1\],$i,0,$low\[0\],$low\[1\],$i,255));
}
push(@list,ipexpand($high\[0\],$high\[1\],$high\[2\],0,@high));
}
} elsif ($low\[3\] != $high\[3\]) {
if ($low\[3\] == 0 && $high\[3\] == 255) {
push(@list,”$low\[0\].$low\[1\].$low\[2\]$dot”);
} else {
push(@list,map { “$low\[0\].$low\[1\].$low\[2\].$\_” } $low\[3\]..$high\[3\]); }
} else {
push(@list,”$low\[0\].$low\[1\].$low\[2\].$low\[3\]”); }</p> <p> return @list
}

```

-

If your router is a Cisco device and you don’t have BGP support on it (or don’t want to use it), you can also use a script called cisco-tools, developed by Marco d’Itri.

- The tool can be [downloaded directly](http://www.linux.it/~md/software/cisco-tools-0.3.tgz) from his [website](http://www.linux.it/~md/software/).
- Every time cisco-tools is run by crontab, it will download the list and report if there are changes. When run interactively it will remove old entries and ask whether any new entry should be used or not.

Here is a short list of helpful links for various firewalls, web filters & proxies:

- On the [OISF](https://oisf.net/) Community website: [Suricata](http://www.openinfosecfoundation.org/) rules [from Emerging Threats](http://rules.emergingthreats.net/blockrules/emerging-drop.suricata.rules).
- Code [in PHP](http://www.potato-people.com/code/misctools/spamhausdrop.phps) to create IPTables.
- Here is a Bash script to sync the DROP list into a Quagga/Linux route server: [spamhaus2quagga.sh](https://gist.github.com/dotysan/8463112).
- A script to add the DROP list into Linux iptables:[spamhaus.sh](https://github.com/cowgill/spamhaus).

NOTE: The data file & CIDR ranges may have to be manipulated for each system’s unique requirements.

USE AT YOUR OWN RISK!

-

Ranges in DROP are connected to the corresponding SBL record mentioned in the DROP file.

- Once the SBL record is removed, the ranges will automatically leave DROP.
- To request removal from ASN-DROP, you can use the [IP and Domain Reputation Checker](https://check.spamhaus.org/) to search for the ASN, then follow the steps detailed.

-

If you are a long term user of the text (.txt) DROP files a jq command can be used to convert the JSON file:

DROP

```
tail -1 drop_v4.json | jq -r '"; Spamhaus DROP List \(.timestamp | strftime("%Y/%m/%d")) - \(.copyright)\n; https://www.spamhaus.org/drop/drop_v4.json\n; Last-Modified: \(.timestamp | strftime("%a, %d %b %Y %H:%M:%S UTC"))\n; Expires: \(.timestamp+93600 | strftime("%a, %^C%b %Y %H:%M:%S UTC"))"' > drop.txt
jq -r 'select(.type == null) | (.cidr) + " ; " + (.sblid)' drop_v4.json >> drop.txt
echo "; EOF" >> drop.txt

```

DROPv6

```
tail -1 drop_v6.json | jq -r '"; Spamhaus IPv6 DROP List \(.timestamp | strftime("%Y/%m/%d")) - \(.copyright)\n; https://www.spamhaus.org/drop/drop_v6.json\n; Last-Modified: \(.timestamp | strftime("%a, %d %b %Y %H:%M:%S UTC"))\n; Expires: \(.timestamp+93600 | strftime("%a, %^C%b %Y %H:%M:%S UTC"))"' > dropv6.txt
jq -r 'select(.type == null) | (.cidr) + " ; " + (.sblid)' drop_v6.json >> dropv6.txt
echo "; EOF" >> dropv6.txt

```

-

Please DO NOT auto-fetch the DROP list more than once per hour!

The DROP list changes quite slowly. There is no need to update cached data more than once per hour, in fact once per day is more than enough in most cases.

Automated downloads must be at least one hour apart.

NOTE: Excessive downloads may result in your IP being firewalled from the Spamhaus website.

-

[DROP](https://www.spamhaus.org/blocklists/do-not-route-or-peer/) can be used by any appliance that has the ability to block or filter IP address ranges on their network.

The [DROP list](https://www.spamhaus.org/drop/drop.txt) is also open for all to download and use, there is no fee for usage. Our requirements are simple:

- When used in a product, credit for the use must be given to The Spamhaus Project, and the date and copy text should remain with the file and data.

Please check regularly to ensure you have the latest version of the DROP list.

- The check and update should be automated. Importing this list into a network filter without regular updates can cause problems.
- The DROP list data should not be downloaded from our website more than once per day.

NOTE:

Most of the other Spamhaus data-sets (SBL, XBL, PBL) are designed for filtering during the SMTP connection.

- The DROP list is small in comparison and should not be considered a replacement for them.
- DROP can be used to enhance existing filtering and security measures.

For Internet Service Providers (ISPs) or organisations that can run the Border Gateway Protocol (BGP) on their border routers, Spamhaus offers DROP along with its botnet C&C list (BGPCC) as a BGP feed, with which any networking device can peer using the BGP procotol. More information about this service can be found on the [Spamhaus BGPf page](https://www.spamhaus.org/blocklists/network-protection/).

-

-

The Spamhaus DROP (Don’t Route Or Peer) list contains [autonomous system numbers (ASNs)](https://en.wikipedia.org/wiki/Autonomous_system_(Internet)) that are hijacked or leased by professional spam or cyber-crime operations and used for dissemination of malware, trojan downloaders, botnet controllers, etc, and send no legitimate traffic.

-

ASNs are listed on the [ASN-DROP lis](https://www.spamhaus.org/drop/asndrop.json)t if they meet the criteria for inclusion in it: if we are observing them to have been hijacked or leased by professional spam or cyber-crime operations.

-

To request removal from ASN-DROP, you can use the [IP and Domain Reputation Checker](https://check.spamhaus.org/) to search for the ASN, then follow the steps detailed.

## Other Blocklists Available From Spamhaus

### CSS

Combined Spam Sources

[Learn more](https://www.spamhaus.org/blocklists/combined-spam-sources/)

### DBL

Domain Blocklist

[Learn more](https://www.spamhaus.org/blocklists/domain-blocklist/)

### XBL

Exploits Blocklist

[Learn more](https://www.spamhaus.org/blocklists/exploits-blocklist/)

### PBL

Policy Blocklist

[Learn more](https://www.spamhaus.org/blocklists/policy-blocklist/)

### SBL

Spamhaus Blocklist

[Learn more](https://www.spamhaus.org/blocklists/spamhaus-blocklist/)

### ZEN

PBL, SBL & XBL combines

[Learn more](https://www.spamhaus.org/blocklists/zen-blocklist/)

[Home](https://www.spamhaus.org/)

[Home](https://www.spamhaus.org/)

...

[Home](https://www.spamhaus.org/)
