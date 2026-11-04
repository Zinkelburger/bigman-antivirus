---
url: "https://docs.npmjs.com/cli/v12/commands/npm-install-scripts"
title: "npm-install-scripts | npm Docs"
captured: "2026-11-03T02:58:26Z"
via: "live"
source_url: "https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/"
sha256: "87de2064d76c8916d1509008f5cb84ccb5ab23535877aa094575cb6c035c54c4"
cited_in:
  - "docs/research/hardening.md"
  - "docs/threats/02-supply-chain-packages.md"
---

# npm-install-scripts | npm Docs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.npmjs.com/cli/v12/commands/npm-install-scripts>

[Skip to search](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#search-box-input)[Skip to content](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#skip-to-content)

[npm Docs](https://docs.npmjs.com/)

[npmjs.com](https://npmjs.com/)[Status](https://status.npmjs.com/)[Support](https://npmjs.com/support/)

# npm-install-scripts

Manage install-script approvals for dependencies

Select CLI Version:

Table of contents

## [Synopsis](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#synopsis)

```

npm install-scripts approve <pkg> [<pkg> ...]

npm install-scripts approve --all

npm install-scripts deny <pkg> [<pkg> ...]

npm install-scripts deny --all

npm install-scripts ls

npm install-scripts prune

```

Note: This command is unaware of workspaces.

## [Description](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#description)

Manages the allowScripts field in your project's package.json, which records which of your dependencies are permitted to run install scripts (preinstall, install, postinstall, and prepare for non-registry sources). This is the recommended way to maintain that field.

Dependency install scripts are blocked by default. Install commands silently skip lifecycle scripts for any dependency that does not have a matching entry in allowScripts, and end with a list of the packages whose scripts were skipped so you can review them here.

This command only works inside a project that has a package.json. Running it with --global (-g) fails with an EGLOBAL error, since global installs (npm install -g) and one-off executions (npm exec / npx) have no project package.json to write to. To allow install scripts in those contexts, use the --allow-scripts flag at install time (for example npm install -g --allow-scripts=canvas,sharp) or persist the setting with npm config set allow-scripts=canvas,sharp --location=user.

There are four subcommands:

```

npm install-scripts approve <pkg> [<pkg> ...]

npm install-scripts approve --all

npm install-scripts deny <pkg> [<pkg> ...]

npm install-scripts deny --all

npm install-scripts ls

npm install-scripts prune

```

approve allows install scripts for the named packages. <pkg> matches every installed version of that package. By default it writes pinned entries (pkg@1.2.3), which keep their approval narrowed to the specific version you reviewed. Pass --no-allow-scripts-pin to write name-only entries that allow any future version. --all approves every package with unreviewed install scripts in one go.

deny records an explicit denial for the named packages (a name-only false entry), which survives npm install-scripts approve --all and excludes the package from any future blanket approval. --all denies every package with unreviewed install scripts.

ls is read-only: it lists every package whose install scripts are not yet covered by allowScripts, without modifying package.json.

prune removes allowScripts entries that no longer match an installed package with an install script, either because the package is no longer installed (a transitive dependency changed, or a pinned pkg@1.2.3 was upgraded) or because it no longer has an install script. Both approvals (true) and denials (false) are removed. It edits only the allowScripts field in package.json, never .npmrc or --allow-scripts. Pass --dry-run to preview without writing. Unparseable keys are left alone.

approve honours the asymmetric pin rule: if you re-approve a package whose installed version has changed, the existing pin is rewritten to track the new installed version. Multi-version statements (pkg@1 || 2) are left alone, since they likely capture intent that the command cannot infer. Existing false entries always win; approve will not silently re-allow a package you previously denied.

The standalone commands [npm approve-scripts](https://docs.npmjs.com/cli/v12/commands/npm-approve-scripts) and [npm deny-scripts](https://docs.npmjs.com/cli/v12/commands/npm-deny-scripts) are aliases for npm install-scripts approve and npm install-scripts deny.

## [Examples](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#examples)

```

# Approve all currently-installed install scripts after reviewing them

npm install-scripts approve --all

# Approve specific packages, pinned to their installed version

npm install-scripts approve canvas sharp

# Deny a package so it stays blocked

npm install-scripts deny telemetry-pkg

# Preview which packages still need review

npm install-scripts ls

# Preview stale allowScripts entries, then remove them

npm install-scripts prune --dry-run

npm install-scripts prune

```

## [Configuration](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#configuration)

### [all](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#all)

- Default: false
- Type: Boolean

Show or act on all packages, not just the ones your project directly depends on. For npm outdated and npm ls this lists every outdated or installed package. For npm approve-scripts and npm deny-scripts it selects every package with pending install scripts.

### [allow-scripts-pin](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#allow-scripts-pin)

- Default: true
- Type: Boolean

Write pinned (pkg@version) entries when approving install scripts. Set to false to write name-only entries that allow any version. Has no effect on npm deny-scripts, which always writes name-only entries regardless of this setting.

### [dry-run](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#dry-run)

- Default: false
- Type: Boolean

Indicates that you don't want npm to make any changes and that it should only report what it would have done. This can be passed into any of the commands that modify your local installation, eg, install, update, dedupe, uninstall, as well as pack and publish.

Note: This is NOT honored by other network related commands, eg dist-tags, owner, etc.

### [json](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#json)

- Default: false
- Type: Boolean

Whether or not to output JSON data, rather than the normal output.

- In npm pkg set it enables parsing set values with JSON.parse() before saving them to your package.json.

Not supported by all npm commands.

## [See Also](https://docs.npmjs.com/cli/v12/commands/npm-install-scripts/#see-also)

- [npm approve-scripts](https://docs.npmjs.com/cli/v12/commands/npm-approve-scripts)
- [npm deny-scripts](https://docs.npmjs.com/cli/v12/commands/npm-deny-scripts)
- [npm install](https://docs.npmjs.com/cli/v12/commands/npm-install)
- [npm rebuild](https://docs.npmjs.com/cli/v12/commands/npm-rebuild)
- [package.json](https://docs.npmjs.com/cli/v12/configuring-npm/package-json)

[Edit this page on GitHub](https://github.com/npm/cli/edit/latest/docs/lib/content/commands/npm-install-scripts.md)

2 contributorshttps://github.com/JamieMageehttps://github.com/manzoorwanijk

Last edited by [JamieMagee](https://github.com/JamieMagee) on [June 25, 2026](https://github.com/npm/cli/commit/c4e53561e5c112f635b1eb5bdca8198d8dffcd12)

## Table of contents
