---
url: "https://falco.org/docs/reference/rules/default-rules/"
title: "Default Rules"
site: "Falco"
captured: "2026-11-02T18:31:13Z"
via: "live"
sha256: "6652382257bc8c1d974cd07fd49fa6f9a821461c98fe1ee6eac6a61598d3fc36"
cited_in:
  - "docs/threats/05-malware-families-and-detection.md"
---

# Default Rules

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://falco.org/docs/reference/rules/default-rules/>

# Default Rules

List of default rules for Falco

Falco provides default rules maintained by the maintainers with contributions from the community. More information on this [page](https://falco.org/docs/rules/default-custom/).

This list presents the default rules based on syscall events.

By default, only the stable rules are loaded by Falco, you can install the sandbox or incubating rules by referencing them in the Helm chart:

```
helm install falco falcosecurity/falco \
--set "falcoctl.config.artifact.install.refs={falco-rules:5,falco-incubating-rules:6,falco-sandbox-rules:6}" \
--set "falcoctl.config.artifact.follow.refs={falco-rules:5,falco-incubating-rules:6,falco-sandbox-rules:6}" \
--set "falco.rules_files={/etc/falco/falco_rules.yaml,/etc/falco/falco-incubating_rules.yaml,/etc/falco/falco-sandbox_rules.yaml,/etc/falco/falco_rules.local.yaml,/etc/falco/rules.d}"

```

Where the option falcoctl.config.artifact.install.refs governs which rules are downloaded at startup, falcoctl.config.artifact.follow.refs identifies which rules are automatically updated and falco.rules_files indicates which rules are loaded by the engine.

You can find all the rules in their official [repository](https://github.com/falcosecurity/rules/blob/main/rules/).

Description:

```
Detect any new SSH connection on port 22 to a host other than those in an allowed list of hosts.  This rule absolutely requires profiling your environment beforehand. Network-based rules are extremely  crucial in any security program, as they can often provide the only definitive evidence. However,  effectively operationalizing them can be challenging due to the potential for noise.

```

Condition:

```
inbound_outbound  and ssh_port  and not allowed_ssh_hosts

```

Output:

```
Disallowed SSH Connection (connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty %container.info)
```

Maturity: deprecated

Priority: NOTICE

Tags: maturity_deprecated host container network mitre_lateral_movement T1021.004

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Disallowed_SSH_Connection

Description:

```
Detect any outbound connection to a destination outside of an allowed set of ips, networks, or domain names.  This rule absolutely requires profiling your environment beforehand. Network-based rules are extremely crucial  in any security program, as they can often provide the only definitive evidence. However, effectively operationalizing  them can be challenging due to the potential for noise.

```

Condition:

```
outbound  and not ((fd.sip in (allowed_outbound_destination_ipaddrs)) or
         (fd.snet in (allowed_outbound_destination_networks)) or
         (fd.sip.name in (allowed_outbound_destination_domains)))

```

Output:

```
Disallowed outbound connection destination (connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty %container.info)
```

Maturity: deprecated

Priority: NOTICE

Tags: maturity_deprecated host container network mitre_command_and_control TA0011

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Unexpected_outbound_connection_destination

Description:

```
Detect traffic to an unauthorized server process and port within pre-defined containers.  This rule absolutely requires profiling your environment beforehand and also necessitates adjusting the list of containers  to which this rule will be applied. The current expression logic will never evaluate to true unless the list is populated.  Network-based rules are extremely crucial in any security program, as they can often provide the only definitive evidence.  However, effectively operationalizing them can be challenging due to the potential for noise. Notably, this rule is challenging  to operationalize.

```

Condition:

```
inbound_outbound  and container  and container.image.repository in (allowed_image)  and not proc.name in (authorized_server_binary)  and not fd.sport in (authorized_server_port)

```

Output:

```
Network connection outside authorized port and binary (connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty %container.info)
```

Maturity: deprecated

Priority: WARNING

Tags: maturity_deprecated container network mitre_discovery TA0011 NIST_800-53_CM-7

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Outbound_or_Inbound_Traffic_not_to_Authorized_Server_Process_and_Port

Description:

```
Detect outbound connections to command and control servers using a list of IP addresses and fully qualified domain names (FQDNs). This rule absolutely requires profiling your environment beforehand and also necessitates adjusting the template lists. The current  expression logic will never evaluate to true unless the lists are populated. Network-based rules are extremely crucial in any  security program, as they can often provide the only definitive evidence. However, effectively operationalizing them can be challenging  due to the potential for noise. Notably, this rule is challenging to operationalize.

```

Condition:

```
outbound  and ((fd.sip in (c2_server_ip_list)) or
     (fd.sip.name in (c2_server_fqdn_list)))

```

Output:

```
Outbound connection to C2 server (c2_domain=%fd.sip.name c2_addr=%fd.sip connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty %container.info)
```

Maturity: deprecated

Priority: WARNING

Tags: maturity_deprecated host container network mitre_command_and_control TA0011

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Outbound_Connection_to_C2_Servers

Description:

```
Detect attempts to modify shell configuration files, primarily aimed at establishing persistence by automatically inserting commands into scripts executed by shells. The upstream rule excludes shell processes because they often create unnecessary noise. However, this might lead to missed detections. To customize the rule for your situation, you can fine-tune it using enhanced profiling. For example, you might want to only consider interactive shell processes (where proc.tty != 0).

```

Condition:

```
open_write and (fd.filename in (shell_config_filenames) or
     fd.name in (shell_config_files) or
     fd.directory in (shell_config_directories))
and not proc.name in (shell_binaries) and not exe_running_docker_save and not user_known_shell_config_modifiers

```

Output:

```
A shell configuration file has been modified | file=%fd.name pcmdline=%proc.pcmdline evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: WARNING

Tags: maturity_incubating host container filesystem mitre_persistence T1546.004

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Modify_Shell_Configuration_File

Description:

```
Detect scheduled cron jobs; this is a highly generic detection and certainly needs adjustments and profiling in your environment before operationalization. Simultaneously, exploiting the functionality of cron jobs is among one of the oldest TTPs used by adversaries.

```

Condition:

```
((open_write and fd.name startswith /etc/cron) or
 (spawned_process and proc.name = "crontab"))
and not user_known_cron_jobs

```

Output:

```
Cron jobs were scheduled to run | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host container filesystem mitre_execution T1053.003

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Schedule_Cron_Jobs

Description:

```
This rule identifies attempts to read files within ssh directories using programs that are not related to ssh. It's a simple and versatile detection method that works well alongside more specific rules focused on sensitive file access. You have a couple of options for using this rule effectively: you can adjust the specialized rules to cover all the important scenarios and ensure precedence in rule smatching for those, or you can analyze the combined view of ssh-related file access across various rules on your downstream computing platform. Just like with other rules, you can narrow down monitoring to specific processes, or you can limit it to interactive access only.

```

Condition:

```
(open_read or open_directory)
 and (user_ssh_directory or fd.name startswith /root/.ssh)
 and not user_known_read_ssh_information_activities
 and not proc.name in (ssh_binaries)

```

Output:

```
ssh-related file/directory read by non-ssh program | file=%fd.name pcmdline=%proc.pcmdline evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: ERROR

Tags: maturity_incubating host container filesystem mitre_collection T1005

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Read_ssh_information

Description:

```
A program related to the database server creates an unexpected child process (other than itself). This is not supposed to happen and often follows SQL injection attacks. This behavioral detection could indicate potential unauthorized data extraction or tampering with the database.

```

Condition:

```
spawned_process and proc.pname in (db_server_binaries) and not proc.name in (db_server_binaries) and not postgres_running_wal_e and not user_known_db_spawned_processes

```

Output:

```
Database-related program spawned process other than itself | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host container process database mitre_execution T1190

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#DB_program_spawned_process

Description:

```
An attempt to alter the namespace of a process (often performed while creating a container) through the setns syscall. Conversely, the same syscall setns is triggered when an unauthorized attempt is made to break out from the container to the host, for example, when using commands like `nsenter --target 1` and similar ones. Recommending to profile your environment and refine this rule for effective operationalization.

```

Condition:

```
evt.type=setns and proc_name_exists and not (container.id=host and proc.name in (docker_binaries, k8s_binaries, lxd_binaries, nsenter)) and not proc.name in (sysdigcloud_binaries, sysdig, calico, oci-umount, cilium-cni, network_plugin_binaries) and not proc.name in (user_known_change_thread_namespace_binaries) and not proc.name startswith "runc" and not proc.cmdline startswith "containerd" and not proc.pname in (sysdigcloud_binaries, hyperkube, kubelet, protokube, dockerd, tini, aws) and not java_running_sdjagent and not kubelet_running_loopback and not rancher_agent and not rancher_network_manager and not calico_node and not weaveworks_scope and not user_known_change_thread_namespace_activities

```

Output:

```
Namespace change (setns) by unexpected program | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host container process mitre_privilege_escalation T1611

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Change_thread_namespace

Description:

```
Unprivileged users in containers may not have CAP_SYS_ADMIN or other elevated privileges. However, they can use the unshare system call with CLONE_NEWNS or CLONE_NEWUSER to create or clone a namespace or user with the necessary privileges to conduct further attacks. It is best practice to block the unshare system call via seccomp if it is not needed. Misuse of unshare can be related to misconfigured Kubernetes clusters, for example.

```

Condition:

```
evt.type=unshare and container and not thread.cap_permitted contains CAP_SYS_ADMIN

```

Output:

```
Change namespace privileges via unshare | res=%evt.res evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating container mitre_privilege_escalation T1611

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Change_namespace_privileges_via_unshare

Description:

```
Detect the initial process initiation within a privileged container, with exemptions for known and trusted images. This rule primarily serves as an excellent auditing mechanism since highly privileged containers, when compromised, can result in significant harm. For instance, if another rule triggers within such a privileged container, it could be seen as more suspicious, prompting a closer inspection.

```

Condition:

```
container_started and container.privileged=true and not falco_privileged_containers and not user_privileged_containers and not redhat_image

```

Output:

```
Privileged container started | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: INFO

Tags: maturity_incubating container cis mitre_execution T1610 PCI_DSS_10.2.5

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Launch_Privileged_Container

Description:

```
Identify containers that start with a powerful set of capabilities, with exceptions for recognized trusted images. Similar to the "Launch Privileged Container" rule, this functions as a robust auditing rule. Compromised highly privileged containers can lead to substantial harm. For instance, if another rule is triggered within such a container, it might raise suspicion, prompting closer scrutiny.

```

Condition:

```
container_started and excessively_capable_container and not falco_privileged_containers and not user_privileged_containers

```

Output:

```
Excessively capable container started | cap_permitted=%thread.cap_permitted evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: INFO

Tags: maturity_incubating container cis mitre_execution T1610

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Launch_Excessively_Capable_Container

Description:

```
Detect any unexpected network activity performed by system binaries that typically shouldn't perform network activity, including coreutils binaries (like sleep, mkdir, who, date, and others) or user management binaries (such as login, systemd, usermod, deluser, adduser, chpasswd, and others). This serves as a valuable baseline detection for network-related activities.

```

Condition:

```
inbound_outbound and fd.sockfamily = ip and (system_procs or proc.name in (shell_binaries)) and not proc.name in (known_system_procs_network_activity_binaries) and not login_doing_dns_lookup and not user_expected_system_procs_network_activity_conditions

```

Output:

```
Known system binary sent/received network traffic | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host network process mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#System_procs_network_activity

Description:

```
Detect curl or wget usage with HTTP_PROXY environment variable. Attackers can manipulate the HTTP_PROXY variable's value to redirect application's internal HTTP requests. This could expose sensitive information like authentication keys and private data.

```

Condition:

```
spawned_process and http_proxy_procs and proc.env icontains HTTP_PROXY and not allowed_ssh_proxy_env

```

Output:

```
Curl or wget run with disallowed HTTP_PROXY environment variable | env=%proc.env evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host container users mitre_execution T1204

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Program_run_with_disallowed_http_proxy_env

Description:

```
Detecting UDP traffic on ports other than 53 (DNS) or other commonly used ports. Misusing UDP is a known TTP among attackers. Monitoring unusual network activity is highly valuable but often generates significant noise, as is the case with this detection.

```

Condition:

```
inbound_outbound and fd.l4proto=udp and not expected_udp_traffic

```

Output:

```
Unexpected UDP Traffic Seen | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host container network mitre_exfiltration TA0011

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Unexpected_UDP_Traffic

Description:

```
Detect attempts to change users through the use of setuid, with exceptions for sudo/su. The users "root" and "nobody" using setuid on themselves are also excluded, as setuid calls in these cases typically involve reducing privileges. By setting the setuid bit, an attacker could execute code in a different user's context, potentially with higher privileges. One drawback is the potential for noise, as many applications legitimately use this approach.

```

Condition:

```
evt.type=setuid and (known_user_in_container or not container) and not (user.name=root or user.uid=0) and not somebody_becoming_themselves and not proc.name in (known_setuid_binaries, userexec_binaries, mail_binaries, docker_binaries,
                      nomachine_binaries)
and not proc.name startswith "runc:" and not java_running_sdjagent and not nrpe_becoming_nagios and not user_known_non_sudo_setuid_conditions

```

Output:

```
Unexpected setuid call by non-sudo, non-root program | arg_uid=%evt.arg.uid evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host container users mitre_privilege_escalation T1548.001

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Non_sudo_setuid

Description:

```
Detect activity by any programs that can manage users, passwords, or permissions (such as login, systemd, usermod, deluser, adduser, chpasswd, and others). sudo and su are excluded. Activity in containers is also excluded -- some containers create custom users on top of a base linux distribution at startup. Some innocuous command lines that don't actually change anything are excluded. You might want to consider applying this rule to container actions as well.

```

Condition:

```
spawned_process and not container and proc.name in (user_mgmt_binaries) and not proc.name in (su, sudo, lastlog, nologin, unix_chkpwd) and not proc.pname in (cron_binaries, systemd, systemd.postins, udev.postinst, run-parts) and not proc.cmdline startswith "passwd -S" and not proc.cmdline startswith "useradd -D" and not proc.cmdline startswith "systemd --version" and not run_by_qualys and not run_by_sumologic_securefiles and not run_by_yum and not run_by_ms_oms and not run_by_google_accounts_daemon and not chage_list and not user_known_user_management_activities

```

Output:

```
User management binary command run outside of container | gparent=%proc.aname[2] ggparent=%proc.aname[3] gggparent=%proc.aname[4] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host users software_mgmt mitre_persistence T1098

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#User_mgmt_binaries

Description:

```
Detect the creation of files under /dev except for authorized device management programs. This can reveal rootkits hiding files in /dev. Additionally, consider the "Execution from /dev/shm" rule. The upstream rule already covers some tuning scenarios that you can further expand upon.

```

Condition:

```
(evt.type = creat or (evt.type in (open,openat,openat2))) and evt.arg.flags contains O_CREAT and fd.directory = /dev and not proc.name in (dev_creation_binaries) and not fd.name in (allowed_dev_files) and not fd.name startswith /dev/tty and not user_known_create_files_below_dev_activities

```

Output:

```
File created below /dev by untrusted program | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: ERROR

Tags: maturity_incubating host filesystem mitre_persistence T1543

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Create_files_below_dev

Description:

```
Detects attempts to communicate with the EC2 Instance Metadata Service from a container. This detection is narrowly focused and might not apply to your environment. In addition, it could generate noise and require fine-tuning.

```

Condition:

```
outbound and container and fd.sip="169.254.169.254" and not ec2_metadata_containers

```

Output:

```
Outbound connection to EC2 instance metadata service | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating network aws container mitre_credential_access T1552.005

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Contact_EC2_Instance_Metadata_Service_From_Container

Description:

```
Detects attempts to communicate with the Cloud Instance Metadata Service from a container. This detection is narrowly focused and might not apply to your environment. In addition, it could generate noise and require fine-tuning.

```

Condition:

```
outbound and container and fd.sip="169.254.169.254" and not user_known_metadata_access

```

Output:

```
Outbound connection to cloud instance metadata service | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating network container mitre_discovery T1565

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Contact_cloud_metadata_service_from_container

Description:

```
Detect package management processes executed within containers. An excellent auditing rule to monitor general drifts in containers. Particularly useful for newer rules like "Drop and execute new binary in container" during incident response investigations. This helps identify common anti-patterns of ad-hoc debugging. Simultaneously, to maintain optimal hygiene, it's recommended to prevent container drifts and instead opt for redeploying new containers.

```

Condition:

```
spawned_process and container and user.name != "_apt" and package_mgmt_procs and not package_mgmt_ancestor_procs and not user_known_package_manager_in_container and not pkg_mgmt_in_kube_proxy

```

Output:

```
Package management process launched in container | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: ERROR

Tags: maturity_incubating container process software_mgmt mitre_persistence T1505

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Launch_Package_Management_Process_in_Container

Description:

```
Detect network tools (like netcat, nmap, tcpdump, socat, and more) launched within containers without any additional filters. This serves as a valuable general detection, but it's recommended to invest engineering effort to fine-tune it and prevent a high volume of legitimate logs. This rule complements the more specific "Netcat Remote Code Execution in Container" rule.

```

Condition:

```
spawned_process and container and network_tool_procs and not user_known_network_tool_activities

```

Output:

```
Network tool launched in container | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating container network process mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Launch_Suspicious_Network_Tool_in_Container

Description:

```
Detect network tools (like netcat, nmap, tcpdump, socat, and more) launched within containers without any additional filters. This serves as a valuable general detection, but it's recommended to invest engineering effort to fine-tune it and prevent a high volume of legitimate logs. The host equivalent of "Launch Suspicious Network Tool in Container.".

```

Condition:

```
spawned_process and not container and network_tool_procs and not user_known_network_tool_activities

```

Output:

```
Network tool launched on host | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host network process mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Launch_Suspicious_Network_Tool_on_Host

Description:

```
Detect shell history deletion, frequently used by unsophisticated adversaries to eliminate evidence. Note that it can also trigger when exiting a Terminal shell, such as with `kubectl exec`, which may introduce some noise.

```

Condition:

```
(modify_shell_history or truncate_shell_history) and not var_lib_docker_filepath and not proc.name in (docker_binaries)

```

Output:

```
Shell history deleted or renamed | file=%fd.name name=%evt.arg.name path=%evt.arg.path oldpath=%evt.arg.oldpath evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: WARNING

Tags: maturity_incubating host container process filesystem mitre_defense_evasion T1070

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Delete_or_rename_shell_history

Description:

```
This rule is focused on detecting the use of setuid or setgid bits set via chmod. These bits, when set for an application, result in the application running with the privileges of the owning user or group. By enabling the setuid or setgid bits, an attacker could run code in a different user's context, possibly with elevated privileges. However, there's a trade-off with noise, given that numerous applications legitimately run chmod. This rule is related to the "Non sudo setuid" rule.

```

Condition:

```
chmod and (evt.arg.mode contains "S_ISUID" or evt.arg.mode contains "S_ISGID") and not proc.name in (user_known_chmod_applications) and not exe_running_docker_save and not user_known_set_setuid_or_setgid_bit_conditions

```

Output:

```
Setuid or setgid bit is set via chmod | fd=%evt.arg.fd filename=%evt.arg.filename mode=%evt.arg.mode evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host container process users mitre_privilege_escalation T1548.001

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Set_Setuid_or_Setgid_bit

Description:

```
Detect remote file copy tools (like rsync, scp, sftp, dcp) launched within a container, potentially indicating data exfiltration. Suggest refining this rule to accommodate legitimate use cases.

```

Condition:

```
spawned_process and container and remote_file_copy_procs and not user_known_remote_file_copy_activities

```

Output:

```
Remote file copy tool launched in container | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating container network process mitre_exfiltration T1020

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Launch_Remote_File_Copy_Tools_in_Container

Description:

```
Detect network traffic (inbound or outbound) from a container to a destination outside the local subnet. To operationalize this rule, profile your environment and update the template macro namespace_scope_network_only_subnet. Customizing network-related rules usually demands substantial engineering effort to ensure their functionality.

```

Condition:

```
inbound_outbound and container and k8s.ns.name in (namespace_scope_network_only_subnet) and not network_local_subnet

```

Output:

```
Network connection outside local subnet | fd_rip_name=%fd.rip.name fd_lip_name=%fd.lip.name fd_cip_name=%fd.cip.name fd_sip_name=%fd.sip.name connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: WARNING

Tags: maturity_incubating container network mitre_discovery T1046 PCI_DSS_6.4.2

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Network_Connection_outside_Local_Subnet

Description:

```
Detect filesystem mounts (using the mount binary) within a privileged container. Due to the elevated privileges, this action could be one of the TTPs used in an attempt to escape from a container to the host. This type of action is often preceded by reconnaissance activities, for which you can also create custom rules.

```

Condition:

```
spawned_process and container and container.privileged=true and proc.name=mount and not mount_info and not known_gke_mount_in_privileged_containers and not known_aks_mount_in_privileged_containers and not known_eks_mount_in_privileged_containers and not user_known_mount_in_privileged_containers

```

Output:

```
Mount was executed inside a privileged container | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: WARNING

Tags: maturity_incubating container cis filesystem mitre_privilege_escalation T1611

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Mount_Launched_in_Privileged_Container

Description:

```
Detect ingress remote file copy tools (such as curl or wget) launched inside containers. This rule can be considered a valuable auditing tool, but it has the potential to generate notable noise and requires careful profiling before full operationalization.

```

Condition:

```
spawned_process and container and (ingress_remote_file_copy_procs or curl_download) and not user_known_ingress_remote_file_copy_activities

```

Output:

```
Ingress remote file copy tool launched in container | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating container network process mitre_command_and_control TA0011

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Launch_Ingress_Remote_File_Copy_Tools_in_Container

Description:

```
An attempt to read process environment variables from /proc files. The consequences are akin to accessing traditional sensitive files, as sensitive data, including secrets, might be stored in environment variables. Understanding your environment, such as identifying critical namespaces, and incorporating extra filtering statements to alert exclusively for those, can enhance the rule's effectiveness.

```

Condition:

```
open_read and container and (fd.name glob /proc/*/environ) and not proc.name in (known_binaries_to_read_environment_variables_from_proc_files)

```

Output:

```
Environment variables were retrieved from /proc files | file=%fd.name gparent=%proc.aname[2] ggparent=%proc.aname[3] gggparent=%proc.aname[4] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: WARNING

Tags: maturity_incubating container filesystem process mitre_discovery T1083

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Read_environment_variable_from_%2fproc_files

Description:

```
Detect the copying of artifacts from a container's file system using the Kubernetes control plane (kubectl cp). This rule can identify potential exfiltration of application secrets from containers' file systems, potentially revealing the outcomes of unauthorized access and control plane misuse via stolen identities (such as stolen credentials like Kubernetes serviceaccount tokens). Can be customized by the adopter to only monitor specific artifact paths, containers, or namespaces as needed.

```

Condition:

```
open_read and container and proc.name=tar and container_entrypoint and proc.tty=0 and not system_level_side_effect_artifacts_kubectl_cp

```

Output:

```
Exfiltrating Artifacts via Kubernetes Control Plane | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating container filesystem mitre_exfiltration TA0010

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Exfiltrating_Artifacts_via_Kubernetes_Control_Plane

Description:

```
After gaining access, attackers can modify the authorized_keys file to maintain persistence on a victim host. Where authorized_keys files are modified via cloud APIs or command line interfaces, an adversary may achieve privilege escalation on the target virtual machine if they add a key to a higher-privileged user. This rules aims at detecting any modification to the authorized_keys file, that is usually located under the .ssh directory in any user's home directory. This rule complements the more generic auditing rule "Read ssh information" by specifically detecting the writing of new, potentially attacker-provided keys.

```

Condition:

```
open_write and (user_ssh_directory or fd.name startswith /root/.ssh) and fd.name endswith authorized_keys and not proc.name in (ssh_binaries)

```

Output:

```
Adding ssh keys to authorized_keys | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: WARNING

Tags: maturity_incubating host filesystem mitre_persistence T1098.004

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Adding_ssh_keys_to_authorized_keys

Description:

```
Process run with suspect environment variable that could be attempting privilege escalation. One use case is detecting the use of the GLIBC_TUNABLES environment variable, which could be used for privilege escalation on systems running vulnerable glibc versions. Only known and carefully profiled processes that legitimately exhibit this behavior should be excluded from this rule. This rule is expected to trigger on every attempt, even failed ones.

```

Condition:

```
spawned_process and glibc_tunables_env

```

Output:

```
Process run with suspect environment variable which could be attempting privilege escalation | env=%proc.env evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host container users mitre_privilege_escalation TA0004

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Potential_Local_Privilege_Escalation_via_Environment_Variables_Misuse

Description:

```
This rule detects possible CVE-2024-3094 exploitation when the SSH daemon process loads a vulnerable version of the liblzma library. An attacker could exploit this to interfere with authentication in sshd via systemd, potentially compromising sensitive data or escalating their privileges.
```

Condition:

```
open_read and proc.name=sshd and (fd.name contains "liblzma.so.5.6.0" or fd.name contains "liblzma.so.5.6.1")

```

Output:

```
SSHD loaded a backdoored version of liblzma library %fd.name with parent %proc.pname and cmdline %proc.cmdline | process=%proc.name parent=%proc.pname file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid proc_exepath=%proc.exepath command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: incubating

Priority: WARNING

Tags: maturity_incubating host container mitre_initial_access T1556

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Backdoored_library_loaded_into_SSHD_%28CVE-2024-3094%29

Description:

```
BPF is a kernel technology that can be misused for malicious purposes, like "Linux Kernel Module Injection". This rule should be considered an auditing rule to notify you of any unprofiled BPF tools running in your environment. However, it requires customization after profiling your environment. BPF-powered agents make bpf syscalls all the time, so this rule only sends logs for BPF_PROG_LOAD calls (bpf cmd=5) in the enter event. If you also want to log whether the syscall failed or succeeded, remove the direction filter and add the evt.arg.res_or_fd output field.

```

Condition:

```
evt.type=bpf and (evt.arg.cmd=5 or evt.arg.cmd=BPF_PROG_LOAD) and not bpf_profiled_procs

```

Output:

```
BPF Program Not Profiled | bpf_cmd=%evt.arg.cmd evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: incubating

Priority: NOTICE

Tags: maturity_incubating host container mitre_persistence TA0003

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#BPF_Program_Not_Profiled

Description:

```
Detect any inbound connection from a source outside of an allowed set of ips, networks, or domain names. This rule absolutely requires profiling your environment beforehand. Network-based rules are extremely crucial in any security program, as they can often provide the only definitive evidence. However, effectively operationalizing them can be challenging due to the potential for noise.

```

Condition:

```
inbound and not ((fd.cip in (allowed_inbound_source_ipaddrs)) or
         (fd.cnet in (allowed_inbound_source_networks)) or
         (fd.cip.name in (allowed_inbound_source_domains)))

```

Output:

```
Disallowed inbound connection source | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: NOTICE

Tags: maturity_sandbox host container network mitre_command_and_control TA0011

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Unexpected_inbound_connection_source

Description:

```
This rule detects attempts made by non-shell programs to read shell configuration files. It offers additional generic auditing. It serves as a baseline detection alert for unusual shell configuration file accesses. The rule "Modify Shell Configuration File" might be more relevant and adequate for your specific cases.

```

Condition:

```
open_read and (fd.filename in (shell_config_filenames) or
     fd.name in (shell_config_files) or
     fd.directory in (shell_config_directories))
and not proc.name in (shell_binaries)

```

Output:

```
A shell configuration file was read by a non-shell program | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: WARNING

Tags: maturity_sandbox host container filesystem mitre_discovery T1546.004

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Read_Shell_Configuration_File

Description:

```
This rule generically detects updates to package repositories and can be seen as an auditing measure. Recommend evaluating its relevance for your specific environment.

```

Condition:

```
((open_write and access_repositories) or (modify and modify_repositories)) and not package_mgmt_procs and not package_mgmt_ancestor_procs and not exe_running_docker_save and not user_known_update_package_registry

```

Output:

```
Repository files get updated | newpath=%evt.arg.newpath file=%fd.name pcmdline=%proc.pcmdline evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: NOTICE

Tags: maturity_sandbox host container filesystem mitre_execution T1072

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Update_Package_Repository

Description:

```
Trying to write to any file below specific binary directories can serve as an auditing rule to track general system changes. Such rules can be noisy and challenging to interpret, particularly if your system frequently undergoes updates. However, careful profiling of your environment can transform this rule into an effective rule for detecting unusual behavior associated with system changes, including compliance-related cases.

```

Condition:

```
open_write and bin_dir and not package_mgmt_procs and not exe_running_docker_save and not python_running_get_pip and not python_running_ms_oms and not user_known_write_below_binary_dir_activities

```

Output:

```
File below a known binary directory opened for writing | file=%fd.name pcmdline=%proc.pcmdline gparent=%proc.aname[2] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: ERROR

Tags: maturity_sandbox host container filesystem mitre_persistence T1543

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Write_below_binary_dir

Description:

```
Trying to write to any file below a set of monitored directories can serve as an auditing rule to track general system changes. Such rules can be noisy and challenging to interpret, particularly if your system frequently undergoes updates. However, careful profiling of your environment can transform this rule into an effective rule for detecting unusual behavior associated with system changes, including compliance-related cases.

```

Condition:

```
open_write and monitored_dir and not package_mgmt_procs and not coreos_write_ssh_dir and not exe_running_docker_save and not python_running_get_pip and not python_running_ms_oms and not google_accounts_daemon_writing_ssh and not cloud_init_writing_ssh and not user_known_write_monitored_dir_conditions

```

Output:

```
File below a monitored directory opened for writing | file=%fd.name pcmdline=%proc.pcmdline gparent=%proc.aname[2] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: ERROR

Tags: maturity_sandbox host container filesystem mitre_persistence T1543

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Write_below_monitored_dir

Description:

```
Trying to write to any file below /etc can serve as an auditing rule to track general system changes. Such rules can be noisy and challenging to interpret, particularly if your system frequently undergoes updates. However, careful profiling of your environment can transform this rule into an effective rule for detecting unusual behavior associated with system changes, including compliance-related cases.

```

Condition:

```
write_etc_common
```

Output:

```
File below /etc opened for writing | file=%fd.name pcmdline=%proc.pcmdline gparent=%proc.aname[2] ggparent=%proc.aname[3] gggparent=%proc.aname[4] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: ERROR

Tags: maturity_sandbox host container filesystem mitre_persistence T1098

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Write_below_etc

Description:

```
Trying to write to any file directly below / or /root can serve as an auditing rule to track general system changes. Such rules can be noisy and challenging to interpret, particularly if your system frequently undergoes updates. However, careful profiling of your environment can transform this rule into an effective rule for detecting unusual behavior associated with system changes, including compliance-related cases. Lastly, this rule stands out as potentially the noisiest one among rules related to "write below.

```

Condition:

```
open_write and root_dir and proc_name_exists and not fd.name in (known_root_files) and not fd.directory pmatch (known_root_directories) and not exe_running_docker_save and not gugent_writing_guestagent_log and not dse_writing_tmp and not zap_writing_state and not airflow_writing_state and not rpm_writing_root_rpmdb and not maven_writing_groovy and not chef_writing_conf and not kubectl_writing_state and not cassandra_writing_state and not galley_writing_state and not calico_writing_state and not rancher_writing_root and not runc_writing_exec_fifo and not mysqlsh_writing_state and not known_root_conditions and not user_known_write_root_conditions and not user_known_write_below_root_activities

```

Output:

```
File below / or /root opened for writing | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: ERROR

Tags: maturity_sandbox host container filesystem mitre_persistence TA0003

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Write_below_root

Description:

```
Trying to write to the rpm database by any non-rpm related program can serve as an auditing rule to track general system changes. Such rules can be noisy and challenging to interpret, particularly if your system frequently undergoes updates. However, careful profiling of your environment can transform this rule into an effective rule for detecting unusual behavior associated with system changes, including compliance-related cases.

```

Condition:

```
open_write and fd.name startswith /var/lib/rpm and not rpm_procs and not ansible_running_python and not python_running_chef and not exe_running_docker_save and not amazon_linux_running_python_yum and not user_known_write_rpm_database_activities

```

Output:

```
rpm database opened for writing by a non-rpm program | file=%fd.name pcmdline=%proc.pcmdline evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: ERROR

Tags: maturity_sandbox host container filesystem software_mgmt mitre_persistence T1072

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Write_below_rpm_database

Description:

```
Trying to modify any file below a set of binary directories can serve as an auditing rule to track general system changes. Such rules can be noisy and challenging to interpret, particularly if your system frequently undergoes updates. However, careful profiling of your environment can transform this rule into an effective rule for detecting unusual behavior associated with system changes, including compliance-related cases.

```

Condition:

```
modify and bin_dir_rename and not package_mgmt_procs and not exe_running_docker_save and not user_known_modify_bin_dir_activities

```

Output:

```
File below known binary directory renamed/removed | file=%fd.name pcmdline=%proc.pcmdline evt_args=%evt.args evt_type=%evt.type evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: ERROR

Tags: maturity_sandbox host container filesystem mitre_defense_evasion T1222.002

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Modify_binary_dirs

Description:

```
Trying to create a directory below a set of binary directories can serve as an auditing rule to track general system changes. Such rules can be noisy and challenging to interpret, particularly if your system frequently undergoes updates. However, careful profiling of your environment can transform this rule into an effective rule for detecting unusual behavior associated with system changes, including compliance-related cases.

```

Condition:

```
mkdir and bin_dir_mkdir and not package_mgmt_procs and not user_known_mkdir_bin_dir_activities and not exe_running_docker_save

```

Output:

```
Directory below known binary directory created | directory=%evt.arg.path evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: ERROR

Tags: maturity_sandbox host container filesystem mitre_persistence T1222.002

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Mkdir_binary_dirs

Description:

```
Detect the initial process launched within a container that has a mount from a sensitive host directory (e.g. /proc). Exceptions are made for known trusted images. This rule holds value for generic auditing; however, its noisiness varies based on your environment.

```

Condition:

```
container_started and sensitive_mount and not falco_sensitive_mount_containers and not user_sensitive_mount_containers

```

Output:

```
Container with sensitive mount started | mounts=%container.mounts evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: INFO

Tags: maturity_sandbox container cis mitre_execution T1610

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Launch_Sensitive_Mount_Container

Description:

```
Detect the initial process launched within a container that is not in a list of allowed containers. This rule holds value for generic auditing; however, this rule requires a good understanding of your setup and consistent effort to keep the list of allowed containers current. In some situations, this can be challenging to manage.

```

Condition:

```
container_started and not allowed_containers

```

Output:

```
Container started and not in allowed list | evt_type=%evt.type user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: WARNING

Tags: maturity_sandbox container mitre_lateral_movement T1610

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Launch_Disallowed_Container

Description:

```
Any inbound network activity performed by any interpreted program (perl, python, ruby, etc.). While it offers broad coverage and behavioral insights, operationalizing it effectively requires significant time and might result in a moderate level of noise. Suggesting customizing this rule to be more specific. For example, you could set it up to alert only for important namespaces after studying their usual behavior.

```

Condition:

```
inbound and interpreted_procs

```

Output:

```
Interpreted program received/listened for network traffic | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: NOTICE

Tags: maturity_sandbox host container network mitre_exfiltration TA0011

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Interpreted_procs_inbound_network_activity

Description:

```
Any outbound network activity performed by any interpreted program (perl, python, ruby, etc.). While it offers broad coverage and behavioral insights, operationalizing it effectively requires significant time and might result in a moderate level of noise. Suggesting customizing this rule to be more specific. For example, you could set it up to alert only for important namespaces after studying their usual behavior.

```

Condition:

```
outbound and interpreted_procs

```

Output:

```
Interpreted program performed outgoing network connection | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: NOTICE

Tags: maturity_sandbox host container network mitre_exfiltration TA0011

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Interpreted_procs_outbound_network_activity

Description:

```
Detect attempts to utilize K8s NodePorts from a container. K8s NodePorts are accessible on the eth0 interface of each node, and they facilitate external traffic into a Kubernetes cluster. Attackers could misuse them for unauthorized access. The rule uses default port ranges, but check for custom ranges and make necessary adjustments. Also, consider tuning this rule as needed.

```

Condition:

```
inbound_outbound and container and fd.sport >= 30000 and fd.sport <= 32767 and not nodeport_containers

```

Output:

```
Unexpected K8s NodePort Connection | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: NOTICE

Tags: maturity_sandbox network k8s container mitre_persistence T1205.001 NIST_800-53_AC-6

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Unexpected_K8s_NodePort_Connection

Description:

```
Detecting hidden files or directories creation can serve as an auditing rule to track general system changes. Such rules can be noisy and challenging to interpret, particularly if your system frequently undergoes updates. However, careful profiling of your environment can transform this rule into an effective rule for detecting unusual behavior associated with system changes, including compliance-related cases.

```

Condition:

```
((modify and evt.arg.newpath contains "/.") or
 (mkdir and evt.arg.path contains "/.") or
 (open_write and evt.arg.flags contains "O_CREAT" and fd.name contains "/." and not fd.name pmatch (exclude_hidden_directories)))
and not user_known_create_hidden_file_activities and not exe_running_docker_save

```

Output:

```
Hidden file or directory created | file=%fd.name newpath=%evt.arg.newpath evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: NOTICE

Tags: maturity_sandbox host container filesystem mitre_defense_evasion T1564.001

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Create_Hidden_Files_or_Directories

Description:

```
Miners usually connect to miner pools using standard ports, and this rule flags such activity. Important: Falco currently sends DNS requests to resolve miner pool domains, which could trigger other alerts. Prior to enabling this rule, it's advised to ensure whether this is acceptable for your environment. This rule is specifically disabled for that reason.

```

Condition:

```
net_miner_pool and not trusted_images_query_miner_domain_dns

```

Output:

```
Outbound connection to IP/Port flagged by https://cryptoioc.ch | ip=%fd.rip connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox host container network mitre_impact T1496

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Detect_outbound_connections_to_common_miner_pool_ports

Description:

```
Miners commonly specify the mining pool to connect to using a URI that starts with "stratum+tcp". However, this rule is highly specific to this technique, and matching command-line arguments can generally be bypassed quite easily.

```

Condition:

```
spawned_process and (proc.cmdline contains "stratum+tcp" or
     proc.cmdline contains "stratum2+tcp" or
     proc.cmdline contains "stratum+ssl" or
     proc.cmdline contains "stratum2+ssl")

```

Output:

```
Possible miner running | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox host container process mitre_impact T1496

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Detect_crypto_miners_using_the_Stratum_protocol

Description:

```
Detect the execution of a Kubernetes client tool (like docker, kubectl, crictl) within a container, which is typically not expected behavior. Although this rule targets container workloads, monitoring the use of tools like crictl on the host over interactive access could also be valuable for broader auditing objectives.

```

Condition:

```
spawned_process and container and not user_known_k8s_client_container_parens and proc.name in (k8s_client_binaries)

```

Output:

```
Kubernetes Client Tool Launched in Container | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: WARNING

Tags: maturity_sandbox container mitre_execution T1610

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Kubernetes_Client_Tool_Launched_in_Container

Description:

```
Detect new executables created within a container as a result of chmod. While this detection can generate significant noise, chmod usage is frequently linked to dropping and executing malicious implants. The newer rule "Drop and execute new binary in container" provides more precise detection of this TTP using unambiguous kernel signals. It is recommended to use the new rule. However, this rule might be more relevant for auditing if applicable in your environment, such as when chmod is used on files within the /tmp folder.

```

Condition:

```
chmod and container and evt.rawres>=0 and ((evt.arg.mode contains "S_IXUSR") or
     (evt.arg.mode contains "S_IXGRP") or
     (evt.arg.mode contains "S_IXOTH"))
and not runc_writing_exec_fifo and not runc_writing_var_lib_docker and not user_known_container_drift_activities

```

Output:

```
Drift detected (chmod), new executable created in a container | filename=%evt.arg.filename name=%evt.arg.name mode=%evt.arg.mode evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: ERROR

Tags: maturity_sandbox container process filesystem mitre_execution T1059

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Container_Drift_Detected_%28chmod%29

Description:

```
Detect new executables created within a container as a result of open+create. The newer rule "Drop and execute new binary in container" provides more precise detection of this TTP using unambiguous kernel signals. It is recommended to use the new rule.

```

Condition:

```
evt.type in (open,openat,openat2,creat) and evt.rawres>=0 and evt.is_open_exec=true and container and not runc_writing_exec_fifo and not runc_writing_var_lib_docker and not user_known_container_drift_activities

```

Output:

```
Drift detected (open+create), new executable created in a container | filename=%evt.arg.filename name=%evt.arg.name mode=%evt.arg.mode evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: ERROR

Tags: maturity_sandbox container process filesystem mitre_execution T1059

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Container_Drift_Detected_%28open%2bcreate%29

Description:

```
Container detected running as the root user. This should be taken into account especially when policies disallow containers from running with root user privileges. Note that a root user in containers doesn't inherently possess extensive power, as modern container environments define privileges through Linux capabilities. To learn more, check out the rule "Launch Privileged Container".

```

Condition:

```
spawned_process and container and proc.vpid=1 and user.uid=0 and not user_known_run_as_root_container

```

Output:

```
Container launched with root user privilege | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: INFO

Tags: maturity_sandbox container process users mitre_execution T1610

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Container_Run_as_Root_User

Description:

```
Affecting sudo (<= 1.9.5p2), there's a privilege escalation vulnerability. By executing sudo using the sudoedit -s or sudoedit -i command with a command-line argument that ends with a single backslash character, an unprivileged user can potentially escalate privileges to root. This rule is highly specific and might be bypassed due to potential issues with string matching on command line arguments.

```

Condition:

```
spawned_process and user.loginuid != 0 and (proc.name=sudoedit or proc.name = sudo) and (proc.args contains -s or proc.args contains -i or proc.args contains --login) and (proc.args contains "\ " or proc.args endswith \)

```

Output:

```
Detect Sudo Privilege Escalation Exploit (CVE-2021-3156) | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox host container filesystem users mitre_privilege_escalation T1548.003

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Sudo_Potential_Privilege_Escalation

Description:

```
Detect a successful unprivileged userfaultfd syscall, which could serve as an attack primitive for exploiting other vulnerabilities. To fine-tune this rule, consider using the template list "user_known_userfaultfd_processes".

```

Condition:

```
evt.type = userfaultfd and user.uid != 0 and (evt.rawres >= 0 or evt.res != -1) and not proc.name in (user_known_userfaultfd_processes)

```

Output:

```
An userfaultfd syscall was successfully executed by an unprivileged user | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox host container process mitre_defense_evasion TA0005

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Unprivileged_Delegation_of_Page_Faults_Handling_to_a_Userspace_Process

Description:

```
This rule detects attempts to exploit a privilege escalation vulnerability in Polkit's pkexec. Through the execution of specially crafted code, a local user can exploit this weakness to attain root privileges on a compromised system. This rule is highly specific in its scope.

```

Condition:

```
spawned_process and user.loginuid != 0 and proc.name=pkexec and proc.args = ''
```

Output:

```
Detect Polkit pkexec Local Privilege Escalation Exploit (CVE-2021-4034) | args=%proc.args evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox host container process users mitre_privilege_escalation TA0004

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Polkit_Local_Privilege_Escalation_Vulnerability_%28CVE-2021-4034%29

Description:

```
Detecting a Java process downloading a class file which could indicate a successful exploit of the log4shell Log4j vulnerability (CVE-2021-44228). This rule is highly specific in its scope.

```

Condition:

```
java_network_read and evt.buffer bcontains cafebabe

```

Output:

```
Java process class file download | server_ip=%fd.sip server_port=%fd.sport connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox host container process mitre_initial_access T1190

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Java_Process_Class_File_Download

Description:

```
This rule detect an attempt to write on container entrypoint symlink (/proc/self/exe). Possible CVE-2019-5736 Container Breakout exploitation attempt. This rule has a more narrow scope.

```

Condition:

```
open_write and container and (fd.name=/proc/self/exe or fd.name startswith /proc/self/fd/) and not docker_procs and not proc.cmdline = "runc:[1:CHILD] init"

```

Output:

```
Detect Potential Container Breakout Exploit (CVE-2019-5736) | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: WARNING

Tags: maturity_sandbox container filesystem mitre_initial_access T1611

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Modify_Container_Entrypoint

Description:

```
Detect any use of {base64} decoding in a container. Legitimate applications may decode encoded payloads. The template list known_decode_payload_containers can be used for simple tuning and customization, or you can adopt custom, more refined tuning. Less sophisticated adversaries may {base64}-decode their payloads not only to obfuscate them, but also to ensure that the payload remains intact when the application processes it. Note that injecting commands into an application's input often results in the application processing passed strings like "sh -c". In these cases, you may be lucky and the encoded blob will also be logged. Otherwise, all you will see is the {base64} decoding command, as the encoded blob was already interpreted by the shell.

```

Condition:

```
spawned_process and container and base64_decoding and not container.image.repository in (known_decode_payload_containers)

```

Output:

```
Decoding Payload in Container | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: INFO

Tags: maturity_sandbox container process mitre_command_and_control T1132

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Decoding_Payload_in_Container

Description:

```
This rule detects basic interactive reconnaissance commands that are typically run by unsophisticated attackers or used in internal Red Team exercises. Interactive is defined as a terminal being present (proc.tty != 0). This could be any form of reverse shell or usage of kubectl exec or ssh etc. In addition, filtering for the process being the process group leader indicates that the command was "directly" typed into the terminal and not run as a result of a script. This rule is a basic auditing or template rule. You can expand the list of reconnaissance commands, such as by adding "ls". Common anti-patterns are SRE activity or debugging, but it is still worth capturing this generically. Typically, you would expect other rules to fire as well in relation to this activity.

```

Condition:

```
spawned_process and recon_binaries_procs and proc.tty != 0 and proc.is_vpgid_leader=true

```

Output:

```
Basic Interactive Reconnaissance | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: NOTICE

Tags: maturity_sandbox host container process mitre_reconnaissance TA0043

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Basic_Interactive_Reconnaissance

Description:

```
Netcat/Socat Program runs on host that allows remote code execution and may be utilized as a part of a variety of reverse shell payload https://github.com/swisskyrepo/PayloadsAllTheThings/. These programs are of higher relevance as they are commonly installed on UNIX-like operating systems.

```

Condition:

```
spawned_process and not container and ((proc.name = "nc" and (proc.cmdline contains "-e" or
                            proc.cmdline contains "-c")) or
     (proc.name = "ncat" and (proc.args contains "--sh-exec" or
                              proc.args contains "--exec" or proc.args contains "-e " or
                              proc.args contains "-c " or proc.args contains "--lua-exec")) or
     (proc.name = 'socat' and (proc.args contains "EXEC" or
                               proc.args contains "SYSTEM")))

```

Output:

```
Netcat/Socat runs on host that allows remote code execution | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: WARNING

Tags: maturity_sandbox host network process mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Netcat%2fSocat_Remote_Code_Execution_on_Host

Description:

```
Detect network tools being launched when an NPM package is installed. Malicious NPM packages may invoke  preinstall or postinstall commands, which can involve network tools to download malicious payloads or exfiltrate  sensitive information. Network tools spawned by npm, node, yarn, pnpm, bun, or related processes during  package installation should be investigated. This rule complements the more generic "Launch Suspicious Network  Tool in Container" rule with a specific focus on the supply chain attack vector via NPM packages.

```

Condition:

```
spawned_process and container and network_tool_procs and npm_package_install_ancestor and not user_known_network_tool_in_npm_install_activities

```

Output:

```
Network tool executed during NPM package installation | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty gparent=%proc.aname[2] ggparent=%proc.aname[3] gggparent=%proc.aname[4] exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: WARNING

Tags: maturity_sandbox container network process mitre_execution T1195.002

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Network_Tool_Executed_During_NPM_Package_Install

Description:

```
Detects execution of known cryptocurrency mining software by matching process names against a list of common miners. Cryptominers are commonly deployed by attackers after gaining initial access to monetize compromised systems. This rule complements the existing "Detect crypto miners using the Stratum protocol" rule by catching miners that may use renamed binaries but still match known miner process names. Consider tuning this rule if you have legitimate mining operations in your environment.

```

Condition:

```
spawned_process and is_miner_process

```

Output:

```
Cryptominer process executed | process=%proc.name exe=%proc.exepath evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox host container process mitre_impact T1496

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Known_Cryptominer_Process_Executed

Description:

```
Detects a web server process (nginx, apache, php-fpm, etc.) spawning an interactive shell. This is a strong indicator of webshell exploitation or remote code execution vulnerability being actively exploited. Webshells are a common persistence mechanism used by attackers after compromising web applications. The macro user_known_web_server_shell_activities can be overridden to tune for legitimate use cases like CGI scripts. Note that simple shell wrappers used in health checks are excluded.

```

Condition:

```
spawned_process and spawned_by_web_server and proc.name in (shell_binaries) and not proc.cmdline startswith "sh -c /usr/bin" and not proc.cmdline contains "healthcheck" and not user_known_web_server_shell_activities

```

Output:

```
Web server spawned shell | shell=%proc.name web_server=%proc.pname gparent=%proc.aname[2] ggparent=%proc.aname[3] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid command=%proc.cmdline terminal=%proc.tty exe=%proc.exepath exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox host container process mitre_persistence mitre_execution T1505.003

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Web_Server_Spawned_Shell

Description:

```
Detects web server processes spawning suspicious child processes like curl, wget, netcat, or scripting interpreters. These often indicate exploitation of web application vulnerabilities, where attackers download additional payloads or establish network connections. This rule is more sensitive than the shell spawning rule and may require tuning in environments where web applications legitimately execute system commands. Override the macro user_known_web_server_child_activities for custom tuning.

```

Condition:

```
spawned_process and spawned_by_web_server and proc.name in (suspicious_web_children) and not proc.name in (shell_binaries) and not proc.cmdline contains "healthcheck" and not proc.cmdline contains "status" and not user_known_web_server_child_activities

```

Output:

```
Web server spawned suspicious child process | child=%proc.name web_server=%proc.pname gparent=%proc.aname[2] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: WARNING

Tags: maturity_sandbox host container process mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Web_Server_Spawned_Suspicious_Child_Process

Description:

```
Detects common reverse shell patterns spawned from web server processes. Attackers frequently establish reverse shells after exploiting web vulnerabilities to maintain interactive access to the compromised system. This rule looks for specific command patterns known to create reverse shells, including bash /dev/tcp redirects, netcat with -e flag, and common scripting language reverse shell idioms.

```

Condition:

```
spawned_process and spawned_by_web_server and (proc.cmdline contains "/dev/tcp/" or
     proc.cmdline contains "nc -e" or
     proc.cmdline contains "ncat -e" or
     proc.cmdline contains "bash -i" or
     proc.cmdline contains "python -c 'import socket" or
     proc.cmdline contains "python3 -c 'import socket" or
     proc.cmdline contains "perl -e 'use Socket" or
     proc.cmdline contains "ruby -rsocket" or
     (proc.cmdline contains "mkfifo" and proc.cmdline contains "/tmp"))

```

Output:

```
Reverse shell spawned from web server | process=%proc.name web_server=%proc.pname evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox host container process network mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Reverse_Shell_from_Web_Server

Description:

```
Detects container processes accessing raw block devices, which could be used to escape container isolation by directly reading or writing to host storage. This technique allows attackers to access the host filesystem, modify system files, or extract sensitive data even without traditional container escape. Requires the container to have privileged access or specific device permissions. Override user_known_privileged_device_access for legitimate storage operations.

```

Condition:

```
(open_read or open_write) and container and (fd.name startswith /dev/sd or
     fd.name startswith /dev/nvme or
     fd.name startswith /dev/vd or
     fd.name startswith /dev/xvd or
     fd.name = /dev/mem or
     fd.name = /dev/kmem)
and not user_known_privileged_device_access

```

Output:

```
Container accessing block device | device=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox container filesystem mitre_privilege_escalation T1611

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Privileged_Container_Device_Access

Description:

```
Detects container processes accessing paths that typically only exist when the host filesystem is mounted into the container (like /host, /rootfs, /hostfs). This may indicate a container escape attempt or a misconfigured volume mount that exposes the host filesystem. Legitimate monitoring or backup containers may access these paths, so tune accordingly using the user_known_host_path_access macro.

```

Condition:

```
(open_read or open_write) and container and (fd.name startswith /host/ or
     fd.name startswith /rootfs/ or
     fd.name startswith /hostfs/)
and not known_container_runtime_host_access and not user_known_host_path_access

```

Output:

```
Container accessing host filesystem paths | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: WARNING

Tags: maturity_sandbox container filesystem mitre_privilege_escalation T1611

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Container_Access_to_Host_Sensitive_Paths

Description:

```
Detects a container process opening an NVIDIA or AMD GPU or accelerator character device. Opening such a device is required to submit compute work to the hardware, making this a high-signal indicator of GPU cryptojacking when it originates from a workload that is not expected to use accelerators. It complements the network- and process-name-centric cryptominer rules, which a GPU miner can evade by using an unknown binary name and an unknown mining pool. Because a miner cannot submit work to a GPU without opening the device, this device open is a reliable behavioral chokepoint for the Resource Hijacking technique.

```

Condition:

```
open_gpu_device and container and not user_known_gpu_workloads

```

Output:

```
Container accessing GPU device | device=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox container filesystem mitre_impact T1496

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#Container_Accessing_GPU_Device

Description:

```
Detects execution of an NVIDIA or AMD GPU management or query tool, such as nvidia-smi or rocm-smi, inside a container. Attackers commonly run these tools immediately after gaining access to a GPU-equipped node to fingerprint the available accelerators before deploying a miner. Inside a workload container this is rarely legitimate outside of machine learning and HPC images. It is a signature-style detection that pairs with the device-access rule to provide both a behavioral and a signature view of the same accelerator-hijacking technique.

```

Condition:

```
spawned_process and container and proc.name in (gpu_management_binaries) and not user_known_gpu_workloads

```

Output:

```
GPU management tool run in container | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: sandbox

Priority: CRITICAL

Tags: maturity_sandbox container process mitre_impact T1496

Status: disabled

Link: https://falco.org/docs/reference/rules/default-rules/#GPU_Management_Tool_Run_in_Container

Description:

```
Web applications can be vulnerable to directory traversal attacks that allow accessing files outside of the web app's root directory (e.g. Arbitrary File Read bugs). System directories like /etc are typically accessed via absolute paths. Access patterns outside of this (here path traversal) can be regarded as suspicious. This rule includes failed file open attempts.

```

Condition:

```
(open_read or open_file_failed) and (etc_dir or user_ssh_directory or
     fd.name startswith /root/.ssh or
     fd.name contains "id_rsa")
and directory_traversal and not proc.pname in (shell_binaries)

```

Output:

```
Read monitored file via directory traversal | file=%fd.name fileraw=%fd.nameraw gparent=%proc.aname[2] ggparent=%proc.aname[3] gggparent=%proc.aname[4] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container filesystem mitre_credential_access T1555

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Directory_traversal_monitored_file_read

Description:

```
An attempt to read any sensitive file (e.g. files containing user/password/authentication information) by a trusted program after startup. Trusted programs might read these files at startup to load initial state, but not afterwards. Can be customized as needed. In modern containerized cloud infrastructures, accessing traditional Linux sensitive files might be less relevant, yet it remains valuable for baseline detections. While we provide additional rules for SSH or cloud vendor-specific credentials, you can significantly enhance your security program by crafting custom rules for critical application credentials unique to your environment.

```

Condition:

```
open_read and sensitive_files and server_procs and not proc_is_new and proc.name!="sshd" and not user_known_read_sensitive_files_activities

```

Output:

```
Sensitive file opened for reading by trusted program after startup | file=%fd.name pcmdline=%proc.pcmdline gparent=%proc.aname[2] ggparent=%proc.aname[3] gggparent=%proc.aname[4] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container filesystem mitre_credential_access T1555

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Read_sensitive_file_trusted_after_startup

Description:

```
An attempt to read any sensitive file (e.g. files containing user/password/authentication information). Exceptions are made for known trusted programs. Can be customized as needed. In modern containerized cloud infrastructures, accessing traditional Linux sensitive files might be less relevant, yet it remains valuable for baseline detections. While we provide additional rules for SSH or cloud vendor-specific credentials, you can significantly enhance your security program by crafting custom rules for critical application credentials unique to your environment.

```

Condition:

```
open_read and sensitive_files and proc_name_exists and not proc.name in (user_mgmt_binaries, userexec_binaries, package_mgmt_binaries,
 cron_binaries, read_sensitive_file_binaries, shell_binaries, hids_binaries,
 vpn_binaries, mail_config_binaries, nomachine_binaries, sshkit_script_binaries,
 in.proftpd, mandb, salt-call, salt-minion, postgres_mgmt_binaries,
 google_oslogin_
 )
and not cmp_cp_by_passwd and not ansible_running_python and not run_by_qualys and not run_by_chef and not run_by_google_accounts_daemon and not user_read_sensitive_file_conditions and not mandb_postinst and not perl_running_plesk and not perl_running_updmap and not veritas_driver_script and not perl_running_centrifydc and not runuser_reading_pam and not linux_bench_reading_etc_shadow and not user_known_read_sensitive_files_activities and not user_read_sensitive_file_containers and not systemd_sensitive_file_readers

```

Output:

```
Sensitive file opened for reading by non-trusted program | file=%fd.name gparent=%proc.aname[2] ggparent=%proc.aname[3] gggparent=%proc.aname[4] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container filesystem mitre_credential_access T1555

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Read_sensitive_file_untrusted

Description:

```
An attempt to spawn a shell below a non-shell application. The non-shell applications that are monitored are defined in the protected_shell_spawner macro, with protected_shell_spawning_binaries being the list you can easily customize. For Java parent processes, please note that Java often has a custom process name. Therefore, rely more on proc.exe to define Java applications. This rule can be noisier, as you can see in the exhaustive existing tuning. However, given it is very behavior-driven and broad, it is universally relevant to catch general Remote Code Execution (RCE). Allocate time to tune this rule for your use cases and reduce noise. Tuning suggestions include looking at the duration of the parent process (proc.ppid.duration) to define your long-running app processes. Checking for newer fields such as proc.vpgid.name and proc.vpgid.exe instead of the direct parent process being a non-shell application could make the rule more robust.

```

Condition:

```
spawned_process and shell_procs and proc.pname exists and protected_shell_spawner and not proc.pname in (shell_binaries, gitlab_binaries, cron_binaries, user_known_shell_spawn_binaries,
                       needrestart_binaries,
                       mesos_shell_binaries,
                       erl_child_setup, exechealthz,
                       PM2, PassengerWatchd, c_rehash, svlogd, logrotate, hhvm, serf,
                       lb-controller, nvidia-installe, runsv, statsite, erlexec, calico-node,
                       "puma reactor")
and not proc.cmdline in (known_shell_spawn_cmdlines) and not proc.aname in (unicorn_launche) and not consul_running_net_scripts and not consul_running_alert_checks and not nginx_starting_nginx and not nginx_running_aws_s3_cp and not run_by_package_mgmt_binaries and not serf_script and not check_process_status and not run_by_foreman and not python_mesos_marathon_scripting and not splunk_running_forwarder and not postgres_running_wal_e and not postgres_running_cnpg and not redis_running_prepost_scripts and not rabbitmq_running_scripts and not rabbitmqctl_running_scripts and not run_by_appdynamics and not user_shell_container_exclusions

```

Output:

```
Shell spawned by untrusted binary | parent_exe=%proc.pexe parent_exepath=%proc.pexepath pcmdline=%proc.pcmdline gparent=%proc.aname[2] ggparent=%proc.aname[3] aname[4]=%proc.aname[4] aname[5]=%proc.aname[5] aname[6]=%proc.aname[6] aname[7]=%proc.aname[7] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: NOTICE

Tags: maturity_stable host container process shell mitre_execution T1059.004

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Run_shell_untrusted

Description:

```
System (e.g. non-login) users spawning new processes. Can add custom service users (e.g. apache or mysqld). 'Interactive' is defined as new processes as descendants of an ssh session or login process. Consider further tuning by only looking at processes in a terminal / tty (proc.tty != 0). A newer field proc.is_vpgid_leader could be of help to distinguish if the process was "directly" executed, for instance, in a tty, or executed as a descendant process in the same process group, which, for example, is the case when subprocesses are spawned from a script. Consider this rule as a great template rule to monitor interactive accesses to your systems more broadly. However, such a custom rule would be unique to your environment. The rule "Terminal shell in container" that fires when using "kubectl exec" is more Kubernetes relevant, whereas this one could be more interesting for the underlying host.

```

Condition:

```
spawned_process and system_users and interactive and not user_known_system_user_login

```

Output:

```
System user ran an interactive command | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: INFO

Tags: maturity_stable host container users mitre_execution T1059 NIST_800-53_AC-2

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#System_user_interactive

Description:

```
A shell was used as the entrypoint/exec point into a container with an attached terminal. Parent process may have legitimately already exited and be null (read container_entrypoint macro). Common when using "kubectl exec" in Kubernetes. Correlate with k8saudit exec logs if possible to find user or serviceaccount token used (fuzzy correlation by namespace and pod name). Rather than considering it a standalone rule, it may be best used as generic auditing rule while examining other triggered rules in this container/tty.

```

Condition:

```
spawned_process and container and shell_procs and proc.tty != 0 and container_entrypoint and not user_expected_terminal_shell_in_container_conditions

```

Output:

```
A shell was spawned in a container with an attached terminal | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: NOTICE

Tags: maturity_stable container shell mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Terminal_shell_in_container

Description:

```
Detect attempts to communicate with the K8S API Server from a container by non-profiled users. Kubernetes APIs play a pivotal role in configuring the cluster management lifecycle. Detecting potential unauthorized access to the API server is of utmost importance. Audit your complete infrastructure and pinpoint any potential machines from which the API server might be accessible based on your network layout. If Falco can't operate on all these machines, consider analyzing the Kubernetes audit logs (typically drained from control nodes, and Falco offers a k8saudit plugin) as an additional data source for detections within the control plane.

```

Condition:

```
evt.type=connect and (fd.typechar=4 or fd.typechar=6) and container and k8s_api_server and not k8s_containers and not user_known_contact_k8s_api_server_activities

```

Output:

```
Unexpected connection to K8s API Server from container | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: NOTICE

Tags: maturity_stable container network k8s mitre_discovery T1565

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Contact_K8S_API_Server_From_Container

Description:

```
Netcat Program runs inside container that allows remote code execution and may be utilized as a part of a variety of reverse shell payload https://github.com/swisskyrepo/PayloadsAllTheThings/. These programs are of higher relevance as they are commonly installed on UNIX-like operating systems. Can fire in combination with the "Redirect STDOUT/STDIN to Network Connection in Container" rule as it utilizes a different evt.type.

```

Condition:

```
spawned_process and container and ((proc.name = "nc" and (proc.cmdline contains " -e" or
                            proc.cmdline contains " -c")) or
     (proc.name = "ncat" and (proc.args contains "--sh-exec" or
                              proc.args contains "--exec" or proc.args contains "-e " or
                              proc.args contains "-c " or proc.args contains "--lua-exec"))
     )

```

Output:

```
Netcat runs inside container that allows remote code execution | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable container network process mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Netcat_Remote_Code_Execution_in_Container

Description:

```
Detect attempts to search for private keys or passwords using the grep or find command. This is often seen with unsophisticated attackers, as there are many ways to access files using bash built-ins that could go unnoticed. Regardless, this serves as a solid baseline detection that can be tailored to cover these gaps while maintaining an acceptable noise level.

```

Condition:

```
spawned_process and ((grep_commands and private_key_or_password) or
     (proc.name = "find" and (proc.args contains "id_rsa" or
                              proc.args contains "id_dsa" or
                              proc.args contains "id_ed25519" or
                              proc.args contains "id_ecdsa"
      )
    ))

```

Output:

```
Grep private keys or passwords activities found | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container process filesystem mitre_credential_access T1552.001

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Search_Private_Keys_or_Passwords

Description:

```
Detect clearing of critical access log files, typically done to erase evidence that could be attributed to an adversary's actions. To effectively customize and operationalize this detection, check for potentially missing log file destinations relevant to your environment, and adjust the profiled containers you wish not to be alerted on.

```

Condition:

```
open_write and access_log_files and evt.arg.flags contains "O_TRUNC" and not containerd_activities and not trusted_logging_images and not allowed_clear_log_files

```

Output:

```
Log files were tampered | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container filesystem mitre_defense_evasion T1070 NIST_800-53_AU-10

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Clear_Log_Activities

Description:

```
Detect a process running to clear bulk data from disk with the intention to destroy data, possibly interrupting availability to systems. Profile your environment and use user_known_remove_data_activities to tune this rule.

```

Condition:

```
spawned_process and clear_data_procs and not user_known_remove_data_activities

```

Output:

```
Bulk data has been removed from disk | file=%fd.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container process filesystem mitre_impact T1485

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Remove_Bulk_Data_from_Disk

Description:

```
Detect symlinks created over a curated list of sensitive files or subdirectories under /etc/ or root directories. Can be customized as needed. Refer to further and equivalent guidance within the rule "Read sensitive file untrusted".

```

Condition:

```
create_symlink and (evt.arg.target in (sensitive_file_names) or evt.arg.target in (sensitive_directory_names))

```

Output:

```
Symlinks created over sensitive files | target=%evt.arg.target linkpath=%evt.arg.linkpath evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container filesystem mitre_credential_access T1555

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Create_Symlink_Over_Sensitive_Files

Description:

```
Detect hardlink created over a curated list of sensitive files or subdirectories under /etc/ or root directories. Can be customized as needed. Refer to further and equivalent guidance within the rule "Read sensitive file untrusted".

```

Condition:

```
create_hardlink and (evt.arg.oldpath in (sensitive_file_names))

```

Output:

```
Hardlinks created over sensitive files | target=%evt.arg.oldpath linkpath=%evt.arg.newpath evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container filesystem mitre_credential_access T1555

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Create_Hardlink_Over_Sensitive_Files

Description:

```
Detect new packet socket at the device driver (OSI Layer 2) level in a container. Packet socket could be used for ARP Spoofing and privilege escalation (CVE-2020-14386) by an attacker. Noise can be reduced by using the user_known_packet_socket_binaries template list.

```

Condition:

```
evt.type=socket and container and evt.arg.domain contains AF_PACKET and not proc.name in (user_known_packet_socket_binaries)

```

Output:

```
Packet socket was created in a container | socket_info=%evt.args connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: NOTICE

Tags: maturity_stable container network mitre_credential_access T1557.002

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Packet_socket_created_in_container

Description:

```
Detect redirection of stdout/stdin to a network connection within a container, achieved by utilizing a variant of the dup syscall (potential reverse shell or remote code execution https://github.com/swisskyrepo/PayloadsAllTheThings/). This detection is behavior-based and may generate noise in the system, and can be adjusted using the user_known_stand_streams_redirect_activities template macro. Tuning can be performed similarly to existing detections based on process lineage or container images, and/or it can be limited to interactive tty (tty != 0).

```

Condition:

```
dup and container and evt.rawres in (0, 1, 2) and fd.type in ("ipv4", "ipv6") and not user_known_stand_streams_redirect_activities

```

Output:

```
Redirect stdout/stdin to network connection | gparent=%proc.aname[2] ggparent=%proc.aname[3] gggparent=%proc.aname[4] fd.sip=%fd.sip connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: NOTICE

Tags: maturity_stable container network process mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Redirect_STDOUT%2fSTDIN_to_Network_Connection_in_Container

Description:

```
Inject Linux Kernel Modules from containers using insmod or modprobe with init_module and finit_module syscalls, given the precondition of sys_module effective capabilities. Profile the environment and consider allowed_container_images_loading_kernel_module to reduce noise and account for legitimate cases.

```

Condition:

```
kernel_module_load and container and thread.cap_effective icontains sys_module and not container.image.repository in (allowed_container_images_loading_kernel_module)

```

Output:

```
Linux Kernel Module injection from container | parent_exepath=%proc.pexepath gparent=%proc.aname[2] gexepath=%proc.aexepath[2] module=%proc.args res=%evt.res evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container process mitre_persistence TA0003

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Linux_Kernel_Module_Injection_Detected

Description:

```
Detect file system debugger debugfs launched inside a privileged container which might lead to container escape. This rule has a more narrow scope.

```

Condition:

```
spawned_process and container and container.privileged=true and proc.name=debugfs

```

Output:

```
Debugfs launched started in a privileged container | evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable container cis process mitre_privilege_escalation T1611

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Debugfs_Launched_in_Privileged_Container

Description:

```
Detect an attempt to exploit a container escape using release_agent file. By running a container with certains capabilities, a privileged user can modify release_agent file and escape from the container.

```

Condition:

```
open_write and container and fd.name endswith release_agent and (user.uid=0 or thread.cap_effective contains CAP_DAC_OVERRIDE) and thread.cap_effective contains CAP_SYS_ADMIN

```

Output:

```
Detect an attempt to exploit a container escape using release_agent file | file=%fd.name cap_effective=%thread.cap_effective evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: CRITICAL

Tags: maturity_stable container process mitre_privilege_escalation T1611

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Detect_release_agent_File_Container_Escapes

Description:

```
Detect an attempt to inject potentially malicious code into a process using PTRACE in order to evade process-based defenses or elevate privileges. Common anti-patterns are debuggers. Additionally, profiling your environment via the known_ptrace_procs template macro can reduce noise. A successful ptrace syscall generates multiple logs at once.

```

Condition:

```
ptrace_attach_or_injection and proc_name_exists and not known_ptrace_procs

```

Output:

```
Detected ptrace PTRACE_ATTACH attempt | proc_pcmdline=%proc.pcmdline evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container process mitre_privilege_escalation T1055.008

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#PTRACE_attached_to_process

Description:

```
Detect usage of the PTRACE system call with the PTRACE_TRACEME argument, indicating a program actively attempting to avoid debuggers attaching to the process. This behavior is typically indicative of malware activity. Read more about PTRACE in the "PTRACE attached to process" rule.

```

Condition:

```
evt.type=ptrace and evt.arg.request contains PTRACE_TRACEME and proc_name_exists

```

Output:

```
Detected potential PTRACE_TRACEME anti-debug attempt | proc_pcmdline=%proc.pcmdline evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: NOTICE

Tags: maturity_stable host container process mitre_defense_evasion T1622

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#PTRACE_anti-debug_attempt

Description:

```
Detect attempts to search for private keys or passwords using the grep or find command, particularly targeting standard AWS credential locations. This is often seen with unsophisticated attackers, as there are many ways to access files using bash built-ins that could go unnoticed. Regardless, this serves as a solid baseline detection that can be tailored to cover these gaps while maintaining an acceptable noise level. This rule complements the rule "Search Private Keys or Passwords".

```

Condition:

```
spawned_process and ((grep_commands and private_aws_credentials) or
     (proc.name = "find" and proc.args endswith ".aws/credentials"))

```

Output:

```
Detected AWS credentials search activity | proc_pcmdline=%proc.pcmdline proc_cwd=%proc.cwd group_gid=%group.gid group_name=%group.name user_loginname=%user.loginname evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container process aws mitre_credential_access T1552

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Find_AWS_Credentials

Description:

```
This rule detects file execution in the /dev/shm directory, a tactic often used by threat actors to store their readable, writable, and occasionally executable files. /dev/shm acts as a link to the host or other containers, creating vulnerabilities for their compromise as well. Notably, /dev/shm remains unchanged even after a container restart. Consider this rule alongside the newer "Drop and execute new binary in container" rule.

```

Condition:

```
spawned_process and (proc.exe startswith "/dev/shm/" or
    (proc.cwd startswith "/dev/shm/" and proc.exe startswith "./" ) or
    (shell_procs and proc.args startswith "-c /dev/shm") or
    (shell_procs and proc.args startswith "-i /dev/shm") or
    (shell_procs and proc.args startswith "/dev/shm") or
    (proc.cwd startswith "/dev/shm/" and proc.args startswith "./" ))
and not container.image.repository in (falco_privileged_images, trusted_images)

```

Output:

```
File execution detected from /dev/shm | evt_res=%evt.res file=%fd.name proc_cwd=%proc.cwd proc_pcmdline=%proc.pcmdline user_loginname=%user.loginname group_gid=%group.gid group_name=%group.name evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: WARNING

Tags: maturity_stable host container mitre_execution T1059.004

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Execution_from_%2fdev%2fshm

Description:

```
Detect if an executable not belonging to the base image of a container is being executed. The drop and execute pattern can be observed very often after an attacker gained an initial foothold. is_exe_upper_layer filter field only applies for container runtimes that use overlayfs as union mount filesystem. Adopters can utilize the provided template list known_drop_and_execute_containers containing allowed container images known to execute binaries not included in their base image. Alternatively, you could exclude non-production namespaces in Kubernetes settings by adjusting the rule further. This helps reduce noise by applying application and environment-specific knowledge to this rule. Common anti-patterns include administrators or SREs performing ad-hoc debugging.

```

Condition:

```
spawned_process and container and proc.is_exe_upper_layer=true and not container.image.repository in (known_drop_and_execute_containers) and not known_drop_and_execute_activities

```

Output:

```
Executing binary not part of base image | proc_exe=%proc.exe proc_sname=%proc.sname gparent=%proc.aname[2] proc_exe_ino_ctime=%proc.exe_ino.ctime proc_exe_ino_mtime=%proc.exe_ino.mtime proc_exe_ino_ctime_duration_proc_start=%proc.exe_ino.ctime_duration_proc_start proc_cwd=%proc.cwd container_start_ts=%container.start_ts evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: CRITICAL

Tags: maturity_stable container process mitre_persistence TA0003 PCI_DSS_11.5.1

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Drop_and_execute_new_binary_in_container

Description:

```
Detect any new outbound SSH connection from the host or container using a non-standard port. This rule holds the potential to detect a family of reverse shells that cause the victim machine to connect back out over SSH, with STDIN piped from the SSH connection to a shell's STDIN, and STDOUT of the shell piped back over SSH. Such an attack can be launched against any app that is vulnerable to command injection. The upstream rule only covers a limited selection of non-standard ports. We suggest adding more ports, potentially incorporating ranges based on your environment's knowledge and custom SSH port configurations. This rule can complement the "Redirect STDOUT/STDIN to Network Connection in Container" or "Disallowed SSH Connection" rule.

```

Condition:

```
outbound and proc.exe endswith ssh and fd.l4proto=tcp and ssh_non_standard_ports_network

```

Output:

```
Disallowed SSH Connection | connection=%fd.name lport=%fd.lport rport=%fd.rport fd_type=%fd.type fd_proto=%fd.l4proto evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty
```

Maturity: stable

Priority: NOTICE

Tags: maturity_stable host container network process mitre_execution T1059

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Disallowed_SSH_Connection_Non_Standard_Port

Description:

```
Detect if a binary is executed from memory using the memfd_create technique. This is a well-known defense evasion technique for executing malware on a victim machine without storing the payload on disk and to avoid leaving traces about what has been executed. Adopters can whitelist processes that may use fileless execution for benign purposes by adding items to the list known_memfd_execution_processes.

```

Condition:

```
spawned_process and proc.is_exe_from_memfd=true and not known_memfd_execution_processes

```

Output:

```
Fileless execution via memfd_create | container_start_ts=%container.start_ts proc_cwd=%proc.cwd evt_res=%evt.res proc_sname=%proc.sname gparent=%proc.aname[2] evt_type=%evt.type user=%user.name user_uid=%user.uid user_loginuid=%user.loginuid process=%proc.name proc_exepath=%proc.exepath parent=%proc.pname command=%proc.cmdline terminal=%proc.tty exe_flags=%evt.arg.flags
```

Maturity: stable

Priority: CRITICAL

Tags: maturity_stable host container process mitre_defense_evasion T1620

Status: enabled

Link: https://falco.org/docs/reference/rules/default-rules/#Fileless_execution_via_memfd_create

#### Was this page helpful?

Let us know! You feedback will help us to improve the content and to stay in touch with our users.

Glad to hear it! Please [tell us how we can improve](https://github.com/falcosecurity/falco-website/issues/new).

Sorry to hear that. Please [tell us how we can improve](https://github.com/falcosecurity/falco-website/issues/new).

Last modified Sep 16, 2026: [docs(rules): refresh Helm artifact references (b86b2aa)](https://github.com/falcosecurity/falco-website/commit/b86b2aae2f2809406cafd087fdf0a5a26ab5992c)
