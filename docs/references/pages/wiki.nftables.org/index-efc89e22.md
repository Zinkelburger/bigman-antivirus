---
url: "https://wiki.nftables.org/"
title: "nftables wiki"
captured: "2026-11-02T18:50:04Z"
via: "live"
source_url: "https://wiki.nftables.org/wiki-nftables/index.php/Main_Page"
sha256: "54b0c6dd43c09272bb985b4a86887d09203ace501f2d22e396d53616d576d15b"
wayback: "https://web.archive.org/web/20210816031021/https://wiki.nftables.org/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# nftables wiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.nftables.org/>
> Snapshot: <https://web.archive.org/web/20210816031021/https://wiki.nftables.org/>

# Main Page

From nftables wiki

[Jump to navigation](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#mw-head) [Jump to search](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#searchInput)

Welcome to the nftables HOWTO documentation page. Here you will find documentation on how to build, install, configure and use nftables.

If you have any suggestion to improve it, please send your comments to Netfilter users mailing list <netfilter@vger.kernel.org>.

## Contents

- [1 News](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#News)
- [2 Introduction](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Introduction)
- [3 Reference](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Reference)
- [4 Installing nftables](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Installing_nftables)
- [5 Upgrading from xtables to nftables](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Upgrading_from_xtables_to_nftables)
- [6 Basic operation](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Basic_operation)
- [7 Expressions: Matching packets](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Expressions:_Matching_packets)
- [8 Statements: Acting on packet matches](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Statements:_Acting_on_packet_matches)
- [9 Advanced data structures for performance packet classification](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Advanced_data_structures_for_performance_packet_classification)
- [10 Examples](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Examples)
- [11 Development](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Development)
- [12 External links](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#External_links)
- [13 Thanks](https://wiki.nftables.org/wiki-nftables/index.php/Main_Page#Thanks)

# [News](https://wiki.nftables.org/wiki-nftables/index.php/News)

# Introduction

- [What is nftables?](https://wiki.nftables.org/wiki-nftables/index.php/What_is_nftables%3F)
- [How to obtain help/support](https://wiki.nftables.org/wiki-nftables/index.php/How_to_obtain_help/support)

# Reference

- [man nft - netfilter website](https://www.netfilter.org/projects/nftables/manpage.html)
- [man nft - mankier.com](https://www.mankier.com/8/nft)
- [Quick reference, nftables in 10 minutes](https://wiki.nftables.org/wiki-nftables/index.php/Quick_reference-nftables_in_10_minutes)
- [Netfilter hooks](https://wiki.nftables.org/wiki-nftables/index.php/Netfilter_hooks) and nftables integration with existing Netfilter components
- [Understanding nftables families](https://wiki.nftables.org/wiki-nftables/index.php/Nftables_families)
- [Data types](https://wiki.nftables.org/wiki-nftables/index.php/Data_types)
- [Connection tracking system (conntrack)](https://wiki.nftables.org/wiki-nftables/index.php/Connection_Tracking_System), used for stateful firewalling and NAT
- [Troubleshooting and FAQ](https://wiki.nftables.org/wiki-nftables/index.php/Troubleshooting)
- [Additional documentation](https://wiki.nftables.org/wiki-nftables/index.php/Further_documentation)

# Installing nftables

- [Using nftables from distributions](https://wiki.nftables.org/wiki-nftables/index.php/Nftables_from_distributions)
- [Building and installing nftables from sources](https://wiki.nftables.org/wiki-nftables/index.php/Building_and_installing_nftables_from_sources)

# Upgrading from xtables to nftables

- [Legacy xtables tools](https://wiki.nftables.org/wiki-nftables/index.php/Legacy_xtables_tools)
- [Moving from iptables to nftables](https://wiki.nftables.org/wiki-nftables/index.php/Moving_from_iptables_to_nftables)
- [Moving from ipset to nftables](https://wiki.nftables.org/wiki-nftables/index.php/Moving_from_ipset_to_nftables)

# Basic operation

- [Configuring tables](https://wiki.nftables.org/wiki-nftables/index.php/Configuring_tables)
- [Configuring chains](https://wiki.nftables.org/wiki-nftables/index.php/Configuring_chains)
- [Simple rule management](https://wiki.nftables.org/wiki-nftables/index.php/Simple_rule_management)
- [Atomic rule replacement](https://wiki.nftables.org/wiki-nftables/index.php/Atomic_rule_replacement)
- [Error reporting from the command line](https://wiki.nftables.org/wiki-nftables/index.php/Error_reporting_from_the_command_line)
- [Building rules through expressions](https://wiki.nftables.org/wiki-nftables/index.php/Building_rules_through_expressions)
- [Operations at ruleset level](https://wiki.nftables.org/wiki-nftables/index.php/Operations_at_ruleset_level)
- [Monitoring ruleset updates](https://wiki.nftables.org/wiki-nftables/index.php/Monitoring_ruleset_updates)
- [Scripting](https://wiki.nftables.org/wiki-nftables/index.php/Scripting)
- [Ruleset debug/tracing](https://wiki.nftables.org/wiki-nftables/index.php/Ruleset_debug/tracing)
- [Ruleset debug/VM code analysis](https://wiki.nftables.org/wiki-nftables/index.php/Ruleset_debug/VM_code_analysis)
- [Output text modifiers](https://wiki.nftables.org/wiki-nftables/index.php/Output_text_modifiers)

# Expressions: Matching packets

- [Matching packet metainformation](https://wiki.nftables.org/wiki-nftables/index.php/Matching_packet_metainformation)
- [Matching packet headers](https://wiki.nftables.org/wiki-nftables/index.php/Matching_packet_headers)
- [Matching connection tracking stateful metainformation](https://wiki.nftables.org/wiki-nftables/index.php/Matching_connection_tracking_stateful_metainformation)
- [Matching routing information](https://wiki.nftables.org/wiki-nftables/index.php/Matching_routing_information)
- [Rate limiting matchings](https://wiki.nftables.org/wiki-nftables/index.php/Rate_limiting_matchings)

# Statements: Acting on packet matches

- [Accepting and dropping packets](https://wiki.nftables.org/wiki-nftables/index.php/Accepting_and_dropping_packets)
- [Rejecting traffic](https://wiki.nftables.org/wiki-nftables/index.php/Rejecting_traffic)
- [Jumping to chain](https://wiki.nftables.org/wiki-nftables/index.php/Jumping_to_chain)
- [Counters](https://wiki.nftables.org/wiki-nftables/index.php/Counters)
- [Logging traffic](https://wiki.nftables.org/wiki-nftables/index.php/Logging_traffic)
- [Performing Network Address Translation (NAT)](https://wiki.nftables.org/wiki-nftables/index.php/Performing_Network_Address_Translation_(NAT))
- [Setting packet metainformation](https://wiki.nftables.org/wiki-nftables/index.php/Setting_packet_metainformation)
- [Setting packet connection tracking metainformation](https://wiki.nftables.org/wiki-nftables/index.php/Setting_packet_connection_tracking_metainformation)
- [Mangling packet headers](https://wiki.nftables.org/wiki-nftables/index.php/Mangling_packet_headers) (including stateless NAT)
- [Duplicating packets](https://wiki.nftables.org/wiki-nftables/index.php/Duplicating_packets)
- [Load balancing](https://wiki.nftables.org/wiki-nftables/index.php/Load_balancing)
- [Queueing to userspace](https://wiki.nftables.org/wiki-nftables/index.php/Queueing_to_userspace)

# Advanced data structures for performance packet classification

- [Intervals](https://wiki.nftables.org/wiki-nftables/index.php/Intervals)
- [Concatenations](https://wiki.nftables.org/wiki-nftables/index.php/Concatenations)
- [Math operations](https://wiki.nftables.org/wiki-nftables/index.php/Math_operations)
- [Stateful objects](https://wiki.nftables.org/wiki-nftables/index.php/Stateful_objects)

- [Counters](https://wiki.nftables.org/wiki-nftables/index.php/Counters)
- [Quotas](https://wiki.nftables.org/wiki-nftables/index.php/Quotas)
- [Limits](https://wiki.nftables.org/wiki-nftables/index.php/Limits)
- [Connlimits](https://wiki.nftables.org/wiki-nftables/index.php/Connlimits) (ct count)

- Other objects

- [Conntrack helpers](https://wiki.nftables.org/wiki-nftables/index.php/Conntrack_helpers) (ct helper, Layer 7 ALG)
- [Conntrack timeout policies](https://wiki.nftables.org/wiki-nftables/index.php/Ct_timeout) (ct timeout)
- [Conntrack expectations](https://wiki.nftables.org/wiki-nftables/index.php/Ct_expectation) (ct expectation)
- [Synproxy](https://wiki.nftables.org/wiki-nftables/index.php/Synproxy)
- [Secmarks](https://wiki.nftables.org/wiki-nftables/index.php/Secmark)

- Generic set infrastructure

- [Sets](https://wiki.nftables.org/wiki-nftables/index.php/Sets)
- [Element timeouts](https://wiki.nftables.org/wiki-nftables/index.php/Element_timeouts)
- [Updating sets from the packet path](https://wiki.nftables.org/wiki-nftables/index.php/Updating_sets_from_the_packet_path)
- [Maps](https://wiki.nftables.org/wiki-nftables/index.php/Maps)
- [Verdict maps](https://wiki.nftables.org/wiki-nftables/index.php/Verdict_Maps_(vmaps))
- [Metering](https://wiki.nftables.org/wiki-nftables/index.php/Meters) (formerly known as flow tables before nftables 0.8.1)

- [Flowtables](https://wiki.nftables.org/wiki-nftables/index.php/Flowtables) (the fastpath network stack bypass)

# Examples

- [Simple ruleset for a workstation](https://wiki.nftables.org/wiki-nftables/index.php/Simple_ruleset_for_a_workstation)
- [Simple ruleset for a server](https://wiki.nftables.org/wiki-nftables/index.php/Simple_ruleset_for_a_server)
- [Simple ruleset for a home router](https://wiki.nftables.org/wiki-nftables/index.php/Simple_ruleset_for_a_home_router)
- [Bridge filtering](https://wiki.nftables.org/wiki-nftables/index.php/Bridge_filtering)
- [Multiple NATs using nftables maps](https://wiki.nftables.org/wiki-nftables/index.php/Multiple_NATs_using_nftables_maps)
- [Classic perimetral firewall example](https://wiki.nftables.org/wiki-nftables/index.php/Classic_perimetral_firewall_example)
- [Port knocking example](https://wiki.nftables.org/wiki-nftables/index.php/Port_knocking_example)
- [Classification to tc structure example](https://wiki.nftables.org/wiki-nftables/index.php/Classification_to_tc_structure_example)
- [Using configuration management systems](https://wiki.nftables.org/wiki-nftables/index.php/Using_configuration_management_systems) (like puppet, ansible, etc)
- [GeoIP matching](https://wiki.nftables.org/wiki-nftables/index.php/GeoIP_matching)

# Development

Check [Portal:DeveloperDocs - documentation for netfilter developers](https://wiki.nftables.org/wiki-nftables/index.php/Portal:DeveloperDocs).

Some hints on the general development progress:

- [List of updates since Linux kernel 3.13](https://wiki.nftables.org/wiki-nftables/index.php/List_of_updates_since_Linux_kernel_3.13)
- [List of updates in the nft command line tool](https://wiki.nftables.org/wiki-nftables/index.php/List_of_updates_in_the_nft_command_line_tool)
- [Supported features compared to {ip,ip6,eb,arp}tables](https://wiki.nftables.org/wiki-nftables/index.php/Supported_features_compared_to_xtables)
- [List of available translations via iptables-translate tool](https://wiki.nftables.org/wiki-nftables/index.php/List_of_available_translations_via_iptables-translate_tool)

# External links

Watch some videos:

- Watch [Getting a grasp of nftables](https://www.youtube.com/watch?v=FXTRRwXi3b4), thanks to [NLUUG association](https://www.nluug.nl/index-en.html) for recording this.
- Watch [The ultimate packet classifier for GNU/Linux](https://www.youtube.com/watch?v=CaYp0d2wiuU#t=1m47s), thanks to the FSFE for paying my trip to Barcelona and for recommending me as speaker to the KDE Spanish branch.
- [Florian Westphal - Why nftables?](https://www.youtube.com/watch?v=Sy0JDX451ns)
- Watch [NLUUG - Goodbye iptables, Hello nftables](https://www.youtube.com/watch?v=0wQfSfDVN94)

Watch videos to track updates:

- Watch [Netdev 2.1 - Netfilter mini-workshop (2017)](https://www.youtube.com/watch?v=qXVOA2MKA1s)
- Watch [Netdev 2.2 - Netf‌ilter mini-workshop (2018)](https://youtu.be/iCj10vEKPrw)
- Watch [Netdev 0x12 - Netf‌ilter mini-workshop (2019)](https://youtu.be/0hqfzp6tpZo)
- Watch [Netdev 0x14 - Netfilter mini-Workshop (2020)](https://www.youtube.com/watch?v=GqGGo4svj7s&feature=youtu.be)

Additional documentations and articles:

- Tutorial [Extending nftables by Xiang Gao](https://zasdfgbnm.github.io/2017/09/07/Extending-nftables/)
- Article [New in Debian stable Stretch: nftables](http://ral-arturo.org/2017/05/05/debian-stretch-stable-nftables.html)
- Article [How to use nftables from python](https://ral-arturo.org/2020/11/22/python-nftables-tutorial.html) and git repository [python-nftables-tutorial.git](https://github.com/aborrero/python-nftables-tutorial)

# Thanks

To the NLnet foundation for initial sponsorship of this HOWTO:

https://nlnet.nl

To Eric Leblond, for boostrapping the [Nftables quick howto](https://home.regit.org/netfilter-en/nftables-quick-howto/) in 2013.

Retrieved from "http://wiki.nftables.org/wiki-nftables/index.php?title=Main_Page&oldid=1152"

## Navigation menu

### Search

https://wiki.nftables.org/wiki-nftables/index.php/Main_Page
