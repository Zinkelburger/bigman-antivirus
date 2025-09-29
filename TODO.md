# UI
## ClamTK equivalent
- ClamTK GUI no longer maintained
https://github.com/Cisco-Talos/clamav
https://github.com/dave-theunsub/clamtk
- run clamscan, desktop notification on issues (?)
So many options, have toggles, and tooltips to explain them
https://linux.die.net/man/1/clamscan
- ability to start/stop/configure clamd
- set up scheduled scans
- show scan history: times & results
- quarantine management (view, restore, delete)
- run freshclam, show date of last refresh
- shows persistently in the taskbar? Like how Norton does on windows
- real time protection with `clamonacc`

https://en.wikipedia.org/wiki/ClamAV
- freshclam needs to be run with `sudo` once to initialize itself
- automatically updates with `sudo systemctl status clamav-freshclam`
freshclam configuration file, which is typically located at /etc/clamav/freshclam.conf or /etc/freshclam.conf.
- use polkit to ask for password
- can query status without sudo. systemctl status clamav-daemon
- 

Inside this file, you'll find a line that looks like this:

# Number of database checks per day.
# Default: 12 (every two hours)
# Checks 24

- unit tests to confirm clamav update didn't break the program

## GUI for other security tools
https://github.com/ossec/ossec-hids
https://cisofy.com/lynis/
ufw (firewall gui)
- Gufw
- KDE, etc. already provide this
- show the user ss -tulnp. Click to view the binary location
Firejail (run things in a sandbox)

## Upload to virus total
- Use VirusTotal API, have to get a token

- Submit to virustotal

## Linpeas output
- Parse linpeas output, provide recommendations to remediate them

## Common CCDC attack things
- Startup scripts systemd, etc. locations
- Check against a list of standard/default ones, or a baseline (?)

# Browser Extension:
## Dumb phishing stuff
- Detect mismatch between <a> tag and its contents
Some gTLDs can have digits, fix the regex, etc.
Won't match paths (/page.html)

- Detect lack of dictionary matches
micrasoft.com
microsft.com

- ADGuard has a list of suspicious urls it uses?

- Detect mixing uppercase and lowercase letters, lowercase everything
neccdi.org vs neccdl.org
appie.com

# PDF:
- Suspicious segments
JS, check if URI matches link
Phishing via pdfs is very common. E.g. game tester gets a pdf for a game, but its the wrong website to download from
`pdfid`
`pdf-parser.py` to extract the malicious parts
- detect obfuscated code. Is it really that hard
dictonary words to chars ratio
not that simple, because you can shorten JS for valid reasons
<https://ontheline.trincoll.edu/images/bookdown/sample-local-pdf.pdf>

Chrome looks for manifests in these locations:
- Linux: ~/.config/google-chrome/NativeMessagingHosts/
- Windows: HKEY_CURRENT_USER\SOFTWARE\Google\Chrome\NativeMessagingHosts\
- macOS: ~/Library/Application Support/Google/Chrome/NativeMessagingHosts/

# Crypto:
- Check to make sure addresses are correct
- White list of addresses, prevent modification, idk
How do people send crypto, how are the addresses being modified? Can we detect these modifications
Can we trick crypto extensions, lol. Like "put a crypto address up, does it get modified?" and detect tem

# Bash:
- All the time people have you install stuff like `wget website.com | bash`
instead, can we do `wget website.com | bigman`
tells you what will be executed

# Filesystem:
- Protect .git credentials

# Firewall
"You download a geolocation database, create an ipset in firewalld and automatically drop ANY traffic from 20 or so 'dodgy' countries you select. You can probably guess most of them. There's a blog I found somewhere with details. I used their country list and it worked great so I'm now doing on all servers that have a port open to the internet. It may seem a bit brute forcey but for me at least no legitimate traffic comes from those locations. May not be appropriate for everyone's needs."

# OSSEC
File Integrity Monitoring: OSSEC creates cryptographic checksums of important system files. It will alert you if any of these files are modified, which could indicate a compromise.
Log Analysis: It collects, correlates, and analyzes logs from your operating system and applications to find patterns that suggest an attack, brute-force attempt, or other malicious activity.
Rootkit Detection: It has a component (rootcheck) that specifically looks for the signs of rootkits, which are designed to hide their presence on a system.
Active Response: It can be configured to take automatic actions when certain alerts are triggered, such as blocking an IP address that is repeatedly failing to log in.

- Wazuh is good for collecting logs

- Protect firefox/chrome local storage from being read, info stealers. Crypto wallets, etc.

Test cases for each kind of attack!
