---
url: "https://wiki.archlinux.org/title/GNOME/Keyring"
title: "GNOME/Keyring - ArchWiki"
captured: "2026-11-02T18:49:55Z"
via: "live"
sha256: "97aa04db52bae80da03f62ddcb7e9f3039b49f02e69321545c05e652770357d4"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# GNOME/Keyring - ArchWiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.archlinux.org/title/GNOME/Keyring>

# GNOME/Keyring

- [Español](https://wiki.archlinux.org/title/GNOME_(Espa%C3%B1ol)/Keyring_(Español))
- [日本語](https://wiki.archlinux.jp/index.php/GNOME/Keyring)
- [Русский](https://wiki.archlinux.org/title/GNOME_(%D0%A0%D1%83%D1%81%D1%81%D0%BA%D0%B8%D0%B9)/Keyring_(Русский))
- [中文（简体）](https://wiki.archlinuxcn.org/wiki/GNOME/Keyring)

From ArchWiki

< [GNOME](https://wiki.archlinux.org/title/GNOME)

Related articles

- [GnuPG](https://wiki.archlinux.org/title/GnuPG)
- [Oo7](https://wiki.archlinux.org/title/Oo7)
- [OpenPGP](https://wiki.archlinux.org/title/OpenPGP)

[GNOME Keyring](https://wiki.gnome.org/Projects/GnomeKeyring) is "a collection of components in GNOME that store secrets, passwords, keys, certificates and make them available to applications."

It provides [org.freedesktop.secrets](https://www.freedesktop.org/wiki/Specifications/secret-storage-spec/), an API that allows client applications to store secrets securely using a service running in the user's login session.

Note As of [Gnome 51](https://release.gnome.org/51/) the default key storage is provided by [oo7](https://wiki.archlinux.org/title/Oo7).

## Security

### Protection against malicious application

There was a security issue (known as [CVE-2018-19358](https://nvd.nist.gov/vuln/detail/CVE-2018-19358)) reported in the past regarding the behaviour of the GNOME/Keyring API. Any application [can easily read any secret](https://github.com/sungjungk/keyring_crack) if the keyring is unlocked. And, if a user is logged in, then the login/default collection is unlocked. Available D-Bus protection mechanisms (involving the busconfig and policy XML elements) are not used by default and would be easy to bypass anyway.

The GNOME project [disagrees](https://gitlab.gnome.org/GNOME/gnome-keyring/-/issues/5#note_1876550) with this vulnerability report because, according to their stated security model, untrusted applications must not be allowed to communicate with the secret service.

Applications running via well-behaved sandboxes (such as Flatpak without additional permissions) only have filtered access to the session bus.

### Keyring is not locked when session is locked

When the session is locked, the keyring is not automatically locked.[[1]](https://gitlab.gnome.org/GNOME/gnome-keyring/-/issues/62) This means that the passwords remain in memory which opens the system up to a [DMA attack](https://en.wikipedia.org/wiki/DMA_attack).

## Installation

[gnome-keyring](https://archlinux.org/packages/?name=gnome-keyring) is a member of the [gnome](https://archlinux.org/groups/x86_64/gnome/) group and is thus usually present on systems running GNOME. The package can otherwise be [installed](https://wiki.archlinux.org/title/Install) on its own. [libsecret](https://archlinux.org/packages/?name=libsecret) should also be installed to grant other applications access to your keyrings. Although [libgnome-keyring](https://aur.archlinux.org/packages/libgnome-keyring/)AUR is deprecated (and superseded by libsecret), it may still be required by certain applications.

The gnome-keyring-daemon is automatically started via a systemd user service upon logging in. It can also be started upon request via a socket.

Extra utilities related to GNOME Keyring include:

- secret-tool — Access the GNOME Keyring (and any other service implementing the [DBus Secret Service API](https://specifications.freedesktop.org/secret-service-spec/latest/)) from the command line.

https://wiki.gnome.org/Projects/Libsecret || [libsecret](https://archlinux.org/packages/?name=libsecret)

- lssecret — List all secret items using libsecret (e.g. GNOME Keyring).

https://gitlab.com/GrantMoyer/lssecret || [lssecret-git](https://aur.archlinux.org/packages/lssecret-git/)AUR

## Manage using GUI

You can manage the contents of GNOME Keyring using Seahorse; [install](https://wiki.archlinux.org/title/Install) the [seahorse](https://archlinux.org/packages/?name=seahorse) package.

Passwords for keyrings (e.g., the default keyring, "Login") can be changed and even removed. See [Create a new keyring](https://help.gnome.org/seahorse/keyring-create.html) and [Update the keyring password](https://help.gnome.org/seahorse/keyring-update-password.html) in GNOME Help for more information.

## Using the keyring

The [PAM](https://wiki.archlinux.org/title/PAM) module pam_gnome_keyring.so initialises GNOME Keyring partially, unlocking the login keyring in the process. The gnome-keyring-daemon is automatically started with a systemd user service.

### PAM step

Note To use automatic unlocking without automatic login, the password for the user account should be the same as the login keyring. See [#Automatically change keyring password with user password](https://wiki.archlinux.org/title/GNOME/Keyring#Automatically_change_keyring_password_with_user_password).

Tip

- To use automatic unlocking with automatic login, you can set a blank password for the keyring. Note that the contents of the keyring are stored unencrypted in this case.
- Alternatively, if using GDM and LUKS, GDM can unlock your keyring if it matches your LUKS password. For this to work, you need to use the [systemd init in your mkinitcpio.conf](https://wiki.archlinux.org/title/Mkinitcpio#Common_hooks) as well as the [appropriate kernel parameters](https://wiki.archlinux.org/title/Dm-crypt/System_configuration#Using_systemd-cryptsetup-generator), and you should make sure that your system's real-time clock is set to UTC rather than local time (see [[2]](https://gitlab.gnome.org/GNOME/gdm/-/issues/701#note_1740585)). See [[3]](https://reddit.com/r/Fedora/comments/jwnqq5/) for more details.
- If you desire to skip the PAM step, the keyring must be unlocked manually or via another method. See [#Using gnome-keyring-daemon outside desktop environments (KDE, GNOME, XFCE, ...)](https://wiki.archlinux.org/title/GNOME/Keyring#Using_gnome-keyring-daemon_outside_desktop_environments_(KDE,_GNOME,_XFCE,_...)) and the [GnomeKeyring wiki](https://wiki.gnome.org/Projects/GnomeKeyring/RunningDaemon).

When using a display manager, the keyring works out of the box for most cases. [GDM](https://wiki.archlinux.org/title/GDM), [LightDM](https://wiki.archlinux.org/title/LightDM), [LXDM](https://wiki.archlinux.org/title/LXDM), and [SDDM](https://wiki.archlinux.org/title/SDDM) already have the necessary PAM configuration. For a display manager that does not automatically unlock the keyring edit the appropriate file instead of /etc/pam.d/login as mentioned below.

When using console-based login (ie, not using a Display Manager), edit /etc/pam.d/login:

Add auth optional pam_gnome_keyring.so at the end of the auth section and session optional pam_gnome_keyring.so auto_start at the end of the session section.

```
/etc/pam.d/login
```

```
#%PAM-1.0

auth       required     pam_securetty.so
auth       requisite    pam_nologin.so
auth       include      system-local-login
auth       optional     pam_gnome_keyring.so
account    include      system-local-login
session    include      system-local-login
session    optional     pam_gnome_keyring.so auto_start

```

Note If using the [greetd](https://wiki.archlinux.org/title/Greetd) login manager, the file that needs to be modified is /etc/pam.d/greetd, instead of /etc/pam.d/login.

### Unlocking with fingerprint login

Fingerprint authentication with [fprintd](https://wiki.archlinux.org/title/Fprint) does not set PAM_AUTHTOK: the reader returns a verdict, not a password. pam_gnome_keyring.so therefore has nothing to unlock the login keyring with, and the first application that requests a secret prompts for one [[4]](https://gitlab.gnome.org/GNOME/gnome-keyring/-/issues/1). The same applies to any authentication method that does not produce the account password.

Possible approaches:

- Enter the account password at the first prompt of the session. The keyring stays unlocked for the rest of it.
- Set a blank password for the login keyring, as described in [#PAM step](https://wiki.archlinux.org/title/GNOME/Keyring#PAM_step). The keyring contents are then stored unencrypted.
- Have a PAM module that runs before pam_gnome_keyring.so set PAM_AUTHTOK to the keyring password. Where that password is stored, and what protects it, is what distinguishes such implementations.

## SSH keys

GNOME Keyring can act as a wrapper around [ssh-agent](https://wiki.archlinux.org/title/SSH_keys#ssh-agent). In that mode, it will display a GUI password entry dialog each time you need to unlock an SSH key. The dialog includes a checkbox to remember the password you type, which, if selected, will allow fully passwordless use of that key in the future as long as your login keyring is unlocked.

The SSH functionality is disabled by default in gnome-keyring-daemon builds since version [1:46](https://gitlab.gnome.org/GNOME/gnome-keyring/-/commit/25c5a1982467802fa12c6852b03c57924553ba73). It has been [moved](https://gitlab.gnome.org/GNOME/gcr/-/merge_requests/67) into /usr/lib/gcr-ssh-agent, which is part of the [gcr-4](https://archlinux.org/packages/?name=gcr-4) package.

### Setup gcr

[Enable](https://wiki.archlinux.org/title/Enable) and [start](https://wiki.archlinux.org/title/Start) the gcr-ssh-agent.socket systemd [user unit](https://wiki.archlinux.org/title/User_unit).

Once the unit starts, it will create the socket file $XDG_RUNTIME_DIR/gcr/ssh.

Manual configuration of SSH_AUTH_SOCK environment variable is not necessary if gcr-ssh-agent.socket unit is active. The value of SSH_AUTH_SOCK environment variable shall be set to $XDG_RUNTIME_DIR/gcr/ssh when user logs out and logs in.

### Using

You can run

```
$ ssh-add -L

```

to list loaded SSH keys in the running agent. This can help ensure you started the appropriate service and set SSH_AUTH_SOCK correctly.

To permanently save a passphrase in the keyring, use ssh-askpass from the [seahorse](https://archlinux.org/packages/?name=seahorse) package:

```
$ /usr/lib/seahorse/ssh-askpass my_key

```

Tip In some situations (e.g. when you are not using a login manager or desktop environment), an alternative is to use the Seahorse secret-tool [to store a passphrase](https://bbs.archlinux.org/viewtopic.php?pid=2217761#p2217761).

To manually add an SSH key from another directory:

```
$ ssh-add ~/.private/id_ed25519
Enter passphrase for ~/.private/id_ed25519:

```

Note You have to have the corresponding .pub file in the same directory as the private key (~/.ssh/id_ed25519.pub in the example). Also, make sure that the public key is the file name of the private key plus .pub (for example, my_key.pub).

To disable all manually added keys:

```
$ ssh-add -D

```

### Disabling

If you wish to run an alternative SSH agent (e.g. [ssh-agent](https://wiki.archlinux.org/title/SSH_keys#ssh-agent) directly or [gpg-agent](https://wiki.archlinux.org/title/Gpg-agent)), it is a good idea to disable GNOME Keyring's ssh-agent wrapper. Doing so is not strictly necessary, since each agent listens on a different socket and SSH_AUTH_SOCK can be used to choose between them, but it can make debugging issues easier. Note that the GNOME implementation does not support many scripting features such BatchMode [[5]](https://gitlab.gnome.org/GNOME/gcr/-/issues/117).

To disable gcr-ssh-agent, ensure gcr-ssh-agent.socket and gcr-ssh-agent.service are both disabled and stopped in systemd.

## Tips and tricks

### Integration with applications

- [Chromium](https://wiki.archlinux.org/title/Chromium#Force_a_password_store)

### Locking a keyring

```
$ dbus-send --session --dest=org.freedesktop.secrets \
   --type=method_call  \
   /org/freedesktop/secrets \
   org.freedesktop.Secret.Service.Lock \
   array:objpath:/org/freedesktop/secrets/collection/login

```

This command performs a [D-Bus](https://wiki.archlinux.org/title/D-Bus) method call to lock the login keyring. Alternatively, if you want to use a GUI, Seahorse can be used lock the keyring.

### Flushing passphrases

```
$ gnome-keyring-daemon -r -d

```

This command starts gnome-keyring-daemon, shutting down previously running instances.

### Git integration

The GNOME keyring is useful in conjunction with [Git](https://wiki.archlinux.org/title/Git) when you are pushing over HTTPS. The [libsecret](https://archlinux.org/packages/?name=libsecret) package [needs to be installed for this functionality to be available](https://wiki.archlinux.org/title/GNOME/Keyring#Installation).

Configure Git to use the libsecret helper:

```
$ git config --global credential.helper /usr/lib/git-core/git-credential-libsecret

```

The next time you run git push, you will be asked to unlock your keyring if it is not already unlocked.

### GnuPG integration

Several applications which use GnuPG require a pinentry-program to be set. Set the following to use GNOME 3 pinentry for GNOME Keyring to manage passphrase prompts.

```
~/.gnupg/gpg-agent.conf
```

```
pinentry-program /usr/bin/pinentry-gnome3

```

Another option is to [force loopback for GPG](https://wiki.archlinux.org/title/GnuPG#Unattended_passphrase) which should allow the passphrase to be entered in the application.

### Renaming a keyring

The display name for a keyring (i.e., the name that appears in Seahorse and from file) can be changed by [changing the value of display-name in the unencrypted keyring file](https://ttboj.wordpress.com/2013/01/27/renaming-a-gnome-keyring-for-seahorse-the-passwords-and-keyrings-application/). Keyrings will usually be stored in ~/.local/share/keyrings/ with the .keyring file extension.

### Automatically change keyring password with user password

Note This only affects the login keyring.

[Append](https://wiki.archlinux.org/title/Append) password optional pam_gnome_keyring.so to /etc/pam.d/passwd:

```
/etc/pam.d/passwd
```

```
...
password	optional	pam_gnome_keyring.so
```

### Using gnome-keyring-daemon outside desktop environments (KDE, GNOME, XFCE, ...)

#### Launching

The factual accuracy of this article or section is disputed.

Reason: At least [xinit](https://wiki.archlinux.org/title/Xinit) and [SDDM](https://wiki.archlinux.org/title/SDDM) execute all scripts from /etc/X11/xinit/xinitrc.d/ and [sway](https://wiki.archlinux.org/title/Sway) provides /etc/sway/config.d/50-systemd-user.conf so the problem is not being "outside desktop environments". If gnome-keyring requires [XDG Autostart](https://wiki.archlinux.org/title/XDG_Autostart), the installation/configuration section should say so. (Discuss in [Talk:GNOME/Keyring#Launching gnome-keyring-daemon outside desktop environments (KDE,_GNOME,_XFCE,_...)](https://wiki.archlinux.org/title/Talk:GNOME/Keyring#Launching_gnome-keyring-daemon_outside_desktop_environments_(KDE,_GNOME,_XFCE,_...)))

If you are using sway, i3, or any window manager that does not execute

- /etc/xdg/autostart/gnome-keyring-*.desktop
- /etc/X11/xinit/xinitrc.d/50-systemd-user.sh

your window manager needs to execute the following commands during window manager startup. The commands do not need to be executed in any specific order.

```
dbus-update-activation-environment DISPLAY XAUTHORITY WAYLAND_DISPLAY
```

or

```
dbus-update-activation-environment --all
```

This command passes environment variables from the window manager to session dbus. Without this, GUI prompts cannot be triggered over DBus. For example, this is required for seahorse password prompt.

This is required because session dbus is started before graphical environment is started. Thus, session dbus does not know about the graphical environment you are in. Someone or something has to teach session dbus about the graphical environment by passing environment variables describing the graphical environment to session dbus.

```
gnome-keyring-daemon --start --components=secrets
```

During login, PAM starts gnome-keyring-daemon --login which is responsible for keeping gnome-keyring unlocked with login password. If gnome-keyring-daemon --login is not connected to session dbus within a few minutes, gnome-keyring-daemon --login dies. If gnome-keyring-daemon --start ... is started against session dbus in a window manager, gnome-keyring-daemon --login is connected to session dbus. If your login session does not start gnome-keyring-daemon --start ... before gnome-keyring-daemon --login quits, you can also use any program that uses gnome-keyring or secret service API before gnome-keyring-daemon --login dies.

#### GNOME Keyring XDG Portal

GNOME Keyring exposes an XDG Portal backend (for use with applications sandboxed through [flatpak](https://wiki.archlinux.org/title/Flatpak) for example). In order for it to work outside of GNOME, one must configure it to be used. There are a few ways to do this.

##### Configuring user-level Secrets Portal

The method which avoids modifying system files is to configure gnome-keyring to be preferred as the Secrets portal in $XDG_CONFIG_HOME/xdg-desktop-portal/$XDG_CURRENT_DESKTOP-portals.conf. For example, to configure [sway](https://wiki.archlinux.org/title/Sway) to use gnome-keyring as the preferred portal to use for org.freedesktop.impl.portal.Secret.

```
~/.config/xdg-desktop-portal/sway-portals.conf
```

```
[preferred]
default=wlr;gtk
org.freedesktop.impl.portal.Secret=gnome-keyring

```

##### Modifying gnome-keyring.portal

The factual accuracy of this article or section is disputed.

Reason: Modifies package files in /usr/share, will be undone on pacman upgrades (Discuss in [Talk:GNOME/Keyring](https://wiki.archlinux.org/title/Talk:GNOME/Keyring))

Another method is to add your desktop environment to the /usr/share/xdg-desktop-portal/portals/gnome-keyring.portal configuration file by modifying the UseIn key. For instance, to add [sway](https://wiki.archlinux.org/title/Sway):

```
/usr/share/xdg-desktop-portal/portals/gnome-keyring.portal
```

```
[portal]
DBusName=org.freedesktop.secrets
Interfaces=org.freedesktop.impl.portal.Secret
UseIn=gnome;sway

```

See [XDG Desktop Portal#Backends](https://wiki.archlinux.org/title/XDG_Desktop_Portal#Backends) for more information about XDG Desktop Portal backends.

## Troubleshooting

### Passwords are not remembered

If you are prompted for a password after logging in and you find that your passwords are not saved, then you may need to create/set a default keyring. To do this using Seahorse (a.k.a. Passwords and Keys), see [Create a new keyring](https://help.gnome.org/users/seahorse/stable/keyring-create.html.en) and [Change the default keyring](https://help.gnome.org/users/seahorse/stable/keyring-change-default.html.en) in GNOME Help.

### Resetting the keyring

You will need to [change your login keyring password](https://wiki.archlinux.org/title/GNOME/Keyring#Manage_using_GUI) if you receive the following error message: "The password you use to login to your computer no longer matches that of your login keyring".

Alternatively, you can remove the login.keyring and user.keystore files from ~/.local/share/keyrings/. Be warned that this will permanently delete all saved keys. After removing the files, simply log out and log in again.

### Unable to locate daemon control file

The following error may appear in the [journal](https://wiki.archlinux.org/title/Journal) after logging in:

```
gkr-pam: unable to locate daemon control file

```

This message "can be safely ignored" if there are no other related issues [[6]](https://bbs.archlinux.org/viewtopic.php?pid=1940190#p1940190).

### No such secret collection at path: /

If you try to add a new keyring with Seahorse you may receive this error due to the following reasons:

- The ~/.local/share/keyrings/ directory does not exist. Create it if missing.
- A custom ~/.xinitrc is used. This may be solved by adding the following line [[7]](https://bbs.archlinux.org/viewtopic.php?pid=1640822#p1640822)

```
~/.xinitrc
```

```
source /etc/X11/xinit/xinitrc.d/50-systemd-user.sh

```

### Terminal gives the message "discover_other_daemon: 1"

This is caused by gnome-keyring-daemon being started for the second time. Since a systemd service is delivered together with the daemon, you do not need to start it another way. So make sure to remove the start command from your .zshenv, .bash_profile, .xinitrc, config.fish or similar. Alternatively you can [disable](https://wiki.archlinux.org/title/Disable) the gnome-keyring-daemon.service and gnome-keyring-daemon.socket [user units](https://wiki.archlinux.org/title/User_unit).

### Improper initialization of the keyring

There are a few symptoms of this:

- Programs like SSH or Git hang while waiting for the keyring to provide a password, eventually timing out with errors such as "agent refused operation".
- Seahorse does not display any keyring, and manually creating a keyring named "login" does not appear to do anything.
- The following error message appears in seahorse's output:

```
couldn't load all secret collections: No such secret collection at path: /org/freedesktop/secrets/collection/login
```

To address this, do the following:

- [Restart](https://wiki.archlinux.org/title/Restart) the gnome-keyring-daemon.service systemd [user unit](https://wiki.archlinux.org/title/User_unit). This should properly initialize the login keyring.
- [Restart](https://wiki.archlinux.org/title/Restart) the gcr-ssh-agent.service systemd [user unit](https://wiki.archlinux.org/title/User_unit) if used. Other agents might also need restarting.

If this does not fix the issue, consider [resetting the keyring](https://wiki.archlinux.org/title/GNOME/Keyring#Troubleshooting).

### SSH agent is not available

Symptom :

```
$ ssh-add -L
```

```
Error connecting to agent: Connection refused

```

Ensure the gcr-ssh-agent.socket [user unit](https://wiki.archlinux.org/title/User_unit) is [enabled](https://wiki.archlinux.org/title/Enable) and [started](https://wiki.archlinux.org/title/Start) and that the gnome-keyring-daemon socket and service [user units](https://wiki.archlinux.org/title/User_unit) are enabled and started:

Tip You can use the shell brace expansion with {service,socket}.

### No Process Capabilities, Insecure Memory Might Get Used

If one sees the following error gnome-keyring-daemon: no process capabilities, insecure memory might get used it means the key-ring could get saved to a swap file/partition. This is a security concern as someone could read and extract the secrets without proper authorization. The solution is to tell the kernel not to do that, but gnome-keyring-daemon needs to have the proper permissions to do so.

Run the following command to give gnome-keyring-daemon the proper memory locking permissions:

```
# setcap cap_ipc_lock=+ep /usr/bin/gnome-keyring-daemon

```

## See also

- https://help.gnome.org/users/seahorse/stable/
- [GNOME Wiki page (Archived)](https://wiki.gnome.org/Projects/GnomeKeyring)
- [Packages providing org.freedesktop.secrets](https://archlinux.org/packages/?q=org.freedesktop.secrets)

Retrieved from "https://wiki.archlinux.org/index.php?title=GNOME/Keyring&oldid=888522"

[Categories](https://wiki.archlinux.org/title/Special:Categories):

- [GNOME](https://wiki.archlinux.org/title/Category:GNOME)
- [OpenPGP](https://wiki.archlinux.org/title/Category:OpenPGP)

Hidden category:

- [Pages or sections flagged with Template:Accuracy](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Accuracy)
