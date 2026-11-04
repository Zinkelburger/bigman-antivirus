---
url: "https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/"
title: "Security alert: Attack campaign involving stolen OAuth user tokens issued to two third-party integrators"
site: "The GitHub Blog"
published: "2022-04-15T22:35:02+00:00"
captured: "2026-11-02T18:33:54Z"
via: "live"
sha256: "337518d30de3a649760a307261157fd377f88c7fae32a42d3802d368b1c51098"
wayback: "https://web.archive.org/web/20261020072214/https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Security alert: Attack campaign involving stolen OAuth user tokens issued to two third-party integrators

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/>
> Snapshot: <https://web.archive.org/web/20261020072214/https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/>

[Home](https://github.blog/) / [News & insights](https://github.blog/news-insights/) / [Company news](https://github.blog/news-insights/company-news/)

# Security alert: Attack campaign involving stolen OAuth user tokens issued to two third-party integrators

On April 12, GitHub Security began an investigation that uncovered evidence that an attacker abused stolen OAuth user tokens issued to two third-party OAuth integrators, Heroku and Travis-CI, to download data from dozens of organizations, including npm. Read on to learn more about the impact to GitHub, npm, and our users.

[Mike Hanley](https://github.blog/author/mph4/)·[@mph4](https://github.com/mph4)

April 15, 2022  |  Updated April 27, 2022

|  5 minutes

-   Share:
-  https://x.com/share?text=Security%20alert%3A%20Attack%20campaign%20involving%20stolen%20OAuth%20user%20tokens%20issued%20to%20two%20third-party%20integrators&url=https%3A%2F%2Fgithub.blog%2Fnews-insights%2Fcompany-news%2Fsecurity-alert-stolen-oauth-user-tokens%2F
-  https://www.facebook.com/sharer/sharer.php?t=Security%20alert%3A%20Attack%20campaign%20involving%20stolen%20OAuth%20user%20tokens%20issued%20to%20two%20third-party%20integrators&u=https%3A%2F%2Fgithub.blog%2Fnews-insights%2Fcompany-news%2Fsecurity-alert-stolen-oauth-user-tokens%2F
-  https://www.linkedin.com/shareArticle?title=Security%20alert%3A%20Attack%20campaign%20involving%20stolen%20OAuth%20user%20tokens%20issued%20to%20two%20third-party%20integrators&url=https%3A%2F%2Fgithub.blog%2Fnews-insights%2Fcompany-news%2Fsecurity-alert-stolen-oauth-user-tokens%2F

April 27, 2022 update: Pattern of attacker activity on GitHub.

As of 5:00 PM UTC on April 27, 2022, we are in the process of sending the final expected notifications to GitHub.com customers who had either the Heroku or Travis CI OAuth app integrations authorized in their GitHub accounts.

GitHub’s analysis of the attacker’s behavior reveals the following activities carried out on GitHub.com using stolen OAuth app tokens:

1. The attacker authenticated to the GitHub API using the stolen OAuth tokens issued to Heroku and Travis CI.
2. For most people who had the affected Heroku or Travis CI OAuth apps authorized in their GitHub accounts, the attacker listed all the user’s organizations.
3. The attacker then selectively chose targets based on the listed organizations.
4. The attacker listed the private repositories for user accounts of interest.
5. The attacker then proceeded to clone some of those private repositories.

This pattern of behavior suggests the attacker was only listing organizations in order to identify accounts to selectively target for listing and downloading private repositories. GitHub believes these attacks were highly targeted based on the available information and our analysis of the attacker behavior using the compromised OAuth tokens issued to Travis CI and Heroku.

Following this series of notifications, GitHub will have completed directly notifying each affected user for whom we were able to detect abuse using the stolen OAuth tokens.

Customers should also continue to monitor Heroku and Travis CI for updates on their own investigations into the affected OAuth applications at:

- [Heroku](https://status.heroku.com/incidents/2413)
- [Travis CI](https://blog.travis-ci.com/2022-04-17-securitybulletin)

April 22, 2022 update: GitHub has sent notifications for known victims of repository listing activity using stolen OAuth app tokens.

As of 7:33 PM UTC on April 22, 2022, we’ve notified victims of this campaign whom we have identified as having repository details listed using stolen OAuth app tokens, but did NOT have repository contents downloaded.

The repository listing activity was conducted by the attacker through abuse of third-party OAuth user tokens maintained by Heroku and Travis CI. We do not believe the attacker obtained these tokens via a compromise of GitHub or its systems, because the tokens in question are not stored by GitHub in their original, usable formats which could be abused by an attacker.

The activity was conducted using the /user/repos and /orgs/{org}/repos GitHub API endpoints. Our documentation gives examples of the data returned from requests to these endpoints:

- /user/repos: https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user
- /orgs/{org}/repos: https://docs.github.com/en/rest/reference/repos#list-organization-repositories

Should we identify additional customers who have been affected, we will notify those customers promptly. If you do not receive a notification email from us, that means GitHub has not identified your account as impacted by the current incident.

Customers should also continue to monitor Heroku and Travis CI for updates on their own investigations into the affected OAuth applications.

April 18, 2022 update: GitHub has sent notifications for known victims of third-party OAuth token theft.

As of 9:30 PM UTC on April 18, 2022, we’ve notified victims of this campaign whom we have identified as having repository contents downloaded by an unauthorized party through abuse of third-party OAuth user tokens maintained by Heroku and Travis CI. We do not believe the attacker obtained these tokens via a compromise of GitHub or its systems, because the tokens in question are not stored by GitHub in their original, usable formats which could be abused by an attacker.

Should we identify additional customers who have been affected, we will notify those customers promptly. If you do not receive a notification email from us, that means GitHub has not identified your account as impacted by the current incident.

Customers should also continue to monitor [Heroku](https://status.heroku.com/incidents/2413) and [Travis CI](https://blog.travis-ci.com/2022-04-17-securitybulletin) for updates on their own investigations into the affected OAuth applications.

On April 12, GitHub Security began an investigation that uncovered evidence that an attacker abused stolen OAuth user tokens issued to two third-party OAuth integrators, Heroku and Travis-CI, to download data from dozens of organizations, including [npm](https://github.blog/2020-03-16-npm-is-joining-github/). The applications maintained by these integrators were used by GitHub users, including GitHub itself. We do not believe the attacker obtained these tokens via a compromise of GitHub or its systems, because the tokens in question are not stored by GitHub in their original, usable formats. Following immediate investigation, we disclosed our findings to Heroku and Travis-CI on April 13 and 14; more detail is available below and we will update this blog as we learn more.

Looking across the entire GitHub platform, we have high confidence that compromised OAuth user tokens from Heroku and Travis-CI-maintained OAuth applications were stolen and abused to download private repositories belonging to dozens of victim organizations that were using these apps. Our analysis of other behavior by the threat actor suggests that the actors may be mining the downloaded private repository contents, to which the stolen OAuth token had access, for secrets that could be used to pivot into other infrastructure.

Known-affected OAuth applications as of April 15, 2022:

- Heroku Dashboard (ID: 145909)
- Heroku Dashboard (ID: 628778)
- Heroku Dashboard – Preview (ID: 313468)
- Heroku Dashboard – Classic (ID: 363831)
- Travis CI (ID: 9216)

We are sharing this today as we believe the attacks may be ongoing and action is required for customers to protect themselves.

#### [Impact to GitHub.com and npm](https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/#impact-to-github-com-and-npm)

The initial detection related to this campaign occurred on April 12 when GitHub Security identified unauthorized access to our npm production infrastructure using a compromised AWS API key. Based on subsequent analysis, we believe this API key was obtained by the attacker when they downloaded a set of private npm repositories using a stolen OAuth token from one of the two affected third-party OAuth applications described above. Upon discovering the broader theft of third-party OAuth tokens not stored by GitHub or npm on the evening of April 13, we immediately took action to protect GitHub and npm by revoking tokens associated with GitHub and npm’s internal use of these compromised applications.

We believe that the two impacts to npm are unauthorized access to, and downloading of, the private repositories in the npm organization on GitHub.com and potential access to the npm packages as they exist in AWS S3 storage. At this point, we assess that the attacker did not modify any packages or gain access to any user account data or credentials. We are still working to understand whether the attacker viewed or downloaded private packages. npm uses completely separate infrastructure from GitHub.com; GitHub was not affected in this original attack. Though investigation continues, we have found no evidence that other GitHub-owned private repos were cloned by the attacker using stolen third-party OAuth tokens.

#### [How GitHub responded to protect users of GitHub.com](https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/#how-github-responded-to-protect-users-of-github-com)

Once GitHub identified stolen third-party OAuth tokens affecting GitHub users, GitHub took immediate steps to respond and protect users. GitHub contacted Heroku and Travis-CI to request that they initiate their own security investigations, revoke all OAuth user tokens associated with the affected applications, and begin work to notify their own users.

GitHub remains closely engaged with both organizations in an effort to assist their investigation and recovery efforts, and better protect shared customers.

#### [What GitHub customers and organizations need to know](https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/#what-github-customers-and-organizations-need-to-know)

GitHub is currently working to identify and notify all of the known-affected victim users and organizations that we discovered through our analysis across GitHub.com. These customers will receive a notification email from GitHub with additional details and next steps to assist in their own response within the next 72 hours.

If you do not receive a notification, you and/or your organization have not been identified as affected. GitHub will continue to notify any additional affected users or organizations as they are identified. You should, however, periodically [review what OAuth applications you’ve authorized](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/reviewing-your-authorized-applications-oauth) or are [authorized to access your organization](https://docs.github.com/en/enterprise-cloud@latest/organizations/restricting-access-to-your-organizations-data/about-oauth-app-access-restrictions) and prune anything that’s no longer needed. You can also review your [organization audit logs](https://docs.github.com/en/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/reviewing-the-audit-log-for-your-organization) and [user account security logs](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/reviewing-your-security-log) for unexpected or anomalous activity.

#### [If you have questions or concerns](https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/#if-you-have-questions-or-concerns)

If you have questions or need assistance regarding affected OAuth applications maintained by Heroku, please reach out to Salesforce / Heroku security and support at [help.heroku.com](https://help.heroku.com/), and monitor the [Salesforce Trust](https://trust.salesforce.com) site for additional updates.

If you have questions or need assistance regarding affected OAuth applications maintained by Travis CI, please reach out to compliance@travis-ci.com.

Customers who are directly contacted by GitHub regarding this issue are welcome to contact us according to directions in the notification you received.

For other questions regarding GitHub and npm you can contact [GitHub Support](https://support.github.com/contact?subject=GH-0200941-4895-0+Questions&tags=GH-0200941-4895-0).

#### [Conclusion](https://github.blog/news-insights/company-news/security-alert-stolen-oauth-user-tokens/#conclusion)

The security and trustworthiness of GitHub, npm, and the broader developer ecosystem is our highest priority. Our investigation is ongoing, and we will update this blog, and our communications with affected customers, as we learn more.

## Tags:

-  [npm](https://github.blog/tag/npm/)

##  Written by

###  [Mike Hanley](https://github.blog/author/mph4/)

[@mph4](https://github.com/mph4)

Mike Hanley is the Chief Security Officer and SVP of Engineering at GitHub. Prior to GitHub, Mike was the Vice President of Security at Duo Security, where he built and led the security research, development, and operations functions. After Duo’s acquisition by Cisco for $2.35 billion in 2018, Mike led the transformation of Cisco’s cloud security framework and later served as CISO for the company. Mike also spent several years at CERT/CC as a Senior Member of the Technical Staff and security researcher focused on applied R&D programs for the US Department of Defense and the Intelligence Community.

When he’s not talking about security at GitHub, Mike can be found enjoying Ann Arbor, MI with his wife and eight kids.

##  Related posts

[Company news](https://github.blog/news-insights/company-news/)

###  [GitHub Universe 2026: Evolving every layer of the stack](https://github.blog/news-insights/company-news/github-universe-2026-evolving-every-layer-of-the-stack/)

Today, we laid out our vision for GitHub as the most comprehensive, end-to-end software delivery system in the world.

[News & insights](https://github.blog/news-insights/)

###  [Octoverse: Commits, pushes, reach new highs in 2026 as developers build with agents](https://github.blog/news-insights/octoverse/octoverse-commits-pushes-reach-new-highs-in-2026-as-developers-build-with-agents/)

See how a record year of growth reached every industry, every field, and every kind of builder in the 2026 Octoverse report.

[Company news](https://github.blog/news-insights/company-news/)

###  [The next developer interface is ambient intelligence](https://github.blog/news-insights/company-news/the-next-developer-interface-is-ambient-intelligence/)

AI, agents, voice, and physical controls are shifting software development from operating tools to expressing intent—and helping developers stay focused on building.

## We do newsletters, too

Discover tips, technical guides, and best practices in our biweekly newsletter just for devs.

Your email address
