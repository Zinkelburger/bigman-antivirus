---
url: "https://trufflesecurity.com/blog/anyone-can-access-deleted-and-private-repo-data-github"
title: "Anyone can Access Deleted and Private Repository Data on GitHub ◆ Truffle Security Co."
captured: "2026-11-02T18:46:47Z"
via: "live"
sha256: "3cae600dd8f491819524a8f7969f47276ce62d68fe6be664aa36e6a9492a7d49"
wayback: "https://web.archive.org/web/20261021130104/https://trufflesecurity.com/blog/anyone-can-access-deleted-and-private-repo-data-github"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Anyone can Access Deleted and Private Repository Data on GitHub ◆ Truffle Security Co.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://trufflesecurity.com/blog/anyone-can-access-deleted-and-private-repo-data-github>
> Snapshot: <https://web.archive.org/web/20261021130104/https://trufflesecurity.com/blog/anyone-can-access-deleted-and-private-repo-data-github>

[From The DigThe Vault Governs the Secret. The Copies Still Work.Read the post](https://trufflesecurity.com/blog/the-vault-governs-the-secret-the-copies-still-work)

https://trufflesecurity.com/

TRUFFLEHOG

[CUSTOMERS](https://trufflesecurity.com/customers)

COMPANY

RESOURCES

[LOG IN](https://trufflehog.org/)

[Contact Us](https://trufflesecurity.com/contact)

[From The DigThe Vault Governs the Secret. The Copies Still Work.Read the post](https://trufflesecurity.com/blog/the-vault-governs-the-secret-the-copies-still-work)

https://trufflesecurity.com/

[Contact Us](https://trufflesecurity.com/contact)

Joe Leon

### [The Dig](https://trufflesecurity.com/blog)

July 24, 2024

# Anyone can Access Deleted and Private Repository Data on GitHub

# Anyone can Access Deleted and Private Repository Data on GitHub

Joe Leon

July 24, 2024

Note: Open source TruffleHog can now discover all of these commits, see our follow-up post: https://trufflesecurity.com/blog/trufflehog-now-finds-all-deleted-and-private-commits-on-github

You can access data from deleted forks, deleted repositories and even private repositories on GitHub. And it is available forever. This is known by GitHub, and intentionally designed that way.

This is such an enormous attack vector for all organizations that use GitHub that we’re introducing a new term: Cross Fork Object Reference (CFOR). A CFOR vulnerability occurs when one repository fork can access sensitive data from another fork (including data from private and deleted forks). Similar to an Insecure Direct Object Reference, in CFOR users supply commit hashes to directly access commit data that otherwise would not be visible to them.

Let’s see a few examples.

## Accessing Deleted Fork Data

Consider this common workflow on GitHub:

-

You fork a public repository

-

You commit code to your fork

-

You delete your fork

Is the code you committed to the fork still accessible? It shouldn’t be, right? You deleted it.

It is. And it’s accessible forever. Out of your control.

In the video below, you’ll see us fork a repository, commit data to it, delete the fork, and then access the “deleted” commit data via the original repository.

You might think you’re protected by needing to know the commit hash. You’re not. The hash is discoverable. More on that later.

#### How often can we find data from deleted forks?

Pretty often. We surveyed a few (literally 3) commonly-forked public repositories from a large AI company and easily found 40 valid API keys from deleted forks. The user pattern seemed to be this:

-

Fork the repo.

-

Hard-code an API key into an example file.

-

<Do Work>

-

Delete the fork.

But this gets worse, it works in reverse too:

## Accessing Deleted Repo Data

Consider this scenario:

-

You have a public repo on GitHub.

-

A user forks your repo.

-

You commit data after they fork it (and they never sync their fork with your updates).

-

You delete the entire repo.

Is the code you committed after they forked your repo still accessible?

Yep.

GitHub stores repositories and forks in a [repository network](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/about-permissions-and-visibility-of-forks#about-visibility-of-forks), with the original “upstream” repository acting as the root node. [When a public “upstream” repository that has been forked is “deleted”, GitHub reassigns the root node role to one of the downstream forks](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/what-happens-to-forks-when-a-repository-is-deleted-or-changes-visibility#deleting-a-public-repository). However, all of the commits from the “upstream” repository still exist and are accessible via any fork.

In the video below, we create a repo, fork it and then show how data not synced with the fork can still be accessed by the fork after the original repo is deleted.

This isn’t just some weird edge case scenario. This unfolded last week:

I submitted a P1 vulnerability to a major tech company showing they accidentally committed a private key for an employee’s GitHub account that had significant access to their entire GitHub organization. They immediately deleted the repository, but since it had been forked, I could still access the commit containing the sensitive data via a fork, despite the fork never syncing with the original “upstream” repository.

The implication here is that any code committed to a public repository may be accessible forever as long as there is at least one fork of that repository.

It gets worse.

## Accessing Private Repo Data

Consider this common workflow for open-sourcing a new tool on GitHub:

-

You create a private repo that will eventually be made public.

-

You create a private, internal version of that repo (via forking) and commit additional code for features that you’re not going to make public.

-

You make your “upstream” repository public and keep your fork private.

Are your private features and related code (from step 2) viewable by the public?

Yes. Any code committed between the time you created an internal fork of your tool and when you open-sourced the tool, those commits are accessible on the public repository.

Any commits made to your private fork after you make the “upstream” repository public are not viewable. That’s because changing the visibility of a private “upstream” repository results in two repository networks - one for the private version, and one for the public version.

In the video below, we demonstrate how organizations open-source new tools while maintaining private internal forks, and then show how someone could access commit data from the private internal version via the public one.

Unfortunately, this workflow is one of the most common approaches users and organizations take to developing open-source software. As a result, it’s possible that confidential data and secrets are inadvertently being exposed on an organization's public GitHub repositories.

## How do you actually access the data?

By directly accessing the commit.

Destructive actions in GitHub’s repository network (like the 3 scenarios mentioned above) remove references to commit data from the standard GitHub UI and normal git operations. However, this data still exists and is accessible (if you know the commit hash). This is the tie-in between CFOR and IDOR vulnerabilities - if you know the commit hash you can directly access data that is not intended for you.

Commit hashes are SHA-1 values.

If a user knows the SHA-1 commit hash of a particular commit they want to see, they can directly navigate to that commit at the endpoint: https://github.com/<user/org>/<repo>/commit/<commit_hash>. They’ll see a yellow banner explaining that “[t]his commit does not belong to any branch of this repository, and may belong to a fork outside of the repository.”

Where do you get these hash values?

Commit hashes can be brute forced through GitHub’s UI, particularly because the git protocol permits the use of [short SHA-1 values](https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection#:~:text=to%20any%20commit.-,Short%20SHA%2D1,-Git%20is%20smart) when referencing a commit. A short SHA-1 value is the minimum number of characters required to avoid a collision with another commit hash, with an absolute minimum of 4. The keyspace of all 4 character SHA-1 values is 65,536 (16^4). Brute forcing all possible values can be achieved relatively easily.

For example, consider this commit in TruffleHog’s repository:

To access this commit, users typically visit the URL containing the full SHA-1 commit hash: https://github.com/trufflesecurity/trufflehog/commit/07f01e8337c1073d2c45bb12d688170fcd44c637

But users don’t need to know the entire 32 character SHA-1 value, they only need to correctly guess the Short SHA-1 value, which in this case is 07f01e.

https://github.com/trufflesecurity/trufflehog/commit/07f01e

But what’s more interesting; GitHub exposes a public events API endpoint. You can also query for commit hashes in the [events archive](https://www.gharchive.org/) which is managed by a 3rd party, and saves all GitHub events for the past decade outside of GitHub, even after the repos get deleted.

## GitHub’s Policies

We recently submitted our findings to GitHub via their VDP program. This was their response:

After reviewing the documentation, it’s clear as day that GitHub designed repositories to work like this.

https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/what-happens-to-forks-when-a-repository-is-deleted-or-changes-visibility

We appreciate that GitHub is transparent about their architecture and has taken the time to clearly document what users should expect to happen in the instances documented above.

Our issue is this:

The average user views the separation of private and public repositories as a security boundary, and understandably believes that any data located in a private repository cannot be accessed by public users. Unfortunately, as we documented above, that is not always true. Whatsmore, the act of deletion implies the destruction of data. As we saw above, deleting a repository or fork does not mean your commit data is actually deleted.

## Implications

We have a few takeaways from this:

-

As long as one fork exists, any commit to that repository network (ie: commits on the “upstream” repo or “downstream” forks) will exist forever.

-

This further cements our view that the only way to securely remediate a leaked key on a public GitHub repository is through key rotation. We’ve spent a lot of time documenting how to rotate keys for the most popularly leaked secret types - check our work out here: [howtorotate.com](https://howtorotate.com/docs/introduction/getting-started/).

-

GitHub’s repository architecture necessitates these design flaws and unfortunately, the vast majority of GitHub users will never understand how a repository network actually works and will be less secure because of it.

-

As secret scanning evolves, and we can hopefully scan all commits in a repository network, we’ll be alerting on secrets that might not be our own (ie: they might belong to someone who forked a repository). This will require more diligent triaging.

-

While these three scenarios are shocking, that doesn’t even cover all of the ways GitHub could be storing deleted data from your repositories. Check out our [recent post](https://trufflesecurity.com/blog/trufflehog-scans-deleted-git-branches) (and related TruffleHog update) about how you also need to scan for secrets in deleted branches.

Finally, while our research focused on GitHub, it’s important to note that some of these issues exist on other version control system products.

## [More from THE DIG](https://trufflesecurity.com/blog)

Thoughts, research findings, reports, and more from Truffle Security Co.

[Oct 29, 2026

###### The Vault Governs the Secret. The Copies Still Work.](https://trufflesecurity.com/blog/the-vault-governs-the-secret-the-copies-still-work)[Oct 27, 2026

###### Truffle Security Announces Collaboration with Google Cloud to Help Secure Exposed Secrets](https://trufflesecurity.com/blog/truffle-security-announces-collaboration-with-google-cloud)[Oct 22, 2026

###### Model Distillation: Why Attackers Want Your API Keys](https://trufflesecurity.com/blog/model-distillation-why-attackers-want-your-api-keys)

# [T](https://trufflesecurity.com/blog)he Dig

Thoughts, research findings, reports, and more from Truffle Security Co.

[Oct 29, 2026

###### The Vault Governs the Secret. The Copies Still Work.](https://trufflesecurity.com/blog/the-vault-governs-the-secret-the-copies-still-work)[Oct 27, 2026

###### Truffle Security Announces Collaboration with Google Cloud to Help Secure Exposed Secrets](https://trufflesecurity.com/blog/truffle-security-announces-collaboration-with-google-cloud)

STAY STRONG

DIG DEEP

TRUFFLEHOG

[Open-source](https://trufflesecurity.com/trufflehog)

[Enterprise](https://trufflesecurity.com/trufflehog-enterprise)

[Analyze](https://trufflesecurity.com/trufflehog-analyze)

[SaaS Analyze](https://trufflesecurity.com/trufflehog-analyze/saas)

[GCP Analyze](https://trufflesecurity.com/trufflehog-analyze/gcp)

[AWS Analyze](https://trufflesecurity.com/trufflehog-analyze/aws)

[Forager](https://trufflesecurity.com/trufflehog-forager)

[Detectors](https://trufflesecurity.com/detectors)

[Integrations](https://trufflesecurity.com/integrations)

[Trust & Security](https://trufflesecurity.com/security)

[Pricing](https://trufflesecurity.com/pricing)

[CUSTOMERS](https://trufflesecurity.com/customers)

[Optro](https://trufflesecurity.com/customers/optro)

[TextNow](https://trufflesecurity.com/customers/textnow)

[Klaviyo](https://trufflesecurity.com/customers/klaviyo)

COMPANY

[About](https://trufflesecurity.com/about)

[Careers](https://trufflesecurity.com/careers)

[Press](https://trufflesecurity.com/press)

[FAQ](https://trufflesecurity.com/faq)

[Partners](https://trufflesecurity.com/partners)

NEW!

[Contact us](https://trufflesecurity.com/contact)

RESOURCES

[Blog](https://trufflesecurity.com/blog)

[Newsletter](https://trufflesecurity.com/newsletter)

[Library](https://trufflesecurity.com/library)

[Events](https://trufflesecurity.com/events)

[Videos](https://trufflesecurity.com/videos)

[GitHub](https://github.com/trufflesecurity)

[Enterprise docs](https://docs.trufflesecurity.com/)

[Open-source docs](https://github.com/trufflesecurity/trufflehog#trufflehog)

[How to rotate](https://howtorotate.com/)

[Brand assets](https://trufflesecurity.com/branding)

DOING IT THE RIGHT WAY

[SINCE 2021](https://trufflesecurity.com/partners)

[#trufflehog-community](https://join.slack.com/t/trufflehog-community/shared_invite/zt-pw2qbi43-Aa86hkiimstfdKH9UCpPzQ)[#Secret Scanning](https://discord.gg/8Hzbrnkr7E)

© 2026 Truffle Security Co.

[Privacy policy](https://trufflesecurity.com/privacy-policy)

[Terms and conditions](https://trufflesecurity.com/terms-conditions)

[Data processing agreement](https://trufflesecurity.com/data-processing-agreement)

[Acceptable use policy](https://trufflesecurity.com/acceptable-use-policy)

STAY STRONG

DIG DEEP

TRUFFLEHOG

[Open-source](https://trufflesecurity.com/trufflehog)

[Enterprise](https://trufflesecurity.com/trufflehog-enterprise)

[Analyze](https://trufflesecurity.com/trufflehog-analyze)

[SaaS Analyze](https://trufflesecurity.com/trufflehog-analyze/saas)

[GCP Analyze](https://trufflesecurity.com/trufflehog-analyze/gcp)

[AWS Analyze](https://trufflesecurity.com/trufflehog-analyze/aws)

[Forager](https://trufflesecurity.com/trufflehog-forager)

[Detectors](https://trufflesecurity.com/detectors)

[Integrations](https://trufflesecurity.com/integrations)

[Trust & Security](https://trufflesecurity.com/security)

[Pricing](https://trufflesecurity.com/pricing)

[CUSTOMERS](https://trufflesecurity.com/customers)

[Optro](https://trufflesecurity.com/customers/optro)

[TextNow](https://trufflesecurity.com/customers/textnow)

[Klaviyo](https://trufflesecurity.com/customers/klaviyo)

COMPANY

[About](https://trufflesecurity.com/about)

[Careers](https://trufflesecurity.com/careers)

[Press](https://trufflesecurity.com/press)

[FAQ](https://trufflesecurity.com/faq)

[Partners](https://trufflesecurity.com/partners)

NEW!

[Contact us](https://trufflesecurity.com/contact)

RESOURCES

[Blog](https://trufflesecurity.com/blog)

[Newsletter](https://trufflesecurity.com/newsletter)

[Library](https://trufflesecurity.com/library)

[Events](https://trufflesecurity.com/events)

[Videos](https://trufflesecurity.com/videos)

[GitHub](https://github.com/trufflesecurity)

[Enterprise docs](https://docs.trufflesecurity.com/)

[Open-source docs](https://github.com/trufflesecurity/trufflehog#trufflehog)

[How to rotate](https://howtorotate.com/)

[Brand assets](https://trufflesecurity.com/branding)

[#trufflehog-community](https://join.slack.com/t/trufflehog-community/shared_invite/zt-pw2qbi43-Aa86hkiimstfdKH9UCpPzQ)[#Secret Scanning](https://discord.gg/8Hzbrnkr7E)

© 2026 Truffle Security Co.

[Privacy policy](https://trufflesecurity.com/privacy-policy)

[Terms and conditions](https://trufflesecurity.com/terms-conditions)

[Data processing agreement](https://trufflesecurity.com/data-processing-agreement)

[Acceptable use policy](https://trufflesecurity.com/acceptable-use-policy)

infra
