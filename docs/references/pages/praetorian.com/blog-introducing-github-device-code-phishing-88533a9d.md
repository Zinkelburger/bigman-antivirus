---
url: "https://www.praetorian.com/blog/introducing-github-device-code-phishing/"
title: "Introducing: GitHub Device Code Phishing"
site: "Praetorian"
published: "2025-06-12T13:23:57+00:00"
captured: "2026-11-02T18:58:14Z"
via: "live"
sha256: "f51f7ec4d84a968d6202ea30bee43e5d3db8c0a5497c4a9418f18e2376361d89"
wayback: "https://web.archive.org/web/20260905080758/https://www.praetorian.com/blog/introducing-github-device-code-phishing/"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Introducing: GitHub Device Code Phishing

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.praetorian.com/blog/introducing-github-device-code-phishing/>
> Snapshot: <https://web.archive.org/web/20260905080758/https://www.praetorian.com/blog/introducing-github-device-code-phishing/>

[Skip to content](https://www.praetorian.com/blog/introducing-github-device-code-phishing/#content)

Meet Constantine – Find Mythos-level vulnerabilities in your code. It proves them, patches them, PRs them back. Autonomously.

[Download Datasheet](https://www.praetorian.com/resources/constantine-datasheet/)

https://www.praetorian.com

-    [CI/CD Security](https://www.praetorian.com/category/ci-cd-security/)

# Introducing: GitHub Device Code Phishing

-    [John Stawinski](https://www.praetorian.com/author/john-stawinski/), [Mason Davis](https://www.praetorian.com/author/mason-davis/), [Matt Jackoski](https://www.praetorian.com/author/matt-jackoski/)
-  [June 12, 2025](https://www.praetorian.com/blog/2025/06/12/)

What if all it took to compromise a GitHub organization–and thus, the organization’s supply chain–was an eight-digit code and a phone call?

Introducing: GitHub Device Code Phishing.

While security teams have been battling Azure Active Directory device code phishing attacks for years, threat actors have overlooked GitHub’s OAuth2 device flow as an attack vector.

At Praetorian, [our Red Team](https://www.praetorian.com/services/red-team/) works to identify creative initial access vectors that could have immediate, widespread impact. Given the recent increase in GitHub-related attacks, we feel obligated to share these techniques with the community so Blue Teams can be prepared.

Today, we’ll break down GitHub Device Code phishing, show real-life case studies of how we’ve used the technique to compromise some of the most mature modern organizations, and teach you how to protect your organization from risk.

## Background: Device Code Phishing in the Wild

Before we discuss GitHub-specific attacks, let’s define device code phishing and explain why it’s become such a popular technique among threat actors.

OAuth 2.0 Device Authorization Grant (RFC 8628) was designed to solve a legitimate problem: how do you authenticate on devices that don’t have web browsers or have limited input capabilities? Think smart TVs, IoT devices, or command-line tools.

The solution is simple:

- You try to log in to an application from a new device (like logging into Netflix from a TV).
- The application (Netflix) requests a device code from the authorization server.
- The authorization server returns a code called a device code, along with a URL.
- The application displays the device code and verification URL to the user (Netflix telling you, “Navigate to https://www.netflix.com/tv8 and enter this code”)
- You navigate to the verification URL on a separate device (like your laptop or smartphone) and enter the device code.
- You authenticate and authorize the device.
- The device (your TV) polls the authorization server and receives an access token.

The weakness in Device Code Authorization is that there is no guarantee that the user who generated the code is the one who completes the authentication.

Following the Netflix example, I can use my TV to generate a code and send it to my friend. If I convince my friend to navigate to https://www.netflix.com/tv8 and log in with my code, it will authorize my TV to log in to their account.

### Background: Azure Active Directory (AAD) Device Code Phishing

Another service that supports device code authentication is Azure Active Directory.

Attackers have been abusing this flow in AAD environments for years. AAD Device Code phishing has become so prevalent that it has earned countless security blog posts and appeared in major conference talks like RSA, BlackHat, and DEF CON. Device Code phishing attacks persist because they only use existing, legitimate functionality, and they grant long-term, persistent access.

## GitHub: Now It’s Your Turn

GitHub device code phishing represents a natural evolution of attack techniques that attackers have used against Microsoft environments for years.

Incidents like the recent[tj-actions supply chain attack](https://unit42.paloaltonetworks.com/github-actions-supply-chain-attack/) have proved that GitHub access can be the fastest way to launch a supply chain attack against hundreds of thousands of users. As organizations increasingly centralize their development infrastructure around GitHub, these attacks will become more attractive to threat actors seeking access to source code, CI/CD secrets, and intellectual property.

## Show Me The Process

Compromising a target’s GitHub account takes five steps:

- Code Generation
- Social Engineering
- User Authentication
- Token Retrieval
- Own Everything

We’ll break down this entire flow and then provide a tool that security teams can use to automate the process.

A diagram of how an attacker could abuse the GitHub Device Code Authentication flow.

### 1. Code Generation

The attacker uses GitHub’s OAuth API to generate a device code, requesting specific [OAuth scopes](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps). These scopes define what the token will be able to do. We typically include at least user, repo, and workflow.

```
curl -X POST https://github.com/login/device/code \

  -H "Accept: application/json" \

  -d "client_id=01ab8ac9400c4e429b23&scope=user+repo+workflow"
```

In this example, we used the Visual Studio Code Client ID (01ab8ac9400c4e429b23). Because this app is maintained by GitHub, it will display fewer warnings to the target user. You can tailor the client ID based on the social engineering ruse.

The server returns a response containing these components:

- device_code: We will use this code to retrieve the authentication token at the end
- user_code: The eight-digit code the target needs to enter
- verification_uri: https://github.com/login/device (for GitHub cloud)
- expires_in: Time window for the attack (typically 15 minutes)

### 2. Social Engineering

The attacker delivers the code and verification URL to their target. We’ll leave the specific ruses up to you. Given that it uses native functionality, it’s very easy to build trust. We’ve successfully convinced targets, typically developers, to complete the device flow >90% of the time when calling on the phone (this data is across several large organizations).

Remember, defending against phishing attacks is about technical controls and detection and response. An attacker will always be able to find an overly trusting employee.

### 3. User Authentication

The target navigates to https://github.com/login/device and enters the provided code.

The initial device code page.

If the organization you are trying to compromise uses SSO, the target must select “Authorize” for the target org and then complete SSO authentication (like Okta, for example). If the target has made it this far, this part is unlikely to raise suspicion.

Authorizing organizations for SSO.

Next, GitHub will instruct the target to authorize the application.

After app authorization, GitHub redirects the user to a confirmation page.

GitHub’s confirmation page.

### 4. Retrieve the Token

The attacker’s “device code” is now authorized to retrieve an OAuth token on behalf of the user. Using the device code returned during code generation, the attacker sends this post request to retrieve the token:

```
curl -X POST https://github.com/login/oauth/access_token \

  -H "Accept: application/json" \

  -d "client_id=01ab8ac9400c4e429b23&device_code=be9<code_from_earlier>&&grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Adevice_code" -k | jq
```

### 5. Own Everything

Now that the attacker has the token, it’s open season.

If you’ve ever executed a red team in a modern, cloud-native environment, you know that once you get into DevOps, it’s game over. This TTP has led to some of our most rapid paths to reach the engagement objective.

What makes this particularly dangerous is the scope of access. A single GitHub OAuth token can allow an attacker to:

- Exfiltrate intellectual property from private source code repositories
- Compromise GitHub Actions secrets and use them to launch further attacks
- Execute code on self-hosted GitHub runners to gain [persistent internal access to the environment](https://www.praetorian.com/blog/self-hosted-github-runners-are-backdoors/)
- Backdoor key repositories to launch supply chain attacks against downstream users

After discovering GitHub device code phishing about a year ago, we’ve leveraged it to compromise Fortune 500 organizations in multiple Red Team engagements. Typically, all this requires is calling developers, convincing them to complete device code authentication, and then using a tool like [Gato](https://github.com/praetorian-inc/gato) to conduct privilege escalation and lateral movement.

Occasionally, engagements require more creative applications of GitHub Device Code Phishing. Here are two examples of unique applications from recent Red Teams.

## Case Study 1: Proxied GitHub Enterprise Device Code Phishing

TL;DR: On a recent engagement, we used compromised endpoints to proxy the GitHub Device Code authentication flow when targeting GitHub Enterprise instances that were only accessible internally.

Many organizations protect their most sensitive repositories behind GitHub Enterprise (GHE) instances that aren’t directly accessible from the internet.

To generate the device code, retrieve the token, and use the token, you need network access to the target GitHub Enterprise server. In these cases, GitHub Device Code phishing becomes a lateral movement and privilege escalation opportunity rather than an initial access vector.

Note to Red Teamers: If they detect your GitHub Enterprise activity, they can trace the IP back to the compromised machine. As this TTP becomes more popular, internal GitHub Device Code Phishing may become less opsec-friendly. We recommend using a disjoint foothold (one that you can afford to lose and that can’t be tied to your other footholds or activity).

### Initial Access Vector: Helpdesk, Can You Help Me

Our engagement progress began when we uncovered an internal Employee helpdesk phone number. If you’ve been following our blog posts, you know [we love to target helpdesk](https://www.praetorian.com/blog/helpdesk-telephone-attack/) – they usually have high privileges in an environment and are motivated to solve your problems.

We called the helpdesk, impersonating an employee struggling to open a PDF. After “troubleshooting” for thirty minutes on a Zoom call, we asked the operator if they could try opening the file on their machine to determine if it was a file problem or a system problem.

Once they opened the file (which was actually a trojanized LNK), our payload detonated, connecting to our C2 infrastructure, and we had code execution on the machine.

The objective of this engagement was to compromise the organization’s AI/ML intellectual property. The help desk operator had high privileges, but the available opportunities to move towards their development environment had a risk of detection above our tolerance at that point in the engagement.

So, we turned back to phishing. We could reach their internal GitHub Enterprise instance from the compromised endpoint. Proxying our traffic through the endpoint and using their internal directories to get developers’ phone numbers, we made another call.

“Hey, this is __ from helpdesk (we knew a lot about the organization’s helpdesk at this point, making them easy to impersonate). I’m looking at a ServiceNow ticket that says your device will lose GitHub Enterprise access two days from now. Do you have two minutes to update your device registration over the phone?”

After successfully walking the AI/ML developer through the device code authentication flow, we grabbed an OAuth token. This token had write or admin access to hundreds of AI/ML-related internal repositories, including their flagship AI/ML products. Within days, we had pivoted into their Azure environment, exfiltrated their critical data, and marked the objective as “complete.”

## Case Study 2: Dynamic Device Code Phishing

TL;DR: We built [GitPhish](https://github.com/praetorian-inc/GitPhish), an automated tool that generates device codes on-demand and delivers them via professional GitHub Pages sites. By creating convincing GitHub accounts and automating the entire flow, we scaled GitHub device code phishing to target a large number of developers simultaneously and with a convincing landing page.

### The Challenge: Time-Sensitive GitHub Phishing at Scale

GitHub Device code phishing worked effectively during phone interactions but posed a significant challenge when scaled via email and SMS campaigns due to device codes expiring after just 15 minutes.

This short-lived nature introduced two main complications:

- Pre-generating codes with staggered timing was operationally complex.
- Precise timing was essential, as targeted employees needed to act immediately to comply.

Imagine sending 20 phishing emails simultaneously at 9 AM EST containing a device code. By the time a developer on the West Coast checks their email at 2 PM PST, they encounter an expired code, rendering your phishing attempt ineffective and raising suspicion, potentially alerting security operations. Not ideal.

### Our Solution: GitPhish (Dynamic Code Generation)

To overcome these issues, we developed GitPhish—an automated platform designed explicitly for GitHub device code phishing. GitPhish leverages GitHub’s infrastructure to establish immediate legitimacy and includes two primary components:

- GitHub Pages Landing Page
- Backend GitHub Authentication Broker

#### GitHub Pages Landing Page

GitHub Pages offers trustworthy, free static hosting for public repositories, resulting in legitimate-looking URLs constructed using the format: username.github.io/repository-name. For example, an account named “devicesync” with a repository named “security-verification” results in the URL:

devicesync.github.io/security-verification

GitPhish automates deployment by creating repositories, committing professional phishing templates, and enabling GitHub Pages hosting via an attacker-controlled GitHub account.

The phishing page deployed by GitPhish contains embedded JavaScript that interfaces with the backend authentication broker. When the victim accesses this page, the client-side script instantly requests a fresh, valid device code from the backend, ensuring every visitor receives a usable code.

The pages use professional, corporate-style messaging, prompting visitors to “verify device access” or “complete security authentication,” reinforcing credibility and reducing suspicion.

#### Backend GitHub Authentication Broker

The backend component addresses the device code expiration issue by generating codes dynamically in real-time. This authentication broker is an HTTPS server built with Flask, managing the entire OAuth flow behind the scenes.

When a user accesses the phishing page, the JavaScript triggers an asynchronous request to the backend’s /ingest endpoint. Immediately, the broker generates a fresh device code through GitHub’s OAuth API:

curl -X POST https://github.com/login/device/code \

-H “Accept: application/json” \

-d “client_id=01ab8ac9400c4e429b23&scope=user+repo+workflow”

This method ensures each visitor receives a fully valid, 15-minute lifespan device code, effectively solving the timing issue in traditional device code phishing scenarios.

### Dynamic Device Code Phishing in Action

Our initial GitPhish deployment occurred during an end-to-end Red Team engagement targeting a Fortune 50 company with a significant GitHub footprint. We needed an efficient solution capable of targeting a broad developer audience rapidly and effectively.

Using GitPhish, we successfully executed a phishing campaign with high scalability and obtained several valid GitHub access tokens. We were able to use those tokens to move laterally within the organization’s CI/CD environment, eventually compromising their root AWS account (a story for another time).

## Can I Stop Device Code Phishing?

Currently, we are not aware of a way to directly disallow or disable the OAuth 2 Device Flow on GitHub organizations or GitHub Enterprise. If anyone knows a way to do this, please let us know.

## Well, Then, How Do I Detect It?

If you’ve ever skimmed a GitHub audit log, you know it’s hardly beach‑reading, but buried in that scroll of JSON is the first clue that someone may have just tricked an engineer into approving a rogue device code prompt. The org_credential_authorization.grant event indicates that a user has authorized a token (personal access token, third-party application, or an OAuth flow from a device code) to access your GitHub organization. This by itself is not inherently malicious. So, how do you discern what is legitimate vs malicious?

### Layered Detection

#### Initial Authorization Log Event:

Authorization Action: The log indicates an authorization event but doesn’t explain the motivation behind it. Developers regularly use automation tools and GitHub Personal Access Tokens (PATs) for day-to-day tasks and CI/CD pipeline operations. These logs also capture third-party application authorizations acting on behalf of users.

IP Address of Victim: The actor_ip field captures the IP address of the user approving the authorization. However, this may reflect the user’s actual location—such as a coffee shop Wi-Fi—and not the attacker’s infrastructure, which often remains concealed.

Token Scopes: The token_scopes field describes the permissions assigned to the issued token. This detail can be crucial for threat hunting. Attackers commonly automate device code phishing campaigns, often assigning identical scopes across multiple tokens. Particularly risky scopes include repo, workflow, and any admin:* scopes.

None of these indicators alone conclusively signal a device code phishing attack, so additional contextual analysis is essential.

#### Layer 1 – Initial Contextual Checks

#### Layer 2 – Network Monitoring

Monitor network logs within the VPN or other edge networks to see if employees are visiting https://github.com/login/device. Even a single hit is interesting if you don’t usually see that path. Multiple hits within minutes could indicate a larger-scale phishing attack.

Caveat: This is a legitimate GitHub operation, so it could just be part of developers’ normal processes within your organization.

#### Layer 3 – Post‑Grant Behavior

Detecting a single device code phish is not easy and likely to get lost in the noise, especially if it is a commonly used feature within your organization. Therefore, it’s essential to look for post-exploitation activity. An opportunistic attacker will try to expand access quickly, including scanning organization repositories for misconfigurations. Look out for:

- Excessive Repository Clones: repo.clone or repo.download_zip on dozens of projects rapidly.
- Secret Scanning: Sudden influx of API calls to the actions/secrets endpoint.
- Anomalous Workflows: GitHub Actions present a significant attack surface. Attackers may exploit this to inject themselves into CI/CD processes.

For more details on GitHub post-exploitation, check out [Gato by Praetorian](https://github.com/praetorian-inc/gato).

### Revoking Access

If a successful device code phish is suspected, have a playbook ready to:

- Revoke SSO access for compromised accounts immediately. This helps limit further attacker activity.
- Review audit logs thoroughly to determine the attacker’s initial entry point, scope of actions, resources accessed, and any unusual activities.
- Evaluate all repositories that the attacker accessed for changes, commits, forks, or unexpected activity. Ensure integrity by reviewing recent commits and pull requests.
- Investigate potentially exposed secrets within CI/CD processes. This includes checking environment variables, secrets stored in GitHub Actions workflows, and any recently changed secrets.
- Assess CI runners potentially compromised by attacker actions. Check runner history, executed jobs, and any anomalous resource utilization to detect malicious activity or code execution.

### Potential Hardening Ideas

There are a few defensive measures that can be taken to reduce the likelihood and impact of GitHub device code phishing attacks against an organization. These defensive measures should take into account your organization’s specific needs and common developer patterns.

- IP Allow-listing: GitHub offers the ability to allow-list access to GitHub organizations. An example of this would be allowing only IP addresses from your orgs ASN to access your GitHub organization (including repositories, etc.).

- Important: Using IP allow-listing will break GitHub actions if your organization uses any GitHub-hosted runners. Therefore, pursue this configuration with caution.

- Restrict Access to GitHub Device Login Endpoint: If IP allow-listing is impractical or excessively restrictive, consider blocking or restricting access to https://github.com/login/device at the network edge or proxy level. If your developers don’t commonly use device code authentication, this restriction can significantly deter potential phishing attempts without disrupting standard workflows.

## Be Prepared

It is only a matter of time before threat actors start using GitHub Device Code Phishing in the wild (if they haven’t already). We believe this because [CI/CD security assessments](https://www.praetorian.com/services/ci-cd-security-engagement/) are one of our busiest service lines, and we track this threat landscape closely.

By reading this document, you’re already a step ahead of the field. Implement high-fidelity alerts for device code authentication and apply the principle of least privilege when granting DevOps access to employees to limit the blast radius of a compromised token.

Now is the time to prepare for GitHub device code phishing before it becomes as widespread and damaging as its Microsoft-targeting predecessors. In the world of supply chain attacks, the question isn’t whether your organization will be targeted – it’s whether you’ll be ready when the call comes.

### Take Action Now

Join our upcoming webinar where we’ll demonstrate these techniques live and dive deeper into the detection strategies:[GitHub Device Code Phishing: From Research to Real-World Exploitation](https://www.praetorian.com/resources/github-device-code-phishing-from-research-to-real-world-exploitation/)

For hands-on training, we’re teaching advanced CI/CD attack chains (including GitHub Device Code techniques) at Black Hat USA 2025. Our “Pipeline to Pwn: Mastering Modern CI/CD Attack Chains” training will give you practical experience with these attacks and defenses:[register here via BlackHat.](https://www.blackhat.com/us-25/training/schedule/#pipeline-to-pwn-mastering-modern-cicd-attack-chains-43711)

The threat actors are already dialing in GitHub infrastructure attacks. Make sure your detection capabilities are ready for the call.

## Bonus: Device Code Phishing With Custom OAuth Apps

In the examples we’ve shown so far, we’ve used the client IDs of existing applications, like Visual Studio Code, when initiating the device code flow. One interesting sidebar is that anyone can create a GitHub OAuth app and generate their own client IDs.

Let’s say you are targeting Contoso Corp. You can make your own GitHub App named “ContosoApp,” upload the Contoso logo, and use that client ID to initiate the device code flow.

Once you register the application, you’ll have access to the app’s Client ID. Use this Client ID to initiate the device code flow.

You can also customize the app by uploading the target organization’s logo.

Now, when the target goes to authenticate with the device code, they’ll see the disguised app.

Looking closely, there are a few downsides to this approach.

- The “created by” user will be your personal GitHub account (which can be fixed by using burner accounts).
- The bottom panel will show “Not owned or operated by GitHub,” the creation date, and the user count.

Additionally, GitHub will send a notification email for apps they consider to be “third-party,” aka, not owned and operated by GitHub.

Using your own OAuth App can help customize the authorization process according to your phishing ruse, but it introduces several warnings and red flags visible to the target user. For those reasons, we prefer to use client IDs of existing applications owned and operated by GitHub.

The bottom panel for a VSCode device authentication. Much better.

## About the Authors

### [John Stawinski](https://www.praetorian.com/author/john-stawinski/)

John is a Red Team Operator at Praetorian, focused on covert operations, CICD + supply chain security, corporate engagements, and public vulnerability research.

https://www.linkedin.com/in/john-stawinski-72ba87191/

### [Mason Davis](https://www.praetorian.com/author/mason-davis/)

Mason is a Red Team operator at Praetorian, focused on objective-based Red Team operations, CI/CD and supply chain exploitation, and advancing internal and open-source offensive tooling.

https://www.linkedin.com/in/mason-davis/

### [Matt Jackoski](https://www.praetorian.com/author/matt-jackoski/)

Matt is a Red Team operator at Praetorian and currently is focused on performing adversarial emulation engagements for clients. Matt has a background in computer science and engineering with a focus in offensive security.

https://www.linkedin.com/in/matthew-jackoski/

## Catch the Latest

Catch our latest exploits, news, articles, and events.

https://www.praetorian.com/blog/php2shell/

-    [CVE](https://www.praetorian.com/category/cve/), [Offensive Security](https://www.praetorian.com/category/offensive-security/), [Uncategorized](https://www.praetorian.com/category/uncategorized/), [Vulnerability Research](https://www.praetorian.com/category/vulnerability-research/)

-   October 27, 2026

https://www.praetorian.com/blog/php2shell/

## [php2shell in Practice: Exploitable, Conditional, and Hard to Mass-Exploit](https://www.praetorian.com/blog/php2shell/)

[Read More](https://www.praetorian.com/blog/php2shell/)

https://www.praetorian.com/blog/slack-integration/

-    [Identity Security](https://www.praetorian.com/category/identity-security/), [Praetorian Guard Platform](https://www.praetorian.com/category/guard/)

-   October 9, 2026

https://www.praetorian.com/blog/slack-integration/

## [Hunt, Triage, and Act on Guard Findings in Slack](https://www.praetorian.com/blog/slack-integration/)

[Read More](https://www.praetorian.com/blog/slack-integration/)

https://www.praetorian.com/blog/autonomous-penetration-testing-hannibal/

-    [AI Offensive Security](https://www.praetorian.com/category/ai-security/ai-offensive-security/), [AI Security](https://www.praetorian.com/category/ai-security/), [Application Security](https://www.praetorian.com/category/application-security/), [Offensive Security](https://www.praetorian.com/category/offensive-security/), [Open Source Tools](https://www.praetorian.com/category/open-source-tools/), [Praetorian Guard Platform](https://www.praetorian.com/category/guard/)

-   September 23, 2026

https://www.praetorian.com/blog/autonomous-penetration-testing-hannibal/

## [Hannibal Puts Autonomous Penetration Testing Under Your Control](https://www.praetorian.com/blog/autonomous-penetration-testing-hannibal/)

[Read More](https://www.praetorian.com/blog/autonomous-penetration-testing-hannibal/)

## Ready to Discuss Your Next Continuous Threat Exposure Management Initiative?

Praetorian’s Offense Security Experts are Ready to Answer Your Questions

[Get Started](https://www.praetorian.com/contact-us/)
