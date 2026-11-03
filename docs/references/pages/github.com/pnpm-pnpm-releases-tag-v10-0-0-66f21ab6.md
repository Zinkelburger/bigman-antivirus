---
url: "https://github.com/pnpm/pnpm/releases/tag/v10.0.0"
title: "Release pnpm 10 · pnpm/pnpm"
site: "GitHub"
captured: "2026-11-03T02:58:40Z"
via: "live"
sha256: "c99c5c7c96141665a63c3eee6bd6e153a724b22acaaeb7a090b8c7fed3fbacef"
wayback: "https://web.archive.org/web/20261023175858/https://github.com/pnpm/pnpm/releases/tag/v10.0.0"
cited_in:
  - "docs/research/hardening.md"
---

# Release pnpm 10 · pnpm/pnpm

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/pnpm/pnpm/releases/tag/v10.0.0>
> Snapshot: <https://web.archive.org/web/20261023175858/https://github.com/pnpm/pnpm/releases/tag/v10.0.0>

## Repository navigation

# pnpm 10

[github-actions](https://github.com/apps/github-actions) released this  07 Jan 14:48

· [5520 commits](https://github.com/pnpm/pnpm/compare/v10.0.0...main) to main since this release

[v10.0.0](https://github.com/pnpm/pnpm/tree/v10.0.0)

This tag was signed with the committer’s verified signature.

https://github.com/zkochan  [zkochan](https://github.com/zkochan) Zoltan Kochan

GPG key ID: 649E4D4AF74E7DEC

Verified

[Learn about vigilant mode](https://docs.github.com/github/authenticating-to-github/displaying-verification-statuses-for-all-of-your-commits).

[42ecf04](https://github.com/pnpm/pnpm/commit/42ecf04fd0e442af8610ae4231855e004732dbf7)

This commit was signed with the committer’s verified signature.

https://github.com/zkochan  [zkochan](https://github.com/zkochan) Zoltan Kochan

GPG key ID: 649E4D4AF74E7DEC

Verified

[Learn about vigilant mode](https://docs.github.com/github/authenticating-to-github/displaying-verification-statuses-for-all-of-your-commits).

## Major Changes

-

Lifecycle scripts of dependencies are not executed during installation by default! This is a breaking change aimed at increasing security. In order to allow lifecycle scripts of specific dependencies, they should be listed in the pnpm.onlyBuiltDependencies field of package.json [#8897](https://github.com/pnpm/pnpm/pull/8897). For example:

```
{
  "pnpm": {
    "onlyBuiltDependencies": ["fsevents"]
  }
}
```

Read [pnpm 10.0.0 Blocks Lifecycle Scripts by Default](https://socket.dev/blog/pnpm-10-0-0-blocks-lifecycle-scripts-by-default) to learn about the motivation of the change.

If you want the old pre v10 behaviour, so you want to allow all dependencies to run postinstall scripts, then add this to your package.json:

```
{
  "pnpm": {
    "neverBuiltDependencies": []
  }
}
```

-

pnpm link behavior updated:

The pnpm link command now adds overrides to the root package.json.

- In a workspace: The override is added to the root of the workspace, linking the dependency to all projects in the workspace.
- Global linking: To link a package globally, run pnpm link from the package’s directory. Previously, you needed to use pnpm link -g.
Related PR: [#8653](https://github.com/pnpm/pnpm/pull/8653)

-

Secure hashing with SHA256:

Various hashing algorithms have been updated to SHA256 for enhanced security and consistency:

- Long paths inside node_modules/.pnpm are now hashed with SHA256.
- Long peer dependency hashes in the lockfile now use SHA256 instead of MD5. (This affects very few users since these are only used for long keys.)
- The hash stored in the packageExtensionsChecksum field of pnpm-lock.yaml is now SHA256.
- The side effects cache keys now use SHA256.
- The pnpmfile checksum in the lockfile now uses SHA256 ([#8530](https://github.com/pnpm/pnpm/pull/8530)).
- The patch file checksums in the lockfile now use SHA256.

-

Configuration updates:

-

manage-package-manager-versions: enabled by default. pnpm now manages its own version based on the packageManager field in package.json by default.

-

public-hoist-pattern: nothing is hoisted by default. Packages containing eslint or prettier in their name are no longer hoisted to the root of node_modules. Related Issue: [#8378](https://github.com/pnpm/pnpm/issues/8378)

-

Upgraded @yarnpkg/extensions to v2.0.3. This may alter your lockfile.

-

virtual-store-dir-max-length: the default value on Windows has been reduced to 60 characters.

-

Reduced environment variables for scripts:
During script execution, fewer npm_package_* environment variables are set. Only name, version, bin, engines, and config remain.
Related Issue: [#8552](https://github.com/pnpm/pnpm/issues/8552)

-

All dependencies are now installed even if NODE_ENV=production. Related Issue: [#8827](https://github.com/pnpm/pnpm/issues/8827)

-

Changes to the global store:

-

Store version bumped to v10.

-

Some registries allow identical content to be published under different package names or versions. To accommodate this, index files in the store are now stored using both the content hash and package identifier.

This approach ensures that we can:

- Validate that the integrity in the lockfile corresponds to the correct package, which might not be the case after a poorly resolved Git conflict.
- Allow the same content to be referenced by different packages or different versions of the same package.
Related PR: [#8510](https://github.com/pnpm/pnpm/pull/8510)
Related Issue: [#8204](https://github.com/pnpm/pnpm/issues/8204)

-

More efficient side effects indexing. The structure of index files in the store has changed. Side effects are now tracked more efficiently by listing only file differences rather than all files.
Related PR: [#8636](https://github.com/pnpm/pnpm/pull/8636)

-

A new index directory stores package content mappings. Previously, these files were in files.

-

Other breaking changes:

- The # character is now escaped in directory names within node_modules/.pnpm.
Related PR: [#8557](https://github.com/pnpm/pnpm/pull/8557)
- Running pnpm add --global pnpm or pnpm add --global @pnpm/exe now fails with an error message, directing you to use pnpm self-update instead.
Related PR: [#8728](https://github.com/pnpm/pnpm/pull/8728)
- Dependencies added via a URL now record the final resolved URL in the lockfile, ensuring that any redirects are fully captured.
Related Issue: [#8833](https://github.com/pnpm/pnpm/issues/8833)
- The pnpm deploy command now only works in workspaces that have inject-workspace-packages=true. This limitation is introduced to allow us to create a proper lockfile for the deployed project using the workspace lockfile.
- Removed conversion from lockfile v6 to v9. If you need v6-to-v9 conversion, use pnpm CLI v9.
- pnpm test now passes all parameters after the test keyword directly to the underlying script. This matches the behavior of pnpm run test. Previously you needed to use the -- prefix.
Related PR: [#8619](https://github.com/pnpm/pnpm/pull/8619)

-

node-gyp updated to version 11.

-

pnpm deploy now tries creating a dedicated lockfile from a shared lockfile for deployment. It will fallback to deployment without a lockfile if there is no shared lockfile or force-legacy-deploy is set to true.

## Minor Changes

-

Added support for a new type of dependencies called ["config dependencies"](https://pnpm.io/config-dependencies). These dependencies are installed before all the other types of dependencies (before "dependencies", "devDependencies", "optionalDependencies").

Config dependencies cannot have dependencies of their own or lifecycle scripts. They should be added using exact version and the integrity checksum. Example:

```
{
  "pnpm": {
    "configDependencies": {
      "my-configs": "1.0.0+sha512-30iZtAPgz+LTIYoeivqYo853f02jBYSd5uGnGpkFV0M3xOt9aN73erkgYAmZU43x4VfqcnLxW9Kpg3R5LC4YYw=="
    }
  }
}
```

Related RFC: [#8](https://github.com/pnpm/rfcs/pull/8).
Related PR: [#8915](https://github.com/pnpm/pnpm/pull/8915).

-

New settings:

-

New verify-deps-before-run setting. This setting controls how pnpm checks node_modules before running scripts:

- install: Automatically run pnpm install if node_modules is outdated.
- warn: Print a warning if node_modules is outdated.
- prompt: Prompt the user to confirm running pnpm install if node_modules is outdated.
- error: Throw an error if node_modules is outdated.
- false: Disable dependency checks.
Related Issue: [#8585](https://github.com/pnpm/pnpm/issues/8585)

-

New inject-workspace-packages setting enables hard-linking all local workspace dependencies instead of symlinking them. Previously, this could be achieved using [dependenciesMeta[].injected](https://pnpm.io/package_json#dependenciesmetainjected), which remains supported.
Related PR: [#8836](https://github.com/pnpm/pnpm/pull/8836)

-

Faster repeat installs:

On repeated installs, pnpm performs a quick check to ensure node_modules is up to date.
Related PR: [#8838](https://github.com/pnpm/pnpm/pull/8838)

-

pnpm add integrates with default workspace catalog:

When adding a dependency, pnpm add checks the default workspace catalog. If the dependency and version requirement match the catalog, pnpm add uses the catalog: protocol. Without a specified version, it matches the catalog’s version. If it doesn’t match, it falls back to standard behavior.
Related Issue: [#8640](https://github.com/pnpm/pnpm/issues/8640)

-

pnpm dlx now resolves packages to their exact versions and uses these exact versions for cache keys. This ensures pnpm dlx always installs the latest requested packages.
Related PR: [#8811](https://github.com/pnpm/pnpm/pull/8811)

-

No node_modules validation on certain commands. Commands that should not modify node_modules (e.g., pnpm install --lockfile-only) no longer validate or purge node_modules.
Related PR: [#8657](https://github.com/pnpm/pnpm/pull/8657)

## Platinum Sponsors

|  https://bit.dev/?utm_source=pnpm&utm_medium=release_notes   |  https://sanity.io/?utm_source=pnpm&utm_medium=release_notes   |  https://figma.com/?utm_source=pnpm&utm_medium=release_notes

## Gold Sponsors

|  https://discord.com/?utm_source=pnpm&utm_medium=release_notes   |  https://prisma.io/?utm_source=pnpm&utm_medium=release_notes

|  https://uscreen.de/?utm_source=pnpm&utm_medium=release_notes   |  https://www.jetbrains.com/?utm_source=pnpm&utm_medium=release_notes

|  https://nx.dev/?utm_source=pnpm&utm_medium=release_notes   |  https://coderabbit.ai/?utm_source=pnpm&utm_medium=release_notes

|  https://route4me.com/?utm_source=pnpm&utm_medium=release_notes   |  https://workleap.com/?utm_source=pnpm&utm_medium=release_notes

|  https://canva.com/?utm_source=pnpm&utm_medium=release_notes

Assets 10

Loading

130 people reacted

6  [Join discussion](https://github.com/pnpm/pnpm/discussions/8945)
