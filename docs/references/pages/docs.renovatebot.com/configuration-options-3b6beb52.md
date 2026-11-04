---
url: "https://docs.renovatebot.com/configuration-options/#minimumreleaseage"
title: "Configuration Options - Renovate Docs"
captured: "2026-11-02T18:29:07Z"
via: "live"
sha256: "4e139ae787a14c69970ea1b1c2c8b6c98b6d6e3e73a9cca48cc2fa28cab842fc"
wayback: "https://web.archive.org/web/20261102135418/https://docs.renovatebot.com/configuration-options/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Configuration Options - Renovate Docs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.renovatebot.com/configuration-options/#minimumreleaseage>
> Snapshot: <https://web.archive.org/web/20261102135418/https://docs.renovatebot.com/configuration-options/>

https://github.com/renovatebot/renovate/edit/main/docs/usage/configuration-options.md

# Configuration Options[¶](https://docs.renovatebot.com/configuration-options/#configuration-options)

This document describes all the configuration options you may use in a Renovate configuration file. Any config you define applies to the whole repository (e.g. if you have a monorepo).

A subtype in the configuration table specifies what type you're allowed to use within the main element.

If a config option has a parent defined, it means it's only allowed to configure it within an object with the parent name, such as packageRules or hostRules.

When an array or object configuration option is mergeable, it means that values inside it will be added to any existing object or array that existed with the same name.

Tip

This documentation corresponds with the JSON schema in [docs.renovatebot.com/renovate-schema.json](https://docs.renovatebot.com/renovate-schema.json).

Note

Config options with type=string are always non-mergeable, so mergeable=false.

## Locations for configuration filenames[¶](https://docs.renovatebot.com/configuration-options/#locations-for-configuration-filenames)

You can store your Renovate configuration file in one of these locations:

- renovate.json
- renovate.jsonc
- renovate.json5
- .github/renovate.json
- .github/renovate.jsonc
- .github/renovate.json5
- .gitlab/renovate.json
- .gitlab/renovate.jsonc
- .gitlab/renovate.json5
- .renovaterc
- .renovaterc.json
- .renovaterc.jsonc
- .renovaterc.json5
- package.json (within a "renovate" section)

Also, be sure to check out Renovate's [shareable config presets](https://docs.renovatebot.com/config-presets/) to save yourself from reinventing any wheels. Shareable config presets work with JSON and JSON5 file formats.

If you have any questions about the config options, or want to get help/feedback about a config, go to the [discussions tab in the Renovate repository](https://github.com/renovatebot/renovate/discussions) and start a [new "Request Help" discussion](https://github.com/renovatebot/renovate/discussions/new?category=request-help). We will do our best to answer your question(s).

Or in a custom file present within the [configFileNames](https://docs.renovatebot.com/self-hosted-configuration/#configfilenames).

When Renovate runs on a repository, it tries to find the configuration files in the order listed above. Renovate first checks all the files in the configFileNames array before checking from the above file list. Renovate stops the search after it finds the first match.

Warning

Storing the Renovate configuration in a package.json file is deprecated and support may be removed in the future.

Note

Renovate supports JSONC for .json files and any config files without file extension (e.g. .renovaterc). We also recommend you prefer using a .jsonc file if you want to add comments to your configuration, instead of a .json5 file. Using an explicit .jsonc file is preferred over using a .json file with comments, as it can cause issues with editors and syntax highlighting.

Renovate always uses the config from the repository's default branch, even if that configuration specifies baseBranchPatterns. Renovate does not read/override the config from within each base branch if present.

## abandonmentThreshold[¶](https://docs.renovatebot.com/configuration-options/#abandonmentthreshold)

Flags packages that have not been updated within this period as abandoned.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | string

| cli  | --abandonment-threshold

| env  | RENOVATE_ABANDONMENT_THRESHOLD

The abandonmentThreshold option allows Renovate to flag packages as abandoned when they haven't received updates for a specified period of time.

Renovate adds an isAbandoned boolean property to the package lookup result when:

- abandonmentThreshold is defined (not null)
- The package has a mostRecentTimestamp timestamp available from the datasource

The mostRecentTimestamp timestamp represents the release date of the highest version, but only if that version also has the most recent timestamp among all releases. This ensures abandonment detection is based on normal package release patterns.

If a package's most recent release date plus the abandonmentThreshold duration is in the past, the package is marked as abandoned (isAbandoned: true).

This option accepts time duration strings like 1 year, 6 months, 90 days, etc.

Example usage:

```
{
  "abandonmentThreshold": "2 years"
}

```

You can also apply this setting selectively using packageRules:

```
{
  "packageRules": [
    {
      "matchDatasources": ["npm"],
      "abandonmentThreshold": "1 year"
    }
  ]
}

```

## addLabels[¶](https://docs.renovatebot.com/configuration-options/#addlabels)

Labels to add to Pull Request.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| mergeable  | true

| supportsTemplating  | true

| cli  | --add-labels

| env  | RENOVATE_ADD_LABELS

| default  | []

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

The labels field is non-mergeable, meaning that any config setting a list of PR labels will replace any existing list. If you want to append labels for matched rules, then define an addLabels array with one (or more) label strings. All matched addLabels strings will be attached to the PR.

Consider this example:

```
{
  "labels": ["dependencies"],
  "packageRules": [
    {
      "matchPackageNames": ["/eslint/"],
      "labels": ["linting"]
    },
    {
      "matchDepTypes": ["optionalDependencies"],
      "addLabels": ["optional"]
    }
  ]
}

```

With the above config:

- Optional dependencies will have the labels dependencies and optional
- ESLint dependencies will have the label linting
- All other dependencies will have the label dependencies

If you want to use dynamic labels, you can use [templates](https://docs.renovatebot.com/templates/) such as this example using depName for addLabels:

```
{
  "addLabels": ["{{depName}}"]
}

```

Note

Keep your labels within the maximum character limit for your Git hosting platform. Renovate usually truncates labels to 50 characters, except for GitLab, which has a 255 character limit.

## additionalBranchPrefix[¶](https://docs.renovatebot.com/configuration-options/#additionalbranchprefix)

Additional string value to be appended to branchPrefix.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | ""

| supportsTemplating  | true

| env  | RENOVATE_ADDITIONAL_BRANCH_PREFIX

| previouslyKnownAs  |

```
["managerBranchPrefix"]

```

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

By default, the value for this config option is an empty string. Normally you don't need to set this config option.

Here's an example where additionalBranchPrefix can help you. Say you're using a monorepo and want to split pull requests based on the location of the package definition, so that individual teams can manage their own Renovate pull requests. This can be done with this configuration:

```
{
  "additionalBranchPrefix": "{{parentDir}}-"
}

```

## additionalReviewers[¶](https://docs.renovatebot.com/configuration-options/#additionalreviewers)

Additional reviewers for Pull Requests (in contrast to reviewers, this option adds to the existing reviewer list, rather than replacing it).

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| mergeable  | true

| cli  | --additional-reviewers

| env  | RENOVATE_ADDITIONAL_REVIEWERS

| default  | []

This option adds to the existing reviewer list, rather than replacing it like reviewers.

Use additionalReviewers when you want to add to a preset or base list, without replacing the original. For example, when adding focused reviewers for a specific package group.

Please note that Reviewers are only added during creation of a PR, but are not modified afterwards.

## assignAutomerge[¶](https://docs.renovatebot.com/configuration-options/#assignautomerge)

Assign reviewers and assignees even if the PR is to be automerged.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --assign-automerge

| env  | RENOVATE_ASSIGN_AUTOMERGE

By default, Renovate will not assign reviewers and assignees to an automerge-enabled PR unless it fails status checks. By configuring this setting to true, Renovate will instead always assign reviewers and assignees for automerging PRs at time of creation.

## assignees[¶](https://docs.renovatebot.com/configuration-options/#assignees)

Assignees for Pull Request (either username or email address depending on the platform).

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| cli  | --assignees

| env  | RENOVATE_ASSIGNEES

| default  | []

Must be valid usernames on the platform in use. Teams are not supported, and Renovate passes team: entries to the platform unchanged. To request a review from a team, use [reviewers](https://docs.renovatebot.com/configuration-options/#reviewers) instead.

## assigneesFromCodeOwners[¶](https://docs.renovatebot.com/configuration-options/#assigneesfromcodeowners)

Determine assignees based on configured code owners and changes in PR.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --assignees-from-code-owners

| env  | RENOVATE_ASSIGNEES_FROM_CODE_OWNERS

If enabled Renovate tries to determine PR assignees by matching rules defined in a CODEOWNERS file against the changes in the PR.

Read the docs for your platform for details on syntax and allowed file locations:

- [GitHub Docs, About code owners](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)
- [GitLab, Code Owners](https://docs.gitlab.com/ee/user/project/codeowners/)
- [Bitbucket, Set up and use code owners](https://support.atlassian.com/bitbucket-cloud/docs/set-up-and-use-code-owners/)

## assigneesSampleSize[¶](https://docs.renovatebot.com/configuration-options/#assigneessamplesize)

Take a random sample of given size from assignees.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| cli  | --assignees-sample-size

| env  | RENOVATE_ASSIGNEES_SAMPLE_SIZE

If configured, Renovate will take a random sample of given size from assignees and assign them only, instead of assigning the entire list of assignees you have configured.

## autoApprove[¶](https://docs.renovatebot.com/configuration-options/#autoapprove)

Set to true to automatically approve PRs.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| supportedPlatforms  | azure, gerrit, gitlab

| cli  | --auto-approve

| env  | RENOVATE_AUTO_APPROVE

| previouslyKnownAs  |

```
["azureAutoApprove"]

```

Setting this to true will automatically approve the PRs.

You can also configure this using packageRules if you want to use it selectively (e.g. per-package).

## autoReplaceGlobalMatch[¶](https://docs.renovatebot.com/configuration-options/#autoreplaceglobalmatch)

Control whether replacement regular expressions are global matches or only the first match.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | true

| cli  | --auto-replace-global-match

| env  | RENOVATE_AUTO_REPLACE_GLOBAL_MATCH

Setting this to false will replace only the first match during replacements updates.

Disabling this is useful for situations where values are repeated within the dependency string, such as when the currentVersion is also featured somewhere within the currentDigest, but you only want to replace the first instance.

Consider this example:

```
FROM java:8@sha256:0e8b2a860

```

```
{
  "packageRules": [
    {
      "matchPackageNames": ["java"],
      "replacementName": "eclipse-temurin",
      "replacementVersion": "11"
    }
  ]
}

```

With the above replacement scenario, the current dependency has a version of 8, which also features several times within the digest section.

When using the default autoReplaceGlobalMatch configuration, Renovate will try to replace all instances of 8 within the dependency string with the replacementVersion value of 11. This will replace more than is intended and will be caught during replacement validation steps, resulting in the replacement PR to not be created.

When setting autoReplaceGlobalMatch configuration to false, Renovate will only replace the first occurrence of 8 and will successfully create a replacement PR.

## automerge[¶](https://docs.renovatebot.com/configuration-options/#automerge)

Whether to automerge branches/PRs automatically, without human intervention.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --automerge

| env  | RENOVATE_AUTOMERGE

By default, Renovate raises PRs but leaves them to someone or something else to merge them. By configuring this setting, you allow Renovate to automerge PRs or even branches. Using automerge reduces the amount of human intervention required.

Usually you won't want to automerge all PRs, for example most people would want to leave major dependency updates to a human to review first. You could configure Renovate to automerge all but major this way:

```
{
  "packageRules": [
    {
      "matchUpdateTypes": ["minor", "patch", "pin", "digest"],
      "automerge": true
    }
  ]
}

```

Also note that this option can be combined with other nested settings, such as dependency type. So for example you could choose to automerge all (passing) devDependencies only this way:

```
{
  "packageRules": [
    {
      "matchDepTypes": ["devDependencies"],
      "automerge": true
    }
  ]
}

```

Note

Branches creation follows [schedule](https://docs.renovatebot.com/configuration-options/#schedule) and the automerge follows [automergeSchedule](https://docs.renovatebot.com/configuration-options/#automergeschedule).

Note

On Azure there can be a delay between a PR being set as completed by Renovate, and Azure merging the PR / finishing its tasks. Renovate tries to delay until Azure is in the expected state, but it will continue if it takes too long. In some cases this can result in a dependency not being merged, and a fresh PR being created for the dependency.

Note

By default, Renovate will not assign reviewers and assignees to an automerge-enabled PR unless it fails status checks. By configuring [assignAutomerge](https://docs.renovatebot.com/configuration-options/#assignautomerge) setting to true, Renovate will instead always assign reviewers and assignees for automerging PRs at time of creation.

Automerge and GitHub branch protection rules

You must select at least one status check in the Require status checks to pass before merging section of your branch protection rules on GitHub, if you match all three conditions:

- automerge=true
- platformAutomerge=true, Renovate defaults to true
- You use GitHub's Require status checks to pass before merging branch protection rule

If you don't select any status check, and you use platform automerge, then GitHub might automerge PRs with failing tests!

## automergeComment[¶](https://docs.renovatebot.com/configuration-options/#automergecomment)

PR comment to add to trigger automerge. Only used if automergeType=pr-comment.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "automergeComment"

| cli  | --automerge-comment

| env  | RENOVATE_AUTOMERGE_COMMENT

Use this only if you configure automergeType="pr-comment".

Example use:

```
{
  "automerge": true,
  "automergeType": "pr-comment",
  "automergeComment": "bors: r+"
}

```

## automergeSchedule[¶](https://docs.renovatebot.com/configuration-options/#automergeschedule)

Limit automerge to these times of day or week.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| cli  | --automerge-schedule

| default  |

```
["at any time"]

```

Use the automergeSchedule option to define times of week or month during which Renovate may automerge its PRs. The default value for automergeSchedule is "at any time", which functions the same as setting a null schedule. To configure this option refer to [schedule](https://docs.renovatebot.com/configuration-options/#schedule) as the syntax is the same.

Warning

When platformAutomerge is enabled, Renovate enqueues the platform PR automerge at time of creation, so the schedule specified in automergeSchedule cannot be followed. If it's essential that automerging only happens within the specific automergeSchedule time window, then you need to set platformAutomerge to false and instead rely on Renovate's automerge instead of the platform one.

## automergeStrategy[¶](https://docs.renovatebot.com/configuration-options/#automergestrategy)

The merge strategy to use when automerging PRs. Used only if automergeType=pr.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| allowedValues  |

```
["auto", "fast-forward", "merge-commit", "rebase", "rebase-merge", "squash"]

```

| default  | "auto"

| supportedPlatforms  | azure, bitbucket, forgejo, gitea, github

| cli  | --automerge-strategy

| env  | RENOVATE_AUTOMERGE_STRATEGY

The automerge strategy defaults to auto, so Renovate decides how to merge pull requests as best it can. If possible, Renovate follows the merge strategy set on the platform itself for the repository.

If you've set automerge=true and automergeType=pr for any of your dependencies, then you may choose what automerge strategy Renovate uses by setting the automergeStrategy config option. If you're happy with the default behavior, you don't need to do anything.

On supported platforms, automergeStrategy also applies when using platform-native automerge.

You may choose from these values:

- auto, Renovate decides how to merge
- fast-forward, "fast-forwarding" the main branch reference, no new commits in the resultant tree
- merge-commit, create a new merge commit
- rebase, rewrite history as part of the merge, but usually keep the individual commits
- rebase-merge, create a new merge commit, but rebase the commits prior merging (azure-only)
- squash, flatten the commits that are being merged into a single new commit

Platforms may only support some of these merge strategies.

If the chosen automerge strategy is not supported on your platform then Renovate stops automerging. In that case you'll have to set a supported automerge strategy.

## automergeType[¶](https://docs.renovatebot.com/configuration-options/#automergetype)

How to automerge, if enabled.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| allowedValues  |

```
["branch", "pr", "pr-comment"]

```

| default  | "pr"

| cli  | --automerge-type

| env  | RENOVATE_AUTOMERGE_TYPE

This setting is only applicable if you opt in to configure automerge to true for any of your dependencies.

Automerging defaults to using Pull Requests (automergeType="pr"). In that case Renovate first creates a branch and associated Pull Request, and then automerges the PR on a subsequent run once it detects the PR's status checks are "green". If by the next run the PR is already behind the base branch it will be automatically rebased, because Renovate only automerges branches which are up-to-date and green. If Renovate is scheduled for hourly runs on the repository but commits are made every 15 minutes to the main branch, then an automerge like this will keep getting deferred with every rebase.

Tip

If you have no tests but still want Renovate to automerge, you need to add "ignoreTests": true to your configuration.

If you prefer that Renovate more silently automerge without Pull Requests at all, you can configure "automergeType": "branch". In this case Renovate will:

- Create the branch, wait for test results
- Rebase it any time it gets out of date with the base branch
- Automerge the branch commit if it's: (a) up-to-date with the base branch, and (b) passing all tests
- As a backup, raise a PR only if either: (a) tests fail, or (b) tests remain pending for too long (default: 25 hours, see [prNotPendingHours](https://docs.renovatebot.com/configuration-options/#prnotpendinghours))

Note that "automergeType": "branch" only works with GitHub merge queues or GitLab merge trains if Renovate is on the merge queue's bypass list, because the merge queue does not allow pushing directly to the base branch otherwise. If the push is rejected, Renovate falls back to creating a PR.

The final value for automergeType is "pr-comment", intended only for users who already have a "merge bot" such as [bors-ng](https://github.com/bors-ng/bors-ng) and want Renovate to not actually automerge by itself and instead tell bors-ng to merge for it, by using a comment in the PR. If you're not already using bors-ng or similar, don't worry about this option.

## azureWorkItemId[¶](https://docs.renovatebot.com/configuration-options/#azureworkitemid)

The id of an existing work item on Azure Boards to link to each PR.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| default  | 0

| supportedPlatforms  | azure

| cli  | --azure-work-item-id

| env  | RENOVATE_AZURE_WORK_ITEM_ID

When creating a PR in Azure DevOps, some branches can be protected with branch policies to [check for linked work items](https://learn.microsoft.com/azure/devops/repos/git/branch-policies#check-for-linked-work-items). Creating a work item in Azure DevOps is beyond the scope of Renovate, but Renovate can link an already existing work item when creating PRs.

## azureWorkItemType[¶](https://docs.renovatebot.com/configuration-options/#azureworkitemtype)

The work item type Renovate uses for its issues (e.g. the Dependency Dashboard) on Azure DevOps.

| Name  | Value

| scopes  | repo

| type  | string

| default  | "Issue"

| supportedPlatforms  | azure

| cli  | --azure-work-item-type

| env  | RENOVATE_AZURE_WORK_ITEM_TYPE

The work item type Renovate uses when creating its issues (such as the Dependency Dashboard) on Azure DevOps.

Azure DevOps has no dedicated "issue" concept, so Renovate creates a work item instead. It defaults to the Issue type, which only exists in the Basic process. Projects using another process (for example Agile or Scrum), or a custom inherited process, may not have an Issue type, in which case creation fails. Set this to a work item type that exists in your project, such as Task.

Note

Renovate finds its existing issues by title, not by work item type. If you change azureWorkItemType while an issue (such as the Dependency Dashboard) already exists, Renovate keeps using the existing work item and does not recreate it with the new type. To switch an existing issue to the new type, either rename the old work item (so its title no longer matches) and close it, or close and delete the old work item. Renovate then creates a fresh work item of the configured type on the next run.

## baseBranchPatterns[¶](https://docs.renovatebot.com/configuration-options/#basebranchpatterns)

List of one or more custom base branches defined as exact strings and/or via regex expressions.

| Name  | Value

| scopes  | repo

| type  | array

| subType  | string

| cli  | --base-branch-patterns

| env  | RENOVATE_BASE_BRANCH_PATTERNS

| previouslyKnownAs  |

```
["baseBranches"]

```

| default  | []

This configuration option was formerly known as baseBranches.

By default, Renovate will detect and process only the repository's default branch. For most projects, this is the expected approach. Renovate also allows users to explicitly configure baseBranchPatterns, e.g. for use cases such as:

- You wish Renovate to process only a non-default branch, e.g. dev:

```
{
  "baseBranchPatterns": ["dev"]
}

```

- You have multiple release streams you need Renovate to keep up to date, e.g. in branches main and next:

```
{
  "baseBranchPatterns": ["main", "next"]
}

```

- You want to update your main branch and consistently named release branches, e.g. main and release/<version>:

```
{
  "baseBranchPatterns": ["main", "/^release\\/.*/"]
}

```

It's possible to add this setting into the renovate.json file as part of the "Configure Renovate" onboarding PR. If so then Renovate will reflect this setting in its description and use package file contents from the custom base branch(es) instead of default.

Also, check [useBaseBranchConfig](https://docs.renovatebot.com/configuration-options/#usebasebranchconfig) if you want to configure different configuration for each base branch.

The simplest approach is exact matches, e.g. ["main", "dev"]. baseBranchPatterns also supports Regular Expressions that must begin and end with /, e.g.:

```
{
  "baseBranchPatterns": ["main", "/^release\\/.*/"]
}

```

You can negate the regex by prefixing it with !. Only use a single negation and do not mix it with other branch names, since all branches are combined with or. With a negation, all branches except those matching the regex will be added to the result:

```
{
  "baseBranchPatterns": ["!/^pre-release\\/.*/"]
}

```

You can also use the special "$default" string to denote the repository's default branch, which is useful if you have it in an org preset, e.g.:

```
{
  "baseBranchPatterns": ["$default", "/^release\\/.*/"]
}

```

Note

Do not use the baseBranchPatterns config option when you've set a forkToken. You may need a forkToken when you're using the Forking Renovate app.

## bbAutoResolvePrTasks[¶](https://docs.renovatebot.com/configuration-options/#bbautoresolveprtasks)

The PR tasks will be automatically completed after the PR is raised.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| supportedPlatforms  | bitbucket

| cli  | --bb-auto-resolve-pr-tasks

| env  | RENOVATE_BB_AUTO_RESOLVE_PR_TASKS

Configuring this to true means that Renovate will mark all PR Tasks as complete.

## bbUseDefaultReviewers[¶](https://docs.renovatebot.com/configuration-options/#bbusedefaultreviewers)

Use the default reviewers (Bitbucket only).

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | true

| supportedPlatforms  | bitbucket, bitbucket-server

| cli  | --bb-use-default-reviewers

| env  | RENOVATE_BB_USE_DEFAULT_REVIEWERS

Configuring this to true means that Renovate will detect and apply the default reviewers rules to PRs (Bitbucket only).

## branchConcurrentLimit[¶](https://docs.renovatebot.com/configuration-options/#branchconcurrentlimit)

Limit to a maximum of x concurrent branches. 0 means no limit, null (default) inherits value from prConcurrentLimit.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| cli  | --branch-concurrent-limit

| env  | RENOVATE_BRANCH_CONCURRENT_LIMIT

By default, Renovate doesn’t enforce its own concurrent branch limit. However, it inherits the [prConcurrentLimit](https://docs.renovatebot.com/configuration-options/#prconcurrentlimit), which defaults to 10. This effectively caps concurrent branches at 10, though in many repositories a lower limit, such as 3 or 5, tends to be more efficient.

If you want the same limit for both concurrent branches and concurrent PRs, then set a value for prConcurrentLimit and it will be reused for branch calculations too. But if you want to allow more concurrent branches than concurrent PRs, you can configure both values (e.g. branchConcurrentLimit=5 and prConcurrentLimit=3).

This limit is enforced on a per-repository basis.

Example config:

```
{
  "branchConcurrentLimit": 3
}

```

Warning

Leaving PRs/branches as unlimited or as a high number increases the time it takes for Renovate to process a repository. If you find that Renovate is too slow when rebasing out-of-date branches, decrease the branchConcurrentLimit.

If you have too many concurrent branches which rebase themselves each run, Renovate can take a lot of time to rebase. Solutions:

- Decrease the concurrent branch limit (note: this won't go and delete any existing, so won't have an effect until you either merge or close existing ones manually)
- Remove automerge and/or automatic rebasing (set rebaseWhen to conflicted). However if you have branch protection saying PRs must be up to date then it's not ideal to remove automatic rebasing

## branchName[¶](https://docs.renovatebot.com/configuration-options/#branchname)

Branch name template.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "{{{branchPrefix}}}{{{additionalBranchPrefix}}}{{{branchTopic}}}"

| supportsTemplating  | true

| env  | RENOVATE_BRANCH_NAME

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

This feature has been deprecated

We strongly recommended that you avoid configuring this field directly. Please edit branchPrefix, additionalBranchPrefix, or branchTopic instead.

If you truly need to configure this then it probably means either:

- You are hopefully mistaken, and there's a better approach you should use, so open a new "config help" discussion at the [Renovate discussions tab](https://github.com/renovatebot/renovate/discussions) or
- You have a use case we didn't expect, please open a discussion to see if we want to get a feature request from you

## branchNameStrict[¶](https://docs.renovatebot.com/configuration-options/#branchnamestrict)

Whether to be strict about the use of special characters within the branch name.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --branch-name-strict

| env  | RENOVATE_BRANCH_NAME_STRICT

If true, Renovate removes special characters when slugifying the branch name:

- all special characters are removed
- only alphabetic characters are allowed
- hyphens - are used to separate sections

The default false behavior will mean that special characters like . and / may end up in the branch name.

Note

Renovate will not apply any search/replace to the branchPrefix part of the branch name. If you don't want any / in your branch name then you will also need to change branchPrefix from the default renovate/ value to something like renovate-.

## branchPrefix[¶](https://docs.renovatebot.com/configuration-options/#branchprefix)

Prefix to use for all branch names.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "renovate/"

| supportsTemplating  | true

| cli  | --branch-prefix

| env  | RENOVATE_BRANCH_PREFIX

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

You can modify this field if you want to change the prefix used. For example if you want branches to be like deps/eslint-4.x instead of renovate/eslint-4.x then you configure branchPrefix = deps/. Or if you wish to avoid forward slashes in branch names then you could use renovate_ instead, for example.

branchPrefix must be configured at the root of the configuration (e.g. not within any package rule) and is not allowed to use template values. e.g. instead of renovate/{{parentDir}}-, configure the template part in additionalBranchPrefix, like "additionalBranchPrefix": "{{parentDir}}-".

Note

This setting does not change the default onboarding branch name, i.e. renovate/configure. If you wish to change that too, you need to also configure the field onboardingBranch in your global self-hosted config.

## branchPrefixOld[¶](https://docs.renovatebot.com/configuration-options/#branchprefixold)

Old branchPrefix value to check for existing PRs.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "renovate/"

| cli  | --branch-prefix-old

| env  | RENOVATE_BRANCH_PREFIX_OLD

Renovate uses branch names as part of its checks to see if an update PR was created previously, and already merged or ignored. If you change branchPrefix, then no previously closed PRs will match, which could lead to Renovate recreating PRs in such cases. Instead, set the old branchPrefix value as branchPrefixOld to allow Renovate to look for those branches too, and avoid this happening.

## branchTopic[¶](https://docs.renovatebot.com/configuration-options/#branchtopic)

Branch topic.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "{{{depNameSanitized}}}-{{{newMajor}}}{{#if separateMinorPatch}}{{#if isPatch}}.{{{newMinor}}}{{/if}}{{/if}}{{#if separateMultipleMinor}}{{#if isMinor}}.{{{newMinor}}}{{/if}}{{/if}}.x{{#if isLockfileUpdate}}-lockfile{{/if}}"

| supportsTemplating  | true

| env  | RENOVATE_BRANCH_TOPIC

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

This field is combined with branchPrefix and additionalBranchPrefix to form the full branchName. branchName uniqueness is important for dependency update grouping or non-grouping so be cautious about ever editing this field manually. This is an advanced field, and it's recommend you seek a config review before applying it.

## bumpVersion[¶](https://docs.renovatebot.com/configuration-options/#bumpversion)

Bump the version in the package file being updated.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| allowedValues  |

```
["major", "minor", "patch", "prerelease"]

```

| supportedManagers  | helmv3, npm, nuget, maven, sbt

| cli  | --bump-version

| env  | RENOVATE_BUMP_VERSION

| default  | null

Currently, this config option only works with these managers:

- cargo
- helmv3
- npm
- nuget
- maven
- ocb
- pep621
- poetry
- sbt

Raise a feature request if you want to use this config option with other package managers.

Use bumpVersion if you want Renovate to update the version field in your package file when it updates the dependencies in that file. This can be handy when you have automated your package's release, as you you don't need extra steps after the Renovate upgrade, you can just release a new version.

Configure this value to "prerelease", "patch", "minor" or "major" to have Renovate update the version in your edited package file. e.g. if you wish Renovate to always increase the target package.json version with a patch update, configure this to "patch".

For npm only you can also configure this field to "mirror:x" where x is the name of a package in the package.json. Doing so means that the package.json version field will mirror whatever the version is that x depended on. Make sure that version is a pinned version of course, as otherwise it won't be valid.

For sbt note that Renovate will update the version string only for packages that have the version string in their project's built.sbt file.

## bumpVersions[¶](https://docs.renovatebot.com/configuration-options/#bumpversions)

A list of bumpVersion config options to bump generic version numbers.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | object

| default  | []

This feature is flagged as experimental

Experimental features might be changed or even removed at any time.

The bumpVersions option allows Renovate to update semantic version strings in your code when dependencies are updated. This is useful for files or version fields that Renovate does not natively support or for custom versioning needs.

The option is an array of rules, each specifying how and where to bump versions. Each rule includes the following fields:

- filePatterns: A list of regex patterns to match file names. These patterns follow Renovate's [string pattern matching syntax](https://docs.renovatebot.com/string-pattern-matching/). Templates can also be used for dynamic patterns.
- matchStrings: An array of regex patterns to locate version strings within the matched files. Any of the regexes can match the content. Each pattern must include a named capture group version to extract the version string.
- bumpType: Specifies the type of version bump which defaults to patch. This field supports templates for conditional logic. Supported values are documented in the [bumpVersions.bumpType section](https://docs.renovatebot.com/configuration-options/#bumpversionsbumptype).
- name (optional): A descriptive name for the rule, which is used in logs for easier identification.

Tip

You can use templates in filePatternTemplates, bumpType, and matchStrings. This way you can leverage the power of Renovate's templating engine to change based on the context of the upgrade.

Here is an example of a bumpVersions configuration:

```
{
  "bumpVersions": [
    {
      "name": "Updating release version file",
      "filePatterns": [".release-version"],
      "bumpType": "minor",
      "matchStrings": ["^(?<version>.+)$"]
    }
  ]
}

```

In this example:

- Renovate scans files named .release-version.
- It matches the entire file content as the version string.
- It bumps the version to the next minor release.

Conditional Bumping with packageRules:

You can use packageRules to apply bumpVersions conditionally. For example, to bump versions only for updates in the charts/ directory:

```
{
  "packageRules": [
    {
      "matchFileNames": ["charts/**"],
      "bumpVersions": [
        {
          "filePatterns": ["{{packageFileDir}}/Chart.{yaml,yml}"],
          "matchStrings": ["version:\\s(?<version>[^\\s]+)"],
          "bumpType": "{{#if isPatch}}patch{{else}}minor{{/if}}"
        }
      ]
    }
  ]
}

```

In this configuration:

- If Renovate updates dependencies in the charts/ directory check the Chart.yaml file next to the updated file.
- The version string is extracted from lines matching version: <value>.
- The bumpType is dynamically set to patch for if the dependency update is a patch update and minor otherwise.

debugging bumpVersions

Use [logLevelRemap](https://docs.renovatebot.com/configuration-options/#loglevelremap) to remap, trace log level messages to a higher level e.g. debug. All messages are prefixed with bumpVersions or bumpVersions(<name>) to help you filter them in the logs.

```
{
  "logLevelRemap": [
    {
      "matchMessage": "/bumpVersions/",
      "newLogLevel": "debug"
    }
  ]
}

```

short versions

It's possible to bump short versions:

- 1 → 2 (major)
- 1.1 → 2.0 (major)
- 1.2 → 1.3 (minor)

### bumpVersions.bumpType[¶](https://docs.renovatebot.com/configuration-options/#bumpversionsbumptype)

The semver level to use when bumping versions. This is used by the bumpVersions feature.

| Name  | Value

| type  | string

| parents  | bumpVersions

| allowedValues  |

```
["major", "minor", "patch", "prerelease", "sync"]

```

| supportsTemplating  | true

| cli  | --bump-type

| env  | RENOVATE_BUMP_TYPE

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

The bumpType field specifies the type of version bump to apply. Supported values are:

- prerelease
- patch
- minor
- major
- sync

This field supports templates for conditional logic. For example:

```
{
  "bumpType": "{{#if isPatch}}patch{{else}}minor{{/if}}"
}

```

In this example, the bump type is set to patch for patch updates and minor for all other cases.

Using sync to sync with dependency updates

The sync type is special: it uses the same version as the dependency manager update that triggered the branch. This is useful when you want to keep version files in sync with actual dependency updates.

For example, if you have a .release-version file that should always match the version of a specific dependency:

```
{
  "bumpVersions": [
    {
      "filePatterns": [".release-version"],
      "bumpType": "sync",
      "matchStrings": ["^(?<version>.+)$"]
    }
  ]
}

```

When Renovate updates a dependency to version 2.5.3, it will also update the .release-version file to 2.5.3.

Note

When using bumpType: "sync", Renovate uses the newVersion from the first upgrade in the branch. If no upgrades are found in the branch, the version bump will be skipped and a debug message will be logged.

### bumpVersions.filePatterns[¶](https://docs.renovatebot.com/configuration-options/#bumpversionsfilepatterns)

A list of patterns to match files that contain the version string.

| Name  | Value

| type  | array

| subType  | string

| parents  | bumpVersions

| supportsTemplating  | true

| cli  | --file-patterns

| env  | RENOVATE_FILE_PATTERNS

| default  | []

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

The filePatterns field defines which files should be scanned for version strings. It accepts one or more patterns following Renovate's [string pattern matching syntax](https://docs.renovatebot.com/string-pattern-matching/). Templates can also be used for dynamic patterns. See [Templates](https://docs.renovatebot.com/templates/) for more information.

For example:

```
{
  "filePatterns": ["**/version.txt", "{{packageFileDir}}/Chart.yaml"]
}

```

This configuration matches files named version.txt in any directory and Chart.yaml files in specific package directories.

### bumpVersions.matchStrings[¶](https://docs.renovatebot.com/configuration-options/#bumpversionsmatchstrings)

Queries to use. Valid only within bumpVersions or customManagers object. Templating is supported within bumpVersions only.

| Name  | Value

| type  | array

| subType  | string

| parents  | customManagersbumpVersions

| supportsTemplating  | true

| default  | []

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Use matchStrings within bumpVersions to define patterns to match version strings in files. See [customManagers.matchStrings](https://docs.renovatebot.com/configuration-options/#custommanagersmatchstrings) for full documentation.

### bumpVersions.name[¶](https://docs.renovatebot.com/configuration-options/#bumpversionsname)

A name for the bumpVersion config. This is used for logging and debugging.

| Name  | Value

| type  | string

| parents  | bumpVersions

| cli  | --name

| env  | RENOVATE_NAME

| default  | null

The name field is an optional identifier for the bump version rule. It is used in logs to help identify which rule is being applied.

For example:

```
{
  "name": "Update release version"
}

```

This name will appear in Renovate logs, making it easier to debug or trace specific rules.

## cloneSubmodules[¶](https://docs.renovatebot.com/configuration-options/#clonesubmodules)

Set to true to initialize submodules during repository clone.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | false

| cli  | --clone-submodules

| env  | RENOVATE_CLONE_SUBMODULES

Enabling this option will mean that detected Git submodules will be cloned at time of repository clone. By default all will be cloned, but this can be customized by configuring cloneSubmodulesFilter too. Submodules are always cloned recursively.

Important: private submodules aren't supported by Renovate, unless the underlying ssh layer already has the correct permissions.

## cloneSubmodulesFilter[¶](https://docs.renovatebot.com/configuration-options/#clonesubmodulesfilter)

List of submodules names or patterns to clone when cloneSubmodules=true.

| Name  | Value

| scopes  | repo

| type  | array

| subType  | string

| default  |

```
["*"]

```

| cli  | --clone-submodules-filter

| env  | RENOVATE_CLONE_SUBMODULES_FILTER

Use this option together with cloneSubmodules if you wish to clone only a subset of submodules.

This config option supports regex and glob filters, including negative matches. For more details on this syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/).

## commitBody[¶](https://docs.renovatebot.com/configuration-options/#commitbody)

Commit message body template. Will be appended to commit message, separated by two line returns.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| supportsTemplating  | true

| env  | RENOVATE_COMMIT_BODY

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Configure this if you wish Renovate to add a commit body, otherwise Renovate uses a regular single-line commit.

For example, To add [skip ci] to every commit you could configure:

```
{
  "commitBody": "[skip ci]"
}

```

Onboarding and config migration commits expose only {{{gitAuthor}}} from the repository config to this template.

To add [git trailers](https://git-scm.com/docs/git-interpret-trailers) like Signed-off-by to commits, use [commitTrailers](https://docs.renovatebot.com/configuration-options/#committrailers) instead.

## commitBodyTable[¶](https://docs.renovatebot.com/configuration-options/#commitbodytable)

If enabled, append a table in the commit message body describing all updates in the commit.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --commit-body-table

| env  | RENOVATE_COMMIT_BODY_TABLE

## commitHourlyLimit[¶](https://docs.renovatebot.com/configuration-options/#commithourlylimit)

Rate limit commits to maximum x per hour. 0 means no limit.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| default  | 0

| cli  | --commit-hourly-limit

| env  | RENOVATE_COMMIT_HOURLY_LIMIT

This config option limits the number of commits Renovate pushes in an hour. It includes commits pushed while creating a new branch or rebasing an existing one.

Use commitHourlyLimit to strictly control the rate at which Renovate triggers CI runs. Both branch creation and rebasing trigger CI runs, so limiting these operations helps you control your CI load.

For example, with commitHourlyLimit: 2, in a given hour Renovate might:

- Create 2 new branches (triggering 2 CI runs), then stop until the next hour, or
- Create 1 new branch and rebase 1 existing branch (2 CI runs total), then stop

This limit is enforced on a per-repository basis and per hourly period (:00 through :59).

This setting differs from prHourlyLimit in an important way:

- prHourlyLimit only limits PR creation. Renovate can still rebase existing branches, which triggers additional CI runs
- commitHourlyLimit limits both branch creation and automatic rebasing, giving you stricter control over CI usage

If you want strict control over CI load, use commitHourlyLimit. If you only want to limit the rate of new PRs, use [prHourlyLimit](https://docs.renovatebot.com/configuration-options/#prhourlylimit).

Tip

Manual rebases (requested via checkbox, Dependency Dashboard, or rebase label) always bypass this limit.

## commitMessage[¶](https://docs.renovatebot.com/configuration-options/#commitmessage)

Message to use for commit messages and pull request titles.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "{{{commitMessagePrefix}}} {{{commitMessageAction}}} {{{commitMessageTopic}}} {{{commitMessageExtra}}} {{{commitMessageSuffix}}}"

| supportsTemplating  | true

| env  | RENOVATE_COMMIT_MESSAGE

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

This feature has been deprecated

We deprecated editing the commitMessage directly, and we recommend you stop using this config option. Instead use config options like commitMessageAction, commitMessageExtra, and so on, to create the commit message you want.

## commitMessageAction[¶](https://docs.renovatebot.com/configuration-options/#commitmessageaction)

Action verb to use in commit messages and PR titles.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "Update"

| supportsTemplating  | true

| env  | RENOVATE_COMMIT_MESSAGE_ACTION

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Warning

For advanced use only! Use at your own risk!

This is used to alter commitMessage and prTitle without needing to copy/paste the whole string. Actions may be like Update, Pin, Roll back, Refresh, etc. Check out the default value for commitMessage to understand how this field is used.

## commitMessageExtra[¶](https://docs.renovatebot.com/configuration-options/#commitmessageextra)

Extra description used after the commit message topic - typically the version.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "to {{#if isPinDigest}}{{{newDigestShort}}}{{else}}{{#if isMajor}}{{prettyNewMajor}}{{else}}{{#if isSingleVersion}}{{prettyNewVersion}}{{else}}{{#if newValue}}{{{newValue}}}{{else}}{{{newDigestShort}}}{{/if}}{{/if}}{{/if}}{{/if}}"

| supportsTemplating  | true

| env  | RENOVATE_COMMIT_MESSAGE_EXTRA

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Warning

For advanced use only! Use at your own risk!

This is used to alter commitMessage and prTitle without needing to copy/paste the whole string. The "extra" is usually an identifier of the new version, e.g. "to v1.3.2" or "to tag 9.2".

## commitMessageLowerCase[¶](https://docs.renovatebot.com/configuration-options/#commitmessagelowercase)

Lowercase PR- and commit titles.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| allowedValues  |

```
["auto", "never"]

```

| default  | "auto"

| cli  | --commit-message-lower-case

| env  | RENOVATE_COMMIT_MESSAGE_LOWER_CASE

With semanticCommits pr- and commit-titles will by default ("auto") be converted to all-lowercase. Set this to "never" to leave the titles untouched, allowing uppercase characters in semantic commit titles.

## commitMessagePrefix[¶](https://docs.renovatebot.com/configuration-options/#commitmessageprefix)

Prefix to add to start of commit messages and PR titles. Uses a semantic prefix if semanticCommits is enabled.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| supportsTemplating  | true

| env  | RENOVATE_COMMIT_MESSAGE_PREFIX

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Warning

For advanced use only! Use at your own risk!

This is used to alter commitMessage and prTitle (which also affects how it's displayed in the Dependency Dashboard) without needing to copy/paste the whole string. The "prefix" is usually an automatically applied semantic commit prefix, but it can also be statically configured.

## commitMessageSuffix[¶](https://docs.renovatebot.com/configuration-options/#commitmessagesuffix)

Suffix to add to end of commit messages and PR titles.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| supportsTemplating  | true

| env  | RENOVATE_COMMIT_MESSAGE_SUFFIX

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Warning

For advanced use only! Use at your own risk!

This is used to add a suffix to commit messages. Usually left empty except for internal use (multiple base branches, and vulnerability alerts).

## commitMessageTopic[¶](https://docs.renovatebot.com/configuration-options/#commitmessagetopic)

The upgrade topic/noun used in commit messages and PR titles.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "dependency {{depName}}"

| supportsTemplating  | true

| env  | RENOVATE_COMMIT_MESSAGE_TOPIC

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Warning

For advanced use only! Use at your own risk!

You can use commitMessageTopic to change the commitMessage and prTitle without copy/pasting the whole string. The "topic" usually refers to the dependency being updated, for example: "dependency react".

We recommend you use matchManagers and commitMessageTopic in a packageRules array to set the commit message topic, like this:

```
{
  "packageRules": [
    {
      "matchManagers": ["github-actions"],
      "commitMessageTopic": "{{depName}}"
    }
  ]
}

```

## commitTrailers[¶](https://docs.renovatebot.com/configuration-options/#committrailers)

Structured git trailers (Key: value lines) to add in the final block of the commit message.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| mergeable  | true

| supportsTemplating  | true

| env  | RENOVATE_COMMIT_TRAILERS

| default  | []

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Use this option to add [git trailers](https://git-scm.com/docs/git-interpret-trailers) to the commits Renovate creates. Each entry must be a full trailer line in the form Key: value, where the key may only have letters, digits and -. Repeated keys are allowed.

For example:

```
{
  "packageRules": [
    {
      "matchDepTypes": ["devDependencies"],
      "commitTrailers": ["Changelog: skip"]
    }
  ]
}

```

If you want Renovate to sign off its commits, you can add the [:gitSignOff preset](https://docs.renovatebot.com/presets-default/#gitsignoff) to your extends array:

```
{
  "extends": [":gitSignOff"]
}

```

Onboarding and config migration commits expose only {{{gitAuthor}}} from the repository config to this template.

## composerIgnorePlatformReqs[¶](https://docs.renovatebot.com/configuration-options/#composerignoreplatformreqs)

Configure use of --ignore-platform-reqs or --ignore-platform-req for the Composer package manager.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| cli  | --composer-ignore-platform-reqs

| env  | RENOVATE_COMPOSER_IGNORE_PLATFORM_REQS

| default  | []

By default, Renovate will ignore Composer platform requirements as the PHP platform used by Renovate most probably won't match the required PHP environment of your project as configured in your composer.json file.

Composer 2.2 and up will be run with --ignore-platform-req='ext-*' --ignore-platform-req='lib-*', which ignores extension and library platform requirements but not the PHP version itself and should work in most cases.

Older Composer versions will be run with --ignore-platform-reqs, which means that all platform constraints (including the PHP version) will be ignored by default. This can result in updated dependencies that are not compatible with your platform.

To customize this behavior, you can explicitly ignore platform requirements (for example ext-zip) by setting them separately in this array. Each item will be added to the Composer command with --ignore-platform-req, resulting in it being ignored during its invocation. Note that this requires your project to use Composer V2, as V1 doesn't support excluding single platform requirements. The used PHP version will be guessed automatically from your composer.json definition, so php should not be added as explicit dependency.

If an empty array is configured, Renovate uses its default behavior.

Set to null (not recommended) to fully omit --ignore-platform-reqs/--ignore-platform-req during Composer invocation. This requires the Renovate image to be fully compatible with your Composer platform requirements in order for the Composer invocation to succeed, otherwise Renovate will fail to create the updated lock file. The Composer output should inform you about the reasons the update failed.

## confidential[¶](https://docs.renovatebot.com/configuration-options/#confidential)

If enabled, issues created by Renovate are set as confidential.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | false

| supportedPlatforms  | gitlab

| cli  | --confidential

| env  | RENOVATE_CONFIDENTIAL

If enabled, all issues created by Renovate are set as confidential, even in a public repository.

Note

The Dependency Dashboard issue will also be confidential. By default issues created by Renovate are visible to all users.

Note

This option is applicable to GitLab only.

## configMigration[¶](https://docs.renovatebot.com/configuration-options/#configmigration)

Enable this to get config migration PRs when needed.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | false

| cli  | --config-migration

| env  | RENOVATE_CONFIG_MIGRATION

This feature is flagged as experimental

Config migration PRs are still being improved, in particular to reduce the amount of reordering and whitespace changes.
To track this feature visit the following GitHub issue [#16359](https://github.com/renovatebot/renovate/issues/16359).

If enabled, Renovate raises a pull request when it needs to migrate the Renovate config file. Renovate only performs configMigration on .json and .json5 files.

We're adding new features to Renovate often. Often you can keep using your Renovate config and use the new features right away. But sometimes you need to update your Renovate configuration. To help you with this, Renovate will create config migration pull requests, when you enable configMigration.

Example:

After we changed the [baseBranchPatterns](https://docs.renovatebot.com/configuration-options/#basebranchpatterns) feature, the Renovate configuration migration pull request would make this change:

```
{
- "baseBranch": "main"
+ "baseBranchPatterns": ["main"]
}

```

Warning

The configMigration feature writes plain JSON for .json files, and JSON5 for .json5 files. Renovate may downgrade JSON5 content to plain JSON. When downgrading JSON5 to JSON Renovate may also remove the JSON5 comments. This can happen because Renovate wrongly converts JSON5 to JSON, thus removing the comments.

For more details, read the [config migration documentation](https://docs.renovatebot.com/config-migration/).

## configWarningReuseIssue[¶](https://docs.renovatebot.com/configuration-options/#configwarningreuseissue)

Set this to true to make Renovate reuse/reopen an existing closed Config Warning issue, instead of opening a new one each time.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | false

| cli  | --config-warning-reuse-issue

| env  | RENOVATE_CONFIG_WARNING_REUSE_ISSUE

If no existing issue is found, Renovate's default behavior is to create a new Config Warning issue. Accordingly, you'll get a new issue each time you have a Config Warning, although never more than one open at a time. Configure this option to true if you prefer Renovate to reopen any found matching closed issue whenever there is a config warning. The downside of this is that you may end up with a very old issue getting "recycled" each time and it will sort older than others.

## constraints[¶](https://docs.renovatebot.com/configuration-options/#constraints)

Configuration object to define language or manager version constraints.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | object

| default  |

```
{"ghActionsLock": "v0.4.0"}

```

| mergeable  | true

| supportedManagers  | bundler, composer, github-actions, gomod, mise, npm, pep621, pipenv, poetry

| freeChoice  | true

| env  | RENOVATE_CONSTRAINTS

Constraints are used in package managers which use third-party tools to update "artifacts" like lock files or checksum files. Typically, the constraint is detected automatically by Renovate from files within the repository and there is no need to manually configure it.

Constraints are also used to manually restrict which datasource versions are possible to upgrade to based on their language support. For now this datasource constraint feature only supports python, other compatibility restrictions will be added in the future.

```
{
  "constraints": {
    "python": "2.7"
  }
}

```

If you need to override constraints that Renovate detects from the repository, wrap it in the [global self-hosted configuration's force object](https://docs.renovatebot.com/self-hosted-configuration/#force) like so:

```
module.exports = {
  force: {
    constraints: {
      node: '< 15.0.0',
    },
  },
};

```

The following constraints are available to specify which package managers/language constraints/tools Renovate will install for your repository:

| Tool  | Additional Information  | Versioning  | Datasource

| apm  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| bazelisk  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| bun  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| bundler  |   | [ruby](https://docs.renovatebot.com/modules/versioning/ruby/)  | [rubygems](https://docs.renovatebot.com/modules/datasource/rubygems/)

| cocoapods  |   | [ruby](https://docs.renovatebot.com/modules/versioning/ruby/)  | [rubygems](https://docs.renovatebot.com/modules/datasource/rubygems/)

| composer  |   | [composer](https://docs.renovatebot.com/modules/versioning/composer/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| conan  |   | [pep440](https://docs.renovatebot.com/modules/versioning/pep440/)  | [pypi](https://docs.renovatebot.com/modules/datasource/pypi/)

| copier  |   | [pep440](https://docs.renovatebot.com/modules/versioning/pep440/)  | [pypi](https://docs.renovatebot.com/modules/datasource/pypi/)

| corepack  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [npm](https://docs.renovatebot.com/modules/datasource/npm/)

| deno  |   | [deno](https://docs.renovatebot.com/modules/versioning/deno/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| devbox  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| dotnet  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [dotnet-version](https://docs.renovatebot.com/modules/datasource/dotnet-version/)

| erlang  |   | [semver-coerced](https://docs.renovatebot.com/modules/versioning/semver-coerced/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| elixir  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| flux  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| gh  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| gleam  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| golang  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| gradle  |   | [gradle](https://docs.renovatebot.com/modules/versioning/gradle/)  | [gradle-version](https://docs.renovatebot.com/modules/datasource/gradle-version/)

| hashin  |   | [pep440](https://docs.renovatebot.com/modules/versioning/pep440/)  | [pypi](https://docs.renovatebot.com/modules/datasource/pypi/)

| helm  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| helmfile  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| java  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [java-version](https://docs.renovatebot.com/modules/datasource/java-version/)

| jb  |   | [loose](https://docs.renovatebot.com/modules/versioning/loose/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| kustomize  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| maven  |   | [maven](https://docs.renovatebot.com/modules/versioning/maven/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| mise  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| nix  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| node  |   | [node](https://docs.renovatebot.com/modules/versioning/node/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| npm  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [npm](https://docs.renovatebot.com/modules/datasource/npm/)

| paket  |   | [paket](https://docs.renovatebot.com/modules/versioning/paket/)  | [nuget](https://docs.renovatebot.com/modules/datasource/nuget/)

| pdm  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| php  |   | [composer](https://docs.renovatebot.com/modules/versioning/composer/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| pip-tools  |   | [pep440](https://docs.renovatebot.com/modules/versioning/pep440/)  | [pypi](https://docs.renovatebot.com/modules/datasource/pypi/)

| pipenv  |   | [pep440](https://docs.renovatebot.com/modules/versioning/pep440/)  | [pypi](https://docs.renovatebot.com/modules/datasource/pypi/)

| pnpm  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [npm](https://docs.renovatebot.com/modules/datasource/npm/)

| pixi  |   | [conda](https://docs.renovatebot.com/modules/versioning/conda/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| poetry  |   | [pep440](https://docs.renovatebot.com/modules/versioning/pep440/)  | [pypi](https://docs.renovatebot.com/modules/datasource/pypi/)

| python  |   | [python](https://docs.renovatebot.com/modules/versioning/python/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| ruby  | Also used in the rubygems Datasource  | [ruby](https://docs.renovatebot.com/modules/versioning/ruby/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| rust  |   | [rust-release-channel](https://docs.renovatebot.com/modules/versioning/rust-release-channel/)  | [rust-version](https://docs.renovatebot.com/modules/datasource/rust-version/)

| uv  |   | [pep440](https://docs.renovatebot.com/modules/versioning/pep440/)  | [pypi](https://docs.renovatebot.com/modules/datasource/pypi/)

| yarn  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [npm](https://docs.renovatebot.com/modules/datasource/npm/)

| yarn-slim  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [npm](https://docs.renovatebot.com/modules/datasource/npm/)

| dart  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [dart-version](https://docs.renovatebot.com/modules/datasource/dart-version/)

| flutter  |   | [npm](https://docs.renovatebot.com/modules/versioning/npm/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

| vendir  |   | [semver](https://docs.renovatebot.com/modules/versioning/semver/)  | [github-releases](https://docs.renovatebot.com/modules/datasource/github-releases/)

Note

When using [binarySource=global](https://docs.renovatebot.com/self-hosted-configuration/#binarysource), the constraints options do not take effect.

Additionally, there are several additional constraints that can be specified:

| Constraint  | Additional Information

| ghActionsLock  | Used in the github-actions manager to specify a release tag for the [github/gh-actions-lock](https://github.com/github/gh-actions-lock) gh CLI extension, which regenerates .github/workflows/actions.lock.

Must be a full release tag, prefixed with v, such as v0.1.7. Set it to an empty string to always install the latest release.

| go  | Used in the gomod manager to specify the version of the Go toolchain to use.

In precedence order:

1. config: constraints.go
1. go.mod: toolchain directive
1. go.mod: go directive

NOTE that the constraints.golang is not used (https://github.com/renovatebot/renovate/issues/42601)

| gomodMod  | Used in the gomod manager to specify a tag for [github.com/marwan-at-work/mod](https://github.com/marwan-at-work/mod).

Must be prefixed with v.

| jenkins  | Used in the jenkins-plugins datasource to specify a minimum version of Jenkins that a plugin must support.

| pipTools  | Used in the pip-compile manager to specify a version of pip-tools to use. @deprecated TODO remove in #42599

| platform  | Used in the rubygems datasource to specify the platform that the Gem dependency supports.

| rubygems  | Used in the rubygems datasource to specify the version of the rubygems tool that is needed to use this Gem.

| vscode  | Used in the npm manager to track the version of VSCode that the package is compatible with.

| dotnet-sdk  | Used in the nuget manager to track .NET SDK version required.

| perl  | Used in the cpanfile manager to track Perl version required.

| %goMod  | Used in the gomod manager to determine the [minimum version of Go required to use this module](https://go.dev/ref/mod#go-mod-file-go).

Note that this is prefixed with a % to explicitly note that this is not a tool that Containerbase knows.

Note

Make sure not to mix this up with the term compatibility, which Renovate uses in the context of version releases, e.g. if a Docker image is node:12.16.0-alpine then the -alpine suffix represents compatibility.

## constraintsFiltering[¶](https://docs.renovatebot.com/configuration-options/#constraintsfiltering)

Perform release filtering based on language constraints.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | string

| allowedValues  |

```
["none", "strict"]

```

| default  | "none"

| env  | RENOVATE_CONSTRAINTS_FILTERING

This option controls whether Renovate filters new releases based on configured or detected constraints. Renovate supports two options:

- none: No release filtering (all releases allowed)
- strict: If the release's constraints match the package file constraints, then it's included

More advanced filtering options may come in future.

There must be a constraints object in your Renovate config, or constraints detected from package files, for this to work. Additionally, the "datasource" within Renovate must be capable of returning constraints values about each package's release.

This feature is limited to the following datasources:

- crate
- go
- jenkins-plugins
- npm
- packagist
- pub
- pypi
- rubygems

Sometimes when using private registries they may omit constraints information, which then is another reason such filtering may not work even if the datasource and corresponding default public registry supports it.

Warning

Enabling this feature may result in many package updates being filtered out silently. See below for a description of how it works.

Warning

Enabling this feature when using [binarySource=global](https://docs.renovatebot.com/self-hosted-configuration/#binarysource) can lead to situations where Renovate suggests updates that it cannot then update, as it does not have the right tool versions installed.

When constraintsFiltering=strict, the following logic applies:

- Are there constraints for this repository, either detected from source or from config?
- Does this package's release declare constraints of its own (e.g. engines in Node.js)?
- If so, filter out this release unless the repository constraint is a subset of the release constraint

Here are some examples:

| Your repo engines.node  | Dependency release engines.node  | Result

| 18  | 16 \|\| 18  | allowed

| ^18.10.0  | >=18  | allowed

| ^16.10.0 \|\| >=18.0.0  | >= 16.0.0  | allowed

| >=16  | 16 \|\| 18  | filtered

| 16  | ^16.10.0  | filtered

When using with npm, we recommend you:

- Use constraintsFiltering on dependencies, not devDependencies (usually you do not need to be strict about development dependencies)
- Do not enable rollbackPrs at the same time (otherwise your current version may be rolled back if it's incompatible)

## constraintsVersioning[¶](https://docs.renovatebot.com/configuration-options/#constraintsversioning)

Override the versioning scheme used when filtering releases by specific constraint names. Does not apply to tools.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| mergeable  | true

| freeChoice  | true

Use constraintsVersioning to override the versioning scheme used when filtering releases by specific constraint names with [constraintsFiltering](https://docs.renovatebot.com/configuration-options/#constraintsfiltering).

Note

Each key must be an [additional constraint name](https://docs.renovatebot.com/configuration-options/#constraints) (i.e. not a tool name), and each value must be a valid versioning scheme ID.

Datasources that support constraints filtering may set sensible defaults for their constraints. You can override these defaults in your Renovate config.

For example, to use a SemVer-style versioning scheme when defining the rubygems constraint:

Use SemVer-style range for defining version constraint, instead of 'ruby' versioning

```
{
  "constraints": {
    "rubygems": "^1.3"
  },
  "constraintsVersioning": {
    "rubygems": "semver-coerced"
  }
}

```

## customDatasources[¶](https://docs.renovatebot.com/configuration-options/#customdatasources)

Defines custom datasources for usage by managers.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| additionalProperties  |

```
{"type": "object"}

```

| mergeable  | true

| cli  | --custom-datasources

| env  | RENOVATE_CUSTOM_DATASOURCES

This feature is flagged as experimental

Experimental features might be changed or even removed at any time.
To track this feature visit the following GitHub issue [#23286](https://github.com/renovatebot/renovate/issues/23286).

Use customDatasources to fetch releases from APIs or statically hosted sites and Renovate has no own datasource. These datasources can be referred by customManagers or can be used to overwrite default datasources.

For more details see the [custom datasource documentation](https://docs.renovatebot.com/modules/datasource/custom/).

### customDatasources.defaultRegistryUrlTemplate[¶](https://docs.renovatebot.com/configuration-options/#customdatasourcesdefaultregistryurltemplate)

Template for generating a defaultRegistryUrl for custom datasource.

| Name  | Value

| type  | string

| default  | ""

| parents  | customDatasources

| supportsTemplating  | true

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

This field is used to build a registryUrl for the dependency. It is not needed if either:

- The dependency can be found with the default registryUrls of the datasource (e.g. npmjs registry if the datasource is npm), or
- The matching groups you specified as part of the matching already include a registryUrl group As this is a template it can be dynamically set. E.g. add the packageName as part of the URL:

```
{
  customDatasources: {
    foo: {
      defaultRegistryUrlTemplate: 'https://example.foo.bar/v1/{{ packageName }}',
    },
  },
}

```

### customDatasources.format[¶](https://docs.renovatebot.com/configuration-options/#customdatasourcesformat)

Format of the custom datasource.

| Name  | Value

| type  | string

| parents  | customDatasources

| default  | "json"

| allowedValues  |

```
["html", "json", "plain", "toml", "yaml"]

```

Defines which format the API is returning. Currently json or plain are supported, see the custom [datasource documentation](https://docs.renovatebot.com/modules/datasource/custom/) for more information.

### customDatasources.transformTemplates[¶](https://docs.renovatebot.com/configuration-options/#customdatasourcestransformtemplates)

List of jsonata transformation rules.

| Name  | Value

| type  | array

| subType  | string

| parents  | customDatasources

| supportsTemplating  | true

| cli  | --transform-templates

| env  | RENOVATE_TRANSFORM_TEMPLATES

| default  | []

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

transformTemplates is a list of [jsonata rules](https://docs.jsonata.org/simple) which get applied serially. Use this if the API does not return a Renovate compatible schema.

## customManagers[¶](https://docs.renovatebot.com/configuration-options/#custommanagers)

Custom managers using regex matching.

| Name  | Value

| scopes  | repo

| type  | array

| subType  | object

| cli  | --custom-managers

| mergeable  | true

| env  | RENOVATE_CUSTOM_MANAGERS

| previouslyKnownAs  |

```
["regexManagers"]

```

| default  | []

Use customManagers(previously regexManagers) entries to configure the custom managers in Renovate.

You can define custom managers to handle:

- Proprietary file formats or conventions
- Popular file formats not yet supported as a manager by Renovate

Renovate has two custom managers:

| Custom manager  | Matching engine

| regex  | Regular Expression, with named capture groups.

| jsonata  | JSONata query.

To use a custom manager, you must give Renovate this information:

- managerFilePatterns: regex/glob pattern of the file to extract deps from
- matchStrings: regex patterns or jsonata queries used to process the file

The matchStrings must capture/extract the following three fields:

- datasource
- depName and / or packageName
- currentValue

Alternatively, you could also use corresponding templates (e.g. depNameTemplate) for these fields. But, we recommend you use only one of these methods, or you'll get confused.

Also, we recommend you explicitly set which versioning Renovate should use.

Renovate defaults to semver-coerced versioning if both condition are met:

- The versioning field is missing in the custom manager config
- The Renovate datasource does not set its own default versioning

For more details and examples regarding each custom manager, see our documentation for the [regex manager](https://docs.renovatebot.com/modules/manager/regex/) and the [JSONata manager](https://docs.renovatebot.com/modules/manager/jsonata/). For template fields, use the triple brace {{{ }}} notation to avoid Handlebars escaping any special characters.

Tip

Look at our [Custom Manager Presets](https://docs.renovatebot.com/presets-customManagers/), they may have what you need.

### customManagers.autoReplaceStringTemplate[¶](https://docs.renovatebot.com/configuration-options/#custommanagersautoreplacestringtemplate)

Optional extractVersion for extracted dependencies. Valid only within a customManagers object.

| Name  | Value

| type  | string

| parents  | customManagers

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Allows overwriting how the matched string is replaced. This allows for some migration strategies. E.g. moving from one Docker image repository to another one.

helm-values.yaml

```
# The image of the service <registry>/<repo>/<image>:<tag>
image: my.old.registry/aRepository/andImage:1.18-alpine

```

The regex definition

```
{
  "customManagers": [
    {
      "customType": "regex",
      "managerFilePatterns": ["/values.yaml$/"],
      "matchStrings": [
        "image:\\s+(?<depName>my\\.old\\.registry/aRepository/andImage):(?<currentValue>[^\\s]+)"
      ],
      "depNameTemplate": "my.new.registry/aRepository/andImage",
      "autoReplaceStringTemplate": "image: {{{depName}}}:{{{newValue}}}",
      "datasourceTemplate": "docker"
    }
  ]
}

```

This will lead to following update where 1.21-alpine is the newest version of my.new.registry/aRepository/andImage:

```
# The image of the service <registry>/<repo>/<image>:<tag>
image: my.new.registry/aRepository/andImage:1.21-alpine

```

Note

Can only be used with the custom regex manager.

### customManagers.currentValueTemplate[¶](https://docs.renovatebot.com/configuration-options/#custommanagerscurrentvaluetemplate)

Optional currentValue for extracted dependencies. Valid only within a customManagers object.

| Name  | Value

| type  | string

| parents  | customManagers

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

If the currentValue for a dependency is not captured with a named group then it can be defined in config using this field. It will be compiled using Handlebars and the regex groups result.

### customManagers.customType[¶](https://docs.renovatebot.com/configuration-options/#custommanagerscustomtype)

Custom manager to use. Valid only within a customManagers object.

| Name  | Value

| type  | string

| allowedValues  |

```
["jsonata", "regex"]

```

| parents  | customManagers

| default  | null

It specifies which custom manager to use. There are two available options: regex and jsonata.

Example:

```
{
  "customManagers": [
    {
      "customType": "regex",
      "managerFilePatterns": ["/values.yaml$/"],
      "matchStrings": [
        "ENV .*?_VERSION=(?<currentValue>.*) # (?<datasource>.*?)/(?<depName>.*?)\\s"
      ]
    }
  ]
}

```

Parsing a JSON file with a custom manager

```
{
  "customManagers": [
    {
      "customType": "jsonata",
      "fileFormat": "json",
      "managerFilePatterns": ["/file.json/"],
      "matchStrings": [
        "packages.{ \"depName\": package, \"currentValue\": version }"
      ],
      "datasourceTemplate": "github-tags"
    }
  ]
}

```

### customManagers.datasourceTemplate[¶](https://docs.renovatebot.com/configuration-options/#custommanagersdatasourcetemplate)

Optional datasource for extracted dependencies. Valid only within a customManagers object.

| Name  | Value

| type  | string

| parents  | customManagers

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

If the datasource for a dependency is not captured with a named group, then it can be defined in config using this field. It will be compiled using Handlebars and the regex groups result.

### customManagers.depNameTemplate[¶](https://docs.renovatebot.com/configuration-options/#custommanagersdepnametemplate)

Optional depName for extracted dependencies. Valid only within a customManagers object.

| Name  | Value

| type  | string

| parents  | customManagers

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

If depName cannot be captured with a named capture group in matchString then it can be defined manually using this field. It will be compiled using Handlebars and the regex groups result.

### customManagers.depTypeTemplate[¶](https://docs.renovatebot.com/configuration-options/#custommanagersdeptypetemplate)

Optional depType for extracted dependencies. Valid only within a customManagers object.

| Name  | Value

| type  | string

| parents  | customManagers

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

If depType cannot be captured with a named capture group in matchString then it can be defined manually using this field. It will be compiled using Handlebars and the regex groups result.

### customManagers.extractVersionTemplate[¶](https://docs.renovatebot.com/configuration-options/#custommanagersextractversiontemplate)

Optional extractVersion for extracted dependencies. Valid only within a customManagers object.

| Name  | Value

| type  | string

| parents  | customManagers

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

If extractVersion cannot be captured with a named capture group in matchString, then it can be defined manually using this field. It will be compiled using Handlebars and the regex groups result. See [extractVersion](https://docs.renovatebot.com/configuration-options/#extractversion) for how the value is used.

### customManagers.fileFormat[¶](https://docs.renovatebot.com/configuration-options/#custommanagersfileformat)

It specifies the syntax of the package file being managed by the custom JSONata manager.

| Name  | Value

| type  | string

| allowedValues  |

```
["json", "toml", "yaml"]

```

| parents  | customManagers

| requiredIf  | [object Object]

| default  | null

Note

Can only be used with the custom jsonata manager.

It specifies the syntax of the package file that's managed by the custom jsonata manager. This setting helps the system correctly parse and interpret the configuration file's contents.

Only the json, toml and yaml formats are supported. yaml files are parsed as multi document YAML files.

Parsing a JSON file with a custom manager

```
{
  "customManagers": [
    {
      "customType": "jsonata",
      "fileFormat": "json",
      "managerFilePatterns": ["/.renovaterc/"],
      "matchStrings": [
        "packages.{ 'depName': package, 'currentValue': version }"
      ],
      "datasourceTemplate": "github-tags"
    }
  ]
}

```

Parsing a YAML file with a custom manager

```
{
  "customManagers": [
    {
      "customType": "jsonata",
      "fileFormat": "yaml",
      "managerFilePatterns": ["/file.yml/"],
      "matchStrings": [
        "packages.{ 'depName': package, 'currentValue': version }"
      ],
      "datasourceTemplate": "github-tags"
    }
  ]
}

```

Parsing a TOML file with a custom manager

```
{
  "customManagers": [
    {
      "customType": "jsonata",
      "fileFormat": "toml",
      "managerFilePatterns": ["/file.toml/"],
      "matchStrings": [
        "packages.{ 'depName': package, 'currentValue': version }"
      ],
      "datasourceTemplate": "github-tags"
    }
  ]
}

```

### customManagers.matchStrings[¶](https://docs.renovatebot.com/configuration-options/#custommanagersmatchstrings)

Queries to use. Valid only within bumpVersions or customManagers object. Templating is supported within bumpVersions only.

| Name  | Value

| type  | array

| subType  | string

| parents  | customManagersbumpVersions

| supportsTemplating  | true

| default  | []

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Each matchStrings must be one of the following:

- A valid regular expression, which may optionally include named capture groups (if using customType=regex)
- Or, a valid, escaped [JSONata](https://docs.jsonata.org/overview.html) query (if using customType=json)

Example:

matchStrings with a valid regular expression

```
{
  "matchStrings": [
    "ENV .*?_VERSION=(?<currentValue>.*) # (?<datasource>.*?)/(?<depName>.*?)\\s"
  ]
}

```

matchStrings with a valid JSONata query

```
{
  "matchStrings": [
    "packages.{ \"depName\": package, \"currentValue\": version }"
  ]
}

```

Note

You do not need to add leading and trailing slashes in matchStrings.

### customManagers.matchStringsStrategy[¶](https://docs.renovatebot.com/configuration-options/#custommanagersmatchstringsstrategy)

Strategy how to interpret matchStrings.

| Name  | Value

| type  | string

| default  | "any"

| allowedValues  |

```
["any", "recursive", "combination"]

```

| parents  | customManagers

matchStringsStrategy controls behavior when multiple matchStrings values are provided. Three options are available:

- any (default)
- recursive
- combination

Note

matchStringsStrategy can only be used in a custom regex manager config!

#### any[¶](https://docs.renovatebot.com/configuration-options/#any)

Each provided matchString will be matched individually to the content of the packageFile. If a matchString has multiple matches in a file each will be interpreted as an independent dependency.

As example the following configuration will update all three lines in the Dockerfile.

renovate.json

```
{
  "customManagers": [
    {
      "customType": "regex",
      "managerFilePatterns": ["/^Dockerfile$/"],
      "matchStringsStrategy": "any",
      "matchStrings": [
        "ENV [A-Z]+_VERSION=(?<currentValue>.*) # (?<datasource>.*?)/(?<depName>.*?)(\\&versioning=(?<versioning>.*?))?\\s",
        "FROM (?<depName>\\S*):(?<currentValue>\\S*)"
      ],
      "datasourceTemplate": "docker"
    }
  ]
}

```

Dockerfile

```
FROM amd64/ubuntu:24.04
ENV GRADLE_VERSION=6.2 # gradle-version/gradle&versioning=maven
ENV NODE_VERSION=10.19.0 # github-tags/nodejs/node&versioning=node

```

#### recursive[¶](https://docs.renovatebot.com/configuration-options/#recursive)

If using recursive the matchStrings will be looped through and the full match of the last will define the range of the next one. This can be used to narrow down the search area to prevent multiple matches. But the recursive strategy still allows the matching of multiple dependencies as described below. All matches of the first matchStrings pattern are detected, then each of these matches will be used as basis for the input for the next matchStrings pattern, and so on. If the next matchStrings pattern has multiple matches then it will split again. This process will be followed as long there is a match plus a next matchingStrings pattern is available.

Matched groups will be available in subsequent matching layers.

This is an example how this can work. The first custom manager will only upgrade grafana/loki as looks for the backup key then looks for the test key and then uses this result for extraction of necessary attributes. But the second custom manager will upgrade both definitions as its first matchStrings matches both test keys.

renovate.json

```
{
  "customManagers": [
    {
      "customType": "regex",
      "managerFilePatterns": ["/^example.json$/"],
      "matchStringsStrategy": "recursive",
      "matchStrings": [
        "\"backup\":\\s*{[^}]*}",
        "\"test\":\\s*\\{[^}]*}",
        "\"name\":\\s*\"(?<depName>.*)\"[^\"]*\"type\":\\s*\"(?<datasource>.*)\"[^\"]*\"value\":\\s*\"(?<currentValue>.*)\""
      ],
      "datasourceTemplate": "docker"
    },
    {
      "customType": "regex",
      "managerFilePatterns": ["/^example.json$/"],
      "matchStringsStrategy": "recursive",
      "matchStrings": [
        "\"test\":\\s*\\{[^}]*}",
        "\"name\":\\s*\"(?<depName>.*)\"[^\"]*\"type\":\\s*\"(?<datasource>.*)\"[^\"]*\"value\":\\s*\"(?<currentValue>.*)\""
      ],
      "datasourceTemplate": "docker"
    }
  ]
}

```

example.json

```
{
  "backup": {
    "test": {
      "name": "grafana/loki",
      "type": "docker",
      "value": "1.6.1"
    }
  },
  "setup": {
    "test": {
      "name": "python",
      "type": "docker",
      "value": "3.9.0"
    }
  }
}

```

#### combination[¶](https://docs.renovatebot.com/configuration-options/#combination)

You may use this option to combine the values of multiple lines inside a file. You can combine multiple lines with matchStringStrategy values, but the combination approach is less susceptible to white space or line breaks stopping a match.

combination can only match one dependency per file. To update multiple dependencies with combination you must define multiple custom managers.

Matched group values will be merged to form a single dependency.

renovate.json

```
{
  "customManagers": [
    {
      "customType": "regex",
      "managerFilePatterns": ["/^main.yml$/"],
      "matchStringsStrategy": "combination",
      "matchStrings": [
        "prometheus_image:\\s*\"(?<depName>.*)\"\\s*//",
        "prometheus_version:\\s*\"(?<currentValue>.*)\"\\s*//"
      ],
      "datasourceTemplate": "docker"
    },
    {
      "customType": "regex",
      "managerFilePatterns": ["/^main.yml$/"],
      "matchStringsStrategy": "combination",
      "matchStrings": [
        "thanos_image:\\s*\"(?<depName>.*)\"\\s*//",
        "thanos_version:\\s*\"(?<currentValue>.*)\"\\s*//"
      ],
      "datasourceTemplate": "docker"
    }
  ]
}

```

Ansible variable file (YAML)

```
prometheus_image: "prom/prometheus"  // a comment
prometheus_version: "v2.21.0" // a comment
------
thanos_image: "prom/prometheus"  // a comment
thanos_version: "0.15.0" // a comment

```

In the above example, each custom manager will match a single dependency each.

### customManagers.packageNameTemplate[¶](https://docs.renovatebot.com/configuration-options/#custommanagerspackagenametemplate)

Optional packageName for extracted dependencies, else defaults to depName value. Valid only within a customManagers object.

| Name  | Value

| type  | string

| parents  | customManagers

| supportsTemplating  | true

| previouslyKnownAs  |

```
["lookupNameTemplate"]

```

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

packageName is used for looking up dependency versions. It will be compiled using Handlebars and the regex groups result. It will default to the value of depName if left unconfigured/undefined.

### customManagers.registryUrlTemplate[¶](https://docs.renovatebot.com/configuration-options/#custommanagersregistryurltemplate)

Optional registry URL for extracted dependencies. Valid only within a customManagers object.

| Name  | Value

| type  | string

| parents  | customManagers

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

If the registryUrls for a dependency is not captured with a named group then it can be defined in config using this field. It will be compiled using Handlebars and the regex groups result.

### customManagers.versioningTemplate[¶](https://docs.renovatebot.com/configuration-options/#custommanagersversioningtemplate)

Optional versioning for extracted dependencies. Valid only within a customManagers object.

| Name  | Value

| type  | string

| parents  | customManagers

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

If the versioning for a dependency is not captured with a named group then it can be defined in config using this field. It will be compiled using Handlebars and the regex groups result.

## customizeDashboard[¶](https://docs.renovatebot.com/configuration-options/#customizedashboard)

Customize sections in the Dependency Dashboard issue.

| Name  | Value

| scopes  | repo

| type  | object

| freeChoice  | true

| additionalProperties  |

```
{"type": "string"}

```

| cli  | --customize-dashboard

| env  | RENOVATE_CUSTOMIZE_DASHBOARD

You may use the customizeDashboard object to customize the Dependency Dashboard.

Supported fields:

- repoProblemsHeader: This field will replace the header of the Repository Problems in the Dependency Dashboard issue.

## defaultRegistryUrls[¶](https://docs.renovatebot.com/configuration-options/#defaultregistryurls)

List of registry URLs to use as the default for a datasource.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | array

| subType  | string

| default  | []

Override a datasource's default registries with this config option. The datasources's customRegistrySupport value must be true for the config option to work.

Default registries are only used when both:

- The manager did not extract any registryUrls values, and
- No registryUrls values have been applied via config, such as packageRules

Think of defaultRegistryUrls as a way to specify the "fallback" registries for a datasource, for use when no registryUrls are extracted or configured. Compare that to registryUrls, which are a way to override registries.

## dependencyDashboard[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboard)

Whether to create a "Dependency Dashboard" issue in the repository.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | false

| cli  | --dependency-dashboard

| env  | RENOVATE_DEPENDENCY_DASHBOARD

| previouslyKnownAs  |

```
["masterIssue"]

```

Starting from version v26.0.0 the "Dependency Dashboard" is enabled by default as part of the commonly-used config:recommended preset.

To disable the Dependency Dashboard, add the preset :disableDependencyDashboard or set dependencyDashboard to false.

```
{
  "extends": ["config:recommended", ":disableDependencyDashboard"]
}

```

Configuring dependencyDashboard to true will lead to the creation of a "Dependency Dashboard" issue within the repository. This issue has a list of all PRs pending, open, closed (unmerged) or in error. The goal of this issue is to give visibility into all updates that Renovate is managing.

Examples of what having a Dependency Dashboard will allow you to do:

- View all PRs in one place, rather than having to filter PRs by author
- Rebase/retry multiple PRs without having to open each individually
- Override any rate limiting (e.g. concurrent PRs) or scheduling to force Renovate to create a PR that would otherwise be suppressed
- Recreate an unmerged PR (e.g. for a major update that you postponed by closing the original PR)

Tip

Enabling the Dependency Dashboard by itself does not change the "control flow" of Renovate. Renovate still creates and manages PRs, and still follows your schedules and rate limits. The Dependency Dashboard gives you extra visibility and control over your updates.

## dependencyDashboardApproval[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboardapproval)

Controls if updates need manual approval from the Dependency Dashboard issue before PRs are created.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --dependency-dashboard-approval

| env  | RENOVATE_DEPENDENCY_DASHBOARD_APPROVAL

| previouslyKnownAs  |

```
["masterIssueApproval"]

```

This feature allows you to use Renovate's Dependency Dashboard to force approval of updates before they are created.

By setting dependencyDashboardApproval to true in config (including within packageRules), you can tell Renovate to wait for your approval from the Dependency Dashboard before creating a branch/PR. You can approve a pending PR by selecting the checkbox in the Dependency Dashboard issue.

Tip

When you set dependencyDashboardApproval to true the Dependency Dashboard issue will be created automatically, you do not need to turn on dependencyDashboard explicitly.

You can configure Renovate to wait for approval for:

- all package upgrades
- major, minor, patch level upgrades
- specific package upgrades
- upgrades coming from specific package managers

If you want to require approval for all upgrades, set dependencyDashboardApproval to true:

```
{
  "dependencyDashboardApproval": true
}

```

If you want to require approval for major updates, set dependencyDashboardApproval to true within a major object:

```
{
  "major": {
    "dependencyDashboardApproval": true
  }
}

```

If you want to approve specific packages, set dependencyDashboardApproval to true within a packageRules entry where you have defined a specific package or pattern.

```
{
  "packageRules": [
    {
      "matchPackageNames": ["/^@package-name/"],
      "dependencyDashboardApproval": true
    }
  ]
}

```

## dependencyDashboardAutoclose[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboardautoclose)

Set to true to let Renovate close the Dependency Dashboard issue if there are no more updates.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | false

| cli  | --dependency-dashboard-autoclose

| env  | RENOVATE_DEPENDENCY_DASHBOARD_AUTOCLOSE

| previouslyKnownAs  |

```
["masterIssueAutoclose"]

```

You can configure this to true if you prefer Renovate to close an existing Dependency Dashboard whenever there are no outstanding PRs left.

## dependencyDashboardCategory[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboardcategory)

The category to group branches on the Dependency Dashboard issue.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

You can use this to categorize updates on the Dependency Dashboard. For example, to visually distinguish between production and dev updates, or to split between teams or logical parts of a monorepo. In practice this means it introduces an extra level of hierarchy/heading in the Dashboard's markdown.

To create a category for all CI/CD updates, you can configure a package rule like this:

```
{
  "packageRules": [
    {
      "matchCategories": ["cd", "ci"],
      "dependencyDashboardCategory": ":hammer_and_wrench: Continuous Integration & Continuous Deployment"
    }
  ]
}

```

When you configure categories, updates that do not match any of the configured categories will be grouped under the "Other" category.

Some sections in the Dependency Dashboard also contain an action that applies to all updates in that section, such as to approve or rebase all updates. If there are categories in that section, the action will be put in an extra "All" category at the end of the section. This is to explicitly clarify that the action applies to all updates in the section, not to just a specific category.

The Dependency Dashboard categories are only used to visually organize updates within the Dependency Dashboard issue. They do not impact grouping of updates into a single update like how e.g. branchName and groupName do.

## dependencyDashboardFooter[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboardfooter)

Any text added here will be placed last in the Dependency Dashboard issue body, with a divider separator before it.

| Name  | Value

| scopes  | repo

| type  | string

| supportsTemplating  | true

| cli  | --dependency-dashboard-footer

| env  | RENOVATE_DEPENDENCY_DASHBOARD_FOOTER

| previouslyKnownAs  |

```
["masterIssueFooter"]

```

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

## dependencyDashboardHeader[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboardheader)

Any text added here will be placed first in the Dependency Dashboard issue body.

| Name  | Value

| scopes  | repo

| type  | string

| default  | "This issue lists Renovate updates and detected dependencies. Read the [Dependency Dashboard](https://docs.renovatebot.com/key-concepts/dashboard/) docs to learn more."

| supportsTemplating  | true

| cli  | --dependency-dashboard-header

| env  | RENOVATE_DEPENDENCY_DASHBOARD_HEADER

| previouslyKnownAs  |

```
["masterIssueHeader"]

```

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

## dependencyDashboardLabels[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboardlabels)

These labels will always be applied on the Dependency Dashboard issue, even when they have been removed manually.

| Name  | Value

| scopes  | repo

| type  | array

| subType  | string

| cli  | --dependency-dashboard-labels

| env  | RENOVATE_DEPENDENCY_DASHBOARD_LABELS

| previouslyKnownAs  |

```
["masterIssueLabels"]

```

| default  | []

The labels only get updated when the Dependency Dashboard issue updates its content and/or title. It is pointless to edit the labels, as Renovate restores the labels on each run.

## dependencyDashboardOSVVulnerabilitySummary[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboardosvvulnerabilitysummary)

Control if the Dependency Dashboard issue lists CVEs supplied by [osv.dev](https://osv.dev).

| Name  | Value

| scopes  | repo

| type  | string

| allowedValues  |

```
["none", "all", "unresolved"]

```

| default  | "none"

| cli  | --dependency-dashboard-o-s-v-vulnerability-summary

| env  | RENOVATE_DEPENDENCY_DASHBOARD_O_S_V_VULNERABILITY_SUMMARY

This feature is flagged as experimental

Experimental features might be changed or even removed at any time.

Use this option to control if the Dependency Dashboard lists the OSV-sourced CVEs for your repository. You can choose from:

- none (default) do not list any CVEs
- unresolved list CVEs that have no fixes
- all list all CVEs

You will only get OSV-based vulnerability alerts for direct dependencies.

This feature is independent of the osvVulnerabilityAlerts option.

The source of these CVEs is [OSV.dev](https://osv.dev/).

## dependencyDashboardReportAbandonment[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboardreportabandonment)

Controls whether abandoned packages are reported in the dependency dashboard.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | true

| cli  | --dependency-dashboard-report-abandonment

| env  | RENOVATE_DEPENDENCY_DASHBOARD_REPORT_ABANDONMENT

Controls whether abandoned packages are reported in the dependency dashboard.

When enabled (default), Renovate will display a collapsible section in the dependency dashboard listing packages that have been identified as abandoned based on the abandonmentThreshold configuration. This helps you identify dependencies that may need attention due to lack of maintenance.

Set this to false if you prefer not to see abandoned packages in your dependency dashboard.

## dependencyDashboardTitle[¶](https://docs.renovatebot.com/configuration-options/#dependencydashboardtitle)

Title for the Dependency Dashboard issue.

| Name  | Value

| scopes  | repo

| type  | string

| default  | "Dependency Dashboard"

| cli  | --dependency-dashboard-title

| env  | RENOVATE_DEPENDENCY_DASHBOARD_TITLE

| previouslyKnownAs  |

```
["masterIssueTitle"]

```

Configure this option if you prefer a different title for the Dependency Dashboard.

## description[¶](https://docs.renovatebot.com/configuration-options/#description)

Plain text description for a config or preset.

| Name  | Value

| scopes  | any

| type  | array

| subType  | string

| mergeable  | true

| default  | []

The description field can be used inside any configuration object to add a human-readable description of the object's config purpose. A description field embedded within a preset is also collated as part of the onboarding description unless the preset only consists of presets itself. Presets which consist only of other presets have their own description omitted from the onboarding description because they will be fully described by the preset descriptions within.

[!NOTE] To overwrite descriptions of child presets, use [overrideDescription](https://docs.renovatebot.com/configuration-options/#overridedescription) in place of description.

## digest[¶](https://docs.renovatebot.com/configuration-options/#digest)

Configuration to apply when updating a digest (no change in tag/version).

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| default  |

```
{
  "branchTopic": "{{{depNameSanitized}}}-digest",
  "commitMessageExtra": "to {{newDigestShort}}",
  "commitMessageTopic": "{{{depName}}} digest"
}

```

| mergeable  | true

| env  | RENOVATE_DIGEST

Add to this object if you wish to define rules that apply only to PRs that update digests.

## draftPR[¶](https://docs.renovatebot.com/configuration-options/#draftpr)

If set to true then Renovate creates draft PRs, instead of normal status PRs.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| supportedPlatforms  | azure, bitbucket, forgejo, gitea, github, gitlab, scm-manager

| cli  | --draft-p-r

| env  | RENOVATE_DRAFT_P_R

If you want the PRs created by Renovate to be considered as drafts rather than normal PRs, you could add this property to your renovate.json:

```
{
  "draftPR": true
}

```

This option is evaluated at PR/MR creation time.

Note

Forgejo, Gitea and GitLab implement draft status by checking if the PR's title starts with certain strings. This means that draftPR on Forgejo, Gitea and GitLab are incompatible with the legacy method of triggering Renovate to rebase a PR by renaming the PR to start with rebase!.

## enabled[¶](https://docs.renovatebot.com/configuration-options/#enabled)

Enable or disable corresponding functionality.

| Name  | Value

| type  | boolean

| default  | true

| scopes  | repo, packageRule, manager, updateType

| parents  | vulnerabilityAlertshostRules

| cli  | --enabled

| env  | RENOVATE_ENABLED

The most common use of enabled is if you want to turn Renovate's functionality off, for some reason.

For example, if you wanted to disable Renovate completely on a repository, you could make this your renovate.json:

```
{
  "enabled": false
}

```

Note

When Renovate is disabled on a repository entirely (via enabled=false), if Renovate runs against the repo again, it will perform a one-time cleanup of the repository, closing open Issues, PRs and deleting any branches.

To disable Renovate for all eslint packages, you can configure a package rule like:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["eslint**"],
      "enabled": false
    }
  ]
}

```

To disable Renovate for npm devDependencies but keep it for dependencies you could configure:

```
{
  "packageRules": [
    {
      "matchManagers": ["npm"],
      "matchDepTypes": ["devDependencies"],
      "enabled": false
    }
  ]
}

```

## enabledManagers[¶](https://docs.renovatebot.com/configuration-options/#enabledmanagers)

A list of package managers to enable. Only managers on the list are enabled.

| Name  | Value

| scopes  | repo

| type  | array

| subType  | string

| allowedValues  |

```
[
  "ansible",
  "ansible-galaxy",
  "ant",
  "apm",
  "argocd",
  "asdf",
  "azure-pipelines",
  "batect",
  "batect-wrapper",
  "bazel",
  "bazel-module",
  "bazelisk",
  "bicep",
  "bitbucket-pipelines",
  "bitrise",
  "buildkite",
  "buildpacks",
  "bun",
  "bun-version",
  "bundler",
  "cake",
  "cargo",
  "cdnurl",
  "circleci",
  "cloudbuild",
  "cocoapods",
  "composer",
  "conan",
  "copier",
  "cpanfile",
  "crossplane",
  "crow",
  "deno",
  "deps-edn",
  "devbox",
  "devcontainer",
  "docker-compose",
  "dockerfile",
  "droneci",
  "fleet",
  "flux",
  "fvm",
  "git-submodules",
  "github-actions",
  "gitlabci",
  "gitlabci-include",
  "glasskube",
  "gleam",
  "gomod",
  "gradle",
  "gradle-wrapper",
  "haskell-cabal",
  "helm-requirements",
  "helm-values",
  "helmfile",
  "helmsman",
  "helmv3",
  "hermit",
  "homeassistant-manifest",
  "homebrew",
  "html",
  "jenkins",
  "jsonnet-bundler",
  "kotlin-script",
  "kubernetes",
  "kustomize",
  "leiningen",
  "maven",
  "maven-wrapper",
  "meteor",
  "mint",
  "mise",
  "mix",
  "nix",
  "nodenv",
  "npm",
  "nuget",
  "nvm",
  "ocb",
  "osgi",
  "paket",
  "pep621",
  "pep723",
  "pip-compile",
  "pip_requirements",
  "pip_setup",
  "pipenv",
  "pixi",
  "poetry",
  "pre-commit",
  "proto",
  "pub",
  "puppet",
  "pyenv",
  "quadlet",
  "renovate-config",
  "ruby-version",
  "runtime-version",
  "rust-toolchain",
  "sbt",
  "scalafmt",
  "setup-cfg",
  "smithy",
  "sveltos",
  "swift",
  "tekton",
  "terraform",
  "terraform-version",
  "terragrunt",
  "terragrunt-version",
  "tflint-plugin",
  "travis",
  "typst",
  "unity3d",
  "velaci",
  "vendir",
  "woodpecker",
  "xcodegen",
  "jsonata",
  "regex",
  "custom.jsonata",
  "custom.regex"
]

```

| mergeable  | false

| cli  | --enabled-managers

| env  | RENOVATE_ENABLED_MANAGERS

| default  | []

This is a way to allow only certain package managers and implicitly disable all others.

Example:

```
{
  "enabledManagers": ["dockerfile", "npm"]
}

```

To enable custom managers you will need to add custom. prefix before their names

Example:

```
{
  "enabledManagers": ["custom.regex"]
}

```

For the full list of available managers, see the [Supported Managers](https://docs.renovatebot.com/modules/manager/#supported-managers) documentation.

## encrypted[¶](https://docs.renovatebot.com/configuration-options/#encrypted)

An object containing configuration encrypted with project key.

| Name  | Value

| scopes  | any

| type  | object

| additionalProperties  |

```
{"type": "string"}

```

| cli  | --encrypted

| env  | RENOVATE_ENCRYPTED

Before you put any secrets in your repository configuration, encrypt the secrets. You can encrypt secrets using either a HTML page, or the CLI.

To encrypt secrets for the Mend Renovate App for github.com with a HTML page, go to [app.renovatebot.com/encrypt](https://app.renovatebot.com/encrypt) and complete the form. If you're self-hosting Renovate, you may download and edit the form, to use your own PGP public key.

You can also encrypt secrets from the CLI, using the curl, echo, jq, gpg, grep and tr CLI programs. Here is an example:

```
curl https://app.renovatebot.com/renovate.pgp --output renovate.pgp
echo -n '{"o":"your-organization", "r":"your-repository (optional)", "v":"your-secret-value"}' | jq . -c | gpg --encrypt -a --recipient-file renovate.pgp | grep -v '^----' | tr -d '\n'

```

The above script uses:

- curl to download the Mend Renovate hosted app's public key
- echo to echo a JSON object into jq
- jq to validate the JSON and then compact it
- gpg to encrypt the contents
- grep and tr to extract the encrypted payload which we will use

The jq step is optional, you can leave it out if you wish. Its primary value is validating that the string you echo to gpg is valid JSON, and compact.

Note

Encrypted secrets must have at least an org/group scope, and optionally a repository scope. This means that Renovate will check if a secret's scope matches the current repository before applying it, and warn/discard if there is a mismatch.

Encrypted secrets usually have a single organization. But you may encrypt a secret with more than one organization, for example: org1,org2. This way the secret can be used in both the org1 and org2 organizations.

For more information on how to use secrets for private packages, read [Private package support](https://docs.renovatebot.com/getting-started/private-packages/).

## env[¶](https://docs.renovatebot.com/configuration-options/#env)

Environment variables that Renovate uses when executing package manager commands.

| Name  | Value

| scopes  | repo

| type  | object

| additionalProperties  |

```
{"type": "string"}

```

| cli  | --env

| env  | RENOVATE_ENV

This option allows users to specify explicit environment variables values. It is valid only as a top-level configuration option and not, for example, within packageRules.

Warning

The administrator of your Renovate deployment must configure a list of allowed environment names in the [allowedEnv](https://docs.renovatebot.com/self-hosted-configuration/#allowedenv) config option, before users can use those allowed names in the env option.

Behavior:

- This option only applies when Renovate runs package manager commands (e.g. npm install), within the updateArtifacts() function
- Values set in the env configuration override corresponding environment variables, including those from customEnvVariables and process.env

Example renovate.json with env configuration

```
{
  "env": {
    "SOME_ENV_VARIABLE": "SOME_STRING_VALUE"
  }
}

```

## excludeCommitPaths[¶](https://docs.renovatebot.com/configuration-options/#excludecommitpaths)

A file matching any of these glob patterns will not be committed, even if the file has been updated.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| cli  | --exclude-commit-paths

| env  | RENOVATE_EXCLUDE_COMMIT_PATHS

| default  | []

Warning

For advanced use only! Use at your own risk!

Be careful you know what you're doing with this option. The initial intended use is to allow the user to exclude certain dependencies from being added/removed/modified when "vendoring" dependencies. Example:

```
{
  "excludeCommitPaths": ["vendor/golang.org/x/text/**"]
}

```

The above would mean Renovate would not include files matching the above glob pattern in the commit, even if it thinks they should be updated.

## expandCodeOwnersGroups[¶](https://docs.renovatebot.com/configuration-options/#expandcodeownersgroups)

Expand the configured code owner groups into a full list of group members.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| supportedPlatforms  | gitlab

| cli  | --expand-code-owners-groups

| env  | RENOVATE_EXPAND_CODE_OWNERS_GROUPS

If configured, Renovate will expand any matching CODEOWNERS groups into a full list of group members and assign them individually instead of the group. This is particularly useful when combined with assigneesSampleSize and assigneesFromCodeOwners, so that only a subset of the Codeowners are assigned instead of the whole group.

On GitLab, this also expands [role handles](https://docs.gitlab.com/user/project/codeowners/reference/#add-a-role-as-a-code-owner) (@@developer, @@maintainer, @@owner) into the project members who hold that exact role.

## extends[¶](https://docs.renovatebot.com/configuration-options/#extends)

Configuration presets to use or extend.

| Name  | Value

| scopes  | any

| type  | array

| subType  | string

| env  | RENOVATE_EXTENDS

| default  | []

See [shareable config presets](https://docs.renovatebot.com/config-presets/) for details. Learn how to use presets by reading the [Key concepts, Presets](https://docs.renovatebot.com/key-concepts/presets/#how-to-use-presets) page.

## extractVersion[¶](https://docs.renovatebot.com/configuration-options/#extractversion)

A regex (re2) to extract a version from a datasource's raw version string.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | string

| format  | regex

| default  | null

Only use this config option when the raw version strings from the datasource do not match the expected format that you need in your package file. You must define a "named capture group" called version like in the examples below.

extractVersion is only applied to the versions returned by the datasource. It is not applied to the currentValue from your package file, so currentValue must already be in the extracted format.

For example, to extract only the major.minor precision from a GitHub release, the following would work:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["foo"],
      "extractVersion": "^(?<version>v\\d+\\.\\d+)"
    }
  ]
}

```

The above will change a raw version of v1.31.5 to v1.31, for example.

Alternatively, to strip a release- prefix:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["bar"],
      "extractVersion": "^release-(?<version>.*)$"
    }
  ]
}

```

The above will change a raw version of release-2.0.0 to 2.0.0, for example. A similar one could strip leading v prefixes:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["baz"],
      "extractVersion": "^v(?<version>.*)$"
    }
  ]
}

```

## fetchChangeLogs[¶](https://docs.renovatebot.com/configuration-options/#fetchchangelogs)

Controls if and when changelogs/release notes are fetched.

| Name  | Value

| type  | string

| allowedValues  |

```
["off", "branch", "pr"]

```

| default  | "pr"

| parents  | packageRules(the root document)

| env  | RENOVATE_FETCH_CHANGE_LOGS

Use this config option to configure changelogs/release notes fetching. The available options are:

- off - disable changelogs fetching
- branch - fetch changelogs while creating/updating branch
- pr(default) - fetches changelogs while creating/updating pull-request

Avoid setting fetchChangeLogs=branch, because this slows down Renovate. But if you're embedding changelogs in commit information, you may use fetchChangeLogs=branch.

Renovate can fetch changelogs when they are hosted on one of these platforms:

- Bitbucket Cloud
- Bitbucket Server / Data Center
- GitHub (.com and Enterprise Server)
- GitLab (.com and CE/EE)

If you are running on any platform except github.com, you need to [configure a Personal Access Token](https://docs.renovatebot.com/getting-started/running/#githubcom-token-for-changelogs-and-tools) to allow Renovate to fetch changelogs notes from github.com.

You may use [package rules](https://docs.renovatebot.com/configuration-options/#packagerules) to override the value of fetchChangeLogs for matching dependencies, with later rules overriding earlier ones.

The following re-enables fetching of changelogs when creating pull-requests for lodash updates.

```
{
  "fetchChangeLogs": "off",
  "packageRules": [
    {
      "matchSourceUrls": ["https://github.com/lodash/lodash"],
      "fetchChangeLogs": "pr"
    }
  ]
}

```

The following disables fetching of changelogs for any package in aws-sdk-go-v2, which can be time-consuming due to the repository's large number of tags:

```
{
  "packageRules": [
    {
      "matchSourceUrls": ["https://github.com/aws/aws-sdk-go-v2{/**,}"],
      "fetchChangeLogs": "off"
    }
  ]
}

```

Note

Renovate can only show changelogs from some platforms and some package managers. We're planning improvements so that Renovate can show more changelogs. Read [issue 14138 on GitHub](https://github.com/renovatebot/renovate/issues/14138) to get an overview of the planned work.

## filterUnavailableUsers[¶](https://docs.renovatebot.com/configuration-options/#filterunavailableusers)

Filter reviewers and assignees based on their availability.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| supportedPlatforms  | gitlab

| cli  | --filter-unavailable-users

| env  | RENOVATE_FILTER_UNAVAILABLE_USERS

When this option is enabled PRs are not assigned to users that are unavailable. This option only works on platforms that support the concept of user availability. For now, you can only use this option on the GitLab platform.

## followTag[¶](https://docs.renovatebot.com/configuration-options/#followtag)

If defined, packages will follow this release tag exactly.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | string

| default  | null

Warning

For advanced use only! Use at your own risk!

For followTag to work, the datasource must support distribution streams or tags, like for example npm does.

The main use case is to follow a pre-release tag of a dependency, say TypeScripts's "insiders" build:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["typescript"],
      "followTag": "insiders"
    }
  ]
}

```

If you've set a followTag then Renovate skips its normal major/minor/patch upgrade logic and stable/unstable consistency logic, and instead keeps your dependency version synced strictly to the version in the tag.

Renovate follows tags strictly, this can cause problems when a tagged stream is no longer maintained. For example: you're following the next tag, but later the stream you actually want is called stable instead. If next is no longer getting updates, you must switch your followTag to stable to get updates again.

## forkModeDisallowMaintainerEdits[¶](https://docs.renovatebot.com/configuration-options/#forkmodedisallowmaintaineredits)

Disallow maintainers to push to Renovate pull requests when running in fork mode.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| supportedPlatforms  | github

| cli  | --fork-mode-disallow-maintainer-edits

| env  | RENOVATE_FORK_MODE_DISALLOW_MAINTAINER_EDITS

Use forkModeDisallowMaintainerEdits to disallow maintainers from editing Renovate's pull requests when in fork mode.

If GitHub pull requests are created from a [fork repository](https://docs.github.com/en/get-started/quickstart/fork-a-repo), the PR author can decide to allow upstream repository to modify the PR directly.

Allowing maintainers to edit pull requests directly is helpful when Renovate pull requests require more changes. The reviewer can simply push to the pull request without having to create a new PR. [More details here](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/allowing-changes-to-a-pull-request-branch-created-from-a-fork).

You may decide to disallow edits to Renovate pull requests in order to workaround issues in Renovate where modified fork branches are not deleted properly: [See this issue](https://github.com/renovatebot/renovate/issues/16657). If this option is enabled, reviewers will need to create a new PR if more changes are needed.

Note

This option is only relevant if you set forkToken.

## forkProcessing[¶](https://docs.renovatebot.com/configuration-options/#forkprocessing)

Whether to process forked repositories. By default, all forked repositories are skipped when in autodiscover mode.

| Name  | Value

| scopes  | repo

| type  | string

| allowedValues  |

```
["auto", "enabled", "disabled"]

```

| default  | "auto"

| cli  | --fork-processing

| env  | RENOVATE_FORK_PROCESSING

By default, Renovate skips any forked repositories when in autodiscover mode. It even skips a forked repository that has a Renovate configuration file, because Renovate doesn't know if that file was added by the forked repository.

Process a fork in autodiscover mode

If you want Renovate to run on a forked repository when in autodiscover mode then:

- Ensure a renovate.json config exists with "forkProcessing": "enabled" in your repository,
- Or run the CLI command with --fork-processing=enabled

Process a fork in other modes

If you're running Renovate in some other mode, for example when giving a list of repositories to Renovate, but want to skip forked repositories: set "forkProcessing": "disabled" in your global config.

When using the Mend Renovate App

The behavior of forkProcessing depends on how you allow Renovate to run on your account.

Renovate runs on all repositories

If you allow Renovate to run on all your repositories, forkProcessing will be "disabled". To run Renovate on a fork: add "forkProcessing": "enabled" to the forked repository's renovate.json file.

Renovate runs on selected repositories

If you allow Renovate to run on "Selected" repositories, forkProcessing will be "enabled" for each "Selected" repository.

Allowed filenames

Only the onboardingConfigFileName (which defaults to renovate.json) is supported for forkProcessing. You can't use other filenames because Renovate only checks the default filename when using the Git-hosting platform's API.

## gitAuthor[¶](https://docs.renovatebot.com/configuration-options/#gitauthor)

Author to use for Git commits. Must conform to [RFC5322](https://datatracker.ietf.org/doc/html/rfc5322).

| Name  | Value

| scopes  | repo

| type  | string

| supportsTemplating  | true

| cli  | --git-author

| env  | RENOVATE_GIT_AUTHOR

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

You can customize the Git author that's used whenever Renovate creates a commit, although we do not recommend this. When this field is unset (default), Renovate will use its platform credentials (e.g. token) to learn/discover its account's git author automatically.

Note

If running as a GitHub App and using platformCommit, GitHub itself sets the git author in commits so you should not configure this field.

The gitAuthor option accepts a [RFC5322](https://datatracker.ietf.org/doc/html/rfc5322)-compliant string. It's recommended to include a name followed by an email address, e.g.

```
Development Bot <dev-bot@my-software-company.com>

```

Danger

We strongly recommend that the Git author email you use is unique to Renovate. Otherwise, if another bot or human shares the same email and pushes to one of Renovate's branches then Renovate will mistake the branch as unmodified and potentially force push over the changes.

## gitIgnoredAuthors[¶](https://docs.renovatebot.com/configuration-options/#gitignoredauthors)

Git author emails ignored by Renovate. Entries can be exact [RFC5322](https://datatracker.ietf.org/doc/html/rfc5322) strings, glob patterns, or regex patterns using Renovate regex syntax.

| Name  | Value

| scopes  | repo

| type  | array

| subType  | string

| cli  | --git-ignored-authors

| env  | RENOVATE_GIT_IGNORED_AUTHORS

| default  | []

Specify commit author emails ignored by Renovate.

By default, Renovate will treat any PR as modified if another Git author has added to the branch. When a PR is considered modified, Renovate won't perform any further commits such as if it's conflicted or needs a version update. If you have other bots which commit on top of Renovate PRs, and don't want Renovate to treat these PRs as modified, then add the other Git author(s) to gitIgnoredAuthors.

```
{
  "gitIgnoredAuthors": ["some-bot@example.org"]
}

```

gitIgnoredAuthors values can be exact [RFC5322](https://datatracker.ietf.org/doc/html/rfc5322)-compliant email strings, glob patterns, or regex patterns. For more details on the syntax and supported patterns, see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/).

## gitLabIgnoreApprovals[¶](https://docs.renovatebot.com/configuration-options/#gitlabignoreapprovals)

Ignore approval rules for MRs created by Renovate, which is useful for automerge.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --git-lab-ignore-approvals

| env  | RENOVATE_GIT_LAB_IGNORE_APPROVALS

Ignore the default project level approval(s), so that Renovate can automerge its merge requests, without needing approval(s). Under the hood, it creates a MR-level approval rule where approvals_required is set to 0. This option works only when automerge=true and either automergeType=pr or automergeType=branch. Also, approval rules overriding should not be [prevented in GitLab settings](https://docs.gitlab.com/ee/user/project/merge_requests/approvals/settings.html#prevent-editing-approval-rules-in-merge-requests).

## goGetDirs[¶](https://docs.renovatebot.com/configuration-options/#gogetdirs)

Directory pattern to run go get on.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| default  |

```
["./..."]

```

| supportedManagers  | gomod

| cli  | --go-get-dirs

| env  | RENOVATE_GO_GET_DIRS

By default, Renovate will run go get -d -t ./... to update the go.sum. If you need to modify this path, for example in order to ignore directories, you can override the default ./... value using this option:

```
{
  "goGetDirs": ["./some-project/", "./tools/..."]
}

```

## group[¶](https://docs.renovatebot.com/configuration-options/#group)

Config if groupName is enabled.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | object

| default  |

```
{"branchTopic": "{{{groupSlug}}}", "commitMessageTopic": "{{{groupName}}}"}

```

| mergeable  | true

Warning

For advanced use only! Use at your own risk!

The default configuration for groups are essentially internal to Renovate and you normally shouldn't need to modify them. But you may add settings to any group by defining your own group configuration object.

## groupName[¶](https://docs.renovatebot.com/configuration-options/#groupname)

Human understandable name for the dependency group.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| supportsTemplating  | true

| cli  | --group-name

| env  | RENOVATE_GROUP_NAME

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

There are multiple cases where it can be useful to group multiple upgrades together. Internally Renovate uses this for branches such as "Pin Dependencies", "Lock File Maintenance", etc. Another example used previously is to group together all related eslint packages, or perhaps angular or babel. To enable grouping, you configure the groupName field to something non-null.

The groupName field allows free text and does not have any semantic interpretation by Renovate. All updates sharing the same groupName will be placed into the same branch/PR. For example, to group all non-major devDependencies updates together into a single PR:

```
{
  "packageRules": [
    {
      "matchDepTypes": ["devDependencies"],
      "matchUpdateTypes": ["patch", "minor"],
      "groupName": "devDependencies (non-major)"
    }
  ]
}

```

Note

Replacement updates will never be grouped.
Lock file maintenance will never be grouped with other dependency updates.

## groupSingleUpdates[¶](https://docs.renovatebot.com/configuration-options/#groupsingleupdates)

Apply group settings even when the group contains only one update.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

Note

This option was [recently made opt-in, instead of opt-out](https://github.com/renovatebot/renovate/pull/44168).

## groupSlug[¶](https://docs.renovatebot.com/configuration-options/#groupslug)

Slug to use for group (e.g. in branch name). Slug is calculated from groupName if null.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| supportsTemplating  | true

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

By default, Renovate will "slugify" the groupName to determine the branch name. For example if you named your group "devDependencies (non-major)" then the branchName would be renovate/devdependencies-non-major. If you wished to override this then you could configure like this:

```
{
  "packageRules": [
    {
      "matchDepTypes": ["devDependencies"],
      "matchUpdateTypes": ["patch", "minor"],
      "groupName": "devDependencies (non-major)",
      "groupSlug": "dev-dependencies"
    }
  ]
}

```

As a result of the above, the branchName would be renovate/dev-dependencies instead.

Note

You shouldn't usually need to configure this unless you really care about your branch names.

## hashedBranchLength[¶](https://docs.renovatebot.com/configuration-options/#hashedbranchlength)

If enabled, branch names will use a hashing function to ensure each branch has that length.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| env  | RENOVATE_HASHED_BRANCH_LENGTH

Some code hosting systems have restrictions on the branch name lengths, this option lets you get around these restrictions. You can set the hashedBranchLength option to a number of characters that works for your system and then Renovate will generate branch names with the correct length by hashing additionalBranchPrefix and branchTopic, and then truncating the hash so that the full branch name (including branchPrefix) has the right number of characters.

Example: If you have set branchPrefix: "deps-" and hashedBranchLength: 12 it will result in a branch name like deps-5bf36ec instead of the traditional pretty branch name like deps-react-17.x.

## hostRules[¶](https://docs.renovatebot.com/configuration-options/#hostrules)

Host rules/configuration including credentials.

| Name  | Value

| scopes  | repo

| type  | array

| subType  | object

| cli  | --host-rules

| mergeable  | true

| env  | RENOVATE_HOST_RULES

| previouslyKnownAs  |

```
["endpoints"]

```

| default  | []

The primary purpose of hostRules is to configure credentials for host authentication. You tell Renovate how to match against the host you need authenticated, and then you also tell it which credentials to use.

The lookup keys for hostRules are: hostType and matchHost, both of which are optional.

Supported credential fields are token, username, password, timeout, enabled and insecureRegistry.

Example for configuring docker auth:

```
{
  "hostRules": [
    {
      "matchHost": "docker.io",
      "username": "<some-username>",
      "password": "<some-password>"
    }
  ]
}

```

If multiple hostRules match a request, then they will be applied in the following order/priority:

- rules with only hostType specified
- rules with only matchHost specified (sorted by matchHost length if multiple match)
- rules with both matchHost and hostType specified (sorted by matchHost length if multiple match)

To disable requests to a particular host, you can configure a rule like:

```
{
  "hostRules": [
    {
      "matchHost": "registry.npmjs.org",
      "enabled": false
    }
  ]
}

```

A preset alternative to the above is:

```
{
  "extends": [":disableHost(registry.npmjs.org)"]
}

```

To match a specific port, add the port to matchHost:

```
{
  "hostRules": [
    {
      "matchHost": "https://domain.com:9118",
      "enabled": false
    }
  ]
}

```

Note

A matchHost with a port or a path but no scheme, like domain.com:9118 or domain.com/path, is treated as https://domain.com:9118 or https://domain.com/path. To match a port over plain http, include the scheme.

Note

Disabling a host is only 100% effective if added to self-hosted config. Renovate currently still checks its cache for results first before trying to connect, so if a public host is blocked in your repository config (e.g. renovate.json) then it's possible you may get cached results from that host if another repository using the same Renovate deployment has successfully queried for the same dependency recently.

Note

enabled is not resolved by specificity alone: host rules from the self-hosted administrator's own config are resolved at a higher trust level than those from repository config or a preset. So if the administrator's own rules set enabled for a host, repository config or a preset cannot re-enable - or disable - that host, no matter how specific its matchHost is.

### hostRules.abortIgnoreStatusCodes[¶](https://docs.renovatebot.com/configuration-options/#hostrulesabortignorestatuscodes)

A list of HTTP status codes safe to ignore even when abortOnError=true.

| Name  | Value

| type  | array

| subType  | number

| parents  | hostRules

| default  | []

This field can be used to configure status codes that Renovate ignores and passes through when abortOnError is set to true. For example to also skip 404 responses then configure the following:

```
{
  "hostRules": [
    {
      "abortOnError": true,
      "abortIgnoreStatusCodes": [404]
    }
  ]
}

```

Tip

This field is not mergeable, so the last-applied host rule takes precedence.

### hostRules.abortOnError[¶](https://docs.renovatebot.com/configuration-options/#hostrulesabortonerror)

If enabled, Renovate aborts its run when HTTP request errors occur.

| Name  | Value

| type  | boolean

| default  | false

| parents  | hostRules

Use this field to configure Renovate to abort runs for custom hosts. By default, Renovate will only abort for known public hosts, which has the downside that transient errors for other hosts can cause autoclosing of PRs.

To abort Renovate runs for HTTP failures from any host:

```
{
  "hostRules": [
    {
      "abortOnError": true
    }
  ]
}

```

To abort Renovate runs for any docker datasource failures:

```
{
  "hostRules": [
    {
      "hostType": "docker",
      "abortOnError": true
    }
  ]
}

```

To abort Renovate for errors for a specific docker host:

```
{
  "hostRules": [
    {
      "matchHost": "docker.company.com",
      "abortOnError": true
    }
  ]
}

```

When this field is enabled, Renovate will abort its run if it encounters either (a) any low-level http error (e.g. ETIMEDOUT) or (b) gets a response not matching any of the configured abortIgnoreStatusCodes (e.g. 500 Internal Error);

### hostRules.artifactAuth[¶](https://docs.renovatebot.com/configuration-options/#hostrulesartifactauth)

A list of package managers to enable artifact auth. Only managers on the list are enabled. All are enabled if null.

| Name  | Value

| type  | array

| subType  | string

| parents  | hostRules

| allowedValues  |

```
["composer"]

```

| default  | []

This feature is flagged as experimental

Experimental features might be changed or even removed at any time.

You may use this field whenever it is needed to only enable authentication for a specific set of managers.

For example, using this option could be used whenever authentication using Git for private composer packages is already being handled through the use of SSH keys, which results in no need for also setting up authentication using tokens.

```
{
  "hostRules": [
    {
      "hostType": "gitlab",
      "matchHost": "gitlab.myorg.com",
      "token": "abc123",
      "artifactAuth": ["composer"]
    }
  ]
}

```

Supported artifactAuth and hostType combinations:

| artifactAuth  | hostTypes

| composer  | gitlab, packagist, github, git-tags

### hostRules.authType[¶](https://docs.renovatebot.com/configuration-options/#hostrulesauthtype)

Authentication type for HTTP header. e.g. "Bearer" or "Basic". Use "Token-Only" to use only the token without an authorization type.

| Name  | Value

| type  | string

| parents  | hostRules

| default  | "Bearer"

You may use the authType option to create a custom HTTP authorization header. For authType to work, you must also set your own token.

Do not set authType=Bearer: it's the default setting for Renovate anyway. Do not set a username or password when you're using authType, as authType doesn't use usernames or passwords.

An example for npm basic auth with token:

```
{
  "hostRules": [
    {
      "matchHost": "npm.custom.org",
      "token": "<some-token>",
      "authType": "Basic"
    }
  ]
}

```

This will generate the following header: authorization: Basic <some-token>.

To use a bare token in the authorization header (required by e.g. Hex) - use the authType "Token-Only":

```
{
  "hostRules": [
    {
      "matchHost": "https://hex.pm/api/repos/private_repo/",
      "token": "<some-token>",
      "authType": "Token-Only"
    }
  ]
}

```

This will generate the header authorization: <some-token>.

### hostRules.concurrentRequestLimit[¶](https://docs.renovatebot.com/configuration-options/#hostrulesconcurrentrequestlimit)

Limit concurrent requests per host.

| Name  | Value

| type  | integer

| parents  | hostRules

Usually the default setting is fine, but you can use concurrentRequestLimit to limit the number of concurrent outstanding requests. You only need to adjust this setting if a datasource is rate limiting Renovate or has problems with the load. The limit will be set for any host it applies to.

Example config:

```
{
  "hostRules": [
    {
      "matchHost": "api.github.com",
      "concurrentRequestLimit": 2
    }
  ]
}

```

Use an exact host for matchHost and not a domain (e.g. api.github.com as shown above and not github.com). Do not combine with hostType in the same rule or it won't work.

### hostRules.dnsCache[¶](https://docs.renovatebot.com/configuration-options/#hostrulesdnscache)

Enable got DNS cache.

| Name  | Value

| type  | boolean

| default  | false

| parents  | hostRules

This feature is flagged as experimental

Experimental features might be changed or even removed at any time.

This feature has been deprecated

This option is deprecated and will be removed in a future release.

This option is deprecated and has no effect. Renovate removed its DNS cache in v38.0.0.

### hostRules.enableHttp2[¶](https://docs.renovatebot.com/configuration-options/#hostrulesenablehttp2)

Enable got HTTP/2 support.

| Name  | Value

| type  | boolean

| default  | false

| parents  | hostRules

Enable got [http2](https://github.com/sindresorhus/got/blob/v11.5.2/readme.md#http2) support.

### hostRules.headers[¶](https://docs.renovatebot.com/configuration-options/#hostrulesheaders)

Put fields to be forwarded to the HTTP request headers in the headers config option.

| Name  | Value

| type  | object

| additionalProperties  |

```
{"type": "string"}

```

| parents  | hostRules

Warning

For advanced use only! Use at your own risk!

You can provide a headers object that includes fields to be forwarded to the HTTP request headers. By default, all headers starting with "X-" are allowed.

A self-hosted administrator may configure an override for [allowedHeaders](https://docs.renovatebot.com/self-hosted-configuration/#allowedheaders) to configure more permitted headers.

headers you configure in a repository config file, or a preset it extends, are checked against allowedHeaders. Any header which is not permitted is dropped, and a warning is logged.

allowedHeaders does not constrain the self-hosted administrator's own hostRules (for example in a config.js file, or a repositories[] entry): their headers are always applied, regardless of allowedHeaders.

When more than one of your host rules matches a request, the headers of the most specific matching rule are used, and replace the headers of the broader rules it matched alongside.

A self-hosted administrator's own host rules are resolved the same way, and whichever headers that leaves them sending to the host are then applied on top of yours:

- your host rules cannot stop one of those headers from being sent, and
- where one of those headers has a name you also set, the administrator's value is used.

This covers the headers the administrator actually sends to that host. One they set in a broader rule of their own, but masked with a narrower rule of their own, is not sent there - and so is yours to set.

For example:

```
{
  "hostRules": [
    {
      "matchHost": "https://domain.com/all-versions",
      "headers": {
        "X-custom-header": "secret"
      }
    }
  ]
}

```

### hostRules.hostType[¶](https://docs.renovatebot.com/configuration-options/#hostruleshosttype)

hostType for a package rule. Can be a platform name or a datasource name.

| Name  | Value

| type  | string

| parents  | hostRules

| default  | null

hostType is another way to filter rules and can be either a platform such as github and bitbucket-server, or it can be a datasource such as docker and rubygems. You usually don't need to configure it in a host rule if you have already configured matchHost and only one host type is in use for those, as is usually the case. hostType can help for cases like an enterprise registry that serves multiple package types and has different authentication for each, although it's often the case that multiple matchHost rules could achieve the same thing.

### hostRules.httpsCertificate[¶](https://docs.renovatebot.com/configuration-options/#hostruleshttpscertificate)

The certificate chains in PEM format.

| Name  | Value

| type  | string

| parents  | hostRules

Specifies the [Certificate chains](https://en.wikipedia.org/wiki/X.509#Certificate_chains_and_cross-certification) in [PEM format](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) for mTLS authentication.

### hostRules.httpsCertificateAuthority[¶](https://docs.renovatebot.com/configuration-options/#hostruleshttpscertificateauthority)

The overriding trusted CA certificate.

| Name  | Value

| type  | string

| parents  | hostRules

By default, Renovate uses the curated list of well-known [CA](https://en.wikipedia.org/wiki/Certificate_authority)s by Mozilla. You may use another Certificate Authority instead, by setting it in the httpsCertificateAuthority config option.

### hostRules.httpsPrivateKey[¶](https://docs.renovatebot.com/configuration-options/#hostruleshttpsprivatekey)

The private key in PEM format.

| Name  | Value

| type  | string

| parents  | hostRules

Specifies the private key in [PEM format](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) for mTLS authentication.

Warning

Do not put your private key into this field, to avoid losing confidentiality completely. You must use [secrets](https://docs.renovatebot.com/self-hosted-configuration/#secrets) to pass it down securely instead.

### hostRules.insecureRegistry[¶](https://docs.renovatebot.com/configuration-options/#hostrulesinsecureregistry)

Explicitly turn on insecure Docker registry access (HTTP).

| Name  | Value

| type  | boolean

| default  | false

| parents  | hostRules

Warning

For advanced use only! Use at your own risk!

Enable this option to allow Renovate to connect to an [insecure Docker registry](https://docs.docker.com/registry/insecure/) that is HTTP only. This is insecure and is not recommended.

Example:

```
{
  "hostRules": [
    {
      "matchHost": "reg.insecure.com",
      "insecureRegistry": true
    }
  ]
}

```

### hostRules.keepAlive[¶](https://docs.renovatebot.com/configuration-options/#hostruleskeepalive)

Enable HTTP keep-alive for hosts.

| Name  | Value

| type  | boolean

| default  | false

| parents  | hostRules

| previouslyKnownAs  |

```
["keepalive"]

```

Warning

For advanced use only! Use at your own risk!

If enabled, this allows a single TCP connection to remain open for multiple HTTP(S) requests/responses.

### hostRules.matchHost[¶](https://docs.renovatebot.com/configuration-options/#hostrulesmatchhost)

A domain name, host name or base URL to match against.

| Name  | Value

| type  | string

| parents  | hostRules

| default  | null

This can be a base URL (e.g. https://api.github.com) or a hostname like github.com or api.github.com. If the value starts with http(s) then it will only match against URLs which start with the full base URL. Otherwise, it will be matched by checking if the URL's hostname matches the matchHost directly or ends with it. When checking the end of the hostname, a single dot is prefixed to the value of matchHost, if one is not already present, to ensure it can only match against whole domain segments.

A base URL ending in / (e.g. https://gitlab.com/api/v4/projects/1/) matches that path and everything below it, also when the URL has no trailing /, but not a sibling path like https://gitlab.com/api/v4/projects/10.

The matchHost URL must be the same as the registryUrl set in .npmrc, or you'll get authentication issues when the artifacts are updated when yarn or npm runs.

```
{
  "hostRules": [
    {
      "matchHost": "https://gitlab.myorg.com/api/v4/packages/npm/",
      "token": "abc123"
    }
  ]
}

```

The above corresponds with an .npmrc like the following:

```
registry=https://gitlab.myorg.com/api/v4/packages/npm/

```

Note

Values containing a URL path or a port but missing a scheme will be prepended with https:// (e.g. domain.com/path → https://domain.com/path, domain.com:9118 → https://domain.com:9118).

### hostRules.maxRequestsPerSecond[¶](https://docs.renovatebot.com/configuration-options/#hostrulesmaxrequestspersecond)

Limit requests rate per host.

| Name  | Value

| type  | integer

| parents  | hostRules

| default  | 0

In addition to concurrentRequestLimit, you can limit the maximum number of requests that can be made per one second. It can be used to set minimal delay between two requests to the same host. Fractional values are allowed, e.g. 0.25 means 1 request per 4 seconds. Default value 0 means no limit.

Example config:

```
{
  "hostRules": [
    {
      "matchHost": "api.github.com",
      "maxRequestsPerSecond": 2
    }
  ]
}

```

### hostRules.maxRetryAfter[¶](https://docs.renovatebot.com/configuration-options/#hostrulesmaxretryafter)

Maximum retry-after header value to wait for before retrying a failed request.

| Name  | Value

| type  | integer

| default  | 60

| parents  | hostRules

A remote host may return a 4xx response with a Retry-After header value, which indicates that Renovate has been rate-limited. Renovate may try to contact the host again after waiting a certain time, that's set by the host. By default, Renovate tries again after the Retry-After header value has passed, up to a maximum of 60 seconds. If the Retry-After value is more than 60 seconds, Renovate will abort the request instead of waiting.

You can configure a different maximum value in seconds using maxRetryAfter:

```
{
  "hostRules": [
    {
      "matchHost": "api.github.com",
      "maxRetryAfter": 25
    }
  ]
}

```

### hostRules.readOnly[¶](https://docs.renovatebot.com/configuration-options/#hostrulesreadonly)

Match against requests that only read data and do not mutate anything.

| Name  | Value

| type  | boolean

| parents  | hostRules

| default  | true

If the readOnly field is being set to true inside the host rule, it will match only against the requests that are known to be read operations. Examples are GET requests or HEAD requests, but also it could be certain types of GraphQL queries.

This option could be used to avoid rate limits for certain platforms like GitHub or Bitbucket, by offloading the read operations to a different user.

```
{
  "hostRules": [
    {
      "matchHost": "api.github.com",
      "readOnly": true,
      "token": "********"
    }
  ]
}

```

If more than one token matches for a read-only request then the readOnly token will be given preference.

### hostRules.timeout[¶](https://docs.renovatebot.com/configuration-options/#hostrulestimeout)

Timeout (in milliseconds) for queries to external endpoints.

| Name  | Value

| type  | integer

| parents  | hostRules

Use this figure to adjust the timeout for queries. The default is 60s, which is quite high. To adjust it down to 10s for all queries, do this:

```
{
  "hostRules": [
    {
      "timeout": 10000
    }
  ]
}

```

## ignoreDeprecated[¶](https://docs.renovatebot.com/configuration-options/#ignoredeprecated)

Avoid upgrading from a non-deprecated version to a deprecated one.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | true

| cli  | --ignore-deprecated

| env  | RENOVATE_IGNORE_DEPRECATED

By default, Renovate won't update a dependency version to a deprecated release unless the current version was itself deprecated. The goal of this is to make sure you don't upgrade from a non-deprecated version to a deprecated one, only because it's higher than the current version.

If for some reason you wish to force deprecated updates with Renovate, you can configure ignoreDeprecated to false, which we do not recommend for most situations.

## ignoreDeps[¶](https://docs.renovatebot.com/configuration-options/#ignoredeps)

Dependencies to ignore.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | array

| subType  | string

| mergeable  | true

| cli  | --ignore-deps

| env  | RENOVATE_IGNORE_DEPS

| default  | []

The ignoreDeps configuration field allows you to define a list of dependency names to be ignored by Renovate. Currently it supports only "exact match" dependency names and not any patterns. e.g. to ignore both eslint and eslint-config-base you would add this to your config:

```
{
  "ignoreDeps": ["eslint", "eslint-config-base"]
}

```

The above is the same as if you wrote this package rule:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["eslint", "eslint-config-base"],
      "enabled": false
    }
  ]
}

```

## ignorePaths[¶](https://docs.renovatebot.com/configuration-options/#ignorepaths)

Skip any package file whose path matches one of these. Can be a string or glob pattern.

| Name  | Value

| scopes  | repo, manager

| type  | array

| mergeable  | false

| subType  | string

| default  |

```
["**/node_modules/**", "**/bower_components/**"]

```

| cli  | --ignore-paths

| env  | RENOVATE_IGNORE_PATHS

Renovate will extract dependencies from every file it finds in a repository, unless that file is explicitly ignored. With this setting you can selectively ignore package files that would normally be "autodiscovered" and updated by Renovate.

For instance if you have a project with an "examples/" directory you wish to ignore:

```
{
  "ignorePaths": ["**/examples/**"]
}

```

Renovate's default ignore is node_modules and bower_components only. If you are extending from the popular config:recommended preset then it adds ignore patterns for vendor, examples, test(s) and fixtures directories too.

## ignorePlugins[¶](https://docs.renovatebot.com/configuration-options/#ignoreplugins)

Set this to true if allowPlugins=true but you wish to skip running plugins when updating lock files.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --ignore-plugins

| env  | RENOVATE_IGNORE_PLUGINS

Set this to true if running plugins causes problems. Applicable for Composer only for now.

## ignorePresets[¶](https://docs.renovatebot.com/configuration-options/#ignorepresets)

A list of presets to ignore, including any that are nested inside an extends array.

| Name  | Value

| scopes  | any

| type  | array

| subType  | string

| env  | RENOVATE_IGNORE_PRESETS

| default  | []

Use this if you are extending a complex preset but don't want to use every "sub preset" that it includes. For example, consider this config:

```
{
  "extends": ["config:recommended"],
  "ignorePresets": ["group:monorepos"]
}

```

It would take the entire "config:recommended" preset - which has a lot of sub-presets - but ignore the "group:monorepos" rule.

## ignoreReviewers[¶](https://docs.renovatebot.com/configuration-options/#ignorereviewers)

Reviewers to be ignored in PR reviewers presence (either username or email address depending on the platform).

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| cli  | --ignore-reviewers

| env  | RENOVATE_IGNORE_REVIEWERS

| default  | []

By default, Renovate does not add assignees or reviewers to PRs which are configured for automerge. If tests have failed, Renovate then does add them, but only if the assignees and reviewers list is empty. In the case that a user is automatically added as reviewer (such as Renovate Approve bot) and you want to ignore it for the purpose of this decision, add it to the ignoreReviewers list.

```
{
  "reviewers": ["foo"],
  "ignoreReviewers": ["renovate-approve"]
}

```

Please note that Reviewers are only added during creation of a PR, but are not modified afterwards.

## ignoreScripts[¶](https://docs.renovatebot.com/configuration-options/#ignorescripts)

Set this to false if allowScripts=true and you wish to run scripts when updating lock files.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | true

| supportedManagers  | npm, bun, composer, copier

| cli  | --ignore-scripts

| env  | RENOVATE_IGNORE_SCRIPTS

By default, Renovate will disable package manager scripts. Allowing packager manager scripts is a risk:

- Untrusted or compromised repository users could use package manager scripts to exploit the system where Renovate runs, and
- Malicious package authors could use scripts to exploit a repository and Renovate system, for example to exfiltrate source code and secrets

No script execution on free Mend-hosted Renovate

The Mend Renovate App does not allow scripts to run. We do not plan to let users on free tiers run scripts, because the risk of abuse is too high.

Renovate Enterprise Cloud can be configured to run scripts

Scripts can be enabled for paying customers on Mend.io hosted apps. Please ask Mend.io sales about "Renovate Enterprise Cloud".

Allowing scripts if self-hosting Renovate

If you are self-hosting Renovate, and want to allow Renovate to run any scripts:

- Set the self-hosted config option [allowScripts](https://docs.renovatebot.com/self-hosted-configuration/#allowscripts) to true in your admin configuration
- Set ignoreScripts to false for the package managers you want to allow to run scripts (only works for the supportedManagers listed in the table above)

## ignoreTests[¶](https://docs.renovatebot.com/configuration-options/#ignoretests)

Set to true to enable automerging without tests.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --ignore-tests

| env  | RENOVATE_IGNORE_TESTS

Currently Renovate's default behavior is to only automerge if every status check has succeeded.

Setting this option to true means that Renovate will ignore all status checks. You can set this if you don't have any status checks but still want Renovate to automerge PRs. Beware: configuring Renovate to automerge without any tests can lead to broken builds on your base branch, please think again before enabling this!

## ignoreUnstable[¶](https://docs.renovatebot.com/configuration-options/#ignoreunstable)

Ignore versions with unstable SemVer.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | true

| cli  | --ignore-unstable

| env  | RENOVATE_IGNORE_UNSTABLE

By default, Renovate won't update any package versions to unstable versions (e.g. 4.0.0-rc3) unless the current version has the same major.minor.patch and was already unstable (e.g. it was already on 4.0.0-rc2). Renovate will also not "jump" unstable versions automatically, e.g. if you are on 4.0.0-rc2 and newer versions 4.0.0 and 4.1.0-alpha.1 exist then Renovate will update you to 4.0.0 only. If you need to force permanent unstable updates for a package, you can add a package rule setting ignoreUnstable to false. In that case you will usually also want to set respectLatest to false so that Renovate considers versions ahead of latest.

Also check out the followTag configuration option above if you wish Renovate to keep you pinned to a particular release tag.

## includePaths[¶](https://docs.renovatebot.com/configuration-options/#includepaths)

Include package files only within these defined paths.

| Name  | Value

| scopes  | repo, manager

| type  | array

| subType  | string

| cli  | --include-paths

| env  | RENOVATE_INCLUDE_PATHS

| default  | []

If you wish for Renovate to process only select paths in the repository, use includePaths.

Alternatively, if you need to exclude certain paths in the repository then consider ignorePaths instead. If you are more interested in including only certain package managers (e.g. npm), then consider enabledManagers instead.

## internalChecksAsSuccess[¶](https://docs.renovatebot.com/configuration-options/#internalchecksassuccess)

Whether to consider passing internal checks such as minimumReleaseAge when determining branch status.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --internal-checks-as-success

| env  | RENOVATE_INTERNAL_CHECKS_AS_SUCCESS

By default, internal Renovate checks such as renovate/stability-days are not counted towards a branch being "green" or not. This is primarily to prevent automerge when the only check is a passing Renovate check.

Internal checks will always be counted/considered if they are in pending or failed states. If there are multiple passing checks for a branch, including non-Renovate ones, then this setting won't make any difference.

Change this setting to true if you want to use internal Renovate checks towards a passing branch result.

## internalChecksFilter[¶](https://docs.renovatebot.com/configuration-options/#internalchecksfilter)

When and how to filter based on internal checks.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert, updateType

| type  | string

| allowedValues  |

```
["strict", "flexible", "none"]

```

| default  | "strict"

| cli  | --internal-checks-filter

| env  | RENOVATE_INTERNAL_CHECKS_FILTER

This setting determines whether Renovate controls when and how filtering of internal checks are performed, particularly when multiple versions of the same update type are available. Currently this applies to the minimumReleaseAge check only.

- none: No filtering will be performed, and the highest release will be used regardless of whether it's pending or not
- strict: All pending releases will be filtered. PRs will be skipped unless a non-pending version is available
- flexible: Similar to strict, but in the case where all versions are pending then a PR will be created with the highest pending version

The flexible mode can result in "flapping" of Pull Requests, for example: a pending PR with version 1.0.3 is first released but then downgraded to 1.0.2 once it passes minimumReleaseAge. We recommend that you use the strict mode, and enable the dependencyDashboard so that you can see suppressed PRs.

## keepUpdatedLabel[¶](https://docs.renovatebot.com/configuration-options/#keepupdatedlabel)

If set, users can add this label to PRs to request they be kept updated with the base branch.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| supportedPlatforms  | azure, forgejo, gerrit, gitea, github, gitlab

| cli  | --keep-updated-label

| env  | RENOVATE_KEEP_UPDATED_LABEL

| default  | null

On supported platforms you may add a label to a PR so that Renovate recreates/rebases the PR when the branch falls behind the base branch. Adding the keepUpdatedLabel label to a PR makes Renovate behave as if rebaseWhen were set to behind-base-branch, but only for the given PR. Renovate does not remove the label from the PR after it finishes rebasing. This is different from the rebaseLabel option, where Renovate removes the label from the PR after rebasing.

keepUpdatedLabel can be useful when you have approved certain PRs and want Renovate to keep the PRs up-to-date until you're ready to merge them. The setting keepUpdatedLabel is best used in this scenario:

- By default, you configure rebaseWhen to never or conflicted to reduce rebasing
- Sometimes, you want Renovate to keep specific PRs up-to-date with their base branch (equivalent to rebaseWhen=behind-base-branch)

## labels[¶](https://docs.renovatebot.com/configuration-options/#labels)

Labels to set in Pull Request.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| supportsTemplating  | true

| cli  | --labels

| env  | RENOVATE_LABELS

| default  | []

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

By default, Renovate won't add any labels to PRs. If you want Renovate to add labels to PRs it creates then define a labels array of one or more label strings. If you want the same label(s) for every PR then you can configure it at the top level of config. However you can also fully override them on a per-package basis.

Consider this example:

```
{
  "labels": ["dependencies"],
  "packageRules": [
    {
      "matchPackageNames": ["/eslint/"],
      "labels": ["linting"]
    }
  ]
}

```

With the above config, every PR raised by Renovate will have the label dependencies while PRs containing eslint-related packages will instead have the label linting.

Behavior details:

- On Forgejo, Gitea, GitHub and GitLab: Renovate will keep PR labels in sync with configured labels, provided that no other user or bot has made changes to the labels after PR creation. If labels are changed by any other account, Renovate will stop making further changes.
- For other platforms, Renovate will add labels only at time of PR creation and not update them after that.

The labels array is non-mergeable, meaning if multiple packageRules match then Renovate uses the last value for labels. If you want to add/combine labels, use the addLabels config option, which is mergeable.

Note

Keep your labels within the maximum character limit for your Git hosting platform. Renovate usually truncates labels to 50 characters, except for GitLab, which has a 255 character limit.

## lockFileMaintenance[¶](https://docs.renovatebot.com/configuration-options/#lockfilemaintenance)

Configuration for lock file maintenance.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| default  |

```
{
  "enabled": false,
  "recreateWhen": "always",
  "branchTopic": "lock-file-maintenance",
  "commitMessageAction": "Lock file maintenance",
  "commitMessageTopic": null,
  "commitMessageExtra": null,
  "schedule": ["before 4am on monday"],
  "groupName": null,
  "prBodyDefinitions": {"Change": "All locks refreshed"}
}

```

| mergeable  | true

| env  | RENOVATE_LOCK_FILE_MAINTENANCE

You can use lockFileMaintenance to refresh lock files to keep them up-to-date.

When Renovate performs lockFileMaintenance it deletes the lock file and runs the relevant package manager. That package manager creates a new lock file, where all dependency versions are updated to the latest version. Renovate then commits that lock file to the update branch and creates the lock file update PR.

Supported lock files:

| Manager  | Lockfile

| apm  | apm.lock.yaml

| bazel-module  | MODULE.bazel.lock

| bazelisk  | MODULE.bazel.lock

| bun  | bun.lockb, bun.lock

| bundler  | Gemfile.lock

| cargo  | Cargo.lock

| composer  | composer.lock

| conan  | conan.lock

| deno  | deno.lock

| devbox  | devbox.lock

| gleam  | manifest.toml

| gradle  | gradle.lockfile

| helmfile  | helmfile.lock

| helmv3  | Chart.lock

| jsonnet-bundler  | jsonnetfile.lock.json

| mise  | mise.lock

| mix  | mix.lock

| nix  | flake.lock

| npm  | package-lock.json, pnpm-lock.yaml, yarn.lock

| nuget  | packages.lock.json

| paket  | paket.lock

| pep621  | pdm.lock, uv.lock

| pip-compile  | requirements.txt

| pipenv  | Pipfile.lock

| pixi  | pixi.lock

| poetry  | poetry.lock

| pub  | pubspec.lock

| terraform  | .terraform.lock.hcl

| terragrunt  | .terraform.lock.hcl

| vendir  | vendir.lock.yml

Support for new lock files may be added via feature request.

By default, lockFileMaintenance is disabled. To enable lockFileMaintenance add this to your configuration:

```
{
  "lockFileMaintenance": { "enabled": true }
}

```

To reduce "noise" in the repository, Renovate performs lockFileMaintenance "before 4am on monday", i.e. to achieve once-per-week semantics. Depending on its running schedule, Renovate may run a few times within that time window - even possibly updating the lock file more than once - but it hopefully leaves enough time for tests to run and automerge to apply, if configured.

## logLevelRemap[¶](https://docs.renovatebot.com/configuration-options/#loglevelremap)

Remap log levels to different levels.

| Name  | Value

| scopes  | repo

| type  | array

| subType  | object

| mergeable  | true

| default  | []

This option allows you to remap log levels for specific messages.

Be careful with remapping warn or error messages to lower log levels, as it may hide important information.

```
{
  "logLevelRemap": [
    {
      "matchMessage": "/^pip-compile:/",
      "newLogLevel": "info"
    },
    {
      "matchMessage": "Package lookup error",
      "newLogLevel": "warn"
    },
    {
      "matchMessage": "/^Please upgrade the version of Node.js/",
      "newLogLevel": "info"
    }
  ]
}

```

### logLevelRemap.matchMessage[¶](https://docs.renovatebot.com/configuration-options/#loglevelremapmatchmessage)

Regex/minimatch expression to match against log message.

| Name  | Value

| type  | string

| parents  | logLevelRemap

| default  | null

Use matchMessage to match the log message you want to remap. Accepts a string or regex pattern.

### logLevelRemap.newLogLevel[¶](https://docs.renovatebot.com/configuration-options/#loglevelremapnewloglevel)

New log level to use if matchMessage matches.

| Name  | Value

| type  | string

| allowedValues  |

```
["trace", "debug", "info", "warn", "error", "fatal"]

```

| parents  | logLevelRemap

| default  | null

For log level remapping, newLogLevel will set for the particular log message:

```
{
  "logLevelRemap": [
    {
      "matchMessage": "/Error executing maven wrapper update command/",
      "newLogLevel": "warn"
    }
  ]
}

```

## major[¶](https://docs.renovatebot.com/configuration-options/#major)

Configuration to apply when an update type is major.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| mergeable  | true

| env  | RENOVATE_MAJOR

Add to this object if you wish to define rules that apply only to major updates.

## maxMajorIncrement[¶](https://docs.renovatebot.com/configuration-options/#maxmajorincrement)

Limit the maximum major version increment allowed. Set to 0 to disable.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | integer

| default  | 500

This is particularly useful for preventing upgrades from [SemVer](https://semver.org/)-based versions to [CalVer](https://calver.org/)-based ones.

For example, if you're on version 19.0.0, Renovate will upgrade to 22.0.0 but filter out versions like 999.0.0 and 2025.0.0.

You can use packageRules to customize this behavior for specific packages:

```
{
  "packageRules": [
    {
      "description": "Allow to upgrade Home Assistant from 0.x to 2025.x",
      "matchPackageNames": ["homeassistant"],
      "maxMajorIncrement": 0
    }
  ]
}

```

The above allows unlimited major version upgrades by setting maxMajorIncrement to 0.

Alternatively, to limit major upgrades to within 10 versions:

```
{
  "packageRules": [
    {
      "description": "Limit xmldoc major upgrades to within 10 versions",
      "matchPackageNames": ["xmldoc"],
      "maxMajorIncrement": 10
    }
  ]
}

```

## managerFilePatterns[¶](https://docs.renovatebot.com/configuration-options/#managerfilepatterns)

RegEx (re2) and glob patterns for matching manager files.

| Name  | Value

| type  | array

| subType  | string

| mergeable  | true

| scopes  | manager

| parents  | customManagers

| default  | []

Formerly known as fileMatch, which supported regex-only.

managerFilePatterns supports both regex and glob patterns. Any existing config containing fileMatch patterns will be automatically migrated.

Do not use the below guide for fileMatch if you are using an older version of Renovate.

managerFilePatterns tells Renovate which repository files to parse and extract. Patterns in the user config are added to the default values, they do not replace the default values.

The default managerFilePatterns patterns can not be removed. If you need to include, or exclude, specific paths then use the [ignorePaths](https://docs.renovatebot.com/configuration-options/#ignorepaths) or [includePaths](https://docs.renovatebot.com/configuration-options/#includepaths) instead.

Some managers have sensible defaults. For example, the Go manager looks for any go.mod file by default, which covers most cases without extra configuration.

At other times, the possible files is too vague for Renovate to have any default. For example, Kubernetes manifests can exist in any *.yaml file. We do not want Renovate to parse every YAML file in every repository, just in case some of them have a Kubernetes manifest. Therefore Renovate's default managerFilePatterns for the kubernetes manager is an empty array ([]). Because the array is empty, you as user must tell Renovate which directories/files to check.

Finally, there are cases where Renovate's default managerFilePatterns is good, but you may be using non-standard filenames. For example, Renovate's default managerFilePatterns for Dockerfile is ['/(^|/|\\.)([Dd]ocker|[Cc]ontainer)file$/', '/(^|/)([Dd]ocker|[Cc]ontainer)file[^/]*$/']. This will catch files like backend/Dockerfile, prefix.Dockerfile or Dockerfile-suffix, but it will miss files like ACTUALLY_A_DOCKERFILE.template. Because managerFilePatterns is "mergeable", you can add the missing file to the filePattern like this:

```
{
  "dockerfile": {
    "managerFilePatterns": ["/^ACTUALLY_A_DOCKERFILE\\.template$/"]
  }
}

```

You must configure managerFilePatterns inside a manager object. In the example above, the manager object is the dockerfile. For reference, here is a [list of supported managers](https://docs.renovatebot.com/modules/manager/#supported-managers).

## milestone[¶](https://docs.renovatebot.com/configuration-options/#milestone)

The number of a milestone. If set, the milestone will be set when Renovate creates the PR.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| supportedPlatforms  | github

| cli  | --milestone

| env  | RENOVATE_MILESTONE

If set to the number of an existing [GitHub milestone](https://docs.github.com/en/issues/using-labels-and-milestones-to-track-work/about-milestones), Renovate will add that milestone to its PR. Renovate will only add a milestone when it creates the PR.

Example Renovate config

```
{
  "milestone": 12
}

```

## minimumGroupSize[¶](https://docs.renovatebot.com/configuration-options/#minimumgroupsize)

The minimum number of updates which must be in a group for branches to be created.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| default  | 1

| cli  | --minimum-group-size

| env  | RENOVATE_MINIMUM_GROUP_SIZE

If set to to a positive value x then branch creation will be postponed until x or more updates are available in the branch.

This applies to both these scenarios:

- Grouped updates with more than one dependency updated together, and
- Branches with multiple updates of the same dependency (e.g. in multiple files)

Example:

Create only a grouped update when there are 3 or more node updates

```
{
  "packageRules": [
    {
      "description": "We need to update Node in two places - always wait until both upgrades are available",
      "matchDepNames": ["node"],
      "groupName": "Node.js",
      "minimumGroupSize": 3
    }
  ]
}

```

## minimumReleaseAge[¶](https://docs.renovatebot.com/configuration-options/#minimumreleaseage)

Time required before a new release is considered stable.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert, updateType

| type  | string

| cli  | --minimum-release-age

| env  | RENOVATE_MINIMUM_RELEASE_AGE

minimumReleaseAge is a feature that requires Renovate to wait for a specified amount of time before suggesting a dependency update.

Note

Minimum Release Age has [a separate documentation page](https://docs.renovatebot.com/key-concepts/minimum-release-age/) for more in-depth documentation about the functionality. This also includes documentation that was previously in this section, regarding how to specify release timestamps for custom registries.

If minimumReleaseAge is set to a time duration and the update has a release timestamp header, then Renovate will check if the set duration has passed. This behaviour can be changed using [minimumReleaseAgeBehaviour](https://docs.renovatebot.com/configuration-options/#minimumreleaseagebehaviour).

Note: Renovate will wait for the set duration to pass for each separate version. Renovate does not wait until the package has seen no releases for x time-duration(minimumReleaseAge).

Do not use minimumReleaseAge to slow down fast releasing project updates. Instead setup a custom schedule for that package, read [our selective-scheduling help](https://docs.renovatebot.com/noise-reduction/#selective-scheduling) to learn how.

When the time passed since the release is less than the set minimumReleaseAge: Renovate adds a "pending" status check to that update's branch. After enough days have passed: Renovate replaces the "pending" status with a "passing" status check.

The datasource that Renovate uses must have a release timestamp for the minimumReleaseAge config option to work. Some datasources may have a release timestamp, but in a format Renovate does not support. In those cases a feature request needs to be implemented.

You can confirm if your datasource supports the release timestamp by viewing [the documentation for the given datasource](https://docs.renovatebot.com/modules/datasource/).

Note

Configuring this option will add a renovate/stability-days option to the status checks.

Note

As of Renovate 42.19.5, using minimumReleaseAge=0 days is treated the same as minimumReleaseAge=null.

Examples of how you can use minimumReleaseAge:

#### Suppress branch/PR creation for X days[¶](https://docs.renovatebot.com/configuration-options/#suppress-branchpr-creation-for-x-days)

If you use minimumReleaseAge=3 days, prCreation="not-pending" and internalChecksFilter="strict" then Renovate only creates branches when 3 (or more days) have passed since the version was released. We recommend you set dependencyDashboard=true, so you can see these pending PRs.

Warning

prCreation="not-pending" only opens the PR once branch checks complete. If your CI runs only on pull_request events (so Renovate branches have no checks), also set internalChecksAsSuccess=true so the release-age check can turn the branch green - otherwise PR creation may be [deferred indefinitely](https://docs.renovatebot.com/configuration-options/#prcreation).

#### Prevent holding broken npm packages[¶](https://docs.renovatebot.com/configuration-options/#prevent-holding-broken-npm-packages)

npm packages less than 72 hours (3 days) old can be unpublished from the npm registry, which could result in a service impact if you have already updated to it. Set minimumReleaseAge to 3 days for npm packages to prevent relying on a package that can be removed from the registry:

```
{
  "packageRules": [
    {
      "matchDatasources": ["npm"],
      "minimumReleaseAge": "3 days"
    }
  ]
}

```

#### Await X time duration before Automerging[¶](https://docs.renovatebot.com/configuration-options/#await-x-time-duration-before-automerging)

If you enable automerge and minimumReleaseAge, Renovate Renovate will create PRs immediately, but only automerge them when the minimumReleaseAge time-duration has passed.

Renovate adds a "renovate/stability-days" pending status check to each branch/PR. This pending check prevents the branch going green to automerge before the time has passed.

## minimumReleaseAgeBehaviour[¶](https://docs.renovatebot.com/configuration-options/#minimumreleaseagebehaviour)

When set in conjunction with minimumReleaseAge, controls whether the releaseTimestamp for a dependency update is required.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert, updateType

| type  | string

| default  | "timestamp-required"

| allowedValues  |

```
["timestamp-required", "timestamp-optional"]

```

| cli  | --minimum-release-age-behaviour

| env  | RENOVATE_MINIMUM_RELEASE_AGE_BEHAVIOUR

When minimumReleaseAge is set to a time duration, the minimumReleaseAgeBehaviour will be used to control whether the release timestamp is required.

When set to timestamp-required, this version is not treated stable unless there is release timestamp, and that release timestamp is past the [minimumReleaseAge](https://docs.renovatebot.com/configuration-options/#minimumreleaseage).

When set to timestamp-optional, Renovate will treat a release without a releaseTimestamp as stable.

This only applies when used with [minimumReleaseAge](https://docs.renovatebot.com/configuration-options/#minimumreleaseage).

## minimumReleaseAgeBuffer[¶](https://docs.renovatebot.com/configuration-options/#minimumreleaseagebuffer)

Extra time added to minimumReleaseAge before an update is considered stable.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert, updateType

| type  | string

| default  | "30 minutes"

| nullable  | true

| cli  | --minimum-release-age-buffer

| env  | RENOVATE_MINIMUM_RELEASE_AGE_BUFFER

Use this option to extend the time Renovate waits before suggesting an update, on top of [minimumReleaseAge](https://docs.renovatebot.com/configuration-options/#minimumreleaseage).

Some projects publish a set of related packages, where companion packages (like platform-specific binaries) are published after the main package. For example, @biomejs/biome@2.5.10 may be published some minutes before @biomejs/cli-linux-x64@2.5.10. If your package manager enforces its own release age cooldown, like pnpm's minimumReleaseAge setting or npm's --before flag, Renovate may suggest the main package as soon as it passes minimumReleaseAge, while a companion package published later is still within the package manager's cooldown. The package manager then refuses to update the lock file, and the update fails with an artifact error.

minimumReleaseAgeBuffer avoids this by letting Renovate wait longer than the package manager's cooldown. Renovate defers the update until minimumReleaseAge plus the buffer duration has passed, while the package manager keeps using the plain minimumReleaseAge cutoff. This way, packages published up to the buffer time after the suggested release have also passed the package manager's cooldown.

By default, Renovate uses a buffer of 30 minutes. Set minimumReleaseAgeBuffer to null to disable the buffer.

For example, with the following configuration Renovate waits 3 days plus 1 hour before suggesting an update, while the package manager's cooldown stays at 3 days:

```
{
  "minimumReleaseAge": "3 days",
  "minimumReleaseAgeBuffer": "1 hour"
}

```

This only applies when used with [minimumReleaseAge](https://docs.renovatebot.com/configuration-options/#minimumreleaseage).

## minor[¶](https://docs.renovatebot.com/configuration-options/#minor)

Configuration to apply when an update type is minor.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| mergeable  | true

| env  | RENOVATE_MINOR

Add to this object if you wish to define rules that apply only to minor updates.

## mode[¶](https://docs.renovatebot.com/configuration-options/#mode)

Mode of operation.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "full"

| allowedValues  |

```
["full", "silent"]

```

| cli  | --mode

| env  | RENOVATE_MODE

This configuration option was created primarily for use with Mend's hosted app, but can also be useful for some self-hosted use cases.

It enables a new silent mode to allow repos to be scanned for updates and for users to be able to request such updates be opened in PRs on demand through the Mend UI, without needing the Dependency Dashboard issue in the repo.

Although similar, the options mode=silent and dryRun can be used together. When both are configured, dryRun takes precedence, so for example PRs won't be created.

Configuring silent mode is quite similar to dryRun=lookup except:

- It will bypass onboarding checks (unlike when performing a dry run on a non-onboarded repo) similar to requireConfig=optional
- It can create branches/PRs if checkedBranches is set
- It will keep any existing branches up-to-date (e.g. ones created previously using checkedBranches)

When in silent mode Renovate will:

- not create or update any Issue: even the Dependency Dashboard or Config Warning Issues will stay as-is
- not read checkboxes from an existing Dependency Dashboard: use checkedBranches to request branches instead
- not prune or close any existing Issues
- not create any Config Migration PRs, even if you explicitly enabled Config Migration PRs in your Renovate config

## npmToken[¶](https://docs.renovatebot.com/configuration-options/#npmtoken)

npm token used to authenticate with the default registry.

| Name  | Value

| scopes  | repo

| type  | string

| cli  | --npm-token

| env  | RENOVATE_NPM_TOKEN

| default  | null

See [Private npm module support](https://docs.renovatebot.com/getting-started/private-packages/) for details on how this is used. Typically you would encrypt it and put it inside the encrypted object.

## npmrc[¶](https://docs.renovatebot.com/configuration-options/#npmrc)

String copy of .npmrc file. Use \n instead of line breaks.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| cli  | --npmrc

| env  | RENOVATE_NPMRC

| default  | null

See [Private npm module support](https://docs.renovatebot.com/getting-started/private-packages/) for details on how this is used.

## npmrcMerge[¶](https://docs.renovatebot.com/configuration-options/#npmrcmerge)

Whether to merge config.npmrc with repo .npmrc content if both are found.

| Name  | Value

| scopes  | repo, manager

| type  | boolean

| default  | false

| cli  | --npmrc-merge

| env  | RENOVATE_NPMRC_MERGE

This option exists to provide flexibility about whether npmrc strings in config should override .npmrc files in the repo, or be merged with them. In some situations you need the ability to force override .npmrc contents in a repo (npmrcMerge=false) while in others you might want to simply supplement the settings already in the .npmrc (npmrcMerge=true). A use case for the latter is if you are a Renovate admin and wish to provide a default token for npmjs.org without removing any other .npmrc settings which individual repositories have configured (such as scopes/registries).

If false (default), it means that defining config.npmrc will result in any .npmrc file in the repo being overridden and its values ignored. If configured to true, it means that any .npmrc file in the repo will have config.npmrc prepended to it before running npm.

## osvVulnerabilityAlerts[¶](https://docs.renovatebot.com/configuration-options/#osvvulnerabilityalerts)

Use vulnerability alerts from osv.dev.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | false

| cli  | --osv-vulnerability-alerts

| env  | RENOVATE_OSV_VULNERABILITY_ALERTS

This feature is flagged as experimental

Experimental features might be changed or even removed at any time.
To track this feature visit the following GitHub issue [#20542](https://github.com/renovatebot/renovate/issues/20542).

Renovate integrates with [OSV](https://osv.dev/), an Open Source vulnerability database, to check if extracted dependencies have known vulnerabilities. Set osvVulnerabilityAlerts to true to get pull requests with vulnerability fixes (once they are available).

You will only get OSV-based vulnerability alerts for direct dependencies. Renovate only queries the OSV database for dependencies that use one of these datasources:

- [crate](https://docs.renovatebot.com/modules/datasource/crate/)
- [go](https://docs.renovatebot.com/modules/datasource/go/)
- [hackage](https://docs.renovatebot.com/modules/datasource/hackage/)
- [hex](https://docs.renovatebot.com/modules/datasource/hex/)
- [maven](https://docs.renovatebot.com/modules/datasource/maven/)
- [npm](https://docs.renovatebot.com/modules/datasource/npm/)
- [nuget](https://docs.renovatebot.com/modules/datasource/nuget/)
- [packagist](https://docs.renovatebot.com/modules/datasource/packagist/)
- [pypi](https://docs.renovatebot.com/modules/datasource/pypi/)
- [rubygems](https://docs.renovatebot.com/modules/datasource/rubygems/)

The entire database is downloaded locally by [osv-offline](https://github.com/renovatebot/osv-offline) and queried offline.

If you self-host Renovate, you can change how the database is downloaded with these osv-offline environment variables:

- OSV_OFFLINE_ROOT_DIR: where the database is stored
- OSV_OFFLINE_MAX_AGE: how old the database may be before it's downloaded again, as an [ms](https://github.com/vercel/ms) duration such as 30m, 4h or 7d (default: 1d)
- OSV_OFFLINE_DATABASE_FORMAT=split: download the database of each ecosystem on its first use, instead of all of them at once
- OSV_OFFLINE_DATABASE_URL and OSV_OFFLINE_DATABASE_INDEX_URL: download from an internal mirror
- OSV_OFFLINE_DISABLE_DOWNLOAD=true: use a database you've put into OSV_OFFLINE_ROOT_DIR yourself, without internet access

Read the [osv-offline readme](https://github.com/renovatebot/osv-offline/tree/main/packages/osv-offline#readme) for all options, mirrors, and which hosts to allow.

#### Malicious package detection and protection[¶](https://docs.renovatebot.com/configuration-options/#malicious-package-detection-and-protection)

Note

This functionality is currently experimental, being actively worked on, and will soon be available for usage outside of osvVulnerabilityAlerts.

If Renovate detects a malicious dependency using data from OSV, it will surface this in log warnings, and prevent PRs from being created.

If you currently have a dependency that is using a malicious version, Renovate will report this via a warning log.

If Renovate finds a dependency update available, and that dependency update is found to be malicious, Renovate will skip any updates to the dependency, marking it with skipReason: malicious-update-proposed, and report this via a warning log.

## overrideDescription[¶](https://docs.renovatebot.com/configuration-options/#overridedescription)

Description which replaces the descriptions of any presets which this config extends.

| Name  | Value

| scopes  | any

| type  | array

| subType  | string

| default  | []

Use overrideDescription instead of [description](https://docs.renovatebot.com/configuration-options/#description) if this config's description should replace, and not be added to, the descriptions collated from the presets which this config extends.

This is useful for a preset which extends many other presets, and where a single line describes them better than one line per preset would. For example, workarounds:all extends around twenty presets, but its overrideDescription means the onboarding PR shows only:

```
- Apply crowd-sourced workarounds for known problems with packages.

```

Renovate applies overrideDescription when it resolves presets, so the resolved config only ever has a description.

## packageRules[¶](https://docs.renovatebot.com/configuration-options/#packagerules)

Rules for matching packages.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert

| type  | array

| mergeable  | true

| cli  | --package-rules

| env  | RENOVATE_PACKAGE_RULES

| default  | []

packageRules is a powerful feature that lets you apply rules to individual packages or to groups of packages using regex pattern matching.

packageRules is a collection of rules, that are all evaluated. If multiple rules match a dependency, configurations from matching rules will be merged together. The order of rules matters, because later rules may override configuration options from earlier ones, if they both specify the same option.

The matching process for a package rule:

- Each package rule must include at least one match... matcher.
- If multiple matchers are included in one package rule, all of them must match.
- Each matcher must contain at least one pattern. Some matchers allow both positive and negative patterns.
- If a matcher includes any positive patterns, it must match at least one of them.
- A matcher returns false if it matches any negative pattern, even if a positive match also occurred.

For more details on positive and negative pattern syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/).

Here is an example if you want to group together all packages starting with eslint into a single branch/PR:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["eslint**"],
      "groupName": "eslint packages"
    }
  ]
}

```

Note how the above uses matchPackageNames with a prefix pattern.

Here's an example config to limit the "noisy" AWS SDK packages to weekly updates:

```
{
  "packageRules": [
    {
      "description": "Schedule AWS SDK updates on Sunday nights (9 PM - 12 AM)",
      "matchPackageNames": ["@aws-sdk/*"],
      "schedule": ["* 21-23 * * 0"]
    }
  ]
}

```

For Maven dependencies, the package name is <groupId:artefactId>, e.g. "matchPackageNames": ["com.thoughtworks.xstream:xstream"]

Note how the above uses an exact match string for matchPackageNames instead of a pattern However you can mix together both patterns and exact matches in the same package rule and the rule will be applied if either match. Example:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["neutrino", "@neutrino/**"],
      "groupName": "neutrino monorepo"
    }
  ]
}

```

The above rule will group together the neutrino package and any package starting with @neutrino/.

File name matches are convenient to use if you wish to apply configuration rules to certain package or lock files using patterns. For example, if you have an examples directory and you want all updates to those examples to use the chore prefix instead of fix, then you could add this configuration:

```
{
  "packageRules": [
    {
      "matchFileNames": ["examples/**"],
      "extends": [":semanticCommitTypeAll(chore)"]
    }
  ]
}

```

If you wish to limit Renovate to apply configuration rules to certain files in the root repository directory, you have to use matchFileNames with a minimatch glob (which can include an exact file name match) or RE2 regex.

For more details on supported syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/).

For example you have multiple package.json and want to use dependencyDashboardApproval only on the root package.json:

```
{
  "packageRules": [
    {
      "matchFileNames": ["package.json"],
      "dependencyDashboardApproval": true
    }
  ]
}

```

Tip

Order your packageRules so the least important rules are at the top, and the most important rules at the bottom. This way important rules override settings from earlier rules if needed.

Warning

Avoid nesting any object-type configuration in a packageRules array, such as a major or minor block.

### packageRules.allowedVersions[¶](https://docs.renovatebot.com/configuration-options/#packagerulesallowedversions)

A version range or regex pattern capturing allowed versions for dependencies.

| Name  | Value

| type  | string

| parents  | packageRules

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

You can use allowedVersions - usually within a packageRules entry - to limit how far to upgrade a dependency.

For example, if you want to upgrade to Angular v1.5 but not to angular v1.6 or higher, you could set allowedVersions to <= 1.5 or < 1.6.0:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["angular"],
      "allowedVersions": "<=1.5"
    }
  ]
}

```

Renovate calculates the valid syntax for this at runtime, because it depends on the dynamic versioning scheme.

You can also use the following template fields with allowedVersions:

- currentVersion
- major
- minor
- patch

For example, if you want to upgrade to Angular to only the next major, you could set allowedVersions like this:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["angular"],
      "allowedVersions": "<={{add major 1}}"
    }
  ]
}

```

Warning

allowedVersions and matchUpdateTypes cannot be used in the same package rule.

#### Using regular expressions[¶](https://docs.renovatebot.com/configuration-options/#using-regular-expressions)

You can use Regular Expressions in the allowedVersions config. You must begin and end your Regular Expression with the / character!

For example, this config only allows 3 or 4-part versions, without any prefixes in the version:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["com.thoughtworks.xstream:xstream"],
      "allowedVersions": "/^[0-9]+\\.[0-9]+\\.[0-9]+(\\.[0-9]+)?$/"
    }
  ]
}

```

Again: note how the Regular Expression begins and ends with the / character.

#### Ignore versions with negated regex syntax[¶](https://docs.renovatebot.com/configuration-options/#ignore-versions-with-negated-regex-syntax)

You can use a special negated regex syntax to ignore certain versions. You must use the !/ / syntax, like this:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["chalk"],
      "allowedVersions": "!/java$/"
    }
  ]
}

```

### packageRules.changelogUrl[¶](https://docs.renovatebot.com/configuration-options/#packageruleschangelogurl)

Set a custom URL for the changelog. Renovate will put this URL in the PR body text.

| Name  | Value

| type  | string

| parents  | packageRules

| supportsTemplating  | true

| previouslyKnownAs  |

```
["customChangelogUrl"]

```

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Sometimes Renovate does not show the correct changelog for a package. As a workaround for this problem, you can give Renovate the URL to the changelog with the changelogUrl config option. When set, Renovate will put a link to the changelogs in the Renovate PR body.

Renovate does not show the complete changelogs from the changelogUrl in its PR body text, you only get the URL from Renovate. To read the changelogs you must use the link.

Setting the changelog URL for the dummy package

```
{
  "packageRules": [
    {
      "matchPackageNames": ["dummy"],
      "changelogUrl": "https://github.com/org/dummy"
    }
  ]
}

```

changelogUrl supports template compilation.

Setting the changelog URL for the dummy package using a template

```
{
  "packageRules": [
    {
      "matchPackageNames": ["dummy"],
      "changelogUrl": "https://github.com/org/monorepo/blob/{{{sourceDirectory}}}/my-custom-changelog.txt"
    }
  ]
}

```

Note

Renovate can fetch changelogs from Bitbucket, Bitbucket Server / Data Center, Forgejo, Gitea, GitHub and GitLab platforms only, and setting the URL to an unsupported host/platform type won't change that.

For more details on supported syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/).

### packageRules.matchBaseBranches[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchbasebranches)

List of strings containing exact matches (e.g. ["main"]) and/or regex expressions (e.g. ["/^release/.*/"]). Valid only within a packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

Use this field to restrict rules to a particular branch. e.g.

```
{
  "baseBranchPatterns": ["main"],
  "packageRules": [
    {
      "matchBaseBranches": ["main"],
      "matchPackageNames": ["!/^eslint/"],
      "enabled": false
    }
  ]
}

```

This field also supports Regular Expressions if they begin and end with /. e.g.

```
{
  "baseBranchPatterns": ["main", "/^release\\/.*/"],
  "packageRules": [
    {
      "matchBaseBranches": ["/^release/.*/"],
      "matchPackageNames": ["!/^eslint/"],
      "enabled": false
    }
  ]
}

```

### packageRules.matchCategories[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchcategories)

List of categories to match (for example: ["python"]). Valid only within a packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

Use matchCategories to restrict rules to a particular language or group. Matching is done using "any" logic, i.e. "match any of the following categories". The categories can be found in the [manager documentation](https://docs.renovatebot.com/modules/manager/).

Note

Rules with matchCategories are only applied after extraction of dependencies. If you want to configure which managers are being extracted at all, use enabledManagers instead.

```
{
  "packageRules": [
    {
      "matchCategories": ["python"],
      "addLabels": ["py"]
    }
  ]
}

```

### packageRules.matchConfidence[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchconfidence)

Merge confidence levels to match against (low, neutral, high, very high). Valid only within packageRules object.

| Name  | Value

| type  | array

| subType  | string

| allowedValues  |

```
["low", "neutral", "high", "very high"]

```

| parents  | packageRules

| mergeable  | true

| default  | []

Warning

This configuration option needs a Mend API key, and is in private beta testing only. API keys are not available for free or via the renovatebot/renovate repository.

Grouping high merge confidence updates

```
{
  "packageRules": [
    {
      "matchConfidence": ["high", "very high"],
      "groupName": "high merge confidence"
    }
  ]
}

```

### packageRules.matchCurrentAge[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchcurrentage)

Matches the current age of the package derived from its release timestamp. Valid only within a packageRules object.

| Name  | Value

| type  | string

| parents  | packageRules

| mergeable  | true

| default  | null

Use this field if you want to match packages based on the age of the current (existing, in-repo) version.

For example, if you want to group updates for dependencies where the existing version is more than 2 years old:

```
{
  "packageRules": [
    {
      "matchCurrentAge": "> 2 years",
      "groupName": "old dependencies"
    }
  ]
}

```

The matchCurrentAge string must start with one of >, >=, < or <=.

Only one date part is supported, so you cannot do > 1 year 1 month. Instead you should do > 13 months.

Note

We recommend you only use the words hour(s), day(s), week(s), month(s) and year(s) in your time ranges.

### packageRules.matchCurrentValue[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchcurrentvalue)

A regex or glob pattern to match against the raw currentValue string of a dependency. Valid only within a packageRules object.

| Name  | Value

| type  | string

| parents  | packageRules

| mergeable  | true

| default  | null

This option is matched against the currentValue field of a dependency.

matchCurrentValue supports Regular Expressions and glob patterns. For example, the following enforces that updates from 1.* versions will be merged automatically:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["io.github.resilience4j**"],
      "matchCurrentValue": "1.*",
      "automerge": true
    }
  ]
}

```

Regular Expressions must begin and end with /.

```
{
  "packageRules": [
    {
      "matchPackageNames": ["io.github.resilience4j**"],
      "matchCurrentValue": "/^1\\./"
    }
  ]
}

```

This field also supports a special negated regex syntax to ignore certain versions. Use the syntax !/ / like this:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["io.github.resilience4j**"],
      "matchCurrentValue": "!/^0\\./"
    }
  ]
}

```

### packageRules.matchCurrentVersion[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchcurrentversion)

A version, or range of versions, to match against the current version of a package. Valid only within a packageRules object.

| Name  | Value

| type  | string

| parents  | packageRules

| mergeable  | true

| default  | null

The currentVersion field will be one of the following (in order of preference):

- locked version if a lock file exists
- resolved version
- current value

Consider using instead matchCurrentValue if you wish to match against the raw string value of a dependency.

matchCurrentVersion can be an exact version or a version range:

```
{
  "packageRules": [
    {
      "matchCurrentVersion": ">=1.0.0",
      "matchPackageNames": ["angular"]
    }
  ]
}

```

The syntax of the version range must follow the [versioning scheme](https://docs.renovatebot.com/modules/versioning/#supported-versioning) used by the matched package(s). This is usually defined by the [manager](https://docs.renovatebot.com/modules/manager/#supported-managers) which discovered them or by the default versioning for the package's [datasource](https://docs.renovatebot.com/modules/datasource/). For example, a Gradle package would typically need Gradle constraint syntax (e.g. [,7.0)) and not SemVer syntax (e.g. <7.0).

This field also supports Regular Expressions which must begin and end with /. For example, the following enforces that only 1.* versions will be used:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["io.github.resilience4j**"],
      "matchCurrentVersion": "/^1\\./"
    }
  ]
}

```

This field also supports a special negated regex syntax to ignore certain versions. Use the syntax !/ / like this:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["io.github.resilience4j**"],
      "matchCurrentVersion": "!/^0\\./"
    }
  ]
}

```

### packageRules.matchDatasources[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchdatasources)

List of datasources to match (e.g. ["orb"]). Valid only within a packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

Use this field to restrict rules to a particular datasource. e.g.

```
{
  "packageRules": [
    {
      "matchDatasources": ["orb"],
      "labels": ["circleci-orb!!"]
    }
  ]
}

```

### packageRules.matchDepNames[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchdepnames)

Dep names to match. Valid only within a packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

This field behaves the same as matchPackageNames except it matches against depName instead of packageName.

### packageRules.matchDepTypes[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchdeptypes)

List of depTypes to match (e.g. [peerDependencies]). Valid only within packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

Use this field if you want to limit a packageRule to certain depType values. This matching can be an exact match, Glob match, or Regular Expression match.

For more details on supported syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/). Note that Glob matching (including exact name matching) is case-insensitive.

Invalid if used outside of a packageRule.

### packageRules.matchFileNames[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchfilenames)

List of patterns to match against package and lock file paths. Valid only within a packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| default  | []

Renovate will compare matchFileNames glob or RE2 regex matching against the dependency's package file and also lock file if one exists.

For more details on supported syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/).

The following example matches package.json but not package/frontend/package.json:

```
{
  "packageRules": [
    {
      "matchFileNames": ["package.json"],
      "labels": ["npm"]
    }
  ]
}

```

The following example matches any package.json, including files like backend/package.json:

```
{
  "packageRules": [
    {
      "description": "Group dependencies from package.json files",
      "matchFileNames": ["**/package.json"],
      "groupName": "All package.json changes"
    }
  ]
}

```

The following example matches any file in directories starting with app/:

```
{
  "packageRules": [
    {
      "description": "Group all dependencies from the app directory",
      "matchFileNames": ["app/**"],
      "groupName": "App dependencies"
    }
  ]
}

```

The following example matches any .toml file in a v1, v2 or v3 directory:

```
{
  "packageRules": [
    {
      "description": "Group all dependencies from legacy projects",
      "matchFileNames": ["/v[123]/.*\\.toml/"],
      "groupName": "Legacy project dependencies"
    }
  ]
}

```

It is recommended that you avoid using "negative" globs, like **/!(package.json), because such patterns might still return true if they match against the lock file name (e.g. package-lock.json).

### packageRules.matchIsBreaking[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchisbreaking)

Matches only if the update is considered breaking (e.g. major, or minor for 0.x in ecosystems like Cargo) when true, or non-breaking when false. Valid only within a packageRules object.

| Name  | Value

| type  | boolean

| parents  | packageRules

| default  | true

Use matchIsBreaking to match updates based on whether Renovate considers them breaking. Set it to true to match only breaking updates, or false to match only non-breaking updates.

What counts as breaking depends on the versioning of the dependency:

- Versionings with their own notion of breaking changes decide themselves, for example Cargo treats a minor bump of a 0.x crate (0.1.0 to 0.2.0) as breaking
- For all other versionings, an update is breaking if its updateType is major

Rules with matchIsBreaking never match when there is no update to evaluate, for example for lockFileMaintenance.

The following example automerges all non-breaking updates of packages in the @myorg scope:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["@myorg{/,}**"],
      "matchIsBreaking": false,
      "automerge": true
    }
  ]
}

```

### packageRules.matchJsonata[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchjsonata)

A JSONata expression to match against the full config object. Valid only within a packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

Use the matchJsonata field to define custom matching logic using [JSONata](https://jsonata.org/) query logic. Renovate will evaluate the provided JSONata expressions against the passed values (manager, packageName, etc.).

See [the JSONata docs](https://docs.jsonata.org/) for more details on JSONata syntax.

Here are some example matchJsonata strings for inspiration:

```
$exists(deprecationMessage)
$exists(vulnerabilityFixVersion)
manager = 'dockerfile' and depType = 'final'
updateType = 'major' and newVersionAgeInDays < 7
$detectPlatform(sourceUrl) = 'github'
$matchRegexOrGlob(packageName, ["@myorg{/,}**"]) or $matchRegexOrGlob(registryUrls, ["https://registry.example.com/**"])

```

matchJsonata accepts an array of strings, and will return true if any of those JSONata expressions evaluate to true.

Renovate provides the following custom JSONata functions:

- $detectPlatform(url) - Takes a URL string and returns the detected platform (azure, bitbucket, bitbucket-server, forgejo, gitea, github, gitlab) or null.
- $matchRegexOrGlob(input, patterns) - Returns true if input matches patterns, using Renovate's [string pattern matching](https://docs.renovatebot.com/string-pattern-matching/) syntax. input can be a string or an array of strings, in which case any matching element returns true. patterns is an array of strings, or a single string. Returns false if input is missing. Use this to combine conditions on different fields with or, which separate match* options can't do because they are combined with and.

### packageRules.matchManagers[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchmanagers)

List of package managers to match (e.g. ["pipenv"]). Valid only within a packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

Use this field to restrict rules to a particular package manager. e.g.

```
{
  "packageRules": [
    {
      "matchPackageNames": ["node"],
      "matchManagers": ["dockerfile"],
      "enabled": false
    }
  ]
}

```

### packageRules.matchNewValue[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchnewvalue)

A regex or glob pattern to match against the raw newValue string of a dependency. Valid only within a packageRules object.

| Name  | Value

| type  | string

| parents  | packageRules

| mergeable  | true

| default  | null

This option is matched against the newValue field of a dependency.

matchNewValue supports Regular Expressions and glob patterns. For example, the following enforces that updates to 1.* versions will be merged automatically:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["io.github.resilience4j**"],
      "matchNewValue": "1.*",
      "automerge": true
    }
  ]
}

```

Regular Expressions must begin and end with /.

```
{
  "packageRules": [
    {
      "matchPackageNames": ["io.github.resilience4j**"],
      "matchNewValue": "/^1\\./"
    }
  ]
}

```

This field also supports a special negated regex syntax to ignore certain versions. Use the syntax !/ / like this:

```
{
  "packageRules": [
    {
      "matchPackageNames": ["io.github.resilience4j**"],
      "matchNewValue": "!/^0\\./"
    }
  ]
}

```

For more details on this syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/).

### packageRules.matchPackageNames[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchpackagenames)

Package names to match. Valid only within a packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

Use this field to match against the packageName field. This matching can be an exact match, Glob match, or Regular Expression match.

For more details on supported syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/). Note that Glob matching (including exact name matching) is case-insensitive.

exact name match

```
{
  "packageRules": [
    {
      "matchDatasources": ["npm"],
      "matchPackageNames": ["angular"],
      "rangeStrategy": "pin"
    }
  ]
}

```

The above will configure rangeStrategy to pin only for the npm package angular.

prefix match using Glob

```
{
  "packageRules": [
    {
      "matchPackageNames": ["@angular/**", "!@angular/abc"],
      "rangeStrategy": "replace"
    }
  ]
}

```

The above will set a replaceStrategy for any npm package which starts with @angular/ except @angular/abc.

pattern match using RegEx

```
{
  "packageRules": [
    {
      "matchDatasources": ["npm"],
      "matchPackageNames": ["/^angular/"],
      "groupName": "Angular"
    }
  ]
}

```

The above will group together any npm package which starts with the string angular.

### packageRules.matchRegistryUrls[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchregistryurls)

A list of URLs (or URL patterns) to match against registryUrls.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

Use this option to match packages based on their registryUrls. Any package whose registryUrls array contains at least one URL matching any of the provided patterns will be selected.

Here's an example of where you use this to disable updates from a specific private registry:

```
{
  "packageRules": [
    {
      "matchRegistryUrls": ["https://private.registry.example.com/**"],
      "enabled": false
    }
  ]
}

```

### packageRules.matchRepositories[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchrepositories)

List of repositories to match (e.g. ["**/*-archived"]). Valid only within a packageRules object.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

### packageRules.matchSourceUrls[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchsourceurls)

A list of exact match URLs (or URL patterns) to match sourceUrl against.

| Name  | Value

| type  | array

| subType  | string

| parents  | packageRules

| mergeable  | true

| default  | []

Here's an example of where you use this to group together all packages from the Vue monorepo:

```
{
  "packageRules": [
    {
      "matchSourceUrls": ["https://github.com/vuejs/vue"],
      "groupName": "Vue monorepo packages"
    }
  ]
}

```

For more details on supported syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/).

### packageRules.matchUpdateTypes[¶](https://docs.renovatebot.com/configuration-options/#packagerulesmatchupdatetypes)

Update types to match against (major, minor, pin, pinDigest, etc). Valid only within packageRules object.

| Name  | Value

| type  | array

| subType  | string

| allowedValues  |

```
[
  "major",
  "minor",
  "patch",
  "pin",
  "pinDigest",
  "digest",
  "lockFileMaintenance",
  "lockfileUpdate",
  "rollback",
  "bump",
  "replacement"
]

```

| parents  | packageRules

| mergeable  | true

| default  | []

Use matchUpdateTypes to match rules against types of updates. For example to apply a special label to major updates:

```
{
  "packageRules": [
    {
      "matchUpdateTypes": ["major"],
      "labels": ["UPDATE-MAJOR"]
    }
  ]
}

```

For more details on supported syntax see Renovate's [string pattern matching documentation](https://docs.renovatebot.com/string-pattern-matching/).

Warning

Packages that follow SemVer are allowed to make breaking changes in any 0.x version, even patch and minor. Check if you're using any 0.x package, and see if you need custom packageRules for it. When setting up automerge for dependencies, make sure to stop accidental automerges of 0.x versions. Read the [automerge non-major updates](https://docs.renovatebot.com/key-concepts/automerge/#automerge-non-major-updates) docs for a config example that blocks 0.x updates.

Warning

matchUpdateTypes and allowedVersions cannot be used in the same package rule.

Tokens can be configured via hostRules using the "merge-confidence" hostType:

```
{
  "hostRules": [
    {
      "hostType": "merge-confidence",
      "token": "********"
    }
  ]
}

```

### packageRules.overrideDatasource[¶](https://docs.renovatebot.com/configuration-options/#packagerulesoverridedatasource)

Override the datasource value.

| Name  | Value

| type  | string

| parents  | packageRules

| default  | null

Warning

For advanced use only! Use at your own risk!

If a particular datasource/packageName combination has a lookup problem, you may be able to fix it by changing datasource and potentially also packageName. Here is an example:

```
{
  "packageRules": [
    {
      "matchDatasources": ["docker"],
      "matchPackageNames": ["renovate/renovate"],
      "overrideDatasource": "npm",
      "overridePackageName": "renovate"
    }
  ]
}

```

overrideDatasource does not support template compilation. Be cautious as using this setting incorrectly could break all lookups.

### packageRules.overrideDepName[¶](https://docs.renovatebot.com/configuration-options/#packagerulesoverridedepname)

Override the depName value.

| Name  | Value

| type  | string

| parents  | packageRules

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Warning

For advanced use only! Use at your own risk!

Be careful using this feature because it may cause undesirable changes such as to branch names.

In Renovate terminology, packageName is the exact package name needing to be looked up on a registry, while depName is essentially the "pretty" name. For example, the packageName is docker.io/library/node while the depName might be node for short.

depName is used in PR titles as well as branch names, so changes to depName will have effects on those.

overrideDepName supports template compilation. Example:

```
{
  "packageRules": [
    {
      "matchDatasources": ["docker"],
      "overrideDepName": "{{replace 'docker.io/library/' '' depName}}"
    }
  ]
}

```

Be cautious as using this setting incorrectly could break all lookups.

### packageRules.overridePackageName[¶](https://docs.renovatebot.com/configuration-options/#packagerulesoverridepackagename)

Override the packageName value.

| Name  | Value

| type  | string

| parents  | packageRules

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Warning

For advanced use only! Use at your own risk!

See the [packageRules.overrideDatasource](https://docs.renovatebot.com/configuration-options/#packagerulesoverridedatasource) documentation for an example of use. overridePackageName supports template compilation.

Be cautious as using this setting incorrectly could break all lookups.

### packageRules.prPriority[¶](https://docs.renovatebot.com/configuration-options/#packagerulesprpriority)

Set sorting priority for PR creation. PRs with higher priority are created first, negative priority last.

| Name  | Value

| type  | integer

| allowNegative  | true

| default  | 0

| parents  | packageRules

Sometimes Renovate needs to rate limit its creation of PRs, e.g. hourly or concurrent PR limits. By default, Renovate sorts/prioritizes based on the update type, going from smallest update to biggest update. Renovate creates update PRs in this order:

- pinDigest
- pin
- digest
- patch
- minor
- major

If you have dependencies that are more or less important than others then you can use the prPriority field for PR sorting. The default value is 0, so setting a negative value will make dependencies sort last, while higher values sort first.

Here's an example of how you would define PR priority so that devDependencies are raised last and react is raised first:

```
{
  "packageRules": [
    {
      "matchDepTypes": ["devDependencies"],
      "prPriority": -1
    },
    {
      "matchPackageNames": ["react"],
      "prPriority": 5
    }
  ]
}

```

### packageRules.replacementName[¶](https://docs.renovatebot.com/configuration-options/#packagerulesreplacementname)

The name of the new dependency that replaces the old deprecated dependency.

| Name  | Value

| type  | string

| parents  | packageRules

| default  | null

This config option only works with some managers. We're working to support more managers, subscribe to issue [renovatebot/renovate#24883](https://github.com/renovatebot/renovate/discussions/24883) to follow our progress.

Managers which do not support replacement:

- bazel
- git-submodules
- gomod
- gradle
- homebrew
- regex
- sbt

Use the replacementName config option to set the name of a replacement package.

Can be used in combination with replacementVersion.

You can suggest a new community package rule by editing [the replacements.json file on the Renovate repository](https://github.com/renovatebot/renovate/blob/main/lib/data/replacements.json) and opening a pull request.

Note

Replacement updates will never be grouped.
Lock file maintenance will never be grouped with other dependency updates.

### packageRules.replacementNameTemplate[¶](https://docs.renovatebot.com/configuration-options/#packagerulesreplacementnametemplate)

Controls what the replacement package name.

| Name  | Value

| type  | string

| default  | "{{{packageName}}}"

| parents  | packageRules

| supportsTemplating  | true

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Note

replacementName will take precedence if used within the same package rule.

Use the replacementNameTemplate config option to control the replacement name.

Use the triple brace {{{ }}} notation to avoid Handlebars escaping any special characters.

For example, the following package rule can be used to replace the registry for docker images:

```
{
  "packageRules": [
    {
      "matchDatasources": ["docker"],
      "matchPackageNames": ["docker.io/**"],
      "replacementNameTemplate": "{{{replace 'docker\\.io/' 'ghcr.io/' packageName}}}"
    }
  ]
}

```

Or, to add a registry prefix to any docker images that do not contain an explicit registry:

```
{
  "packageRules": [
    {
      "description": "official images",
      "matchDatasources": ["docker"],
      "matchPackageNames": ["/^[a-z-]+$/"],
      "replacementNameTemplate": "some.registry.org/library/{{{packageName}}}"
    },
    {
      "description": "non-official images",
      "matchDatasources": ["docker"],
      "matchPackageNames": ["/^[a-z-]+/[a-z-]+$/"],
      "replacementNameTemplate": "some.registry.org/{{{packageName}}}"
    }
  ]
}

```

### packageRules.replacementVersion[¶](https://docs.renovatebot.com/configuration-options/#packagerulesreplacementversion)

The version of the new dependency that replaces the old deprecated dependency.

| Name  | Value

| type  | string

| parents  | packageRules

| default  | null

This config option only works with some managers. We're working to support more managers, subscribe to issue [renovatebot/renovate#14149](https://github.com/renovatebot/renovate/issues/14149) to follow our progress. For a list of managers which do not support replacement read the replacementName config option docs.

Use the replacementVersion config option to set the version of a replacement package. Must be used with replacementName. For example to replace the npm package jade with version 2.0.0 of the package pug:

```
{
  "packageRules": [
    {
      "matchDatasources": ["npm"],
      "matchPackageNames": ["jade"],
      "replacementName": "pug",
      "replacementVersion": "2.0.0"
    }
  ]
}

```

### packageRules.replacementVersionTemplate[¶](https://docs.renovatebot.com/configuration-options/#packagerulesreplacementversiontemplate)

Template field for the version of the new dependency that replaces the old deprecated dependency.

| Name  | Value

| type  | string

| parents  | packageRules

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Note

replacementVersion will take precedence if used within the same package rule.

Use the replacementVersionTemplate config option to control the replacement version.

For example, the following package rule can be used to replace version with major-only version (17.0.1 → 17):

```
{
  "packageRules": [
    {
      "matchPackageNames": ["dummy"],
      "replacementVersionTemplate": "{{ lookup (split currentValue '.') 0 }}"
    }
  ]
}

```

### packageRules.sourceDirectory[¶](https://docs.renovatebot.com/configuration-options/#packagerulessourcedirectory)

The source directory in which the package is present at its source.

| Name  | Value

| type  | string

| parents  | packageRules

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Use this field to set the directory in which the package is present at the source of the package.

Setting the source directory for the kube-prometheus package from bitnami charts repo

```
{
  "packageRules": [
    {
      "matchPackageNames": ["dummy"],
      "sourceUrl": "https://github.com/bitnami/charts",
      "sourceDirectory": "bitnami/kube-prometheus"
    }
  ]
}

```

Note

sourceDirectory should be only be configured along with sourceUrl.

### packageRules.sourceUrl[¶](https://docs.renovatebot.com/configuration-options/#packagerulessourceurl)

The source URL of the package.

| Name  | Value

| type  | string

| parents  | packageRules

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Use this field to set the source URL for a package, including overriding an existing one. Source URLs are necessary to link to the source of the package and in order to look up changelogs.

The sourceUrl field supports template compilation, allowing you to dynamically construct URLs based on package information.

Setting the source URL for the dummy package

```
{
  "packageRules": [
    {
      "matchPackageNames": ["dummy"],
      "sourceUrl": "https://github.com/org/dummy"
    }
  ]
}

```

Using templates to construct source URLs for OpenTofu providers

```
{
  "packageRules": [
    {
      "matchDatasources": ["terraform-provider"],
      "registryUrls": ["https://registry.opentofu.org"],
      "sourceUrl": "https://github.com/{{replace '/' '/terraform-provider-' packageName}}"
    }
  ]
}

```

In the example above, a package name like hashicorp/aws will be transformed to https://github.com/hashicorp/terraform-provider-aws.

## patch[¶](https://docs.renovatebot.com/configuration-options/#patch)

Configuration to apply when an update type is patch.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| mergeable  | true

| env  | RENOVATE_PATCH

Add to this object if you wish to define rules that apply only to patch updates.

## pin[¶](https://docs.renovatebot.com/configuration-options/#pin)

Configuration to apply when an update type is pin.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| default  |

```
{
  "rebaseWhen": "behind-base-branch",
  "groupName": "Pin Dependencies",
  "groupSlug": "pin-dependencies",
  "commitMessageAction": "Pin",
  "group": {"commitMessageTopic": "dependencies", "commitMessageExtra": ""}
}

```

| mergeable  | true

| env  | RENOVATE_PIN

Add to this object if you wish to define rules that apply only to PRs that pin dependencies.

## pinDigest[¶](https://docs.renovatebot.com/configuration-options/#pindigest)

Configuration to apply when pinning a digest (no change in tag/version).

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| default  |

```
{
  "groupName": "Pin Dependencies",
  "groupSlug": "pin-dependencies",
  "commitMessageAction": "Pin",
  "group": {"commitMessageTopic": "dependencies", "commitMessageExtra": ""}
}

```

| mergeable  | true

| env  | RENOVATE_PIN_DIGEST

Add to this object if you wish to define rules that apply only to PRs that pin digests.

## pinDigests[¶](https://docs.renovatebot.com/configuration-options/#pindigests)

Whether to add digests to Dockerfile source images.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | false

| cli  | --pin-digests

| env  | RENOVATE_PIN_DIGESTS

If enabled Renovate will pin Docker images or GitHub Actions by means of their SHA256 digest and not only by tag so that they are immutable.

## platformAutomerge[¶](https://docs.renovatebot.com/configuration-options/#platformautomerge)

Controls if platform-native auto-merge is used.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | true

| supportedPlatforms  | azure, bitbucket-server, forgejo, gitea, github, gitlab

| cli  | --platform-automerge

| env  | RENOVATE_PLATFORM_AUTOMERGE

Note

If you use the default platformAutomerge=true then you should enable your Git hosting platform's capabilities to enforce test passing before PR merge. If you don't do this, the platform might merge Renovate PRs even if the repository's tests haven't started, are in still in progress, or possibly even when they have failed. On GitHub this is called "Require status checks before merging", which you can find in the "Branch protection rules" section of the settings for your repository. [GitHub docs, about protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches) [GitHub docs, require status checks before merging](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches#require-status-checks-before-merging) If you're using another platform, search their documentation for a similar feature.

Note

If you mark renovate/artifacts as a required status check in your branch protection rules, you must also set [statusCheckWhen.artifactError](https://docs.renovatebot.com/configuration-options/#statuscheckwhen) to "always". The default "failed" only sets the check when artifact updates fail, so a required check rule would block every PR that has no artifact errors (the check would never appear). Setting it to "always" makes Renovate report green when there are no errors and red when there are, which is what branch protection requires.

If you have enabled automerge and set automergeType=pr in the Renovate config, then leaving platformAutomerge as true speeds up merging via the platform's native automerge functionality.

Where supported, platform-native automerge uses [automergeStrategy](https://docs.renovatebot.com/configuration-options/#automergestrategy) to select the merge method.

On Bitbucket Server, GitHub and GitLab, Renovate re-enables the PR for platform-native automerge whenever it's rebased.

platformAutomerge will configure PRs to be merged after all (if any) branch policies have been met. This option is available for Azure, Bitbucket Server, Forgejo, Gitea, GitHub and GitLab. It falls back to Renovate-based automerge if the platform-native automerge is not available.

You can also fine-tune the behavior by setting packageRules if you want to use it selectively (e.g. per-package).

Note that the outcome of rebaseWhen=auto can differ when platformAutomerge=true. Normally when you set rebaseWhen=auto Renovate rebases any branch that's behind the base branch automatically, and some people rely on that. This behavior is no longer guaranteed when platformAutomerge is true because the platform might automerge a branch which is not up-to-date. For example, GitHub might automerge a Renovate branch even if it's behind the base branch at the time.

Please check platform specific docs for version requirements.

To learn how to use GitHub's Merge Queue feature with Renovate, read our [GitHub Merge Queue](https://docs.renovatebot.com/key-concepts/automerge/#github-merge-queue) docs.

## platformCommit[¶](https://docs.renovatebot.com/configuration-options/#platformcommit)

Use platform API to perform commits instead of using Git directly.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "auto"

| allowedValues  |

```
["auto", "disabled", "enabled"]

```

| supportedPlatforms  | github

| cli  | --platform-commit

| env  | RENOVATE_PLATFORM_COMMIT

Only use this option if you run Renovate as a [GitHub App](https://docs.github.com/en/developers/apps/getting-started-with-apps/about-apps). It does not apply when you use a Personal Access Token as credential.

When platformCommit is enabled, Renovate will create commits with GitHub's API instead of using git directly. This way Renovate can use GitHub's [Commit signing support for bots and other GitHub Apps](https://github.blog/2019-08-15-commit-signing-support-for-bots-and-other-github-apps/) feature.

Note

When using platform commits, GitHub determines the git author string to use and Renovate's own gitAuthor is ignored.

## postUpdateOptions[¶](https://docs.renovatebot.com/configuration-options/#postupdateoptions)

Enable post-update options to be run after package/artifact updating.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| allowedValues  |

```
[
  "bundlerConservative",
  "composerWithAll",
  "composerNoMinimalChanges",
  "dotnetWorkloadRestore",
  "gomodMassage",
  "gomodTidy",
  "gomodTidy1.17",
  "gomodTidyAll",
  "gomodTidyE",
  "gomodUpdateImportPaths",
  "gomodSkipVendor",
  "gomodVendor",
  "goGenerate",
  "helmUpdateSubChartArchives",
  "kustomizeInflateHelmCharts",
  "npmDedupe",
  "npmInstallTwice",
  "pnpmDedupe",
  "yarnDedupeFewer",
  "yarnDedupeHighest"
]

```

| mergeable  | true

| default  | []

### bundlerConservative[¶](https://docs.renovatebot.com/configuration-options/#bundlerconservative)

Enable conservative mode for bundler (Ruby dependencies). This will only update the immediate dependency in the lockfile instead of all subdependencies.

### composerNoMinimalChanges[¶](https://docs.renovatebot.com/configuration-options/#composernominimalchanges)

Run composer update with no --minimal-changes flag (does not affect lock file maintenance, which will never use --minimal-changes).

### composerWithAll[¶](https://docs.renovatebot.com/configuration-options/#composerwithall)

Run composer update with --with-all-dependencies flag instead of the default --with-dependencies.

### dotnetWorkloadRestore[¶](https://docs.renovatebot.com/configuration-options/#dotnetworkloadrestore)

Run dotnet workload restore before dotnet restore commands.

### goGenerate[¶](https://docs.renovatebot.com/configuration-options/#gogenerate)

Run go generate ./.... This will then commit any files which were added or modified by running go generate.

Note this will not install any other tools as part of the process. See [Go Tool](https://tip.golang.org/doc/go1.24#tools) usage for how to incorporate these as part of your build process.

This runs after vendoring, if vendoring was required.

Note

In order for this option to function, the global configuration option [allowedUnsafeExecutions](https://docs.renovatebot.com/self-hosted-configuration/#allowedunsafeexecutions) must include goGenerate.

### gomodMassage[¶](https://docs.renovatebot.com/configuration-options/#gomodmassage)

Enable massaging replace directives before calling go commands.

### gomodSkipVendor[¶](https://docs.renovatebot.com/configuration-options/#gomodskipvendor)

Never run go mod vendor after Go module updates.

### gomodTidy[¶](https://docs.renovatebot.com/configuration-options/#gomodtidy)

Run go mod tidy after Go module updates. This is implicitly enabled for major module updates when gomodUpdateImportPaths is enabled.

### gomodTidy1.17[¶](https://docs.renovatebot.com/configuration-options/#gomodtidy117)

Run go mod tidy -compat=1.17 after Go module updates.

### gomodTidyAll[¶](https://docs.renovatebot.com/configuration-options/#gomodtidyall)

After running go mod tidy on the updated module, also run it on every other go.mod which references that module through a local replace directive, in dependency order. Use this in Go monorepos, where an update to a shared module must also reach the go.sum files of the modules which depend on it. Implies gomodTidy, and needs Go 1.20 or later. Avoid combining this with gomodMassage, which comments out the relative replace directives that this option follows.

### gomodTidyE[¶](https://docs.renovatebot.com/configuration-options/#gomodtidye)

Run go mod tidy -e after Go module updates.

### gomodUpdateImportPaths[¶](https://docs.renovatebot.com/configuration-options/#gomodupdateimportpaths)

Update source import paths on major module updates, using [mod](https://github.com/marwan-at-work/mod).

### gomodVendor[¶](https://docs.renovatebot.com/configuration-options/#gomodvendor)

Always run go mod vendor after Go module updates even if vendor files aren't detected.

### helmUpdateSubChartArchives[¶](https://docs.renovatebot.com/configuration-options/#helmupdatesubchartarchives)

Update subchart archives in the /charts folder.

### kustomizeInflateHelmCharts[¶](https://docs.renovatebot.com/configuration-options/#kustomizeinflatehelmcharts)

Inflate updated helm charts referenced in the kustomization.

### npmDedupe[¶](https://docs.renovatebot.com/configuration-options/#npmdedupe)

Run npm install with --prefer-dedupe for npm >= 7 or npm dedupe after package-lock.json update for npm <= 6.

### npmInstallTwice[¶](https://docs.renovatebot.com/configuration-options/#npminstalltwice)

Run npm install commands twice to work around bugs where npm generates invalid lock files if run only once.

During lock file maintenance, Renovate always runs npm install twice, even without this option, because regenerating a lock file from scratch is known to need a second pass.

### pnpmDedupe[¶](https://docs.renovatebot.com/configuration-options/#pnpmdedupe)

Run pnpm dedupe after pnpm-lock.yaml updates.

### yarnDedupeFewer[¶](https://docs.renovatebot.com/configuration-options/#yarndedupefewer)

Run yarn-deduplicate --strategy fewer after yarn.lock updates.

### yarnDedupeHighest[¶](https://docs.renovatebot.com/configuration-options/#yarndedupehighest)

Run yarn-deduplicate --strategy highest (yarn dedupe --strategy highest for Yarn >=2.2.0) after yarn.lock updates.

## postUpgradeTasks[¶](https://docs.renovatebot.com/configuration-options/#postupgradetasks)

Post-upgrade tasks that are executed before a commit is made by Renovate.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | object

| default  |

```
{
  "commands": [],
  "fileFilters": [],
  "executionMode": "update",
  "installTools": {}
}

```

| cli  | --post-upgrade-tasks

| env  | RENOVATE_POST_UPGRADE_TASKS

Post-upgrade tasks are commands that are executed by Renovate after a dependency has been updated but before the commit is created. The intention is to run any other command line tools that would modify existing files or generate new files when a dependency changes.

Post-upgrade tasks are blocked by default for security reasons, so the admin of Renovate needs to choose whether to allow specific commands or any commands to be run - it depends on how much they trust their users in repos.

Tip

In Mend-hosted Renovate apps, commands remain blocked by default, but [depending on your plan](https://docs.renovatebot.com/mend-hosted/faq/#how-can-i-run-arbitrary-commands-through-postupgradetasks), you have more control over this.

Each command must match at least one of the patterns defined in allowedCommands (a global-only configuration option) in order to be executed. If the list of allowed tasks is empty then no tasks will be executed.

e.g.

```
{
  "postUpgradeTasks": {
    "commands": ["tslint --fix"],
    "fileFilters": ["yarn.lock", "**/*.js"],
    "executionMode": "update"
  }
}

```

The postUpgradeTasks configuration consists of four fields:

### postUpgradeTasks.commands[¶](https://docs.renovatebot.com/configuration-options/#postupgradetaskscommands)

A list of post-upgrade commands that are executed before a commit is made by Renovate.

| Name  | Value

| type  | array

| subType  | string

| parents  | postUpgradeTasks

| supportsTemplating  | true

| env  | RENOVATE_COMMANDS

| default  | []

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

A list of commands that are executed after Renovate has updated a dependency but before the commit is made.

You can use Handlebars templating in these commands. They will be compiled prior to the comparison against [allowedCommands](https://docs.renovatebot.com/self-hosted-configuration/#allowedcommands).

Note

Do not use git add in your commands to add new files to be tracked, add them by including them in your [postUpgradeTasks.fileFilters](https://docs.renovatebot.com/configuration-options/#postupgradetasksfilefilters) instead.

### postUpgradeTasks.dataFileTemplate[¶](https://docs.renovatebot.com/configuration-options/#postupgradetasksdatafiletemplate)

A template to create post-upgrade command data file from.

| Name  | Value

| type  | string

| parents  | postUpgradeTasks

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

A template to create data file from. The template uses the same format as commands. The data file is created as a temporary file and the path to the data file is stored in the RENOVATE_POST_UPGRADE_COMMAND_DATA_FILE environment variable avaliable to each post-upgrade command. The primary purpose of the data file is to store some update information in a file which would be consumed from a post-upgrade command. This is particularly useful if a post-upgrade command needs to have a long line of arguments. Example:

```
{
  "postUpgradeTasks": {
    "commands": [
      "my-script.py --data-file \"$RENOVATE_POST_UPGRADE_COMMAND_DATA_FILE\""
    ],
    "dataFileTemplate": "[{{#each upgrades}}{\"depName\": \"{{{depName}}}\", \"currentValue\": \"{{{currentValue}}}\", \"newValue\": \"{{{newValue}}}\"}{{#unless @last}},{{\/unless}}{{\/each}}]"
  }
}

```

Note

dataFileTemplate is ignored if there is no commands configured.

### postUpgradeTasks.executionMode[¶](https://docs.renovatebot.com/configuration-options/#postupgradetasksexecutionmode)

Controls when the post upgrade tasks run: on every update, or once per upgrade branch.

| Name  | Value

| type  | string

| parents  | postUpgradeTasks

| allowedValues  |

```
["update", "branch"]

```

| default  | "update"

| env  | RENOVATE_EXECUTION_MODE

Defaults to update, but can also be set to branch. This sets the level the postUpgradeTask runs on, if set to update the postUpgradeTask will be executed for every dependency on the branch. If set to branch the postUpgradeTask is executed for the whole branch.

### postUpgradeTasks.fileFilters[¶](https://docs.renovatebot.com/configuration-options/#postupgradetasksfilefilters)

Files that match the glob pattern will be committed after running a post-upgrade task.

| Name  | Value

| type  | array

| subType  | string

| parents  | postUpgradeTasks

| default  |

```
["**/*"]

```

| env  | RENOVATE_FILE_FILTERS

A list of glob-style matchers that determine which files will be included in the final commit made by Renovate. Dotfiles are included.

Optional field which defaults to any non-ignored file in the repo (**/* glob pattern). Specify a custom value for this if you wish to exclude certain files which are modified by your postUpgradeTasks and you don't want committed.

### postUpgradeTasks.installTools[¶](https://docs.renovatebot.com/configuration-options/#postupgradetasksinstalltools)

Install tools before executing commands

| Name  | Value

| type  | object

| parents  | postUpgradeTasks

| mergeable  | false

| freeChoice  | true

| env  | RENOVATE_INSTALL_TOOLS

Whether to install any additional tools dynamically before executing the commands.

The possible tool names that are known by [Containerbase](https://github.com/containerbase/base) are:

- apm
- bazelisk
- bun
- bundler
- cocoapods
- composer
- conan
- copier
- corepack
- deno
- devbox
- dotnet
- erlang
- elixir
- flux
- gh
- gleam
- golang
- gradle
- hashin
- helm
- helmfile
- java
- jb
- kustomize
- maven
- mise
- nix
- node
- npm
- paket
- pdm
- php
- pip-tools
- pipenv
- pnpm
- pixi
- poetry
- python
- ruby
- rust
- uv
- yarn
- yarn-slim
- dart
- flutter
- vendir

Note

When using [binarySource=global](https://docs.renovatebot.com/self-hosted-configuration/#binarysource), the installTools options do not take effect.

For example:

Adding a tool requirement

```
{
  "postUpgradeTasks": {
    "commands": ["make generate"],
    "installTools": {
      "golang": {}
    }
  }
}

```

Utilising constraints for tool requirements

```
{
  "constraints": {
    "node": "^18.0.0 || >=20.0.0"
  },
  "postUpgradeTasks": {
    "commands": ["npm install", "npm run update-readme"],
    "installTools": {
      "node": {}
    }
  }
}

```

Note

The tool objects inside installTools currently do not expose any additional configurability.

### postUpgradeTasks.workingDirTemplate[¶](https://docs.renovatebot.com/configuration-options/#postupgradetasksworkingdirtemplate)

A template describing the working directory in which post-upgrade tasks should be executed.

| Name  | Value

| type  | string

| parents  | postUpgradeTasks

| supportsTemplating  | true

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

A template describing the working directory in which the commands should be executed, relative to the repository root. If the template evaluates to a false value, then the command will be executed from the root of the repository. Example:

```
{
  "postUpgradeTasks": {
    "commands": ["my-script.py"],
    "workingDirTemplate": "{{{packageFileDir}}}"
  }
}

```

## prBodyColumns[¶](https://docs.renovatebot.com/configuration-options/#prbodycolumns)

List of columns to use in PR bodies.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| default  |

```
["Package", "Type", "Update", "Change", "Pending"]

```

| cli  | --pr-body-columns

| env  | RENOVATE_PR_BODY_COLUMNS

Use this array to provide a list of column names you wish to include in the PR tables.

Adding the package file name to the table

```
{
  "prBodyColumns": [
    "Package",
    "Update",
    "Type",
    "New value",
    "Package file",
    "References"
  ]
}

```

Note

"Package file" is predefined in the default prBodyDefinitions object so does not require a definition before it can be used.

## prBodyDefinitions[¶](https://docs.renovatebot.com/configuration-options/#prbodydefinitions)

Table column definitions to use in PR tables.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | object

| freeChoice  | true

| mergeable  | true

| default  |

```
{
  "Package": "{{{depNameLinked}}}{{#if newName}}{{#unless (equals depName newName)}} → {{{newNameLinked}}}{{/unless}}{{/if}}",
  "Type": "{{{depType}}}",
  "Update": "{{{updateType}}}",
  "Current value": "{{{currentValue}}}",
  "New value": "{{{newValue}}}",
  "Change": "`{{{displayFrom}}}` → `{{{displayTo}}}`",
  "Pending": "{{{displayPending}}}",
  "References": "{{{references}}}",
  "Package file": "{{{packageFile}}}",
  "Age": "{{#if newVersion}}![age](https://developer.mend.io/api/mc/badges/age/{{datasource}}/{{replace '/' '%2f' packageName}}/{{{newVersion}}}?slim=true){{/if}}",
  "Adoption": "{{#if newVersion}}![adoption](https://developer.mend.io/api/mc/badges/adoption/{{datasource}}/{{replace '/' '%2f' packageName}}/{{{newVersion}}}?slim=true){{/if}}",
  "Passing": "{{#if newVersion}}![passing](https://developer.mend.io/api/mc/badges/compatibility/{{datasource}}/{{replace '/' '%2f' packageName}}/{{{currentVersion}}}/{{{newVersion}}}?slim=true){{/if}}",
  "Confidence": "{{#if newVersion}}![confidence](https://developer.mend.io/api/mc/badges/confidence/{{datasource}}/{{replace '/' '%2f' packageName}}/{{{currentVersion}}}/{{{newVersion}}}?slim=true){{/if}}"
}

```

| cli  | --pr-body-definitions

| env  | RENOVATE_PR_BODY_DEFINITIONS

You can configure this object to either:

- modify the template for an existing table column in PR bodies, or
- add a definition for a new/additional column.

Modifying the default value for the Package column to put it inside a code block

```
{
  "prBodyDefinitions": {
    "Package": "`{{{depName}}}`"
  }
}

```

Adding a custom Sourcegraph column definition

```
{
  "prBodyDefinitions": {
    "Sourcegraph": "[![code search for \"{{{depName}}}\"](https://sourcegraph.com/search/badge?q=repo:%5Egithub%5C.com/{{{repository}}}%24+case:yes+-file:package%28-lock%29%3F%5C.json+{{{depName}}}&label=matches)](https://sourcegraph.com/search?q=repo:%5Egithub%5C.com/{{{repository}}}%24+case:yes+-file:package%28-lock%29%3F%5C.json+{{{depName}}})"
  },
  "prBodyColumns": [
    "Package",
    "Update",
    "New value",
    "References",
    "Sourcegraph"
  ]
}

```

Tip

Columns must also be included in the prBodyColumns array in order to be used, so that's why it's included above in the example.

## prBodyHeadingDefinitions[¶](https://docs.renovatebot.com/configuration-options/#prbodyheadingdefinitions)

Table header definitions to use in PR tables.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | object

| freeChoice  | true

| mergeable  | true

| default  |

```
{
  "Age": "[Age](https://docs.renovatebot.com/merge-confidence/)",
  "Adoption": "[Adoption](https://docs.renovatebot.com/merge-confidence/)",
  "Passing": "[Passing](https://docs.renovatebot.com/merge-confidence/)",
  "Confidence": "[Confidence](https://docs.renovatebot.com/merge-confidence/)"
}

```

| cli  | --pr-body-heading-definitions

| env  | RENOVATE_PR_BODY_HEADING_DEFINITIONS

You can configure this object to modify the table headers present in the PR body. This can include Markdown or any other syntax that the platform supports.

Modifying the default value for the Package and Age table headers

```
{
  "prBodyHeadingDefinitions": {
    "Package": "📦 Package",
    "Age": "[Age](https://docs.renovatebot.com/merge-confidence)"
  },
  "prBodyColumns": ["Package", "Age"]
}

```

Tip

Columns must also be included in the prBodyColumns array in order to be used, so that's why it's included above in the example.

Note

Templating is not supported for this option.

## prBodyNotes[¶](https://docs.renovatebot.com/configuration-options/#prbodynotes)

List of extra notes or templates to include in the Pull Request body.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| mergeable  | true

| supportsTemplating  | true

| cli  | --pr-body-notes

| env  | RENOVATE_PR_BODY_NOTES

| default  | []

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

Use this field to add custom content inside PR bodies, including conditionally.

Adding an extra Warning to major updates

```
{
  "prBodyNotes": ["{{#if isMajor}}:warning: MAJOR MAJOR MAJOR :warning:{{/if}}"]
}

```

## prBodyTemplate[¶](https://docs.renovatebot.com/configuration-options/#prbodytemplate)

Pull Request body template. Controls which sections are rendered in the body of the pull request.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "{{{header}}}{{{table}}}{{{warnings}}}{{{notes}}}{{{changelogs}}}{{{configDescription}}}{{{controls}}}{{{footer}}}"

| supportsTemplating  | true

| env  | RENOVATE_PR_BODY_TEMPLATE

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

The available sections are:

- header
- table
- warnings
- notes
- changelogs
- configDescription
- controls
- footer

## prConcurrentLimit[¶](https://docs.renovatebot.com/configuration-options/#prconcurrentlimit)

Limit to a maximum of x concurrent branches/PRs. 0 means no limit.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| default  | 10

| cli  | --pr-concurrent-limit

| env  | RENOVATE_PR_CONCURRENT_LIMIT

This setting - if enabled - limits Renovate to a maximum of x concurrent PRs open at any time.

This limit is enforced on a per-repository basis.

Note

Renovate always creates security PRs, even if the concurrent PR limit is already reached. Security PRs have [SECURITY] in their PR title.

## prCreation[¶](https://docs.renovatebot.com/configuration-options/#prcreation)

When to create the PR for a branch.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| allowedValues  |

```
["immediate", "not-pending", "status-success", "approval"]

```

| default  | "immediate"

| cli  | --pr-creation

| env  | RENOVATE_PR_CREATION

This setting tells Renovate when to create PRs:

- immediate (default): Renovate creates PRs immediately after creating the corresponding branch
- not-pending: Renovate waits until status checks have completed (passed or failed) before raising the PR
- status-success: Renovate only creates PRs if/when the the test pass
- approval: Renovate creates branches for updates immediately, but creates the PR after getting Dependency Dashboard approval

When prCreation is set to immediate, you'll get a Pull Request and possible associated notification right away when a new update is available. You'll have to wait until the checks have been performed, before you can decide if you want to merge the PR.

When prCreation is set to not-pending, Renovate creates the PR only once all tests have passed or failed. When you get the PR notification, you can take action immediately, as you have the full test results. If there are no checks associated, Renovate will create the PR once [prNotPendingHours](https://docs.renovatebot.com/configuration-options/#prnotpendinghours) have elapsed since the branch's latest commit (default: 25 hours).

not-pending can stall PR creation when there are no branch-level checks

A branch with no external status checks is reported as pending, so not-pending never sees checks "complete" and instead relies on the prNotPendingHours fallback. That fallback is measured from the branch's latest commit, not from when the branch was created, so anything that rewrites the branch (rebase, re-lock, etc.) restarts the timer. On repositories whose branches are rebased or re-locked more often than prNotPendingHours, PRs can be deferred indefinitely. This commonly happens when CI only runs on pull_request events, so bare Renovate branches carry no checks. If you rely on minimumReleaseAge/internalChecksFilter for the "wait" behavior, set [internalChecksAsSuccess](https://docs.renovatebot.com/configuration-options/#internalchecksassuccess)=true so the green internal stability check counts as success and the branch can go green without external checks, or use prCreation=immediate instead.

When prCreation is set to status-success, Renovate creates the PR only if all tests have passed. When a branch remains without PR due to a failing test: select the corresponding PR from the Dependency Dashboard, and push your fixes to the branch.

When prCreation is set to approval, Renovate creates the PR only when approved via the Dependency Dashboard. Renovate still creates the branch immediately.

Note

For all cases of non-immediate PR creation, Renovate doesn't run instantly once tests complete. Instead, Renovate create the PR on its next run after the relevant tests have completed, so there will be some delay.

Warning

If you set prCreation=approval you must not use dependencyDashboardApproval=true!

## prFooter[¶](https://docs.renovatebot.com/configuration-options/#prfooter)

Text added here will be placed last in the PR body, with a divider separator before it.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "This PR has been generated by [Mend Renovate CLI](https://github.com/renovatebot/renovate)."

| supportsTemplating  | true

| cli  | --pr-footer

| env  | RENOVATE_PR_FOOTER

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

## prHeader[¶](https://docs.renovatebot.com/configuration-options/#prheader)

Text added here will be placed first in the PR body.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| supportsTemplating  | true

| cli  | --pr-header

| env  | RENOVATE_PR_HEADER

| default  | null

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

## prHourlyLimit[¶](https://docs.renovatebot.com/configuration-options/#prhourlylimit)

Rate limit PRs to maximum x created per hour. 0 means no limit.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| default  | 2

| cli  | --pr-hourly-limit

| env  | RENOVATE_PR_HOURLY_LIMIT

This config option slows down the rate at which Renovate creates PRs.

Slowing Renovate down can be handy when you're onboarding a repository with a lot of dependencies. What may happen if you don't set a prHourlyLimit:

- Renovate creates an Onboarding PR
- You merge the onboarding PR to activate Renovate
- Renovate creates a "Pin Dependencies" PR (if needed)
- You merge the "Pin Dependencies" PR
- Renovate creates every single upgrade PR needed, which can be a lot

The above may cause:

- Renovate's PRs to overwhelm your CI systems
- a lot of test runs, because branches are rebased each time you merge a PR

To prevent these problems you can set prHourlyLimit to a value like 1 or 2. Renovate will only create that many PRs within each hourly period (:00 through :59). You still get all the PRs in a reasonable time, perhaps over a day or so. Now you can merge the PRs at a do-able rate, once the tests pass.

Tip

The prHourlyLimit setting does not limit the number of concurrently open PRs, only the rate at which PRs are created. The prHourlyLimit setting is enforced on a per-repository basis.

## prNotPendingHours[¶](https://docs.renovatebot.com/configuration-options/#prnotpendinghours)

Timeout in hours for when prCreation=not-pending.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| default  | 25

| cli  | --pr-not-pending-hours

| env  | RENOVATE_PR_NOT_PENDING_HOURS

If you configure prCreation=not-pending, then Renovate will wait until tests are non-pending (all pass or at least one fails) before creating PRs. However there are cases where PRs may remain in pending state forever, e.g. absence of tests or status checks that are configure to pending indefinitely. This is why we configured an upper limit for how long we wait until creating a PR.

Note

If the option minimumReleaseAge is non-zero then Renovate disables the prNotPendingHours functionality.

## prTitle[¶](https://docs.renovatebot.com/configuration-options/#prtitle)

Pull Request title template. Inherits from commitMessage if null.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| supportsTemplating  | true

| env  | RENOVATE_PR_TITLE

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

This feature has been deprecated

Direct editing of prTitle is now deprecated. Instead use config options like commitMessageAction, commitMessageExtra, and so on, as they will be passed through to prTitle.

The PR title is important for some of Renovate's matching algorithms (e.g. determining whether to recreate a PR or not) so ideally don't modify it much.

## prTitleStrict[¶](https://docs.renovatebot.com/configuration-options/#prtitlestrict)

Whether to bypass appending extra context to the Pull Request title.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| env  | RENOVATE_PR_TITLE_STRICT

This feature is flagged as experimental

Experimental features might be changed or even removed at any time.

There are certain scenarios where the default behavior appends extra context to the PR title.

These scenarios include if a baseBranch or if there is a grouped update and either separateMajorMinor or separateMinorPatch is true.

Using this option allows you to skip these default behaviors and use other templating methods to control the format of the PR title.

## printConfig[¶](https://docs.renovatebot.com/configuration-options/#printconfig)

If enabled, Renovate logs the fully resolved config for each repository, plus the fully resolved presets.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | false

| cli  | --print-config

| env  | RENOVATE_PRINT_CONFIG

This option is useful for troubleshooting, particularly if using presets. e.g. run renovate foo/bar --print-config > config.log and the fully-resolved config will be included in the log file.

## pruneBranchAfterAutomerge[¶](https://docs.renovatebot.com/configuration-options/#prunebranchafterautomerge)

Set to true to enable branch pruning after automerging.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | true

| cli  | --prune-branch-after-automerge

| env  | RENOVATE_PRUNE_BRANCH_AFTER_AUTOMERGE

By default Renovate deletes, or "prunes", the branch after automerging. Set pruneBranchAfterAutomerge to false to keep the branch after automerging.

## pruneStaleBranches[¶](https://docs.renovatebot.com/configuration-options/#prunestalebranches)

Set to false to disable pruning stale branches.

| Name  | Value

| scopes  | repo

| type  | boolean

| default  | true

| cli  | --prune-stale-branches

| env  | RENOVATE_PRUNE_STALE_BRANCHES

By default, Renovate will "prune" any of its own branches/PRs which it thinks are no longer needed. Such branches are referred to as "stale", and may be the result of Open, Merged, or Closed/Ignored PRs. It usually doesn't know why they're there, instead it simply knows that it has no need for them.

If a branch appears stale but has been modified by a different git author, then Renovate won't delete the branch or autoclose any associated PR. Instead, it will update the title to append " - abandoned" plus add a comment noting that autoclosing is skipped. Read the [Abandoned PRs](https://docs.renovatebot.com/key-concepts/pull-requests/#abandoned-prs) docs to learn more.

If a branch appears stale and hasn't been modified, then:

- If an Open PR exist for the branch, then Renovate will rename the PR to append " - autoclosed" before closing/abandoning it
- Renovate will delete the branch

You can configure pruneStaleBranches=false to disable deleting orphan branches and autoclosing PRs, but then you will be responsible for such branch/PR "cleanup" so it is not recommended.

## rangeStrategy[¶](https://docs.renovatebot.com/configuration-options/#rangestrategy)

Determines how to modify or update existing ranges.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | string

| default  | "auto"

| allowedValues  |

```
["auto", "pin", "bump", "replace", "widen", "update-lockfile", "in-range-only"]

```

Behavior:

- auto = Renovate decides (this will be done on a manager-by-manager basis)
- pin = convert ranges to exact versions, e.g. ^1.0.0 → 1.1.0
- bump = e.g. bump the range even if the new version satisfies the existing range, e.g. ^1.0.0 → ^1.1.0
- replace = Replace the range with a newer one if the new version falls outside it, and update nothing otherwise
- widen = Widen the range with newer one, e.g. ^1.0.0 → ^1.0.0 || ^2.0.0
- update-lockfile = Update the lock file when in-range updates are available, otherwise replace for updates out of range. Works for bundler, cargo, composer, gleam, npm, yarn, pnpm, terraform, poetry and uv so far
- in-range-only = Update the lock file when in-range updates are available, ignore package file updates

Renovate's "auto" strategy works like this for npm:

- Widen peerDependencies
- If an existing range already ends with an "or" operator like "^1.0.0 || ^2.0.0", then Renovate widens it into "^1.0.0 || ^2.0.0 || ^3.0.0"
- Otherwise, if the update is outside the existing range, Renovate replaces the range. So "^2.0.0" is replaced by "^3.0.0"
- Finally, if the update is in-range, Renovate will update the lockfile with the new exact version.

By default, Renovate assumes that if you are using ranges then it's because you want them to be wide/open. Renovate won't deliberately "narrow" any range by increasing the semver value inside.

For example, if your package.json specifies a value for left-pad of ^1.0.0 and the latest version on npmjs is 1.2.0, then Renovate won't change anything because 1.2.0 satisfies the range. If instead you'd prefer to be updated to ^1.2.0 in cases like this, then configure rangeStrategy to bump in your Renovate config.

This feature supports caret (^) and tilde (~) ranges only, like ^1.0.0 and ~1.0.0.

The in-range-only strategy may be useful if you want to leave the package file unchanged and only do update-lockfile within the existing range. The in-range-only strategy behaves like update-lockfile, but discards any updates where the new version of the dependency is not equal to the current version. We recommend you avoid using the in-range-only strategy unless you strictly need it. Using the in-range-only strategy may result in you being multiple releases behind without knowing it.

## rebaseLabel[¶](https://docs.renovatebot.com/configuration-options/#rebaselabel)

Label to request a rebase from Renovate.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "rebase"

| cli  | --rebase-label

| env  | RENOVATE_REBASE_LABEL

On supported platforms it is possible to add a label to a PR to manually request Renovate to recreate/rebase it. By default this label is "rebase" but you can configure it to anything you want by changing this rebaseLabel field.

## rebaseWhen[¶](https://docs.renovatebot.com/configuration-options/#rebasewhen)

Controls when Renovate rebases an existing branch.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| allowedValues  |

```
["auto", "never", "conflicted", "behind-base-branch", "automerging"]

```

| default  | "auto"

| cli  | --rebase-when

| env  | RENOVATE_REBASE_WHEN

Possible values and meanings:

- auto: Renovate will autodetect the best setting. It will use behind-base-branch if configured to automerge or repository has been set to require PRs to be up to date. Otherwise, conflicted will be used instead. On GitHub, if the base branch has a merge queue, conflicted is used because the merge queue already tests PRs against the head of the base branch. The same applies on GitLab if merge trains are enabled on the project
- automerging: Renovate will use behind-base-branch if configured to automerge, Otherwise, never will be used instead
- never: Renovate will never rebase the branch or update it unless manually requested
- conflicted: Renovate will rebase only if the branch is conflicted
- behind-base-branch: Renovate will rebase whenever the branch falls 1 or more commit behind its base branch

rebaseWhen=conflicted is not recommended if you have enabled Renovate automerge, because:

- It could result in a broken base branch if two updates are merged one after another without testing the new versions together
- If you have enforced that PRs must be up-to-date before merging (e.g. using branch protection on GitHub), then automerge won't be possible as soon as a PR gets out-of-date but remains non-conflicted

It is also recommended to avoid rebaseWhen=never as it can result in conflicted branches with outdated PR descriptions and/or status checks.

Avoid setting rebaseWhen=never and then also setting prCreation=not-pending as this can prevent creation of PRs.

## recreateWhen[¶](https://docs.renovatebot.com/configuration-options/#recreatewhen)

Recreate PRs even if same ones were closed previously.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "auto"

| allowedValues  |

```
["auto", "always", "never"]

```

| cli  | --recreate-when

| env  | RENOVATE_RECREATE_WHEN

This feature used to be called recreateClosed.

By default, Renovate detects if it proposed an update to a project before, and will not propose the same update again. For example the Webpack 3.x case described in the [separateMajorMinor](https://docs.renovatebot.com/configuration-options/#separatemajorminor) documentation. You can use recreateWhen to customize this behavior down to a per-package level. For example we override it to always in the following cases where branch names and PR titles must be reused:

- Package groups
- When pinning versions
- Lock file maintenance

You can select which behavior you want from Renovate:

- always: Recreates all closed or blocking PRs
- auto: The default option. Recreates only [immortal PRs](https://docs.renovatebot.com/key-concepts/pull-requests/#immortal-prs) (default)
- never: No PR is recreated, doesn't matter if it is immortal or not

We recommend that you stick with the default setting for this option. Only change this setting if you really need to.

## registryAliases[¶](https://docs.renovatebot.com/configuration-options/#registryaliases)

Aliases for registries.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| mergeable  | true

| type  | object

| additionalProperties  |

```
{"type": "string"}

```

| supportedManagers  | ansible, argocd, bitbucket-pipelines, buildpacks, crossplane, crow, devcontainer, docker-compose, dockerfile, droneci, gitlabci, helm-requirements, helmfile, helmsman, helmv3, kubernetes, kustomize, maven, terraform, vendir, woodpecker

| cli  | --registry-aliases

| env  | RENOVATE_REGISTRY_ALIASES

| previouslyKnownAs  |

```
["aliases"]

```

You can use the registryAliases object to set registry aliases. Renovate applies all registryAliases objects, from top to bottom. The aliases support variables with default values (using the :- syntax) which can be useful for Docker Compose files.

This feature works with the following managers:

- [ansible](https://docs.renovatebot.com/modules/manager/ansible/)
- [argocd](https://docs.renovatebot.com/modules/manager/argocd/)
- [bazel-module](https://docs.renovatebot.com/modules/manager/bazel-module/)
- [bitbucket-pipelines](https://docs.renovatebot.com/modules/manager/bitbucket-pipelines/)
- [circleci](https://docs.renovatebot.com/modules/manager/circleci/)
- [crow](https://docs.renovatebot.com/modules/manager/crow/)
- [docker-compose](https://docs.renovatebot.com/modules/manager/docker-compose/)
- [dockerfile](https://docs.renovatebot.com/modules/manager/dockerfile/)
- [droneci](https://docs.renovatebot.com/modules/manager/droneci/)
- [fleet](https://docs.renovatebot.com/modules/manager/fleet/)
- [flux](https://docs.renovatebot.com/modules/manager/flux/)
- [github-actions](https://docs.renovatebot.com/modules/manager/github-actions/)
- [gitlabci](https://docs.renovatebot.com/modules/manager/gitlabci/)
- [helm-requirements](https://docs.renovatebot.com/modules/manager/helm-requirements/)
- [helm-values](https://docs.renovatebot.com/modules/manager/helm-values/)
- [helmfile](https://docs.renovatebot.com/modules/manager/helmfile/)
- [helmsman](https://docs.renovatebot.com/modules/manager/helmsman/)
- [helmv3](https://docs.renovatebot.com/modules/manager/helmv3/)
- [kubernetes](https://docs.renovatebot.com/modules/manager/kubernetes/)
- [terraform](https://docs.renovatebot.com/modules/manager/terraform/)
- [woodpecker](https://docs.renovatebot.com/modules/manager/woodpecker/)

Setting generic aliases

```
{
  "registryAliases": {
    "jfrogecosystem": "some.jfrog.mirror",
    "jfrog.com": "some.jfrog.mirror"
  }
}

```

Setting aliases for a specific Renovate manager only

```
{
  "gitlabci": {
    "registryAliases": {
      "$HARBOR_HOST/$HARBOR_PROJECT": "registry.example.com/proxy",
      "$HARBOR_HOST/tools": "registry.example.com/tools"
    }
  }
}

```

Using variables with default values

```
{
  "registryAliases": {
    "${CI_REGISTRY:-}": "my-registry.io"
  }
}

```

Both image reference formats work:

- With a slash after the variable: ${CI_REGISTRY:-}/image:1.0
- Without a slash after the variable: ${CI_REGISTRY:-}image:1.0

The alias value works with or without a trailing slash.

If you are using a pull-through cache (for instance on Amazon Elastic Container Registry (ECR)):

Using a pull-through cache for public images

```
{
  "registryAliases": {
    "12345612312.dkr.ecr.us-east-1.amazonaws.com/public-images-go-here/ghcr.io": "ghcr.io",
    "12345612312.dkr.ecr.us-east-1.amazonaws.com/public-images-go-here/dockerhub": "docker.io"
  }
}

```

## registryUrls[¶](https://docs.renovatebot.com/configuration-options/#registryurls)

List of URLs to try for dependency lookup. Package manager specific.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | array

| subType  | string

| default  | []

Usually Renovate is able to either (a) use the default registries for a datasource, or (b) automatically detect during the manager extract phase which custom registries are in use. In case there is a need to configure them manually, it can be done using this registryUrls field, typically using packageRules like so:

```
{
  "packageRules": [
    {
      "matchDatasources": ["docker"],
      "registryUrls": ["https://docker.mycompany.domain"]
    }
  ]
}

```

The field supports multiple URLs but it is datasource-dependent on whether only the first is used or multiple.

In some situations managers can not derive which registry a given dependency comes from. In that case, managers skip it with skipReason: unknown-registry rather than look it up somewhere which may hold the wrong versions. When specifying registryUrls alongside a dependency with skipReason: unknown-registry, Renovate will use the configured registryUrls.

## replacement[¶](https://docs.renovatebot.com/configuration-options/#replacement)

Configuration to apply when replacing a dependency.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| default  |

```
{
  "branchTopic": "{{{depNameSanitized}}}-replacement",
  "commitMessageAction": "Replace",
  "commitMessageExtra": "with {{newName}} {{#if isMajor}}{{{prettyNewMajor}}}{{else}}{{#if isSingleVersion}}{{{prettyNewVersion}}}{{else}}{{{newValue}}}{{/if}}{{/if}}",
  "prBodyNotes": [
    "This is a special PR that replaces `{{{depName}}}` with the community suggested minimal stable replacement version."
  ]
}

```

| mergeable  | true

| env  | RENOVATE_REPLACEMENT

Add to this object if you wish to define rules that apply only to PRs that replace dependencies.

## replacementApproach[¶](https://docs.renovatebot.com/configuration-options/#replacementapproach)

Select whether to perform a direct replacement or alias replacement.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| allowedValues  |

```
["replace", "alias"]

```

| supportedManagers  | npm

| default  | "replace"

| cli  | --replacement-approach

| env  | RENOVATE_REPLACEMENT_APPROACH

For npm manager when replacementApproach=alias then instead of replacing "foo": "1.2.3" with "@my/foo": "1.2.4" we would instead replace it with "foo": "npm:@my/foo@1.2.4".

```
{
  "replacementApproach": "alias"
}

```

## respectLatest[¶](https://docs.renovatebot.com/configuration-options/#respectlatest)

Ignore versions newer than npm "latest" version.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | true

| cli  | --respect-latest

| env  | RENOVATE_RESPECT_LATEST

Similar to ignoreUnstable, this option controls whether to update to versions that are greater than the version tagged as latest in the repository. By default, renovate will update to a version greater than latest only if the current version is itself past latest.

Note

By default, respectLatest will be set to false for Maven results if a latest tag is found. This is because many Maven registries don't have a reliable latest tag - it just means whatever was last published. You need to override this to respectLatest=true in packageRules in order to use it.

## reviewers[¶](https://docs.renovatebot.com/configuration-options/#reviewers)

Requested reviewers for Pull Requests (either username or email address depending on the platform).

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| cli  | --reviewers

| env  | RENOVATE_REVIEWERS

| default  | []

Must be valid usernames.

Required reviewers on Azure DevOps

To mark a reviewer as required on Azure DevOps, you must use the prefix required:.

For example: if the username or team name is bar then you would set the config option like this:

```
{
  "reviewers": ["required:bar"]
}

```

Required reviewers on Forgejo

If you're assigning a team to review on Forgejo, you must use the prefix team: and add the last part of the team name. Say the full team name on Forgejo is organization/foo, then you'd set the config option like this:

```
{
  "reviewers": ["team:foo"]
}

```

Required reviewers on GitHub

If you're assigning a team to review on GitHub, you must use the prefix team: and add the last part of the team name. Say the full team name on GitHub is @organization/foo, then you'd set the config option like this:

```
{
  "reviewers": ["team:foo"]
}

```

Please note that Reviewers are only added during creation of a PR, but are not modified afterwards.

Note

By default, Renovate will not assign reviewers and assignees to an automerge-enabled PR unless it fails status checks. By configuring [assignAutomerge](https://docs.renovatebot.com/configuration-options/#assignautomerge) setting to true, Renovate will instead always assign reviewers and assignees for automerging PRs at time of creation.

## reviewersFromCodeOwners[¶](https://docs.renovatebot.com/configuration-options/#reviewersfromcodeowners)

Determine reviewers based on configured code owners and changes in PR.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --reviewers-from-code-owners

| env  | RENOVATE_REVIEWERS_FROM_CODE_OWNERS

If enabled Renovate tries to determine PR reviewers by matching rules defined in a CODEOWNERS file against the changes in the PR.

Read the docs for your platform for details on syntax and allowed file locations:

- [GitHub Docs, About code owners](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)
- [GitLab, Code Owners](https://docs.gitlab.com/ee/user/project/codeowners/)
- [Bitbucket, Set up and use code owners](https://support.atlassian.com/bitbucket-cloud/docs/set-up-and-use-code-owners/)

Please note that Reviewers are only added during creation of a PR, but are not modified afterwards.

## reviewersSampleSize[¶](https://docs.renovatebot.com/configuration-options/#reviewerssamplesize)

Take a random sample of given size from reviewers.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | integer

| cli  | --reviewers-sample-size

| env  | RENOVATE_REVIEWERS_SAMPLE_SIZE

## rollback[¶](https://docs.renovatebot.com/configuration-options/#rollback)

Configuration to apply when rolling back a version.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | object

| default  |

```
{
  "branchTopic": "{{{depNameSanitized}}}-rollback",
  "commitMessageAction": "Roll back",
  "semanticCommitType": "fix"
}

```

| mergeable  | true

| env  | RENOVATE_ROLLBACK

Add to this object if you wish to define rules that apply only to PRs that roll back versions.

## rollbackPrs[¶](https://docs.renovatebot.com/configuration-options/#rollbackprs)

Create PRs to roll back versions if the current version is not found in the registry.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | false

| cli  | --rollback-prs

| env  | RENOVATE_ROLLBACK_PRS

There are times when a dependency version in use by a project gets removed from the registry. For some registries, existing releases or even whole packages can be removed or "yanked" at any time, while for some registries only very new or unused releases can be removed. Renovate's "rollback" feature exists to propose a downgrade to the next-highest release if the current release is no longer found in the registry.

Renovate does not create these rollback PRs by default, so this functionality needs to be opted-into. We recommend you do this selectively with packageRules and not globally.

## schedule[¶](https://docs.renovatebot.com/configuration-options/#schedule)

Limit branch creation to these times of day or week.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| cli  | --schedule

| default  |

```
["at any time"]

```

The schedule option allows you to define times of the day, week or month when you are willing to allow Renovate to create branches.

Setting a schedule does not itself cause or trigger Renovate to run. It's like putting a sign on your office which says "DHL deliveries only accepted between 9-11am". Such a sign won't cause DHL to come to your office only at 9-11am, instead it simply means that if they come at any other time of the day then they'll honor the sign and skip you. It also means that if they rarely attempt between 9-11am then you'll often get no deliveries in a day.

Similarly, if you set too restrictive of a Renovate schedule and the chance of Renovate running on your repo during those hours is low, then you might find your dependency updates regularly skipped. For this reason we recommend you allow a time window of at least 3-4 hours in any schedule, unless your instance of Renovate is expected to run more frequently than that.

Renovate supports the standard [Cron syntax](https://crontab.guru/crontab.5.html), as well as deprecated support for a subset of [Later syntax](https://github.com/breejs/later). We recommend you always use Cron syntax, due to its superior testing and robustness. Config support questions are no longer accepted for Later syntax problems - you will be recommended to use Cron instead.

The default value for schedule is "at any time", which is functionally the same as declaring a null schedule or * * * * * with Cron. i.e. Renovate will create Pull Requests at any time of any day, as needed.

The easiest way to define a schedule is to use a preset if one of them fits your requirements. See [Schedule presets](https://docs.renovatebot.com/presets-schedule/) for details and feel free to request a new one in the source repository if you think it would help others.

Here are some example schedules and their Cron equivalent:

| English description  | Supported by Later?  | Cron syntax

| every weekend  | Yes  | * * * * 0,6

| before 5:00am  | Yes  | * 0-4 * * *

| after 10pm and before 5am every weekday  | Yes  | * 22-23,0-4 * * 1-5

| on friday and saturday  | Yes  | * * * * 5,6

| every 3 months on the first day of the month  | Yes  | * * 1 */3 *

Note

For Cron schedules, you must use the * wildcard for the minutes value, as Renovate doesn't support minute granularity. And the cron schedule must have five comma separated parts.

One example might be that you don't want Renovate to run during your typical business hours, so that your build machines don't get clogged up testing package.json updates. You could then configure a schedule like this at the repository level:

```
{
  "description": "Schedule on weekdays at night (10 PM - 4 AM) and anytime on weekends",
  "schedule": ["* 22-23,0-4 * * *", "* * * * 0,6"]
}

```

This would mean that Renovate can run for 7 hours each night, plus all the time on weekends. Note how the above example makes use of the "OR" logic of combining multiple schedules in the array.

Note

If both the day of the week and the day of the month are restricted in the schedule, then Renovate only runs when both the day of the month and day of the week match! For example: * * 1-7 * 4 means Renovate only runs on the first Thursday of the month.

It's common to use schedule in combination with [timezone](https://docs.renovatebot.com/configuration-options/#timezone). You should configure [updateNotScheduled=false](https://docs.renovatebot.com/configuration-options/#updatenotscheduled) if you want the schedule more strictly enforced so that updates to existing branches aren't pushed out of schedule. You can also configure [automergeSchedule](https://docs.renovatebot.com/configuration-options/#automergeschedule) to limit the hours in which branches/PRs are automerged (if [automerge](https://docs.renovatebot.com/configuration-options/#automerge) is configured).

## semanticCommitScope[¶](https://docs.renovatebot.com/configuration-options/#semanticcommitscope)

Commit scope to use if Semantic Commits are enabled.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "deps"

| nullable  | true

| supportsTemplating  | true

| cli  | --semantic-commit-scope

| env  | RENOVATE_SEMANTIC_COMMIT_SCOPE

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

By default you will see Angular-style commit prefixes like "chore(deps):". If you wish to change it to something else like "package" then it will look like "chore(package):". You can also use parentDir or baseDir to namespace your commits for monorepos e.g. "{{parentDir}}".

## semanticCommitType[¶](https://docs.renovatebot.com/configuration-options/#semanticcommittype)

Commit type to use if Semantic Commits is enabled.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "chore"

| supportsTemplating  | true

| cli  | --semantic-commit-type

| env  | RENOVATE_SEMANTIC_COMMIT_TYPE

This option supports Renovate's template syntax

See [templates](https://docs.renovatebot.com/templates/) for available variables and helpers.

By default you will see Angular-style commit prefixes like "chore(deps):". If you wish to change it to something else like "ci" then it will look like "ci(deps):".

## semanticCommits[¶](https://docs.renovatebot.com/configuration-options/#semanticcommits)

Enable Semantic Commit prefixes for commits and PR titles.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| allowedValues  |

```
["auto", "enabled", "disabled"]

```

| default  | "auto"

| cli  | --semantic-commits

| env  | RENOVATE_SEMANTIC_COMMITS

If you are using a semantic prefix for your commits, then you will want to enable this setting. Although it's configurable to a package-level, it makes most sense to configure it at a repository level. If configured to enabled, then the semanticCommitScope and semanticCommitType fields will be used for each commit message and PR title.

Renovate autodetects if your repository is already using semantic commits or not and follows suit, so you only need to configure this if you wish to override Renovate's autodetected setting.

## separateMajorMinor[¶](https://docs.renovatebot.com/configuration-options/#separatemajorminor)

If set to false, Renovate will upgrade dependencies to their latest release only. Renovate will not separate major or minor branches.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | true

| cli  | --separate-major-minor

| env  | RENOVATE_SEPARATE_MAJOR_MINOR

Renovate's default behavior is to create a separate branch/PR if both minor and major version updates exist (note that your choice of rangeStrategy value can influence which updates exist in the first place however). For example, if you were using Webpack 2.0.0 and versions 2.1.0 and 3.0.0 were both available, then Renovate would create two PRs so that you have the choice whether to apply the minor update to 2.x or the major update of 3.x. If you were to apply the minor update then Renovate would keep updating the 3.x branch for you as well, e.g. if Webpack 3.0.1 or 3.1.0 were released. If instead you applied the 3.0.0 update then Renovate would clean up the unneeded 2.x branch for you on the next run.

It is recommended that you leave this option to true, because of the polite way that Renovate handles this. For example, let's say in the above example that you decided you wouldn't update to Webpack 3 for a long time and don't want to build/test every time a new 3.x version arrives. In that case, simply close the "Update Webpack to version 3.x" PR and it won't be recreated again even if subsequent Webpack 3.x versions are released. You can continue with Webpack 2.x for as long as you want and get any updates/patches that are made for it. Then eventually when you do want to update to Webpack 3.x you can make that update to package.json yourself and commit it to the base branch once it's tested. After that, Renovate will resume providing you updates to 3.x again! i.e. if you close a major upgrade PR then it won't come back again, but once you make the major upgrade yourself then Renovate will resume providing you with minor or patch updates.

This option also has priority over package groups configured by packageRule. So Renovate will propose separate PRs for major and minor updates of packages even if they are grouped. If you want to enforce grouped package updates, you need to set this option to false within the packageRule.

## separateMinorPatch[¶](https://docs.renovatebot.com/configuration-options/#separateminorpatch)

If set to true, Renovate will separate minor and patch updates into separate branches.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | false

| cli  | --separate-minor-patch

| env  | RENOVATE_SEPARATE_MINOR_PATCH

| previouslyKnownAs  |

```
["separatePatchReleases"]

```

By default, Renovate groups patch (1.0.x) and minor (1.x.0) releases into a single PR. For example: you are running version 1.0.0 of a package, which has two updates:

- 1.0.1, a patch type update
- 1.1.0, a minor type update

By default, Renovate creates a single PR for the 1.1.0 version.

If you want Renovate to create separate PRs for patch and minor upgrades, set separateMinorPatch to true. Getting separate updates from Renovate can be handy when you want to, for example, automerge patch updates but manually merge minor updates.

## separateMultipleMajor[¶](https://docs.renovatebot.com/configuration-options/#separatemultiplemajor)

If set to true, PRs will be raised separately for each available major upgrade version.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | false

| cli  | --separate-multiple-major

| env  | RENOVATE_SEPARATE_MULTIPLE_MAJOR

| previouslyKnownAs  |

```
["multipleMajorPrs"]

```

Configure this to true if you wish to get one PR for every separate major version upgrade of a dependency. e.g. if you are on webpack@v1 currently then default behavior is a PR for upgrading to webpack@v3 and not for webpack@v2. If this setting is true then you would get one PR for webpack@v2 and one for webpack@v3.

## separateMultipleMinor[¶](https://docs.renovatebot.com/configuration-options/#separatemultipleminor)

If set to true, Renovate creates separate PRs for each minor stream.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | false

| cli  | --separate-multiple-minor

| env  | RENOVATE_SEPARATE_MULTIPLE_MINOR

This feature is flagged as experimental

Experimental features might be changed or even removed at any time.

Enable this for dependencies when it is important to split updates into separate PRs per minor release stream (e.g. python).

For example, if you are on python@v3.9.0 currently, then by default Renovate creates a PR to upgrade you to the latest version such as python@v3.12.x. By default, Renovate skips versions in between, like python@v3.10.x.

But if you set separateMultipleMinor=true then you get separate PRs for each minor stream, like python@3.9.x, python@v3.10.x and python@v3.11.x, etc.

## skipArtifactsUpdate[¶](https://docs.renovatebot.com/configuration-options/#skipartifactsupdate)

Skip Renovate's automatic artifact updating.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | false

| cli  | --skip-artifacts-update

| env  | RENOVATE_SKIP_ARTIFACTS_UPDATE

Use this option when automatic artifact updating fails, is incorrect, or not needed.

This option was formerly known as updateLockFiles.

When this option is set, Renovate won't attempt to update artifacts such as lock files, so you will need to update them yourself, either manually or through secondary automation such as CI workflows.

Note

When this option is used in package rules, along with grouped upgrades, artifact updating will only be skipped if every upgrade in the grouped branch wants to skip it.

Warning

When artifact updates are skipped and the packageManager field in package.json is updated, the new version will not contain a hash. The hash is only applied when artifacts are updated. For example, a value of packageManager: "yarn@3.0.0+sha224.deadbeef" would be updated to just packageManager: "yarn@3.1.0" rather than packageManager: "yarn@3.1.0+sha224.f0cacc1a".

## skipInstalls[¶](https://docs.renovatebot.com/configuration-options/#skipinstalls)

Skip installing modules/dependencies if lock file updating is possible without a full install.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| supportedManagers  | npm

| cli  | --skip-installs

| env  | RENOVATE_SKIP_INSTALLS

| default  | null

By default, Renovate will use the most efficient approach to updating package files and lock files, which in most cases skips the need to perform a full module install. If this is set to false, then a full install of modules will be done. This is currently applicable to npm only, and only used in cases where bugs in npm result in incorrect lock files being updated.

## statusCheckNames[¶](https://docs.renovatebot.com/configuration-options/#statuschecknames)

Custom strings to use as status check names.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | object

| mergeable  | true

| default  |

```
{
  "artifactError": "renovate/artifacts",
  "configValidation": "renovate/config-validation",
  "mergeConfidence": "renovate/merge-confidence",
  "minimumReleaseAge": "renovate/stability-days"
}

```

| cli  | --status-check-names

| env  | RENOVATE_STATUS_CHECK_NAMES

Warning

For advanced use only! Use at your own risk!

You can customize the name/context of status checks that Renovate adds to commits/branches/PRs.

This option enables you to modify any existing status checks name/context, but adding new status checks this way is not supported. Setting the value to null or an empty string, effectively disables or skips that status check. This option is mergeable, which means you only have to specify the status checks that you want to modify.

Example of overriding status check strings

```
{
  "statusCheckNames": {
    "minimumReleaseAge": "custom/stability-days",
    "mergeConfidence": "custom/merge-confidence-level"
  }
}

```

## statusCheckWhen[¶](https://docs.renovatebot.com/configuration-options/#statuscheckwhen)

Control when each Renovate status check is set on branches. Keys match statusCheckNames.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | object

| mergeable  | true

| default  |

```
{
  "artifactError": "failed",
  "configValidation": "always",
  "mergeConfidence": "always",
  "minimumReleaseAge": "always"
}

```

| cli  | --status-check-when

| env  | RENOVATE_STATUS_CHECK_WHEN

Warning

For advanced use only! Use at your own risk!

Control when each Renovate status check is set on branches. The keys match those available in [statusCheckNames](https://docs.renovatebot.com/configuration-options/#statuschecknames). This option is mergeable, so you only need to specify the checks you want to change.

Allowed values per key:

- always: Renovate always sets the status check on every branch commit — green on success, red on failure.
- never: Renovate never sets this status check. Use this to skip a check entirely.
- failed: Renovate only sets the status check when there is a failure. Successes are silent (no green check).

Default values:

| Key  | Default  | Reasoning

| artifactError  | failed  | Legacy behavior — only reports artifact failures

| configValidation  | always  | Always reports validation pass/fail

| mergeConfidence  | always  | Always reports confidence level

| minimumReleaseAge  | always  | Always reports stability status

For example, to make renovate/artifacts a required check, set artifactError to "always" so it always reports green/red, allowing you to mark it as required in your branch protection rules. To skip a check entirely, set any key to "never" to suppress that status check. To only report failures for stability, set minimumReleaseAge to "failed" if you only care about the red/pending status.

Example: always set artifacts check, skip merge confidence

```
{
  "statusCheckWhen": {
    "artifactError": "always",
    "mergeConfidence": "never"
  }
}

```

## stopUpdatingLabel[¶](https://docs.renovatebot.com/configuration-options/#stopupdatinglabel)

Label to make Renovate stop updating a PR.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| default  | "stop-updating"

| supportedPlatforms  | azure, forgejo, gerrit, gitea, github, gitlab

| cli  | --stop-updating-label

| env  | RENOVATE_STOP_UPDATING_LABEL

This feature only works on supported platforms, check the table above.

If you want Renovate to stop updating a PR, you can apply a label to the PR. By default, Renovate listens to the label: "stop-updating".

You can set your own label name with the "stopUpdatingLabel" field:

```
{
  "stopUpdatingLabel": "take-a-break-renovate"
}

```

## suppressNotifications[¶](https://docs.renovatebot.com/configuration-options/#suppressnotifications)

Options to suppress various types of warnings and other notifications.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | array

| subType  | string

| allowedValues  |

```
[
  "artifactErrors",
  "branchAutomergeFailure",
  "configErrorIssue",
  "dependencyLookupWarnings",
  "lockFileErrors",
  "missingCredentialsError",
  "onboardingClose",
  "prEditedNotification",
  "prIgnoreNotification"
]

```

| mergeable  | true

| default  | []

Use this field to suppress various types of warnings and other notifications from Renovate. For example:

```
{
  "suppressNotifications": ["prIgnoreNotification"]
}

```

The above config will suppress the comment which is added to a PR whenever you close a PR unmerged.

## timezone[¶](https://docs.renovatebot.com/configuration-options/#timezone)

Must conform to [IANA Time Zone](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones) format.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | string

| cli  | --timezone

| env  | RENOVATE_TIMEZONE

| default  | null

We recommend that you only configure the timezone option if both of these are true:

- you want to use the schedule feature
- and you want Renovate to evaluate the schedule in your timezone

Please see the above link for valid timezone names.

## toolSettings[¶](https://docs.renovatebot.com/configuration-options/#toolsettings)

Tool specific configuration. Global self-hosted configuration takes precedence.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | object

| default  |

```
{"jvmMaxMemory": 512, "jvmMemory": 512}

```

| env  | RENOVATE_TOOL_SETTINGS

When Renovate updates a dependency and needs to invoke processes leveraging Java, for example Gradle for [the gradle](https://docs.renovatebot.com/modules/manager/gradle/) or [the gradle-wrapper](https://docs.renovatebot.com/modules/manager/gradle-wrapper/) managers, the repository's Gradle Wrapper will be invoked, if present.

The JVM heap size for the Java invocations is 512m by default. This can be overridden using the following options.

This option can be used on the repository level and in the [Renovate configuration](https://docs.renovatebot.com/self-hosted-configuration/) using the following options.

Note

The JVM memory settings specified in the global self-hosted configuration set by the administrator in [toolSettings.jvmMaxMemory](https://docs.renovatebot.com/self-hosted-configuration/) limits the memory settings for all repositories. The default limit for all repositories is 512m.

Note

The JVM memory settings are considered for the gradle and gradle-wrapper manager.

### toolSettings.jvmMaxMemory[¶](https://docs.renovatebot.com/configuration-options/#toolsettingsjvmmaxmemory)

Maximum JVM memory in MB to use for updates that use a Java VM, like the Gradle Wrapper, defaults to 512. Repo configuration for this value will be ignored if it exceeds the global configuration for toolSettings.jvmMaxMemory

| Name  | Value

| type  | integer

| parents  | toolSettings

| supportedManagers  | gradle-wrapper

Maximum heap size in MB for Java VMs. Defaults to 512 for both the repository level and self-hosted configuration.

To allow repositories to use more than 512m of heap during any invocations of the Gradle Wrapper, configure the jvmMaxMemory option in the [toolSettings.jvmMaxMemory](https://docs.renovatebot.com/self-hosted-configuration/).

### toolSettings.jvmMemory[¶](https://docs.renovatebot.com/configuration-options/#toolsettingsjvmmemory)

Initial JVM memory in MB to use for updates that use a Java VM, like the Gradle Wrapper, defaults to jvmMaxMemory. Repo configuration for this value will be ignored if it exceeds the global configuration for toolSettings.jvmMaxMemory

| Name  | Value

| type  | integer

| parents  | toolSettings

| supportedManagers  | gradle-wrapper

Initial heap size in MB for Java VMs. Must be less than or equal to jvmMaxMemory. Defaults to jvmMaxMemory.

### toolSettings.nodeMaxMemory[¶](https://docs.renovatebot.com/configuration-options/#toolsettingsnodemaxmemory)

Maximum memory in MiB for Node child processes invoked by Renovate. If unset, the Node process will automagically determine the memory limit to use. Repo configuration for this value will be ignored if it exceeds the global configuration for toolSettings.nodeMaxMemory

| Name  | Value

| type  | integer

| parents  | toolSettings

| supportedManagers  | npm

Note

This does not apply to every Node process created by Renovate, only the managers noted above.

## updateInternalDeps[¶](https://docs.renovatebot.com/configuration-options/#updateinternaldeps)

Whether to update internal dep versions in a monorepo. Works on Yarn Workspaces.

| Name  | Value

| scopes  | repo, manager

| type  | boolean

| default  | false

| supportedManagers  | npm

| cli  | --update-internal-deps

| env  | RENOVATE_UPDATE_INTERNAL_DEPS

Renovate defaults to skipping any internal package dependencies within monorepos. In such case dependency versions won't be updated by Renovate.

To opt in to letting Renovate update internal package versions normally, set this configuration option to true.

## updateNotScheduled[¶](https://docs.renovatebot.com/configuration-options/#updatenotscheduled)

Whether to update branches when not scheduled. Renovate will not create branches outside of the schedule.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | boolean

| default  | true

| cli  | --update-not-scheduled

| env  | RENOVATE_UPDATE_NOT_SCHEDULED

When schedules are in use, it generally means "no updates". However there are cases where updates might be desirable - e.g. if you have configured prCreation=not-pending, or you have rebaseWhen=behind-base-branch and the base branch is updated so you want Renovate PRs to be rebased.

This defaults to true, meaning that Renovate will perform certain "desirable" updates to existing PRs even when outside of schedule. If you wish to disable all updates outside of scheduled hours then configure this field to false.

## updatePinnedDependencies[¶](https://docs.renovatebot.com/configuration-options/#updatepinneddependencies)

Whether to update pinned (single version) dependencies or not.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | boolean

| default  | true

| cli  | --update-pinned-dependencies

| env  | RENOVATE_UPDATE_PINNED_DEPENDENCIES

By default, Renovate will try to update all detected dependencies, regardless of whether they are defined using pinned single versions (e.g. 1.2.3) or constraints/ranges (e.g. (^1.2.3). You can set this option to false if you wish to disable updating for pinned (single version) dependencies specifically.

## useBaseBranchConfig[¶](https://docs.renovatebot.com/configuration-options/#usebasebranchconfig)

Whether to read configuration from base branches instead of only the default branch.

| Name  | Value

| scopes  | repo

| type  | string

| allowedValues  |

```
["merge", "none"]

```

| default  | "none"

| cli  | --use-base-branch-config

| env  | RENOVATE_USE_BASE_BRANCH_CONFIG

By default, Renovate will read config file from the default branch only and will ignore any config files in base branches. You can configure useBaseBranchConfig=merge to instruct Renovate to merge the config from each base branch over the top of the config in the default branch.

The config file name in the base branch must be the same as in the default branch and cannot be package.json. This scenario may be useful for testing the config changes in base branches instantly.

## userStrings[¶](https://docs.renovatebot.com/configuration-options/#userstrings)

User-facing strings for the Renovate comment when a PR is closed.

| Name  | Value

| scopes  | repo, packageRule, manager, updateType, vulnerabilityAlert, group

| type  | object

| freeChoice  | true

| default  |

```
{
  "ignoreTopic": "Renovate Ignore Notification",
  "ignoreMajor": "Because you closed this PR without merging, Renovate will ignore this update. You will not get PRs for *any* future `{{{newMajor}}}.x` releases. But if you manually upgrade to `{{{newMajor}}}.x` then Renovate will re-enable `minor` and `patch` updates automatically.",
  "ignoreDigest": "Because you closed this PR without merging, Renovate will ignore this update. You will not get PRs for the `{{{depName}}}` `{{{newDigestShort}}}` update again.",
  "ignoreOther": "Because you closed this PR without merging, Renovate will ignore this update (`{{{newValue}}}`). You will get a PR once a newer version is released. To ignore this dependency forever, add it to the `ignoreDeps` array of your Renovate config.",
  "artifactErrorWarning": "You probably do not want to merge this PR as-is."
}

```

| cli  | --user-strings

| env  | RENOVATE_USER_STRINGS

When a PR is closed, Renovate posts a comment to let users know that future updates will be ignored. If you want, you can change the text in the comment with the userStrings config option.

You can edit these user-facing strings:

- artifactErrorWarning: Text of the PR comment when artifact errors occur during updates.
- ignoreDigest: Text of the PR comment for digest upgrades.
- ignoreMajor: Text of the PR comment for major upgrades.
- ignoreOther: Text of the PR comment for other (neither digest nor major) upgrades.
- ignoreTopic: Topic of the PR comment.

For example:

```
{
  "userStrings": {
    "artifactErrorWarning": "Custom text for artifact errors.",
    "ignoreTopic": "Custom topic for PR comment",
    "ignoreMajor": "Custom text for major upgrades.",
    "ignoreDigest": "Custom text for digest upgrades.",
    "ignoreOther": "Custom text for other upgrades."
  }
}

```

## versionCompatibility[¶](https://docs.renovatebot.com/configuration-options/#versioncompatibility)

A regex (re2) with named capture groups to show how version and compatibility are split from a raw version string.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | string

| format  | regex

| default  | null

This option is used for advanced use cases where the version string embeds more data than just the version. It's typically used with docker and tags datasources.

Here are two examples:

- The image tag ghcr.io/umami-software/umami:postgresql-v1.37.0 embeds text like postgresql- as a prefix to the actual version to differentiate different DB types.
- Docker image tags like node:18.10.0-alpine embed the base image as a suffix to the version.

Here is an example of solving these types of cases:

```
{
  "packageRules": [
    {
      "matchDatasources": ["docker"],
      "matchPackageNames": ["ghcr.io/umami-software/umami"],
      "versionCompatibility": "^(?<compatibility>.*)-(?<version>.*)$",
      "versioning": "semver"
    },
    {
      "matchDatasources": ["docker"],
      "matchPackageNames": ["node"],
      "versionCompatibility": "^(?<version>[^-]+)(?<compatibility>-.*)?$",
      "versioning": "node"
    }
  ]
}

```

This feature is most useful when the currentValue is a version and not a range/constraint.

This feature can be used in combination with extractVersion although that's likely only a rare edge case. When combined, extractVersion is applied to datasource results first, and then versionCompatibility. extractVersion should be used when the raw version string returned by the datasource contains extra details (such as a v prefix) when compared to the value/version used within the repository.

During the lookup phase, Renovate evaluates the versionCompatibility regex against the currentValue string. If there is a match, the version part is stored internally temporarily as compareValue and the compatibility part stored as currentCompatibility. Storing currentCompatibility allows Renovate to reuse this value later to filter for new versions with the same compatibility. Renovate applies this compatibility check to datasource lookup results by passing both the versionCompatibility and currentCompatibility strings to a filter.

For a new version to be allowed, it must:

- Satisfy the versionCompatibility regex, and
- Have the same compatibility part as the currentValue

At this point, Renovate's core lookup logic is comparing versions to versions, and ignoring compatibility strings like -jre8. Finally, once updates are decided, Renovate restores the compatibility part to the newValue result.

## versioning[¶](https://docs.renovatebot.com/configuration-options/#versioning)

Versioning to use for filtering and comparisons.

| Name  | Value

| scopes  | repo, packageRule, manager, vulnerabilityAlert

| type  | string

| allowedValues  |

```
[
  "apk",
  "aws-eks-addon",
  "aws-machine-image",
  "azure-rest-api",
  "bazel-module",
  "cargo",
  "composer",
  "conan",
  "conda",
  "deb",
  "debian",
  "deno",
  "devbox",
  "docker",
  "elm",
  "exact",
  "git",
  "github-actions",
  "glasskube",
  "go-mod-directive",
  "gomod",
  "gradle",
  "hashicorp",
  "helm",
  "hermit",
  "hex",
  "ivy",
  "kubernetes-api",
  "lambda-node",
  "loose",
  "maven",
  "nixpkgs",
  "node",
  "npm",
  "nuget",
  "paket",
  "pep440",
  "perl",
  "poetry",
  "pvp",
  "python",
  "redhat",
  "regex",
  "rez",
  "rpm",
  "ruby",
  "rust-release-channel",
  "same-major",
  "semver",
  "semver-coerced",
  "semver-partial",
  "swift",
  "ubuntu",
  "unity3d",
  "unity3d-packages"
]

```

| previouslyKnownAs  |

```
["versionScheme"]

```

| default  | null

Usually, each language or package manager has a specific type of "versioning": JavaScript uses npm's SemVer implementation, Python uses pep440, etc.

Renovate also uses custom versioning, like "docker" to address the most common way people tag versions using Docker, and "loose" as a fallback that tries SemVer first. Otherwise Renovate does its best to sort and compare.

By exposing versioning to config, you can override the default versioning for a package manager if needed. We do not recommend overriding the default versioning, but there are some cases such as Docker or Gradle where versioning is not strictly defined and you may need to specify the versioning type per-package.

Renovate supports 4-part versions (1.2.3.4) in full for the NuGet package manager. Other managers can use the "loose" versioning fallback: the first 3 parts are used as the version, all trailing parts are used for alphanumeric sorting.

A key characteristic of the "docker" versioning is that it attempts to preserve the precision of the current version string. For example, if your current version has two parts (e.g., 5.6), Renovate will propose updates that also have two parts (e.g., 5.7), rather than a three-part SemVer equivalent (e.g., 5.7.0).

Example Use Case: "Floating Patch Versions" using 2 digits versioning

You may want to use a 2-part version like 5.6 to indicate the minor version while automatically receiving the latest patch updates (a "floating patch"). At the same time, you still want Renovate to create merge requests for minor or major updates like 5.7 or 6.2. The default semver versioning would update 5.6 to a 3-part version like 5.6.1, which would break the floating patch behavior.

In the below renovate.json extract example, Renovate will use the docker versioning for the .gitlab-ci.yml file, so you can pin the minor version while still receiving the latest patch updates.

```
{
  "packageRules": [
    {
      "matchFileNames": [".gitlab-ci.yml"],
      "versioning": "docker"
    }
  ]
}

```

When using this strategy, make sure that the package you're referencing does support 2-part versions, e.g. that it has a version like 5.6 available in the registry in addition to 5.6.1 or 5.6.2.

## vulnerabilityAlerts[¶](https://docs.renovatebot.com/configuration-options/#vulnerabilityalerts)

Config to apply when a PR is needed due to a vulnerability in the existing package version.

| Name  | Value

| scopes  | repo

| type  | object

| default  |

```
{
  "groupName": null,
  "schedule": [],
  "dependencyDashboardApproval": false,
  "minimumReleaseAge": null,
  "rangeStrategy": "update-lockfile",
  "commitMessageSuffix": "[SECURITY]",
  "branchTopic": "{{{datasource}}}-{{{depNameSanitized}}}-vulnerability",
  "prCreation": "immediate",
  "vulnerabilityFixStrategy": "lowest",
  "prConcurrentLimit": 0
}

```

| mergeable  | true

| supportedPlatforms  | github

Renovate can read GitHub's Vulnerability Alerts to customize its Pull Requests. For this to work, you must enable the [Dependency graph](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/enabling-the-dependency-graph), and [Dependabot alerts](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-security-and-analysis-settings-for-your-repository). Follow these steps:

- While logged in to GitHub, navigate to your repository
- Select the "Settings" tab
- Select "Advanced Security" in the sidebar
- Enable the "Dependency graph"
- Enable "Dependabot alerts"
- If you're running Renovate in app mode: make sure the app has read permissions for "Dependabot alerts". If you're the account administrator, browse to the app (for example [the Mend Renovate App](https://github.com/apps/renovate)), select "Configure", and then scroll down to the "Permissions" section and make sure that read access to "Dependabot alerts" is mentioned

Once the above conditions are met, and you got one or more vulnerability alerts from GitHub for this repository, then Renovate tries to raise fix PRs.

You may use the vulnerabilityAlerts configuration object to customize vulnerability-fix PRs.

Setting a custom label and assignee

```
{
  "vulnerabilityAlerts": {
    "labels": ["security"],
    "automerge": true,
    "assignees": ["@renovate-tests"]
  }
}

```

Warning

There's a small chance that a wrong vulnerability alert results in a flapping/looping vulnerability fix. If you allow Renovate to automerge vulnerability fixes, please check if the automerged fix is correct.

Note

By default, when Renovate creates a vulnerabilityAlerts PR, it ignores settings like branchConcurrentLimit, commitHourlyLimit, prConcurrentLimit, prHourlyLimit, or schedule. This means that Renovate always tries to create a vulnerabilityAlerts PR. In short: vulnerability alerts "skip the line".

Note

Some managers disable certain dependencies by default, such as indirect Go modules or Gradle strictly constraints. Renovate still raises vulnerability-fix PRs for those dependencies, unless you disable them yourself with a packageRules entry that sets enabled=false.

If you get many alerts at once, for example when you onboard a repository which was not using Renovate before, then set prConcurrentLimit or branchConcurrentLimit inside the vulnerabilityAlerts object. Vulnerability fixes get their own budget, they do not take a share of the repository-wide one:

Allowing up to 5 vulnerability fix PRs on top of 5 regular PRs

```
{
  "prConcurrentLimit": 5,
  "vulnerabilityAlerts": {
    "prConcurrentLimit": 5
  }
}

```

With the config above Renovate creates at most 10 open PRs: 5 for regular updates, plus 5 for vulnerability fixes. The default is 0, which means "no limit", so vulnerability alerts keep skipping the line until you set a limit yourself. You do not need to set branchConcurrentLimit as well, because its null default already means "inherit prConcurrentLimit". These limits also apply to updates that a packageRules entry marks with isVulnerabilityAlert=true.

Note

Only the concurrent limits can be set this way. Renovate always ignores commitHourlyLimit, prHourlyLimit, prCommitsPerRunLimit and schedule for vulnerability alerts, you can not override this.

To disable the vulnerability alerts feature, set enabled=false in a vulnerabilityAlerts config object, like this:

Disabling vulnerability alerts

```
{
  "vulnerabilityAlerts": {
    "enabled": false
  }
}

```

Note

If you want to raise only vulnerability fix PRs, you may use the security:only-security-updates preset.

### vulnerabilityAlerts.vulnerabilityFixStrategy[¶](https://docs.renovatebot.com/configuration-options/#vulnerabilityalertsvulnerabilityfixstrategy)

Strategy to use when fixing vulnerabilities. lowest will propose the earliest version with a fix, highest will always pick the latest version.

| Name  | Value

| type  | string

| allowedValues  |

```
["lowest", "highest"]

```

| default  | "lowest"

| parents  | vulnerabilityAlerts

| cli  | --vulnerability-fix-strategy

| env  | RENOVATE_VULNERABILITY_FIX_STRATEGY

When a vulnerability fix is available, Renovate will default to picking the lowest fixed version (vulnerabilityFixStrategy=lowest). For example, if the current version is 1.0.0, and a vulnerability is fixed in 1.1.0, while the latest version is 1.2.0, then Renovate will propose an update to 1.1.0 as the vulnerability fix.

If vulnerabilityFixStrategy=highest is configured then Renovate will use its normal strategy for picking upgrades, e.g. in the above example it will propose an update to 1.2.0 to fix the vulnerability.

Setting vulnerabilityFixStrategy to highest

```
{
  "vulnerabilityAlerts": {
    "vulnerabilityFixStrategy": "highest"
  }
}

```
