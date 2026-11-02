---
url: "https://pytorch.org/blog/compromised-nightly-dependency/"
title: "Compromised PyTorch-nightly dependency chain between December 25th and December 30th, 2022. - PyTorch"
site: "PyTorch"
published: "2022-12-31T13:12:00+00:00"
captured: "2026-11-02T18:40:39Z"
via: "live"
sha256: "cedf6240329fd701bf7cf0706ff92346a83f78f8c989c2a89c13799499d324a3"
wayback: "https://web.archive.org/web/20261023002223/https://pytorch.org/blog/compromised-nightly-dependency/"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# Compromised PyTorch-nightly dependency chain between December 25th and December 30th, 2022. - PyTorch

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://pytorch.org/blog/compromised-nightly-dependency/>
> Snapshot: <https://web.archive.org/web/20261023002223/https://pytorch.org/blog/compromised-nightly-dependency/>

[Close Search](https://pytorch.org/blog/compromised-nightly-dependency/#)

https://pytorch.org

[search](https://pytorch.org/blog/compromised-nightly-dependency/#searchbox)

[Menu](https://pytorch.org/blog/compromised-nightly-dependency/#slide-out-widget-area)

[Blog](https://pytorch.org/blog/category/blog/)

# Compromised PyTorch-nightly dependency chain between December 25th and December 30th, 2022.

By [PyTorch Foundation](https://pytorch.org/blog/compromised-nightly-dependency/#)December 31, 2022November 14th, 2024[No Comments](https://pytorch.org/blog/compromised-nightly-dependency/#respond)

If you installed PyTorch-nightly on Linux via pip between December 25, 2022 and December 30, 2022, please uninstall it and torchtriton immediately, and use the latest nightly binaries (newer than Dec 30th 2022).

```
$ pip3 uninstall -y torch torchvision torchaudio torchtriton
$ pip3 cache purge

```

PyTorch-nightly Linux packages installed via pip during that time installed a dependency, torchtriton, which was compromised on the Python Package Index (PyPI) code repository and ran a malicious binary. This is what is known as a supply chain attack and directly affects dependencies for packages that are hosted on public package indices.

NOTE: Users of the PyTorch stable packages are not affected by this issue.

## How to check if your Python environment is affectedhttps://pytorch.org/blog/compromised-nightly-dependency/#how-to-check-if-your-python-environment-is-affected

The following command searches for the malicious binary in the torchtriton package (PYTHON_SITE_PACKAGES/triton/runtime/triton) and prints out whether your current Python environment is affected or not.

```
python3 -c "import pathlib;import importlib.util;s=importlib.util.find_spec('triton'); affected=any(x.name == 'triton' for x in (pathlib.Path(s.submodule_search_locations[0] if s is not None else '/' ) / 'runtime').glob('*'));print('You are {}affected'.format('' if affected else 'not '))"

```

The malicious binary is executed when the triton package is imported, which requires explicit code to do and is not PyTorch’s default behavior.

## The Backgroundhttps://pytorch.org/blog/compromised-nightly-dependency/#the-background

At around 4:40pm GMT on December 30 (Friday), we learned about a malicious dependency package (torchtriton) that was uploaded to the Python Package Index (PyPI) code repository with the same package name as the one we ship on the [PyTorch nightly package index](https://download.pytorch.org/whl/nightly). Since the [PyPI index takes precedence](https://github.com/pypa/pip/issues/8606), this malicious package was being installed instead of the version from our official repository. This design enables somebody to register a package by the same name as one that exists in a third party index, and pip will install their version by default.

This malicious package has the same name torchtriton but added in code that uploads sensitive data from the machine.

## What we knowhttps://pytorch.org/blog/compromised-nightly-dependency/#what-we-know

torchtriton on PyPI contains a malicious triton binary which is installed at PYTHON_SITE_PACKAGES/triton/runtime/triton. Its SHA256 hash is listed below.

SHA256(triton)= 2385b29489cd9e35f92c072780f903ae2e517ed422eae67246ae50a5cc738a0e

The binary’s main function does the following:

- Get system information

- nameservers from /etc/resolv.conf
- hostname from gethostname()
- current username from getlogin()
- current working directory name from getcwd()
- environment variables

- Read the following files

- /etc/hosts
- /etc/passwd
- The first 1,000 files in $HOME/*
- $HOME/.gitconfig
- $HOME/.ssh/*

- Upload all of this information, including file contents, via encrypted DNS queries to the domain *.h4ck[.]cfd, using the DNS server wheezy[.]io

The binary’s file upload functionality is limited to files less than 99,999 bytes in size. It also uploads only the first 1,000 files in $HOME (but all files < 99,999 bytes in the .ssh directory).

## Steps taken towards mitigationhttps://pytorch.org/blog/compromised-nightly-dependency/#steps-taken-towards-mitigation

- torchtriton has been removed as a dependency for our nightly packages and replaced with pytorch-triton ([pytorch/pytorch#91539](https://github.com/pytorch/pytorch/pull/91539)) and a dummy package registered on PyPI (so that this issue doesn’t repeat)
- All nightly packages that depend on torchtriton have been removed from our package indices at https://download.pytorch.org until further notice
- We have reached out to the PyPI security team to get proper ownership of the torchtriton package on PyPI and to delete the malicious version

### Docs

Access comprehensive developer documentation for PyTorch

[View Docs ›](https://pytorch.org/docs)

### Tutorials

Get in-depth tutorials for beginners and advanced developers

[View Tutorials ›](https://pytorch.org/tutorials)

### Resources

Find development resources and get your questions answered

[View Resources ›](https://pytorch.org/resources)

## Stay in touch for updates, event info, and the latest news

By submitting this form, I consent to receive marketing emails from the LF and its projects regarding their events, training, research, developments, and related announcements. I understand that I can unsubscribe at any time using the links in the footers of the emails I receive. [Privacy Policy](https://www.linuxfoundation.org/privacy/).

- [x-twitter](https://twitter.com/pytorch)
- [facebook](https://www.facebook.com/pytorch)
- [linkedin](https://www.linkedin.com/company/pytorch)
- [youtube](https://www.youtube.com/pytorch)
- [github](https://github.com/pytorch/pytorch)
- [slack](https://join.slack.com/t/pytorch/shared_invite/zt-2j2la612p-miUinTTaxXczKOJw48poHA)
- [discord](https://discord.com/invite/eNSRmh92XT)

© 2026 PyTorch. Copyright © The Linux Foundation®. All rights reserved. The Linux Foundation has registered trademarks and uses trademarks. For more information, including terms of use, privacy policy, and trademark usage, please see our [Policies](https://www.linuxfoundation.org/legal/policies) page. [Trademark Usage](https://www.linuxfoundation.org/trademark-usage). [Privacy Policy](http://www.linuxfoundation.org/privacy)

[Close Menu](https://pytorch.org/blog/compromised-nightly-dependency/#)

- [Learn](https://pytorch.org/blog/compromised-nightly-dependency/#)

- [Get Started](https://pytorch.org/get-started/locally/)
- [Tutorials](https://pytorch.org/tutorials/)
- [Learn the Basics](https://pytorch.org/tutorials/beginner/basics/intro.html)
- [PyTorch Recipes](https://docs.pytorch.org/tutorials/recipes_index.html)
- [Intro to PyTorch – YouTube Series](https://pytorch.org/tutorials/beginner/introyt.html)
- [Videos](https://pytorch.org/webinars/)
- [PyTorch Certification](https://pytorch.org/pytorch-certification/)

- [Community](https://pytorch.org/blog/compromised-nightly-dependency/#)

- [Landscape](https://landscape.pytorch.org/)
- [Join the Ecosystem](https://pytorch.org/join-ecosystem)
- [Community Hub](https://pytorch.org/community-hub/)
- [Forums](https://discuss.pytorch.org/)
- [Developer Resources](https://pytorch.org/resources/)
- [Events](https://pytorch.org/events/)
- [Working Groups](https://pytorch.org/working-groups/)
- [Meeting Calendar](https://pytorch.org/meeting-calendar/)
- [Contributor Award](https://pytorch.org/contributor-awards/)
- [Ambassadors](https://pytorch.org/programs/ambassadors/)

- [Projects](https://pytorch.org/projects/)

- [PyTorch](https://pytorch.org/projects/pytorch/)

- [Executorch](https://pytorch.org/projects/executorch/)

- [vLLM](https://pytorch.org/projects/vllm/)
- [DeepSpeed](https://pytorch.org/projects/deepspeed/)
- [Ray](https://pytorch.org/projects/ray/)
- [Helion](https://pytorch.org/projects/helion/)
- [Safetensors](https://pytorch.org/projects/safetensors/)
- [Host Your Project](https://pytorch.org/projects/host-your-project/)

- [Docs](https://pytorch.org/blog/compromised-nightly-dependency/#)

- [PyTorch](https://pytorch.org/docs/stable/index.html)
- [Domains](https://pytorch.org/domains/)

- [Blog & News](https://pytorch.org/blog/compromised-nightly-dependency/#)

- [Blog](https://pytorch.org/blog/)
- [Announcements](https://pytorch.org/announcements)
- [Case Studies](https://pytorch.org/case-studies/)
- [Newsletter](https://pytorch.org/newsletter/)

- [About](https://pytorch.org/blog/compromised-nightly-dependency/#)

- [PyTorch Foundation](https://pytorch.org/foundation/)
- [Members](https://pytorch.org/members)
- [Governing Board](https://pytorch.org/governing-board/)
- [Technical Advisory Council](https://pytorch.org/tac/)
- [Cloud Credit Program](https://pytorch.org/credits/)
- [Staff](https://pytorch.org/staff/)
- [Contact](https://pytorch.org/contact/)
- [Brand Guidelines](https://pytorch.org/brand-guidelines/)

- [JOIN](https://pytorch.org/join/)

https://pytorch.org/blog/compromised-nightly-dependency/#  https://pytorch.org/blog/compromised-nightly-dependency/#
