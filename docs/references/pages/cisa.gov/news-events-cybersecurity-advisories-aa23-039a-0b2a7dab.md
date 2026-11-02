---
url: "https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-039a"
title: "ESXiArgs Ransomware Virtual Machine Recovery Guidance | CISA"
site: "Cybersecurity and Infrastructure Security Agency CISA"
captured: "2026-11-02T18:53:43Z"
via: "live"
sha256: "7411054e5b0c4a1fc79f2b217974772b4b275e59d151fbe0fa643c7eb75f891c"
wayback: "https://web.archive.org/web/20260923202001/https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-039a"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# ESXiArgs Ransomware Virtual Machine Recovery Guidance | CISA

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-039a>
> Snapshot: <https://web.archive.org/web/20260923202001/https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-039a>

Cybersecurity Advisory

#  ESXiArgs Ransomware Virtual Machine Recovery Guidance

Release Date

February 08, 2023

Alert Code

AA23-039A

Related topics:

[Securing Networks](https://www.cisa.gov/topics/cyber-threats-and-advisories/securing-networks) , [Cyber Threats and Response](https://www.cisa.gov/topics/cyber-threats-and-response) , [Malware, Phishing, and Ransomware](https://www.cisa.gov/topics/cyber-threats-and-response/malware-phishing-and-ransomware)

### Summary

The Cybersecurity and Infrastructure Security Agency (CISA) and the Federal Bureau of Investigation (FBI) are releasing this joint Cybersecurity Advisory (CSA) in response to the ongoing ransomware campaign, known as “ESXiArgs.” Malicious actors may be exploiting known vulnerabilities in VMware ESXi servers that are likely running unpatched and out-of-service or out-of-date versions of VMware ESXi software to gain access and deploy ransomware. The ESXiArgs ransomware encrypts configuration files on ESXi servers, potentially rendering virtual machines (VMs) unusable.

CISA has released an ESXiArgs recovery script at [github.com/cisagov/ESXiArgs-Recover](https://github.com/cisagov/ESXiArgs-Recover). Organizations that have fallen victim to ESXiArgs ransomware can use this script to attempt to recover their files. This CSA provides guidance on how to use the script.
ESXiArgs actors have compromised over 3,800 servers globally. CISA and FBI encourage all organizations managing VMware ESXi servers to:

- Update servers to the latest version of VMware ESXi software,
- Harden ESXi hypervisors by disabling the Service Location Protocol (SLP) service, and
- Ensure the ESXi hypervisor is not exposed to the public internet.

If malicious actors have compromised your organization with ESXiArgs ransomware, CISA and FBI recommend following the script and guidance provided in this CSA to attempt to recover access to your files.

Download the PDF version of this report:

[ESXiArgs Ransomware Virtual Machine Recovery Guidance](https://www.cisa.gov/sites/default/files/2023-02/aa23-039a-esxiargs-ransomware-virtual-machine-recovery-guidance.pdf) (PDF, 711.08 KB )

Note: CISA and FBI will update this CSA as more information becomes available.

### Technical Details

Open-source reporting indicates that malicious actors are exploiting known vulnerabilities in VMware ESXi software to gain access to servers and deploy ESXiArgs ransomware. The actors are likely targeting end-of-life ESXi servers or ESXi servers that do not have the available ESXi software patches applied.[[1]](https://blogs.vmware.com/security/2023/02/83330.html)

ESXiArgs ransomware encrypts certain configuration files on ESXi servers, potentially rendering VMs unusable. Specifically, the ransomware encrypts configuration files associated with the VMs; it does not encrypt flat files. As a result, it is possible, in some cases, for victims to reconstruct the encrypted configuration files based on the unencrypted flat file. The recovery script documented below automates the process of recreating configuration files. The full list of file extensions encrypted by the malware is: vmdk, vmx, vmxf, vmsd, vmsn, vswp, vmss, nvram, vmem.

### Recovery Guidance

CISA and FBI do not encourage paying the ransom as payment does not guarantee victim files will be recovered. Furthermore, payment may also embolden adversaries to target additional organizations, encourage other criminal actors to engage in the distribution of ransomware, and/or fund illicit activities. Regardless of whether you or your organization have decided to pay the ransom, CISA and FBI urge you to promptly report ransomware incidents to a [local FBI Field Office](https://www.fbi.gov/contact-us/field-offices), or to CISA at [cisa.gov/report](https://edit-testint.cisa.gov/report).

CISA is providing these steps to enable organizations to attempt recovery of their VMs. CISA’s GitHub ESXiArgs recovery script, which also outlines these steps, is available at [github.com/cisagov/ESXiArgs-Recover](https://github.com/cisagov/ESXiArgs-Recover). CISA is aware that some organizations have reported success in recovering files without paying ransoms. CISA’s script is based on findings published by third-party researchers.[[2]](https://enes.dev/)

Any organization seeking to use CISA’s ESXiArgs recovery script should carefully review the script to determine if it is appropriate for their environment before deploying it. This script does not seek to delete the encrypted configuration files, but instead seeks to create new configuration files that enable access to the VMs. While CISA works to ensure that scripts like this one are safe and effective, this script is delivered without warranty, either implicit or explicit. Do not use this script without understanding how it may affect your system. CISA does not assume liability for damage caused by this script. Note: Organizations that run into problems with the script can create a GitHub issue at https://github.com/cisagov/ESXiArgs-Recover/issues; CISA will do our best to resolve concerns.

- Quarantine or take affected hosts offline to ensure that repeat infection does not occur.
- Download CISA’s recovery script and save it as /tmp/recover.sh.
For example, with wget: wget -O /tmp/recover.sh https://raw.githubusercontent.com/cisagov/ESXiArgs-Recover/main/recover.sh.
- Give the script execute permissions: chmod +x /tmp/recover.sh
- Navigate to the folder of a VM you would like to recover and run ls to view the files.

- Note: You may browse these folders by running ls /vmfs/volumes/datastore1. For instance, if the folder is called example, run cd /vmfs/volumes/datastore1/example.

- View files by running ls. Note the name of the VM (via naming convention: [name].vmdk).
- Run the recovery script with /tmp/recover.sh [name], where [name] is the name of the VM determined previously.

- If the VM is a thin format, run /tmp/recover.sh [name] thin.
- If successful, the recovery script will output that it has successfully run. If unsuccessful, it may not be possible for the recovery script to recover your VMs; consider engaging external incident response help.

- If the script succeeded, re-register the VM.

- If the ESXi web interface is inaccessible, remove the ransom note and restore access via the following steps. (Note: Taking the steps below moves the ransom note to the file ransom.html. Consider archiving this file for future incident review.)

- Run cd /usr/lib/vmware/hostd/docroot/ui/ && mv index.html ransom.html && mv index1.html index.html.
- Run cd /usr/lib/vmware/hostd/docroot && mv index.html ransom.html && rm index.html && mv index1.html index.html.
- Reboot the ESXi server (e.g., with the reboot command). After a few minutes, you should be able to navigate to the web interface.
- In the ESXi web interface, navigate to the Virtual Machines page.
- If the VM you restored already exists, right click on the VM and select Unregister (see figure 1).

Figure 1: Unregistering the virtual machine.

- Select Create / Register VM (see figure 2).
- Select Register an existing virtual machine (see figure 2).

Figure 2: Registering the virtual machine, selecting machine to register.

Click Select one or more virtual machines, a datastore or a directory to navigate to the folder of the VM you restored. Select the vmx file in the folder (see figure 3).

Figure 3: Registering the virtual machine, finalizing registration.

Select Next and Finish. You should now be able to use the VM as normal.

Figure 3: Registering the virtual machine, finalizing registration.

Select Next and Finish. You should now be able to use the VM as normal.

- Update servers to the latest software version, disable the Service Location Protocol (SLP) service, and ensure the ESXi hypervisor is not configured to be exposed to the public internet before putting systems back online.

#### Additional Incident Response

The above script only serves as a method to recover essential services. Although CISA and FBI have not seen any evidence that the actors have established persistence, we recommend organizations take the following additional incident response actions after applying the script:

- Review network logging to and from ESXi hosts and the guest VMs for unusual scanning activity.
- Review traffic from network segments occupied by the ESXi hosts and guests. Consider restricting non-essential traffic to and from these segments.

If you detect activity from the above, implement your incident response plan. CISA and FBI urge you to promptly report ransomware incidents to [a local FBI Field Office](https://www.fbi.gov/contact-us/field-offices), or to CISA at [cisa.gov/report](https://www.cisa.gov/report).

Organizations should also collect and review artifacts, such as running processes/services, unusual authentications, and recent network connections.

See the joint CSA from the cybersecurity authorities of Australia, Canada, New Zealand, the United Kingdom, and the United States on [Technical Approaches to Uncovering and Remediating Malicious Activity](https://us-cert.cisa.gov/ncas/alerts/aa20-245a) for additional guidance on hunting or investigating a network, and for common mistakes in incident handling. CISA also encourages government network administrators to see CISA’s Federal Government Cybersecurity Incident and Vulnerability Response Playbooks. Although tailored to federal civilian branch agencies, these playbooks provide operational procedures for planning and conducting cybersecurity incident and vulnerability response activities and detail steps for both incident and vulnerability response.

Additional resources for recovering .vmdk files can be found on a third-party researcher’s website.[[2]](https://enes.dev/)

### Mitigations

Note: These mitigations align with the cross-sector Cybersecurity Performance Goals (CPGs) developed by CISA and the National Institute of Standards and Technology (NIST). The CPGs provide a minimum set of practices and protections that CISA and NIST recommend all organizations implement. CISA and NIST based the CPGs on existing cybersecurity frameworks and guidance to protect against the most common and impactful threats, tactics, techniques, and procedures. For more information on the CPGs, including additional recommended baseline protections, see [cisa.gov/cpg](https://www.cisa.gov/cpg).

CISA and FBI recommend all organizations:

- Temporarily remove connectivity for the associated ESXi server(s).

- Upgrade your ESXi servers to the latest version of VMware ESXi software [[CPG 5.1]](https://www.cisa.gov/sites/default/files/publications/2022_00092_CISA_CPG_Report_508c.pdf). ESXi releases are cumulative, and the latest builds are documented in VMware’s article, Build numbers and versions of VMware ESXi/ESX.
- Harden ESXi hypervisors by disabling the Service Location Protocol (SLP) service, which ESXiArgs may leverage. For more information on executing workarounds, see VMware’s guidance [How to Disable/Enable the SLP Service on VMware ESXi](https://kb.vmware.com/s/article/76372).
- Ensure your ESXi hypervisor is not configured to be exposed to the public internet.

In addition, CISA and FBI recommend organizations apply the following recommendations to prepare for, mitigate/prevent, and respond to ransomware incidents.

### Preparing for Ransomware

- Maintain offline backups of data, and regularly test backup and restoration [[CPG 7.3]](https://www.cisa.gov/sites/default/files/publications/2022_00092_CISA_CPG_Report_508c.pdf). These practices safeguard an organization’s continuity of operations or at least minimize potential downtime from a ransomware incident and protect against data losses.
- Ensure all backup data is encrypted, immutable (i.e., cannot be altered or deleted), and covers the entire organization’s data infrastructure.
- Create, maintain, and exercise a basic cyber incident response plan and associated communications plan that includes response procedures for a ransomware incident [[CPG 7.1, 7.2]](https://www.cisa.gov/sites/default/files/publications/2022_00092_CISA_CPG_Report_508c.pdf).

###  Mitigating and Preventing Ransomware

- Restrict Server Message Block (SMB) Protocol within the network to only access servers that are necessary and remove or disable outdated versions of SMB (i.e., SMB version 1). Threat actors use SMB to propagate malware across organizations.
- [Require phishing-resistant MFA](https://www.cisa.gov/sites/default/files/publications/fact-sheet-implementing-phishing-resistant-mfa-508c.pdf) for as many services as possible [[CPG 1.3]](https://www.cisa.gov/sites/default/files/publications/2022_00092_CISA_CPG_Report_508c.pdf)—particularly for webmail, VPNs, accounts that access critical systems, and privileged accounts that manage backups.
- Review the security posture of third-party vendors and those interconnected with your organization. Ensure all connections between third-party vendors and outside software or hardware are monitored and reviewed for suspicious activity.
- Implement allow-listing policies for applications and remote access that only allow systems to execute known and permitted programs.
- Open document readers in protected viewing modes to help prevent active content from running.
- Implement user training program and phishing exercises to raise awareness among users about the risks of visiting suspicious websites, clicking on suspicious links, and opening suspicious attachments. Reinforce the appropriate user response to phishing and spearphishing emails.
- Use strong passwords [[CPG 1.4]](https://www.cisa.gov/sites/default/files/publications/2022_00092_CISA_CPG_Report_508c.pdf) and avoid reusing passwords for multiple accounts. See CISA Tip [Choosing and Protecting Passwords](https://www.cisa.gov/tips/st04-002) and the NIST’s [Special Publication 800-63B: Digital Identity Guidelines](https://csrc.nist.gov/publications/detail/sp/800-63b/final) for more information.
- Require administrator credentials to install software [[CPG 1.5]](https://www.cisa.gov/sites/default/files/publications/2022_00092_CISA_CPG_Report_508c.pdf).
- Audit user accounts with administrative or elevated privileges and configure access controls with least privilege in mind [[CPG 1.5]](https://www.cisa.gov/sites/default/files/publications/2022_00092_CISA_CPG_Report_508c.pdf).
- Install and regularly update antivirus and antimalware software on all hosts.
- Consider adding an email banner to messages coming from outside your organizations.
- Disable hyperlinks in received emails.
- Consider participating in CISA’s no-cost [Automated Indicator Sharing (AIS) program](https://www.cisa.gov/ais) to receive real-time exchange of machine-readable cyber threat indicators and defensive measures.

### Responding to Ransomware Incidents

If a ransomware incident occurs at your organization:

- Follow your organization’s Ransomware Response Checklist (see Preparing for Ransomware section).
- Scan backups. If possible, scan backup data with an antivirus program to check that it is free of malware. This should be performed using an isolated, trusted system to avoid exposing backups to potential compromise.
- Follow the notification requirements as outlined in your cyber incident response plan.
- Report incidents to CISA at [cisa.gov/report](https://www.cisa.gov/report), FBI at a [local FBI Field Office](https://www.fbi.gov/contact-us/field-offices), or the U.S. Secret Service (USSS) at a [USSS Field Office](http://www.secretservice.gov/contact/field-offices/).
- Apply incident response best practices found in the joint Cybersecurity Advisory, [Technical Approaches to Uncovering and Remediating Malicious Activity](https://us-cert.cisa.gov/ncas/alerts/aa20-245a), developed by CISA and the cybersecurity authorities of Australia, Canada, New Zealand, and the United Kingdom.

Note: CISA and FBI strongly discourage paying ransoms as doing so does not guarantee files and records will be recovered. Furthermore, payment may also embolden adversaries to target additional organizations, encourage other criminal actors to engage in the distribution of ransomware, and/or fund illicit activities.

### Resources

See [Stopransomware.gov](https://www.cisa.gov/stopransomware), a whole-of-government approach, for ransomware resources and alerts.

### Acknowledgements

CISA and FBI would like to thank VMware for their contributions to this CSA.

### References

[VMware Security Response Center (vSRC) Response to 'ESXiArgs' Ransomware Attack…](https://blogs.vmware.com/security/2023/02/83330.html)

[Enes Sonmez and Ahmet Aykac, YoreGroup Tech Team: decrypt your crypted files in…](https://enes.dev/)

### Revisions

February, 2023: Initial Version

This product is provided subject to this [Notification](https://www.cisa.gov/notification) and this [Privacy & Use](https://www.cisa.gov/privacy-policy) policy.

### Tags

Topics: [Cyber Threats and Response](https://www.cisa.gov/topics/cyber-threats-and-response), [Information Sharing](https://www.cisa.gov/topics/cyber-threats-and-response/information-sharing), [Malware, Phishing, and Ransomware](https://www.cisa.gov/topics/cyber-threats-and-response/malware-phishing-and-ransomware), [Securing Networks](https://www.cisa.gov/topics/cyber-threats-and-advisories/securing-networks)

Co-Sealers and Partners: Federal Bureau of Investigation

## Please share your thoughts

We recently updated our anonymous [product survey](https://cisasurvey.gov1.qualtrics.com/jfe/form/SV_9n4TtB8uttUPaM6?product=https://www.cisa.gov/news-events/cybersecurity-advisories/aa23-039a); we welcome your feedback.

## Related Advisories

Oct 20, 2026

Cybersecurity Advisory | AA26-293A

###  [#StopRansomware: SafePay Ransomware](https://www.cisa.gov/news-events/cybersecurity-advisories/aa26-293a)

Oct 08, 2026

Cybersecurity Advisory | AA26-281A

###  [Chinese Government-linked Cyber Threat Actors Combine Automated and Hands-on Hacking Tools to Steal Sensitive Data](https://www.cisa.gov/news-events/cybersecurity-advisories/aa26-281a)

Sep 08, 2026

Cybersecurity Advisory | AA26-251A

###  [China-Based Artificial Intelligence Companies Conducting Industrial-Scale Distillation Campaigns Against U.S. AI Companies](https://www.cisa.gov/news-events/cybersecurity-advisories/aa26-251a)

Aug 25, 2026

Cybersecurity Advisory | AA26-237A

###  [A Tale of Two SOCs: Insights From Two Red Team Assessments](https://www.cisa.gov/news-events/cybersecurity-advisories/aa26-237a)
