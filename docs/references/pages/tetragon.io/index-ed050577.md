---
url: "https://tetragon.io/"
title: "Tetragon - eBPF-based Security Observability and Runtime Enforcement"
site: "Tetragon - eBPF-based Security Observability and Runtime Enforcement"
captured: "2026-11-02T18:47:08Z"
via: "live"
sha256: "26ab0a02bc2a0cd5ee2cb2c38689594d69bcea0cf2d53b7bc5a1d642d2be9d5d"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Tetragon - eBPF-based Security Observability and Runtime Enforcement

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://tetragon.io/>

Tetragon is a sub-project under  Cilium and a proud  CNCF project

# eBPF-based Security Observability and Runtime Enforcement

Tetragon is a flexible Kubernetes-aware security observability and runtime enforcement tool that applies policy and filtering directly with eBPF, allowing for reduced observation overhead, tracking of any process, and real-time enforcement of policies.

[Try Tetragon](https://tetragon.io/docs/getting-started/) [Watch video](https://www.youtube.com/watch?v=kTGU-Nc2Db0)

## Tech leaders use Tetragon

-
-
-
-
-
-
-

## Why Tetragon?

-

### Minimal Overhead

eBPF enables deep observability with low performance overhead mitigating risks without the latency introduced by user-space processing.

-

### Kubernetes-Aware

Tetragon extends Cilium's design by recognizing workload identities like namespace and pod metadata, surpassing traditional observability.

-

### Simplified Operation

Tetragon offers pre-defined policy libraries for rapid deployment and operational insight, reducing setup time and complexity at scale.

-

### Kernel-level enforcement

Tetragon blocks malicious activities at the kernel level, closing the window for exploitation without succumbing to TOCTOU attack vectors.

-

### Real-time Policy Engine

Synchronous monitoring, filtering, and enforcement are performed entirely within the kernel using eBPF.

-

### Advanced Application Insights

Tetragon goes beyond traditional monitoring, capturing events like process execution, network communications, and file access.

## What can Tetragon do?

Cilium Tetragon is a flexible Kubernetes-aware security observability and runtime

[Read the documentation](https://tetragon.io/docs/use-cases/)

- Controlling binary execution (e.g. disallow binary execution from /tmp or allow binaries to be executed)
- Detect Linux Namespace & Privilege Changes
- Kubernetes Data Exfiltration
- File Integrity Monitoring
- And more!

## How does Tetragon work?

Tetragon monitors processes, syscalls, file and network activity in the kernel, correlating threats with network data to identify responsible binaries. It shares insights via JSON logs and a gRPC endpoint.

- [### Tetragon Policy Library

Catalog of vetted Tetragon policies for various use cases, implementing observability and runtime enforcement mechanisms.

Read more](https://tetragon.io/docs/policy-library/)
- [### Code of Conduct

To make Tetragon a welcoming and harassment-free experience for everyone, we follow the Cilium Code of Conduct.

Read more](https://github.com/cilium/cilium/blob/main/CODE_OF_CONDUCT.md)

## How to Install Tetragon?

-
-

Install Tetragon on Kubernetes Download via helm chart

#### Kubernetes

```

                    helm repo add cilium https://helm.cilium.io
                    helm repo update
                    helm install tetragon cilium/tetragon -n kube-system
                    kubectl rollout status -n kube-system ds/tetragon -w

```

Install on any Linux system Install and manage Tetragon via released packages

#### Linux

```

                    curl -LO https://github.com/cilium/tetragon/releases/download/v1.8.0/tetragon-v1.8.0-amd64.tar.gz
                    tar -xvf tetragon-v1.8.0-amd64.tar.gz
                    cd tetragon-v1.8.0-amd64/
                    sudo ./install.sh
                    sudo systemctl status tetragon

```

Deploy as a container Run the Tetragon image from the Tetragon quay repository

#### Container

```

                    docker run --name tetragon --rm -d
                    --pid=host --cgroupns=host --privileged
                    -v /sys/kernel/btf/vmlinux:/var/lib/tetragon/btf
                    quay.io/cilium/tetragon:v1.8.0

```

- [Read the documentation](https://tetragon.io/docs/getting-started/) [Watch tutorial](https://www.youtube.com/watch?v=VAk9W4Hc5iE)

## Exciting Updates and Announcements

[Highlights Innovations in Modern Security Approaches

- User Story

### Why This Social Networking Company Made Tetragon a Default for their Kubernetes Clusters

15 April, 2025](https://cilium.io/blog/2025/04/15/tetragon-social-networking-user-story/)

- [- Tutorial
- Community

### Tetragon Detects io_uring File Access

25 Dec, 2025](https://www.youtube.com/watch?v=ujZnwkC08Hk)
- [- Conference
- External

### Tetragon related talks at Kubecon EU 2025

21 Feb, 2025](https://cilium.io/blog/2025/02/21/cilium-at-kubecon-eu-2025/)
- [- Article
- External

### Tetragon: 2024 in Review

31 Dec, 2024](https://cilium.io/blog/2024/12/31/tetragon-2024-year-in-review/)
- [- eCHO episode
- External

### eCHO Episode 134: Tetragon in Action

03 May, 2024](https://www.youtube.com/watch?v=VGQT8DVxAGo)
- [- Tutorial
- External

### Restrict Access to Secure Files with Tetragon | eBPF Runtime Enforcement

22 Feb, 2024](https://www.youtube.com/watch?v=SiQm6N3ucyc)
- [- Tutorial
- External

### eBPF for Runtime Enforcement | Tetragon Introduction and Overview

21 Feb, 2024](https://www.youtube.com/watch?v=MObLmvBeu00)
- [- Article
- Community

### Tetragon community Call

20 Feb, 2024](https://docs.google.com/document/d/1BFMJLdtisiCSLfMct0GHof_ioL-5QVNLEaeMSlk_7Eo/edit?usp=sharing)
- [- Conference Talk
- External

### Coping with Zero Days with Cilium Tetragon

17 Sep, 2024](https://www.youtube.com/watch?v=m50WKCPjXuI)

## Watch videos on Tetragon [Show all videos](https://tetragon.io/docs/resources#conference-talks)

- [### The Next Log4jshell?! Preparing for CVEs with eBPF!

John Fastabend & Natalia Reka Ivanko • May 1, 2023](https://www.youtube.com/watch?v=u8HKg5pENj4)
- [### Tutorial: Getting Familiar with Security Observability Using eBPF & Cilium Tetragon

Duffie Cooley & Raphaël Pinson • Apr 30, 2023](https://www.youtube.com/watch?v=kTGU-Nc2Db0)
- [### Securing the Superpowers: Who Loaded That EBPF Program?

John Fastabend & Natalia Reka Ivanko • Feb 3, 2023](https://www.youtube.com/watch?v=UBVTJ0LeXxc)

[Show all videos](https://tetragon.io/docs/resources#conference-talks)

## Security Bugs

We strongly encourage you to report security vulnerabilities to our private security mailing list: security@cilium.io - first, before disclosing them in any public forums. This is a private mailing list where only members of the Cilium security team are subscribed to, and is treated as top priority.

Report a bug

## Event in a box

All you need for a successful Tetragon talk. Are you passionate about Tetragon and want to share your experience with the broader community? We want to help you tell your story! Whether writing a blog post, creating a YouTube or Twitch video, or speaking at a conference, we will help you deliver your Tetragon story to a broad audience.

[Request event help](https://cilium.io/telling-story/)

## Welcome to the community!

- [Contribute on Github](https://github.com/cilium/tetragon/)
- [Join our Slack channel](https://cilium.slack.com/join/shared_invite/zt-2fofar18z-~IbPR9AIO3iJufu95FQm2w#/shared-invite/email)
- [Follow us on Bluesky](https://bsky.app/profile/cilium.io)
- [Follow us on LinkedIn](https://www.linkedin.com/company/cilium/)
- [Explore Tetragon labs](https://isovalent.com/labs/tetragon-getting-started/)

-

## What is a Tracing Policy?

Tracing Policies define what situations Tetragon should react to...

- 1
- 2
- 3

-

## Showcase Tetragon: Slides for Speakers

We've created a slide deck for talks, presentations, and demos on Tetragon. Feel free to use it as-is or customize it to fit your specific needs.

[See presentation](https://docs.google.com/presentation/d/1jNJ0pPkHjvv4QN_K01FGbNLaWD5gqgbwxCNAehn4PBQ/edit#slide=id.p)

## Telling the Tetragon Story

Creating an abstract, putting a presentation together, or writing a blog post doesn’t come naturally to everyone. If you are eager to tell your Cilium story and need help, we’re here for you.

Not a native speaker and/or not confident about your writing skills? No worries. Bring the story and we’ll help you tell it in an engaging way.
