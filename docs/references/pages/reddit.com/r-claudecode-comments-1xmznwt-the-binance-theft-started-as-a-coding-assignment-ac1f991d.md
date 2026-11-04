---
url: "https://www.reddit.com/r/ClaudeCode/comments/1xmznwt/the_binance_theft_started_as_a_coding_assignment/"
title: "The Binance theft started as a coding assignment. I’ve been sent the same kind of repo all year. All were exploits."
site: "Reddit, r/ClaudeCode"
published: "2026-11-03T17:44:30Z"
captured: "2026-11-03T20:36:39Z"
via: "reddit-api"
sha256: "b9ff677cc763a9cf9b72687127a18fb8ad2890a1271ec6fc4318c60092e4514e"
cited_in:
  - "docs/research/interview-repo-lures.md"
  - "docs/threats/01-phishing-social-engineering.md"
  - "docs/threats/02-supply-chain-packages.md"
---

# The Binance theft started as a coding assignment. I’ve been sent the same kind of repo all year. All were exploits.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.reddit.com/r/ClaudeCode/comments/1xmznwt/the_binance_theft_started_as_a_coding_assignment/>

Posted by u/UBUNTU-Buddha in r/ClaudeCode on 2026-11-03T17:44:30Z.

Binance said this week that the October 29 theft, about $6.3 billion, began months earlier with a fake job. Reuters, via NBC: in June a wallet engineer installed a tampered coding tool. Binance caught that one, replaced the machine, and rotated credentials. The attacker already knew who else was on the wallet team.



In September a second engineer was recruited through a real job site. There was a video interview with someone playing HR, then a second interview with the supposed CEO of a company that does not exist. At the end he was told to download a “coding assignment” onto his work computer. Binance says the repo ran during setup, swapped in a poisoned copy of a normal developer tool, and the endpoint product did not see it. On October 20 that machine, plus an internal AI tool, was used to reach two other wallet engineers. On October 29 stolen credentials deployed a container and a script, and the wallets moved. The FBI has blamed a North Korean group. Pyongyang has denied it. Binance says customer funds are covered.



That middle step is not exotic. It is the interview-repo lure.



Since January I have published the ones that were sent to me, all of them caught before they ran. They are writeups, not exploits, and the indicators are defanged: [github.com/Ubuntu-Buddha/security-advisories](http://github.com/Ubuntu-Buddha/security-advisories)



2026-01-27, W3GLFun. A Web3 “technical test.” The error handler fetched code from a paste site and ran it when the app started, with the full Node runtime. I did clone that one. npm run dev was typed in the wrong folder, so nothing of theirs executed. GitHub Trust & Safety took the account down the same day. The hiring platform first said there was no violation, then wrote back that the user may have shared malware. A first “all clear” is not a final answer. The same paste host later showed up in malicious npm packages. I called the pattern consistent with the public “Contagious Interview” reporting. I did not claim who operated it.



2026-01-29, Optu. Same shape, payload URL hidden in an environment variable, fired on server start. Found before it ran. The account is gone.



2026-01-30, two repos. .vscode/tasks.json set to run when you open the folder. You do not install anything. Opening the project in VS Code or Cursor is the run. Found before it ran. Both accounts are gone.



2026-01, a careers page. A fake “GAPI Update” executable, hosted like a normal application page. Found before anyone ran it.



2026-10-15, a GitLab take-home. A hex payload stuffed in a file that pretended to be a font, loaded when the project was set up. I reviewed that one remotely and did not execute it. Reported to GitLab and Upwork the same day. Outcome was still pending when I published.



These are not Binance’s samples. I am not saying I found their intrusion, or that one group wrote all of them. I am saying the instruction was the same: here is a company, here is a CEO/recruiter, here is the repo, please run it on the machine you actually work on.



In fact, I have never been sent an interview repo that didn't contain an exploit. 100% were compromised or predatory.



Two rules came out of that, and they are the whole protocol.



Do not download it if you can read it in the browser. Raw files on GitHub, or the file view while you are logged in. Look at .vscode/tasks.json, package.json scripts, and anything that fetches text and then runs it, before git clone, before npm install, and before “Open in Desktop.” Opening the folder is enough for the VS Code one. A first “all clear” from the platform is not a reason to run it anyway.



If you must run it, run it where a compromise is boring. A disposable VM or container, no wallets, no SSH keys, no cloud tokens, network off. Your daily machine is the wrong place. So is a work laptop that can deploy anything.



The other half of the Binance story is the internal AI tool. An agent that can run commands, and that also reads the repo, the README, or the email, will follow instructions written by the person who sent the repo. Filters that try to spot the bad sentence miss some.



That is why I built StrangerDanger, a Claude Code plugin. The rule is the title of the README: readers don’t act, actors don’t read.



Web pages, documents, and strangers’ code are read only by subagents that cannot run a shell and cannot write. The agents that can run commands are kept off the web and out of the folders you mark untrusted. What they get back is a report, and that report is still data, not an order. A second hook blocks the common “download this and pipe it to a shell” forms. Setup turns Claude Code’s sandbox on. It is MIT, pinned at v0.2.0, and marked experimental.



It is not a magic shield. Every agent still runs as you. The hooks constrain tool calls. They do not take your authority away. The launch gate is a tripwire, and v0.2.0 still has known gaps. Nothing in the plugin makes a stranger’s repo safe to run on your machine. Use the throwaway VM anyway. I also published the holes I found in my own v0.1.0 gate (GHSA-w42w-fc6x-88wr), fixed in v0.1.1. Read “What this does not do” before you rely on it.



Binance had detection, a replaced laptop, and rotated credentials in June, and still lost the wallets in October, because one engineer ran an interview repo on a computer that could reach production. The repo is the exploit. The interview is just how they get you to open it.



\------



What safety/security protocols do you practice now while building with agents?

_The post body only; comments are not archived. Read through Reddit's API because the site serves an empty, script-rendered page to plain HTTP clients._
