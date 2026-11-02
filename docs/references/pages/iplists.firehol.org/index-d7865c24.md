---
url: "https://iplists.firehol.org/"
title: "FireHOL IP Lists | IP Blacklists | IP Reputation Feeds"
site: "FireHOL IP Lists"
captured: "2026-11-02T18:33:33Z"
via: "live"
sha256: "f467860e6c777e6f57d86ba0d87c1606fdac74ca2894ec88957588d2d68254b3"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# FireHOL IP Lists | IP Blacklists | IP Reputation Feeds

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://iplists.firehol.org/>

#  loading...

Please wait while ipset data are being loaded...

Have you seen [netdata](http://netdata.firehol.org/)?

|     | Do you like this site?

| [Help us make it better](https://iplists.firehol.org/#).

## Overview of

| name |

| category |

| maintainer |

| IP family |

| ipset hash |

| ipset entries |

| unique IPs |

| source |

| local copy |

| changesets |

| check
frequency |

| average update
frequency |

| aggregation |

| fetch errors |

| monitoring since |

| last time
updated
by its maintainers |

| last time
processed
by us |

| last time
we checked |

### All IP lists monitored

- [By Category](https://iplists.firehol.org/#all_by_category)
- [By Maintainer](https://iplists.firehol.org/#all_by_maintainer)
- [Alphabetically](https://iplists.firehol.org/#all_alphabetically)

## About

## Evolution of

Each time the IP list is changed, modified, or updated we keep track of its size (both number of entries and number of unique IPs matched). Using this information we can detect what the list maintainers do, get an idea of the list trend and its maintainers habbits.

Using the chart below we attempt to answer these questions:

-

##### [How many entries does it have?](https://iplists.firehol.org/#evolutionCollapseOne)

If you are going to use this IP list as a blocklist / blacklist at a firewall, its size can be important for the performance of the firewall.
Keep in mind that the performance of Linux netfilter / iptables firewalls that use ipsets (like [FireHOL](http://firehol.org/) does), is not affected by the size of an ipset. Any number of entries can be added and the firewall will just do one lookup for every packet checked against the ipset. Linux ipsets are affected only by the number of different subnets in an ipset. [FireHOL](http://firehol.org/) solves this by automatically reducing the number of unique subnets on all hash:net ipsets (check [this article](https://github.com/firehol/firehol/wiki/iprange:-optimizing-ipsets-for-iptables#reducing-ipsets) for more information on how this is done).

-

##### [How many unique IPs does it match?](https://iplists.firehol.org/#evolutionCollapseTwo)

The number of unique IPs matched by an IP list, determines the effectiveness of the blacklist / blocklist.
Generally, smaller IP lists are more focused and safer to use as firewall blacklists / blocklists. Fewer unique IPs means fewer possible false positives.
On the other hand a very small list will not provide a significant level of protection.

-

##### [Is it updated frequently and regularly?](https://iplists.firehol.org/#evolutionCollapseThree)

We need IP lists that are well maintained, frequently and regularly.
In the chart below, every point is updated only when the list maintainers add IPs to, or remove IPs from the IP list, so even if the number of unique IPs remains the same, a point in the chart indicates that something changed in it. The exact number of unique IPs added and removed with each update can be seen on the chart next to the one below.
The frequency of updates is irrelevant to the retention policy of the IP list. We will examine its retention below in the sections below.

-

##### [Does it have a consistent size through time?](https://iplists.firehol.org/#evolutionCollapseFour)

We don't want surprises. Sudden increases or decreases is generally an indication of poor maintenance.
Of course, there are cases where an IP list will by definition have sudden changes in its size.

The chart below shows the last 500 updates, of the IP list.

- Entries is the number of entries the ipset has.
- UniqueIPs is the number of unique IPs the ipset matches.

|

|

| Loading evolution chart...

|

The chart below shows the change history of the IP list, i.e. the number of unique IPs added and removed with each update.

Using the chart below we attempt to answer these questions:

-

##### [How much of this IP list is changed on every update?](https://iplists.firehol.org/#changesetCollapseOne)

There are IP lists that, although they have an almost constant size, they change their contents almost entirely on every update.
In other cases, similar IP lists have minimal incremental updates.
The following chart attempts to visualize this.

|

|

| Loading changesets chart...

|

## Country Map of

Each time an ipset is updated we check it against the [MaxMind GeoLite2 country](https://dev.maxmind.com/geoip/geoip2/geolite2/), the [IPDeny.com country](http://www.ipdeny.com/), the [IP2Location.com Lite country](http://www.ip2location.com/), the [IPIP.net country](http://ipip.net/) and the [DB-IP.com Lite country](https://db-ip.com/) databases, to find the list's unique IPs per country.

Using the maps below we attempt to answer these questions:

-

##### [Which countries does it currently match?](https://iplists.firehol.org/#mapCollapseOne)

If you are going to install this IP list as a blocklist / blacklist at a firewall, it is important to know which countries will be mainly affected, since you are going to block access from/to these IPs.

All lists suffer from false positives to some degree, so using this IP list at your firewall might block some of your users or customers.

-

##### [Where do the attackers or the abusers come from?](https://iplists.firehol.org/#mapCollapseTwo)

Some lists focus only on specific regions of the world. The following map illustrates this. It is a heat map of the list's focus.

- [MaxMind.com GeoLite2](https://iplists.firehol.org/#geolite2)
- [IPDeny.com](https://iplists.firehol.org/#ipdeny)
- [IP2Location.com Lite](https://iplists.firehol.org/#ip2location)
- [IPIP.net](https://iplists.firehol.org/#ipip)
- [DB-IP.com Lite](https://iplists.firehol.org/#dbip)

|

|

| Loading geolite2 map...

|

|

|

| Loading ipdeny map...

|

|

|

| Loading ip2location map...

|

|

|

| Loading ipip map...

|

|

|

| Loading dbip map...

|

## Age of IPs listed in

The age of each IP in the list is shown below. The time shown is calculated in realtime; it will be refreshed as time passes, even if the list is not updated.

Using the chart below we attempt to answer these questions:

-

##### [What is the current age of the IPs listed?](https://iplists.firehol.org/#ageCollapseOne)

Most lists include IPs that match some criteria (e.g. an attack or abuse is detected originated from the IP in question). Once an IP is listed, it remains listed for a pre-defined amount of time, unless it matches the criteria again, in which case its expiration time is refreshed.

Many lists announce the duration they list IPs. Many don't and almost all lists have exceptions that do not follow the announced rules.

A false positive is in place when an IP that was properly detected and added to the list, was released and re-used by another person, before being unlisted from the list. Since the world is full of dynamic IP users, false positives is the biggest problem of blocklist / blacklists.

In the chart below we show the exact age of the IPs currently listed. Small ages are good. Long ages are not necessarily bad. Normally, longer ages should only be a small part of the list's size.

Pay attention to the 50% mark. This is the average age of the IPs in the list. Pay also attention to the 75% (most probable) and the 90% (expected max) marks.

-

##### [Does the list include any static data?](https://iplists.firehol.org/#ageCollapseTwo)

The ideal age chart of a well maintained IP list should a straight line from the bottom left corner, to the upper right corner of the chart.

Of course, this is affected by the pressure of different attacks and possibly the different listing policies for different types of attacks.

In general though, this chart should be as granural as possible.

Long horizontal lines indicate either sustaining attacks, or unreasonably high listing policies.

|

|

| Loading age chart...

|

## Retention Policy of

The retention policy of the list shows the duration IPs were listed, when they were listed. This is calculated every time the list maintainers remove an IP from the list. The chart below shows the retention policy detected, since we started monitoring the list (it is not limited to a certain timeframe).

Using the chart below we attempt to answer these questions:

-

##### [When are IPs being removed from the list?](https://iplists.firehol.org/#retentionCollapseOne)

This chart shows data for the past IPs, currently unlisted.
The vertical parts of the "stair steps" in this chart, indicate periods of intensive IPs cleanup. This is their retention policy.
If the chart contains more than one "stair steps", the list has many different retention policies.

|

|

| Loading retention chart...

|

## Overlaps of  with other IP lists

Using the chart below we attempt to answer these questions:

-

##### [Is the list a derivative of other lists?](https://iplists.firehol.org/#comparisonCollapseOne)

Check the column Their %. A high percentage in this column, indicates that the IP list of that row is included in .

-

##### [Is the list included in any other list?](https://iplists.firehol.org/#comparisonCollapseTwo)

Check the column This %. A high percentage in this column, indicates that  is included in the IP list of that row.

-

##### [Does the list share IPs with other lists?](https://iplists.firehol.org/#comparisonCollapseThree)

Focus on the last two columns: Their % and This %. These two percentages show the percentage of overlap this list has with other IP lists.

Using the comparison table, we can easily find out that, for example, abuse is often initiated from anonymizing IPs (like open proxies) and malwares.

| Category  | List  | Unique IPs  | Common IPs  | Their %  | This %

### Comments on

2015-2017 [Costa Tsaousis](https://iplists.firehol.org/cdn-cgi/l/email-protection#9cfff3efe8fddce8effdf3e9eff5efb2fbee), for [FireHOL](http://firehol.org) a firewall for humans!.
The data on this page are automatically generated using FireHOL's [update-ipsets.sh](https://github.com/firehol/firehol/blob/master/sbin/update-ipsets) (for downloading the lists from their sources and generating the data for this site), which utilizes [iprange](https://github.com/firehol/firehol/wiki/iprange:-optimizing-ipsets-for-iptables) (for comparing and manipulating IP lists). Both are part of [FireHOL](http://firehol.org), which is provided under GPL v2, so you are free to get, use, adapt and re-distribute.
This site is provided as-is, without any warranty. IP Lists are a property of their maintainers.
This site is a single static page, with all its data uploaded as static JSON and CSV files every time an IP List is updated. For the final result, it utilizes IP data and web services provided by third parties. It uses IP lists and related data provided and maintained by their respective owners (mentioned together with each IP list), IP-to-country geolocation data provided by [maxmind.com](https://www.maxmind.com) (GeoLite2), [ipdeny.com](http://www.ipdeny.com), [ip2location.com](http://www.ip2location.com) (Lite) and [ipip.net](http://ipip.net), javascript chart libraries provided by [highcharts.com](http://www.highcharts.com), comments engine provided by [disqus.com](https://disqus.com), social media sharing buttons provided by [shareaholic.com](https://shareaholic.com), the HTML, CSS and JS framework [bootstrap](https://getbootstrap.com/), the [bootstrap-table](http://bootstrap-table.wenzhixin.net.cn/) component, icons provided by [iconsdb.com](http://www.iconsdb.com/) and it uses several services provided by [github](https://github.com).

#### About this site

This site aggregates, analyzes, compares and documents publicly available IP Feeds, with a focus on attacks and abuse. It is automatically generated and maintained using open source software (check the wiki), that can be installed and run on your systems too, to download all IP lists directly from their maintainers, process them and re-generate the site and its data.

Special care has been given to make this analysis as scientific and objective as possible, respecting the hard work of the security teams, security companies and security professionals who offer these IP lists to the rest of us.

Of course, security is achieved with a lot more than IP lists. And not all IP lists included here should be used for blocking traffic at a firewall or border router. Many of them, should be used, for example, to influence the way applications handle clients, or help in the development of further threat analysis.

Unfortunately, the InfoSec industry still considers as a standard industry practice the trade of Threat Intelligence for money.

This is disappointing.
Why?

Threat Intelligence requires knowledge, skills and sophisticated tools to be effective. Instead of selling these skills and tools, security firms selling threat intel state clearly they have valid information that identifies criminals. But they want money to reveal it.

This is contradictory to what we consider acceptable, if it was about criminal activity other than cyber.

So, I have concluded that either the InfoSec industry has a severe cultural fault, or they have nothing. The super duper feeds they advertise are just a marketing tool to attract customers. They sell an illusion...

Many will argue that collecting threat intel is expensive.

Of course it is!
Then, you will also accept it if someone opens an online shop to sell information about a gang that breaks houses in your neighbor, as long as it cost them enough to acquire this information. Yes?

To my understanding Threat Intelligence cannot be effective when it is treated as Intellectual Property.
Hopefully, many security companies and professionals agree and openly distribute the result of their hard work.
