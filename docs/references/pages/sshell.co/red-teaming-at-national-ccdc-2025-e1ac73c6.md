---
url: "https://www.sshell.co/red-teaming-at-national-ccdc-2025/"
title: "Red Teaming at National CCDC 2025"
site: "sshell@blog:~$ ▌"
published: "2025-07-05T17:14:47.000Z"
captured: "2026-11-02T18:59:39Z"
via: "live"
sha256: "8f3f5c0bd508c3d2f8fe91d297c33998989f3d020107ca35be003b35bd532655"
wayback: "https://web.archive.org/web/20260612203601/https://www.sshell.co/red-teaming-at-national-ccdc-2025/"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Red Teaming at National CCDC 2025

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.sshell.co/red-teaming-at-national-ccdc-2025/>
> Snapshot: <https://web.archive.org/web/20260612203601/https://www.sshell.co/red-teaming-at-national-ccdc-2025/>

Jul 05, 2025    [ccdc](https://www.sshell.co/tag/ccdc/)

# Red Teaming at National CCDC 2025

There's nothing quite like the feeling of playing Doom on someone's hypervisor and watching as they frantically try to figure out how to eject you from the system.

### Intro

Another season of Collegiate Cyber Defense Competition (CCDC) has officially ended. I had the good fortune this year to be on the red team for National CCDC this year, and I can only describe it as "some of the most fun you can have on a computer." There's nothing quite like the feeling of playing Doom on someone's hypervisor and watching as they frantically try to figure out how to eject you from the system, or spawning 100 copies of an unkillable goose that runs around wreaking havoc on the desktop of a domain controller.

As with every year, I'm consistently blown away by the skill and kindness of the individuals I got to play with. You would never believe the insane tradecraft used and tools created for no other purpose than the noble pursuit of turning the green service indicators on the scoreboard into red ones. CCDC is one of the few events a year where I always walk away inspired to learn more about operating system internals and create better tools, as so many have before me.

I always receive questions every year from blue teamers (which I love to answer) but in the interest of making the advice more accessible, I wanted to give a quick rundown on the environment this year as well as provide some tips for those looking to participate in future CCDC on either side.

### Quick Intro to CCDC

- Each "blue team" is made up of 8 students from a given school charged with protecting a small network from active attackers.
- The red team is made up of a variable number of offensive security professionals to attack these networks.
- Other teams that keep the competition going are Gold Team (officials that organize, run, and manage the competition), White Team (officials who are responsible for enforcing rules and calculating final scores), Black Team (who create and manage the competition environment, manage communications, and provide support), and Orange Team (officials who are in charge of manual testing of services, content, and functionality in environments.)
- Scores are calculated based on how many of the services competitors can keep available throughout the competition. Points are deducted for each separate intrusion by red team or failure to secure sensitive "customer" information. Asking Black Team to restore/recover a machine or violating "service level agreements" will incur additional penalties. Teams can gain points by completing "injects" which emulate real-world business situations (preparing reports, spinning up new services, etc.)
- CCDC is broken into 9 regional blocks (by state)
- Each region has invitational and qualifying rounds, followed by the finals.
- Winner of the finals goes on to National CCDC. Second place in each region goes to a "wildcard" playoff to compete for the last spot.
- No matter which team says they "win", red team are always the real winners.

### This Year's Environment

The environment this year had a good mixture of familiar mainstays (WordPress, Magento, osTicket, Dolibarr) and a number of fun applications I'd never seen before to fit the medical/genetic theme. I don't perfectly recall all the vulnerabilities and versions, so you're going to have to make do with some guesses here.

| Product  | Use  | Vulnerability

| OrangeHRM  | HR software  | Multiple CVEs

| BioPHP Minitools  | Many unique applications all piled together  | Weird and jank

| GaiaEHR  | Electronic health records system  | Lots (it's old PHP) + factoryReset.php is funny

| Dolibarr 17.0.0  | Enterprise resource planning + CRM  | RCE via [CVE-2023-38888](https://nvd.nist.gov/vuln/detail/CVE-2023-38888?ref=sshell.co)

| DoliMed  | Dolibarr with a plugin  |

| Magento  | PHP store platform  | Exposed configuration files + /downloader endpoint is RCE with credentials

| G-Onramp  | Genome browser + tools  | Can install custom tools and run code. Default creds found [here](https://wustl.app.box.com/v/g-onramp-vm-v1/file/362482959571?ref=sshell.co). Disk image [here](https://wustl.app.box.com/v/g-onramp-vm-v1/file/363266327762?ref=sshell.co)

| osTicket (<1.10.1)  | Support ticket software  | [CVE-2017-15580](https://nvd.nist.gov/vuln/detail/CVE-2017-15580?ref=sshell.co)

| WordPress  | Literally WordPress  | wp-file-manager v6.0 plugin vulnerable to RCE via [CVE-2020-25213](https://nvd.nist.gov/vuln/detail/CVE-2020-25213?ref=sshell.co)

| SquirrelMail  | Webmail client  | Post-auth RCE via [CVE-2017-7692](https://nvd.nist.gov/vuln/detail/CVE-2017-7692?ref=sshell.co)

| Fedora Server web login  | Web console + server administration  | Privilege escalation via [CVE-2019-15666](https://nvd.nist.gov/vuln/detail/cve-2019-15666?ref=sshell.co)

### Initial Access

Perhaps the most difficult challenge as a red teamer at these events is choosing from the embarrassment of riches laid out before you. As one of the few people who specializes on the web side of things, I started by taking inventory of what was we had in front of us.

WordPress always stands out as an easy target that I'm more than familiar with. Black team was nice enough to leave directory listings enabled which made enumeration easy. A simple peek in the wp-content would tell us most of what we need to know, and plugins are easily versioned in most cases by checking wp-content/plugins/YOUR-PLUGIN-HERE/readme.txt for the number immediately after "Stable tag." [WPScan vulnerability database](https://wpscan.com/plugin/wp-file-manager/?ref=sshell.co) is good for quickly confirming we had easy unauthenticated code execution here from an outdated version of wp-file-manager. Nice.

Magento is another app that is very common with relatively straightforward exploitation. Open directories here again mean that we snagged all of the user sessions and config files which contained encryption keys, database creds, etc. This information, when found and reported, incurred a large score penalty for the blue teams.

There are a lot of bugs that existed in web apps that the red team knew about but didn't even get around to exploiting. For most services, exploits weren't even necessarily required, cracked passwords from other services on the beginning of day 1 really opened most doors on web apps. Sometimes you'll even see pre-existing webshells from "prior compromises" left behind by black team as well.

Most of the exploit paths for web apps require a little digging around on the site and some google-fu to find a proof-of-concept for a vulnerability. Every application is generally vulnerable in some way, and most would probably be considered "easy" level web challenges on a CTF.

### Database

Databases are funny little things because they make up the core function of so many applications. With 13 web applications and 2 databases, they're sure to be juicy targets, so we wanted to jump in and dump those as fast as we can. Inside the database we found tons of personal information, encrypted credit card numbers (those secrets from earlier are handy here for decryption), passwords, and pretty much anything else you can ask for. Besides your domain controller, DNS server, and firewall, the database server is one of the few assets that can turn the scoreboard red in a matter of seconds.

As a note, databases themselves are not scored services and should generally never be accessed except by a set number of known hosts. If you're a blue teamer, you can (and should) easily monitor connections here to find IPs to block and restrict access from the outside. Despite this, by the end of the first day, no team had fully restricted access or rotated all 4 sets of default database creds.

### Persistence

Much of the ability to persist is based on early compromise of systems and cracking hashes dumped from those systems. Environments are generally set up in a way that there are a good number of users (~10-20) and these users all have passwords that are propagated out to (or reused on) different hosts. There are some exceptions for things like local admin accounts on different services.

Lots of persistence comes from shells on these boxes calling back to red team C2s, both inside and outside of the competition network. Multiple blog posts can be made talking about any single one of these topics I've touched on, but I'd venture to say that persistence would be the longest (and most fun) out of all of them. Shouts out to the teams that didn't notice multiple gigabytes of traffic over NTP, I'm sure those machines just REALLY liked checking to see what time it was.

There's not a whole lot to talk about with web persistence at the application level that will be surprising to blue teamers or security professionals.

- Webshells are everyone's favorite go-to, but easily found by just looking for recently created/modified files (can always [timestomp](https://attack.mitre.org/techniques/T1070/006/?ref=sshell.co) them, but not a super common practice in this environment.)
- Secrets gleaned from config files will allow you to sign cookies and create your own admin sessions (plus teams rarely know or think to rotate them.)
- Backdoored plugins are fun, but essentially act as webshells that can only target specific applications. Some things you get for "free" are neat, like being able to execute on any/every page.

If we wanted to go deeper, the idea of "the backdoored plugin" applies at every level: WordPress plugins, PHP extensions, nginx modules, kernel modules, etc. Every red teamer has their own favorite flavor of persistence, and almost all of them are equal parts diabolical and hilarious (which seems to be a general theme.)

### Note about Competition

A great thing to keep in mind is that even if your instance of a service doesn't get compromised, other instances will be. Within an hour or two, every default password and configuration is known by the red team. Often times to avoid detection, red teamers will test and deploy payloads against other teams to avoid being noisy on the network of their primary target. There are plenty of aspects where the right decision isn't always a logical one in the real world.

### Effective Blue Teaming

- Rotate passwords. We have your passwords figured out and documented probably before you even do. In many cases they will continue to work everywhere the rest of the competition.
- Identify and exploit different use patterns. Failed logins, database connection requests, directory brute forcing, etc. If you spin up a new webserver and someone visits it, why would they be there? If your team authenticates to SSH with keys, who is left establishing sessions with passwords? Can you distinguish and exploit the way services are scored? Where are machines calling out to? Was that gigabyte spike of DNS traffic from the domain controller going to a real DNS server?
- Block judiciously. Red team can change IPs, but it's annoying. Competitions have been doing a better job intermixing red team IP ranges, but some of the most successful teams are those that close the door early and often. Callbacks are only good if they can get out.
- If it's not scored, just shut it down. Some competitions have more 'useless' boxes than others, but in general minimize attack surface when you can. In this environment, was wp-file-manager critical to the operation of the WordPress site? Probably not.
- Small speedbumps matter. A lot of attacks are going to be opportunistically sprayed across all teams by red teamers whose primary focus is a team that is not yours. It doesn't always need to be the most resilient defense, it just has to be one that's annoying enough to make someone want to attack a different target.
- Think outside the box. There are some fun stories of teams shutting down the majority of services at the start of the day, patching them, and then putting them back online. Did it work for them? Who knows. It doesn't matter anyway, your team is simply built different. You guys got this.

### So You Want to be on Red Team?

Some of the questions I get asked the most aren't about how to perform better in competition, but how to get involved with being on the red team for the next season. Most of the time the answer is to contact the red team lead for your region and ask to help with invitationals, help out and learn as much as you can, and go from there. Reaching out and being persistent will open lots of doors for you both here, and in general.

In my personal journey, I learned a lot (and had a lot of fun) by helping out with black team for a year or two before hopping over to red team. Black team is full of a lot of insanely talented, unsung heroes and always needs more help. Some of the people who are the best at breaking things are the ones who started out by building things. It's not flashy, but you're guaranteed to be a better hacker by having that experience.

Other tips include:

- Be prepared. I've seen people put in an unbelievable amount of effort to build out specific tooling that will end up being used maybe once or twice a year. Some of the best red teamers you'll ever meet have built out their own C2 frameworks mostly born out of the need for specific tooling at CCDC. Have your own personal tools and one-liners ready. If you show that you're serious and put in the hours, you'll be invited back. Red team has a lot of fun, but there's a lot of dedication that you don't see in the weeks and months leading up to an event.
- Be humble. If you hear someone you don't know say "I'm not great at Windows stuff," it may be that they've forgotten more about Windows than you'll ever learn. If you're trying to show off, you're 10x more likely to make a fool out of yourself than impress someone. If there's one constant that I can rely on, it's that every event I will learn something new from the people around me.
- Remember that everyone is on the same team at the end of the day. People take time out of their lives because the event is fun, but ultimately everyone comes together to provide an unparalleled learning experience for the students on the blue team. You may not think it, but red team wants to see blue teams learn, grow, and succeed. We're the first to celebrate when we see blue teams doing cool shit, and in general love answering questions.

### Conclusion

- CCDC is a tremendous amount of fun and a world-class opportunity to learn and grow for everyone involved.
- Even at the highest levels of competition there is a lot of room to grow.
- There's room (and need) for volunteers at every level in every region.
- If you're in college there are TONS of other great cybersecurity competitions out there. For example, check out the [Collegiate Penetration Testing Competition](https://cp.tc/?ref=sshell.co).

### Additional Reading

- Dan Borges

- [Red Teaming at National CCDC 2024](https://lockboxx.blogspot.com/2024/05/red-teaming-at-nccdc-2024.html?ref=sshell.co)
- [Red Teaming at National CCDC 2023](https://lockboxx.blogspot.com/2023/05/red-teaming-at-national-ccdc-2023.html?ref=sshell.co)
- [Red Teaming at National CCDC 2022](https://lockboxx.blogspot.com/2022/05/red-teaming-at-national-ccdc-2022.html?ref=sshell.co)
- [CCDC (At Large Regional) Review 2025](https://lockboxx.blogspot.com/2025/03/alccdc-2025-review.html?ref=sshell.co)
- [CCDC (At Large Regional) Review 2024](https://lockboxx.blogspot.com/2024/03/vccdc-2024-regional-review.html?ref=sshell.co)
- [CCDC (At Large Regional) Review 2023](https://lockboxx.blogspot.com/2023/02/vccdc-2023-regional-review.html?ref=sshell.co)
- [CCDC (At Large Regional) Review 2022](https://lockboxx.blogspot.com/2022/03/vccdc-2022-regional-review.html?ref=sshell.co)
- [CCDC (At Large Regional) Review 2021](https://lockboxx.blogspot.com/2021/03/alccdc-2021-regional-review.html?ref=sshell.co)
- [CCDC Visibility and Blocking Tips (2023)](https://lockboxx.blogspot.com/2023/09/ccdc-visability-and-blocking-tips.html?ref=sshell.co)
- [CCDC Blue Team Prep (2020)](https://lockboxx.blogspot.com/2020/11/ccdc-blue-team-prep.html?ref=sshell.co)

- Alex Levinson

- [The Evolution of CCDC (2022)](https://alexlevinson.wordpress.com/2022/04/27/the-evolution-of-ccdc/?ref=sshell.co)
- [CCDC is the "Real World" and Here's Why (2018)](https://alexlevinson.wordpress.com/2018/04/21/ccdc-is-the-real-world-and-heres-why/?ref=sshell.co)
- [Know Your Opponent: My CCDC Toolbox](https://alexlevinson.wordpress.com/2017/05/09/know-your-opponent-my-ccdc-toolbox/?ref=sshell.co)

- WinterKnight

- [How to Win CCDC (2025)](https://www.winterknight.net/how-to-win-ccdc/?ref=sshell.co)
- [How to Win CCDC: Dealing with Red Team (2025)](https://www.winterknight.net/how-to-win-ccdc-red-team/?ref=sshell.co)
- [How to Win CCDC: Competition Overview (2025)](https://www.winterknight.net/how-to-win-ccdc-overview/?ref=sshell.co)
- [How to Win CCDC: Injects (2025)](https://www.winterknight.net/how-to-win-ccdc-injects/?ref=sshell.co)
- [How to Win CCDC: Team Dynamics (2025)](https://www.winterknight.net/how-to-win-ccdc-team-dynamics/?ref=sshell.co)
- [2025 Minnesota and Indiana CCDC Debrief](https://www.winterknight.net/ccdc-state-2025/?ref=sshell.co)

- Tom Kopchak

- [NECCDC 2023: Red Team Adventures](https://hurricanelabs.com/blog/neccdc-2023-red-team-adventures/?ref=sshell.co)
- [NECCDC 2020: Red Team Review](https://hurricanelabs.com/blog/neccdc-2020-red-team-review/?ref=sshell.co)
- [NECCDC 2019: Red Team Review](https://hurricanelabs.com/blog/neccdc-2019-red-team-review/?ref=sshell.co)
- [SOC Talk: Red Teaming for NECCDC (2022, podcast, 16 min)](https://hurricanelabs.com/podcast/soc-talk-red-teaming-for-neccdc/?ref=sshell.co)

- Blue Teams

- jakegines.in - [Northeast + National CCDC Blog Post (2024)](https://www.jakegines.in/blog/2024/ccdc?ref=sshell.co)
- infrasec.sh - [2024 Northeast CCDC](https://infrasec.sh/post/neccdc-2024/?ref=sshell.co)
- bokkisec.com - [CCDC Journey - Zero to Nationals (2024)](https://bokkisec.com/blog/CCDC_Journey-Zero_to_Nationals?ref=sshell.co)
- sourque.com - [CCDC Nationals in San Antonio, TX (2022)](https://sourque.com/blog/ccdc/?ref=sshell.co)
- hernan.de - [From Competitor to Captain: My CCDC Journey (2020)](https://hernan.de/blog/bootstrapping-a-ccdc-team/?ref=sshell.co)
- blog.bajiri.com - [Various CCDC Documents (2023)](https://blog.bajiri.com/p/ccdc-documents.html?ref=sshell.co)
- blog.bajiri.com - [CCDC 2022-2023 Log](https://blog.bajiri.com/2022/09/ccdc-2022-23-log-1.html?ref=sshell.co)
- altoid0.com - [CCDC Nationals - Reigniting the Legacy](https://altoid0.com/ccdc-nationals-reigniting-the-legacy?ref=sshell.co)
- National CyberWatch Center - [How to Prepare for CCDC (old, but comprehensive)](https://atecentral.net/downloads/737/NCC_How_To_Prepare_For_the_CCDC.pdf?ref=sshell.co)

Subscribe to my monthly newsletter

No spam, no sharing to third party. Only you and me.
