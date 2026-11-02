---
url: "https://github.com/trufflesecurity/trufflehog"
title: "GitHub - trufflesecurity/trufflehog: Find, verify, and analyze leaked credentials"
site: "GitHub"
captured: "2026-11-02T19:08:46Z"
via: "live"
sha256: "c3b0c0760b4bb75b2082ae242075c192b3783e8412c4fc355c3afdd66d744121"
wayback: "https://web.archive.org/web/20261022041717/https://github.com/trufflesecurity/trufflehog"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# GitHub - trufflesecurity/trufflehog: Find, verify, and analyze leaked credentials

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/trufflesecurity/trufflehog>
> Snapshot: <https://web.archive.org/web/20261022041717/https://github.com/trufflesecurity/trufflehog>

https://camo.githubusercontent.com/c8b6dcceeff8daf7ea2e4c4d45f0b1923381fc8fb8b74de998f542bd3d2ffa11/68747470733a2f2f73746f726167652e676f6f676c65617069732e636f6d2f74727566666c65686f672d7374617469632d736f75726365732f706978656c5f7069672e706e67

## TruffleHog

https://github.com/trufflesecurity/trufflehog#trufflehog

Find leaked credentials.

https://goreportcard.com/report/github.com/trufflesecurity/trufflehog/v3 https://github.com/trufflesecurity/trufflehog/blob/main/LICENSE https://github.com/trufflesecurity/trufflehog/blob/main/pkg/detectors

# 🔎 Now Scanning

https://github.com/trufflesecurity/trufflehog#mag_right-now-scanning

https://github.com/trufflesecurity/trufflehog/blob/main/assets/scanning_logos.svg

...and more

To learn more about TruffleHog and its features and capabilities, visit our [product page](https://trufflesecurity.com/trufflehog?gclid=CjwKCAjwouexBhAuEiwAtW_Zx5IW87JNj97Ci7heFnA5ar6-DuNzT2Y5nIl9DuZ-FOUqx0Qg3vb9nxoClcEQAvD_BwE).

# 🌐 TruffleHog Enterprise

https://github.com/trufflesecurity/trufflehog#globe_with_meridians-trufflehog-enterprise

Are you interested in continuously monitoring Git, Jira, Slack, Confluence, Microsoft Teams, Sharepoint (and more) for credentials? We have an enterprise product that can help! Learn more at https://trufflesecurity.com/trufflehog-enterprise.

We take the revenue from the enterprise product to fund more awesome open source projects that the whole community can benefit from.

# What is TruffleHog 🐽

https://github.com/trufflesecurity/trufflehog#what-is-trufflehog-

TruffleHog is the most powerful secrets Discovery, Classification, Validation, and Analysis tool. In this context, secret refers to a credential a machine uses to authenticate itself to another machine. This includes API keys, database passwords, private encryption keys, and more.

## Discovery 🔍

https://github.com/trufflesecurity/trufflehog#discovery-

TruffleHog can look for secrets in many places including Git, chats, wikis, logs, API testing platforms, object stores, filesystems and more.

## Classification 📁

https://github.com/trufflesecurity/trufflehog#classification-

TruffleHog classifies over 800 secret types, mapping them back to the specific identity they belong to. Is it an AWS secret? Stripe secret? Cloudflare secret? Postgres password? SSL Private key? Sometimes it's hard to tell looking at it, so TruffleHog classifies everything it finds.

## Validation ✅

https://github.com/trufflesecurity/trufflehog#validation-

For every secret TruffleHog can classify, it can also log in to confirm if that secret is live or not. This step is critical to know if there’s an active present danger or not.

## Analysis 🔬

https://github.com/trufflesecurity/trufflehog#analysis-

For the 20 some of the most commonly leaked out credential types, instead of sending one request to check if the secret can log in, TruffleHog can send many requests to learn everything there is to know about the secret. Who created it? What resources can it access? What permissions does it have on those resources?

# 📢 Join Our Community

https://github.com/trufflesecurity/trufflehog#loudspeaker-join-our-community

Have questions? Feedback? Jump into Slack or Discord and hang out with us.

Join our [Slack Community](https://join.slack.com/t/trufflehog-community/shared_invite/zt-pw2qbi43-Aa86hkiimstfdKH9UCpPzQ)

Join the [Secret Scanning Discord](https://discord.gg/8Hzbrnkr7E)

# 📺 Demo

https://github.com/trufflesecurity/trufflehog#tv-demo

https://camo.githubusercontent.com/89e2214944f8282f90c4cca2a5a642a83e7fbdd97c818cf788a745a774413370/68747470733a2f2f73746f726167652e676f6f676c65617069732e636f6d2f74727566666c652d64656d6f732f6e6f6e2d696e7465726163746976652e737667

```
docker run --rm -it -v "$PWD:/pwd" trufflesecurity/trufflehog:latest github --org=trufflesecurity
```

# 💾 Installation

https://github.com/trufflesecurity/trufflehog#floppy_disk-installation

Several options are available for you:

### MacOS users

https://github.com/trufflesecurity/trufflehog#macos-users

```
brew install trufflehog
```

### Docker:

https://github.com/trufflesecurity/trufflehog#docker

Ensure Docker engine is running before executing the following commands:

####  Unix

https://github.com/trufflesecurity/trufflehog#unix

```
docker run --rm -it -v "$PWD:/pwd" trufflesecurity/trufflehog:latest github --repo https://github.com/trufflesecurity/test_keys
```

####  Windows Command Prompt

https://github.com/trufflesecurity/trufflehog#windows-command-prompt

```
docker run --rm -it -v "%cd:/=\%:/pwd" trufflesecurity/trufflehog:latest github --repo https://github.com/trufflesecurity/test_keys
```

####  Windows PowerShell

https://github.com/trufflesecurity/trufflehog#windows-powershell

```
docker run --rm -it -v "${PWD}:/pwd" trufflesecurity/trufflehog github --repo https://github.com/trufflesecurity/test_keys
```

####  M1 and M2 Mac

https://github.com/trufflesecurity/trufflehog#m1-and-m2-mac

```
docker run --platform linux/arm64 --rm -it -v "$PWD:/pwd" trufflesecurity/trufflehog:latest github --repo https://github.com/trufflesecurity/test_keys
```

### Binary releases

https://github.com/trufflesecurity/trufflehog#binary-releases

```
Download and unpack from https://github.com/trufflesecurity/trufflehog/releases
```

### Compile from source

https://github.com/trufflesecurity/trufflehog#compile-from-source

```
git clone https://github.com/trufflesecurity/trufflehog.git
cd trufflehog; go install
```

### Using installation script

https://github.com/trufflesecurity/trufflehog#using-installation-script

```
curl -sSfL https://raw.githubusercontent.com/trufflesecurity/trufflehog/main/scripts/install.sh | sh -s -- -b /usr/local/bin
```

### Using installation script, verify checksum signature (requires cosign to be installed)

https://github.com/trufflesecurity/trufflehog#using-installation-script-verify-checksum-signature-requires-cosign-to-be-installed

```
curl -sSfL https://raw.githubusercontent.com/trufflesecurity/trufflehog/main/scripts/install.sh | sh -s -- -v -b /usr/local/bin
```

### Using installation script to install a specific version

https://github.com/trufflesecurity/trufflehog#using-installation-script-to-install-a-specific-version

```
curl -sSfL https://raw.githubusercontent.com/trufflesecurity/trufflehog/main/scripts/install.sh | sh -s -- -b /usr/local/bin <ReleaseTag like v3.56.0>
```

# 🔐 Verifying the artifacts

https://github.com/trufflesecurity/trufflehog#closed_lock_with_key-verifying-the-artifacts

Checksums are applied to all artifacts, and the resulting checksum file is signed using cosign.

You need the following tool to verify signature:

- [Cosign](https://docs.sigstore.dev/cosign/system_config/installation/)

Verification steps are as follows:

-

Download the artifact files you want, and the following files from the [releases](https://github.com/trufflesecurity/trufflehog/releases) page.

- trufflehog_{version}_checksums.txt
- trufflehog_{version}_checksums.txt.pem
- trufflehog_{version}_checksums.txt.sig

-

Verify the signature:

```
cosign verify-blob <path to trufflehog_{version}_checksums.txt> \
--certificate <path to trufflehog_{version}_checksums.txt.pem> \
--signature <path to trufflehog_{version}_checksums.txt.sig> \
--certificate-identity-regexp 'https://github\.com/trufflesecurity/trufflehog/\.github/workflows/.+' \
--certificate-oidc-issuer "https://token.actions.githubusercontent.com"
```

-

Once the signature is confirmed as valid, you can proceed to validate that the SHA256 sums align with the downloaded artifact:

```
sha256sum --ignore-missing -c trufflehog_{version}_checksums.txt
```

Replace {version} with the downloaded files version

Alternatively, if you are using the installation script, pass -v option to perform signature verification. This requires Cosign binary to be installed prior to running the installation script.

# 🚀 Quick Start

https://github.com/trufflesecurity/trufflehog#rocket-quick-start

## 1: Scan a repo for only verified secrets

https://github.com/trufflesecurity/trufflehog#1-scan-a-repo-for-only-verified-secrets

Command:

```
trufflehog git https://github.com/trufflesecurity/test_keys --results=verified
```

Expected output:

```
🐷🔑🐷  TruffleHog. Unearth your secrets. 🐷🔑🐷

Found verified result 🐷🔑
Detector Type: AWS
Decoder Type: PLAIN
Raw result: AKIAYVP4CIPPERUVIFXG
Line: 4
Commit: fbc14303ffbf8fb1c2c1914e8dda7d0121633aca
File: keys
Email: counter <counter@counters-MacBook-Air.local>
Repository: https://github.com/trufflesecurity/test_keys
Timestamp: 2022-06-16 10:17:40 -0700 PDT
...

```

## 2: Scan a GitHub Org for only verified secrets

https://github.com/trufflesecurity/trufflehog#2-scan-a-github-org-for-only-verified-secrets

```
trufflehog github --org=trufflesecurity --results=verified
```

## 3: Scan a GitHub Org excluding archived repositories

https://github.com/trufflesecurity/trufflehog#3-scan-a-github-org-excluding-archived-repositories

```
trufflehog github --org=trufflesecurity --exclude-archived
```

## 4: Scan a GitHub Repo for only verified secrets and get JSON output

https://github.com/trufflesecurity/trufflehog#4-scan-a-github-repo-for-only-verified-secrets-and-get-json-output

Command:

```
trufflehog git https://github.com/trufflesecurity/test_keys --results=verified --json
```

Expected output:

```
{"SourceMetadata":{"Data":{"Git":{"commit":"fbc14303ffbf8fb1c2c1914e8dda7d0121633aca","file":"keys","email":"counter \u003ccounter@counters-MacBook-Air.local\u003e","repository":"https://github.com/trufflesecurity/test_keys","timestamp":"2022-06-16 10:17:40 -0700 PDT","line":4}}},"SourceID":0,"SourceType":16,"SourceName":"trufflehog - git","DetectorType":2,"DetectorName":"AWS","DecoderName":"PLAIN","Verified":true,"Raw":"AKIAYVP4CIPPERUVIFXG","Redacted":"AKIAYVP4CIPPERUVIFXG","ExtraData":{"account":"595918472158","arn":"arn:aws:iam::595918472158:user/canarytokens.com@@mirux23ppyky6hx3l6vclmhnj","user_id":"AIDAYVP4CIPPJ5M54LRCY"},"StructuredData":null}
...

```

TruffleHog can also output [SARIF](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) with --sarif instead of --json. GitHub code scanning understands SARIF natively, so uploading it surfaces findings inline on pull request diffs and in the repository's Security tab, and tracks findings as new/fixed across scans instead of reporting the same one every run — see the [TruffleHog Github Action](https://github.com/trufflesecurity/trufflehog#octocat-trufflehog-github-action) section below for how to upload it. Note that, unlike the other output formats, SARIF results are buffered in memory for the full scan and written out at the end, since SARIF requires a single JSON document rather than a stream — fine for typical scans, but scans producing a very large number of results will use proportionally more memory.

## 5: Scan a GitHub Repo + its Issues and Pull Requests

https://github.com/trufflesecurity/trufflehog#5-scan-a-github-repo--its-issues-and-pull-requests

```
trufflehog github --repo=https://github.com/trufflesecurity/test_keys --issue-comments --pr-comments
```

## 6: Scan an S3 bucket for high-confidence results (verified + unknown)

https://github.com/trufflesecurity/trufflehog#6-scan-an-s3-bucket-for-high-confidence-results-verified--unknown

```
trufflehog s3 --bucket=<bucket name> --results=verified,unknown
```

## 7: Scan S3 buckets using IAM Roles

https://github.com/trufflesecurity/trufflehog#7-scan-s3-buckets-using-iam-roles

```
trufflehog s3 --role-arn=<iam role arn>
```

## 8: Scan a Github Repo using SSH authentication in Docker

https://github.com/trufflesecurity/trufflehog#8-scan-a-github-repo-using-ssh-authentication-in-docker

```
docker run --rm -v "$HOME/.ssh:/root/.ssh:ro" trufflesecurity/trufflehog:latest git ssh://github.com/trufflesecurity/test_keys
```

## 9: Scan individual files or directories

https://github.com/trufflesecurity/trufflehog#9-scan-individual-files-or-directories

```
trufflehog filesystem path/to/file1.txt path/to/file2.txt path/to/dir
```

## 10: Scan a local git repo

https://github.com/trufflesecurity/trufflehog#10-scan-a-local-git-repo

Clone the git repo. For example test keys repo.

```
git clone git@github.com:trufflesecurity/test_keys.git
```

Run trufflehog from the parent directory (outside the git repo).

```
trufflehog git file://test_keys --results=verified,unknown
```

To guard against malicious git configs in local scanning (see CVE-2025-41390), TruffleHog clones local git repositories to a temporary directory prior to scanning. This follows [Git's security best practices](https://git-scm.com/docs/git#_security). If you want to specify a custom path to clone the repository to (instead of tmp), you can use the --clone-path flag. If you'd like to skip the local cloning process and scan the repository directly (only do this for trusted repos), you can use the --trust-local-git-config flag.

## 11: Scan GCS buckets for only verified secrets

https://github.com/trufflesecurity/trufflehog#11-scan-gcs-buckets-for-only-verified-secrets

```
trufflehog gcs --project-id=<project-ID> --cloud-environment --results=verified
```

## 12: Scan a Docker image for only verified secrets

https://github.com/trufflesecurity/trufflehog#12-scan-a-docker-image-for-only-verified-secrets

Use the --image flag multiple times to scan multiple images.

```
# to scan from a remote registry
trufflehog docker --image trufflesecurity/secrets --results=verified

# to scan from the local docker daemon
trufflehog docker --image docker://new_image:tag --results=verified

# to scan from an image saved as a tarball
trufflehog docker --image file://path_to_image.tar --results=verified
```

## 13: Scan in CI

https://github.com/trufflesecurity/trufflehog#13-scan-in-ci

Set the --since-commit flag to your default branch that people merge into (ex: "main"). Set the --branch flag to your PR's branch name (ex: "feature-1"). Depending on the CI/CD platform you use, this value can be pulled in dynamically (ex: [CIRCLE_BRANCH in Circle CI](https://circleci.com/docs/variables/) and [TRAVIS_PULL_REQUEST_BRANCH in Travis CI](https://docs.travis-ci.com/user/environment-variables/)). If the repo is cloned and the target branch is already checked out during the CI/CD workflow, then --branch HEAD should be sufficient. The --fail flag will return an 183 error code if valid credentials are found.

```
trufflehog git file://. --since-commit main --branch feature-1 --results=verified,unknown --fail
```

## 14: Scan a Postman workspace

https://github.com/trufflesecurity/trufflehog#14-scan-a-postman-workspace

Use the --workspace-id, --collection-id, --environment flags multiple times to scan multiple targets.

```
trufflehog postman --token=<postman api token> --workspace-id=<workspace id>
```

## 15: Scan a Jenkins server

https://github.com/trufflesecurity/trufflehog#15-scan-a-jenkins-server

```
trufflehog jenkins --url https://jenkins.example.com --username admin --password admin
```

## 16: Scan an Elasticsearch server

https://github.com/trufflesecurity/trufflehog#16-scan-an-elasticsearch-server

### Scan a Local Cluster

https://github.com/trufflesecurity/trufflehog#scan-a-local-cluster

There are two ways to authenticate to a local cluster with TruffleHog: (1) username and password, (2) service token.

#### Connect to a local cluster with username and password

https://github.com/trufflesecurity/trufflehog#connect-to-a-local-cluster-with-username-and-password

```
trufflehog elasticsearch --nodes 192.168.14.3 192.168.14.4 --username truffle --password hog
```

#### Connect to a local cluster with a service token

https://github.com/trufflesecurity/trufflehog#connect-to-a-local-cluster-with-a-service-token

```
trufflehog elasticsearch --nodes 192.168.14.3 192.168.14.4 --service-token ‘AAEWVaWM...Rva2VuaSDZ’
```

### Scan an Elastic Cloud Cluster

https://github.com/trufflesecurity/trufflehog#scan-an-elastic-cloud-cluster

To scan a cluster on Elastic Cloud, you’ll need a Cloud ID and API key.

```
trufflehog elasticsearch \
  --cloud-id 'search-prod:dXMtY2Vx...YjM1ODNlOWFiZGRlNjI0NA==' \
  --api-key 'MlVtVjBZ...ZSYlduYnF1djh3NG5FQQ=='
```

## 17. Scan a GitHub Repository for Cross Fork Object References and Deleted Commits

https://github.com/trufflesecurity/trufflehog#17-scan-a-github-repository-for-cross-fork-object-references-and-deleted-commits

The following command will enumerate deleted and hidden commits on a GitHub repository and then scan them for secrets. This is an alpha release feature.

```
trufflehog github-experimental --repo https://github.com/<USER>/<REPO>.git --object-discovery
```

In addition to the normal TruffleHog output, the --object-discovery flag creates two files in a new $HOME/.trufflehog directory: valid_hidden.txt and invalid.txt. These are used to track state during commit enumeration, as well as to provide users with a complete list of all hidden and deleted commits (valid_hidden.txt). If you'd like to automatically remove these files after scanning, please add the flag --delete-cached-data.

Note: Enumerating all valid commits on a repository using this method takes between 20 minutes and a few hours, depending on the size of your repository. We added a progress bar to keep you updated on how long the enumeration will take. The actual secret scanning runs extremely fast.

For more information on Cross Fork Object References, please [read our blog post](https://trufflesecurity.com/blog/anyone-can-access-deleted-and-private-repo-data-github).

## 18. Scan Hugging Face

https://github.com/trufflesecurity/trufflehog#18-scan-hugging-face

### Scan a Hugging Face Model, Dataset, Space or Bucket

https://github.com/trufflesecurity/trufflehog#scan-a-hugging-face-model-dataset-space-or-bucket

```
trufflehog huggingface \
    --model <model_id> \
    --dataset <dataset_id> \
    --space <space_id> \
    --bucket <bucket_id>
```

### Scan all Models, Datasets, Spaces and Buckets belonging to a Hugging Face Organization or User

https://github.com/trufflesecurity/trufflehog#scan-all-models-datasets-spaces-and-buckets-belonging-to-a-hugging-face-organization-or-user

```
trufflehog huggingface --org <orgname> --user <username>
```

(Optionally) When scanning an organization or user, you can skip an entire class of resources with --skip-all-models, --skip-all-datasets, --skip-all-spaces, --skip-all-buckets OR a particular resource with --ignore-models <model_id>, --ignore-datasets <dataset_id>, --ignore-spaces <space_id>, --ignore-buckets <bucket_id>.

### Scan Discussion and PR Comments

https://github.com/trufflesecurity/trufflehog#scan-discussion-and-pr-comments

```
trufflehog huggingface --model <model_id> --include-discussions --include-prs
```

## 19. Scan stdin Input

https://github.com/trufflesecurity/trufflehog#19-scan-stdin-input

```
aws s3 cp s3://example/gzipped/data.gz - | gunzip -c | trufflehog stdin
```

# ❓ FAQ

https://github.com/trufflesecurity/trufflehog#question-faq

- All I see is 🐷🔑🐷 TruffleHog. Unearth your secrets. 🐷🔑🐷 and the program exits, what gives?

- That means no secrets were detected

- Why is the scan taking a long time when I scan a GitHub org

- Unauthenticated GitHub scans have rate limits. To improve your rate limits, include the --token flag with a personal access token

- It says a private key was verified, what does that mean?

- A verified result means TruffleHog confirmed the credential is valid by testing it against the service's API. For private keys, we've confirmed the key can be used live for SSH or SSL authentication. Check out our Driftwood blog post to learn more [Blog post](https://trufflesecurity.com/blog/driftwood-know-if-private-keys-are-sensitive/)

- Is there an easy way to ignore specific secrets?

- If the scanned source [supports line numbers](https://github.com/trufflesecurity/trufflehog/blob/d6375ba92172fd830abb4247cca15e3176448c5d/pkg/engine/engine.go#L358-L365), then you can add a trufflehog:ignore comment on the line containing the secret to ignore that secrets.

- Can I find secrets with trufflehog:ignore included?

- Pass --no-ignore-tag to report results even when their line carries a trufflehog:ignore comment. This is useful for reviewing previously accepted findings.

# 📰 What's new in v3?

https://github.com/trufflesecurity/trufflehog#newspaper-whats-new-in-v3

TruffleHog v3 is a complete rewrite in Go with many new powerful features.

- We've added over 700 credential detectors that support active verification against their respective APIs.
- We've also added native support for scanning GitHub, GitLab, Docker, filesystems, S3, GCS, Circle CI and Travis CI.
- Instantly verify private keys against millions of github users and billions of TLS certificates using our [Driftwood](https://trufflesecurity.com/blog/driftwood) technology.
- Scan binaries, documents, and other file formats
- Available as a GitHub Action and a pre-commit hook

## What is credential verification?

https://github.com/trufflesecurity/trufflehog#what-is-credential-verification

For every potential credential that is detected, we've painstakingly implemented programmatic verification against the API that we think it belongs to. Verification eliminates false positives and provides three result statuses:

- verified: Credential confirmed as valid and active by API testing
- unverified: Credential detected but not confirmed valid (may be invalid, expired, or verification disabled)
- unknown: Verification attempted but failed due to errors, such as a network or API failure

For example, the [AWS credential detector](https://github.com/trufflesecurity/trufflehog/blob/main/pkg/detectors/aws/access_keys/accesskey.go) performs a GetCallerIdentity API call against the AWS API to verify if an AWS credential is active.

# 📝 Usage

https://github.com/trufflesecurity/trufflehog#memo-usage

TruffleHog has a sub-command for each source of data that you may want to scan:

- git
- github
- gitlab
- huggingface
- docker
- s3
- filesystem (files and directories)
- syslog
- circleci
- travisci
- gcs (Google Cloud Storage)
- postman
- jenkins
- elasticsearch
- stdin
- multi-scan

Each subcommand can have options that you can see with the --help flag provided to the sub command:

```
$ trufflehog git --help
usage: TruffleHog [<flags>] <command> [<args> ...]

TruffleHog is a tool for finding credentials.

Flags:
  -h, --[no-]help                Show context-sensitive help (also try --help-long and --help-man).
      --log-level=0              Logging verbosity on a scale of 0 (info) to 5 (trace). Can be
                                 disabled with "-1".
      --[no-]profile             Enables profiling and sets a pprof and fgprof server on :18066.
  -j, --[no-]json                Output in JSON format.
      --[no-]json-legacy         Use the pre-v3.0 JSON format. Only works with git, gitlab,
                                 and github sources.
      --[no-]github-actions      Output in GitHub Actions format.
      --[no-]sarif               Output in SARIF format for upload to GitHub code scanning (e.g.
                                 via github/codeql-action/upload-sarif).
      --concurrency=12           Number of concurrent workers.
      --[no-]no-verification     Don't verify the results.
      --results=RESULTS          Specifies which type(s) of results to output: verified (confirmed
                                 valid by API), unknown (verification failed due to error),
                                 unverified (detected but not verified), filtered_unverified
                                 (unverified but would have been filtered out). Defaults to
                                 verified,unverified,unknown.
      --[no-]no-color            Disable colorized output
      --[no-]allow-verification-overlap
                                 Allow verification of similar credentials across detectors
      --[no-]filter-unverified   Only output first unverified result per chunk per detector if there
                                 are more than one results.
      --filter-entropy=FILTER-ENTROPY
                                 Filter unverified results with Shannon entropy. Start with 3.0.
      --config=CONFIG            Path to configuration file.
      --[no-]print-avg-detector-time
                                 Print the average time spent on each detector.
      --[no-]no-update           Don't check for updates.
      --[no-]fail                Exit with code 183 if results are found.
      --[no-]fail-on-scan-errors
                                 Exit with non-zero error code if an error occurs during the scan.
      --verifier=VERIFIER ...    Set custom verification endpoints.
      --[no-]custom-verifiers-only
                                 Only use custom verification endpoints.
      --detector-timeout=DETECTOR-TIMEOUT
                                 Maximum time to spend scanning chunks per detector (e.g., 30s).
      --archive-max-size=ARCHIVE-MAX-SIZE
                                 Maximum size of archive to scan. (Byte units eg. 512B, 2KB, 4MB)
      --archive-max-depth=ARCHIVE-MAX-DEPTH
                                 Maximum depth of archive to scan.
      --archive-timeout=ARCHIVE-TIMEOUT
                                 Maximum time to spend extracting an archive.
      --include-detectors="all"  Comma separated list of detector types to include. Protobuf name or
                                 IDs may be used, as well as ranges.
      --exclude-detectors=EXCLUDE-DETECTORS
                                 Comma separated list of detector types to exclude. Protobuf name
                                 or IDs may be used, as well as ranges. IDs defined here take
                                 precedence over the include list.
      --[no-]no-verification-cache
                                 Disable verification caching
      --[no-]force-skip-binaries
                                 Force skipping binaries.
      --[no-]force-skip-archives
                                 Force skipping archives.
      --[no-]skip-additional-refs
                                 Skip additional references.
      --user-agent-suffix=USER-AGENT-SUFFIX
                                 Suffix to add to User-Agent.
      --[no-]version             Show application version.

Commands:
help [<command>...]
    Show help.

git [<flags>] <uri>

    Find credentials in git repositories.

github [<flags>]
    Find credentials in GitHub repositories.

github-experimental --repo=REPO [<flags>]
    Run an experimental GitHub scan. Must specify at least one experimental sub-module to run:
    object-discovery.

gitlab --token=TOKEN [<flags>]
    Find credentials in GitLab repositories.

filesystem [<flags>] [<path>...]
    Find credentials in a filesystem.

s3 [<flags>]
    Find credentials in S3 buckets.

gcs [<flags>]
    Find credentials in GCS buckets.

syslog --format=FORMAT [<flags>]
    Scan syslog

circleci --token=TOKEN
    Scan CircleCI

docker [<flags>]
    Scan Docker Image

travisci --token=TOKEN
    Scan TravisCI

postman [<flags>]
    Scan Postman

elasticsearch [<flags>]
    Scan Elasticsearch

jenkins --url=URL [<flags>]
    Scan Jenkins

huggingface [<flags>]
    Find credentials in HuggingFace datasets, models, spaces and buckets.

stdin
    Find credentials from stdin.

multi-scan
    Find credentials in multiple sources defined in configuration.

json-enumerator [<path>...]
    Find credentials from a JSON enumerator input.

analyze
    Analyze API keys for fine-grained permissions information.

```

For example, to scan a git repository, start with

```
trufflehog git https://github.com/trufflesecurity/trufflehog.git

```

## Configuration

https://github.com/trufflesecurity/trufflehog#configuration

TruffleHog supports defining [custom regex detectors](https://github.com/trufflesecurity/trufflehog#custom-regex-detector-alpha) and multiple sources in a configuration file provided via the --config flag. The regex detectors can be used with any subcommand, while the sources defined in configuration are only for the multi-scan subcommand.

The configuration format for sources can be found on Truffle Security's [source configuration documentation page](https://trufflesecurity.com/docs/connect-sources).

Example GitHub source configuration and [options reference](https://docs.trufflesecurity.com/github#Fvm1I):

```
sources:
- connection:
    '@type': type.googleapis.com/sources.GitHub
    repositories:
    - https://github.com/trufflesecurity/test_keys.git
    unauthenticated: {}
  name: example config scan
  type: SOURCE_TYPE_GITHUB
  verify: true
```

You may define multiple connections under the sources key (see above), and TruffleHog will scan all of the sources concurrently.

To see debug logs for just one source, add debug: true to it. That source writes all of its debug logs, and every other source keeps the normal log level. This is useful when one source out of many is misbehaving:

```
sources:
- connection:
    '@type': type.googleapis.com/sources.GitHub
    repositories:
    - https://github.com/trufflesecurity/test_keys.git
    unauthenticated: {}
  name: example config scan
  type: SOURCE_TYPE_GITHUB
  debug: true
```

To turn on debug logs for every source, use --log-level=5 instead.

## S3

https://github.com/trufflesecurity/trufflehog#s3

The S3 source supports assuming IAM roles for scanning in addition to IAM users. This makes it easier for users to scan multiple AWS accounts without needing to rely on hardcoded credentials for each account.

The IAM identity that TruffleHog uses initially will need to have AssumeRole privileges as a principal in the [trust policy](https://aws.amazon.com/blogs/security/how-to-use-trust-policies-with-iam-roles/) of each IAM role to assume.

To scan a specific bucket using locally set credentials or instance metadata if on an EC2 instance:

```
trufflehog s3 --bucket=<bucket-name>
```

To scan a specific bucket using an assumed role:

```
trufflehog s3 --bucket=<bucket-name> --role-arn=<iam-role-arn>
```

Multiple roles can be passed as separate arguments. The following command will attempt to scan every bucket each role has permissions to list in the S3 API:

```
trufflehog s3 --role-arn=<iam-role-arn-1> --role-arn=<iam-role-arn-2>
```

### Narrowing a scan to specific objects

https://github.com/trufflesecurity/trufflehog#narrowing-a-scan-to-specific-objects

Large buckets often hold data that is never worth scanning, such as archives, media, and build artifacts. Two sets of flags keep those objects out of a scan. Both are matched against the object key before the object is downloaded, so a skipped object costs no GET request.

Scan only the objects under one or more key prefixes:

```
trufflehog s3 --bucket=<bucket-name> --include-prefix=infra/ --include-prefix=services/
```

Skip the objects under a key prefix:

```
trufflehog s3 --bucket=<bucket-name> --exclude-prefix=projects/archived/
```

Both prefix flags can be used together to scan a subtree while leaving one part of it out. An object that matches an exclude prefix is always skipped, even if it also matches an include prefix:

```
trufflehog s3 --bucket=<bucket-name> --include-prefix=src/ --exclude-prefix=src/vendor/
```

Prefixes are matched literally, not as globs, and they are not confined to a path boundary. --include-prefix=log matches both logs/app.txt and logs-archive/app.txt.

File extensions are filtered separately. Write them without a leading dot:

```
trufflehog s3 --bucket=<bucket-name> --exclude-extension=zip --exclude-extension=mp4
```

```
trufflehog s3 --bucket=<bucket-name> --include-extension=tf --include-extension=yaml
```

Unlike the prefix flags, --include-extension and --exclude-extension cannot be combined, because naming the extensions to scan already excludes every other one. Using both fails at startup.

Extension matching is case insensitive, so --exclude-extension=zip also skips BACKUP.ZIP. An object whose key has no extension at all, such as Makefile, matches no entry: it is skipped when --include-extension is set, and kept when only --exclude-extension is set.

Two things to watch when using --include-extension. A dotfile counts as all extension, so .env has the extension env and any include list that leaves out env will skip every .env file in the bucket. Since .env files are a common place for secrets to sit, add --include-extension=env unless you mean to skip them. And only the last extension counts, so backup.tar.gz has the extension gz: --exclude-extension=tar.gz matches nothing, while --exclude-extension=gz works.

These flags also apply to object keys only, not to files inside an archive. --exclude-extension=mp4 skips clip.mp4 sitting in the bucket, but not a clip.mp4 packed inside media.zip. Excluding zip skips the archive entirely.

TruffleHog asks S3 to list only the keys under the include prefixes, so a prefix that covers a small part of a large bucket is listed quickly. Exclude prefixes and extensions are applied after listing, so they cut the cost of downloading objects but not the cost of listing them.

Prefixes and extensions are applied together. An object has to pass both to be scanned, so the command below scans src/main.tf but skips both src/bundle.zip and docs/guide.tf:

```
trufflehog s3 --bucket=<bucket-name> --include-prefix=src/ --exclude-extension=zip
```

These flags narrow the objects within a bucket. To narrow which buckets are scanned, use --bucket or --ignore-bucket, which unlike the prefix flags cannot be used together.

Exit Codes:

- 0: No errors and no results were found.
- 1: An error was encountered. Sources may not have completed scans.
- 183: No errors were encountered, but results were found. Will only be returned if --fail flag is used.

##  TruffleHog Github Action

https://github.com/trufflesecurity/trufflehog#octocat-trufflehog-github-action

### General Usage

https://github.com/trufflesecurity/trufflehog#general-usage

```
on:
  push:
    branches:
      - main
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      with:
        fetch-depth: 0
    - name: Secret Scanning
      uses: trufflesecurity/trufflehog@main
      with:
        extra_args: --results=verified,unknown

```

In the example config above, we're scanning for live secrets in all PRs and Pushes to main. Only code changes in the referenced commits are scanned. If you'd like to scan an entire branch, please see the "Advanced Usage" section below.

### Shallow Cloning

https://github.com/trufflesecurity/trufflehog#shallow-cloning

If you're incorporating TruffleHog into a standalone workflow and aren't running any other CI/CD tooling alongside TruffleHog, then we recommend using [Shallow Cloning](https://git-scm.com/docs/git-clone#Documentation/git-clone.txt---depthltdepthgt) to speed up your workflow. Here's an example of how to do it:

```
...
      - shell: bash
        run: |
          if [ "${{ github.event_name }}" == "push" ]; then
            echo "depth=$(($(jq length <<< '${{ toJson(github.event.commits) }}') + 2))" >> $GITHUB_ENV
            echo "branch=${{ github.ref_name }}" >> $GITHUB_ENV
          fi
          if [ "${{ github.event_name }}" == "pull_request" ]; then
            echo "depth=$((${{ github.event.pull_request.commits }}+2))" >> $GITHUB_ENV
            echo "branch=${{ github.event.pull_request.head.ref }}" >> $GITHUB_ENV
          fi
      - uses: actions/checkout@v3
        with:
          ref: ${{env.branch}}
          fetch-depth: ${{env.depth}}
      - uses: trufflesecurity/trufflehog@main
        with:
          extra_args: --results=verified,unknown
...

```

Depending on the event type (push or PR), we calculate the number of commits present. Then we add 2, so that we can reference a base commit before our code changes. We pass that integer value to the fetch-depth flag in the checkout action in addition to the relevant branch. Now our checkout process should be much shorter.

### Canary detection

https://github.com/trufflesecurity/trufflehog#canary-detection

TruffleHog statically detects https://canarytokens.org/.

https://private-user-images.githubusercontent.com/52866392/306720861-74ace530-08c5-4eaf-a169-84a73e328f6f.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3OTM2NDY4MjUsIm5iZiI6MTc5MzY0NjUyNSwicGF0aCI6Ii81Mjg2NjM5Mi8zMDY3MjA4NjEtNzRhY2U1MzAtMDhjNS00ZWFmLWExNjktODRhNzNlMzI4ZjZmLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjExMDIlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYxMTAyVDE5MDg0NVomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTQ4ZGRmMmU4MTIxYmIxNWFlZTNhNzZmZThmYTk2ZDE3ODE2M2ZjNmRkZWE1NzFlMmQzMGZiMTllNWM4MzgzNDImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT1pbWFnZSUyRnBuZyJ9.MQ1PoEFr0ZRyx6XGGH4Yw4lhrrC83ANUB_XZ7eOsuiI

### Advanced Usage

https://github.com/trufflesecurity/trufflehog#advanced-usage

```
- name: TruffleHog
  uses: trufflesecurity/trufflehog@main
  with:
    # Repository path
    path:
    # Start scanning from here (usually main branch).
    base:
    # Scan commits until here (usually dev branch).
    head: # optional
    # Extra args to be passed to the trufflehog cli.
    extra_args: --log-level=2 --results=verified,unknown
    # Scan with a specific TruffleHog version (default: latest).
    version:
    # Docker image to pull. Override to use a registry mirror (default: ghcr.io/trufflesecurity/trufflehog).
    image:
```

If you'd like to specify specific base and head refs, you can use the base argument (--since-commit flag in TruffleHog CLI) and the head argument (--branch flag in the TruffleHog CLI). We only recommend using these arguments for very specific use cases, where the default behavior does not work.

To upload results to GitHub code scanning instead, run TruffleHog directly with --sarif and pass the output to github/codeql-action/upload-sarif:

```
- name: TruffleHog
  run: trufflehog filesystem . --sarif --no-verification > results.sarif
- name: Upload SARIF results
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: results.sarif
```

#### Advanced Usage: Scan entire branch

https://github.com/trufflesecurity/trufflehog#advanced-usage-scan-entire-branch

```
- name: scan-push
        uses: trufflesecurity/trufflehog@main
        with:
          base: ""
          head: ${{ github.ref_name }}
          extra_args: --results=verified,unknown

```

## TruffleHog GitLab CI

https://github.com/trufflesecurity/trufflehog#trufflehog-gitlab-ci

### Example Usage

https://github.com/trufflesecurity/trufflehog#example-usage

```
stages:
  - security

security-secrets:
  stage: security
  allow_failure: false
  image: alpine:latest
  variables:
    SCAN_PATH: "." # Set the relative path in the repo to scan
  before_script:
    - apk add --no-cache git curl jq
    - curl -sSfL https://raw.githubusercontent.com/trufflesecurity/trufflehog/main/scripts/install.sh | sh -s -- -b /usr/local/bin
  script:
    - trufflehog filesystem "$SCAN_PATH" --results=verified,unknown --fail --json | jq
  rules:
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
```

In the example pipeline above, we're scanning for live secrets in all repository directories and files. This job runs only when the pipeline source is a merge request event, meaning it's triggered when a new merge request is created.

## Pre-commit Hook

https://github.com/trufflesecurity/trufflehog#pre-commit-hook

TruffleHog can be used in a pre-commit hook to prevent credentials from leaking before they ever leave your computer.

See the [pre-commit hook documentation](https://github.com/trufflesecurity/trufflehog/blob/main/PreCommit.md) for more information.

## Custom Regex Detector (alpha)

https://github.com/trufflesecurity/trufflehog#custom-regex-detector-alpha

TruffleHog supports detection and verification of custom regular expressions. For detection, at least one regular expression and keyword is required. A keyword is a fixed literal string identifier that appears in or around the regex to be detected. To allow maximum flexibility for verification, a webhook is used containing the regular expression matches.

TruffleHog will send a JSON POST request containing the regex matches to a configured webhook endpoint. If the endpoint responds with a 200 OK response status code, the secret is considered verified. If verification fails due to network/API errors, the result is marked as unknown.

Custom Detectors support a few different filtering mechanisms: entropy, regex targeting the entire match, regex targeting the captured secret, and excluded word lists checked against the secret (captured group if present, entire match if capture group is not present). Note that if your custom detector has multiple regex set (in this example hogID, and hogToken), then the filters get applied to each regex. [Here](https://github.com/trufflesecurity/trufflehog/blob/main/examples/generic_with_filters.yml) is an example of a custom detector using these filters.

The verify section is optional — if you omit it, matches are still reported as unverified, no webhook required. This makes Custom Detectors useful for flagging generic hardcoded secrets (e.g. *.password=, *.secret=) in config files like .properties, .env, or .yaml that TruffleHog's built-in, verified detectors won't otherwise catch. [Here](https://github.com/trufflesecurity/trufflehog/blob/main/examples/generic_config_secrets.yml) is an example tuned for that use case.

NB: This feature is alpha and subject to change.

### Regex Detector Example

https://github.com/trufflesecurity/trufflehog#regex-detector-example

[Here](https://github.com/trufflesecurity/trufflehog/blob/main/pkg/custom_detectors/CUSTOM_DETECTORS.md) is how to setup a custom regex detector with verification server.

## Generic JWT Detection

https://github.com/trufflesecurity/trufflehog#generic-jwt-detection

TruffleHog supports detection and verification of a subset of generic JWTs it finds. Specifically, if a JWT uses public-key cryptography rather than HMAC and the public key can be obtained, TruffleHog can determine whether the JWT is live or not.

## 🔍 Analyze

https://github.com/trufflesecurity/trufflehog#mag-analyze

TruffleHog supports running a deeper analysis of a credential to view its permissions and the resources it has access to.

```
trufflehog analyze
```

# ❤️ Contributors

https://github.com/trufflesecurity/trufflehog#heart-contributors

This project exists thanks to all the people who contribute. [[Contribute](https://github.com/trufflesecurity/trufflehog/blob/main/CONTRIBUTING.md)].

https://github.com/trufflesecurity/trufflehog/graphs/contributors

# 💻 Contributing

https://github.com/trufflesecurity/trufflehog#computer-contributing

Contributions are very welcome! Please see our [contribution guidelines first](https://github.com/trufflesecurity/trufflehog/blob/main/CONTRIBUTING.md).

We no longer accept contributions to TruffleHog v2, but that code is available in the v2 branch.

## Adding new secret detectors

https://github.com/trufflesecurity/trufflehog#adding-new-secret-detectors

We have published some [documentation and tooling to get started on adding new secret detectors](https://github.com/trufflesecurity/trufflehog/blob/main/hack/docs/Adding_Detectors_external.md). Let's improve detection together!

# Use as a library

https://github.com/trufflesecurity/trufflehog#use-as-a-library

Currently, trufflehog is in heavy development and no guarantees can be made on the stability of the public APIs at this time.

# License Change

https://github.com/trufflesecurity/trufflehog#license-change

Since v3.0, TruffleHog is released under a AGPL 3 license, included in [LICENSE](https://github.com/trufflesecurity/trufflehog/blob/main/LICENSE). TruffleHog v3.0 uses none of the previous codebase, but care was taken to preserve backwards compatibility on the command line interface. The work previous to this release is still available licensed under GPL 2.0 in the history of this repository and the previous package releases and tags. A completed CLA is required for us to accept contributions going forward.
