---
url: "https://github.com/C0nd4/CCDC-Blueteam-Manual"
title: "GitHub - C0nd4/CCDC-Blueteam-Manual"
site: "GitHub"
captured: "2026-11-02T18:34:40Z"
via: "live"
sha256: "ba8046038e5aa4db5dcf25d5b0dd80765735284348137f4be5159ce4f47ed784"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# GitHub - C0nd4/CCDC-Blueteam-Manual

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/C0nd4/CCDC-Blueteam-Manual>

# CCDC Blueteam Manual

https://github.com/C0nd4/CCDC-Blueteam-Manual#ccdc-blueteam-manual

## Pre-Competition Methodology

https://github.com/C0nd4/CCDC-Blueteam-Manual#pre-competition-methodology

The events that you will encounter in CCDC will be correlated to real paths attackers will take, but the competition is a bit gamified in order to present a challenge to both the blue team and red team. Many of the things you learn to defend systems in this competition will directly translate to real-world applicable cybersecurity skills.

When initially given the blue team manual before the qualifier round, the number one thing to do is enumerate the attack surfaces of your new machines. Here are some good questions that you should have answered as a team long before the competition begins.

- What operating systems are running?
- What services are running on these boxes?
- Are there any CVEs for these services?
- How do you change the default credentials for these services?
- What ports are required to stay open?

### If you can't build the environment, there is no way you can defend it!

https://github.com/C0nd4/CCDC-Blueteam-Manual#if-you-cant-build-the-environment-there-is-no-way-you-can-defend-it

Once you are told what the environment that you need to defend is going to look like, it is vital to replicate this environment as fast and as accurately as possible. The team should get practice with the services at hand and know how they operate, how they are configured, and how to make changes to them on the fly.

### Stop Chasing Ghosts!

https://github.com/C0nd4/CCDC-Blueteam-Manual#stop-chasing-ghosts

Many people will get caught up with the idea that a red team could be in their systems and will panic at everything they see that they do not recognize. Often the unknown files, processes, or user accounts are standard Linux or Windows features. Chasing these leads that turn out to be nothing malicious will only eat up time and will allow the red team to gain an advantage. It is vital to understand the machine you are defending! If you do not know what the baseline normal is for a machine, you cannot possibly identify an anomaly. This is a major reason for replicating the target environment before the competition. It would be wise to get a list of the common files, processes, and user accounts for the operating systems that you will be defending to minimize your time chasing ghosts.

## Initial Cleaning

https://github.com/C0nd4/CCDC-Blueteam-Manual#initial-cleaning

As soon as you gain access to your new machines during the competition, every single person needs to have a solid plan of action. Often teams will have a plan for the first 15 minutes, then it falls apart when people get stuck or notice suspected red team activity. This is the responsibility of the team leader to ensure everyone is working together and as a single unit. Your team will either succeed together or fail together. Here are some common things to address immediately after gaining access to your systems.

### Linux

https://github.com/C0nd4/CCDC-Blueteam-Manual#linux

Commands listed are not all-encompassing and are for a baseline only!

-

Change all user passwords. Yes, especially the credentials given to you!

# passwd username

-

Audit /etc/shadow for users with passwords set, or no password at all.

cat /etc/shadow

-

Remove all SSH keys present on the box.

# find / -name authorized_keys 2> /dev/null

# find / -name id_rsa 2> /dev/null

-

Audit sudo access given to users.

# cat /etc/sudoers

# cat /etc/sudoers.d/*

# getent group sudo | cut -d: -f4 (The sudo group is Debian, wheel is for RHEL)

-

Audit /etc/passwd to check for account shells and UIDs.

$ cat /etc/passwd | grep :0:

$ cat /etc/passwd | grep -v /bin/false | grep -v /sbin/nologin

-

Verify that there are not any non-standard cron jobs on the system.

# cat /etc/cron.d/*

# for user in $(cut -f1 -d: /etc/passwd); do crontab -u $user -l; done

-

Remove packages that could be used for malicious purposes if not needed.

# apt remove socat nc ncat nmap netcat (apt is for Debian, yum is for RHEL). Other packages must be manually inspected in order to prevent taking down critical services.

-

Stop services that are not critical to the system or competition needs.

# systemctl --type=service --state=active

# systemctl stop servicename

-

Identify SUID and SGID files. Cross-reference with https://gtfobins.github.io/ to narrow down malicous instances of SUID and SGID files.

# find / -perm -4000 -print 2>/dev/null for SUID

# find / -perm -2000 -print 2>/dev/null for SGID

-

Identify world-writable files and directories.

# find / -type d \( -perm -g+w -or -perm -o+w \) -exec ls -adl {} \; for directories

# find / ! -path "*/proc/*" -perm -2 -type f -print 2>/dev/null for files

-

Check who is currently logged into the machine.

$ who

#### Additional Resources

https://github.com/C0nd4/CCDC-Blueteam-Manual#additional-resources

- [Linux Privilege Escalation Guide](https://github.com/swisskyrepo/PayloadsAllTheThings/blob/master/Methodology%20and%20Resources/Linux%20-%20Privilege%20Escalation.md)
- [Linux Persistence Guide](https://github.com/swisskyrepo/PayloadsAllTheThings/blob/master/Methodology%20and%20Resources/Linux%20-%20Persistence.md)

#### Useful Tools

https://github.com/C0nd4/CCDC-Blueteam-Manual#useful-tools

- [LinEnum](https://github.com/rebootuser/LinEnum)
- [pspy](https://github.com/DominicBreuker/pspy)

### Windows

https://github.com/C0nd4/CCDC-Blueteam-Manual#windows

-

Change all user passwords. Yes, especially the credentials given to you!

> net user <username> <password> NOTE: If domain user, append /domain

-

Audit important groups.

> net localgroup Administrators

> net localgroup "Remote Desktop Users"

> net localgroup "Remote Management Users"

-

Disable WinRM if not needed.

PS> Disable-PSRemoting -Force

-

Check Windows Defender registry keys

> regedit.exe HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows Defender

-

Check for tasks set to run through the registry.

- HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
- HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\RunOnce
- HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\RunServices
- HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\RunServicesOnce

-

Check system and user startup folder.

- User: C:\Users\USERNAME\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\
- System: C:\ProgramData\Microsoft\Windows\Start Menu\Programs\StartUp\

-

Audit scheduled tasks.

> schtasks

-

Check PowerShell Execution policy.

PS> Get-ExecutionPolicy

PS> Set-ExecutionPolicy -ExecutionPolicy Restricted -Scope LocalMachine

-

Check Windows Defender status.

PS> Get-MPComputerStatus

-

Audit SMB shares.

> net view \\127.0.0.1

-

Disable Guest account.

> net user guest /active no

#### Additional Resources

https://github.com/C0nd4/CCDC-Blueteam-Manual#additional-resources-1

- [Windows Privilege Escalation Guide](https://github.com/swisskyrepo/PayloadsAllTheThings/blob/master/Methodology%20and%20Resources/Windows%20-%20Privilege%20Escalation.md)
- [Windows Persistence Guide](https://github.com/swisskyrepo/PayloadsAllTheThings/blob/master/Methodology%20and%20Resources/Windows%20-%20Persistence.md)

#### Useful Tools

https://github.com/C0nd4/CCDC-Blueteam-Manual#useful-tools-1

- [PowerUp](https://github.com/PowerShellMafia/PowerSploit/blob/master/Privesc/PowerUp.ps1)

## Monitoring Activity

https://github.com/C0nd4/CCDC-Blueteam-Manual#monitoring-activity

### Linux

https://github.com/C0nd4/CCDC-Blueteam-Manual#linux-1

-

Audit listening ports and established connections

# lsof -i -P -n

-

Audit running processes

# ps aux

-

Monitor logs in /var/log/

### Windows

https://github.com/C0nd4/CCDC-Blueteam-Manual#windows-1

-

Audit listening ports

> netstat –na

-

List the SMB sessions this machine has opened with other systems

> net use

-

List the open SMB sessions with this machine

> net session

-

Audit running processes

> tasklist
