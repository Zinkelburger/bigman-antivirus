---
url: "https://blog.leakix.net/2026/02/ollama-exposed/"
title: "12,000 Ollama Instances Exposed: When 'Local-First' Meets the Real World"
published: "2026-02-23T09:00:00Z"
captured: "2026-11-02T18:22:35Z"
via: "live"
sha256: "11c5645347d2d45f06e2082dd8fae326350dc8e3afa3afd74340dbd3c0b0884e"
wayback: "https://web.archive.org/web/20261011180324/https://blog.leakix.net/2026/02/ollama-exposed/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# 12,000 Ollama Instances Exposed: When 'Local-First' Meets the Real World

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://blog.leakix.net/2026/02/ollama-exposed/>
> Snapshot: <https://web.archive.org/web/20261011180324/https://blog.leakix.net/2026/02/ollama-exposed/>

# 12,000 Ollama Instances Exposed: When 'Local-First' Meets the Real World

February 23, 2026  Valentin Lobstein  7 min read

[research](https://blog.leakix.net/categories/research)[LeakIX](https://blog.leakix.net/categories/leakix)[#ollama](https://blog.leakix.net/tags/ollama)[#AI](https://blog.leakix.net/tags/ai)[#research](https://blog.leakix.net/tags/research)[#LeakIX](https://blog.leakix.net/tags/leakix)[#security](https://blog.leakix.net/tags/security)

Ollama is one of the most popular tools for running large language models locally. 100,000+ stars on GitHub, millions of downloads, the default choice for anyone who wants to self-host an LLM. There’s just one problem: it ships with no authentication, and the maintainers have made it clear they don’t plan to add any.

We found 12,269 Ollama instances exposed on the public internet with zero authentication. Anyone can list models, run inference, and in some cases exfiltrate proprietary fine-tuned weights.

## The Numbers

Our [OllamaPlugin](https://leakix.net/search?scope=leak&q=%2Bplugin%3AOllamaPlugin) continuously scans the internet for exposed Ollama instances. Here’s what we’re seeing right now:

| Country | Exposed Instances

| United States | 1,829

| China | 1,712

| France | 1,425

| Germany | 1,364

| South Korea | 453

| India | 414

| Russia | 393

| Hong Kong | 329

| Finland | 312

| Canada | 256

The top networks hosting exposed instances are AWS (1,686 combined), Hetzner (1,004), OVH (773), and Contabo (634). These aren’t hobbyist setups on home networks. These are cloud deployments, often running expensive GPU instances with production models loaded.

Among the exposed instances, we find everything from llama3.3:latest running on 42 GB of VRAM to proprietary fine-tuned models that represent months of training work. Some instances have 30+ models loaded, representing hundreds of gigabytes of weights sitting on the open internet.

## What’s Exposed

Every exposed Ollama instance gives an unauthenticated attacker access to:

- Model enumeration: /api/tags lists every model with exact sizes
- Free compute: /api/generate and /api/chat let anyone run inference at the owner’s expense
- Model theft: /api/pull and /api/push can exfiltrate model weights, including proprietary fine-tuned models
- Proven RCE chain: [CVE-2024-37032](https://nvd.nist.gov/vuln/detail/CVE-2024-37032) (“Probllama”) is a path traversal in Ollama’s model pull mechanism that chains into unauthenticated RCE as root. The OCI manifest digest field accepts arbitrary path traversal sequences instead of enforcing sha256:<hex>, allowing a rogue registry to write arbitrary files on the server. The exploit writes a shared library payload via ld.so.preload and triggers it through a model chat request. The [Metasploit module](https://github.com/rapid7/metasploit-framework/pull/21006) delivers a full Meterpreter session as root in seconds. Among the 12,269 exposed instances, roughly 1,000 are running vulnerable versions. And since most people deploy Ollama through Docker with default settings, the process runs as root. Exploitation gives immediate root-level access to the container, and often to the host through mounted volumes or privileged mode.

For an attacker, these instances are free GPU time. For a competitor, free access to proprietary models. For a red teamer, initial access into cloud environments where the Ollama instance often runs as root with broad network access to internal services.

And here’s the part that should worry everyone: the absence of authentication makes every future Ollama CVE automatically unauthenticated. Every single vulnerability that will be found in Ollama, whether it’s a path traversal, a buffer overflow, or any memory corruption bug in the C++ llama.cpp engine, will be exploitable by anyone on the internet with zero credentials. No auth means no barrier between a remote attacker and whatever bug comes next. And bugs will come. Ollama is a large C++ project that parses complex binary formats (GGUF models, OCI manifests, safetensors). The attack surface is massive. The moment you get any form of file write on a system running as root, you write a shared library, you write to ld.so.preload, and the next process that spawns loads your code. Ollama’s llama.cpp runner forks processes constantly, so the trigger is immediate. Full system compromise, every time, without exception.

Authentication would not prevent these bugs from existing. But it would prevent random people on the internet from reaching them. Right now, every exposed Ollama instance is one CVE away from a root shell. And there are 12,269 of them.

## Ollama’s Position: “Use a Proxy”

The community has been asking for built-in authentication since October 2023. Here’s the timeline:

- Oct 2023: [Issue #849](https://github.com/ollama/ollama/issues/849). “How to secure the API with api key.” Response: use nginx.
- Nov 2023: [Issue #1053](https://github.com/ollama/ollama/issues/1053). “Requesting support for basic auth or API key authentication.” 25+ comments. Closed.
- Sep 2024: [PR #6223](https://github.com/ollama/ollama/pull/6223). A contributor implemented basic auth using gin’s built-in middleware. Working code, ready to merge. Rejected by jmorganca (Ollama founder): “we suggest doing this with a proxy in front of Ollama for the time being.”
- Jan 2025: [Issue #8536](https://github.com/ollama/ollama/issues/8536). “Support for API_KEY based authentication.” Closed again. Collaborator response: “ollama is an LLM inference engine. Other functionality is added by external projects.” (6 thumbs down.)
- Feb 2025: Multiple additional PRs (#5415, #8321, #9131) adding authentication. All stalled or rejected.

The official position from jmorganca (December 2024):

“Given the different auth configurations, we try to keep Ollama focused on serving an http API that can be fronted by a number of proxy servers such as nginx, caddy and more.”

The problem isn’t that proxy-based auth doesn’t work. It does. The problem is that nobody does it. 12,269 instances prove that telling users to “just add nginx” is not a security strategy.

## Secure by Design Is Not Optional

Let’s be real. You have to assume users will mess up deployments. Not everyone is a sysadmin. Not everyone understands networking. Most people running Ollama are developers, researchers, or hobbyists who just want to run an LLM. They follow the quickstart, set OLLAMA_HOST=0.0.0.0 because they need access from another machine, and move on with their lives. They are not going to set up nginx with bearer token auth. They are not going to write a Caddyfile. They are not going to configure TLS certificates. They want it to work, and that’s it.

And that is completely normal. That is how normal people use software. Blaming users for not adding a reverse proxy in front of every service they deploy is absurd. It is the maintainers’ job to ship secure defaults. Period.

It’s 2026. We’ve been through this exact disaster with Redis, MongoDB, Elasticsearch, Memcached, and a dozen other projects. The playbook is written. The lessons are documented. Secure defaults are not a luxury, not a nice to have, not a “community integration.” They are a baseline expectation for any software that listens on a network socket.

Ollama does none of this. No authentication. No protected mode. No warning when binding to 0.0.0.0. Nothing. And the community has asked for it dozens of times. Working code has been submitted. Multiple PRs, multiple approaches. All rejected. The maintainers are not unaware of the problem. They have made a conscious, deliberate decision to leave their users exposed.

That is not “staying focused on core functionality.” That is negligence.

## We’ve Seen This Before

This exact pattern has played out multiple times:

Redis shipped binding to all interfaces with no authentication. Thousands of instances were compromised. Redis eventually added protected-mode in 3.2 (2016) that refuses external connections unless explicitly configured.

MongoDB defaulted to no authentication and binding to 0.0.0.0. In January 2017, over 27,000 databases were [wiped and held for ransom](https://www.bleepingcomputer.com/news/security/mongodb-apocalypse-is-here-as-ransom-attacks-hit-10-000-servers/). MongoDB eventually changed the defaults.

Elasticsearch went through the same cycle. Thousands of exposed clusters, data breaches, and ransom attacks before Elastic added security as a default feature in 8.0.

Every single one of these projects started with the same argument: “it’s designed for trusted networks.” Every single one eventually added authentication after enough damage forced their hand.

Ollama is at the denial stage of this cycle. With 12,269 instances already exposed, the question isn’t whether incidents will happen. It’s when they’ll be large enough to force a response.

## What You Should Do

If you’re running Ollama and it needs to be accessible over the network:

- Check if you’re exposed right now: search for your IP or domain on [LeakIX](https://leakix.net)
- Bind to localhost: set OLLAMA_HOST=127.0.0.1 (not 0.0.0.0)
- Put a reverse proxy in front: nginx or Caddy with bearer token auth. Example configs are available in the [Ollama issue thread](https://github.com/ollama/ollama/issues/1053)
- Firewall rules: restrict port 11434 to trusted IPs only
- Monitor access: Ollama doesn’t log requests by default, so your proxy should handle access logging

If you’re the Ollama team reading this: a single OLLAMA_API_KEY environment variable and a 20-line middleware check is all it takes. The community already wrote the code. Multiple times. You keep rejecting it.

12,269 exposed instances say the “use a proxy” approach isn’t working. At some point you have to stop blaming users and start taking responsibility for the software you ship.

All results are searchable on [LeakIX](https://leakix.net). Asset owners can request removal through our [take-down process](https://leakix.net/legal).

If you’re running an exposed Ollama instance, you can check your exposure and configure alerts at [leakix.net](https://leakix.net).

##### Share this post

[Twitter](https://twitter.com/intent/tweet?url=https%3a%2f%2fblog.leakix.net%2f2026%2f02%2follama-exposed%2f&text=12%2c000%20Ollama%20Instances%20Exposed%3a%20When%20%27Local-First%27%20Meets%20the%20Real%20World)[LinkedIn](https://www.linkedin.com/sharing/share-offsite/?url=https%3a%2f%2fblog.leakix.net%2f2026%2f02%2follama-exposed%2f)[Reddit](https://www.reddit.com/submit?url=https%3a%2f%2fblog.leakix.net%2f2026%2f02%2follama-exposed%2f&title=12%2c000%20Ollama%20Instances%20Exposed%3a%20When%20%27Local-First%27%20Meets%20the%20Real%20World)

V

##### Valentin Lobstein

Security researcher and technical writer.

[View Profile](https://twitter.com/Chocapikk_)

##### Related Posts

https://blog.leakix.net/2026/02/seleniumgreed-rce/

###### [SeleniumGreed: The RCE That Was Always There](https://blog.leakix.net/2026/02/seleniumgreed-rce/)

Feb 21, 2026

## TL;DR

Every Selenium Grid instance deployed with Firefox nodes is vulnerable to unauthenticated …

https://blog.leakix.net/2026/02/appscan-coverage-2026/

###### [What We Found Scanning the Internet in 2026](https://blog.leakix.net/2026/02/appscan-coverage-2026/)

Feb 9, 2026

One of our scans recently picked up an open vector database. Inside: over a hundred pages of …

https://blog.leakix.net/2024/01/vinchin-backup-rce-chain/

###### [Vinchin Backup & Recovery: CVE-2024-22899 to CVE-2024-22903](https://blog.leakix.net/2024/01/vinchin-backup-rce-chain/)

Jan 25, 2024

## Table of Contents:

- [Default SSH Root Credentials (CVE-2024-22902)](https://blog.leakix.net/2026/02/ollama-exposed/#default-ssh-root-credentials-cve-2024-22902)
- [Hardcoded Database Credentials and …](https://blog.leakix.net/2026/02/ollama-exposed/#hardcoded-database-credentials-and-configuration-flaw-cve-2024-22901)
