---
url: "https://pnpm.io/supply-chain-security"
title: "Mitigating supply chain attacks | pnpm"
site: "pnpm"
captured: "2026-11-02T18:40:24Z"
via: "live"
sha256: "e345ce133b58f139b08da40a5f4ded111c5574c8b5b2a8154abd7cb55ed61524"
wayback: "https://web.archive.org/web/20261022233946/https://pnpm.io/supply-chain-security"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Mitigating supply chain attacks | pnpm

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://pnpm.io/supply-chain-security>
> Snapshot: <https://web.archive.org/web/20261022233946/https://pnpm.io/supply-chain-security>

# Mitigating supply chain attacks

Sometimes npm packages are compromised and published with malware. Luckily, there are companies like [Socket](https://socket.dev/), [Snyk](https://snyk.io), [Xygeni](https://xygeni.io/) and [Aikido](https://www.aikido.dev/) that detect these compromised packages early. The npm registry usually removes the affected versions within hours. However, there is always a window of time between when the malware is published and when it is detected, during which you could be exposed. Fortunately, there are some things you can do with pnpm to minimize the risks.

## Block risky postinstall scripts[#](https://pnpm.io/supply-chain-security#block-risky-postinstall-scripts)

Historically, most compromised packages have used postinstall scripts to run code immediately upon installation. To mitigate this, pnpm does not run the build scripts of dependencies unless they are allowed in [allowBuilds](https://pnpm.io/settings/build#allowbuilds). With [strictDepBuilds](https://pnpm.io/settings/build#strictdepbuilds) (on by default), an install fails if a dependency has a build script that is not allowed. Although there is a setting to re-enable them globally using [dangerouslyAllowAllBuilds](https://pnpm.io/settings/build#dangerouslyallowallbuilds), we recommend explicitly listing only trusted dependencies using [allowBuilds](https://pnpm.io/settings/build#allowbuilds). This way, if a dependency did not require a build in the past, it won't suddenly run a malicious script if a compromised version is published. Still, we recommend being cautious when updating a trusted package that has a postinstall script, as [it might get compromised](https://socket.dev/blog/nx-packages-compromised).

## Prevent exotic transitive dependencies[#](https://pnpm.io/supply-chain-security#prevent-exotic-transitive-dependencies)

You can prevent transitive dependencies from using exotic sources (like git repositories or direct tarball URLs) with [blockExoticSubdeps](https://pnpm.io/settings/dependency-resolution#blockexoticsubdeps), which is on by default. This ensures that all transitive dependencies are resolved from trusted sources, reducing the risk of supply chain attacks.

## Delay dependency updates[#](https://pnpm.io/supply-chain-security#delay-dependency-updates)

Another way to reduce the risk of installing compromised packages is to delay updates to your dependencies. Since malware is usually detected quickly, delaying updates by 24 hours will most likely prevent you from installing a bad version. The [minimumReleaseAge](https://pnpm.io/settings/dependency-resolution#minimumreleaseage) setting defines the minimum number of minutes that must pass after a version is published before pnpm will install it. This defaults to 1440 (1 day), meaning newly published packages will not be resolved until they are at least 1 day old. To opt out, set minimumReleaseAge: 0 in pnpm-workspace.yaml. You can also set it to 10080 to wait one week before installing a new version.

## Enforce trust with trustPolicy[#](https://pnpm.io/supply-chain-security#enforce-trust-with-trustpolicy)

To further protect your supply chain, pnpm also supports a [trustPolicy](https://pnpm.io/settings/dependency-resolution#trustpolicy) setting. When set to no-downgrade, this setting will prevent installation of a package if its trust level has decreased compared to previous releases (for example, if it was previously published by a trusted publisher but now only has provenance or no trust evidence). This helps you avoid installing potentially compromised or less trustworthy versions.

If you need to allow specific packages or versions to bypass the trust policy check, you can use the [trustPolicyExclude](https://pnpm.io/settings/dependency-resolution#trustpolicyexclude) setting. This is useful for known packages that may not meet the trust requirements but are still safe to use.

Additionally, the [trustPolicyIgnoreAfter](https://pnpm.io/settings/dependency-resolution#trustpolicyignoreafter) setting allows you to ignore trust checks for packages published more than a specified time ago. This is helpful for older versions of packages that lack a process for publishing with signatures or provenance.

## Use a lockfile[#](https://pnpm.io/supply-chain-security#use-a-lockfile)

Always lock your dependencies with a lockfile. Commit your lockfile to your repository to avoid unexpected updates.

If you scan pnpm-lock.yaml with a vulnerability scanner or an SBOM generator, check that it handles the [two-document lockfile](https://pnpm.io/lockfile): a tool that reads only the first document reports that the project has no dependencies, and no vulnerabilities, without failing.

## Pin dependencies to the registry they come from[#](https://pnpm.io/supply-chain-security#pin-dependencies-to-the-registry-they-come-from)

If you install from more than one registry, use [namedRegistries](https://pnpm.io/settings/dependency-resolution#namedregistries) aliases for the packages that must come from a specific one. pnpm records those packages in the lockfile under registry-qualified keys (<name>@<registryName>:<version>), so a package cannot be quietly substituted by another registry that publishes the same name and version. See [Named registries in the lockfile](https://pnpm.io/settings/dependency-resolution#named-registries-in-the-lockfile). Packages installed without an alias are resolved through the default [registries](https://pnpm.io/settings/dependency-resolution#registries) configuration as before.
