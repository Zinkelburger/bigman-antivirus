---
url: "https://wiki.archlinux.org/title/KDE_Wallet"
title: "KDE Wallet - ArchWiki"
captured: "2026-11-02T18:49:58Z"
via: "live"
sha256: "3daef32d2e996e9f08cff19216ca55b334b3af187818c2a1a0a4189c85b39c31"
wayback: "https://web.archive.org/web/20261016150422/https://wiki.archlinux.org/title/KDE_Wallet"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# KDE Wallet - ArchWiki

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://wiki.archlinux.org/title/KDE_Wallet>
> Snapshot: <https://web.archive.org/web/20261016150422/https://wiki.archlinux.org/title/KDE_Wallet>

# KDE Wallet

- [Español](https://wiki.archlinux.org/title/KDE_Wallet_(Español))
- [日本語](https://wiki.archlinux.jp/index.php/KDE_Wallet)
- [中文（简体）](https://wiki.archlinuxcn.org/wiki/KDE_Wallet)

From ArchWiki

[KDE Wallet Manager](https://utils.kde.org/projects/kwalletmanager/) is a tool to manage passwords on the [KDE](https://wiki.archlinux.org/title/KDE) Plasma system. Using the KWallet subsystem allows a user to keep its own secrets, but also allows a user to access passwords stored by every application that integrates with KWallet.

A wallet (in the KDE's terminology, sometimes called vault or keyring) is an encrypted volume protected by a user-defined password where user and/or software can store secrets (often, credentials when the user checked "Remember the account" in an application). Those vaults can be created and used manually by the user or created and used automatically in the background by some software that integrates with the wallet subsystem (e.g. mail applications or games). Vaults are often decrypted automatically at the user login using a PAM module (see below).

Tips:

- If you only need to have a wallet available for applications using it, it is suggested to use the default name (i.e. kdewallet) and the same password as the user (for PAM).
- Wallets are stored as encrypted files using the .kwl extension in the ~/.local/share/kwalletd directory by default.

Note Since KDE Frameworks 5.97.0 KDE Wallet supports org.freedesktop.secrets DBus API and can now be used by libsecret for storing and retrieving passwords and other secrets using the Secret Service API.

## Installation

KDE Wallet is often shipped with the [KDE](https://wiki.archlinux.org/title/KDE) Plasma desktop environment. The wallet subsystem can be manually [installed](https://wiki.archlinux.org/title/Install) with the [kwallet](https://archlinux.org/packages/?name=kwallet) package.

Optionally install the [kwalletmanager](https://archlinux.org/packages/?name=kwalletmanager) package for the wallet management tool. This tool can be used to graphically create and manage a KDE Wallet.

## Configuration

### Configure PAM on Plasma 6 (KF6)

The factual accuracy of this article or section is disputed.

Reason: Configuring PAM does not seem necessary to make KWallet function. Plasma Login Manager is also not mentioned (SDDM is no longer the preferred display manager). (Discuss in [Talk:KDE Wallet#PAM configuration seems not to be necessary](https://wiki.archlinux.org/title/Talk:KDE_Wallet#PAM_configuration_seems_not_to_be_necessary))

Plasma 6 uses ksecretd as secret service API for kwallet. Therefore PAM must explicitly point to the daemon.

Add to the relevant PAM files (e.g. system-login, sddm, login depending on setup):

```
auth            optional        pam_kwallet5.so
session         optional        pam_kwallet5.so auto_start kwalletd=/usr/bin/ksecretd
```

Without the kwalletd= parameter, pam_kwallet5.so falls back to default and tries to use kwallet5.

Edit the [PAM](https://wiki.archlinux.org/title/PAM) configuration corresponding to your situation:

- For [SDDM](https://wiki.archlinux.org/title/SDDM), no further edits should be needed because the lines are already present in /etc/pam.d/sddm.
- For [LightDM](https://wiki.archlinux.org/title/LightDM), no further edits should be needed because the lines are already present in /etc/pam.d/lightdm and /etc/pam.d/lightdm-autologin.
- For [GDM](https://wiki.archlinux.org/title/GDM), edit /etc/pam.d/gdm-password accordingly.
- For [greetd](https://wiki.archlinux.org/title/Greetd), edit /etc/pam.d/greetd accordingly.
- For unlocking on tty login (no display manager, or like [greetd-tuigreet](https://archlinux.org/packages/?name=greetd-tuigreet)), edit /etc/pam.d/login accordingly. You will need to specify the force_run parameter.

```
/etc/pam.d/login
```

```
auth            optional        pam_kwallet5.so
session         optional        pam_kwallet5.so auto_start force_run kwalletd=/usr/bin/ksecretd
```

```
/etc/pam.d/greetd
```

```
#%PAM-1.0

auth       required     pam_securetty.so
auth       requisite    pam_nologin.so
auth       include      system-local-login
auth       optional     pam_kwallet5.so
account    include      system-local-login
session    include      system-local-login
session    optional     pam_kwallet5.so auto_start force_run kwalletd=/usr/bin/ksecretd
```

#### Plasma 5 (KF5) – legacy

Older Plasma versions use kwalletd5 and do not require specifying the daemon.

```
/etc/pam.d/login
```

```
auth     optional  pam_kwallet5.so
session  optional  pam_kwallet5.so auto_start
```

### Unlock KDE Wallet automatically on login

To unlock KDE Wallet automatically on login, [install](https://wiki.archlinux.org/title/Install) [kwallet-pam](https://archlinux.org/packages/?name=kwallet-pam) for the [PAM](https://wiki.archlinux.org/title/PAM) compatible module. The chosen KWallet password must be the same as the current [user](https://wiki.archlinux.org/title/User) password.

Note

- [kwallet-pam](https://archlinux.org/packages/?name=kwallet-pam) is not compatible with [GnuPG](https://wiki.archlinux.org/title/GnuPG) keys, the KDE Wallet must use the standard blowfish encryption.
- When using autologin, the wallet can only be unlocked if the autologin method saves the password. [pam_autologin](https://wiki.archlinux.org/title/Pam_autologin) does, for example.
-
- The wallet cannot be unlocked when using a fingerprint reader to login
- The wallet must be named kdewallet (default name). It does not unlock any other wallet(s).
- If using [KDE](https://wiki.archlinux.org/title/KDE), one may want to disable Close when last application stops using it in KDE Wallet settings to prevent the wallet from being closed after each usage ([Wi-Fi](https://wiki.archlinux.org/title/Wi-Fi)-passphrase unlock, etc.).
- It may be needed to remove the default created wallet first, thus removing all stored entries.
- If the kwallet Migration Assistant asks for a password after every login, rename or delete the ~/.kde4/share/apps/kwallet folder.

Tip An alternative is to use KWalletManager and set an empty Kwallet-password, thus preventing the need of entering a password to unlock a wallet. Simply do not enter a password on both fields in Change Password... This may however lead to unwanted (read/write) access to the user's wallet. Enabling Prompt when an application accesses a wallet under Access Control is highly recommended to prevent unwanted access to the wallet.

#### Using Plasma Login Manager autologin and LUKS encryption

Copy /usr/lib/pam.d/plasmalogin-autologin to /etc/pam.d/plasmalogin-autologin then add [pam_systemd_loadkey(8)](https://man.archlinux.org/man/pam_systemd_loadkey.8) before pam_kwallet5.so:

```
/etc/pam.d/plasmalogin-autologin
```

```
-auth       optional    pam_systemd_loadkey.so
-auth       optional    pam_gnome_keyring.so
-auth       optional    pam_kwallet5.so
```

Then add the following to plasmalogin.service:

```
/etc/systemd/system/plasmalogin.service.d/keyringmode.conf
```

```
[Service]
KeyringMode=inherit
```

#### Using SDDM autologin and LUKS encryption

This article or section is a candidate for merging with [#Using Plasma Login Manager autologin and LUKS encryption](https://wiki.archlinux.org/title/KDE_Wallet#Using_Plasma_Login_Manager_autologin_and_LUKS_encryption).

Notes: Prerequisites and configuration are the same, depending on which log-in manager is used by the desktop environment. (Discuss in [Talk:KDE Wallet](https://wiki.archlinux.org/title/Talk:KDE_Wallet))

When the system is encrypted using [dm-crypt](https://wiki.archlinux.org/title/Dm-crypt) it is possible to automatically unlock KDE Wallet using the passphrase that decrypts the disk. When [SDDM#Autologin](https://wiki.archlinux.org/title/SDDM#Autologin) is configured, the wallet can still be unlocked automatically. Edit /etc/pam.d/sddm-autologin to add [pam_systemd_loadkey(8)](https://man.archlinux.org/man/pam_systemd_loadkey.8):

```
/etc/pam.d/sddm-autologin
```

```
-auth       optional    pam_systemd_loadkey.so
-auth       optional    pam_gnome_keyring.so
-auth       optional    pam_kwallet5.so
-session    optional    pam_gnome_keyring.so auto_start
-session    optional    pam_kwallet5.so auto_start kwalletd=/usr/bin/ksecretd
```

Then [edit](https://wiki.archlinux.org/title/Edit) sddm.service, and add:

```
/etc/systemd/system/sddm.service.d/keyringmode.conf
```

```
[Service]
KeyringMode=inherit
```

## Tips and tricks

### Using the KDE Wallet to store ssh key passphrases

[Install](https://wiki.archlinux.org/title/Install) [ksshaskpass](https://archlinux.org/packages/?name=ksshaskpass) package.

Set the SSH_ASKPASS [environment variable](https://wiki.archlinux.org/title/Environment_variable) to ksshaskpass and SSH_ASKPASS_REQUIRE to prefer (prefer to use the askpass program instead of the TTY). To set it automatically on each login, create the following [environment.d(5)](https://man.archlinux.org/man/environment.d.5) file:

```
~/.config/environment.d/ssh_askpass.conf
```

```
SSH_ASKPASS=/usr/bin/ksshaskpass
SSH_ASKPASS_REQUIRE=prefer
```

Restart your session (i.e. relogin) so that the environment variables take effect.

The first time you try to use an SSH key, you will get asked for its passphrase. Make sure to check the Remember password checkbox. Next time, the passphrase will be read from KDE Wallet.

### Using the KDE Wallet to store Git credentials

[Git](https://wiki.archlinux.org/title/Git) can delegate credential handling to a credential helper. By using [ksshaskpass](https://archlinux.org/packages/?name=ksshaskpass) as a credential helper, the HTTP/HTTPS and SMTP passwords can be safely stored in the KDE Wallet.

[Install](https://wiki.archlinux.org/title/Install) the [ksshaskpass](https://archlinux.org/packages/?name=ksshaskpass) package.

Configure Git by setting the GIT_ASKPASS [environment variable](https://wiki.archlinux.org/title/Environment_variable):

```
~/.config/environment.d/git_askpass.conf
```

```
GIT_ASKPASS=/usr/bin/ksshaskpass
```

Tip If the SSH_ASKPASS environment variable [is set to ksshaskpass](https://wiki.archlinux.org/title/KDE_Wallet#Using_the_KDE_Wallet_to_store_ssh_key_passphrases), then additionally setting GIT_ASKPASS is not required.

See [gitcredentials(7)](https://man.archlinux.org/man/gitcredentials.7) for alternatives and more details.

### Store GPG key passphrases

Native KDE windows can be used to prompt for GPG key passphrases and save them in KDE Wallet.

[Configure](https://wiki.archlinux.org/title/GnuPG#pinentry) gpg-agent to use /usr/bin/pinentry-qt.

Enable the Secret Service interface. There are two ways to do this:

- Go to System Settings > KDE Wallet and enable Use KWallet for the Secret Service interface.
- Edit the KDE Wallet configuration file:

```
~/.config/kwalletrc
```

```
[org.freedesktop.secrets]
apiEnabled=true
```

Close the wallet and reopen it to affect these changes. You can do this using [kwalletmanager](https://archlinux.org/packages/?name=kwalletmanager) or by issuing commands to Qt D-Bus directly:

```
$ qdbus org.kde.kwalletd6 /modules/kwalletd6 closeAllWallets
$ qdbus org.kde.kwalletd6 /modules/kwalletd6 open kdewallet 0 $0

```

### KDE Wallet for Chromium and VSCode

[Chromium](https://wiki.archlinux.org/title/Chromium)-based browsers have built-in wallet integration. [VSCode](https://wiki.archlinux.org/title/VSCode)-based [IDEs](https://wiki.archlinux.org/title/IDE) rely on the same Chromium oscrypt module for keyring detection. [[1]](https://code.visualstudio.com/docs/configure/settings-sync#_troubleshooting-keychain-issues)

To enable it, run the program with either the --password-store=kwallet5 or --password-store=kwallet6 or --password-store=detect argument. [[2]](https://source.chromium.org/chromium/chromium/src/+/main:docs/linux/password_storage.md)

To make the change persistent, see [Chromium#Making flags persistent](https://wiki.archlinux.org/title/Chromium#Making_flags_persistent) (Setting CHROMIUM_USER_FLAGS will not work).

### Query passwords from the terminal

Instead of storing passwords in plain text files, you can manually add new entries in your wallet and retrieve them with kwallet-query.

For example, if you want to log into the Docker Hub registry with Podman, which supports getting the passwords from stdin with the --password-stdin flag, you can use the following command to login:

```
$ kwallet-query -r folder_entry wallet_name -f folder_name | podman login docker.io -u dockerhub_username --password-stdin

```

This way, your password is not stored in any text file and neither is it stored in the terminal history file.

In order to run kwallet-query outside of a graphical session (for instance as part of an unattended backup script), set the QT_QPA_PLATFORM=offscreen environment variable:

```
$ QT_QPA_PLATFORM=offscreen kwallet-query -r folder_entry wallet_name -f folder_name

```

### Unlocking KWallet automatically in a window manager

To unlock KWallet protected by the login password, it is necessary to start /usr/lib/pam_kwallet_init in the [autostart](https://wiki.archlinux.org/title/Autostarting#On_window_manager_startup) portion of your window manager's configuration file in addition to configuring [PAM](https://wiki.archlinux.org/title/PAM).

### Disable KWallet

In case you want to permanently disable kwallet:

```
~/.config/kwalletrc
```

```
[Wallet]
Enabled=false
```

Note It is possible to use KWallet as a proxy to other wallet managers. This way applications like [plasma-nm](https://archlinux.org/packages/?name=plasma-nm) do not break. See this [discussion](https://wiki.archlinux.org/title/Talk:KDE_Wallet#KWalletd6_as_proxy_for_KeePassXC_with_Secret_Service).

### Automatic D-Bus activation

Most applications use org.freedesktop.secrets.service [D-Bus](https://wiki.archlinux.org/title/D-Bus) service. KWallet does not provide a service file for it out of the box.

You can achieve automatic activation by creating such service file:

```
~/.local/share/dbus-1/services/org.freedesktop.secrets.service
```

```
[D-BUS Service]
Name=org.freedesktop.secrets
Exec=/usr/bin/kwalletd6
```

## See also

- [Wikipedia:KWallet](https://en.wikipedia.org/wiki/KWallet)
- [dennogumi.org: Unlocking KWallet with PAM](https://web.archive.org/web/20250101021426/https://www.dennogumi.org/2014/04/unlocking-kwallet-with-pam//)
- [org.freedesktop.secrets DBus API initial support](https://invent.kde.org/frameworks/kwallet/-/merge_requests/11)

Retrieved from "https://wiki.archlinux.org/index.php?title=KDE_Wallet&oldid=884163"

[Category](https://wiki.archlinux.org/title/Special:Categories):

- [KDE](https://wiki.archlinux.org/title/Category:KDE)

Hidden categories:

- [Pages or sections flagged with Template:Accuracy](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Accuracy)
- [Pages or sections flagged with Template:Merge](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Merge)
