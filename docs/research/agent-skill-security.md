# Securing agent skills and plugins: what others do, and what Bigman does

Research for SC-25 (2026-11-03). The question was whether skills and plugins for AI coding tools can be vetted without a language model, and how others vet them.

## What is being secured

A skill is a folder with a `SKILL.md` (YAML frontmatter and Markdown instructions) and optional scripts, references and assets. Claude Code, OpenCode, Codex and a dozen other agents load them on demand. Plugins bundle skills with commands, subagents, hooks, MCP and language servers, monitors and programs. OpenCode plugins are different again: JavaScript or TypeScript modules OpenCode loads into its own process at start, with Bun's shell API and no sandbox (https://opencode.ai/docs/plugins/).

Two kinds of risk follow, and they need different tools:

1. **What runs without the model deciding to.** Claude Code runs `` !`command` `` lines and ```` ```! ```` blocks when a skill is used, before the model reads it; frontmatter `hooks` run for the rest of the session; `allowed-tools` approves tools in advance, even for a project skill in a folder you have not trusted (https://code.claude.com/docs/en/skills). Plugins add hooks, MCP and LSP servers, monitors that start with every session, and a `bin/` folder on the agent's PATH (https://code.claude.com/docs/en/plugins-reference). All of this is declared in files, so fixed rules can find all of it.
2. **What the instructions persuade the agent to do.** A fake prerequisite step, a "copy the key from `~/.ssh`" step, wording that hides a step from the user. Known payload shapes (download and run, password-protected archives, credential paths, collection addresses, encoded text) are fixed patterns; free-form persuasion is not.

## What others do (as of November 2026)

| Approach | Examples | Needs a model |
|---|---|---|
| Static rules, optional model pass | NVIDIA SkillSpector: regex, syntax trees, taint tracking, YARA, OSV lookups; `--no-llm` runs all of it, and only its description-versus-behaviour check needs the model (https://github.com/nvidia/skillspector) | No |
| Rules plus vendor models, as a service | Snyk ToxicSkills and Agent Scan, Invariant's mcp-scan; tool descriptions are sent to the vendor (https://snyk.io/blog/toxicskills-malicious-ai-agent-skills-clawhub/, https://invariantlabs.ai/blog/introducing-mcp-scan) | Yes |
| Known-bad lists | Koi's Clawdex check against its database of malicious ClawHub skills (https://thehackernews.com/2026/02/researchers-find-341-malicious-clawhub.html) | No |
| Rules, then a model, then a sandbox run with fake credentials | The 98,380-skill study (https://arxiv.org/abs/2602.06547) | Yes |
| Ordinary software hygiene | Red Hat: folder permissions, code review, SAST, malcontent, secret scanning, sandboxed scripts, upgrade review, signatures (https://developers.redhat.com/articles/2026/03/10/agent-skills-explore-security-threats-and-controls) | No |
| Signing | Small Ed25519 and Sigstore experiments; no standard (https://redreamality.com/blog/agent-skills-2026-survey-lifecycle-map/) | No |

What the numbers say, all self-reported:

- Snyk's rules and models caught every confirmed malicious code and suspicious download in its corpus, and 91% of the prompt injection; the payloads were plain (password-protected zips, base64 around `curl | bash`, reading `~/.aws/credentials`).
- The arXiv study puts 84.2% of vulnerabilities in the prose and 8.5% in code, and finds regex weak on free-form directives. Its most productive signal was a skill doing things its description does not mention, which needs a model to compare the two.

## What Bigman does

- Lists every skill, command, subagent and plugin Claude Code, OpenCode and Codex load from the home folder and `/etc/codex/skills`, with the tools that read each folder (OpenCode also reads `~/.claude/skills` and `~/.agents/skills`; Codex reads `~/.agents/skills`).
- Reports everything in the first category above as a finding and as a plain "runs without asking" list, and the known payload shapes of the second.
- Lets the user approve the exact files they read and reports any later change, which covers the update risk without a signing standard.
- Runs the same rules on project folders, single skill folders and archives, and OpenCode plugin packages, so a download is checked before it is installed.
- Does not judge the instructions. That is the job of the optional reviewer in the plan (Part G), which may add findings and never removes one. The coverage notes of every skills report say so.

SkillSpector with `--no-llm` would be a reasonable optional integration later, like Lynis or YARA-X; Snyk's tools would not, because they upload what they check.

## Checked against this machine

On the developer's own workstation (68 skills and plugins across the three tools), after tuning: one High (a vendor skill whose setup step pipes an install script to `sh`), four bundled programs or WebAssembly modules, two plugins starting MCP servers, three instruction passages that skip confirmation, and the shell rules on one bundled downloader script. The OpenCode plugin that prompted this work, `@gitdamnit/opencode-stranger-danger` 0.2.2, shows no install scripts and no recognised network, process or credential use in the code it ships. The inventory runs in under a second.
