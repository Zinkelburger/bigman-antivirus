---
url: "https://github.com/dev-sec/ssh-baseline"
title: "GitHub - dev-sec/ssh-baseline: DevSec SSH Baseline - InSpec Profile"
site: "GitHub"
captured: "2026-11-02T18:50:46Z"
via: "live"
sha256: "f481ad95711b5cddcff2ac3d2d86c99c42fc0c12cb90d987f1cc7cf7ae4dfc5b"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# GitHub - dev-sec/ssh-baseline: DevSec SSH Baseline - InSpec Profile

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/dev-sec/ssh-baseline>

## Repository navigation

[ssh-baseline](https://github.com/dev-sec/ssh-baseline)

Public

- [Notifications](https://github.com/login?return_to=%2Fdev-sec%2Fssh-baseline)
- [Fork (76)](https://github.com/login?return_to=%2Fdev-sec%2Fssh-baseline)
- [Star (295)](https://github.com/login?return_to=%2Fdev-sec%2Fssh-baseline)

## About

DevSec SSH Baseline - InSpec Profile

[dev-sec.io/baselines/ssh/](https://dev-sec.io/baselines/ssh/)

### Topics

[audit](https://github.com/topics/audit)[baseline](https://github.com/topics/baseline)[devsec](https://github.com/topics/devsec)[hacktoberfest](https://github.com/topics/hacktoberfest)[hardening](https://github.com/topics/hardening)[inspec](https://github.com/topics/inspec)[security](https://github.com/topics/security)[ssh](https://github.com/topics/ssh)

### Resources

[Readme](https://github.com/dev-sec/ssh-baseline#readme-ov-file)

[Apache-2.0 license](https://github.com/dev-sec/ssh-baseline#Apache-2.0-1-ov-file)

[Activity](https://github.com/dev-sec/ssh-baseline/activity)

[Custom properties](https://github.com/dev-sec/ssh-baseline/custom-properties)

### Stars

295 stars

### Watchers

21 watching

### Forks

[76 forks](https://github.com/dev-sec/ssh-baseline/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2Fdev-sec%2Fssh-baseline&report=dev-sec+%28user%29)

[Branches](https://github.com/dev-sec/ssh-baseline/branches)[Tags](https://github.com/dev-sec/ssh-baseline/tags)

https://github.com/dev-sec/ssh-baseline/brancheshttps://github.com/dev-sec/ssh-baseline/tags

## Latest commit

## History

[419 Commits](https://github.com/dev-sec/ssh-baseline/commits/master/)

https://github.com/dev-sec/ssh-baseline/commits/master/

## Folders and files

| Name | Name |

Last commit message

|

Last commit date

|

[.github](https://github.com/dev-sec/ssh-baseline/tree/master/.github)

|

[.github](https://github.com/dev-sec/ssh-baseline/tree/master/.github)

|

|

|

[controls](https://github.com/dev-sec/ssh-baseline/tree/master/controls)

|

[controls](https://github.com/dev-sec/ssh-baseline/tree/master/controls)

|

|

|

[libraries](https://github.com/dev-sec/ssh-baseline/tree/master/libraries)

|

[libraries](https://github.com/dev-sec/ssh-baseline/tree/master/libraries)

|

|

|

[.gitignore](https://github.com/dev-sec/ssh-baseline/blob/master/.gitignore)

|

[.gitignore](https://github.com/dev-sec/ssh-baseline/blob/master/.gitignore)

|

|

|

[.rubocop.yml](https://github.com/dev-sec/ssh-baseline/blob/master/.rubocop.yml)

|

[.rubocop.yml](https://github.com/dev-sec/ssh-baseline/blob/master/.rubocop.yml)

|

|

|

[CHANGELOG.md](https://github.com/dev-sec/ssh-baseline/blob/master/CHANGELOG.md)

|

[CHANGELOG.md](https://github.com/dev-sec/ssh-baseline/blob/master/CHANGELOG.md)

|

|

|

[Gemfile](https://github.com/dev-sec/ssh-baseline/blob/master/Gemfile)

|

[Gemfile](https://github.com/dev-sec/ssh-baseline/blob/master/Gemfile)

|

|

|

[LICENSE](https://github.com/dev-sec/ssh-baseline/blob/master/LICENSE)

|

[LICENSE](https://github.com/dev-sec/ssh-baseline/blob/master/LICENSE)

|

|

|

[README.md](https://github.com/dev-sec/ssh-baseline/blob/master/README.md)

|

[README.md](https://github.com/dev-sec/ssh-baseline/blob/master/README.md)

|

|

|

[Rakefile](https://github.com/dev-sec/ssh-baseline/blob/master/Rakefile)

|

[Rakefile](https://github.com/dev-sec/ssh-baseline/blob/master/Rakefile)

|

|

|

[inspec.yml](https://github.com/dev-sec/ssh-baseline/blob/master/inspec.yml)

|

[inspec.yml](https://github.com/dev-sec/ssh-baseline/blob/master/inspec.yml)

|

|

|

[renovate.json](https://github.com/dev-sec/ssh-baseline/blob/master/renovate.json)

|

[renovate.json](https://github.com/dev-sec/ssh-baseline/blob/master/renovate.json)

|

|

|

## Repository files navigation

# DevSec SSH Baseline

https://github.com/dev-sec/ssh-baseline#devsec-ssh-baseline

This Compliance Profile ensures, that all hardening projects keep the same quality.

- https://github.com/dev-sec/puppet-ssh-hardening
- https://github.com/dev-sec/chef-ssh-hardening
- https://github.com/dev-sec/ansible-ssh-hardening

## Standalone Usage

https://github.com/dev-sec/ssh-baseline#standalone-usage

This Compliance Profile requires [InSpec](https://github.com/chef/inspec) for execution:

```
$ git clone https://github.com/dev-sec/ssh-baseline
$ inspec exec ssh-baseline

```

You can also execute the profile directly from Github:

```
$ inspec exec https://github.com/dev-sec/ssh-baseline

```

## License and Author

https://github.com/dev-sec/ssh-baseline#license-and-author

-

Author:: Patrick Muench patrick.muench1111@googlemail.com

-

Author:: Dominik Richter dominik.richter@googlemail.com

-

Author:: Christoph Hartmann chris@lollyrock.com

-

Author:: Edmund Haselwanter me@ehaselwanter.com

-

Copyright 2014-2017, The DevSec Hardening Framework Team

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

```
http://www.apache.org/licenses/LICENSE-2.0

```

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

## About

DevSec SSH Baseline - InSpec Profile

[dev-sec.io/baselines/ssh/](https://dev-sec.io/baselines/ssh/)

### Topics

[audit](https://github.com/topics/audit)[baseline](https://github.com/topics/baseline)[devsec](https://github.com/topics/devsec)[hacktoberfest](https://github.com/topics/hacktoberfest)[hardening](https://github.com/topics/hardening)[inspec](https://github.com/topics/inspec)[security](https://github.com/topics/security)[ssh](https://github.com/topics/ssh)

### Resources

[Readme](https://github.com/dev-sec/ssh-baseline#readme-ov-file)

[Apache-2.0 license](https://github.com/dev-sec/ssh-baseline#Apache-2.0-1-ov-file)

[Activity](https://github.com/dev-sec/ssh-baseline/activity)

[Custom properties](https://github.com/dev-sec/ssh-baseline/custom-properties)

### Stars

295 stars

### Watchers

21 watching

### Forks

[76 forks](https://github.com/dev-sec/ssh-baseline/forks)

[Report repository](https://github.com/contact/report-content?content_url=https%3A%2F%2Fgithub.com%2Fdev-sec%2Fssh-baseline&report=dev-sec+%28user%29)

## Releases

## Packages

## Used by

## Contributors

## Languages
