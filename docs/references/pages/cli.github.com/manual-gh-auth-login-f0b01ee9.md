---
url: "https://cli.github.com/manual/gh_auth_login"
title: "GitHub CLI"
site: "GitHub CLI"
captured: "2026-11-02T18:25:20Z"
via: "live"
sha256: "d68f3fe8acc7d469ad2c850d825b3fe0992e21127a8d41392b3887f9f2594337"
wayback: "https://web.archive.org/web/20261023160116/https://cli.github.com/manual/gh_auth_login"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# GitHub CLI

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://cli.github.com/manual/gh_auth_login>
> Snapshot: <https://web.archive.org/web/20261023160116/https://cli.github.com/manual/gh_auth_login>

## gh auth login

```
gh auth login [flags]

```

Authenticate with a GitHub host.

The default hostname is github.com. This can be overridden using the --hostname flag.

The default authentication mode is a web-based browser flow. After completion, an authentication token will be stored securely in the system credential store. If a credential store is not found or there is an issue using it gh will fallback to writing the token to a plain text file. See gh auth status for its stored location.

Alternatively, use --with-token to pass in a personal access token (classic) on standard input. The minimum required scopes for the token are: repo, read:org, and gist. Take care when passing a fine-grained personal access token to --with-token as the inherent scoping to certain resources may cause confusing behaviour when interacting with other resources. Favour setting GH_TOKEN for fine-grained personal access token usage.

Alternatively, gh will use the authentication token found in environment variables. This method is most suitable for "headless" use of gh such as in automation. See gh help environment for more info.

To use gh in GitHub Actions, add GH_TOKEN: ${{ github.token }} to env.

The git protocol to use for git operations on this host can be set with --git-protocol, or during the interactive prompting. Although login is for a single account on a host, setting the git protocol will take effect for all users on the host.

Specifying ssh for the git protocol will detect existing SSH keys to upload, prompting to create and upload a new key if one is not found. This can be skipped with --skip-ssh-key flag.

Use --expiring to prefer expiring credentials that gh refreshes automatically. This is a preference, not a guarantee: whether expiring tokens are issued depends on the host and the OAuth app configuration. A host without support issues a non-expiring token instead, and a host configured for them may issue expiring tokens even without the flag. Enable the expiring_credentials configuration to prefer expiring credentials without passing --expiring every time, and pass --expiring=false to request a non-expiring token even when that configuration is enabled. When using expiring credentials for git operations, git 2.46 or newer is recommended so gh can mark the token as non-cacheable; with older git a credential caching helper may store and reuse an expired token.

For more information on OAuth scopes, see https://docs.github.com/en/developers/apps/building-oauth-apps/scopes-for-oauth-apps/.

### Options

-c, --clipboard

Copy one-time OAuth device code to clipboard

--expiring

Prefer expiring credentials that gh refreshes automatically, if the host supports them

-p, --git-protocol <string>

The protocol to use for git operations on this host: {ssh|https}

-h, --hostname <string>

The hostname of the GitHub instance to authenticate with

--insecure-storage

Save authentication credentials in plain text instead of credential store

-s, --scopes <strings>

Additional authentication scopes to request

--skip-ssh-key

Skip generate/upload SSH key prompt

-w, --web

Open a browser to authenticate

--with-token

Read token from standard input

### Examples

```
# Start interactive setup
$ gh auth login

# Open a browser to authenticate and copy one-time OAuth code to clipboard
$ gh auth login --web --clipboard

# Authenticate against github.com by reading the token from a file
$ gh auth login --with-token < mytoken.txt

# Authenticate with specific host
$ gh auth login --hostname enterprise.internal
```

### See also

- [gh auth](https://cli.github.com/manual/gh_auth)
