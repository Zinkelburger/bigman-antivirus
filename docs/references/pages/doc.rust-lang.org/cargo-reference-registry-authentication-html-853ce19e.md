---
url: "https://doc.rust-lang.org/cargo/reference/registry-authentication.html"
title: "Registry Authentication - The Cargo Book"
captured: "2026-11-02T18:27:37Z"
via: "live"
sha256: "de62dd9394ee89c90cc576f8bc8c2d5b78c5990b31153041b6a1d2835434df34"
wayback: "https://web.archive.org/web/20261013151134/https://doc.rust-lang.org/cargo/reference/registry-authentication.html"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Registry Authentication - The Cargo Book

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://doc.rust-lang.org/cargo/reference/registry-authentication.html>
> Snapshot: <https://web.archive.org/web/20261013151134/https://doc.rust-lang.org/cargo/reference/registry-authentication.html>

# [Registry Authentication](https://doc.rust-lang.org/cargo/reference/registry-authentication.html#registry-authentication)

Cargo authenticates to registries with credential providers. These credential providers are external executables or built-in providers that Cargo uses to store and retrieve credentials.

Using alternative registries with authentication requires a credential provider to be configured to avoid unknowingly storing unencrypted credentials on disk. For historical reasons, public (non-authenticated) registries do not require credential provider configuration, and the cargo:token provider is used if no providers are configured.

Cargo also includes platform-specific providers that use the operating system to securely store tokens. The cargo:token provider is also included which stores credentials in unencrypted plain text in the [credentials](https://doc.rust-lang.org/cargo/reference/config.html#credentials) file.

## [Recommended configuration](https://doc.rust-lang.org/cargo/reference/registry-authentication.html#recommended-configuration)

It’s recommended to configure a global credential provider list in $CARGO_HOME/config.toml which defaults to:

- Windows: %USERPROFILE%\.cargo\config.toml
- Unix: ~/.cargo/config.toml

This recommended configuration uses the operating system provider, with a fallback to cargo:token to look in Cargo’s [credentials](https://doc.rust-lang.org/cargo/reference/config.html#credentials) file or environment variables:

```
# ~/.cargo/config.toml
[registry]
global-credential-providers = ["cargo:token", "cargo:libsecret", "cargo:macos-keychain", "cargo:wincred"]

```

Note that later entries have higher precedence. See [registry.global-credential-providers](https://doc.rust-lang.org/cargo/reference/config.html#registryglobal-credential-providers) for more details.

Some private registries may also recommend a registry-specific credential-provider. Check your registry’s documentation to see if this is the case.

## [Built-in providers](https://doc.rust-lang.org/cargo/reference/registry-authentication.html#built-in-providers)

Cargo includes several built-in credential providers. The available built-in providers may change in future Cargo releases (though there are currently no plans to do so).

### [cargo:token](https://doc.rust-lang.org/cargo/reference/registry-authentication.html#cargotoken)

Uses Cargo’s [credentials](https://doc.rust-lang.org/cargo/reference/config.html#credentials) file to store tokens unencrypted in plain text. When retrieving tokens, checks the CARGO_REGISTRIES_<NAME>_TOKEN environment variable. If this credential provider is not listed, then the *_TOKEN environment variables will not work.

### [cargo:wincred](https://doc.rust-lang.org/cargo/reference/registry-authentication.html#cargowincred)

Uses the Windows Credential Manager to store tokens.

The credentials are stored as cargo-registry:<index-url> in the Credential Manager under “Windows Credentials”.

### [cargo:macos-keychain](https://doc.rust-lang.org/cargo/reference/registry-authentication.html#cargomacos-keychain)

Uses the macOS Keychain to store tokens.

The Keychain Access app can be used to view stored tokens.

### [cargo:libsecret](https://doc.rust-lang.org/cargo/reference/registry-authentication.html#cargolibsecret)

Uses [libsecret](https://wiki.gnome.org/Projects/Libsecret) to store tokens.

Any password manager with libsecret support can be used to view stored tokens. The following are a few examples (non-exhaustive):

- [GNOME Keyring](https://wiki.gnome.org/Projects/GnomeKeyring)
- [KDE Wallet Manager](https://apps.kde.org/kwalletmanager5/) (since KDE Frameworks 5.97.0)
- [KeePassXC](https://keepassxc.org/) (since 2.5.0)

### [cargo:token-from-stdout <command> <args>](https://doc.rust-lang.org/cargo/reference/registry-authentication.html#cargotoken-from-stdout-command-args)

Launch a subprocess that returns a token on stdout. Newlines will be trimmed.

- The process inherits the user’s stdin and stderr.
- It should exit 0 on success, and nonzero on error.
- [cargo login](https://doc.rust-lang.org/cargo/commands/cargo-login.html) and [cargo logout](https://doc.rust-lang.org/cargo/commands/cargo-logout.html) are not supported and return an error if used.

The following environment variables will be provided to the executed command:

- CARGO — Path to the cargo binary executing the command.
- CARGO_REGISTRY_INDEX_URL — The URL of the registry index.
- CARGO_REGISTRY_NAME_OPT — Optional name of the registry. Should not be used as a lookup key.

Arguments will be passed on to the subcommand.

## [Credential plugins](https://doc.rust-lang.org/cargo/reference/registry-authentication.html#credential-plugins)

For credential provider plugins that follow Cargo’s [credential provider protocol](https://doc.rust-lang.org/cargo/reference/credential-provider-protocol.html), the configuration value should be a string with the path to the executable (or the executable name if on the PATH).

For example, to install [cargo-credential-1password](https://crates.io/crates/cargo-credential-1password) from crates.io do the following:

Install the provider with cargo install cargo-credential-1password

In the config, add to (or create) registry.global-credential-providers:

```
[registry]
global-credential-providers = ["cargo:token", "cargo-credential-1password --account my.1password.com"]

```

The values in global-credential-providers are split on spaces into path and command-line arguments. To define a global credential provider where the path or arguments contain spaces, use the [[credential-alias] table](https://doc.rust-lang.org/cargo/reference/config.html#credential-alias).
