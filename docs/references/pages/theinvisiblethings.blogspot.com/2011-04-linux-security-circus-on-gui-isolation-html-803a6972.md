---
url: "https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html"
title: "The Linux Security Circus: On GUI isolation"
captured: "2026-11-02T18:46:33Z"
via: "live"
sha256: "6a9e16dff47057be8bb28179b0c4bf6fb9526657202b4ccfea304658322055ed"
wayback: "https://web.archive.org/web/20261026000428/https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# The Linux Security Circus: On GUI isolation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html>
> Snapshot: <https://web.archive.org/web/20261026000428/https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html>

[skip to main](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html#main) | [skip to sidebar](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html#sidebar)

#  [The Invisible Things Lab's blog](https://theinvisiblethings.blogspot.com/)

Kernel, Hypervisor, Virtualization, Trusted Computing and other system-level security stuff

## Saturday, April 23, 2011

###  The Linux Security Circus: On GUI isolation

There certainly is one thing that most Linux users don't realize about their Linux systems... this is the lack of GUI-level isolation, and how it essentially nullifies all the desktop security. I wrote about it a few times, I spoke about it a few times, yet I still come across people who don't realize it all the time.

So, let me stress this one more time: if you have two GUI applications, e.g. an OpenOffice Word Processor, and a stupid Tetris game, both of which granted access to your screen (your X server), then there is no isolation between those two apps. Even if they run as different user accounts! Even if they are somehow sandboxed by SELinux or whatever! None, zero, null, nil!

The X server architecture, designed long time ago by some happy hippies who just thought all the people apps are good and non-malicious, simply allows any GUI application to control any other one. No bugs, no exploits, no tricks, are required. This is all by design. One application can sniff or inject keystrokes to another one, can take snapshots of the screen occupied by windows belonging to another one, etc.

If you don't believe me, I suggest you do a simple experiment. Open a terminal window, as normal user, and run xinput list, which is a standard diagnostic program for Xorg (on Fedora you will likely need to install it first: yum install xorg-x11-apps):

$ xinput list

It will show you all the pointer and keyboard devices that your Xorg knows about. Note the ID of the device listed as “AT keyboard” and then run (as normal user!):

$ xinput test id

It should now start displaying the scancodes for all the keys you press on the keyboard. If it doesn't, it means you used a wrong device ID.

Now, for the best, start another terminal window, and switch to root (e.g. using su, or sudo). Notice how the xinput running as user is able to sniff all your keystrokes, including root password (for su), and then all the keystrokes you enter in your root session. Start some GUI app as root, or as different user, again notice how your xinput can sniff all the keystrokes you enter to this other app!

Yes, I can understand what is happening in your mind and heart right now... Don't worry, others have also passed through it. Feel free to hate me, throw out insults at me, etc. I don't mind, really (I just won't moderate them). When you calm down, continue reading.

In Qubes the above problem doesn't exist, because each domain (each AppVM) has it own local, isolated, dummy X server. The main X server, that runs in Dom0 and that handles the real display is never exposed to any of the AppVMs directly (AppVMs cannot connect to it via the X protocol). For details see this [technical overview](http://wiki.qubes-os.org/trac/wiki/GUIdocs).

You can repeat the same experiment in Qubes. You just need to use the ID of the “qubesdev” device, as shown by xinput list (should be 7). Run the xinput in one of your domains, e.g. in the “red” one. Because we actually use the same device for both mouse and keystrokes, you should now see both the key scancodes, as well as all the mouse events. Notice how your xinput is able to sniff all the events that are destined for other apps belonging to the same domain where you run xinput, and how it is unable to sniff anything targeted to other domains, or Dom0.

BTW, Windows is the only one mainstream OS I'm aware of, that actually attempts to implement some form of GUI-level isolation, starting from Windows Vista. See e.g. this [ancient article](http://theinvisiblethings.blogspot.com/2007/02/running-vista-every-day.html) I wrote in the days when I used Vista at my primary laptop. Of course, it's still easy to bypass this isolation, because of the huge interface that is exposed to each GUI client (that also includes GPU API). Nevertheless, they at least attempt to prevent this at the architecture level.

Posted by   [Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141)    at  [Saturday, April 23, 2011](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html)

Labels: [general](https://theinvisiblethings.blogspot.com/search/label/general), [os security](https://theinvisiblethings.blogspot.com/search/label/os%20security)

#### 27 comments:

Anonymous said...

Try this with SELinux sandbox... "sandbox -X xterm" (on F14 or RHEL6... install policycoreutils-sandbox if needed). It does not see keys destined for another window. This is because sandbox -X also uses its own X server (Xephyr).

How does this differ from Qubes?

[April 23, 2011 5:57 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303574236988#c7925540745482403781)  https://www.blogger.com/comment/delete/24586388/7925540745482403781

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

The primary problem with SELinux sandbox is that it still relies on the big, buggy, bloated Linux kernel to enforce security. In Qubes we rely on Xen to enforce domain isolation (a few orders of magnitude less code, I would say 10^3 less, even including the few daemons/backends we also have in Dom0 and which are reachable from VMs).

Second, SELinux sandbox X server isolation doesn't really eliminate all the attacks coming over the bloated X protocol. As a result it is relatively easy to attack. See, e.g. this paper from Rafal for example of an escape:
http://www.invisiblethingslab.com/resources/misc-2010/xorg-large-memory-attacks.pdf

In Qubes the only point of contact to the real server is our GUI daemon -- see the reference quoted in the blog post for details on the protocol we use. Compare this now to the X protocol. It's orders of magnitude simpler!

SELinux sandbox creates an X server instance (Xephyr) per each sandboxed app, which sounds to me like a huge resource waste. In Qubes we have one X server instance per *domain*.

SELinux sandbox doesn't offer any desktop integration, such as secure copy-and-paste operation. We provide secure inter-domain clipboard on Qubes -- again see the above-quoted reference for technical details.

Finally, SELinux sandbox requires the user to setup complex SELinux polices to achieve also filesystem-level and network-level isolation. In Qubes all the domains are automatically isolated on all those levels.

[April 23, 2011 6:21 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303575708573#c8629263254767793603)  https://www.blogger.com/comment/delete/24586388/8629263254767793603

rinaku said...

I would be interested to know what you think about capability-based security (as in the Capsicum project), and how it compares to what you do in Qubes.

From what I understand (but I know very little about security), capabilities allow the same level of security as Qubes, or even better, but they need the applications to be modified, which seems unrealistic. Is that correct ?

Anyway, I'm not good enough to beta test, but I can't wait for a stable release, to see what it is like to use a system like Qubes.

[April 23, 2011 9:38 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303587485071#c491117241739308124)  https://www.blogger.com/comment/delete/24586388/491117241739308124

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

@rinaku: Capsicum currently doesn't support GUI-level isolation, but I think they're planning to add it at some point in time. When this happens we would seriously consider using it inside our domains, to bring some isolation into each domain as well (currently there is no isolation between apps running in the same domain, specifically for reasons given in the blog article above).

Capability-based systems, as you pointed out, require application modifications. This also means they cannot protect against intentionally malicious apps. Thus I see them as complementing Qubes, but never replacing it. Also, note that capsicum works only on app-level, but cannot protect against e.g. network subsystem compromises (cannot isolate the networking subsystem or other drivers). In Qubes we can easily do that e.g. via NetVM. Another reason to think about it as something to complement Qubes (and to run it inside a domain).

[April 23, 2011 10:32 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303590755991#c8822757024315624134)  https://www.blogger.com/comment/delete/24586388/8822757024315624134

Anonymous said...

Welcome to 1987.

[April 24, 2011 12:36 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303598183621#c1988089403282271590)  https://www.blogger.com/comment/delete/24586388/1988089403282271590

Anonymous said...

That is something I didn't know about... but very interesting. Good Post.

[April 24, 2011 1:17 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303600673309#c8065664697086926986)  https://www.blogger.com/comment/delete/24586388/8065664697086926986

Anonymous said...

Why can't we just fix/modify the X-Server to make GUI level isolation easier?

[April 24, 2011 1:36 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303601774527#c5625637594532577248)  https://www.blogger.com/comment/delete/24586388/5625637594532577248

Anonymous said...

How old/young has somebody to be to write "....designed long time ago by some happy hippies who just thought all the people apps are good and non-malicious..." ?

The problem is the other way around. These guys created some awesome software tailored to their problems and means. Using it 20 years later in a different environment/world is the fault, not their initial design.

Blame the actual generation, respect and praise the founder.

[April 24, 2011 4:44 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303613080744#c5646459249784692744)  https://www.blogger.com/comment/delete/24586388/5646459249784692744

https://www.blogger.com/profile/03245280252760734079

[Ashish](https://www.blogger.com/profile/03245280252760734079) said...

Yes the same hippies who built X11 before you were born....

Please read the man page on Xsecurity
http://www.manpagez.com/man/7/Xsecurity/

While you are at it also read up on xhost, xauth.
Linux is not representative of how X11 really is supposed to work.

[April 24, 2011 6:32 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303619539766#c3460192292039504741)  https://www.blogger.com/comment/delete/24586388/3460192292039504741

https://www.blogger.com/profile/16054408183511995721

[josericardo](https://www.blogger.com/profile/16054408183511995721) said...

Very interesting indeed. I didn't know about this threat and doesn't seem to be fixable. Nevertheless I wanted to know if this is the case for wayland. It would be interesting to run your experiment in it and post the findings on the wayland-devel mailing list.

[April 24, 2011 6:36 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303619793703#c6857748340016988148)  https://www.blogger.com/comment/delete/24586388/6857748340016988148

https://www.blogger.com/profile/02537508544588815164

[Unknown](https://www.blogger.com/profile/02537508544588815164) said...

So, how does Qubes implement global keyboard shortcuts or applications like VNC?

[April 24, 2011 9:26 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303629991144#c8080907047058724597)  https://www.blogger.com/comment/delete/24586388/8080907047058724597

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

@Ashish: the security mechanisms you mention have been designed for a totally different security model than the "desktop GUI security" that is prevalent today.

The point is, as correctly made by some other commenter, that X have been designed long, long, time ago, in different realities, for solving different problems, but the Linux community has blindly adopted it to current conditions, failing to see that it really doesn't fit today. Why do you think the title of this post is "*Linux* Security Circus" instead of "X Security Circus"? (Yes, I know, the problem also affects other OSes, such as e.g. *BSD, etc).

[April 24, 2011 10:27 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303633666563#c577377308571446544)  https://www.blogger.com/comment/delete/24586388/577377308571446544

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

@fliebel: In Qubes all the keystrokes are first processed byt he *trusted* window manger running in Dom0 (the trusted partition). Then, they might be consumed by Qubes (e.g. if this is a global clipboard management sequence, or perhaps the "Expose" sequence), or passed down to the *active* domain's local X server.

VNC is just a regular app running in one of the domains. No special VNC for Qubes is needed.

[April 24, 2011 10:30 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303633843478#c8263043946870583160)  https://www.blogger.com/comment/delete/24586388/8263043946870583160

http://www.phocean.net

[phocean](http://www.phocean.net) said...

Instructive article.
But what about Wayland?
Wayland is claimed to replace X soon. Major distros are already working on its integration.
Are the authors aware of it or does it have the same design flaws?

[April 24, 2011 11:00 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303635644720#c5654461243455067337)  https://www.blogger.com/comment/delete/24586388/5654461243455067337

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

@phocean: good question about Wayland, why don't you check for yourself? ;)

[April 24, 2011 11:05 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303635946395#c8984499246756001640)  https://www.blogger.com/comment/delete/24586388/8984499246756001640

http://www.phocean.net

[phocean](http://www.phocean.net) said...

So I will when I have some free time.
But as it is your research topic, I expected you had already done that or even contacted the developers to inform them.
As it is still under heavy development, I guess it is a good timing to implement good stuff and for expert like you to advise them.

[April 24, 2011 11:16 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303636609932#c1093062950805499047)  https://www.blogger.com/comment/delete/24586388/1093062950805499047

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

@phocean: Don't kid me, kid! What I did here was not a "research" (you might want to check some real research we did at ITL on our website). That was just a Saturday morning blog post with an aim to save some lost souls, like yours... And I really don't have time to go and test all the software in the development that is out there. I do have real job, that among other things includes doing *real* research...

[April 24, 2011 11:23 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303636993771#c7985145446220586547)  https://www.blogger.com/comment/delete/24586388/7985145446220586547

http://www.visucore.com/

[Wladimir](http://www.visucore.com/) said...

*they at least attempt to prevent this at the architecture level.*

I'm not sure that trying and failing is better than not trying, in the case of security :)

That said, I agree application-level isolation on desktop OS is a problem that deserves solving. It is very very hard, though.

As X is going the way of the dodo, have you examined the case with Unity?

[April 24, 2011 11:54 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303638882551#c8217744463156904513)  https://www.blogger.com/comment/delete/24586388/8217744463156904513

Tom said...

There is an extension called XACE that when integrated with selinux fixes these problems. I think it was developed by Eamon Walsh for the NSA.

I don't know how well that works, but it is clear that this is a problem for other people, too.

[April 24, 2011 12:45 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303641903546#c504985300681348565)  https://www.blogger.com/comment/delete/24586388/504985300681348565

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

@Tom: The XACE project seems to be dead for at least 3 years now...

http://www.selinuxproject.org/page/XACE

[April 24, 2011 12:50 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303642256918#c261738829769968110)  https://www.blogger.com/comment/delete/24586388/261738829769968110

Anonymous said...

I ran gksudo and its input was not collected by xinput test. What kind of magic is it doing?

[April 24, 2011 8:07 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303668459694#c644649620501650773)  https://www.blogger.com/comment/delete/24586388/644649620501650773

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

About gksudo: perhaps it somehow owns the input only to itself for the period of displying its prompt window? I bet you cannot use other apps until the window is present?

But that's probably irrelevant, because I bet that once you switch to a different user and start some apps as this new user, then xinput will be able to sniff all those keystrokes without any problem (and you could also easily write a program that could be injecting keystrokes to those apps).

[April 24, 2011 8:36 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303670194939#c2943170451670016594)  https://www.blogger.com/comment/delete/24586388/2943170451670016594

https://www.blogger.com/profile/06159367184391537599

[KimTjik](https://www.blogger.com/profile/06159367184391537599) said...

I don't ask you to waste time on an explanation; you could as well just direct me to a source of information. What I wonder is: what's the real danger of the current sloppy model?

My question is based on the fact that all network connected systems can be remotely compromised, with or without a window system. What priority does this in theory sane improvement get in relation to established precautions?

I understand it's difficult to objectively answer that, while you at the same time invest time in a project addressing described issue.

[April 24, 2011 8:56 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303671365180#c3012803107345955839)  https://www.blogger.com/comment/delete/24586388/3012803107345955839

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

@KimTijk: your assumption about easiness of exploiting any system that is connected to the network, regardless of what apps are running there, is simply flawed.

[April 24, 2011 8:59 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303671572725#c8211774198519194101)  https://www.blogger.com/comment/delete/24586388/8211774198519194101

Anonymous said...

Nice post and you're probably right, but your attitude needs some work kid.

Maybe it's a researcher thing.

[April 24, 2011 9:52 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303674763405#c4459856792553040797)  https://www.blogger.com/comment/delete/24586388/4459856792553040797

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

:)

[April 24, 2011 9:58 PM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303675127042#c6608512110260753044)  https://www.blogger.com/comment/delete/24586388/6608512110260753044

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141) said...

Correction: I was wrong about Xace being dead -- it has been apparently merged into the mainstream Xorg. Its just that... apparently nobody uses it, because it just not suited to solve the problems we talk about here. See e.g. this presentation from Dan Walsh (SELinux maintainer), slide #12:

http://people.fedoraproject.org/~dwalsh/SELinux/Presentations/sandbox.pdf

...which shows how he decided to use a nested X server instead of Xace to implement SELinux sandbox (see comments above), because... "Xace doesn't work" ;)

[April 25, 2011 11:23 AM](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html?showComment=1303723427007#c2194487890821167176)  https://www.blogger.com/comment/delete/24586388/2194487890821167176

[Post a Comment](https://www.blogger.com/comment/fullpage/post/24586388/1548152289459004392)

[Newer Post](https://theinvisiblethings.blogspot.com/2011/05/following-white-rabbit-software-attacks.html)   [Older Post](https://theinvisiblethings.blogspot.com/2011/04/why-us-password-revolution-wont-work.html)  [Home](https://theinvisiblethings.blogspot.com/)

Subscribe to: [Post Comments (Atom)](https://theinvisiblethings.blogspot.com/feeds/1548152289459004392/comments/default)

## About Me

https://www.blogger.com/profile/07657268181166351141

[Joanna Rutkowska](https://www.blogger.com/profile/07657268181166351141)

Founder of Invisible Things Lab, Qubes OS project lead.

[View my complete profile](https://www.blogger.com/profile/07657268181166351141)

## Links

- [twitter: @rootkovska](https://twitter.com/rootkovska)
- [Qubes Project](http://qubes-os.org/)
- [Invisible Things Lab](http://invisiblethingslab.com/)

## Blog Archive

-  ► [2015](https://theinvisiblethings.blogspot.com/2015/) (1)

-  ► [February](https://theinvisiblethings.blogspot.com/2015/02/) (1)

-  ► [2014](https://theinvisiblethings.blogspot.com/2014/) (6)

-  ► [November](https://theinvisiblethings.blogspot.com/2014/11/) (1)

-  ► [September](https://theinvisiblethings.blogspot.com/2014/09/) (1)

-  ► [August](https://theinvisiblethings.blogspot.com/2014/08/) (2)

-  ► [April](https://theinvisiblethings.blogspot.com/2014/04/) (1)

-  ► [January](https://theinvisiblethings.blogspot.com/2014/01/) (1)

-  ► [2013](https://theinvisiblethings.blogspot.com/2013/) (8)

-  ► [December](https://theinvisiblethings.blogspot.com/2013/12/) (1)

-  ► [November](https://theinvisiblethings.blogspot.com/2013/11/) (1)

-  ► [September](https://theinvisiblethings.blogspot.com/2013/09/) (1)

-  ► [August](https://theinvisiblethings.blogspot.com/2013/08/) (1)

-  ► [June](https://theinvisiblethings.blogspot.com/2013/06/) (1)

-  ► [March](https://theinvisiblethings.blogspot.com/2013/03/) (1)

-  ► [February](https://theinvisiblethings.blogspot.com/2013/02/) (2)

-  ► [2012](https://theinvisiblethings.blogspot.com/2012/) (8)

-  ► [December](https://theinvisiblethings.blogspot.com/2012/12/) (1)

-  ► [September](https://theinvisiblethings.blogspot.com/2012/09/) (2)

-  ► [July](https://theinvisiblethings.blogspot.com/2012/07/) (1)

-  ► [June](https://theinvisiblethings.blogspot.com/2012/06/) (1)

-  ► [March](https://theinvisiblethings.blogspot.com/2012/03/) (1)

-  ► [February](https://theinvisiblethings.blogspot.com/2012/02/) (1)

-  ► [January](https://theinvisiblethings.blogspot.com/2012/01/) (1)

-  ▼ [2011](https://theinvisiblethings.blogspot.com/2011/) (17)

-  ► [December](https://theinvisiblethings.blogspot.com/2011/12/) (2)

-  ► [September](https://theinvisiblethings.blogspot.com/2011/09/) (3)

-  ► [August](https://theinvisiblethings.blogspot.com/2011/08/) (1)

-  ► [June](https://theinvisiblethings.blogspot.com/2011/06/) (3)

-  ► [May](https://theinvisiblethings.blogspot.com/2011/05/) (3)

-  ▼ [April](https://theinvisiblethings.blogspot.com/2011/04/) (3)

- [The Linux Security Circus: On GUI isolation](https://theinvisiblethings.blogspot.com/2011/04/linux-security-circus-on-gui-isolation.html)
- [Why the US "password revolution" won't work](https://theinvisiblethings.blogspot.com/2011/04/why-us-password-revolution-wont-work.html)
- [Qubes Beta 1 has been released!](https://theinvisiblethings.blogspot.com/2011/04/qubes-beta-1-has-been-released.html)

-  ► [March](https://theinvisiblethings.blogspot.com/2011/03/) (2)

-  ► [2010](https://theinvisiblethings.blogspot.com/2010/) (15)

-  ► [December](https://theinvisiblethings.blogspot.com/2010/12/) (1)

-  ► [October](https://theinvisiblethings.blogspot.com/2010/10/) (1)

-  ► [September](https://theinvisiblethings.blogspot.com/2010/09/) (4)

-  ► [August](https://theinvisiblethings.blogspot.com/2010/08/) (2)

-  ► [July](https://theinvisiblethings.blogspot.com/2010/07/) (1)

-  ► [June](https://theinvisiblethings.blogspot.com/2010/06/) (1)

-  ► [May](https://theinvisiblethings.blogspot.com/2010/05/) (2)

-  ► [April](https://theinvisiblethings.blogspot.com/2010/04/) (2)

-  ► [January](https://theinvisiblethings.blogspot.com/2010/01/) (1)

-  ► [2009](https://theinvisiblethings.blogspot.com/2009/) (21)

-  ► [December](https://theinvisiblethings.blogspot.com/2009/12/) (1)

-  ► [October](https://theinvisiblethings.blogspot.com/2009/10/) (1)

-  ► [September](https://theinvisiblethings.blogspot.com/2009/09/) (2)

-  ► [August](https://theinvisiblethings.blogspot.com/2009/08/) (2)

-  ► [July](https://theinvisiblethings.blogspot.com/2009/07/) (2)

-  ► [June](https://theinvisiblethings.blogspot.com/2009/06/) (3)

-  ► [May](https://theinvisiblethings.blogspot.com/2009/05/) (1)

-  ► [March](https://theinvisiblethings.blogspot.com/2009/03/) (4)

-  ► [February](https://theinvisiblethings.blogspot.com/2009/02/) (2)

-  ► [January](https://theinvisiblethings.blogspot.com/2009/01/) (3)

-  ► [2008](https://theinvisiblethings.blogspot.com/2008/) (15)

-  ► [September](https://theinvisiblethings.blogspot.com/2008/09/) (3)

-  ► [August](https://theinvisiblethings.blogspot.com/2008/08/) (4)

-  ► [July](https://theinvisiblethings.blogspot.com/2008/07/) (2)

-  ► [May](https://theinvisiblethings.blogspot.com/2008/05/) (1)

-  ► [April](https://theinvisiblethings.blogspot.com/2008/04/) (4)

-  ► [March](https://theinvisiblethings.blogspot.com/2008/03/) (1)

-  ► [2007](https://theinvisiblethings.blogspot.com/2007/) (15)

-  ► [October](https://theinvisiblethings.blogspot.com/2007/10/) (2)

-  ► [August](https://theinvisiblethings.blogspot.com/2007/08/) (2)

-  ► [June](https://theinvisiblethings.blogspot.com/2007/06/) (1)

-  ► [May](https://theinvisiblethings.blogspot.com/2007/05/) (1)

-  ► [April](https://theinvisiblethings.blogspot.com/2007/04/) (2)

-  ► [March](https://theinvisiblethings.blogspot.com/2007/03/) (2)

-  ► [February](https://theinvisiblethings.blogspot.com/2007/02/) (3)

-  ► [January](https://theinvisiblethings.blogspot.com/2007/01/) (2)

-  ► [2006](https://theinvisiblethings.blogspot.com/2006/) (8)

-  ► [November](https://theinvisiblethings.blogspot.com/2006/11/) (1)

-  ► [October](https://theinvisiblethings.blogspot.com/2006/10/) (1)

-  ► [September](https://theinvisiblethings.blogspot.com/2006/09/) (1)

-  ► [August](https://theinvisiblethings.blogspot.com/2006/08/) (1)

-  ► [July](https://theinvisiblethings.blogspot.com/2006/07/) (1)

-  ► [June](https://theinvisiblethings.blogspot.com/2006/06/) (1)

-  ► [May](https://theinvisiblethings.blogspot.com/2006/05/) (2)

## Labels

[qubes](https://theinvisiblethings.blogspot.com/search/label/qubes)   [general](https://theinvisiblethings.blogspot.com/search/label/general)   [trusted computing](https://theinvisiblethings.blogspot.com/search/label/trusted%20computing)   [attack](https://theinvisiblethings.blogspot.com/search/label/attack)   [os security](https://theinvisiblethings.blogspot.com/search/label/os%20security)   [philosophical](https://theinvisiblethings.blogspot.com/search/label/philosophical)   [fighting for a better world](https://theinvisiblethings.blogspot.com/search/label/fighting%20for%20a%20better%20world)   [exploit](https://theinvisiblethings.blogspot.com/search/label/exploit)   [company news](https://theinvisiblethings.blogspot.com/search/label/company%20news)   [trusted execution technology](https://theinvisiblethings.blogspot.com/search/label/trusted%20execution%20technology)   [xen hacking](https://theinvisiblethings.blogspot.com/search/label/xen%20hacking)   [tpm](https://theinvisiblethings.blogspot.com/search/label/tpm)   [chipset](https://theinvisiblethings.blogspot.com/search/label/chipset)   [conferences](https://theinvisiblethings.blogspot.com/search/label/conferences)   [disk encryption](https://theinvisiblethings.blogspot.com/search/label/disk%20encryption)   [hypervisor rootkits](https://theinvisiblethings.blogspot.com/search/label/hypervisor%20rootkits)   [smm](https://theinvisiblethings.blogspot.com/search/label/smm)   [virtualization based rootkits](https://theinvisiblethings.blogspot.com/search/label/virtualization%20based%20rootkits)   [backdoors](https://theinvisiblethings.blogspot.com/search/label/backdoors)   [bad guys attacking joanna](https://theinvisiblethings.blogspot.com/search/label/bad%20guys%20attacking%20joanna)   [cloud](https://theinvisiblethings.blogspot.com/search/label/cloud)   [rootkits](https://theinvisiblethings.blogspot.com/search/label/rootkits)   [usb](https://theinvisiblethings.blogspot.com/search/label/usb)   [xen heap exploiting](https://theinvisiblethings.blogspot.com/search/label/xen%20heap%20exploiting)   [BIOS](https://theinvisiblethings.blogspot.com/search/label/BIOS)   [bitlocker](https://theinvisiblethings.blogspot.com/search/label/bitlocker)   [challanges](https://theinvisiblethings.blogspot.com/search/label/challanges)   [formal verification](https://theinvisiblethings.blogspot.com/search/label/formal%20verification)   [nested virtualization](https://theinvisiblethings.blogspot.com/search/label/nested%20virtualization)   [odyssey](https://theinvisiblethings.blogspot.com/search/label/odyssey)   [personal](https://theinvisiblethings.blogspot.com/search/label/personal)   [r3](https://theinvisiblethings.blogspot.com/search/label/r3)   [saving-the-world-afterhours](https://theinvisiblethings.blogspot.com/search/label/saving-the-world-afterhours)   [secure architecture](https://theinvisiblethings.blogspot.com/search/label/secure%20architecture)

## Total Pageviews
