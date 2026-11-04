---
url: "https://github.com/kunai-project/kunai"
title: "GitHub - kunai-project/kunai: Threat-hunting tool for Linux"
site: "GitHub"
captured: "2026-11-02T19:04:44Z"
via: "live"
sha256: "1e84e0e59502407886ebdf0df1ce92069812158f6872a58581c392c21fe6a490"
wayback: "https://web.archive.org/web/20260802182518/https://github.com/kunai-project/kunai"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# GitHub - kunai-project/kunai: Threat-hunting tool for Linux

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/kunai-project/kunai>
> Snapshot: <https://web.archive.org/web/20260802182518/https://github.com/kunai-project/kunai>

https://github.com/kunai-project/kunai/blob/main/assets/logo.svg

https://github.com/0xrawsec/kunai/actions/workflows/ci.yml https://github.com/kunai-project/kunai/blob/main https://discord.com/invite/AUMaBvHvNU

https://github.com/0xrawsec/kunai/releases/latest https://why.kunai.rocks

# Leitmotiv

https://github.com/kunai-project/kunai#leitmotiv

Kunai is a powerful tool designed to bring actionable insights for tasks such as security monitoring and threat hunting on Linux systems. Think of it as the Linux counterpart to Sysmon on Windows, tailored for comprehensive and precise event monitoring.

## Why Kunai Stands Out

https://github.com/kunai-project/kunai#why-kunai-stands-out

- Chronologically Ordered Events: Events are processed and delivered in the exact order they occur.
- On-Host Correlation: Built-in capabilities for event enrichment and correlation to provide deeper context.
- Container-Aware: Fully compatible with Linux namespaces and container technologies, enabling complete tracing of container activities.

## How It Works

https://github.com/kunai-project/kunai#how-it-works

Kunai leverages eBPF (Extended Berkeley Packet Filter) technology, with kernel-level probes that capture critical events. These probes send data to a userland program, responsible for tasks like reordering, enriching, and correlating the collected events.

On the implementation side, Kunai is predominantly written in Rust, using the robust [Aya library](https://github.com/aya-rs/aya). This design ensures a self-contained standalone binary, embedding both the eBPF probes and the userland processing logic for ease of deployment.

# FAQ

https://github.com/kunai-project/kunai#faq

- Is it compatible with my OS/Kernel ? : Check out [the compatibility page](https://why.kunai.rocks/docs/compatibility)
- What kind of events can I get ? : Please take a read to [events documentation](https://why.kunai.rocks/docs/events/)
- Which version should I use ?: If it is just to test the tool, use the latest build as it is always the best in terms of features and bug fix. However keep in mind that events in non stable releases are subject to change.

# How to Build the Project?

https://github.com/kunai-project/kunai#how-to-build-the-project

Before proceeding, please note that a distribution-agnostic, pre-compiled version of Kunai is available on the [release page](https://github.com/kunai-project/kunai/releases/latest). If you simply want to try Kunai, you likely don’t need to build the project yourself.

## With a Docker image

https://github.com/kunai-project/kunai#with-a-docker-image

You can use a Docker image that includes everything needed to build the project easily: [Kunai build docker image](https://github.com/kunai-project/kunai-build-docker/).
This one-size-fits-all solution should work on any Linux distribution.

## Doing everything by hand

https://github.com/kunai-project/kunai#doing-everything-by-hand

### Requirements

https://github.com/kunai-project/kunai#requirements

Before being able to build everything, you need to install a couple of tools.

- to build many Rust projects (this one included), you need [rustup](https://www.rust-lang.org/tools/install)
- to build kunai you need: clang, libbpf-dev and [bpf-linker](https://github.com/aya-rs/bpf-linker)

Example of commands to install requirements on Ubuntu/Debian:

```
sudo apt update
sudo apt install -y clang libbpf-dev lld musl-tools

# assuming you have rustup and cargo installed
cargo install bpf-linker
```

### Building Kunai

https://github.com/kunai-project/kunai#building-kunai

Once you have the requirements installed, you are good to go.

#### Building debug version

https://github.com/kunai-project/kunai#building-debug-version

```
cargo build
# find your executable in: ./target/debug/kunai
```

#### Building for production.

https://github.com/kunai-project/kunai#building-for-production

For production deployment, we recommend building a static binary using MUSL:

```
# Build static release binary
cargo build --release --target x86_64-unknown-linux-musl

# The static binary will be available at:
./target/x86_64-unknown-linux-musl/release/kunai
```

Why MUSL? MUSL produces static binaries that are more portable across different Linux distributions, avoiding dependency issues with glibc versions. This is particularly important for production deployment where you may not control the target environment.

### Cross-compiling

https://github.com/kunai-project/kunai#cross-compiling

#### aarch64

https://github.com/kunai-project/kunai#aarch64

To cross-compile kunai for aarch64:

-

Install the aarch64 musl target:

```
rustup target add aarch64-unknown-linux-musl
```

-

Install the cross-compilation toolchain (Ubuntu/Debian example):

```
sudo dpkg --add-architecture arm64
sudo apt update
sudo apt install -y git clang libbpf-dev lld musl-tools
sudo apt install -y crossbuild-essential-arm64 musl-tools:arm64
```

-

Build for aarch64:

```
cargo build --release --target aarch64-unknown-linux-musl
```

-

The cross-compiled binary will be available at:

```
./target/aarch64-unknown-linux-musl/release/kunai

```

# Memory Profiling

https://github.com/kunai-project/kunai#memory-profiling

If one believes Kunai has an issue with memory, here is a way to profile it.

```
# compile kunai with debug information for all packages
RUSTFLAGS="-g" cargo build

# use heaptrack
sudo heaptrack kunai
```

# Related Work

https://github.com/kunai-project/kunai#related-work

Sysmon For Linux: https://github.com/Sysinternals/SysmonForLinux

# Acknowledgements

https://github.com/kunai-project/kunai#acknowledgements

- Thanks to all the people behind [Aya](https://github.com/aya-rs), this stuff is just awesome
- Special thanks to [@alessandrod](https://github.com/alessandrod) and [@vadorovsky](https://github.com/vadorovsky)
- Thanks to all the usual guys always supporting my crazy ideas

# Funding

https://github.com/kunai-project/kunai#funding

The NGSOTI project is dedicated to training the next generation of Security Operation Center (SOC) operators, focusing on the human aspect of cybersecurity. It underscores the significance of providing SOC operators with the necessary skills and open-source tools to address challenges such as detection engineering, incident response, and threat intelligence analysis. Involving key partners such as CIRCL, Restena, Tenzir, and the University of Luxembourg, the project aims to establish a real operational infrastructure for practical training. This initiative integrates academic curricula with industry insights, offering hands-on experience in cyber ranges.

NGSOTI is co-funded under Digital Europe Programme (DEP) via the ECCC (European cybersecurity competence network and competence centre).
