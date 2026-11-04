---
url: "https://developers.redhat.com/articles/2026/03/10/agent-skills-explore-security-threats-and-controls"
title: "Agent Skills: Explore security threats and controls | Red Hat Developer"
site: "Red Hat Developer"
published: "2026-03-10T07:16:18+0000"
captured: "2026-11-04T17:35:36Z"
via: "live"
sha256: "f2549d5a298efa26959bda112ab32ad6e6557da1ce46e2469653d4ba97fe9a4e"
wayback: "https://web.archive.org/web/20260922142144/https://developers.redhat.com/articles/2026/03/10/agent-skills-explore-security-threats-and-controls"
cited_in:
  - "docs/research/agent-skill-security.md"
  - "docs/threats/02-supply-chain-packages.md"
---

# Agent Skills: Explore security threats and controls | Red Hat Developer

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://developers.redhat.com/articles/2026/03/10/agent-skills-explore-security-threats-and-controls>
> Snapshot: <https://web.archive.org/web/20260922142144/https://developers.redhat.com/articles/2026/03/10/agent-skills-explore-security-threats-and-controls>

# Agent Skills: Explore security threats and controls

March 10, 2026

[Florencio Cano Gabarda](https://developers.redhat.com/author/florencio-cano-gabarda)

Related topics:

[Artificial intelligence](https://developers.redhat.com/topics/ai-ml)[Automation](https://developers.redhat.com/topics/automation/all)[Developer tools](https://developers.redhat.com/topics/developer-tools)[Open source](https://developers.redhat.com/topics/open-source-communities)[Security](https://developers.redhat.com/topics/security)

Related products:

[Red Hat AI](https://developers.redhat.com/taxonomy/term/37288)[Red Hat OpenShift AI](https://developers.redhat.com/products/red-hat-openshift-ai/overview)

Anthropic announced the release of the Agent Skills functionality on October 16, 2025. This functionality was initially implemented in Claude software, but now it's available on many other agents, including [Goose](https://github.com/block/goose). Agent Skills is based on the concept of skills, a capability that trains an agent or client on tasks tailored to the way users work. Skills are based on folders and files, providing functionality similar to MCP but with a different approach. This article explores how to manage the security threats and access controls associated with adopting the new Agent Skills functionality.

## How Agent Skills works

The following is an example of a skill extracted from the [Agent Skills documentation](https://agentskills.io/what-are-skills). Skills are based on folders and files. Each skill has its own folder containing a SKILL.md file. The following code is the content of the SKILL.md file.

```
---
name: pdf-processing
description: Extract text and tables from PDF files, fill forms, merge documents.
---
# PDF Processing
## When to use this skill
Use this skill when the user needs to work with PDF files...
## How to extract text
1. Use pdfplumber for text extraction...
## How to fill forms
...
```

Here, we are defining a skill to extract text and tables from PDF files, fill forms, and merge documents. The body of the skill describes how to execute the procedural knowledge task, among other information.

This is the basic directory structure:

```
skill-name/
└── SKILL.md          # Required
```

Source: https://agentskills.io/specification#directory-structure

The basic structure of a SKILL.md file consists of an initial section called the frontmatter. The frontmatter is written in YAML, followed by a body written in Markdown. For the full specification, visit the [Agent Skills home page](https://agentskills.io/home).

When an agent works with Agent Skills, the agent loads the metadata within the frontmatter of all available skills. When the agent receives a request, it uses the metadata and available LLMs to decide which skill to use next. Once decided, the agent loads the body of the skill, which may be the whole description of the task or it may refer to other Markdown files in the skill folder. In this case, the agent will load them intelligently as needed. This means that the body of any skill can be split into different files to reduce the amount of information put in the context each time. It is possible to put all the information in one skill file, but it is recommended to put each task in a specific skill file, optimizing the context. In any case, the primary body is in SKILL.md and should point to the other Markdown files that we want to use in the skill.

The SKILL.md file can also refer to scripts, such as Python, Bash, and JavaScript, present in the skill folder for the skill to execute them when needed. These scripts may also have dependencies. As you can imagine, executing scripts involves some security risks.

The Agent Skills specification defines additional [optional directories](https://agentskills.io/specification#optional-directories):

- scripts/ For executable code that may be executed by the skills.
- references/ For additional documentation that skills may use.
- assets/ For static resources such as images, templates, or data files.

## Improve security of the skill files

Skills are based on folders and files. If the permissions for these folders and files are not set correctly to ensure only authorized users can modify them, malicious actors who already have direct or indirect access to the filesystem could exploit this. This risk is not that high because it’s not trivial that a malicious actor already has access to the filesystem, but we should take this risk into account specially when implementing security by design and by default and defense in depth. If the permissions are not correctly set and malicious actors have this opportunity, they could modify skill files to introduce unauthorized instructions, add malicious scripts that can be executed with the agent's permissions, often the same permissions as the user, or alter existing scripts to include malicious code.

The permissions on the skills folders and files should be restricted as much as possible by default. If the skills are stored in another system, for example, a skills registry, the permissions in the registry should also be restricted as much as possible by default. It is recommended that any access or modification to the skill files is logged. The logs generated should be protected to avoid unauthorized modification.

## Malicious skills

Skills may contain executable scripts in different languages, such as Python or Bash. This provides a lot of power to skills, but it also involves security risks. These scripts may contain malware. If the sources can’t be trusted, check the skills’ source code. The more important the tasks, the more thorough the review should be. Depending on your risk appetite, rather than doing a code review, a way to reduce the risk of Agent Skills having malware is to execute malware scans on them, for example, with tools as [malcontent](https://github.com/chainguard-dev/malcontent).

Another way of improving the security of your supply chain with relation to Agent Skills is to require them to be signed and validate their signature before use. There is no widely known initiative to sign Agent Skills, but this is something that users and customers should require if they consider it a relevant security control.

Note that although skills are initially safe, if an automatic mechanism to upgrade them exists, an upgrade can include malicious code or vulnerabilities, especially if they come from untrusted sources. In any case, depending on your risk appetite, reviewing the code of any new version of a skill that you plan to use is recommended.

## Security vulnerabilities

Skills that contain scripts may have their own security vulnerabilities. Therefore, all security controls from secure development best practices apply here, including code reviews, SAST, DAST, and fuzzing.

Providing companies must also implement vulnerability management processes to identify and resolve security issues at regular intervals, in accordance with their SLAs.

Agents could also contain security vulnerabilities Since the SKILL.md file starts with a YAML section, it is possible that the YAML parser of an agent contains a vulnerability and a malformed malicious YAML in a skills file can exploit it to execute commands in the system or leak information.

Another way to reduce the risk of a vulnerability being exploited in skill scripts is to execute them in isolated environments such as containers or sandboxed environments. Examples of technologies that can be used are seccomp, AppArmor, or Firecracker VMs. Egress communication from these isolated environments to the Internet should also be restricted.

## Prompt injection

Part of Agent Skills data flow consists of the agent obtaining information from a source, for example, a document or a webpage, and using that information to compose a prompt to be sent to an LLM to decide the next action or to compose the final output. Since part of the analyzed document is injected in the prompt sent to the LLM, there is a risk of prompt injection. This security issue occurs when input intended as data is interpreted as an instruction by the LLM instead. Agentic systems remain vulnerable to this issue because there is no industry-standard fix. While SQL injection can be mitigated through prepared statements, no similar control currently exists to reliably separate data from instructions in LLM prompts.

Although there are no definitive solutions to eliminate the risk of prompt injection, there are controls that can be applied to reduce the probability and impact of it.

Guardrails is a common security control that is gaining traction in AI systems, especially agentic AI systems that use Skills or other agentic protocols like MCP or A2A. Guardrails are systems that monitor the input and output of an agent or LLM to distinguish between benign and malicious content. If content is benign, the guardrails system lets them pass to the next system. If not, it can perform actions such as modifying the payload, blocking, logging, and throttling content. Since guardrails rely on the ability to distinguish between benign and malicious intent, a classic, and often unsolvable, problem in security, it is not a definitive solution. For that, we need patterns, such as regexes, or use other specialized LLMs. In any case, guardrails are a sound control to reduce the risk of prompt injection. For example, [TrustyAI](https://github.com/trustyai-explainability) is an open source project developed by Red Hat that includes guardrail capabilities.

Another key security control is to limit the permissions that an agent has. At a maximum, an agent should only possess the permissions of the user executing it, never more. Ideally, agents should operate with a restricted subset of those permissions, dynamically derived from the specific task or intent. Dynamic authorization for AI agents remains a compelling area for further exploration. In addition to permission limiting, agents should be executed within isolated environments, such as containers or virtual machines, to provide a robust security boundary.

One more control is using the experimental `allowed-tools` field defined by the Agent Skills specification. The specification states that, as it is experimental, it might not be supported by all agents yet. In any case, it is worth pushing for it. This is a mechanism to limit the tools that will be available to the agent, thus, reducing the risk of a malicious prompt injection or an unintended behavior. `allowed-tools` doesn't reduce the probability of prompt injection, but it reduces the impact.

Many of these security controls discussed not only reduce threats by malicious actors, but also reduce risks related to unintended behaviors of agentic systems due to the inherent probabilistic and non-deterministic nature of current LLMs.

## Credentials management

While the Agent Skills specification does not prescribe a specific method for credential management, secure handling remains a critical security component. Since agents must interact with external systems to perform actions, they require a robust authentication framework. In scenarios where manual user intervention is not feasible, it is essential to implement standardized solutions like OAuth 2.0 to manage these permissions securely.

Under no circumstances should credentials be stored in plain text or embedded directly within the skills themselves. To mitigate the risk of accidental exposure, users should be educated on secure storage practices and utilize automated secret-scanning tools, such as [Trufflehog](https://github.com/trufflesecurity/trufflehog), to detect hardcoded credentials before deployment.

## Final thoughts

Agent Skills introduces a flexible and modular way to extend the functionality of intelligent agents through skill-based orchestration of tasks. This extensibility empowers organizations to build specialized and adaptable AI ecosystems, and expands the attack surface in familiar and novel ways. As shown, risks span from modifying skill at the filesystem level and malicious or vulnerable scripts to prompt injection and credential exposure, demanding a comprehensive and proactive security posture.

Mitigating these risks requires combining traditional secure development practices, such as strict permissions, code reviews, and scanning, with AI-specific controls like guardrails, sandboxing, and controlled permissions. The introduction of constructs such as allowed-tools and signed skill registries marks an important step toward safer deployment, though these mechanisms remain in an early stage of maturity. Organizations adopting Agent Skills should therefore balance innovation with discipline, embedding continuous monitoring, validation, and threat modeling into their workflows.

Ultimately, the security of Agent Skills will depend not only on technical controls but also on the governance and culture surrounding their use. Collaboration between AI developers, security teams, and the open source community will be crucial to evolving standards that can keep pace with this rapidly advancing capability. As Agent Skills continue to mature, their secure adoption will shape the trust, reliability, and resilience of agentic systems using them.

Be sure to check out [TrustyAI on GitHub](https://github.com/trustyai-explainability), a default component of Open Data Hub and [Red Hat OpenShift AI](https://developers.redhat.com/products/red-hat-openshift-ai).
