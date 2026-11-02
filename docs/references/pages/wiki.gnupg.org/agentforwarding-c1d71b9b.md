---
url: "https://wiki.gnupg.org/AgentForwarding"
title: "AgentForwarding - GnuPG wiki"
captured: "2026-11-02T18:49:26Z"
via: "live"
sha256: "1825a2ed3ae94f10c5f614944923614263308c2eae72dd2f58f0d3645716e0ee"
wayback: "https://web.archive.org/web/20260417040653/https://wiki.gnupg.org/AgentForwarding"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# AgentForwarding - GnuPG wiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.gnupg.org/AgentForwarding>
> Snapshot: <https://web.archive.org/web/20260417040653/https://wiki.gnupg.org/AgentForwarding>

https://wiki.gnupg.org/Index

-  [Comments](https://wiki.gnupg.org/AgentForwarding#)
-  Immutable Page
-

-   [Menu](https://wiki.gnupg.org/AgentForwarding#)

- Navigation
- [RecentChanges](https://wiki.gnupg.org/RecentChanges)
- [FindPage](https://wiki.gnupg.org/FindPage)
- [Local Site Map](https://wiki.gnupg.org/AgentForwarding?action=LocalSiteMap)
-
- Help
- [HelpContents](https://wiki.gnupg.org/HelpContents)
- [HelpOnMoinWikiSyntax](https://wiki.gnupg.org/HelpOnMoinWikiSyntax)
-
- Display
- [Attachments](https://wiki.gnupg.org/AgentForwarding?action=AttachFile)
- [Info](https://wiki.gnupg.org/AgentForwarding?action=info)
- [Raw Text](https://wiki.gnupg.org/AgentForwarding?action=raw)
- [Print View](https://wiki.gnupg.org/AgentForwarding?action=print)
-
- Edit
- [Load](https://wiki.gnupg.org/AgentForwarding?action=Load)
- [Save](https://wiki.gnupg.org/AgentForwarding?action=Save)

-  [Login](https://wiki.gnupg.org/AgentForwarding?action=login)

#### Navigation

- [RecentChanges](https://wiki.gnupg.org/RecentChanges)
- [FindPage](https://wiki.gnupg.org/FindPage)
- [HelpContents](https://wiki.gnupg.org/HelpContents)

- [AgentForwarding](https://wiki.gnupg.org/AgentForwarding)

Last updated at 2024-06-08 16:35:42

## Forwarding gpg-agent to a remote system over SSH

[GnuPG](https://wiki.gnupg.org/GnuPG) 2.1 enables you to forward the [GnuPG](https://wiki.gnupg.org/GnuPG)-Agent to a remote system. That means that you can keep your secret keys on a local machine (or even a hardware token like a smartcard or on a [GNUK](https://github.com/me21/gnuk)).

You need at least [GnuPG](https://wiki.gnupg.org/GnuPG) 2.1.1 on both systems.

### GnuPG configuration

For really old versions of [GnuPG](https://wiki.gnupg.org/GnuPG) (< 2.1.17) you need to edit your gpg-agent.conf to configure an extra socket. The extra socket is more restricted then the normal socket and Pinentry messages will differ when gpg-agent is accessed over this socket:

```
extra-socket /home/<user>/.gnupg/S.gpg-agent.extra
```

Replace <user> with your actual username. Current versions of [GnuPG](https://wiki.gnupg.org/GnuPG) create the socket by default.

#### GnuPG on the remote system

It is important to note that to work properly [GnuPG](https://wiki.gnupg.org/GnuPG) on the remote system still needs your public keys. So you have to make sure they are available on the remote system even if your secret keys are not.

### SSH Configuration

Note: With [GnuPG](https://wiki.gnupg.org/GnuPG) >= 2.1.13 the location of the agents socket changed. To find out the name of the local socket, always use:

```
gpgconf --list-dir agent-extra-socket
```

this is the name of the extra socket on the local box. On the server the standard socket needs to be configured. Find this one out with

```
gpgconf --list-dir agent-socket
```

#### OpenSSH >= 6.7

To your ~/.ssh/config you can add:

```
Host gpgtunnel
  HostName server.domain
  RemoteForward <socket_on_remote_box> <extra_socket_on_local_box>
```

If you can modify the the remote machine settings you should put into /etc/ssh/sshd_config the following:

```
StreamLocalBindUnlink yes
```

This enables automatic removal of stale sockets when connecting to the remote machine. Otherwise you will first have to remove the socket on the remote machine before forwarding works.

With StreamLocalBindUnlink yes you can connect with ssh gpgtunnel and just use [GnuPG](https://wiki.gnupg.org/GnuPG) as usual.

Note: On Systems where systemd controls the directories under /var/run/user/<uid> it may be that the socket forwarding fails because /var/run/user/<uid>/gnupg is deleted on logout. To workaround this you can put gpgconf --create-socketdir in the startup script of your shell e.g. ~/.bashrc or ~/.zshrc.

Remote gpg will try to start gpg-agent if it's not running. Remote gpg-agent which will delete your forwarded socket and set up it's own. To avoid this you can pass --no-autostart to remote gpg command.

#### OpenSSH < 6.7

Before OpenSSH 6.7 you need to use socat which is a bit more fragile and requires a loop to stay open.

```
# Execute locally
(while true; do
socat TCP-LISTEN:16668,bind=127.0.0.1 UNIX-CONNECT:$HOME/.gnupg/S.gpg-agent.extra;
done) &

# Connect to the remote system with Port forwarding.
ssh -R16668:localhost:16668 server.domain

# Connect the socket on the remote system
(while true; do
    socat UNIX-LISTEN:$HOME/.gnupg/S.gpg-agent,unlink-close,unlink-early TCP4:localhost:16668;
done) &
```

AgentForwarding (last edited 2024-06-08 16:35:42 by [lucasfcnunes](https://wiki.gnupg.org/lucasfcnunes))

- This site is hosted by [Intevation GmbH](https://intevation.de/)
- |
- [Datenschutzerklärung und Impressum](https://intevation.de/impressum-datenschutz)
- |
- [Privacy Policy and Imprint](https://intevation.de/imprint-privacy-policy)
