---
url: "https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md"
title: "git-credential-manager/docs/credstores.md at main · git-ecosystem/git-credential-manager"
site: "GitHub"
captured: "2026-11-02T18:59:37Z"
via: "live"
sha256: "3e7750738ceeacd8dc79a5e33bdef77e84a2382f9c9c4d820e066749ff9e7e71"
wayback: "https://web.archive.org/web/20261023151245/https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# git-credential-manager/docs/credstores.md at main · git-ecosystem/git-credential-manager

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md>
> Snapshot: <https://web.archive.org/web/20261023151245/https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md>

# Credential stores

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#credential-stores

There are several options for storing credentials that GCM supports:

- Windows Credential Manager
- DPAPI protected files
- macOS Keychain
- [freedesktop.org Secret Service API](https://specifications.freedesktop.org/secret-service/)
- GPG/[pass](https://www.passwordstore.org/) compatible files
- Git's built-in [credential cache](https://git-scm.com/docs/git-credential-cache)
- Plaintext files
- Passthrough/no-op (no credential store)

The default credential stores on macOS and Windows are the macOS Keychain and the Windows Credential Manager, respectively.

GCM comes without a default store on Linux distributions.

You can select which credential store to use by setting the [GCM_CREDENTIAL_STORE](https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/environment.md#GCM_CREDENTIAL_STORE) environment variable, or the [credential.credentialStore](https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/configuration.md#credentialcredentialstore) Git configuration setting. For example:

```
git config --global credential.credentialStore gpg
```

Some credential stores have limitations, or further configuration required depending on your particular setup. See more detailed information below for each credential store.

## Windows Credential Manager

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#windows-credential-manager

Available on: Windows

This is the default store on Windows.

⚠️ Does not work over a network/SSH session.

```
SET GCM_CREDENTIAL_STORE="wincredman"
```

or

```
git config --global credential.credentialStore wincredman
```

This credential store uses the Windows Credential APIs (wincred.h) to store data securely in the Windows Credential Manager (also known as the Windows Credential Vault in earlier versions of Windows).

You can [access and manage data in the credential manager](https://support.microsoft.com/en-US/Windows/Security/credential-manager-in-windows) from the control panel, or via the [cmdkey command-line tool](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/cmdkey).

When connecting to a Windows machine over a network session (such as SSH), GCM is unable to persist credentials to the Windows Credential Manager due to limitations in Windows. Connecting by Remote Desktop doesn't suffer from this limitation.

## DPAPI protected files

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#dpapi-protected-files

Available on: Windows

```
SET GCM_CREDENTIAL_STORE="dpapi"
```

or

```
git config --global credential.credentialStore dpapi
```

This credential store uses Windows DPAPI to encrypt credentials which are stored as files in your file system. The file structure is the same as the [plaintext files credential store](https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#plaintext-files) except the first line (the secret value) is protected by DPAPI.

By default files are stored in %USERPROFILE%\.gcm\dpapi_store. This can be configured using the environment variable GCM_DPAPI_STORE_PATH environment variable.

If the directory doesn't exist it will be created.

## macOS Keychain

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#macos-keychain

Available on: macOS

This is the default store on macOS.

```
export GCM_CREDENTIAL_STORE=keychain
# or
git config --global credential.credentialStore keychain
```

This credential store uses the default macOS Keychain, which is typically the login keychain.

You can [manage data stored in the keychain](https://support.apple.com/en-gb/guide/mac-help/mchlf375f392/mac) using the Keychain Access application.

## [freedesktop.org Secret Service API](https://specifications.freedesktop.org/secret-service/)

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#freedesktoporg-secret-service-api

Available on: Linux

⚠️ Requires a D-Bus session bus and a Secret Service provider.

```
export GCM_CREDENTIAL_STORE=secretservice
# or
git config --global credential.credentialStore secretservice
```

This credential store uses the libsecret library to interact with the Secret Service. It stores credentials securely in 'collections', which can be viewed by tools such as secret-tool and seahorse.

A Secret Service provider, such as GNOME Keyring or KWallet, must be available on the user's D-Bus session bus. The provider handles prompts to create or unlock collections; these prompts may require a graphical user interface.

### Headless/SSH sessions

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#headlessssh-sessions

A graphical session is not required to access an already-unlocked collection. In a headless or SSH session, ensure GCM can access the same D-Bus session bus as the Secret Service provider, and unlock the collection using a mechanism supported by that provider before running GCM.

See [Secret Service in headless Linux sessions](https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/secretservice-headless.md) for more information, including example setup instructions for Ubuntu and the GNOME Keyring provider.

Note

GCM cannot provision a D-Bus session or replace the provider's unlock prompts with terminal or pinentry prompts. For environments without D-Bus, consider the [GPG/pass credential store](https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#gpgpass-compatible-files) or [Git's in-memory credential cache](https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#gits-built-in-credential-cache).

## GPG/[pass](https://www.passwordstore.org/) compatible files

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#gpgpass-compatible-files

Available on: macOS, Linux

⚠️ Requires gpg, pass, and a GPG key pair.

```
export GCM_CREDENTIAL_STORE=gpg
# or
git config --global credential.credentialStore gpg
```

This credential store uses GPG to encrypt files containing credentials which are stored in your file system. The file structure is compatible with the popular [pass](https://www.passwordstore.org/) tool. By default files are stored in ~/.password-store but this can be configured using the pass environment variable PASSWORD_STORE_DIR.

Before you can use this credential store, it must be initialized by the pass utility, which in-turn requires a valid GPG key pair. To initalize the store, run:

```
pass init <gpg-id>
```

..where <gpg-id> is the user ID of a GPG key pair on your system. To create a new GPG key pair, run:

```
gpg --gen-key
```

..and follow the prompts.

### Headless/TTY-only sessions

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#headlesstty-only-sessions

If you are using the gpg credential store in a headless/TTY-only environment, you must ensure you have configured the GPG Agent (gpg-agent) with a suitable pin-entry program for the terminal such as pinentry-tty or pinentry-curses.

If you are connecting to your system via SSH, then the SSH_TTY variable should automatically be set. GCM will pass the value of SSH_TTY to GPG/GPG Agent as the TTY device to use for prompting for a passphrase.

If you are not connecting via SSH, or otherwise do not have the SSH_TTY environment variable set, you must set the GPG_TTY environment variable before running GCM. The easiest way to do this is by adding the following to your profile (~/.bashrc, ~/.profile etc):

```
export GPG_TTY=$(tty)
```

Note: Using /dev/tty does not appear to work here - you must use the real TTY device path, as returned by the tty utility.

## Git's built-in [credential cache](https://git-scm.com/docs/git-credential-cache)

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#gits-built-in-credential-cache

Available on: macOS, Linux

```
export GCM_CREDENTIAL_STORE=cache
# or
git config --global credential.credentialStore cache
```

This credential store uses Git's built-in ephemeral in-memory [credential cache](https://git-scm.com/docs/git-credential-cache). This helps you reduce the number of times you have to authenticate but doesn't require storing credentials on persistent storage. It's good for scenarios like [Azure Cloud Shell](https://docs.microsoft.com/azure/cloud-shell/overview) or [AWS CloudShell](https://aws.amazon.com/cloudshell/), where you don't want to leave credentials on disk but also don't want to re-authenticate on every Git operation.

By default, git credential-cache stores your credentials for 900 seconds. That, and any other [options it accepts](https://git-scm.com/docs/git-credential-cache#_options), may be altered by setting them in the environment variable GCM_CREDENTIAL_CACHE_OPTIONS or the Git config value credential.cacheOptions. (Using the --socket option is untested and unsupported, but there's no reason it shouldn't work.)

```
export GCM_CREDENTIAL_CACHE_OPTIONS="--timeout 300"
# or
git config --global credential.cacheOptions "--timeout 300"
```

## Plaintext files

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#plaintext-files

Available on: Windows, macOS, Linux

⚠️ This is not a secure method of credential storage!

```
export GCM_CREDENTIAL_STORE=plaintext
# or
git config --global credential.credentialStore plaintext
```

This credential store saves credentials to plaintext files in your file system. By default files are stored in ~/.gcm/store or %USERPROFILE%\.gcm\store. This can be configured using the environment variable GCM_PLAINTEXT_STORE_PATH environment variable.

If the directory doesn't exist it will be created.

On POSIX platforms the newly created store directory will have permissions set such that only the owner can read/write/execute (700 or drwx---). Permissions on existing directories will not be modified.

NB. GCM's plaintext store is distinct from [git-credential-store](https://git-scm.com/docs/git-credential-store), though the formats are similar. The default paths differ.

⚠️ WARNING ⚠️

This storage mechanism is NOT secure!

Secrets and credentials are stored in plaintext files without any security!

It is HIGHLY RECOMMENDED to always use one of the other credential store options above. This option is only provided for compatibility and use in environments where no other secure option is available.

If you chose to use this credential store, it is recommended you set the permissions on this directory such that no other users or applications can access files within. If possible, use a path that exists on an external volume that you take with you and use full-disk encryption.

## Passthrough/no-op (no credential store)

https://github.com/git-ecosystem/git-credential-manager/blob/main/docs/credstores.md#passthroughno-op-no-credential-store

Available on: Windows, macOS, Linux

⚠️ .

```
SET GCM_CREDENTIAL_STORE="none"
```

or

```
git config --global credential.credentialStore none
```

This option disables the internal credential store. All operations to store or retrieve credentials will do nothing, and will return success. This is useful if you want to use a different credential store, chained in sequence via Git configuration, and don't want GCM to store credentials.

Note that you'll want to ensure that another credential helper is placed before GCM in the credential.helper Git configuration or else you will be prompted to enter your credentials every time you interact with a remote repository.
