---
url: "https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert"
title: "GitHub project maintainers targeted with fake security alert - Help Net Security"
site: "Help Net Security"
published: "2025-03-17T10:49:32+00:00"
captured: "2026-11-02T18:55:58Z"
via: "live"
source_url: "https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert/"
sha256: "62b5b75809fdc4d332334ab8f9a851bfd97b428ff9adbd0134da92a3d2273c48"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# GitHub project maintainers targeted with fake security alert - Help Net Security

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert>

[Zeljka Zorz](https://www.helpnetsecurity.com/author/zeljkazorz/), Editor-in-Chief, Help Net Security

March 17, 2025

Share https://www.facebook.com/sharer/sharer.php?u=https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert/&title=GitHub project maintainers targeted with fake security alert https://twitter.com/intent/tweet?url=https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert/&text=GitHub+project+maintainers+targeted+with+fake+security+alert https://www.linkedin.com/shareArticle?mini=true&url=https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert/&title=GitHub project maintainers targeted with fake security alert&source=https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert/  https://google.com/preferences/source?q=www.helpnetsecurity.com

# GitHub project maintainers targeted with fake security alert

A phishing campaign targeting GitHub account owners has been trying to scare them with a fake security alert into allowing a malicious OAuth app access to their account and repositories.

### The fake security alert from GitHub

GitHub users have [taken](https://x.com/s3bc40/status/1900965988541030657) to [social media](https://www.reddit.com/r/github/comments/1jck0y7/latest_phishing_scam_i_see_multiple_issues_being/) to [warn](https://x.com/devabdultech/status/1901298602590257154) others about emails ostensibly coming from the GitHub Security Team, alerting recipients about an “unusual access attempt” from an IP address/device located in Reykjavik, Iceland.

The attackers mounted this campaign by [creating a GitHub issue](https://github.com/search?q=Unusual+Sign-in+Activity+Detected+on+Your+GitHub+Account&type=issues) containing the fake security alert, then relying on GitHub to automatically send notification emails to the project maintainers.

https://img2.helpnetsecurity.com/posts2025/GitHub-fake-alert-issue.webp

######

The malicious GitHub issue (Source: @devabdultech)

The phishers have been rotating GitHub accounts to create the issues because GitHub has been shutting them down after they get flagged by targets that saw through the scam.

https://img2.helpnetsecurity.com/posts2025/GitHub-Issues.webp

######

Malicious opened issues (Source: Help Net Security)

### What to do?

Users who have taken the warning at face value, clicked one of the links in the email to secure their account, logged into their account and authorized the Security App (“gitsecurityapp”) OAuth app to supposedly help them in this task have effectively opened the door to their account to the crooks behind this scheme.

The malicious OAuth app allows the attackers to access the target’s public and private repositories, profile info, discussions, update GitHub Action Workflow files, and delete repositories. What they actually do with the acquired access is currently unknown.

Users who have fallen for the scheme are advised to revoke the app’s access, secure their account by changing their login credentials and authorization tokens, and to look for and delete additions and modifications made in their repositories in the meantime.

More about

- [account hijacking](https://www.helpnetsecurity.com/tag/account-hijacking/)
- [GitHub](https://www.helpnetsecurity.com/tag/github/)
- [OAuth](https://www.helpnetsecurity.com/tag/oauth/)
- [phishing](https://www.helpnetsecurity.com/tag/phishing/)

https://google.com/preferences/source?q=https://www.helpnetsecurity.com

Share https://www.facebook.com/sharer/sharer.php?u=https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert/&title=GitHub project maintainers targeted with fake security alert https://twitter.com/intent/tweet?url=https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert/&text=GitHub+project+maintainers+targeted+with+fake+security+alert https://www.linkedin.com/shareArticle?mini=true&url=https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert/&title=GitHub project maintainers targeted with fake security alert&source=https://www.helpnetsecurity.com/2025/03/17/github-project-maintainers-targeted-with-fake-security-alert/  https://google.com/preferences/source?q=www.helpnetsecurity.com
