# Interview-repo lures and the Binance theft

Last researched: 2026-11-03.

## The post

The r/ClaudeCode post ["The Binance theft started as a coding assignment. I've been sent the same kind of repo all year. All were exploits."](https://www.reddit.com/r/ClaudeCode/comments/1xmznwt/the_binance_theft_started_as_a_coding_assignment/) (2026-11-03) ties the October 29 Binance theft to the fake-interview repositories its author has received all year. The author published each one as a defanged write-up at https://github.com/Ubuntu-Buddha/security-advisories. Advisory files below are cited at commit `5512e50`.

The theft, as Reuters reported it (syndicated by NBC News: https://www.nbcnews.com/business/markets/binance-hack-fake-job-interview-heist-rcna607238):

- **June:** a wallet engineer downloaded a tampered coding tool from a search ad. Binance replaced the machine and changed credentials.
- **September:** a second engineer, recruited through a real job site, was interviewed by a fake HR person and a fake CEO, then told to download a "coding assignment" onto his work computer.
- **October 20:** that machine, plus an internal AI tool, was used to take over two more wallet engineers' systems.
- **October 29:** stolen credentials deployed a container and a script, and about $6.3 billion left the wallets. The FBI attributes it to North Korea.

The post adds that the repository ran during setup and swapped in a poisoned copy of a normal developer tool, which Binance's endpoint product did not see. The press reports do not name the tool or say how it was swapped.

## The issues, and what Bigman does about each

| # | Issue in the post | Bigman now | Still a gap |
|---|---|---|---|
| 1 | **Opening the folder is the run.** `.vscode/tasks.json` with `runOn: folderOpen`, output hidden (`reveal: never`, `echo: false`), piping `wget -qO- … \| sh` (2026-01-30 advisory). | `repo.autorun-task` already fired. It is now **High** when the task hides its output, pads its command off screen, or downloads and runs code, and it shows the `linux` override separately. | VS Code's own trust prompt is the last line; Bigman cannot stop a folder being opened. |
| 2 | **Starting the app is the run.** An error handler or controller fetches text and runs it with `new (Function.constructor)('require', …)` (W3GLFun 2026-01-27, Optu 2026-01-29). | New `repo.code-loader` (High): the Function constructor given `require`, or `eval`/`Function`/`vm` within five lines of a fetch or decode. Covers JavaScript, TypeScript and Python. | Code built from many small pieces, or a loader that waits for an unrelated trigger, is not followed. |
| 3 | **The payload address is hidden.** A base64 "API key" in `.env`, decoded with `atob(process.env.DEV_API_KEY)`, pointing at JSON storage services. | New `repo.encoded-url` (base64 or hex that decodes to a URL, in source and `.env` files) and `repo.payload-host` (jsonkeeper.com, npoint.io, jsonsilo.com, mocki.io, including inside decoded addresses). | Vercel, Render and Railway hosts are too common to flag on their own. |
| 4 | **The payload ships as a font.** A `.woff2` of hex text, decoded by `server/config/conf.js` and run (GitLab take-home, 2026-10-15). Other clusters run JavaScript named `.woff2` straight from a folder-open task. | New `repo.disguised-asset` (High): a font, image or media file that is readable text. The decoding loader also fires `repo.code-loader`. | None for this shape. |
| 5 | **"Read it in the browser before you clone."** The author's first rule. | The daemon's project watch checks a repository cloned or unpacked into `~/Projects`, `~/src`, `~/code`, Downloads and similar folders as soon as git finishes, and notifies on a High finding, so nobody has to remember. Before cloning, `bigman repo-check https://github.com/OWNER/REPO` (also GitLab, Codeberg, Bitbucket, and the URL field in **Inspect**) fetches the latest commit into a bare repository and runs every project rule on it. Nothing is checked out, no hook or filter runs, and git gets no credentials, proxies or user configuration. The report names the commit read. | Lockfiles and Python and Rust build files are read only in a cloned folder. The watch notifies rather than blocks, and covers only folders directly inside those roots. |
| 6 | **"Run it where a compromise is boring."** A work machine that can deploy anything is the wrong place. | Every new High finding says so in its next step. | Bigman does not create the disposable VM. |
| 7 | **A poisoned copy of a familiar tool.** Setup swapped a developer tool so the compromise outlived the repository. | `bigman changes` and the live monitor now watch `~/.local/bin` and `~/bin`. A new or changed file there that hides a system command (git, ssh, node) is `persistence.shadowed-command` (High). Other new files are `persistence.change`. | Tools inside a project's `node_modules/.bin`, a global npm prefix or editor extension folders are not watched. |
| 8 | **The AI tool follows the repository.** An agent that runs commands and reads the repo, README or email follows the sender's instructions. | Existing: `repo.ai-hooks`, `repo.mcp-server`, `repo.hidden-instructions`. `repo.ai-hooks` now also covers `statusLine`, `apiKeyHelper` and the other helper commands. New `repo.ai-settings` covers project settings that redirect the tool's API traffic (`env.ANTHROPIC_BASE_URL`, proxies; High, CVE-2026-21852), auto-approve MCP servers or commands (`enableAllProjectMcpServers`, `permissions.defaultMode`, `Bash(*)`), or turn off Copilot's confirmations (`chat.tools.autoApprove`; High, CVE-2025-53773). New `repo.instructions-run-download` covers instruction files that tell the assistant to pipe a download into a shell. | Instructions written in plain language cannot be judged by rules. |
| 9 | **A tampered tool from a search ad** (June stage). | Partial: Inspect and the Downloads watch read installers, archives, packages and AppImages before they run (PH-17). | Bigman has no reputation data for which publisher a tool should come from. |
| 10 | **"A first all clear is not a final answer."** A hiring platform first found no violation. | Reports list what was and was not read (coverage) and never say "safe". | — |

## How to use it

With the daemon running, a take-home cloned or unpacked into `~/Projects`, `~/src`, `~/code`, Downloads or a similar folder is checked as soon as it lands, and a High finding notifies you. Before cloning, run `bigman repo-check https://github.com/OWNER/REPO`, or paste the address into Inspect. If you already have the folder, run `bigman check ./folder`. That adds package scripts and lockfiles to the repo-check rules. Any High finding means: do not open the folder in an editor, and do not run it on this computer.

## Sources

- The post: https://www.reddit.com/r/ClaudeCode/comments/1xmznwt/the_binance_theft_started_as_a_coding_assignment/
- The author's advisories: https://github.com/Ubuntu-Buddha/security-advisories
- Reuters via NBC News: https://www.nbcnews.com/business/markets/binance-hack-fake-job-interview-heist-rcna607238
- GitLab Threat Intelligence, North Korean tradecraft (loader shape and JSON storage hosts), 2026-02-19: https://about.gitlab.com/blog/gitlab-threat-intelligence-reveals-north-korean-tradecraft/
- NVISO, JSON storage services for malware delivery, 2025-11-13: https://blog.nviso.eu/2025/11/13/contagious-interview-actors-now-utilize-json-storage-services-for-malware-delivery/
- OpenSourceMalware, fake font, 2026-01-28: https://opensourcemalware.com/blog/dprk-contagious-interview-campaign-fake-font-uses-malicious-vs-code-fonts
- Microsoft, Contagious Interview, 2026-03-11: https://www.microsoft.com/en-us/security/blog/2026/03/11/contagious-interview-malware-delivered-through-fake-developer-job-interviews/
- Check Point, Claude Code project files (CVE-2025-59536, CVE-2026-21852): https://research.checkpoint.com/2026/rce-and-api-token-exfiltration-through-claude-code-project-files-cve-2025-59536/
- Embrace The Red, Copilot `chat.tools.autoApprove` (CVE-2025-53773): https://embracethered.com/blog/posts/2025/github-copilot-remote-code-execution-via-prompt-injection
- Claude Code settings reference (which project keys run commands): https://code.claude.com/docs/en/settings-reference
