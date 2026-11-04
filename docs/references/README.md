# Reference archive

Bigman's threat catalogue, research notes and rule explanations cite public pages: vendor write-ups, advisories, papers and manuals. Pages move and sites close, so every citation is kept here as well. A finding's evidence should outlive the page it came from.

- `pages/<site>/<slug>.md` is a readable text copy of one cited page. The front matter records the original URL, the title, when and how it was captured (`live`, or `wayback` when the live page failed), a SHA-256 of the exact bytes received, the newest Wayback Machine snapshot, and which files cite it.
- `index.json` maps every cited URL to its copy, or to the reason it could not be captured. It also lists URLs that are deliberately not archived (`skipped`).

These copies are third-party content. Copyright stays with each publisher, and citations in the docs keep pointing at the original URL. Use a copy here when the original is gone or has changed.

## What gets archived

Every `http(s)` URL in the repository's Markdown, except:

- URLs inside code spans or fenced code blocks. Those are examples, indicators, commands or API endpoints, and are never fetched.
- Placeholder hosts (`.invalid`, `.example`, `.test`, `localhost`, `example.org`), bare IP addresses, and URLs with credentials in them.
- Data feeds and bulk downloads (`.json`, `.xml`, `.rss`, `.zip`, `.csv.gz`, `api.*` hosts).

So an indicator such as an attacker's server stays in backticks, and the archiver never contacts it.

## Adding a citation

Cite the original URL in prose, then run:

```sh
python3 -I scripts/archive_references.py --wayback-save
```

This captures any new citations, records an existing Wayback snapshot, and asks the Wayback Machine to snapshot pages it does not have yet. Commit the new files under `docs/references/` with the doc change.

CI runs `scripts/archive_references.py --check`, which works offline and fails when a citation has never been archived. A page that was attempted but could not be captured (removed, behind a bot check, no snapshot) is listed as a warning and does not fail the check. Retry those with `--retry-failed`. Recapture everything with `--refresh`.

Downloaded pages are parsed as text and never executed. PDFs are converted with `pdftotext` when it is installed.
