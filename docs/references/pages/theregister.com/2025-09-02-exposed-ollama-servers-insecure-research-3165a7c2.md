---
url: "https://www.theregister.com/2025/09/02/exposed_ollama_servers_insecure_research/"
title: "AI can't stop the sprint to adopt hot tech without security"
site: "theregister"
published: "2025-09-02T05:15:07.000Z"
captured: "2026-11-02T19:03:36Z"
via: "live"
source_url: "https://www.theregister.com/security/2025/09/02/ai-cant-stop-the-sprint-to-adopt-hot-tech-without-security/416416"
sha256: "0bdb0c1c61d59a40f861fa17925fe6d9eb8b945a4f81932462ba1363a64959ed"
wayback: "https://web.archive.org/web/20260216122456/https://www.theregister.com/2025/09/02/exposed_ollama_servers_insecure_research/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# AI can't stop the sprint to adopt hot tech without security

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.theregister.com/2025/09/02/exposed_ollama_servers_insecure_research/>
> Snapshot: <https://web.archive.org/web/20260216122456/https://www.theregister.com/2025/09/02/exposed_ollama_servers_insecure_research/>

[Security](https://www.theregister.com/tag/security)

#  In the rush to adopt hot new tech, security is often forgotten. AI is no exception

Cisco finds hundreds of Ollama servers open to unauthorized access, creating various nasty risks

Simon Sharwood [Simon Sharwood](https://www.theregister.com/author/simon-sharwood)

Published tue 2 Sep 2025 // 06:15 UTC

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416  https://bsky.app/intent/compose?text=AI%20can't%20stop%20the%20sprint%20to%20adopt%20hot%20tech%20without%20security%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416&title=AI%20can't%20stop%20the%20sprint%20to%20adopt%20hot%20tech%20without%20security https://api.whatsapp.com/send?text=AI%20can't%20stop%20the%20sprint%20to%20adopt%20hot%20tech%20without%20security%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416&text=AI%20can't%20stop%20the%20sprint%20to%20adopt%20hot%20tech%20without%20security [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

Cisco’s Talos security research team has found over 1,100 Ollama servers exposed to the public internet, where miscreants can use them to do nasty things.

Ollama provides a framework that makes it possible to run large language models locally, on a desktop machine or server. Cisco decided to research it because, in the words of Senior Incident Response Architect Dr. Giannis Tziakouris, Ollama has “gained popularity for its ease of use and local deployment capabilities.”

Talos researchers used the Shodan scanning tool to find unsecured Ollama servers, and spotted over 1,100, around 20 percent of which are “actively hosting models susceptible to unauthorized access.” Cisco’s scan found over 1,000 exposed servers within 10 minutes of commencing its sweep of the internet.

REG AD

Leaving an Ollama server dangling on the open internet means anyone who learns of its existence could query the LLM and use its API, perhaps consuming all its resources or running up a big bill for hosted systems. Targeted attacks are possible because Cisco found many of the servers expose information that makes it possible to identify hosts.

REG AD

Cisco’s infosec investigators also worry about the following consequences:

- Model Extraction Attacks — Attackers can reconstruct model parameters by querying an exposed ML server repeatedly.
- Jailbreaking and Content Abuse — LLMs like GPT-4, LLaMA, and Mistral can be manipulated to generate restricted content, including misinformation, malware code, or harmful outputs.
- Backdoor Injection and Model Poisoning — Adversaries could exploit unsecured model endpoints to introduce malicious payloads or load untrusted models remotely.

Cisco classified 80 percent of the open Ollama servers it spotted as “dormant” because they were not running any models, meaning the above attacks would be futile. The bad news is that those servers “remain susceptible to exploitation via unauthorized model uploads or configuration manipulation.”

But Dr. Tziakouris warned “their exposed interfaces could still be leveraged in attacks involving resource exhaustion, denial of service, or lateral movement.”

The USA is home to most of the exposed servers (36.6 percent), followed by China (22.5 percent) and Germany (8.9 percent).

## MORE CONTEXT

-  [### LegalPwn: Tricking LLMs by burying badness in lawyerly fine print](https://www.theregister.com/2025/09/01/legalpwn_ai_jailbreak/)
-  [### xAI's Grok has no place in US federal government, say advocacy groups](https://www.theregister.com/2025/08/29/xais_grok_has_no_place/)
-  [### Not in my browser! Vivaldi capo doubles down on generative AI ban](https://www.theregister.com/2025/08/28/vivaldi_capo_doubles_down_on/)
-  [### Uncle Sam speedruns AI chatbot adoption for federal workers](https://www.theregister.com/2025/08/26/gsa_ai_chatbot_speedrun/)

Tziakouris concluded the findings of the Cisco study “highlight a widespread neglect of fundamental security practices such as access control, authentication, and network isolation in the deployment of AI systems.” As is often the case when organizations rush to adopt the new hotness, frequently without informing IT departments because they don’t want to be told to slow down and do security right.

He thinks things may get worse.

“The uniform adoption of OpenAI-compatible APIs further exacerbates the issue, enabling attackers to scale exploit attempts across platforms with minimal adaptation,” he wrote, before calling for development of “standardized security baselines, automated auditing tools, and improved deployment guidance for LLM infrastructure.”

REG AD

He also acknowledged that Cisco’s Shodan scan cannot deliver a definitive view on LLM security, and called for work on tools that include adaptive fingerprinting and active probing techniques, and target other model-hosting frameworks including Hugging Face, Triton, and vLLM, to help researchers better understand the situation. ®

[cisco](https://www.theregister.com/tag/cisco) [talos](https://www.theregister.com/tag/talos) [ai](https://www.theregister.com/tag/ai) [security](https://www.theregister.com/tag/security)

https://twitter.com/intent/tweet?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416 https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416  https://bsky.app/intent/compose?text=AI%20can't%20stop%20the%20sprint%20to%20adopt%20hot%20tech%20without%20security%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416 https://www.reddit.com/submit?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416&title=AI%20can't%20stop%20the%20sprint%20to%20adopt%20hot%20tech%20without%20security https://api.whatsapp.com/send?text=AI%20can't%20stop%20the%20sprint%20to%20adopt%20hot%20tech%20without%20security%0Ahttps%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416 https://t.me/share/url?url=https%3A%2F%2Fwww.theregister.com%2Fsecurity%2F2025%2F09%2F02%2Fai-cant-stop-the-sprint-to-adopt-hot-tech-without-security%2F416416&text=AI%20can't%20stop%20the%20sprint%20to%20adopt%20hot%20tech%20without%20security [Make us preferred on Google](https://google.com/preferences/source?q=theregister.com)

[## Marvell wants to be a copper topper, bets $115B on photonics leader to push optical networking

IP house believes optics demand is going to explode as Nvidia and others eye bigger systems](https://www.theregister.com/systems/2026/11/02/marvell-wants-to-be-a-copper-topper-bets-115b-on-photonics-leader-to-push-optical-networking/5307028)

[## Oracle gobbles $20 billion borrowing limit a week after setting it

In the middle of a datacenter capex binge, vendor promises no more borrowing until the end of 2030](https://www.theregister.com/paas-and-iaas/2026/11/02/oracle-gobbles-20-billion-borrowing-limit-a-week-after-setting-it/5307014)

[## From expertise to intelligence: NISCO deploys world's first swarm-intelligence steel model at scale

PARTNER CONTENT: 300+ AI applications delivering surges in core metrics and annual cost reduction of over CNY100m](https://www.theregister.com/ai-and-ml/2026/11/02/partner-content-from-expertise-to-intelligence-nisco-deploys-worlds-first-swarm-intelligence-steel-model-at-scale/5306614)

[## Ericsson says it can detect, track drones using only mobile networks

ISAC technology adds detection ability to existing cell networks, enabling them to track low-flying small drones](https://www.theregister.com/networks/2026/11/02/ericsson-says-it-can-detect-track-drones-using-only-mobile-networks/5306958)

[OPINION

## Microsoft's moolah mountain is more massive than ever. It just doesn't want the people who built it

Flash the cash, flush the cache](https://www.theregister.com/columnists/2026/11/02/microsofts-moolah-mountain-is-more-massive-than-ever-it-just-doesnt-want-the-people-who-built-it/5306576)

[## Thieves pilfer environment variables from Trigger AI workflow tool

So, who's ready to clear the dust out of that server closet and move things back on-prem?](https://www.theregister.com/security/2026/11/02/thieves-pilfer-environment-variables-from-trigger-ai-workflow-tool/5306998)
