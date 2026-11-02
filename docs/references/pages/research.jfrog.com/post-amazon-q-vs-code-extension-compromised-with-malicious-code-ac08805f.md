---
url: "https://research.jfrog.com/post/amazon-q-vs-code-extension-compromised-with-malicious-code/"
title: "Amazon Q VS Code Extension Compromised with Malicious Code - JFrog Security Research"
captured: "2026-11-02T18:40:44Z"
via: "live"
sha256: "0f53d95e072cc08905208eb41778882b63c1bada3a802f88c1c5ab365ecee7aa"
wayback: "https://web.archive.org/web/20260520124743/https://research.jfrog.com/post/amazon-q-vs-code-extension-compromised-with-malicious-code/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Amazon Q VS Code Extension Compromised with Malicious Code - JFrog Security Research

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://research.jfrog.com/post/amazon-q-vs-code-extension-compromised-with-malicious-code/>
> Snapshot: <https://web.archive.org/web/20260520124743/https://research.jfrog.com/post/amazon-q-vs-code-extension-compromised-with-malicious-code/>

# Amazon Q VS Code Extension Compromised with Malicious Code

JFrog Security Research Team | 23 Jul, 2025

An attacker was able to compromise the Amazon Q VS Code extension in version 1.84.0 by committing code with a malicious code into its Git Hub repository, according to a [story](https://www.404media.co/hacker-plants-computer-wiping-commands-in-amazons-ai-coding-agent/) by [@404mediaco](https://x.com/404mediaco).

The compromised version has been removed and is no longer available. According to Amazon no users have been impacted.

The attacker’s code adds a function to the VS Code extension which invokes Amazon’s Q cli tool, which allows developers to interact with ML models directly from the terminal, utilizing a malicious prompt. The malicious prompt asks the AI agent in use to produce commands for wiping the machine on which it’s running through bash commands, as well as cloud resources.

This attack vector illustrates the growing sophistication level of supply chain attacks. As AI agents are more prevalent than ever, and in combination with IDE extensions, they can be leveraged to target different stages of the software development life cycle.
