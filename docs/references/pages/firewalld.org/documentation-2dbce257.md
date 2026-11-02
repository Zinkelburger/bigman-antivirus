---
url: "https://firewalld.org/documentation/"
title: "Documentation"
site: "firewalld"
captured: "2026-11-02T18:31:09Z"
via: "live"
sha256: "1817a4e9f914d7612582e022ee7945ca0f6b89cad664e1ec8975fffbdc3e40fb"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://firewalld.org/documentation/>

# Documentation

firewalld provides a dynamically managed firewall with support for network/firewall “zones” to assign a level of trust to a network and its associated connections, interfaces or sources. It has support for IPv4, IPv6, Ethernet bridges and also for IPSet firewall settings. There is a separation of the runtime and permanent configuration options. It also provides an interface for services or applications to add iptables, ip6tables and ebtables rules directly. This interface can also be used by advanced users.

## Table of Contents

-  [Concepts](https://firewalld.org/documentation/concepts.html)
-  [Architecture](https://firewalld.org/documentation/architecture.html)
-  [Configuration](https://firewalld.org/documentation/configuration/)

-  [Directories](https://firewalld.org/documentation/configuration/directories.html)
-  [Runtime versus Permanent](https://firewalld.org/documentation/configuration/runtime-versus-permanent.html)
-  [firewalld.conf](https://firewalld.org/documentation/configuration/firewalld-conf.html)

-  [Utilities](https://firewalld.org/documentation/utilities/)

-  [firewall-cmd](https://firewalld.org/documentation/utilities/firewall-cmd.html)
-  [firewall-offline-cmd](https://firewalld.org/documentation/utilities/firewall-offline-cmd.html)
-  [firewall-config](https://firewalld.org/documentation/utilities/firewall-config.html)
-  [firewall-applet](https://firewalld.org/documentation/utilities/firewall-applet.html)

-  [The daemon: firewalld](https://firewalld.org/documentation/the-daemon-firewalld.html)
-  [Zone](https://firewalld.org/documentation/zone/)

-  [Predefined Zones](https://firewalld.org/documentation/zone/predefined-zones.html)
-  [Connections, Interfaces and Sources](https://firewalld.org/documentation/zone/connections-interfaces-and-sources.html)
-  [Configuration of Zones](https://firewalld.org/documentation/zone/configuration-of-zones.html)
-  [Default Zone](https://firewalld.org/documentation/zone/default-zone.html)
-  [Use of Zones](https://firewalld.org/documentation/zone/use-of-zones.html)
-  [Options](https://firewalld.org/documentation/zone/options.html)
-  [Examples](https://firewalld.org/documentation/zone/examples.html)

-  [Service](https://firewalld.org/documentation/service/)

-  [Options](https://firewalld.org/documentation/service/options.html)
-  [Examples](https://firewalld.org/documentation/service/examples.html)

-  [IPSet](https://firewalld.org/documentation/ipset/)

-  [Options](https://firewalld.org/documentation/ipset/options.html)
-  [Examples](https://firewalld.org/documentation/ipset/examples.html)

-  [Helper](https://firewalld.org/documentation/helper/)

-  [Options](https://firewalld.org/documentation/helper/options.html)
-  [Examples](https://firewalld.org/documentation/helper/examples.html)

-  [ICMP Type](https://firewalld.org/documentation/icmptype/)

-  [Options](https://firewalld.org/documentation/icmptype/options.html)
-  [Examples](https://firewalld.org/documentation/icmptype/examples.html)

-  [Direct Interface](https://firewalld.org/documentation/direct/)

-  [Options](https://firewalld.org/documentation/direct/options.html)
-  [Examples](https://firewalld.org/documentation/direct/examples.html)

-  [HowTo](https://firewalld.org/documentation/howto/)

-  [Enable and Disable firewalld](https://firewalld.org/documentation/howto/enable-and-disable-firewalld.html)
-  [Get firewalld State](https://firewalld.org/documentation/howto/get-firewalld-state.html)
-  [Reload firewalld](https://firewalld.org/documentation/howto/reload-firewalld.html)
-  [Open a Port or Service](https://firewalld.org/documentation/howto/open-a-port-or-service.html)
-  [Add a Service](https://firewalld.org/documentation/howto/add-a-service.html)
-  [Debug firewalld](https://firewalld.org/documentation/howto/debug-firewalld.html)

-  [Manual Pages](https://firewalld.org/documentation/man-pages/)

-  [firewalld(1)](https://firewalld.org/documentation/man-pages/firewalld.html)
-  [firewall-cmd(1)](https://firewalld.org/documentation/man-pages/firewall-cmd.html)
-  [firewall-offline-cmd(1)](https://firewalld.org/documentation/man-pages/firewall-offline-cmd.html)
-  [firewall-config(1)](https://firewalld.org/documentation/man-pages/firewall-config.html)
-  [firewall-applet(1)](https://firewalld.org/documentation/man-pages/firewall-applet.html)
-  [firewalld.conf(5)](https://firewalld.org/documentation/man-pages/firewalld.conf.html)
-  [firewalld.zones(5)](https://firewalld.org/documentation/man-pages/firewalld.zones.html)
-  [firewalld.zone(5)](https://firewalld.org/documentation/man-pages/firewalld.zone.html)
-  [firewalld.policies(5)](https://firewalld.org/documentation/man-pages/firewalld.policies.html)
-  [firewalld.policy(5)](https://firewalld.org/documentation/man-pages/firewalld.policy.html)
-  [firewalld.policy-sets(5)](https://firewalld.org/documentation/man-pages/firewalld.policy-sets.html)
-  [firewalld.policy-set-gateway(5)](https://firewalld.org/documentation/man-pages/firewalld.policy-set-gateway.html)
-  [firewalld.service(5)](https://firewalld.org/documentation/man-pages/firewalld.service.html)
-  [firewalld.ipset(5)](https://firewalld.org/documentation/man-pages/firewalld.ipset.html)
-  [firewalld.helper(5)](https://firewalld.org/documentation/man-pages/firewalld.helper.html)
-  [firewalld.icmptype(5)](https://firewalld.org/documentation/man-pages/firewalld.icmptype.html)
-  [firewalld.richlanguage(5)](https://firewalld.org/documentation/man-pages/firewalld.richlanguage.html)
-  [firewalld.direct(5)](https://firewalld.org/documentation/man-pages/firewalld.direct.html)
-  [firewalld.lockdown-whitelist(5)](https://firewalld.org/documentation/man-pages/firewalld.lockdown-whitelist.html)
-  [firewalld.dbus(5)](https://firewalld.org/documentation/man-pages/firewalld.dbus.html)

-  [External Resources](https://firewalld.org/documentation/external.html)
-  [Working With The Source](https://firewalld.org/documentation/working-with-the-source.html)
