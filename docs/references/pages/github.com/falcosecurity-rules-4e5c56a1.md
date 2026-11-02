---
url: "https://github.com/falcosecurity/rules"
title: "GitHub - falcosecurity/rules: Falco rule repository"
site: "GitHub"
captured: "2026-11-02T18:56:39Z"
via: "live"
sha256: "4ded10635a97ba6a9068f54757918e7694db212ef2331e512dc47d507e69541a"
wayback: "https://web.archive.org/web/20251129171920/https://github.com/falcosecurity/rules"
cited_in:
  - "docs/threats/04-host-integrity-and-hardening-audit.md"
  - "docs/threats/05-malware-families-and-detection.md"
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# GitHub - falcosecurity/rules: Falco rule repository

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/falcosecurity/rules>
> Snapshot: <https://web.archive.org/web/20251129171920/https://github.com/falcosecurity/rules>

# Falco Rules

https://github.com/falcosecurity/rules#falco-rules

https://github.com/falcosecurity/rules/releases/latest https://github.com/falcosecurity/falco/releases/latest

https://falco.org/docs/rules https://falcosecurity.github.io/rules/ https://falco.org/docs/rules/style-guide/

https://falco.org/docs/reference/rules/supported-fields/ https://github.com/falcosecurity/libs/blob/master/driver/event_table.c

https://github.com/falcosecurity/evolution/blob/main/REPOSITORIES.md#core-scope https://github.com/falcosecurity/evolution/blob/main/REPOSITORIES.md#stable https://github.com/falcosecurity/rules/blob/main/LICENSE https://falcosecurity.github.io/rules/

This repository has been created upon this [Proposal](https://github.com/falcosecurity/falco/blob/master/proposals/20221129-artifacts-distribution.md#move-falco-rules-to-their-own-repo) and contains the officially managed [Falco Rules](https://github.com/falcosecurity/rules#falco-rules) by The Falco Project, along with the [Falco Rules Files Registry](https://github.com/falcosecurity/rules#falco-rules-files-registry).

## Falco Rules

https://github.com/falcosecurity/rules#falco-rules-1

Rules tell [Falco](https://github.com/falcosecurity/falco) what to do. These rules are pre-defined detections for various security threats, abnormal behaviors, and compliance-related monitoring.

https://github.com/falcosecurity/rules/blob/main/docs/images/start.png Explore the [Official Documentation](https://falco.org/docs/rules) for a starting point and better understanding of rule concepts. Users can modify the community-contributed Falco rules to fit their needs or use them as examples. In most cases, users also create their own custom rules. Keep in mind that the rules in this repository are related to Falco's primary monitoring functions, specifically for syscalls and container events. Meanwhile, Falco plugin rules are stored within the respective subfolders of the [Plugins](https://github.com/falcosecurity/plugins) repository.

https://github.com/falcosecurity/rules/blob/main/docs/images/insight.png Because Falco rules, especially Sandbox and Incubating rules, are dynamic, it's crucial to stay updated. As threats and systems evolve, Falco evolves with each release. Therefore, regularly check the [Rules Overview Document](https://falcosecurity.github.io/rules/), Falco's [Supported Fields](https://falco.org/docs/reference/rules/supported-fields/), and Falco's release notes with every new release. It is recommended to consistently use the most recent [Falco Release](https://github.com/falcosecurity/falco/releases/latest) to avoid compatibility issues.

https://github.com/falcosecurity/rules/blob/main/docs/images/setting.png Important: The Falco Project only guarantees that the most recent rules releases are compatible with the latest Falco release. Discover all rule files in the [rules/](https://github.com/falcosecurity/rules/blob/main/rules) folder. Refer to our [Release Process](https://github.com/falcosecurity/rules/blob/main/RELEASE.md) and [Rules Maturity Framework](https://github.com/falcosecurity/rules/blob/main/CONTRIBUTING.md#rules-maturity-framework) for rule categorization, release procedures, and usage guidelines. Published upon tagging a new release, the maturity_stable rules in the [falco_rules.yaml](https://github.com/falcosecurity/rules/blob/main/rules/falco_rules.yaml) file are included in the Falco release package. Other maturity-level rules are released separately, requiring explicit installation and possible customization for effective [Adoption](https://github.com/falcosecurity/rules/blob/main/CONTRIBUTING.md#justification-of-rules-maturity-framework-for-falco-adoption).

https://github.com/falcosecurity/rules/blob/main/docs/images/announce.png Beginning with rules version 3.0.0, the required_engine_version follows [Semantic Versioning](https://semver.org/) and requires Falco version 0.37.0 or higher. Since rules version [2.0.0](https://github.com/falcosecurity/rules#falco-rules-2x), we've modified our rules' shipping and distribution process. With Falco >= 0.37.0, [Selective Rules Overrides](https://falco.org/docs/rules/overriding/) aim to further streamline the customization of rules. Since Falco 0.36.0, you can use the [rule_matching](https://github.com/falcosecurity/falco/blob/master/falco.yaml) config to resolve issues with rules overlapping, which is caused by the default "first match wins" principle. Starting from Falco 0.35.0, you have precise control over the syscalls that are being monitored, see [base_syscalls](https://github.com/falcosecurity/falco/blob/master/falco.yaml). Lastly, keep in mind that the [Rules Maturity Framework](https://github.com/falcosecurity/rules/blob/main/CONTRIBUTING.md#rules-maturity-framework) is a best effort on the part of the community, and ultimately, you have to decide if any rules are useful for your use cases.

https://github.com/falcosecurity/rules/blob/main/docs/images/cross.png Be cautious: The main branch has the latest development. Before using rules from the main branch, check for compatibility. Changes like new output fields might cause incompatibilities with the latest stable Falco release. The Falco Project recommends using rules only from the release branches. Lastly, we'd like to highlight the importance of regular engineering effort to effectively adopt Falco rules. Considering that each adopter's system and monitoring needs are unique, it's advisable to view the rules as examples.

https://github.com/falcosecurity/rules/blob/main/docs/images/arrow.png Debugging: Historically, we've noted that issues often arise either from incorrect configurations or genuine bugs, acknowledging that no software is entirely bug-free. The Falco Project continually updates its [Install and Operate](https://falco.org/docs/install-operate/) and [Troubleshooting](https://falco.org/docs/troubleshooting/) guides. We kindly suggest reviewing these guides. In the context of Falco rules, missing fields, such as container images, may be anticipated within our imperfection tolerances under certain circumstances. We are committed to addressing and resolving issues within our control.

## Falco Rules Files Registry

https://github.com/falcosecurity/rules#falco-rules-files-registry

The Falco Rules Files Registry contains metadata and information about rules files distributed by The Falco Project. The registry serves as an additional method of making the rules files available to the community, complementing the process of retrieving the rules files from this repository.

Note: Currently, the registry includes only rules for the syscall call data source; for other data sources see the [Plugins](https://github.com/falcosecurity/plugins) repository.

### Naming Convention

https://github.com/falcosecurity/rules#naming-convention

Rule files must be located in the [/rules](https://github.com/falcosecurity/rules/blob/main/rules) folder of this repository and are named according to the following convention: <ruleset>_rules.yaml.

The <ruleset> portion represents the ruleset name, which must be an alphanumeric string, separated by -, entirely in lowercase, and beginning with a letter.

Rule files are subsequently released using Git tags. The tag name should follow the pattern <ruleset>-rules-<version>, where <version> adheres to [Semantic Versioning](https://semver.org/). See [RELEASE](https://github.com/falcosecurity/rules/blob/main/RELEASE.md) document for more details about our release process.

For instance, the falco ruleset is stored under [/rules/falco_rules.yaml](https://github.com/falcosecurity/rules/blob/main/rules/falco_rules.yaml), and its version 1.0.0 was released using the [falco-rules-1.0.0](https://github.com/falcosecurity/rules/releases/tag/falco-rules-1.0.0) tag.

Note: This convention applies to this repository only. Falco application does not impose any naming convention for naming rule files.

## Falco Rules 2.x

https://github.com/falcosecurity/rules#falco-rules-2x

Since version 2.0.0, the rules distributed from this repository have been split into three parts:

- [Stable](https://github.com/falcosecurity/rules/blob/main/rules/falco_rules.yaml) Falco rules. Those are the only ones that are bundled in the Falco by default. It is very important to have a set of stable rules vetted by the community. To learn more about the criterias that are required for a rule to become stable, see the [Contributing](https://github.com/falcosecurity/rules/blob/main/CONTRIBUTING.md) guide.
- [Incubating](https://github.com/falcosecurity/rules/blob/main/rules/falco-incubating_rules.yaml) rules, which provide a certain level of robustness guarantee but have been identified by experts as catering to more specific use cases, which may or may not be relevant for each adopter.
- [Sandbox](https://github.com/falcosecurity/rules/blob/main/rules/falco-sandbox_rules.yaml) rules, which are more experimental.

Previously, Falco used to bundle all the community rules in its default distribution. Today you can choose which set of rules you want to load in your distribution, depending on your preferred installation method:

### Helm Chart

https://github.com/falcosecurity/rules#helm-chart

If you are using the official Helm chart, you can add the incubating and/or sandbox repository in your [falcoctl](https://github.com/falcosecurity/charts/blob/f1062000e2e61332b3a8ea892a1765e4f4a60ec6/falco/values.yaml#L406) config and by enabling them in the corresponding falco.yaml file.

For instance, in order to install the Helm chart and load all the available Falco rules with automatic update on all of them, you can run

```
helm install falco falcosecurity/falco --set "falcoctl.config.artifact.install.refs={falco-rules:2,falco-incubating-rules:2,falco-sandbox-rules:2}" --set "falcoctl.config.artifact.follow.refs={falco-rules:2,falco-incubating-rules:2,falco-sandbox-rules:2}" --set "falco.rules_file={/etc/falco/k8s_audit_rules.yaml,/etc/falco/rules.d,/etc/falco/falco_rules.yaml,/etc/falco/falco-incubating_rules.yaml,/etc/falco/falco-sandbox_rules.yaml}"

```

Where the option falcoctl.config.artifact.install.refs governs which rules are downloaded at startup, falcoctl.config.artifact.follow.refs identifies which rules are automatically updated and falco.rules_file indicates which rules are loaded by the engine.

### Host installation

https://github.com/falcosecurity/rules#host-installation

If you are managing your Falco installation you should be aware of which directories contain the rules. Those are governed by the rules_file configuration option in your [falco.yaml](https://github.com/falcosecurity/falco/blob/ab6d76e6d2a076ca1403c91aa62213d2cadb73ea/falco.yaml#L146). Normally, there is also a rules.d directory that you can use to upload extra rules or you can add your custom files.

Now you can simply download incubating or sandbox rules from the [rules](https://download.falco.org/?prefix=rules/) repository, uncompress and copy the file there.

## Contributing

https://github.com/falcosecurity/rules#contributing

If you are interested in helping and wish to contribute, we kindly request that you review our general [Contribution Guidelines](https://github.com/falcosecurity/.github/blob/master/CONTRIBUTING.md) and, more specifically, the dedicated [Rules Contributing](https://github.com/falcosecurity/rules/blob/main/CONTRIBUTING.md) guide hosted in this repository. Please be aware that our reviewers will ensure compliance with the rules' acceptance criteria.

## License

https://github.com/falcosecurity/rules#license

This project is licensed to you under the [Apache 2.0 Open Source License](https://github.com/falcosecurity/rules/blob/main/LICENSE).
