---
url: "https://trivy.dev/latest/docs/target/rootfs/"
title: "Trivy - Rootfs"
captured: "2026-11-02T18:47:12Z"
via: "live"
source_url: "https://trivy.dev/docs/latest/target/rootfs/"
sha256: "78b12bbeb87a3d826761700c987294a24566245c167b7dbca951cd6271131c34"
wayback: "https://web.archive.org/web/20251105222612/https://trivy.dev/latest/docs/target/rootfs/"
cited_in:
  - "docs/research/vulnerability-intelligence-and-feeds.md"
---

# Trivy - Rootfs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://trivy.dev/latest/docs/target/rootfs/>
> Snapshot: <https://web.archive.org/web/20251105222612/https://trivy.dev/latest/docs/target/rootfs/>

https://github.com/aquasecurity/trivy/blob/main/docs/target/rootfs.md

# Rootfs[¶](https://trivy.dev/docs/latest/target/rootfs/#rootfs)

Rootfs scanning is for special use cases such as

- Host machine
- [Root filesystem](https://trivy.dev/docs/latest/advanced/container/embed-in-dockerfile/)
- [Unpacked filesystem](https://trivy.dev/docs/latest/advanced/container/unpacked-filesystem/)

```
$ trivy rootfs /path/to/rootfs

```

Note

Rootfs scanning works differently from the Filesystem scanning. You should use trivy fs to scan your local projects in CI/CD. See [here](https://trivy.dev/docs/latest/scanner/vulnerability/) for the differences.

Note

Scanning vulnerabilities for Red Hat has a limitation, see the [Red Hat](https://trivy.dev/docs/latest/coverage/os/rhel/#content-manifests) page for details.

## Performance Optimization[¶](https://trivy.dev/docs/latest/target/rootfs/#performance-optimization)

By default, Trivy traverses all files from the specified root directory to find target files for scanning. However, when you only need to scan specific files with absolute paths, you can avoid this traversal, which makes scanning faster. For example, when scanning only OS packages, no full traversal is performed:

```
$ trivy rootfs --pkg-types os --scanners vuln /

```

When scanning language-specific packages or secrets, traversal is necessary because the location of these files is unknown. If you want to exclude specific directories from scanning for better performance, you can use the [--skip-dirs](https://trivy.dev/docs/latest/configuration/skipping/) option.
