---
url: "https://feodotracker.abuse.ch/blocklist/"
title: "Feodo Tracker | Blocklist"
captured: "2026-11-02T18:30:19Z"
via: "live"
sha256: "7baf5e859d8a7ae9f4ab67b1d8c15067f1613a07ccffd93e6e856c4661c15877"
wayback: "https://web.archive.org/web/20260908150606/https://feodotracker.abuse.ch/blocklist/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Feodo Tracker | Blocklist

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://feodotracker.abuse.ch/blocklist/>
> Snapshot: <https://web.archive.org/web/20260908150606/https://feodotracker.abuse.ch/blocklist/>

# Blocklist

Feodo Tracker offers various blocklist in different formats for different purposes. The blacklists are documented below.

[Botnet C2 IP Blocklist](https://feodotracker.abuse.ch/blocklist/#ip-blocklist) [Botnet C2 Indicators Of Compromise (IOCs)](https://feodotracker.abuse.ch/blocklist/#iocs) [Suricata Botnet C2 IP Ruleset](https://feodotracker.abuse.ch/blocklist/#ip-ids) [Terms of Services](https://feodotracker.abuse.ch/blocklist/#tos)

## Botnet C2 IP Blocklist

Dridex, Heodo (aka Emotet), TrickBot, QakBot (aka QuakBot / Qbot) and BazarLoader (aka BazarBackdoor) botnet command&control servers (C2s) usually reside on compromised servers and such that have been rented and setup by the threat actor itself for the sole purpose of botnet hosting. Feodo Tracker offers a blocklist of IP addresses that are associated with such botnet C2s. It can be used to block botnet C2 traffic from infected machines towards hostline servers on the internet that are under the control of cybercriminals.

To keep the false positive rate as low as possible, an IP address will only get added to the blocklist if it responds with a valid botnet C2 response

The Botnet C2 IP Blocklist gets generated every 5 minutes and is available in the plain-text and JSON format. We recommend you to update the list at least every 15 minutes (or even better: every 5 minutes) to receive the best protection against Dridex, Emotet, TrickBot, QakBot and BazarLoader.

#### Recommended IP blocklist

If you want to block botnet C&C IP addresses but avoid false positives, I highly recommend you to use the following blocklist as it only contains active botnet C&C servers or such that have been active in the past hours. Although false positives can happen on this blocklist, the false positive rate should be low.

[Plain-Text](https://feodotracker.abuse.ch/downloads/ipblocklist_recommended.txt) [JSON](https://feodotracker.abuse.ch/downloads/ipblocklist_recommended.json)

We also have custom formats of the blocklist available for McAfee Web Gateway and Palo Alto Firewall:

[McAfee Web Gateway](https://feodotracker.abuse.ch/downloads/ipblocklist_recommended_mcafee.txt) [Palo Alto Firewall](https://feodotracker.abuse.ch/downloads/ipblocklist_recommended_paloalto.txt)

## Botnet C2 Indicators Of Compromise (IOCs)

If you have a SIEM (Security Information and Event Management) product, you can enrich it with data from Feodo Tracker to get alerted about potential botnet C2 traffic leaving your network. Unlike the IP blocklist above, these datasets do not only contain additional information on tracked botnet C2s but also IP addresses that were acting as a botnet C2 within the past 30 days.

[Download CSV](https://feodotracker.abuse.ch/downloads/ipblocklist.csv) [Download JSON](https://feodotracker.abuse.ch/downloads/ipblocklist.json) [Download IPs only](https://feodotracker.abuse.ch/downloads/ipblocklist.txt)

In case you want to get a comprehensive list of all botnet C2s Feodo Tracker has ever seen, you may use the IoC list below. However, as IP addresses are being re-used/recycled, the false positive of this dataset is much higher.

[Download CSV (Aggressive)](https://feodotracker.abuse.ch/downloads/ipblocklist_aggressive.csv) [Download IPs only (Aggressive)](https://feodotracker.abuse.ch/downloads/ipblocklist_aggressive.txt)

#### Caution

I strongly recommend you to not use the aggressive version of the Botnet C2 Indicators Of Compromise (IOCs) as it definitely will cause false positives.

## Suricata Botnet C2 IP Ruleset

The Suricata Botnet C2 IP Ruleset contains botnet C2s tracked by Feodo Tracker and can be used for both, [Suricata](http://suricata-ids.org/) and [Snort](https://www.snort.org/) open source IDS/IPS. If you are running Suricata or Snort, you can use this ruleset to detect and/or block network connections towards hostline servers (IP address:port combination).

The ruleset gets generated every 5 minutes. We recommend you to update the IDS ruleset at least every 15 minutes (or even better: every 5 minutes) to receive the best protection against Dridex, Emotet, TrickBot, QakBot and BazarLoader.

#### Note

As IP addresses are getting recycled and reused, this blocklist only botnet C2s that are either active or that have been last seen in the past 30 days. The false positive rate for this blocklist should therefore be low.

[Download IDS Ruleset (Suricata and Snort)](https://feodotracker.abuse.ch/downloads/feodotracker.rules)

[Download IDS Ruleset (Suricata and Snort) - tar.gz](https://feodotracker.abuse.ch/downloads/feodotracker.tar.gz)

In case you want to get a comprehensive list of all botnet C2s Feodo Tracker has ever seen, you may use the IDS ruleset below. However, as IP addresses are being re-used/recycled, the false positive of this dataset is much higher.

[Download IDS Ruleset (Aggressive)](https://feodotracker.abuse.ch/downloads/feodotracker_aggressive.rules)

[Download IDS Ruleset (Aggressive) - tar.gr](https://feodotracker.abuse.ch/downloads/feodotracker_aggressive.tar.gz)

#### Caution

I strongly recommend you to not use the aggressive version of the Botnet C2 Indicators Of Compromise (IOCs) as it definitely will cause false positives.

## Terms of Services (ToS)

By using the website of Feodo Tracker, or any of the services/datasets referenced above, refer to https://abuse.ch/terms-of-use/
