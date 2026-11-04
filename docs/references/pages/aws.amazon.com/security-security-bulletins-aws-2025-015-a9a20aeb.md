---
url: "https://aws.amazon.com/security/security-bulletins/AWS-2025-015/"
title: "Security Update for Amazon Q Developer Extension for Visual Studio Code (Version #1.84)"
site: "Amazon Web Services, Inc."
captured: "2026-11-02T18:21:48Z"
via: "live"
sha256: "ef74c426a2ac1d9e7cb979fd6bf81cbae2178f5a7b7e5fa4f204b06259d4a746"
wayback: "https://web.archive.org/web/20261005124204/https://aws.amazon.com/security/security-bulletins/AWS-2025-015/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Security Update for Amazon Q Developer Extension for Visual Studio Code (Version #1.84)

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://aws.amazon.com/security/security-bulletins/AWS-2025-015/>
> Snapshot: <https://web.archive.org/web/20261005124204/https://aws.amazon.com/security/security-bulletins/AWS-2025-015/>

### Security Update for Amazon Q Developer Extension for Visual Studio Code (Version #1.84)

Scope: AWS
Content Type: Important (requires attention)
Publication Date: 2025/07/23 6:00 PM PDT
Updated Date: 2025/07/25 6:00 PM PDT

Description:

Amazon Q Developer for Visual Studio Code (VS Code) Extension is a development tool that integrates Amazon Q's AI-powered coding assistance directly into the VS Code integrated development environment (IDE).

AWS is aware of and has addressed an issue in the Amazon Q Developer for VS Code Extension, which is assigned to [CVE-2025-8217](https://www.cve.org/CVERecord?id=CVE-2025-8217).

In the course of our investigation of [AWS-2025-016](https://aws.amazon.com/security/security-bulletins/aws-2025-016/), we determined that Amazon Q Developer for VS Code Extension had an inappropriately scoped GitHub token in their CodeBuild configuration. With that access token, the threat actor was able to commit malicious code into the extension's open-source repository that was automatically included in a release. After we identified this, we immediately revoked and replaced the credentials, removed the malicious code from the code base, and subsequently released Amazon Q Developer for VS Code Extension version 1.85.0.

AWS Security has inspected the code and determined the malicious code was distributed with the extension but was unsuccessful in executing due to a syntax error. This prevented the malicious code from making changes to any services or customer environments.

We will update this bulletin if we have additional information to share.

Impacted versions:

Amazon Q Developer for Visual Studio Code Extension (version 1.84.0)

Resolution:

AWS has taken all necessary mitigation steps to secure AWS systems and has released Amazon Q Developer for VS Code Extension version 1.85.0. This includes removing 1.84.0 from distribution channels so that no further customers can install it. While the malicious code cannot execute, it is still present in existing installations of 1.84.0. As such, all installations of 1.84.0 should be removed from use and customers should update to 1.85.0, including any forked or derivative copies.

To update your Amazon Q Developer for VS Code Extension:

- Open Visual Studio Code
- Navigate to Extensions panel
- Locate Amazon Q Developer
- Click Update button

Please refer to the following hash for version 1.84.0:

- sha256:47f7840ecab6312d2733e1274c513050405886c70f2037fb2f1e9099872b0464

References:

- https://github.com/aws/aws-toolkit-vscode
- [CVE-2025-8217](https://www.cve.org/CVERecord?id=CVE-2025-8217)
- [GHSA-7g7f-ff96-5gcw](https://github.com/aws/aws-toolkit-vscode/security/advisories/GHSA-7g7f-ff96-5gcw)
- [AWS-2025-016](https://aws.amazon.com/security/security-bulletins/aws-2025-016/)

Please email aws-security@amazon.com with any security questions or concerns.
