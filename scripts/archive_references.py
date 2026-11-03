#!/usr/bin/env python3
"""Keep a copy of every page the docs cite, so findings survive link rot.

Each URL cited in the repository's Markdown is captured as readable text under
docs/references/pages/, with its Wayback Machine snapshot recorded in
docs/references/index.json. If the live page is gone, the Wayback copy is used.

Only citations are archived. URLs inside code spans or fenced code blocks are
examples, indicators or API endpoints and are never fetched. Neither are
placeholder hosts (.invalid, .example, localhost) or bulk data files.

    python3 -I scripts/archive_references.py                 # capture new citations
    python3 -I scripts/archive_references.py --wayback-save  # also request missing snapshots
    python3 -I scripts/archive_references.py --check         # offline: list uncaptured citations
    python3 -I scripts/archive_references.py --url URL       # capture a page that is not cited yet

Downloaded pages are third-party data: they are parsed as text and never run.
"""
from __future__ import annotations

import argparse
import concurrent.futures
import datetime as dt
import hashlib
import html
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import threading
import time
import urllib.error
import urllib.parse
import urllib.request
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "docs" / "references"
INDEX = OUT / "index.json"
PAGES = OUT / "pages"

USER_AGENT = (
    "Mozilla/5.0 (compatible; BigmanReferenceArchiver/1.0; "
    "+https://github.com/Zinkelburger/bigman-antivirus)"
)
TIMEOUT = 30
MAX_BYTES = 20 * 1024 * 1024
HOST_DELAY = 1.0  # seconds between requests to the same site
ARCHIVE_DELAY = 1.0  # seconds between archive.org lookups
SAVE_DELAY = 8.0  # seconds between Save Page Now requests

NOTICE = (
    "> Archived copy of a third-party page, kept so Bigman's findings survive if the "
    "original goes away. Copyright stays with the original publisher; cite the original URL."
)

# Placeholder or local hosts used as examples in the docs.
PLACEHOLDER_SUFFIXES = (".invalid", ".example", ".test", ".tld", ".localhost", ".local")
PLACEHOLDER_HOSTS = {"localhost", "example.com", "example.org", "example.net"}
# Data endpoints and bulk downloads: cited as sources of data, not as articles.
DATA_EXTENSIONS = (
    ".gz", ".zip", ".xz", ".bz2", ".zst", ".tar", ".tgz", ".csv", ".json", ".xml",
    ".rss", ".atom", ".opml", ".txt.gz", ".deb", ".rpm", ".iso", ".exe", ".bin",
)
DATA_HOSTS = {"services.nvd.nist.gov", "osv-vulnerabilities.storage.googleapis.com"}


# ---------------------------------------------------------------- citations

FENCE = re.compile(r"^(\s*)(`{3,}|~{3,})")
CODE_SPAN = re.compile(r"(`+)(.+?)\1", re.S)
URL = re.compile(r"https?://[^\s<>\"'`\]\[]+")


def markdown_files() -> list[Path]:
    out = subprocess.run(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "*.md"],
        cwd=ROOT, capture_output=True, text=True, check=True,
    ).stdout.split()
    skip = ("docs/references/", "node_modules/", "target/")
    return [ROOT / p for p in out if not p.startswith(skip)]


def strip_code(text: str) -> str:
    """Drop fenced blocks and inline code spans, keeping line structure."""
    kept, fence = [], None
    for line in text.splitlines():
        m = FENCE.match(line)
        if fence:
            if m and m.group(2)[0] == fence[0] and len(m.group(2)) >= len(fence):
                fence = None
            kept.append("")
            continue
        if m:
            fence = m.group(2)
            kept.append("")
            continue
        kept.append(line)
    return CODE_SPAN.sub(" ", "\n".join(kept))


def clean_url(raw: str) -> str:
    url = raw.rstrip(".,;:!?*_")
    # Drop a closing parenthesis that belongs to the surrounding prose or link.
    while url.endswith(")") and url.count(")") > url.count("("):
        url = url[:-1].rstrip(".,;:!?*_")
    return url


def skip_reason(url: str) -> str | None:
    try:
        parts = urllib.parse.urlsplit(url)
    except ValueError:
        return "unparseable"
    host = (parts.hostname or "").lower()
    if parts.username or parts.password or "@" in parts.netloc:
        return "credentials or userinfo in URL (example)"
    if not host or "." not in host or host != (parts.hostname or ""):
        return "placeholder host"
    if host in PLACEHOLDER_HOSTS or host.endswith(PLACEHOLDER_SUFFIXES):
        return "placeholder host"
    if re.fullmatch(r"[\d.]+", host) or host.startswith("["):
        return "bare IP address"
    if host in DATA_HOSTS or host.startswith("api."):
        return "API or data endpoint"
    if parts.path.lower().endswith(DATA_EXTENSIONS):
        return "bulk data file"
    if "..." in url or "<" in url:
        return "elided example"
    return None


def citations() -> tuple[dict[str, list[str]], dict[str, str]]:
    """URL -> files citing it, and URL -> reason for URLs that are not archived."""
    cited: dict[str, set[str]] = {}
    skipped: dict[str, str] = {}
    for path in markdown_files():
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        rel = path.relative_to(ROOT).as_posix()
        for raw in URL.findall(strip_code(text)):
            url = clean_url(raw)
            reason = skip_reason(url)
            if reason:
                skipped[url] = reason
            else:
                cited.setdefault(url, set()).add(rel)
    return {u: sorted(f) for u, f in sorted(cited.items())}, dict(sorted(skipped.items()))


# ---------------------------------------------------------------- fetching

class Pace:
    """At most one request per `delay` seconds for each key."""

    def __init__(self, delay: float):
        self.delay = delay
        self.lock = threading.Lock()
        self.next: dict[str, float] = {}

    def wait(self, key: str) -> None:
        with self.lock:
            now = time.monotonic()
            at = max(now, self.next.get(key, 0.0))
            self.next[key] = at + self.delay
        if at > now:
            time.sleep(at - now)


HOSTS = Pace(HOST_DELAY)
ARCHIVE = Pace(ARCHIVE_DELAY)


class DataFile(Exception):
    """A feed or bulk download: cited as a data source, not as a page to keep."""


FEED_TYPES = ("application/rss+xml", "application/atom+xml", "application/xml", "text/xml")


class Fetched:
    def __init__(self, url: str, status: int, ctype: str, charset: str | None, body: bytes):
        self.url, self.status, self.ctype, self.charset, self.body = url, status, ctype, charset, body


def fetch(url: str, pace: Pace, key: str, timeout: int = TIMEOUT) -> Fetched:
    pace.wait(key)
    req = urllib.request.Request(url, headers={
        "User-Agent": USER_AGENT,
        "Accept": "text/html,application/xhtml+xml,application/pdf;q=0.9,text/plain;q=0.8,*/*;q=0.5",
        "Accept-Language": "en",
    })
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        body = resp.read(MAX_BYTES + 1)
        if len(body) > MAX_BYTES:
            raise DataFile(f"larger than {MAX_BYTES // (1024 * 1024)} MB, treated as a data download")
        msg = resp.headers
        return Fetched(resp.geturl(), resp.status, msg.get_content_type(), msg.get_content_charset(), body)


class LookupFailed(Exception):
    pass


def wayback_lookup(url: str) -> str | None:
    """The newest snapshot that returned HTTP 200, or None if there is none.

    Raises LookupFailed when archive.org could not be asked, so "no snapshot" is
    never confused with "could not check".
    """
    api = ("https://web.archive.org/cdx/search/cdx?output=json&limit=-1&fl=timestamp,original"
           "&filter=statuscode:200&url=" + urllib.parse.quote(url, safe=""))
    last = ""
    for attempt in range(4):
        try:
            rows = json.loads(fetch(api, ARCHIVE, "archive.org", timeout=60).body or b"[]")
            if len(rows) < 2:
                return None
            ts, original = rows[-1][0], rows[-1][1]
            return f"https://web.archive.org/web/{ts}/{original}"
        except urllib.error.HTTPError as e:
            last = f"HTTP {e.code}"
            time.sleep((30 if e.code == 429 else 5) * (attempt + 1))
        except (urllib.error.URLError, TimeoutError, ValueError, OSError) as e:
            last = f"{type(e).__name__}: {e}"
            time.sleep(5 * (attempt + 1))
    raise LookupFailed(last)


def wayback_raw(snapshot: str) -> str:
    """Snapshot URL -> the original bytes, without the Wayback toolbar."""
    m = re.match(r"https?://web\.archive\.org/web/(\d+)[a-z_]*/(.+)", snapshot)
    return f"https://web.archive.org/web/{m.group(1)}id_/{m.group(2)}" if m else snapshot


def wayback_save(url: str) -> str | None:
    """Ask Save Page Now for a snapshot. Returns its URL, or None."""
    for attempt in range(4):
        try:
            ARCHIVE.wait("archive.org-save")
            req = urllib.request.Request(
                "https://web.archive.org/save/" + url, headers={"User-Agent": USER_AGENT})
            with urllib.request.urlopen(req, timeout=60) as resp:
                loc = resp.headers.get("Content-Location") or ""
                final = resp.geturl()
            if loc.startswith("/web/"):
                return "https://web.archive.org" + loc
            if "/web/" in final:
                return final.replace("http://", "https://", 1)
            try:
                return wayback_lookup(url)
            except LookupFailed:
                return None
        except urllib.error.HTTPError as e:
            if e.code == 429:
                time.sleep(60 * (attempt + 1))
                continue
            if e.code in (502, 503, 504, 520, 523):
                time.sleep(20)
                continue
            return None
        except (urllib.error.URLError, TimeoutError, OSError):
            return None  # the caller stops after several of these in a row
    return None


# ---------------------------------------------------------------- extraction

SKIP_TAGS = {"script", "style", "noscript", "svg", "template", "nav", "footer", "aside",
             "form", "iframe", "button", "select", "dialog", "canvas", "picture", "video", "audio"}
BLOCK_TAGS = {"p", "div", "section", "article", "main", "header", "blockquote", "figure",
              "figcaption", "table", "tr", "ul", "ol", "dl", "dt", "dd", "hr", "details", "summary"}
VOID_TAGS = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
             "source", "track", "wbr"}


class Extract(HTMLParser):
    """HTML -> plain Markdown-ish text, separately for <article>, <main> and <body>."""

    def __init__(self, base: str):
        super().__init__(convert_charrefs=True)
        self.base = base
        self.skip = 0
        self.skipped: list[str] = []  # open hidden elements, innermost last
        self.pre = 0
        self.depth = {"article": 0, "main": 0}
        self.out = {"article": [], "main": [], "body": []}
        self.title = ""
        self.in_title = False
        self.meta: dict[str, str] = {}
        self.links: list[str | None] = []

    def emit(self, s: str) -> None:
        self.out["body"].append(s)
        for k in ("article", "main"):
            if self.depth[k]:
                self.out[k].append(s)

    def handle_starttag(self, tag, attrs):
        a = dict(attrs)
        if tag == "meta":
            name = (a.get("property") or a.get("name") or "").lower()
            if name in ("og:title", "og:site_name", "article:published_time", "author",
                        "og:description", "description") and a.get("content"):
                self.meta.setdefault(name, a["content"].strip())
            return
        if tag == "title":
            self.in_title = True
            return
        if tag in VOID_TAGS:
            if tag == "br" and not self.skip:
                self.emit("\n")
            return
        if self.skip:
            # Only same-named tags matter: they decide which end tag closes the skip.
            if tag in self.skipped:
                self.skipped.append(tag)
                self.skip = len(self.skipped)
            return
        if tag in SKIP_TAGS or a.get("aria-hidden") == "true" or "hidden" in a:
            # Hidden containers are dropped with everything inside them.
            self.skipped.append(tag)
            self.skip = len(self.skipped)
            return
        if tag in self.depth:
            self.depth[tag] += 1
        if tag == "pre":
            self.pre += 1
            self.emit("\n\n```\n")
        elif re.fullmatch(r"h[1-6]", tag):
            self.emit("\n\n" + "#" * int(tag[1]) + " ")
        elif tag == "li":
            self.emit("\n- ")
        elif tag in ("td", "th"):
            self.emit(" | ")
        elif tag in BLOCK_TAGS:
            self.emit("\n\n")
        elif tag == "a":
            href = a.get("href") or ""
            href = urllib.parse.urljoin(self.base, href) if href else ""
            self.links.append(href if href.startswith(("http://", "https://")) else None)
            self.emit("\x00")  # start-of-link marker, resolved at the end tag

    def handle_endtag(self, tag):
        if tag == "title":
            self.in_title = False
            return
        if self.skip:
            if tag in self.skipped:
                # Close the innermost open skipped element of this name.
                del self.skipped[len(self.skipped) - 1 - self.skipped[::-1].index(tag):]
                self.skip = len(self.skipped)
            return
        if tag in self.depth and self.depth[tag]:
            self.depth[tag] -= 1
        if tag == "pre" and self.pre:
            self.pre -= 1
            self.emit("\n```\n\n")
        elif re.fullmatch(r"h[1-6]", tag) or tag in BLOCK_TAGS:
            self.emit("\n\n")
        elif tag == "a" and self.links:
            href = self.links.pop()
            self.emit("\x01" + (href or "") + "\x02")

    def handle_data(self, data):
        if self.in_title:
            self.title += data
            return
        if self.skip:
            return
        self.emit(data if self.pre else re.sub(r"\s+", " ", data))

    def text(self) -> str:
        """The narrowest region that still holds the page's text.

        Many sites wrap related-post cards in <article>, so <article> wins only when
        it holds at least half of what <main> (or <body>) holds.
        """
        t = {k: finish("".join(v)) for k, v in self.out.items()}
        outer = t["main"] if len(t["main"]) >= 600 else t["body"]
        if len(t["article"]) >= 600 and len(t["article"]) * 2 >= len(outer):
            return t["article"]
        return outer


LINK = re.compile(r"\x00([^\x00\x01]*)\x01([^\x02]*)\x02")


def finish(raw: str) -> str:
    def link(m):
        label, href = m.group(1).strip(), m.group(2)
        if not href or not label or label == href:
            return label or href
        return f"[{label}]({href})"

    for _ in range(3):  # nested anchors resolve inside-out
        raw = LINK.sub(link, raw)
    raw = raw.replace("\x00", "").replace("\x01", "").replace("\x02", "")
    lines, in_pre = [], False
    for line in raw.split("\n"):
        if line.strip() == "```":
            in_pre = not in_pre
            lines.append("```")
            continue
        lines.append(line.rstrip() if in_pre else line.strip())
    text = "\n".join(lines)
    text = re.sub(r"\n{3,}", "\n\n", text)
    text = re.sub(r"(\n- \s*)+\n", "\n", text)  # empty list items
    return text.strip() + "\n"


def decode(body: bytes, charset: str | None) -> str:
    if not charset:
        m = re.search(rb"<meta[^>]+charset=[\"']?([\w-]+)", body[:4096], re.I)
        charset = m.group(1).decode("ascii", "replace") if m else "utf-8"
    try:
        return body.decode(charset, errors="replace")
    except LookupError:
        return body.decode("utf-8", errors="replace")


def pdf_text(body: bytes) -> str:
    if not shutil.which("pdftotext"):
        raise ValueError("PDF and pdftotext is not installed")
    with tempfile.TemporaryDirectory() as d:
        src = Path(d) / "in.pdf"
        src.write_bytes(body)
        out = subprocess.run(["pdftotext", "-layout", "-enc", "UTF-8", str(src), "-"],
                             capture_output=True, timeout=120)
        if out.returncode != 0:
            raise ValueError("pdftotext: " + out.stderr.decode("utf-8", "replace").strip())
        return out.stdout.decode("utf-8", "replace")


def render(got: Fetched) -> tuple[str, str, dict[str, str]]:
    """(title, text, meta) for a fetched page."""
    body = got.body
    if got.ctype == "application/pdf" or body[:5] == b"%PDF-":
        text = pdf_text(body)
        first = next((ln.strip() for ln in text.splitlines() if len(ln.strip()) > 8), "")
        return re.sub(r"\s+", " ", first)[:120], text, {}
    if got.ctype in FEED_TYPES:
        raise DataFile(f"{got.ctype} feed")
    if got.ctype in ("text/plain", "text/markdown", "text/x-markdown"):
        return "", decode(body, got.charset), {}
    if "html" not in got.ctype and not body.lstrip()[:15].lower().startswith((b"<!doctype", b"<html")):
        raise ValueError(f"unsupported content type {got.ctype}")
    p = Extract(got.url)
    p.feed(decode(body, got.charset))
    p.close()
    title = (p.meta.get("og:title") or html.unescape(p.title)).strip()
    return re.sub(r"\s+", " ", title), p.text(), p.meta


# ---------------------------------------------------------------- archive

def page_path(url: str) -> Path:
    parts = urllib.parse.urlsplit(url)
    host = (parts.hostname or "unknown").lower().removeprefix("www.")
    slug = re.sub(r"[^A-Za-z0-9]+", "-", (parts.path + "-" + parts.query).strip("/")).strip("-").lower()
    slug = (slug[:80].rstrip("-") or "index") + "-" + hashlib.sha256(url.encode()).hexdigest()[:8]
    return PAGES / host / f"{slug}.md"


def now_iso() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def yaml_str(s: str) -> str:
    return json.dumps(s, ensure_ascii=False)


def write_page(url: str, entry: dict, text: str) -> None:
    path = ROOT / entry["file"]
    path.parent.mkdir(parents=True, exist_ok=True)
    head = ["---", f"url: {yaml_str(url)}"]
    for key in ("title", "site", "published", "captured", "via", "source_url", "sha256", "wayback"):
        if entry.get(key):
            head.append(f"{key}: {yaml_str(entry[key])}")
    head += ["cited_in:"] + [f"  - {yaml_str(f)}" for f in entry.get("cited_in", [])] + ["---", ""]
    title = entry.get("title") or url
    body = "\n".join(head) + f"\n# {title}\n\n{NOTICE}\n>\n> Original: <{url}>\n"
    if entry.get("wayback"):
        body += f"> Snapshot: <{entry['wayback']}>\n"
    body += "\n" + text
    tmp = path.with_suffix(".tmp")
    tmp.write_text(body, encoding="utf-8")
    tmp.replace(path)


def capture(url: str, cited_in: list[str], known_snapshot: str | None = None) -> dict:
    """Capture one URL: live first, Wayback if the live page fails.

    `known_snapshot` is kept when archive.org cannot be asked this time.
    """
    entry: dict = {"file": page_path(url).relative_to(ROOT).as_posix(), "cited_in": cited_in}
    errors = []
    try:
        snapshot = wayback_lookup(url)
    except LookupFailed as e:
        snapshot = known_snapshot
        if not snapshot:
            entry["wayback_error"] = str(e)
    entry["wayback"] = snapshot
    attempts = [("live", url, HOSTS, urllib.parse.urlsplit(url).hostname or "")]
    if snapshot:
        attempts.append(("wayback", wayback_raw(snapshot), ARCHIVE, "archive.org"))
    for via, src, pace, key in attempts:
        try:
            got = fetch(src, pace, key)
            title, text, meta = render(got)
            # A bot check or an app shell rendered by script is not a copy of the page,
            # even when there is no snapshot to fall back on.
            if len(text.strip()) < 200:
                raise ValueError("page text under 200 characters (likely a bot check or a script-rendered page)")
            entry.update({
                "title": title,
                "site": meta.get("og:site_name"),
                "published": meta.get("article:published_time"),
                "captured": now_iso(),
                "via": via,
                "source_url": got.url if got.url != url else None,
                "http_status": got.status,
                "content_type": got.ctype,
                "sha256": hashlib.sha256(got.body).hexdigest(),
                "error": None,
            })
            write_page(url, entry, text)
            return {k: v for k, v in entry.items() if v is not None}
        except DataFile as e:
            entry.pop("file")
            entry["data"] = str(e)
            entry["attempted"] = now_iso()
            return {k: v for k, v in entry.items() if v is not None}
        except urllib.error.HTTPError as e:
            errors.append(f"{via}: HTTP {e.code}")
        except Exception as e:  # noqa: BLE001 - every failure is recorded, none stop the run
            errors.append(f"{via}: {type(e).__name__}: {e}")
    entry["error"] = "; ".join(errors)
    entry["attempted"] = now_iso()
    entry.pop("file")
    return {k: v for k, v in entry.items() if v is not None}


def load_index() -> dict:
    if INDEX.exists():
        return json.loads(INDEX.read_text(encoding="utf-8"))
    return {"version": 1, "references": {}, "skipped": {}}


def save_index(index: dict) -> None:
    # Sorted copies only: the caller keeps adding to the live dicts.
    OUT.mkdir(parents=True, exist_ok=True)
    out = {
        "version": index.get("version", 1),
        "references": dict(sorted(index["references"].items())),
        "skipped": dict(sorted(index["skipped"].items())),
    }
    tmp = INDEX.with_suffix(".tmp")
    tmp.write_text(json.dumps(out, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
    tmp.replace(INDEX)


def read_front_matter(path: Path) -> dict:
    """The front matter written by write_page, as a dict."""
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        return {}
    head = text[4:text.index("\n---\n", 4)]
    meta: dict = {}
    for line in head.splitlines():
        if line.startswith("  - "):
            meta.setdefault("cited_in", []).append(json.loads(line[4:]))
        elif ": " in line:
            key, value = line.split(": ", 1)
            meta[key] = json.loads(value)
    return meta


def reindex(index: dict) -> int:
    """Add captured pages missing from the index, from their front matter."""
    added = 0
    for page in sorted(PAGES.glob("*/*.md")):
        meta = read_front_matter(page)
        url = meta.pop("url", None)
        if url and not captured(index["references"].get(url)):
            meta["file"] = page.relative_to(ROOT).as_posix()
            index["references"][url] = meta
            added += 1
    return added


def captured(entry: dict | None) -> bool:
    return bool(entry and entry.get("file") and (ROOT / entry["file"]).exists())


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true", help="offline: exit 1 if a citation has no captured copy")
    ap.add_argument("--refresh", action="store_true", help="recapture pages that are already archived")
    ap.add_argument("--retry-failed", action="store_true", help="retry URLs whose last capture failed")
    ap.add_argument("--wayback-save", action="store_true",
                    help="ask the Wayback Machine to snapshot captured URLs that have no snapshot")
    ap.add_argument("--url", action="append", default=[], help="also capture this URL (repeatable)")
    ap.add_argument("--jobs", type=int, default=8, help="sites fetched in parallel (default 8)")
    ap.add_argument("--limit", type=int, default=0, help="capture at most N URLs this run")
    args = ap.parse_args()

    cited, skipped = citations()
    for u in args.url:
        cited.setdefault(u, [])
    index = load_index()
    refs = index["references"]
    index["skipped"] = skipped
    if (n := reindex(index)):
        print(f"recovered {n} captured pages missing from index.json", flush=True)
    for url, files in cited.items():  # keep citation lists current
        if url in refs and files:
            refs[url]["cited_in"] = files

    if args.check:
        # A citation that was never attempted fails the check. One that was tried
        # and could not be captured (site gone, bot wall, no snapshot) is listed but
        # does not, because no rerun can fix it.
        never = [u for u in cited if u not in refs]
        data = [u for u in cited if u in refs and refs[u].get("data")]
        failed = [u for u in cited if u in refs and not captured(refs[u]) and u not in data]
        for u in failed:
            print(f"not captured: {u}  ({refs[u].get('error')})")
        for u in never:
            print(f"not archived yet: {u}  cited in {', '.join(cited[u])}")
        ok = len(cited) - len(never) - len(failed) - len(data)
        print(f"{ok}/{len(cited)} citations captured, {len(data)} are feeds or data downloads, "
              f"{len(failed)} could not be captured, {len(never)} not archived yet; "
              f"{len(skipped)} URLs skipped as examples or data")
        if never:
            print("Run: python3 -I scripts/archive_references.py --wayback-save")
        return 1 if never else 0

    todo = [u for u in cited
            if args.refresh or not refs.get(u)
            or (args.retry_failed and not captured(refs.get(u)) and not refs[u].get("data"))]
    if args.limit:
        todo = todo[: args.limit]
    print(f"{len(cited)} citations, {len(todo)} to capture, {len(skipped)} skipped as examples or data", flush=True)

    by_host: dict[str, list[str]] = {}
    for u in todo:
        by_host.setdefault(urllib.parse.urlsplit(u).hostname or "", []).append(u)

    lock = threading.Lock()
    done = [0]

    def run_host(urls: list[str]) -> None:
        for u in urls:
            entry = capture(u, cited[u], (refs.get(u) or {}).get("wayback"))
            with lock:
                refs[u] = entry
                done[0] += 1
                if "file" in entry:
                    status = "ok " + entry.get("via", "")
                elif entry.get("data"):
                    status = "data, not archived: " + entry["data"]
                else:
                    status = "FAILED " + entry.get("error", "")
                print(f"[{done[0]}/{len(todo)}] {status}  {u}", flush=True)
                if done[0] % 20 == 0:
                    save_index(index)

    try:
        with concurrent.futures.ThreadPoolExecutor(max_workers=max(1, args.jobs)) as pool:
            for f in [pool.submit(run_host, urls) for urls in by_host.values()]:
                f.result()
    finally:
        save_index(index)

    if args.wayback_save:
        need = [u for u in cited if refs.get(u) and not refs[u].get("wayback")]
        print(f"requesting Wayback snapshots for {len(need)} URLs", flush=True)
        ARCHIVE.delay = SAVE_DELAY
        misses = 0
        try:
            for i, u in enumerate(need, 1):
                snap = wayback_save(u)
                misses = 0 if snap else misses + 1
                if misses >= 10:
                    print("Save Page Now failed 10 times in a row; stopping. Rerun later "
                          "with --wayback-save to continue.", flush=True)
                    break
                if snap:
                    refs[u]["wayback"] = snap
                    if captured(refs[u]):
                        page = ROOT / refs[u]["file"]
                        text = page.read_text(encoding="utf-8")
                        if "\nwayback: " not in text.split("\n---\n", 1)[0]:
                            text = text.replace("\ncited_in:", f"\nwayback: {yaml_str(snap)}\ncited_in:", 1)
                            text = text.replace(f"> Original: <{u}>\n", f"> Original: <{u}>\n> Snapshot: <{snap}>\n", 1)
                            page.write_text(text, encoding="utf-8")
                print(f"[{i}/{len(need)}] {'saved' if snap else 'not saved'}  {u}", flush=True)
                if i % 10 == 0:
                    save_index(index)
        finally:
            save_index(index)

    failed = [u for u in cited if not captured(refs.get(u)) and not refs.get(u, {}).get("data")]
    data = sum(1 for u in cited if refs.get(u, {}).get("data"))
    print(f"done: {len(cited) - len(failed) - data}/{len(cited)} captured, {data} feeds or data, {len(failed)} failed "
          f"(rerun with --retry-failed), {sum(1 for u in cited if refs.get(u, {}).get('wayback'))} with Wayback snapshots")
    return 0


if __name__ == "__main__":
    sys.exit(main())
