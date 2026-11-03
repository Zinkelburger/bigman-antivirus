---
url: "https://discourse.ubuntu.com/t/sshd-now-uses-socket-based-activation-ubuntu-22-10-and-later/30189"
title: "SSHd now uses socket-based activation (Ubuntu 22.10 and later)"
site: "Ubuntu Discourse"
published: "2022-08-24T02:55:24+00:00"
captured: "2026-11-03T02:57:21Z"
via: "live"
sha256: "796568226f32c2907c1be8f3d3d1ec6b798f9aa5ef94396f9830b24f965cee8b"
wayback: "https://web.archive.org/web/20261024002931/https://discourse.ubuntu.com/t/sshd-now-uses-socket-based-activation-ubuntu-22-10-and-later/30189"
cited_in:
  - "docs/research/hardening.md"
---

# SSHd now uses socket-based activation (Ubuntu 22.10 and later)

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://discourse.ubuntu.com/t/sshd-now-uses-socket-based-activation-ubuntu-22-10-and-later/30189>
> Snapshot: <https://web.archive.org/web/20261024002931/https://discourse.ubuntu.com/t/sshd-now-uses-socket-based-activation-ubuntu-22-10-and-later/30189>

[Ubuntu Discourse](https://discourse.ubuntu.com/)

#  [SSHd now uses socket-based activation (Ubuntu 22.10 and later)](https://discourse.ubuntu.com/t/sshd-now-uses-socket-based-activation-ubuntu-22-10-and-later/30189)

[Project Discussion](https://discourse.ubuntu.com/c/project/307)    [Server](https://discourse.ubuntu.com/c/project/server/17)

[documentation](https://discourse.ubuntu.com/tag/documentation)

[vorlon](https://discourse.ubuntu.com/u/vorlon)     August 24, 2022, 2:55am   1

As of version 1:9.0p1-1ubuntu1 of openssh-server in Kinetic Kudu (Ubuntu 22.10), OpenSSH in Ubuntu is configured by default to use systemd socket activation. This means that sshd will not be started until an incoming connection request is received. This has been done to reduce the memory consumed by Ubuntu Server instances by default, which is of particular interest with Ubuntu running in VMs or LXD containers: by not running sshd when it is not used, we save at least 3MiB of memory in each instance, representing a savings of roughly 5% on an idle, pristine kinetic container.

At Canonical we care about making Ubuntu as efficient as possible on your hardware and in the cloud, which is why this change has been landed as part of a larger effort to reduce the default memory footprint of our images. A default Ubuntu 22.04 LXD image at release time used 65MiB of RAM, which in kinetic now uses 58MiB after this OpenSSH change; and more improvements are in progress, with the intention of backporting the safer changes to our Ubuntu 22.04 images to improve memory usage for the greatest number of users.

On new installs of Ubuntu 22.10 or later, the OpenSSH change in behavior should be completely transparent to users.

In Ubuntu 22.10, Ubuntu 23.04, and Ubuntu 23.10, on upgrade users who had configured Port settings or a ListenAddress setting in /etc/ssh/sshd_config will find these settings migrated to /etc/systemd/system/ssh.socket.d/addresses.conf. (As an exception, if more than one ListenAddress setting is declared, the configuration is not migrated because systemd’s ListenStream has different semantics: any address configured which is not present at boot time would cause the ssh.socket unit to not start. Because it is not possible to reliably determine at upgrade time whether ssh.socket could fail to start on reboot, if you have more than one ListenAddress configured, your system will not be migrated to socket-based activation but instead the daemon will be started on boot as before.)

In Ubuntu 24.04 LTS these settings are no longer migrated, but the port and address settings are pulled dynamically from sshd.conf via a systemd generator.

Socket activation is recommended wherever possible, but if for any reason you find after migration that this is incompatible with your configuration, it is still possible to revert to the previous non-socket-activated behavior by running:

```
systemctl disable --now ssh.socket
rm -f /etc/systemd/system/ssh.service.d/00-socket.conf
rm -f /etc/systemd/system/ssh.socket.d/addresses.conf
systemctl daemon-reload
systemctl enable --now ssh.service

```

9 Likes

[Ubuntu 24.04 LTS (Noble Numbat) Release Notes](https://discourse.ubuntu.com/t/ubuntu-24-04-lts-noble-numbat-release-notes/39890)

[Kinetic Kudu Release Notes](https://discourse.ubuntu.com/t/kinetic-kudu-release-notes/27976)

[Foundations Team Updates - Thursday 25 August 2022](https://discourse.ubuntu.com/t/foundations-team-updates-thursday-25-august-2022/30190/2)

[Error drag and drop files](https://discourse.ubuntu.com/t/error-drag-and-drop-files/53055/4)

[Systemd shows all services as dead](https://discourse.ubuntu.com/t/systemd-shows-all-services-as-dead/52660/5)

[Enable/Start service from autoinstall/cloud-init?](https://discourse.ubuntu.com/t/enable-start-service-from-autoinstall-cloud-init/50186/20)

[paride](https://discourse.ubuntu.com/u/paride)    August 24, 2022, 12:33pm   2

Works well for me. One thing I verified is that the hosts_access(5) (libwrap) remains the same: it does. I do have a couple of suggestions:

- We still ship sshd_config with #ListenAddress and #Port lines, with no warning about them not being honored anymore by default. Users editing those will be surprised that they have no effect. I think they should be replaced or complemented by a comment explaining the new way for specifying the listen address and port (maybe by pointing to openssh-server.README.Debian, see below). I see that this requires patching upstream’s sshd_config, which is a bit annoying.
- There’s useful info about socket-based activation in README.Debian, but I initially failed to find it because it gets installed by the openssh-client package, as it’s the first package specified in d/control. Consider moving the socket activation bits to d/openssh-server.README.Debian in the source package.

TIL about sd_listen_fds(3)!

4 Likes

[vorlon](https://discourse.ubuntu.com/u/vorlon)    August 26, 2022, 12:31am   3

Thanks for the feedback. It was intended to document in the default sshd_config that these options are unused, but that didn’t get done yet. I’ll make sure that’s handled in the next upload.

As far as finding README.Debian, openssh-server has a strict versioned dependency on openssh-client because /usr/share/doc/openssh-server is a symlink to /usr/share/doc/openssh-client - so I’m not sure why this prevented you from finding it?

1 Like

[paride](https://discourse.ubuntu.com/u/paride)    August 26, 2022, 9:44am   4

As I was inspecting the package more in general I had a look at the files installed by openssh-server and didn’t see README.Debian. This is not what users will (and should) normally do, I think the current approach is good, thanks for pointing me to it.

1 Like

[phil-hughes](https://discourse.ubuntu.com/u/phil-hughes)    October 21, 2022, 1:47pm   6

What will happen if there are Port or ListenAddress entries in an Include file eg in /etc/ssh/sshd_config.d? Thanks

[vorlon](https://discourse.ubuntu.com/u/vorlon)    October 21, 2022, 2:45pm   7

the upgrade code understands and follows Include directives.

2 Likes

[yanchenksea](https://discourse.ubuntu.com/u/yanchenksea)    October 22, 2022, 9:44am   8

Hello,
So in simple terms, how can we change the ssh port on a new install of Ubuntu 22.10?

(I cannot find the right way to do it, after editing the port in /lib/systemd/system/ssh.socket, the port ssh.socket is listening to changed as excepted and it starts successfully, however whenever I try to SSH, ssh.socket on the server side will fail immediately and refuse connections.)

[saxl](https://discourse.ubuntu.com/u/saxl)    October 22, 2022, 1:35pm   9

in README.Debian you write

```
mkdir -p /etc/systemd/system/ssh.socket.d
cat >/etc/systemd/system/ssh.socket.d/listen.conf <<EOF
[Socket]
ListenStream=2222
EOF

```

this makes sshd listen on port 2222 and port 22.
I think you should mention that for sshd to listen n 2222 only you need to do this:

```
mkdir -p /etc/systemd/system/ssh.socket.d
cat >/etc/systemd/system/ssh.socket.d/listen.conf <<EOF
[Socket]
ListenStream=
ListenStream=2222
EOF

```

quote from man systemd.socket

```
These options may be specified more than once, in which case incoming traffic on any of the sockets will trigger service activation, and all listed sockets will be passed to the service,
regardless of whether there is incoming traffic on them or not. If the empty string is assigned to any of these options, the list of addresses to listen on is reset, all prior uses of any
of these options will have no effect.

```

2 Likes

[twinsen](https://discourse.ubuntu.com/u/twinsen)    October 23, 2022, 9:52pm   10

Long story short - for all those who do not like this change:

systemctl disable --now ssh.socket
systemctl enable --now ssh.service

As per: /usr/share/doc/openssh-server/README.Debian.gz
Then the /etc/ssh/sshd_config works again with Ports and Addresses settings.

3 Likes

[saxl](https://discourse.ubuntu.com/u/saxl)    October 24, 2022, 6:38am   11

in some cases when updating /etc/systemd/system/ssh.service.d/00-socket.conf is created.
This file forces the use of ssh.socket. You might also check the existence of this file and delete it followed by a systemctl daemon-reload

2 Likes

[nebulabox](https://discourse.ubuntu.com/u/nebulabox)    January 12, 2023, 4:51am   13

This solution works fine, thanks.

[michael.utech](https://discourse.ubuntu.com/u/michael.utech)    January 28, 2023, 6:17pm   14

I’m trying to bind to an IP address assigned by DHCP by mac and that fails, apparently the socket is created before the IP is set up.

I want to exclude all but this interface from being served sshd. How am I going about that with ssh.socket?

What is the benefit of ssh.socket? The price is that ssh configuration is now split between the actual ssh daemon and systemd. I rarely read ssh documentation or even the comments in sshd_config anymore and was just baffled first that my settings are ignored, then that there was not /etc/systemd/system/ssh.socket.d but an ssh.service.d that apparently was not used. ssh.socket.d does of course not use sshd_config syntax. The package is named ssh, the documentation is named openssh. I had to find out that the actual documentation is in systemd.socket. All that was a bit annoying, since I had something else to accomplish and just wanted to log into that box.

It sounds like in the near future, ssh.service will be deprecated or somehow disabled, so I’m not sure if it’s a good idea to just ignore it and revert to ssh.service.

There must be some substantial benefit of using the socket mechanism? I didn’t find any mention anywhere what that in the case of sshd might be…

[piradix](https://discourse.ubuntu.com/u/piradix)    April 5, 2023, 3:11pm   16

The problem with not using “/etc/ssh/sshd_config” is that if you stop sshd (sshd stop service), and start it (sshd start service) it takes the port from the “/etc/ssh/sshd_config” configuration file, and not from “/etc/systemd/system/ssh.socket.d/listen.conf”!
For me this is not a very good solution because it generates human errors!
A small comment in the config file on “#Port” would be a minimum and welcome
If you like a comment … to avoid the admin to waste their time

1 Like

[hrudy](https://discourse.ubuntu.com/u/hrudy)    April 19, 2023, 8:20pm   17

I am still wondering why Ubuntu would change something that has been working since the last century. For me the fixes didn’t work until I reverted back to the old configuration. So for me, this is 3 strikes.
1 strike for changing it in the first place.
2nd for the socket-based fix not working
3rd strike for inadequate documentation.
I did a search as to why socket based activation is “superior”. Supposedly it is there to minimize memory usage. Allegedly this reduces memory from 65MiB to 58Mib. I guess if this makes the ppl at Canonical happy but even a Raspberry PI 3B has 1 GB of memory.

2 Likes

[dsembry](https://discourse.ubuntu.com/u/dsembry)    April 23, 2023, 4:02am   18

I’m having similar issues with sshd on my ubuntu 22.10 installation. I have followed the instructions in /usr/share/doc/openssh-server/README.Debian.gz:

```
  systemctl disable --now ssh.socket
  systemctl enable --now ssh.service

```

however, as the outputs of a few different commands I’ve typed show, ssh.socket is still running and systemd is still listening to port 22.

```
root@bournemouth:/usr/share/doc/openssh-client# systemctl status ssh.service
● ssh.service - OpenBSD Secure Shell server
     Loaded: loaded (/lib/systemd/system/ssh.service; enabled; preset: enabled)
    Drop-In: /etc/systemd/system/ssh.service.d
             └─00-socket.conf
     Active: active (running) since Sat 2023-04-22 23:42:40 EDT; 10min ago
TriggeredBy: ● ssh.socket
...

root@bournemouth:/usr/share/doc/openssh-client# systemctl status ssh.socket
● ssh.socket - OpenBSD Secure Shell server socket
     Loaded: loaded (/lib/systemd/system/ssh.socket; disabled; preset: enabled)
     Active: active (running) since Sat 2023-04-22 23:42:40 EDT; 10min ago
      Until: Sat 2023-04-22 23:42:40 EDT; 10min ago
   Triggers: ● ssh.service
     Listen: [::]:22 (Stream)
      Tasks: 0 (limit: 9365)
     Memory: 4.0K
        CPU: 1ms
     CGroup: /system.slice/ssh.socket
...

root@bournemouth:/usr/share/doc/openssh-client# lsof -iTCP -sTCP:LISTEN
COMMAND     PID            USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
systemd       1            root   60u  IPv6 214824      0t0  TCP *:ssh (LISTEN)
systemd       1            root  345u  IPv4  17890      0t0  TCP *:sunrpc (LISTEN)
systemd       1            root  347u  IPv6  17891      0t0  TCP *:sunrpc (LISTEN)
rpcbind     685            _rpc    4u  IPv4  17890      0t0  TCP *:sunrpc (LISTEN)
rpcbind     685            _rpc    6u  IPv6  17891      0t0  TCP *:sunrpc (LISTEN)
systemd-r   687 systemd-resolve   14u  IPv4  24694      0t0  TCP localhost:domain (LISTEN)
systemd-r   687 systemd-resolve   16u  IPv4  24696      0t0  TCP localhost:domain (LISTEN)
cupsd      5583            root    7u  IPv6  70007      0t0  TCP ip6-localhost:ipp (LISTEN)
cupsd      5583            root    8u  IPv4  70008      0t0  TCP localhost:ipp (LISTEN)
sshd      12927            root    3u  IPv6 214824      0t0  TCP *:ssh (LISTEN)

```

[vorlon](https://discourse.ubuntu.com/u/vorlon)    April 23, 2023, 4:06pm   19

Sorry, it appears the documenation became inaccurate once we added /etc/systemd/system/ssh.service.d/00-socket.conf. If you remove this file (or rename it to be inactive) and then run the other two commands, it works as expected. Opened https://bugs.launchpad.net/ubuntu/+source/openssh/+bug/2017434 to track resolution of the documentation bug.

[jpf](https://discourse.ubuntu.com/u/jpf)    July 5, 2023, 6:51am   20

Sorry if I’m missing the obvious, but documentation in Ubuntu in general is so terrible to find…

After changing the port in /etc/systemd/system/ssh.socket.d, what do I need to do to make it take effect ? (without rebooting the machine that is)

[jpf](https://discourse.ubuntu.com/u/jpf)    July 5, 2023, 7:07am   21

According to /usr/share/doc/openssh-server/README.Debian.gz (again, documetnation is spread everywhere, it’s hours to find anything), to make it pick up on the new address should require systemctl daemon-reload.

Except it doesn’t work … nothing I do seems to make it listen to my selected port

Disabling the system socket thing makes it work:

```
systemctl disable --now ssh.socket
systemctl enable --now ssh.service

```

Now it is listening to the new port.

1 Like

[jpf](https://discourse.ubuntu.com/u/jpf)    July 5, 2023, 7:22am   22

A bit more searching and [this stack overflow question](https://askubuntu.com/questions/1439461/ssh-default-port-not-changing-ubuntu-22-10) answered it:

I needed to add and extra “ListenAddress=” line, with an empty address !? Yes … those undocumented mysteries … But then it works.

1 Like

[geenat](https://discourse.ubuntu.com/u/geenat)    July 17, 2023, 12:05pm   23

Breaks SSH access for anyone using /etc/ssh/sshd_config on Ubuntu 22.10, 23.04, 23.10…

RIP those who block Port 22. Hope ya’ll had backups or physical access.

[@vorlon](https://discourse.ubuntu.com/u/vorlon)
NEEDS to be added to the OP… make this google-able.

To go back to previous SSH behavior:

- sudo rm /etc/systemd/system/ssh.service.d/00-socket.conf
- systemctl disable --now ssh.socket
- systemctl enable --now ssh.service
- sudo systemctl daemon-reload
- sudo systemctl restart ssh

Thanks

3 Likes

[next page →](https://discourse.ubuntu.com/t/sshd-now-uses-socket-based-activation-ubuntu-22-10-and-later/30189?page=2)
