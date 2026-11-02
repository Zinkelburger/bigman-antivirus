---
url: "https://cwe.mitre.org/data/definitions/601.html"
title: "CWE - CWE-601: URL Redirection to Untrusted Site (5.0)"
captured: "2026-11-02T18:26:30Z"
via: "live"
sha256: "c8fe02d8f5a1dc3848cb9e6653b3b0644e0a27925455643172646f7eadff18a6"
wayback: "https://web.archive.org/web/20261003045222/https://cwe.mitre.org/data/definitions/601.html"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# CWE - CWE-601: URL Redirection to Untrusted Site (5.0)

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://cwe.mitre.org/data/definitions/601.html>
> Snapshot: <https://web.archive.org/web/20261003045222/https://cwe.mitre.org/data/definitions/601.html>

[# Common Weakness Enumeration

Celebrating 20 years of community-developed cybersecurity weakness knowledge!](https://cwe.mitre.org/index.html)

https://cwe.mitre.org/about/new_to_cwe.html

https://cwe.mitre.org/topHW/index.html

https://cwe.mitre.org/top25/

|  [Home](https://cwe.mitre.org/) > [CWE List](https://cwe.mitre.org/data/index.html) > CWE-601: URL Redirection to Untrusted Site (5.0)   |

- [Home](https://cwe.mitre.org/index.html)
-

[Who We Are](https://cwe.mitre.org/about/index.html) [User Stories](https://cwe.mitre.org/about/user_stories.html) [History](https://cwe.mitre.org/about/history.html) [Documents](https://cwe.mitre.org/about/documents.html) [Videos](https://cwe.mitre.org/about/cwe_videos.html)

-

[Basics](https://cwe.mitre.org/about/new_to_cwe.html)

[Root Cause Mapping ►](https://cwe.mitre.org/documents/cwe_usage/guidance.html)

[Guidance](https://cwe.mitre.org/documents/cwe_usage/guidance.html) [Quick Tips](https://cwe.mitre.org/documents/cwe_usage/quick_tips.html) [Examples](https://cwe.mitre.org/documents/cwe_usage/mapping_examples.html)

[How to Contribute Weakness Content](https://cwe.mitre.org/community/submissions/overview.html) [FAQs](https://cwe.mitre.org/about/faq.html) [Glossary](https://cwe.mitre.org/documents/glossary/index.html)

-

[Top-N Lists ►](https://cwe.mitre.org/scoring/index.html#top_n_lists)

[Top 25 Software](https://cwe.mitre.org/top25/) [Top Hardware](https://cwe.mitre.org/topHW/index.html) [Top 10 KEV Weaknesses](https://cwe.mitre.org/top25/archive/2025/2025_kev_list.html)

[CWE List ►](https://cwe.mitre.org/data/index.html)

[Current Version](https://cwe.mitre.org/data/index.html) [Reports](https://cwe.mitre.org/data/reports.html) [Visualizations](https://cwe.mitre.org/data/pdfs.html) [Releases Archive](https://cwe.mitre.org/data/archive.html)

[Downloads](https://cwe.mitre.org/data/downloads.html) [REST API](https://github.com/CWE-CAPEC/REST-API-wg/blob/main/Quick%20Start.md)

-

[News ►](https://cwe.mitre.org/news/)

[Current News](https://cwe.mitre.org/news/archives/news2026.html) [Blog](https://medium.com/@CWE_CAPEC) [Podcast](https://cwe.mitre.org/news/podcast.html) [News Archive](https://cwe.mitre.org/news/archives/index.html)

[CWE Board](https://cwe.mitre.org/community/board.html) [Working Groups & Special Interest Groups](https://cwe.mitre.org/community/working_groups.html) [Email Lists](https://cwe.mitre.org/community/registration.html)

-

[Search CWE List](https://cwe.mitre.org/) [Search Website](https://cwe.mitre.org/find/index.html)

|       |    |

|

|

CWE Glossary Definition

|

|

## CWE-601: URL Redirection to Untrusted Site

|

Weakness ID: 601

[Vulnerability Mapping](https://cwe.mitre.org/data/definitions/601.html#Vulnerability_Mapping_Notes_601): ALLOWED This CWE ID may be used to map to real-world vulnerabilities
Abstraction: Base Base - a weakness that is still mostly independent of a resource or technology, but with sufficient details to provide specific methods for detection and prevention. Base level weaknesses typically describe issues in terms of 2 or 3 of the following dimensions: behavior, property, technology, language, and resource.

View customized information:

Shows the description and diagram, platforms, mitigations, consequences, relationships, memberships, and content history.    For users who are interested in more notional aspects of a weakness. Example: educators, technical writers, and project/program managers.    For users who are concerned with the practical application and details about the nature of a weakness and how to prevent it from happening. Example: tool developers, security researchers, pen-testers, incident response analysts.    For users who are mapping an issue to CWE/CAPEC IDs, i.e., finding the most appropriate CWE for a specific issue (e.g., a CVE record). Example: tool developers, security researchers.    For users who wish to see all available information for the CWE/CAPEC entry.    For users who want to customize what details are displayed.

×

## Edit Custom Filter

More information is available — Please edit the custom filter or select a different filter.

Description

| The web application accepts a user-controlled input that specifies a link to an external site, and uses that link in a redirect.  |  https://cwe.mitre.org/data/images/CWE-601-Diagram.png

Alternate Terms

|  Open Redirect   |

Roles: Common-Name

|  Cross-site Redirect   |

|  Cross-domain Redirect   |

|  Unvalidated Redirect   |

Roles: Common-Name

|  Drive-by download   |

Roles: Attack-Focused

an attack, sometimes enabled by open redirects, which redirects the victim to a site that automatically triggers a download action of malicious software or files

Common Consequences

This table specifies different individual consequences associated with the weakness. The Scope identifies the application security area that is violated, while the Impact describes the negative technical impact that arises if an adversary succeeds in exploiting this weakness. The Likelihood provides information about how likely the specific consequence is expected to be seen relative to the other consequences in the list. For example, there may be high likelihood that a weakness will be exploited to achieve a certain impact, but a low likelihood that it will be exploited to achieve a different impact.

| Impact  | Details

|

Bypass Protection Mechanism; Gain Privileges or Assume Identity

|  Scope: Access Control

The user may be redirected to an untrusted page that contains malware which may then compromise the user's system. In some cases, an open redirect can also enable the immediate download of a file without the user's permission, because the redirection to an external site may lead to endpoints on those sites that automatically trigger a download action ("drive-by download" [[REF-1478](https://cwe.mitre.org/data/definitions/601.html#REF-1478_601)]). This will expose the user to extensive risk. The user's interaction with the web server may also be compromised if the malware conducts keylogging or other attacks that steal credentials, personally identifiable information (PII), or other important data.

|

Bypass Protection Mechanism; Gain Privileges or Assume Identity; Other

|  Scope: Access Control, Confidentiality, Other

By modifying the URL value to a malicious site, an attacker may successfully launch a phishing scam. The user may be subjected to phishing attacks by being redirected to an untrusted page. The phishing attack may point to an attacker controlled web page that appears to be a trusted web site. The phishers may then steal the user's credentials and then use these credentials to access the legitimate web site. Because the server name in the modified link is identical to the original site, phishing attempts can appear more trustworthy.

Potential Mitigations

| Phase(s)  | Mitigation

|

Implementation

|

Strategy: Input Validation

Assume all input is malicious. Use an "accept known good" input validation strategy, i.e., use a list of acceptable inputs that strictly conform to specifications. Reject any input that does not strictly conform to specifications, or transform it into something that does.

When performing input validation, consider all potentially relevant properties, including length, type of input, the full range of acceptable values, missing or extra inputs, syntax, consistency across related fields, and conformance to business rules. As an example of business rule logic, "boat" may be syntactically valid because it only contains alphanumeric characters, but it is not valid if the input is only expected to contain colors such as "red" or "blue."

Do not rely exclusively on looking for malicious or malformed inputs. This is likely to miss at least one undesirable input, especially if the code's environment changes. This can give attackers enough room to bypass the intended validation. However, denylists can be useful for detecting potential attacks or determining which inputs are so malformed that they should be rejected outright.

Use a list of approved URLs or domains to be used for redirection.

|

Architecture and Design

|

Use an intermediate disclaimer page that provides the user with a clear warning that they are leaving the current site. Implement a long timeout before the redirect occurs, or force the user to click on the link. Be careful to avoid XSS problems ([CWE-79](https://cwe.mitre.org/data/definitions/79.html)) when generating the disclaimer page.

|

Architecture and Design

|

Strategy: Enforcement by Conversion

When the set of acceptable objects, such as filenames or URLs, is limited or known, create a mapping from a set of fixed input values (such as numeric IDs) to the actual filenames or URLs, and reject all other inputs.

For example, ID 1 could map to "/login.asp" and ID 2 could map to "http://www.example.com/". Features such as the ESAPI AccessReferenceMap [[REF-45](https://cwe.mitre.org/data/definitions/601.html#REF-45_601)] provide this capability.

|

Architecture and Design

|

Ensure that no externally-supplied requests are honored by requiring that all redirect requests include a unique nonce generated by the application [[REF-483](https://cwe.mitre.org/data/definitions/601.html#REF-483_601)]. Be sure that the nonce is not predictable ([CWE-330](https://cwe.mitre.org/data/definitions/330.html)).

Note:  Note that this can be bypassed using XSS ([CWE-79](https://cwe.mitre.org/data/definitions/79.html)).

|

Architecture and Design; Implementation

|

Strategy: Attack Surface Reduction

Understand all the potential areas where untrusted inputs can enter your software: parameters or arguments, cookies, anything read from the network, environment variables, reverse DNS lookups, query results, request headers, URL components, e-mail, files, filenames, databases, and any external systems that provide data to the application. Remember that such inputs may be obtained indirectly through API calls.

Many open redirect problems occur because the programmer assumed that certain inputs could not be modified, such as cookies and hidden form fields.

|

Operation

|

Strategy: Firewall

Use an application firewall that can detect attacks against this weakness. It can be beneficial in cases in which the code cannot be fixed (because it is controlled by a third party), as an emergency prevention measure while more comprehensive software assurance measures are applied, or to provide defense in depth [[REF-1481](https://cwe.mitre.org/data/definitions/601.html#REF-1481_601)].

Effectiveness: Moderate

Note:  An application firewall might not cover all possible input vectors. In addition, attack techniques might be available to bypass the protection mechanism, such as using malformed inputs that can still be processed by the component that receives those inputs. Depending on functionality, an application firewall might inadvertently reject or modify legitimate requests. Finally, some manual effort may be required for customization.

Relationships

This table shows the weaknesses and high level categories that are related to this weakness. These relationships are defined as ChildOf, ParentOf, MemberOf and give insight to similar items that may exist at higher and lower levels of abstraction. In addition, relationships such as PeerOf, Overlaps, and CanAlsoBe are defined to show similar weaknesses that the user may want to explore.

|   Relevant to the view "Complete Weakness Hierarchy" (View-1000)

| Nature  | Type  | ID  | Name

| ChildOf  |   Class - a weakness that is described in a very abstract fashion, typically independent of any specific language or technology. More specific than a Pillar Weakness, but more general than a Base Weakness. Class level weaknesses typically describe issues in terms of 1 or 2 of the following dimensions: behavior, property, and resource.    |  [610](https://cwe.mitre.org/data/definitions/610.html)   |  Externally Controlled Reference to a Resource in Another Sphere

| CanFollow  |   Variant - a weakness that is linked to a certain type of product, typically involving a specific language or technology. More specific than a Base weakness. Variant level weaknesses typically describe issues in terms of 3 to 5 of the following dimensions: behavior, property, technology, language, and resource.    |  [50](https://cwe.mitre.org/data/definitions/50.html)   |  Path Equivalence: '//multiple/leading/slash'

|   Relevant to the view "Software Development" (View-699)

| Nature  | Type  | ID  | Name

| MemberOf  |   Grouping - a CWE entry that contains a set of other entries that share a common characteristic.    |  [19](https://cwe.mitre.org/data/definitions/19.html)   |  Data Processing Errors

|   Relevant to the view "Weaknesses for Simplified Mapping of Published Vulnerabilities" (View-1003)

| Nature  | Type  | ID  | Name

| ChildOf  |   Class - a weakness that is described in a very abstract fashion, typically independent of any specific language or technology. More specific than a Pillar Weakness, but more general than a Base Weakness. Class level weaknesses typically describe issues in terms of 1 or 2 of the following dimensions: behavior, property, and resource.    |  [610](https://cwe.mitre.org/data/definitions/610.html)   |  Externally Controlled Reference to a Resource in Another Sphere

|   Relevant to the view "Architectural Concepts" (View-1008)

| Nature  | Type  | ID  | Name

| MemberOf  |   Grouping - a CWE entry that contains a set of other entries that share a common characteristic.    |  [1019](https://cwe.mitre.org/data/definitions/1019.html)   |  Validate Inputs

Background Details

| Phishing is a general term for deceptive attempts to coerce private information from users that will be used for identity theft.

Modes Of Introduction

The different Modes of Introduction provide information about how and when this weakness may be introduced. The Phase identifies a point in the life cycle at which introduction may occur, while the Note provides a typical scenario related to introduction during the given phase.

| Phase(s)  | Note

|

Architecture and Design

| OMISSION: This weakness is caused by missing a security tactic during the architecture and design phase.

|

Implementation

|

Applicable Platforms

This listing shows possible areas for which the given weakness could appear. These may be for specific named Languages, Operating Systems, Architectures, Paradigms, Technologies, or a class of such platforms. The platform is listed along with how frequently the given weakness appears for that instance.

| Type  | Applicable Platforms  | Notes

| Languages  |

Class: Not Language-Specific (Undetermined Prevalence)

|

| Technologies  |

Class: Web Based (Undetermined Prevalence)

Web Server (Undetermined Prevalence)

|

Likelihood Of Exploit

Low

Demonstrative Examples

Example 1

The following code obtains a URL from the query string and then redirects the user to that URL.

(bad code)

Example Language: PHP

$redirect_url = $_GET['url'];
header("Location: " . $redirect_url);

The problem with the above code is that an attacker could use this page as part of a phishing scam by redirecting users to a malicious site. For example, assume the above code is in the file example.php. An attacker could supply a user with the following link:

(attack code)

http://example.com/example.php?url=http://malicious.example.com

The user sees the link pointing to the original trusted site (example.com) and does not realize the redirection that could take place.

Example 2

The following code is a Java servlet that will receive a GET request with a url parameter in the request to redirect the browser to the address specified in the url parameter. The servlet will retrieve the url parameter value from the request and send a response to redirect the browser to the url address.

(bad code)

Example Language: Java

public class RedirectServlet extends HttpServlet {

protected void doGet(HttpServletRequest request, HttpServletResponse response) throws ServletException, IOException {

String query = request.getQueryString();
if (query.contains("url")) {

String url = request.getParameter("url");
response.sendRedirect(url);

}

}

}

The problem with this Java servlet code is that an attacker could use the RedirectServlet as part of an e-mail phishing scam to redirect users to a malicious site. An attacker could send an HTML formatted e-mail directing the user to log into their account by including in the e-mail the following link:

(attack code)

Example Language: HTML

<a href="http://bank.example.com/redirect?url=http://attacker.example.net">Click here to log in</a>

The user may assume that the link is safe since the URL starts with their trusted bank, bank.example.com. However, the user will then be redirected to the attacker's web site (attacker.example.net) which the attacker may have made to appear very similar to bank.example.com. The user may then unwittingly enter credentials into the attacker's web page and compromise their bank account. A Java servlet should never redirect a user to a URL without verifying that the redirect address is a trusted site.

Selected Observed Examples

Note: this is a curated list of examples for users to understand the variety of ways in which this weakness can be introduced. It is not a complete list of all CVEs that are related to this CWE entry.

| Reference  | Description

|

[CVE-2005-4206](https://www.cve.org/CVERecord?id=CVE-2005-4206)

|

URL parameter loads the URL into a frame and causes it to appear to be part of a valid page.

|

[CVE-2008-2951](https://www.cve.org/CVERecord?id=CVE-2008-2951)

|

An open redirect vulnerability in the search script in the software allows remote attackers to redirect users to arbitrary web sites and conduct phishing attacks via a URL as a parameter to the proper function.

|

[CVE-2008-2052](https://www.cve.org/CVERecord?id=CVE-2008-2052)

|

Open redirect vulnerability in the software allows remote attackers to redirect users to arbitrary web sites and conduct phishing attacks via a URL in the proper parameter.

|

[CVE-2020-11053](https://www.cve.org/CVERecord?id=CVE-2020-11053)

|

Chain: Go-based Oauth2 reverse proxy can send the authenticated user to another site at the end of the authentication flow. A redirect URL with HTML-encoded whitespace characters can bypass the validation ([CWE-1289](https://cwe.mitre.org/data/definitions/1289.html)) to redirect to a malicious site ([CWE-601](https://cwe.mitre.org/data/definitions/601.html))

Weakness Ordinalities

| Ordinality  | Description

|

Primary

|

(where the weakness exists independent of other weaknesses)

Detection Methods

| Method  | Details

|

Manual Static Analysis

|

Since this weakness does not typically appear frequently within a single software package, manual white box techniques may be able to provide sufficient code coverage and reduction of false positives if all potentially-vulnerable operations can be assessed within limited time constraints.

Effectiveness: High

|

Automated Dynamic Analysis

|

Automated black box tools that supply URLs to every input may be able to spot Location header modifications, but test case coverage is a factor, and custom redirects may not be detected.

|

Automated Static Analysis

|

Automated static analysis tools may not be able to determine whether input influences the beginning of a URL, which is important for reducing false positives.

|

Automated Static Analysis

|

Automated static analysis, commonly referred to as Static Application Security Testing (SAST), can find some instances of this weakness by analyzing source code (or binary/compiled code) without having to execute it. Typically, this is done by building a model of data flow and control flow, then searching for potentially-vulnerable patterns that connect "sources" (origins of input) with "sinks" (destinations where the data interacts with external components, a lower layer such as the OS, etc.)

Effectiveness: High

|

Automated Static Analysis - Binary or Bytecode

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/601.html#REF-1479_601)], the following detection techniques may be useful:

Highly cost effective:

- Bytecode Weakness Analysis - including disassembler + source code weakness analysis
- Binary Weakness Analysis - including disassembler + source code weakness analysis

Effectiveness: High

|

Dynamic Analysis with Automated Results Interpretation

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/601.html#REF-1479_601)], the following detection techniques may be useful:

Highly cost effective:

- Web Application Scanner
- Web Services Scanner
- Database Scanners

Effectiveness: High

|

Dynamic Analysis with Manual Results Interpretation

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/601.html#REF-1479_601)], the following detection techniques may be useful:

Highly cost effective:

- Fuzz Tester
- Framework-based Fuzzer

Effectiveness: High

|

Manual Static Analysis - Source Code

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/601.html#REF-1479_601)], the following detection techniques may be useful:

Highly cost effective:

- Manual Source Code Review (not inspections)

Effectiveness: High

|

Automated Static Analysis - Source Code

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/601.html#REF-1479_601)], the following detection techniques may be useful:

Highly cost effective:

- Source code Weakness Analyzer
- Context-configured Source Code Weakness Analyzer

Effectiveness: High

|

Architecture or Design Review

|

According to SOAR [[REF-1479](https://cwe.mitre.org/data/definitions/601.html#REF-1479_601)], the following detection techniques may be useful:

Highly cost effective:

- Formal Methods / Correct-By-Construction

Cost effective for partial coverage:

- Inspection (IEEE 1028 standard) (can apply to requirements, design, source code, etc.)

Effectiveness: High

Memberships

This MemberOf Relationships table shows additional CWE Categories and Views that reference this weakness as a member. This information is often useful in understanding where a weakness fits within the context of external information sources.

| Nature  | Type  | ID  | Name

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [722](https://cwe.mitre.org/data/definitions/722.html)   |  OWASP Top Ten 2004 Category A1 - Unvalidated Input

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [801](https://cwe.mitre.org/data/definitions/801.html)   |  2010 Top 25 - Insecure Interaction Between Components

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [819](https://cwe.mitre.org/data/definitions/819.html)   |  OWASP Top Ten 2010 Category A10 - Unvalidated Redirects and Forwards

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [864](https://cwe.mitre.org/data/definitions/864.html)   |  2011 Top 25 - Insecure Interaction Between Components

| MemberOf  | View - a subset of CWE entries that provides a way of examining CWE content. The two main view structures are Slices (flat lists) and Graphs (containing relationships between entries).  |  [884](https://cwe.mitre.org/data/definitions/884.html)   |  CWE Cross-section

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [938](https://cwe.mitre.org/data/definitions/938.html)   |  OWASP Top Ten 2013 Category A10 - Unvalidated Redirects and Forwards

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [990](https://cwe.mitre.org/data/definitions/990.html)   |  SFP Secondary Cluster: Tainted Input to Command

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [1345](https://cwe.mitre.org/data/definitions/1345.html)   |  OWASP Top Ten 2021 Category A01:2021 - Broken Access Control

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [1382](https://cwe.mitre.org/data/definitions/1382.html)   |  ICS Operations (& Maintenance): Emerging Energy Technologies

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [1396](https://cwe.mitre.org/data/definitions/1396.html)   |  Comprehensive Categorization: Access Control

| MemberOf  | Grouping - a CWE entry that contains a set of other entries that share a common characteristic.  |  [1436](https://cwe.mitre.org/data/definitions/1436.html)   |  OWASP Top Ten 2025 Category A01:2025 - Broken Access Control

Vulnerability Mapping Notes

|  Usage   |   ALLOWED

(this CWE ID may be used to map to real-world vulnerabilities)

| Reason  |  Acceptable-Use

|

Rationale

|  This CWE entry is at the Base level of abstraction, which is a preferred level of abstraction for mapping to the root causes of vulnerabilities.

|

Comments

|  Carefully read both the name and description to ensure that this mapping is an appropriate fit. Do not try to 'force' a mapping to a lower-level Base/Variant simply to comply with this preferred level of abstraction.

Notes

Other

Whether this issue poses a vulnerability will be subject to the intended behavior of the application. For example, a search engine might intentionally provide redirects to arbitrary URLs.

Taxonomy Mappings

| Mapped Taxonomy Name  | Node ID  | Fit  | Mapped Node Name

| WASC  | 38  |   | URl Redirector Abuse

| Software Fault Patterns  | SFP24  |   | Tainted input to command

Related Attack Patterns

| CAPEC-ID  | Attack Pattern Name

| [CAPEC-178](http://capec.mitre.org/data/definitions/178.html)  | Cross-Site Flashing

References

| [REF-483]  |  Craig A. Shue, Andrew J. Kalafut and Minaxi Gupta. "Exploitable Redirects on the Web: Identification, Prevalence, and Defense".
<https://www.cprogramming.com/tutorial/exceptions.html>. (URL validated: 2023-04-07)

| [REF-484]  |  Russ McRee. "Open redirect vulnerabilities: definition and prevention". Page 43. Issue 17. (IN)SECURE. 2008-07.
<https://img2.helpnetsecurity.com/dl/insecure/INSECURE-Mag-17.pdf>. (URL validated: 2025-07-29)

| [REF-485]  |  Jason Lam. "Top 25 Series - Rank 23 - Open Redirect". SANS Software Security Institute. 2010-03-25.
<https://www.sans.org/blog/top-25-series-rank-23-open-redirect>. (URL validated: 2025-07-29)

| [REF-45]  |  OWASP. "OWASP Enterprise Security API (ESAPI) Project".
<https://owasp.org/www-project-enterprise-security-api/>. (URL validated: 2025-07-24)

| [REF-1478]  |  Wikipedia. "Drive-by download". 2025-05-24.
<https://en.wikipedia.org/wiki/Drive-by_download>. (URL validated: 2025-08-21)

| [REF-1479]  |  Gregory Larsen, E. Kenneth Hong Fong, David A. Wheeler and Rama S. Moorthy. "State-of-the-Art Resources (SOAR) for Software Vulnerability Detection, Test, and Evaluation". 2014-07.
<https://www.ida.org/-/media/feature/publications/s/st/stateoftheart-resources-soar-for-software-vulnerability-detection-test-and-evaluation/p-5061.ashx>. (URL validated: 2025-09-05)

| [REF-1481]  |  D3FEND. "D3FEND: Application Layer Firewall".
<https://d3fend.mitre.org/dao/artifact/d3f:ApplicationLayerFirewall/>. (URL validated: 2025-09-06)

Content History

|  Submissions

| Submission Date  | Submitter  | Organization

|  2007-05-07
(CWE Draft 6, 2007-05-07)   | Anonymous Tool Vendor (under NDA)  |

|

|  Contributions

| Contribution Date  | Contributor  | Organization

|  2023-11-25
(CWE 4.18, 2025-09-09)   | Michal Biesiada  |

|  suggested that CWE include drive-by downloads and contributed references

|  Modifications

| Modification Date  | Modifier  | Organization

|  2026-10-28
(CWE 5.0, 2026-10-28)   | CWE Content Team  | MITRE

|  updated Common_Consequences, Name, Relationships

|  2025-12-11
(CWE 4.19, 2025-12-11)   | CWE Content Team  | MITRE

|  updated Applicable_Platforms, Relationships, Weakness_Ordinalities

|  2025-09-09
(CWE 4.18, 2025-09-09)   | CWE Content Team  | MITRE

|  updated Alternate_Terms, Common_Consequences, Detection_Factors, Potential_Mitigations, References

|  2024-11-19
(CWE 4.16, 2024-11-19)   | CWE Content Team  | MITRE

|  updated Alternate_Terms, Common_Consequences, Description, Diagram, Other_Notes

|  2024-02-29
(CWE 4.14, 2024-02-29)   | CWE Content Team  | MITRE

|  updated Demonstrative_Examples

|  2023-06-29
(CWE 4.12, 2023-06-29)   | CWE Content Team  | MITRE

|  updated Mapping_Notes

|  2023-04-27
(CWE 4.11, 2023-04-27)   | CWE Content Team  | MITRE

|  updated Description, Detection_Factors, References, Relationships

|  2023-01-31
(CWE 4.10, 2023-01-31)   | CWE Content Team  | MITRE

|  updated Related_Attack_Patterns

|  2022-10-13
(CWE 4.9, 2022-10-13)   | CWE Content Team  | MITRE

|  updated Observed_Examples

|  2022-04-28
(CWE 4.7, 2022-04-28)   | CWE Content Team  | MITRE

|  updated Relationships

|  2021-10-28
(CWE 4.6, 2021-10-28)   | CWE Content Team  | MITRE

|  updated Relationships

|  2020-06-25
(CWE 4.1, 2020-06-25)   | CWE Content Team  | MITRE

|  updated Potential_Mitigations

|  2020-02-24
(CWE 4.0, 2020-02-24)   | CWE Content Team  | MITRE

|  updated Applicable_Platforms, Potential_Mitigations, Relationships

|  2019-06-20
(CWE 3.3, 2019-06-20)   | CWE Content Team  | MITRE

|  updated Relationships, Type

|  2019-01-03
(CWE 3.2, 2019-01-03)   | CWE Content Team  | MITRE

|  updated Related_Attack_Patterns

|  2017-11-08
(CWE 3.0, 2017-11-08)   | CWE Content Team  | MITRE

|  updated Likelihood_of_Exploit, Modes_of_Introduction, References, Relationships, Taxonomy_Mappings

|  2015-12-07
(CWE 2.9, 2015-12-07)   | CWE Content Team  | MITRE

|  updated Relationships

|  2014-07-30
(CWE 2.8, 2014-07-31)   | CWE Content Team  | MITRE

|  updated Detection_Factors, Relationships, Taxonomy_Mappings

|  2013-07-17
(CWE 2.5, 2013-07-17)   | CWE Content Team  | MITRE

|  updated References, Relationships

|  2012-10-30
(CWE 2.3, 2012-10-30)   | CWE Content Team  | MITRE

|  updated Potential_Mitigations

|  2012-05-11
(CWE 2.2, 2012-05-15)   | CWE Content Team  | MITRE

|  updated Relationships

|  2011-09-13
(CWE 2.1, 2011-09-13)   | CWE Content Team  | MITRE

|  updated Potential_Mitigations, References

|  2011-06-27
(CWE 2.0, 2011-06-27)   | CWE Content Team  | MITRE

|  updated Relationships

|  2011-06-01
(CWE 1.13, 2011-06-01)   | CWE Content Team  | MITRE

|  updated Common_Consequences

|  2010-09-27
(CWE 1.10, 2010-09-27)   | CWE Content Team  | MITRE

|  updated Potential_Mitigations

|  2010-06-21
(CWE 1.9, 2010-06-21)   | CWE Content Team  | MITRE

|  updated Common_Consequences, Potential_Mitigations, References, Relationships

|  2010-04-05
(CWE 1.8.1, 2010-04-05)   | CWE Content Team  | MITRE

|  updated Demonstrative_Examples

|  2010-02-16
(CWE 1.8, 2010-02-16)   | CWE Content Team  | MITRE

|  updated Applicable_Platforms, Common_Consequences, Detection_Factors, Potential_Mitigations, Related_Attack_Patterns, Relationships, Taxonomy_Mappings

|  2009-12-28
(CWE 1.7, 2009-12-28)   | CWE Content Team  | MITRE

|  updated Demonstrative_Examples, Detection_Factors, Likelihood_of_Exploit, Potential_Mitigations

|  2009-05-27
(CWE 1.4, 2009-05-27)   | CWE Content Team  | MITRE

|  updated Name

|  2009-03-10
(CWE 1.3, 2009-03-10)   | CWE Content Team  | MITRE

|  updated Relationships

|  2008-10-14
(CWE 1.0.1, 2008-10-14)   | CWE Content Team  | MITRE

|  updated Alternate_Terms, Observed_Examples, References

|  2008-10-03
(CWE 1.1, 2008-11-25)   | CWE Content Team  | MITRE

|  updated References and Observed_Examples

|  2008-09-08
(CWE 1.0, 2008-09-09)   | CWE Content Team  | MITRE

|  updated Alternate_Terms, Background_Details, Description, Detection_Factors, Likelihood_of_Exploit, Name, Relationships, Observed_Example, Taxonomy_Mappings

|  2008-07-01
(CWE 1.0, 2008-09-09)   | Eric Dalci  | Cigital

|  updated Potential_Mitigations, Time_of_Introduction

|  Previous Entry Names

| Change Date  | Previous Entry Name

| 2026-10-28   | URL Redirection to Untrusted Site ('Open Redirect')

| 2009-05-27   | URL Redirection to Untrusted Site (aka 'Open Redirect')

| 2008-09-09   | URL Redirection to Untrusted Site

| 2008-04-11   | Unsafe URL Redirection

More information is available — Please edit the custom filter or select a different filter.

Page Last Updated:  Oct 28, 2026

|

|

http://www.mitre.org

|

[Site Map](https://cwe.mitre.org/sitemap.html) | [Terms of Use](https://cwe.mitre.org/about/termsofuse.html) | [Manage Cookies](https://cwe.mitre.org/data/definitions/601.html#) | [Cookie Notice](https://cwe.mitre.org/about/cookie_notice.html) | [Privacy Policy](https://cwe.mitre.org/about/privacy_policy.html) | Contact Us | https://x.com/CweCapec https://www.linkedin.com/showcase/cve-cwe-capec https://bsky.app/profile/cweprogram.bsky.social https://mastodon.social/@CWE_Program https://www.youtube.com/channel/UCpY9VIpRmFK4ebD6orssifA https://cwe.mitre.org/news/podcast.html https://medium.com/@CWE_CAPEC

Use of the Common Weakness Enumeration (CWE™) and the associated references from this website are subject to the [Terms of Use](https://cwe.mitre.org/about/termsofuse.html). CWE is sponsored by the [U.S. Department of Homeland Security](https://www.dhs.gov/) (DHS) [Cybersecurity and Infrastructure Security Agency](https://www.dhs.gov/cisa/cybersecurity-division) (CISA) and managed by the [Homeland Security Systems Engineering and Development Institute](https://www.dhs.gov/science-and-technology/hssedi) (HSSEDI) which is operated by [The MITRE Corporation](http://www.mitre.org/) (MITRE). Copyright © 2006–2026, The MITRE Corporation. CWE, CWSS, CWRAF, and the CWE logo are trademarks of The MITRE Corporation.

|

https://www.dhs.gov/science-and-technology/hssedi
