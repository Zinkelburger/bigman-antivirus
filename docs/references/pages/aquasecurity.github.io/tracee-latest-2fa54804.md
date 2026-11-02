---
url: "https://aquasecurity.github.io/tracee/latest/"
title: "Tracee"
captured: "2026-11-02T18:21:40Z"
via: "live"
sha256: "2f27cd6094038daa50be47e5aa171fcfa28a66c121891c52eb187df6a6f2ea67"
wayback: "https://web.archive.org/web/20260924152705/https://aquasecurity.github.io/tracee/latest/"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# Tracee

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://aquasecurity.github.io/tracee/latest/>
> Snapshot: <https://web.archive.org/web/20260924152705/https://aquasecurity.github.io/tracee/latest/>

# Home

👋 Welcome to Tracee Documentation! To help you get around, please notice the different sections at the top global menu:

- You are currently in the [Getting Started](https://aquasecurity.github.io/tracee/latest/) section where you can find general information and help with first steps.
- In the [Tutorials](https://aquasecurity.github.io/tracee/latest/tutorials/overview/) section you can find step-by-step guides that help you accomplish specific tasks.
- In the [Docs](https://aquasecurity.github.io/tracee/latest/docs/overview/) section you can find the complete reference documentation for all of the different features and settings that Tracee has to offer.
- In the [Contributing](https://aquasecurity.github.io/tracee/latest/contributing/overview/) section you can find technical developer documentation and contribution guidelines.

Before moving on, please consider giving us a GitHub star ⭐️. Thank you!

## About Tracee[¶](https://aquasecurity.github.io/tracee/latest/#about-tracee)

Tracee is a runtime security and observability tool that helps you understand how your system and applications behave using [eBPF technology](https://ebpf.io/what-is-ebpf/). It provides deep visibility into Linux systems by monitoring system calls, network activity, and file operations in real-time.

### What Tracee Does[¶](https://aquasecurity.github.io/tracee/latest/#what-tracee-does)

- 🔍 System Monitoring: Tracks system calls, process execution, file operations, and network activity
- 🛡️ Security Detection: Identifies suspicious behavior patterns and potential security threats
- 📊 Observability: Provides detailed insights into application and system behavior
- 🚨 Real-time Alerts: Generates events for immediate threat detection and response

### Key Features[¶](https://aquasecurity.github.io/tracee/latest/#key-features)

- Zero Code Changes: Monitor existing applications without modification
- Low Overhead: Minimal performance impact using efficient eBPF programs
- Container Aware: Native support for containerized environments and Kubernetes
- Flexible Policies: Customize what to monitor and how to respond to events
- Rich Event Data: Detailed context including process lineage, file paths, and network connections

### Use Cases[¶](https://aquasecurity.github.io/tracee/latest/#use-cases)

- Security Monitoring: Detect malware, privilege escalation, and suspicious activity
- Compliance: Monitor file access, data exfiltration, and system changes
- Troubleshooting: Debug application issues and system behavior
- Forensics: Investigate security incidents with detailed audit trails

## Quickstart[¶](https://aquasecurity.github.io/tracee/latest/#quickstart)

To quickly try Tracee use one of the following snippets. For a more complete installation guide, check out the [Installation section](https://aquasecurity.github.io/tracee/latest/docs/install/).
Tracee should run on most common Linux distributions and kernels. For compatibility information see the [Prerequisites](https://aquasecurity.github.io/tracee/latest/docs/install/prerequisites/) page. Mac users, please read [this FAQ](https://aquasecurity.github.io/tracee/latest/docs/advanced/mac/).

### Using Docker[¶](https://aquasecurity.github.io/tracee/latest/#using-docker)

```
docker run --name tracee -it --rm \
  --pid=host --cgroupns=host --privileged \
  -v /etc/os-release:/etc/os-release-host:ro \
  -v /var/run:/var/run:ro \
  aquasec/tracee:latest

```

For a complete walkthrough please see the [Docker getting started guide](https://aquasecurity.github.io/tracee/latest/docs/install/docker/).

### On Kubernetes[¶](https://aquasecurity.github.io/tracee/latest/#on-kubernetes)

```
helm repo add aqua https://aquasecurity.github.io/helm-charts/
helm repo update
helm install tracee aqua/tracee --namespace tracee --create-namespace

```

```
kubectl logs --follow --namespace tracee daemonset/tracee

```

For a complete walkthrough please see the [Kubernetes getting started guide](https://aquasecurity.github.io/tracee/latest/docs/install/kubernetes/).

## Next Steps[¶](https://aquasecurity.github.io/tracee/latest/#next-steps)

After trying the quickstart, here's how to dive deeper:

### 🎯 For Security Analysts[¶](https://aquasecurity.github.io/tracee/latest/#for-security-analysts)

- [Events Documentation](https://aquasecurity.github.io/tracee/latest/docs/events/): Learn about security events and signatures
- [Policies Guide](https://aquasecurity.github.io/tracee/latest/docs/policies/): Create custom detection rules
- [Kubernetes Tutorial](https://aquasecurity.github.io/tracee/latest/tutorials/k8s-policies/): Monitor containerized workloads

### 🛠️ For DevOps Engineers[¶](https://aquasecurity.github.io/tracee/latest/#for-devops-engineers)

- [Installation Guide](https://aquasecurity.github.io/tracee/latest/docs/install/): Production deployment options
- [Configuration](https://aquasecurity.github.io/tracee/latest/docs/install/config/): Customize Tracee for your environment
- [Outputs](https://aquasecurity.github.io/tracee/latest/docs/outputs/): Integrate with your monitoring stack

### 👨‍💻 For Developers[¶](https://aquasecurity.github.io/tracee/latest/#for-developers)

- [Custom Events](https://aquasecurity.github.io/tracee/latest/docs/events/custom/overview/): Create your own detection logic
- [Contributing Guide](https://aquasecurity.github.io/tracee/latest/contributing/overview/): Help improve Tracee
- [Building from Source](https://aquasecurity.github.io/tracee/latest/contributing/building/building/): Development setup

## Contributing[¶](https://aquasecurity.github.io/tracee/latest/#contributing)

Join the community, and talk to us about any matter in the [GitHub Discussions](https://github.com/aquasecurity/tracee/discussions) or [Slack](https://slack.aquasec.com).
If you run into any trouble using Tracee or you would like to give us user feedback, please [create an issue.](https://github.com/aquasecurity/tracee/issues)

Find more information on [contribution documentation](https://aquasecurity.github.io/tracee/latest/contributing/overview/).

## More about Aqua Security[¶](https://aquasecurity.github.io/tracee/latest/#more-about-aqua-security)

Tracee is an [Aqua Security](https://aquasec.com) open source project.
Learn about our open source work and portfolio [here](https://www.aquasec.com/products/open-source-projects/).
