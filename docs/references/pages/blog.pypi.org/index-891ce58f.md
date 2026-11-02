---
url: "https://blog.pypi.org/"
title: "The Python Package Index Blog"
captured: "2026-11-02T18:22:54Z"
via: "live"
sha256: "a913319d816b467feccebea09690698c71a5886594d1fb3736291fa61e7d42d4"
wayback: "https://web.archive.org/web/20261019175518/https://blog.pypi.org/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# The Python Package Index Blog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://blog.pypi.org/>
> Snapshot: <https://web.archive.org/web/20261019175518/https://blog.pypi.org/>

-  October 26, 2026
-  1 min read

## [Higher limits for organization projects](https://blog.pypi.org/posts/2026-10-26-higher-limits-for-organization-projects/)

Projects owned by a [PyPI organization](https://docs.pypi.org/organization-accounts/) now get higher limits by default:

|   | Personal projects  | Organization projects

| Largest file you can upload  | 100 MB  | 200 MB

| Total size of a project  | 10 GB  | 20 GB

-  October 23, 2026
-  2 min read

## [PyPI now limits versions to 128 characters](https://blog.pypi.org/posts/2026-10-23-version-length-limit/)

PyPI now rejects new uploads whose version is longer than 128 characters.

This affects almost no one: across the roughly 9.9 million releases on PyPI, only 2 have a version that long. Releases that are already on PyPI stay exactly as they are, and installing them works the same as before.

-  October 16, 2026
-  3 min read

## [Checking email reputation, and allowing Outlook again](https://blog.pypi.org/posts/2026-10-16-email-reputation-and-outlook/)

PyPI improves on checks for the reputation of an email address when used to register an account or add to an existing account, and rejects addresses identified as disposable.

With the new check in place, outlook.com and hotmail.com addresses can once again be used on PyPI.

-  October 14, 2026
-  1 min read

## [PyPI's developer documentation has moved to dev.pypi.org](https://blog.pypi.org/posts/2026-10-14-dev-docs-move-to-dev-pypi-org/)

The developer documentation for [Warehouse](https://github.com/pypi/warehouse), the codebase that powers PyPI, has moved from warehouse.pypa.io to [dev.pypi.org](https://dev.pypi.org/).

Existing links to warehouse.pypa.io will keep working: every page now redirects to the same page on dev.pypi.org.

-  September 8, 2026
-  8 min read

## [Incident Report: File Hosting Errors](https://blog.pypi.org/posts/2026-09-08-file-hosting-errors/)

### [Executive summary](https://blog.pypi.org/posts/2026-09-08-file-hosting-errors/#executive-summary)

For about two weeks in August 2026, some PyPI users hit intermittent 502 and 503 errors downloading files from files.pythonhosted.org, triggering failures during installation from PyPI. Thanks to our users filing reports in the [support tracker](https://github.com/pypi/support); one report in particular narrowed the problem to a single cache node.

Two separate problems were uncovered. A canary deployment inside Fastly's network had triggered a misconfiguration at one cache node, causing Fastly's routing layer to return 502 responses for traffic reaching the affected cache node. Separately, I found and fixed several bugs in our own Fastly configuration around origin fallback and range request behavior. Those had been there for a while, and only surfaced while digging into these reports.

Both intermittent problems are now fixed, and downloads are back to full function since August 28.

-  August 31, 2026
-  4 min read

## [Metadata requests no longer tracked in PyPI download counts](https://blog.pypi.org/posts/2026-08-31-download-counts/)

On 2026-08-24 I [shipped a change to the configuration that emits PyPI's download logs](https://github.com/pypi/infra/pull/242) so that only requests for actual distribution artifacts are counted. A request has to end in .whl, .tar.gz, or .zip to produce a download record.

The counts are more accurate now, and existing systems based on [PyPI's BigQuery dataset](https://docs.pypi.org/api/bigquery/), like [pypistats.org](https://pypistats.org/) will display a different shape going forward.

-  August 14, 2026
-  6 min read

## [How AWS Powers PyPI and the PSF](https://blog.pypi.org/posts/2026-08-14-how-aws-powers-pypi-and-the-psf/)

Working on infrastructure at the Python Software Foundation (PSF) as the Director of Engineering is a broad job with many hats. Python turns up everywhere. It's in healthcare systems and government agencies, in research labs and classrooms, in one-person side projects and in infrastructure at companies with six-figure headcounts. Somebody is using it to analyze a genome right now and somebody else is using it to automate a spreadsheet they hate arranging by hand. That range has always been the interesting part of the PSF's work, and it's only broadened in the last couple of years as AI continues to pull people from many disciplines into writing Python.

-  August 11, 2026
-  4 min read

## [The HTML representation of the index API is now frozen](https://blog.pypi.org/posts/2026-08-11-html-index-is-frozen/)

PyPI has adopted [PEP 833](https://peps.python.org/pep-0833/), which "freezes" the [HTML representation](https://packaging.python.org/en/latest/specifications/simple-repository-api/#html-serialization) of the [index API](https://docs.pypi.org/api/index-api/), which is also sometimes called the "simple API" or the "simple repository API."

New packages and releases will continue to appear in the HTML representation, meaning this has no breaking implications for downstream index consumers.

However, future standardization and development efforts will focus on the JSON representation, and downstreams that consume only the HTML representation are strongly encouraged to transition to the JSON representation to ensure access to the latest and greatest index features.

-  July 22, 2026
-  2 min read

## [Releases now reject new files after 14 days](https://blog.pypi.org/posts/2026-07-22-releases-now-reject-new-files-after-14-days/)

The Python Package Index (PyPI) now rejects new files being uploaded to releases that are older than 14 days. This restriction was [put in place](https://github.com/pypi/warehouse/pull/19727) to prevent old and long-stable releases from being poisoned in case publishing tokens or workflows of PyPI projects were compromised. As far as we are aware this has not yet been abused, but there is no technical reason beyond that attackers weren't aware it was possible.

-  July 22, 2026
-  4 min read

## [Planned Updates to the PyPI User Interface](https://blog.pypi.org/posts/2026-07-22-ui-updates/)

Over the next few months, we will be rolling out changes to the PyPI user interface, improving how we surface security signals and updating the pages where users view package details.

Updates will be staged to [TestPyPI](https://test.pypi.org/) and deployed to production in phases. This approach allows our team to thoroughly test the UI with production-like data, while providing the community with an opportunity to share feedback.

The first phase of changes is now staged on [TestPyPI](https://test.pypi.org/) and ready for users to review!
