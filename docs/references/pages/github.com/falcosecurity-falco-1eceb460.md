---
url: "https://github.com/falcosecurity/falco"
title: "GitHub - falcosecurity/falco: Cloud Native Runtime Security"
site: "GitHub"
captured: "2026-11-02T18:55:59Z"
via: "live"
sha256: "1bcceeb1262137059ff0c6f1d07e9280908d2b5104263b8076ddb20be2829c88"
wayback: "https://web.archive.org/web/20260904201456/https://github.com/falcosecurity/falco"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
---

# GitHub - falcosecurity/falco: Cloud Native Runtime Security

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/falcosecurity/falco>
> Snapshot: <https://web.archive.org/web/20260904201456/https://github.com/falcosecurity/falco>

# Falco

https://github.com/falcosecurity/falco#falco

https://github.com/falcosecurity/falco/releases/latest https://github.com/falcosecurity/falco/releases/latest https://github.com/falcosecurity/falco/blob/master/COPYING https://falco.org/docs

https://github.com/falcosecurity/evolution/blob/main/REPOSITORIES.md#core-scope https://github.com/falcosecurity/evolution/blob/main/REPOSITORIES.md#stable https://scorecard.dev/viewer/?uri=github.com/falcosecurity/falco https://bestpractices.coreinfrastructure.org/projects/2317

https://falco.org

[Falco](https://falco.org/) is a cloud native runtime security tool for Linux operating systems. It is designed to detect and alert on abnormal behavior and potential security threats in real-time.

At its core, Falco is a kernel monitoring and detection agent that observes events, such as syscalls, based on custom rules. Falco can enhance these events by integrating metadata from the container runtime and Kubernetes. The collected events can be analyzed off-host in SIEM or data lake systems.

Falco, originally created by [Sysdig](https://sysdig.com), is a graduated project under the [Cloud Native Computing Foundation](https://cncf.io) (CNCF) used in production by various [organisations](https://github.com/falcosecurity/falco/blob/master/ADOPTERS.md).

For detailed technical information and insights into the cyber threats that Falco can detect, visit the official [Falco](https://falco.org/) website.

For comprehensive information on the latest updates and changes to the project, please refer to the [Change Log](https://github.com/falcosecurity/falco/blob/master/CHANGELOG.md).

## The Falco Project

https://github.com/falcosecurity/falco#the-falco-project

The Falco Project codebase is maintained under the [falcosecurity GitHub organization](https://github.com/falcosecurity). The primary repository, [falcosecurity/falco](https://github.com/falcosecurity/falco), holds the source code for the Falco binary, while other sub-projects are hosted in dedicated repositories. This approach of isolating components into specialized repositories enhances modularity and focused development. Notable [core repositories](https://github.com/falcosecurity/evolution?tab=readme-ov-file#core) include:

- [falcosecurity/libs](https://github.com/falcosecurity/libs): This repository hosts Falco's core libraries, which constitute the majority of the binary’s source code and provide essential features, such as kernel drivers.
- [falcosecurity/rules](https://github.com/falcosecurity/rules): It contains the official ruleset for Falco, offering pre-defined detection rules for various security threats and abnormal behaviors.
- [falcosecurity/plugins](https://github.com/falcosecurity/plugins): This repository supports integration with external services through plugins that extend Falco's capabilities beyond syscalls and container events, with plans for evolving specialized functionalities in future releases.
- [falcosecurity/falcoctl](https://github.com/falcosecurity/falcoctl): A command-line utility designed for managing and interacting with Falco.
- [falcosecurity/charts](https://github.com/falcosecurity/charts): This repository publishes Helm charts for deploying Falco and its ecosystem. The Falco chart source lives in [chart/falco](https://github.com/falcosecurity/falco/blob/master/chart/falco).

For further insights into our repositories and additional details about our governance model, please visit the official hub of The Falco Project: [falcosecurity/evolution](https://github.com/falcosecurity/evolution).

## Getting Started with Falco

https://github.com/falcosecurity/falco#getting-started-with-falco

If you're new to Falco, begin your journey with our [Getting Started](https://falco.org/docs/getting-started/) guide. For production deployments, please refer to our comprehensive [Setup](https://falco.org/docs/setup/) documentation.

As final recommendations before deploying Falco, verify environment compatibility, define your detection goals, optimize performance, choose the appropriate build, and plan for SIEM or data lake integration to ensure effective incident response.

### Demo Environment

https://github.com/falcosecurity/falco#demo-environment

A demo environment is provided via a docker-compose file that can be started on a docker host which includes falco, falcosidekick, falcosidekick-ui and its required redis database. For more information see the [docker-compose section](https://github.com/falcosecurity/falco/blob/master/docker/docker-compose)

## Join the Community

https://github.com/falcosecurity/falco#join-the-community

To get involved with the Falco Project please visit the [Community](https://github.com/falcosecurity/community) repository to find more information and ways to get involved.

If you have any questions about Falco or contributing, do not hesitate to file an issue or contact the Falco maintainers and community members for assistance.

How to reach out?

- Join the [#falco](https://kubernetes.slack.com/messages/falco) channel on the [Kubernetes Slack](https://slack.k8s.io).
- Join the [Falco mailing list](https://lists.cncf.io/g/cncf-falco-dev).
- File an [issue](https://github.com/falcosecurity/falco/issues) or make feature requests.

## Commitment to Falco's Own Security

https://github.com/falcosecurity/falco#commitment-to-falcos-own-security

Full reports of various security audits can be found [here](https://github.com/falcosecurity/falco/blob/master/audits).

In addition, you can refer to the [falco](https://github.com/falcosecurity/falco/security) and [libs](https://github.com/falcosecurity/libs/security) security sections for detailed updates on security advisories and policies.

To report security vulnerabilities, please follow the community process outlined in the documentation found [here](https://github.com/falcosecurity/.github/blob/main/SECURITY.md).

## Building

https://github.com/falcosecurity/falco#building

For comprehensive, step-by-step instructions on building Falco from source, please refer to the [official documentation](https://falco.org/docs/developer-guide/source/).

## Testing

https://github.com/falcosecurity/falco#testing

Expand Testing Instructions

Falco's [Build Falco from source](https://falco.org/docs/developer-guide/source/) is the go-to resource to understand how to build Falco from source. In addition, the [falcosecurity/libs](https://github.com/falcosecurity/libs) repository offers additional valuable information about tests and debugging of Falco's underlying libraries and kernel drivers.

Here's an example of a cmake command that will enable everything you need for all unit tests of this repository:

```
cmake \
-DUSE_BUNDLED_DEPS=ON \
-DBUILD_DRIVER=ON \
-DBUILD_FALCO_MODERN_BPF=ON \
-DCREATE_TEST_TARGETS=ON \
-DBUILD_FALCO_UNIT_TESTS=ON ..;
```

Build and run the unit test suite:

```
nproc=$(grep processor /proc/cpuinfo | tail -n 1 | awk '{print $3}');
make -j$(($nproc-1)) falco_unit_tests;
# Run the tests
sudo ./unit_tests/falco_unit_tests;
```

Optionally, build the driver of your choice and test run the Falco binary to perform manual tests.

Lastly, The Falco Project has moved its Falco regression tests to [falcosecurity/testing](https://github.com/falcosecurity/testing).

## How to Contribute

https://github.com/falcosecurity/falco#how-to-contribute

Please refer to the [Contributing](https://github.com/falcosecurity/.github/blob/main/CONTRIBUTING.md) guide and the [Code of Conduct](https://github.com/falcosecurity/evolution/blob/main/CODE_OF_CONDUCT.md) for more information on how to contribute.

## FAQs

https://github.com/falcosecurity/falco#faqs

### Why is Falco in C++ rather than Go or {language}?

https://github.com/falcosecurity/falco#why-is-falco-in-c-rather-than-go-or-language

Expand Information

- The first lines of code at the base of Falco were written some time ago, where Go didn't yet have the same level of maturity and adoption as today.
- The Falco execution model is sequential and mono-thread due to the statefulness requirements of the tool, and so most of the concurrency-related selling points of the Go runtime would not be leveraged at all.
- The Falco code deals with very low-level programming in many places, and we all know that interfacing Go with C is possible but brings tons of complexity and tradeoffs to the table.
- As a security tool meant to consume a crazy high throughput of events per second, Falco needs to squeeze performance in all hot paths at runtime and requires deep control on memory allocation, which the Go runtime can't provide (there's also garbage collection involved).
- Although Go didn't suit the engineering requirements of the core of Falco, we still thought that it could be a good candidate for writing Falco extensions through the plugin system. This is the main reason we gave special attention and high priority to the development of the plugin-sdk-go.
- Go is not a requirement for having statically-linked binaries. In fact, we provide fully-static Falco builds since few years. The only issue with those is that the plugin system can't be supported with the current dynamic library model we currently have.
- The plugin system has been envisioned to support multiple languages, so on our end maintaining a C-compatible codebase is the best strategy to ensure maximum cross-language compatibility.
- In general, plugins have GLIBC requirements/dependencies because they have low-level C bindings required for dynamic loading. A potential solution for the future could be to also support plugin to be statically-linked at compilation time and so released as bundled in the Falco binary. Although no work started yet in this direction, this would solve most issues you reported and would provide a totally-static binary too. Of course, this would not be compatible with dynamic loading anymore, but it may be a viable solution for our static-build flavor of Falco.
- Memory safety is definitely a concern and we try our best to keep an high level of quality even though C++ is quite error prone. For instance, we try to use smart pointers whenever possible, we build the libraries with an address sanitizer in our CI, we run Falco through Valgrind before each release, and have ways to stress-test it to detect performance regressions or weird memory usage (e.g. https://github.com/falcosecurity/event-generator). On top of that, we also have third parties auditing the codebase by time to time. None of this make a perfect safety standpoint of course, but we try to maximize our odds. Go would definitely make our life easier from this perspective, however the tradeoffs never made it worth it so far due to the points above.
- The C++ codebase of falcosecurity/libs, which is at the core of Falco, is quite large and complex. Porting all that code to another language would be a major effort requiring lots of development resource and with an high chance of failure and regression. As such, our approach so far has been to choose refactors and code polishing instead, up until we'll reach an optimal level of stability, quality, and modularity, on that portion of code. This would allow further developments to be smoother and more feasibile in the future.

### What's next for Falco?

https://github.com/falcosecurity/falco#whats-next-for-falco

Stay updated with Falco's evolving capabilities by exploring the [Falco Roadmap](https://github.com/orgs/falcosecurity/projects/5), which provides insights into the features currently under development and planned for future releases.

## License

https://github.com/falcosecurity/falco#license

Falco is licensed to you under the [Apache 2.0](https://github.com/falcosecurity/falco/blob/master/COPYING) open source license.

## Resources

https://github.com/falcosecurity/falco#resources

- [Governance](https://github.com/falcosecurity/evolution/blob/main/GOVERNANCE.md)
- [Code Of Conduct](https://github.com/falcosecurity/evolution/blob/main/CODE_OF_CONDUCT.md)
- [Maintainers Guidelines](https://github.com/falcosecurity/evolution/blob/main/MAINTAINERS_GUIDELINES.md)
- [Maintainers List](https://github.com/falcosecurity/evolution/blob/main/MAINTAINERS.md)
- [Repositories Guidelines](https://github.com/falcosecurity/evolution/blob/main/REPOSITORIES.md)
- [Repositories List](https://github.com/falcosecurity/evolution/blob/main/README.md#repositories)
- [Adopters List](https://github.com/falcosecurity/falco/blob/master/ADOPTERS.md)
- [Release Process](https://github.com/falcosecurity/falco/blob/master/RELEASE.md)
- [Setup documentation](https://falco.org/docs/setup/)
- [Troubleshooting](https://falco.org/docs/troubleshooting/)
