---
url: "https://docs.socket.dev/docs/socket-firewall-free"
title: "Socket Firewall Free"
site: "Socket"
captured: "2026-11-02T18:29:25Z"
via: "live"
sha256: "180ba4a956669470b2b89b004f1e909339accd23c93087cb59f080092995e39f"
wayback: "https://web.archive.org/web/20260922035320/https://docs.socket.dev/docs/socket-firewall-free"
cited_in:
  - "REVIEW.md"
---

# Socket Firewall Free

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.socket.dev/docs/socket-firewall-free>
> Snapshot: <https://web.archive.org/web/20260922035320/https://docs.socket.dev/docs/socket-firewall-free>

- Socket Firewall

# Socket Firewall Free

Socket Firewall Free is a lightweight tool that protects developer machines in real time, blocking malicious dependencies before they ever reach your laptop or build system. It works out of the box — no API key and no configuration required.

```
$ sfw --help

Socket Firewall Free - Network security proxy for package managers

Runs any package manager command with network traffic filtering.

Usage:
  sfw <command>...
  sfw --help

  Examples:
    sfw npm install
    sfw pip install requests

Provided under the following license:
https://github.com/SocketDev/sfw-free/blob/main/README.md#license
```

##

Usage

https://docs.socket.dev/docs/socket-firewall-free#usage

Once installed, sfw should be prefixed to any supported package manager's command:

```
sfw npm install --save [[email protected]](https://docs.socket.dev/cdn-cgi/l/email-protection)
sfw cargo fetch
sfw uv pip install flask
```

Supported Package Managers:

- JavaScript/TypeScript: npm, yarn, pnpm (sfw might work with other package managers, but support is not guaranteed)
- Python: pip, uv
- Rust: cargo

Note: Socket Firewall Enterprise supports additional ecosystems including Go, Java, Ruby, and .NET. See the [Socket Firewall Overview](https://docs.socket.dev/docs/socket-firewall-overview) for details.

##

Installation

https://docs.socket.dev/docs/socket-firewall-free#installation

###

npm Installation

https://docs.socket.dev/docs/socket-firewall-free#npm-installation

The simplest installation method is via npm:

```
npm i -g sfw
# sfw can then be prefixed in front of package manager commands
sfw npm install --save dangerous-package
sfw pip install dangerous-package
```

###

Manual Installation

https://docs.socket.dev/docs/socket-firewall-free#manual-installation

You can also install manually by downloading the appropriate binary from the [releases](https://github.com/SocketDev/sfw-free/releases) page, moving it into your PATH, and making it executable. Keep in mind that, if you do this, you'll need to update the binary periodically as we drop support for older versions.

Note: sfw binaries are not currently signed. On macOS, you may need to configure the OS to allow execution of sfw. You can do this like so:

```
xattr -dr com.apple.quarantine ./path/to/sfw
```

Once the sfw binaries are signed, this step will no longer be necessary.

####

macOS (Apple Silicon)

https://docs.socket.dev/docs/socket-firewall-free#macos-apple-silicon

```
curl -L -o sfw https://github.com/SocketDev/sfw-free/releases/latest/download/sfw-free-macos-arm64
chmod +x sfw
sudo mv sfw /usr/local/bin/
```

####

macOS (Intel)

https://docs.socket.dev/docs/socket-firewall-free#macos-intel

```
curl -L -o sfw https://github.com/SocketDev/sfw-free/releases/latest/download/sfw-free-macos-x86_64
chmod +x sfw
sudo mv sfw /usr/local/bin/
```

####

Linux (x86_64)

https://docs.socket.dev/docs/socket-firewall-free#linux-x86_64

```
curl -L -o sfw https://github.com/SocketDev/sfw-free/releases/latest/download/sfw-free-linux-x86_64
chmod +x sfw
sudo mv sfw /usr/local/bin/
```

####

Linux (ARM64)

https://docs.socket.dev/docs/socket-firewall-free#linux-arm64

```
curl -L -o sfw https://github.com/SocketDev/sfw-free/releases/latest/download/sfw-free-linux-arm64
chmod +x sfw
sudo mv sfw /usr/local/bin/
```

####

Windows (x86_64)

https://docs.socket.dev/docs/socket-firewall-free#windows-x86_64

```
Invoke-WebRequest -Uri "https://github.com/SocketDev/sfw-free/releases/latest/download/sfw-free-windows-x86_64.exe" -OutFile "sfw.exe"
# Move to a directory in your PATH, such as:
Move-Item sfw.exe "%USERPROFILE%\AppData\Local\Microsoft\WindowsApps\sfw.exe"
```

####

Windows (ARM64)

https://docs.socket.dev/docs/socket-firewall-free#windows-arm64

```
Invoke-WebRequest -Uri "https://github.com/SocketDev/sfw-free/releases/latest/download/sfw-free-windows-arm64.exe" -OutFile "sfw.exe"
# Move to a directory in your PATH, such as:
Move-Item sfw.exe "%USERPROFILE%\AppData\Local\Microsoft\WindowsApps\sfw.exe"
```

Note: Additional installation methods may be provided in the future.

##

CI/CD Integration

https://docs.socket.dev/docs/socket-firewall-free#cicd-integration

###

GitHub Actions

https://docs.socket.dev/docs/socket-firewall-free#github-actions

If you'd like to use sfw in a CI/CD environment, we've supplied a GitHub Action you can use to ensure you're always running the latest version:

```
on: push

jobs:
  job-id:
    # Socket Firewall supports Linux, Windows, and macOS
    runs-on: ubuntu-latest
    steps:
      # add Socket Firewall to the runner environment
      - uses: socketdev/action@v1 # Please pin your actions!
        with:
          mode: firewall

      # setup your project (e.g. checkout, setup-node, etc...)
      - uses: actions/checkout@v5 # Please pin your actions!

      # example usage
      - run: sfw npm ci
      - run: sfw npm install lodash
      - run: sfw pip install requests
```

##

Configuration

https://docs.socket.dev/docs/socket-firewall-free#configuration

Socket Firewall Free is a zero-configuration tool that works immediately after installation. No API key, configuration files, or setup is required.

Note: Socket Firewall Enterprise offers configurable security policies, custom registry support, and organization-wide settings. See the [Socket Firewall Overview](https://docs.socket.dev/docs/socket-firewall-overview) for details.

##

Differences to Socket Firewall Enterprise

https://docs.socket.dev/docs/socket-firewall-free#differences-to-socket-firewall-enterprise

Socket Firewall Free is provided as a free tool for open source and commercial projects alike. In order for this to be sustainable, and to prevent abuse, the tool comes with a few limitations:

-

Custom registries are not supported. Socket Firewall Free only works with public registries. Socket Firewall Enterprise supports private and custom registries.

-

AI-detected malware generates warnings, not blocks. Socket utilizes AI scans and subsequent human review when identifying malware. Socket Firewall Free will display a warning if AI-detected potential malware is requested but will not block the network traffic. Only confirmed malware is blocked. Socket Firewall Enterprise allows you to configure this behavior.

-

Limited ecosystem support. Socket Firewall Free supports JavaScript/TypeScript, Python, and Rust. Socket Firewall Enterprise supports additional ecosystems including Go, Java, Ruby, and .NET.

-

Unknown packages are not blocked. Unknown or unscanned versions of packages will not be blocked by Socket Firewall Free. Socket Firewall Enterprise allows you to configure this behavior.

-

No allow-list mechanism. Socket Firewall Free provides no way to allow-list a particular package or version. Socket Firewall Enterprise includes allow-list capabilities.

-

Wrapper mode only. Socket Firewall Free only operates in wrapper mode (command prefix). Socket Firewall Enterprise also supports registry mode for centralized deployment.

-

No dashboard integration. Socket Firewall Free does not send data to your Socket dashboard. Socket Firewall Enterprise provides visibility into installation activity across your organization.

-

Telemetry is not configurable. Socket Firewall Free collects anonymous telemetry (see below). Socket Firewall Enterprise allows you to configure telemetry collection.

-

Rate limits. Use of Socket Firewall Free is not rate-limited except in cases where abuse is detected. The limits are very large and should not be reached in normal use.

For a detailed comparison, see the [Socket Firewall Overview](https://docs.socket.dev/docs/socket-firewall-overview).

##

Telemetry

https://docs.socket.dev/docs/socket-firewall-free#telemetry

Socket Firewall Free collects anonymous telemetry. We recognize this can cause reasonable concern for some, so we want to be transparent. Here's what we collect and why:

- A [unique, non-reversible identifier](https://www.npmjs.com/package/node-machine-id) per machine. This allows us to get a sense for usage trends.
- Information about blocked and permitted packages, e.g. name, namespace, version.
- Latency added by Socket Firewall when fetching packages. This will help us detect service degradation and meaningfully measure the improvements we make to the software.
- Errors. This is pretty bare bones; we do not include information from the local filesystem (e.g. call stacks, paths, file names, etc).
- GitHub organization name. To help us understand adoption across teams, Socket Firewall may collect the name of your GitHub organization from your configured remotes. We only collect the org name — never repository names, source code, or commit history.

Additional telemetry events may be added in future versions as we learn more about usage patterns and continue adding features. However, the information we collect will always be anonymous.

Note: Telemetry is configurable in Socket Firewall Enterprise.

##

License

https://docs.socket.dev/docs/socket-firewall-free#license

Socket Firewall Free is provided under the [PolyForm Shield License 1.0.0](https://polyformproject.org/licenses/shield/1.0.0). You can also find the license text in the [installer repository](https://github.com/SocketDev/sfw-free/blob/main/README.md#license).

##

FAQ

https://docs.socket.dev/docs/socket-firewall-free#faq

###

What ecosystems and package managers are supported?

https://docs.socket.dev/docs/socket-firewall-free#what-ecosystems-and-package-managers-are-supported

Socket Firewall Free supports:

- JavaScript & TypeScript: npm, yarn, pnpm
- Python: pip, uv
- Rust: cargo

Note: Socket Firewall Enterprise supports additional ecosystems including Go, Java (Maven, Gradle), Ruby (gem, Bundler), and .NET (NuGet). See the [Socket Firewall Overview](https://docs.socket.dev/docs/socket-firewall-overview) for details.

###

Does Socket Firewall Free protect against transitive dependencies?

https://docs.socket.dev/docs/socket-firewall-free#does-socket-firewall-free-protect-against-transitive-dependencies

Yes. Socket Firewall Free protects against malicious packages at any depth in your dependency tree, not just top-level dependencies.

###

Why am I not seeing Socket Firewall Free block a package I know is malicious?

https://docs.socket.dev/docs/socket-firewall-free#why-am-i-not-seeing-socket-firewall-free-block-a-package-i-know-is-malicious

Socket Firewall Free works by blocking network requests for package artifacts. If there are no network requests, as is the case when artifacts are cached locally, there is nothing for sfw to block. We recommend that you clear your package manager's cache (e.g. npm cache clean --force) and use sfw from that point onward.

Additionally, Socket Firewall Free only blocks confirmed malware that has been verified through human review. If a package is flagged by AI but not yet confirmed, Socket Firewall Free will display a warning but will not block it. Socket Firewall Enterprise allows you to configure this behavior.

###

How do I clear my package manager cache?

https://docs.socket.dev/docs/socket-firewall-free#how-do-i-clear-my-package-manager-cache

For the most up-to-date information, consult the documentation for the package manager in use. However, if you're already confident and just need a reminder:

```
# npm
npm cache clean --force

# yarn v1
yarn cache clean

# yarn v2
yarn cache clean --mirror

# pnpm
pnpm store prune

# pip
pip cache purge

# uv
uv cache clean

# cargo
cargo clean gc # clear only unused packages
rm -fr ~/.cargo/registry ~/.cargo/git # clear all cached packages
```

###

Can I use Socket Firewall Free with private registries?

https://docs.socket.dev/docs/socket-firewall-free#can-i-use-socket-firewall-free-with-private-registries

No. Custom and private registries are not supported by Socket Firewall Free. Socket Firewall Enterprise supports private registries.

###

What's the difference between AI-detected and confirmed malware?

https://docs.socket.dev/docs/socket-firewall-free#whats-the-difference-between-ai-detected-and-confirmed-malware

Socket utilizes AI scans and subsequent human review when identifying malware:

- AI-detected malware: Packages flagged by Socket's AI as potentially malicious but not yet verified by human review. Socket Firewall Free will display a warning but will not block these packages.
- Confirmed malware: Packages verified as malicious through human review. Socket Firewall Free will block these packages.

Socket Firewall Enterprise allows you to configure how AI-detected malware is handled, including the option to block it.

###

Does Socket Firewall Free work offline?

https://docs.socket.dev/docs/socket-firewall-free#does-socket-firewall-free-work-offline

No. Socket Firewall Free requires network connectivity to check packages against Socket's security API. If you lack network connectivity, Socket Firewall Free cannot function.

###

How do I report a false positive or false negative?

https://docs.socket.dev/docs/socket-firewall-free#how-do-i-report-a-false-positive-or-false-negative

If you believe you have identified a malicious package, or if your package has been incorrectly flagged, please reach out to [[email protected]](https://docs.socket.dev/cdn-cgi/l/email-protection#cbb8bebbbba4b9bf8bb8a4a8a0aebfe5afaebd)

###

Can I disable telemetry in Socket Firewall Free?

https://docs.socket.dev/docs/socket-firewall-free#can-i-disable-telemetry-in-socket-firewall-free

No. Telemetry is always enabled in Socket Firewall Free. Socket Firewall Enterprise allows you to configure telemetry collection.

##

Get Help

https://docs.socket.dev/docs/socket-firewall-free#get-help

If you have any trouble using sfw please reach out via issues at [sfw-free Issues](https://github.com/SocketDev/sfw-free/issues). If you'd like to learn more about Socket Firewall Enterprise, reach out to [[email protected]](https://docs.socket.dev/cdn-cgi/l/email-protection#cdbeaca1a8be8dbea2aea6a8b9e3a9a8bb) or [book a demo](https://socket.dev/demo) today.

Updated 2 months ago

Did this page help you?
