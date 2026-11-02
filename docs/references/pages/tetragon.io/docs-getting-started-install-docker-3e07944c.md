---
url: "https://tetragon.io/docs/getting-started/install-docker/"
title: "Quick Local Docker Install"
site: "Tetragon - eBPF-based Security Observability and Runtime Enforcement"
captured: "2026-11-02T18:47:09Z"
via: "live"
sha256: "233c4c336b4c591b1fccad6469c85fc9d02abd42965e617429b0fb1e962c181d"
wayback: "https://web.archive.org/web/20260715125754/https://tetragon.io/docs/getting-started/install-docker/"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Quick Local Docker Install

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://tetragon.io/docs/getting-started/install-docker/>
> Snapshot: <https://web.archive.org/web/20260715125754/https://tetragon.io/docs/getting-started/install-docker/>

# Quick Local Docker Install

Discover and experiment with Tetragon on your local Linux host

Note

This guide has been tested on Ubuntu 22.04 and 22.10 with respectively kernel 5.15.0 and 5.19.0 on amd64 and arm64 but [any recent distribution](https://github.com/libbpf/libbpf#bpf-co-re-compile-once--run-everywhere) shipping with a relatively recent kernel should work. See the FAQ for further details on the [recommended kernel versions](https://tetragon.io/docs/installation/faq/#what-is-the-minimum-linux-kernel-version-to-run-tetragon).

Note that you cannot run Tetragon using Docker Desktop on macOS because of a limitation of the Docker Desktop Linux virtual machine. Learn more about this issue and how to run Tetragon on a Mac computer in [this section of the FAQ page](https://tetragon.io/docs/installation/faq/#can-i-run-tetragon-on-mac-computers).

## Start Tetragonhttps://tetragon.io/docs/getting-started/install-docker/#start-tetragon

The easiest way to start experimenting with Tetragon is to run it via Docker using the released container images.

```
docker run -d --name tetragon --rm --pull always \
    --pid=host --cgroupns=host --privileged             \
    -v /sys/kernel/btf/vmlinux:/var/lib/tetragon/btf    \
    quay.io/cilium/tetragon:v1.8.0

```

This will start Tetragon in a privileged container running in the background. Running Tetragon as a privileged container is required to load and attach BPF programs. See the [Installation and Configuration](https://tetragon.io/docs/installation/) section for more details.

## Run demo applicationhttps://tetragon.io/docs/getting-started/install-docker/#run-demo-application

To explore Tetragon it is helpful to have a sample workload. You can use Cilium’s demo application with [this Docker Compose file](https://github.com/cilium/tetragon/blob/main/examples/quickstart/docker-compose.yaml), but any workload would work equally well:

```
curl -LO https://github.com/cilium/tetragon/raw/main/examples/quickstart/docker-compose.yaml
docker compose -f docker-compose.yaml up -d

```

You can use docker container ls to verify that the containers are up and running. It might take a short amount of time for the container images to download.

```
CONTAINER ID   IMAGE                             COMMAND                  CREATED          STATUS          PORTS                                   NAMES
cace79752d94   quay.io/cilium/json-mock:v1.3.8   "bash /run.sh"           34 seconds ago   Up 33 seconds                                           starwars-xwing-1
4f8422b43b5b   quay.io/cilium/json-mock:v1.3.8   "bash /run.sh"           34 seconds ago   Up 33 seconds                                           starwars-tiefighter-1
7b60618ca8bd   quay.io/cilium/starwars:v2.1      "/starwars-docker --…"   34 seconds ago   Up 33 seconds   0.0.0.0:8080->80/tcp, :::8080->80/tcp   starwars-deathstar-1

```

## What’s nexthttps://tetragon.io/docs/getting-started/install-docker/#whats-next

Check for [execution events](https://tetragon.io/docs/getting-started/execution/).

Last modified July 29, 2024: [doc: use tag variable for docker-based commands (4cd80e0c3)](https://github.com/cilium/tetragon/commit/4cd80e0c361ebeb8467a7aecba96d34f2b125abc)
